//! The menu shell: the bar of places, the status strip and the footer, which
//! never move, and between them the place that is open. Every later screen
//! hangs on this. How it works: docs/menu-shell.md.
//!
//! All five places are spawned once and only shown or hidden, so a place is
//! found exactly as it was left.

use bevy::{
    input_focus::{FocusCause, InputFocus, tab_navigation::TabGroup},
    prelude::*,
    ui::{InteractionDisabled, Selected},
    ui_widgets::Activate,
};
use serde::Deserialize;

use super::parts::{
    ClosePanel, CounterValue, PartText, button, counter, dim, panel, reason, row, side_panel, tab,
    text, title,
};
use super::tokens::Tokens;
use crate::game::{Game, SceneView, Screen, report_side, report_text, stats_line};
use crate::sim::data::Slot;
use crate::sim::expedition::Phase;

/// The map of the menus, from `assets/data/places.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Places {
    pub settings: String,
    pub places: Vec<Place>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Place {
    pub name: String,
    /// What is decided here, for a place with no tabs.
    #[serde(default)]
    pub job: String,
    /// Show the horses beside the tabs on one side and the cargo on the other.
    #[serde(default)]
    pub horse_columns: bool,
    #[serde(default)]
    pub tabs: Vec<Tab>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tab {
    pub name: String,
    pub job: String,
    #[serde(default)]
    pub content: Content,
}

/// What a tab shows. `Planned` is a tab whose system is not built.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
pub enum Content {
    #[default]
    Planned,
    Gear,
    Report,
}

impl Places {
    pub fn load() -> Places {
        ron::from_str(include_str!("../../../assets/data/places.ron"))
            .unwrap_or_else(|e| panic!("assets/data/places.ron: {e}"))
    }
}

impl Place {
    /// The pages of a place: one per tab, or the place itself when it has none.
    fn pages(&self) -> Vec<(String, &str, Content)> {
        if self.tabs.is_empty() {
            return vec![(self.name.clone(), &self.job, Content::Planned)];
        }
        self.tabs
            .iter()
            .map(|tab| {
                (
                    format!("{}, {}", self.name, tab.name),
                    tab.job.as_str(),
                    tab.content,
                )
            })
            .collect()
    }
}

/// Detail about one thing, open in a place's side panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Detail {
    /// The tab it was opened from; it shows on that tab only.
    pub tab: usize,
    pub item: usize,
    /// The control that opened it, which gets the focus back.
    pub opener: Option<Entity>,
}

/// Where the player is in the menus. One selected horse for every place;
/// each place keeps its own tab and its own open detail.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct Shell {
    place: usize,
    /// Whether the place is showing. Always, in town. On the road it can be
    /// closed, which gives its room to the scene.
    showing: bool,
    horse: usize,
    tabs: Vec<usize>,
    details: Vec<Option<Detail>>,
}

impl Shell {
    pub fn new(places: usize) -> Shell {
        Shell {
            place: 0,
            showing: true,
            horse: 0,
            tabs: vec![0; places],
            details: vec![None; places],
        }
    }

    pub fn place(&self) -> usize {
        self.place
    }

    pub fn horse(&self) -> usize {
        self.horse
    }

    pub fn tab(&self, place: usize) -> usize {
        self.tabs[place]
    }

    pub fn open(&mut self, place: usize) {
        if place < self.tabs.len() {
            self.place = place;
            self.showing = true;
        }
    }

    /// Whether a place is showing. When none is, the scene has its room.
    pub fn is_open(&self) -> bool {
        self.showing
    }

    /// Put the place away. It is found as it was left when it is opened
    /// again. The caller sees to it that this happens on the road only.
    pub fn close(&mut self) {
        self.showing = false;
    }

    /// A press on a place in the bar. It opens that place, except that on
    /// the road a press on the place already showing closes it.
    pub fn press(&mut self, place: usize, in_town: bool) {
        if !in_town && self.showing && place == self.place {
            self.close();
        } else {
            self.open(place);
        }
    }

    pub fn choose_tab(&mut self, place: usize, tab: usize) {
        if place < self.tabs.len() {
            self.tabs[place] = tab;
        }
    }

    pub fn select_horse(&mut self, horse: usize) {
        self.horse = horse;
    }

    /// The detail showing in a place: the one opened from its current tab.
    pub fn detail(&self, place: usize) -> Option<Detail> {
        self.details[place].filter(|detail| detail.tab == self.tabs[place])
    }

    pub fn show_detail(&mut self, place: usize, detail: Detail) {
        if place < self.details.len() {
            self.details[place] = Some(detail);
        }
    }

    /// Close the detail showing in the open place. Returns what opened it.
    pub fn close_detail(&mut self) -> Option<Entity> {
        let detail = self.detail(self.place)?;
        self.details[self.place] = None;
        detail.opener
    }
}

#[derive(Component)]
pub struct PlaceButton(pub usize);

#[derive(Component)]
pub struct TabButton {
    pub place: usize,
    pub tab: usize,
}

#[derive(Component)]
pub struct HorseButton(pub usize);

/// A gear slot of the selected horse. Opens the piece in the side panel.
#[derive(Component)]
pub struct SlotRow {
    pub place: usize,
    pub tab: usize,
    pub slot: usize,
}

/// The one departure action, in the footer.
#[derive(Component)]
pub struct SetOut;

/// Something shown only while the player is there.
#[derive(Component)]
enum Page {
    Place(usize),
    Tab(usize, usize),
    Detail(usize),
    /// What is left between the strip and the footer with every place closed.
    RoadView,
}

/// The whole shell: everything below the scene.
#[derive(Component)]
struct ShellRoot;

/// Text that follows the game. On a text it sets that text; on a counter or
/// a row it sets the value.
#[derive(Component)]
struct Live(Fact);

#[derive(Clone, Copy)]
enum Fact {
    Coins,
    Health(usize),
    SelectedHorse,
    SelectedStats,
    Slot(usize),
    Detail(usize),
    Report,
    ReportSide,
    Footer,
}

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        let places = Places::load();
        app.insert_resource(Shell::new(places.places.len()))
            .insert_resource(places)
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    escape,
                    follow_the_party,
                    move_scene,
                    show_place,
                    mark_chosen,
                    show_facts,
                )
                    .chain(),
            )
            .add_observer(on_activate);
    }
}

fn spawn(mut commands: Commands, tokens: Res<Tokens>, places: Res<Places>, game: Res<Game>) {
    let t = &*tokens;
    let gap = Val::Px(t.space.m);
    let edge = BorderColor::all(t.colour.panel_border.0);
    let band = |border: UiRect| {
        (
            Node {
                align_items: AlignItems::Center,
                column_gap: Val::Px(t.space.l),
                padding: UiRect::axes(Val::Px(t.space.l), Val::Px(t.space.s)),
                border,
                flex_shrink: 0.0,
                ..default()
            },
            BackgroundColor(t.colour.panel.0),
            edge,
        )
    };
    let spacer = || Node {
        flex_grow: 1.0,
        ..default()
    };
    let column = |width: f32| Node {
        width: Val::Px(width),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(t.space.m)),
        row_gap: Val::Px(t.space.s),
        border: UiRect::all(Val::Px(t.size.border)),
        flex_shrink: 0.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(t.layout.town_share * 100.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(t.colour.screen.0),
            TabGroup::default(),
            ShellRoot,
        ))
        .with_children(|root| {
            // The bar: the places, in the order of the data file, then settings.
            root.spawn((
                Node {
                    align_items: AlignItems::FlexEnd,
                    padding: UiRect::horizontal(Val::Px(t.space.l)),
                    border: UiRect::vertical(Val::Px(t.size.border)),
                    flex_shrink: 0.0,
                    ..default()
                },
                BackgroundColor(t.colour.panel.0),
                edge,
            ))
            .with_children(|bar| {
                for (p, place) in places.places.iter().enumerate() {
                    bar.spawn((tab(t, &place.name, None), PlaceButton(p)));
                }
                bar.spawn(spacer());
                bar.spawn((button(t, &places.settings, None), InteractionDisabled));
            });

            // The status strip. Bags, hold and the day keep their positions
            // until the systems behind them exist.
            root.spawn(band(UiRect::bottom(Val::Px(t.size.border))))
                .with_children(|strip| {
                    strip.spawn((counter(t, "coins", "0"), Live(Fact::Coins)));
                    strip.spawn(counter(t, "bags", "-"));
                    strip.spawn(counter(t, "hold", "-"));
                    strip.spawn(spacer());
                    for (h, horse) in game.party.iter().enumerate() {
                        strip
                            .spawn(Node {
                                width: Val::Px(t.size.strip_horse_width),
                                flex_shrink: 0.0,
                                ..default()
                            })
                            .with_children(|cell| {
                                cell.spawn((
                                    row(t, &horse.name, ""),
                                    HorseButton(h),
                                    Live(Fact::Health(h)),
                                ));
                            });
                    }
                    strip.spawn(spacer());
                    strip.spawn(counter(t, "day", "-"));
                });

            // The places. All of them exist all the time; one is shown. The
            // area is clipped, so a place is covered and uncovered as the
            // scene moves instead of being squeezed.
            root.spawn(Node {
                flex_grow: 1.0,
                min_height: Val::Px(0.0),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|area| {
                area.spawn((
                    Page::RoadView,
                    Node {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(t.space.l)),
                        display: Display::None,
                        ..default()
                    },
                ))
                .with_children(|line| {
                    line.spawn(dim(
                        t,
                        "On the road. Open a place to look and plan; press it again, or Escape, to give the scene its room back.",
                    ));
                });
                for (p, place) in places.places.iter().enumerate() {
                    area.spawn((
                        Page::Place(p),
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            flex_shrink: 0.0,
                            ..default()
                        },
                    ))
                    .with_children(|page| {
                        if !place.tabs.is_empty() {
                            page.spawn(Node {
                                padding: UiRect::horizontal(Val::Px(t.space.l)),
                                margin: UiRect::top(Val::Px(t.space.s)),
                                flex_shrink: 0.0,
                                ..default()
                            })
                            .with_children(|tabs| {
                                for (i, each) in place.tabs.iter().enumerate() {
                                    tabs.spawn((
                                        tab(t, &each.name, None),
                                        TabButton { place: p, tab: i },
                                    ));
                                }
                            });
                        }

                        page.spawn(Node {
                            flex_grow: 1.0,
                            min_height: Val::Px(0.0),
                            column_gap: gap,
                            padding: UiRect::all(Val::Px(t.space.m)),
                            ..default()
                        })
                        .with_children(|body| {
                            if place.horse_columns {
                                body.spawn(panel(t))
                                    .insert(column(t.size.horses_width))
                                    .with_children(|horses| {
                                        horses.spawn(dim(t, "horses"));
                                        for (h, horse) in game.party.iter().enumerate() {
                                            horses.spawn((
                                                row(t, &horse.name, ""),
                                                HorseButton(h),
                                                Live(Fact::Health(h)),
                                            ));
                                        }
                                    });
                            }

                            body.spawn(Node {
                                flex_grow: 1.0,
                                min_width: Val::Px(0.0),
                                ..default()
                            })
                            .with_children(|pages| {
                                for (i, (heading, job, content)) in
                                    place.pages().into_iter().enumerate()
                                {
                                    pages
                                        .spawn((
                                            Page::Tab(p, i),
                                            Node {
                                                width: Val::Percent(100.0),
                                                flex_direction: FlexDirection::Column,
                                                padding: UiRect::all(Val::Px(t.space.m)),
                                                row_gap: Val::Px(t.space.s),
                                                overflow: Overflow::clip(),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|tab_page| {
                                            tab_page.spawn(title(t, &heading));
                                            tab_page.spawn(dim(t, job));
                                            spawn_content(tab_page, t, p, i, content);
                                        });
                                }
                            });

                            if place.horse_columns {
                                body.spawn(panel(t))
                                    .insert(column(t.size.cargo_width))
                                    .with_children(|cargo| {
                                        cargo.spawn(dim(t, "cargo"));
                                        cargo.spawn(dim(t, "Nothing here yet."));
                                        cargo.spawn(dim(
                                            t,
                                            "Gear the horses bring back is carried here.",
                                        ));
                                    });
                            }

                            body.spawn((side_panel(t, "Detail"), Page::Detail(p)))
                                .with_children(|side| {
                                    side.spawn((text(t, ""), Live(Fact::Detail(p))));
                                });
                        });
                    });
                }
            });

            // The footer: what the party is doing, and the one departure.
            root.spawn(band(UiRect::top(Val::Px(t.size.border))))
                .with_children(|footer| {
                    footer.spawn((text(t, ""), Live(Fact::Footer)));
                    footer.spawn(spacer());
                    footer.spawn((button(t, "Set out", Some("Space")), SetOut));
                });
        });
}

fn spawn_content(
    page: &mut ChildSpawnerCommands,
    t: &Tokens,
    place: usize,
    tab: usize,
    content: Content,
) {
    match content {
        Content::Planned => {
            page.spawn((text(t, ""), Live(Fact::SelectedHorse)));
            page.spawn(reason(t, "Not built yet."));
        }
        Content::Gear => {
            page.spawn((text(t, ""), Live(Fact::SelectedStats)));
            page.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::top(Val::Px(t.space.s)),
                ..default()
            })
            .with_children(|slots| {
                for slot in Slot::ALL {
                    slots.spawn((
                        row(t, slot.name(), ""),
                        SlotRow {
                            place,
                            tab,
                            slot: slot.index(),
                        },
                        Live(Fact::Slot(slot.index())),
                    ));
                }
            });
        }
        Content::Report => {
            page.spawn(Node {
                column_gap: Val::Px(t.space.l),
                margin: UiRect::top(Val::Px(t.space.s)),
                ..default()
            })
            .with_children(|columns| {
                // Two halves of fixed width: neither moves as the other fills.
                for fact in [Fact::Report, Fact::ReportSide] {
                    columns
                        .spawn(Node {
                            width: Val::Percent(50.0),
                            ..default()
                        })
                        .with_children(|half| {
                            half.spawn((text(t, ""), Live(fact)));
                        });
                }
            });
        }
    }
}

/// One observer for every control of the shell: what a press does depends
/// on what the control is.
fn on_activate(
    activate: On<Activate>,
    mut shell: ResMut<Shell>,
    mut game: ResMut<Game>,
    mut focus: ResMut<InputFocus>,
    places: Query<&PlaceButton>,
    tabs: Query<&TabButton>,
    horses: Query<&HorseButton>,
    slots: Query<&SlotRow>,
    closes: Query<(), With<ClosePanel>>,
    set_outs: Query<(), With<SetOut>>,
) {
    let entity = activate.entity;
    if let Ok(place) = places.get(entity) {
        shell.press(place.0, game.in_town());
    } else if let Ok(tab) = tabs.get(entity) {
        shell.choose_tab(tab.place, tab.tab);
    } else if let Ok(horse) = horses.get(entity) {
        shell.select_horse(horse.0);
    } else if let Ok(row) = slots.get(entity) {
        shell.show_detail(
            row.place,
            Detail {
                tab: row.tab,
                item: row.slot,
                opener: Some(entity),
            },
        );
    } else if closes.contains(entity) {
        close_detail(&mut shell, &mut focus);
    } else if set_outs.contains(entity) {
        game.set_out();
    }
}

fn close_detail(shell: &mut Shell, focus: &mut InputFocus) {
    if let Some(opener) = shell.close_detail() {
        focus.set(opener, FocusCause::Navigated);
    }
}

/// Escape goes back one level: it closes the side panel, and on the road,
/// with no side panel open, the place.
fn escape(
    keys: Res<ButtonInput<KeyCode>>,
    game: Res<Game>,
    mut shell: ResMut<Shell>,
    mut focus: ResMut<InputFocus>,
) {
    if !keys.just_pressed(KeyCode::Escape) || !shell.is_open() {
        return;
    }
    if shell.detail(shell.place()).is_some() {
        close_detail(&mut shell, &mut focus);
    } else if !game.in_town() {
        shell.close();
    }
}

/// Setting out puts the place away, so the road starts with the scene
/// expanded; arriving opens it again, as it was left. Town is where the
/// player decides.
fn follow_the_party(game: Res<Game>, mut shell: ResMut<Shell>, mut was_in_town: Local<bool>) {
    let in_town = game.in_town();
    if in_town != *was_in_town {
        *was_in_town = in_town;
        if in_town {
            let place = shell.place();
            shell.open(place);
        } else {
            shell.close();
        }
    }
}

/// Move the scene towards the layout the shell asks for, and the shell's
/// top edge with it.
fn move_scene(
    time: Res<Time>,
    tokens: Res<Tokens>,
    shell: Res<Shell>,
    mut view: ResMut<SceneView>,
    mut root: Single<&mut Node, With<ShellRoot>>,
) {
    let wanted = if shell.is_open() { 0.0 } else { 1.0 };
    if view.progress != wanted {
        view.step_towards(!shell.is_open(), time.delta_secs(), tokens.time.layout);
    }
    let top = Val::Percent(tokens.layout.share(view.expanded()) * 100.0);
    if root.top != top {
        root.top = top;
    }
}

fn show_place(
    shell: Res<Shell>,
    view: Res<SceneView>,
    mut pages: Query<(&Page, &mut Node), Without<ShellRoot>>,
) {
    // A place that is being put away stays in view until it is covered.
    let place_in_view = shell.is_open() || view.progress < 1.0;
    for (page, mut node) in &mut pages {
        let shown = match *page {
            Page::Place(place) => place == shell.place() && place_in_view,
            Page::Tab(place, tab) => tab == shell.tab(place),
            Page::Detail(place) => shell.detail(place).is_some(),
            Page::RoadView => !place_in_view,
        };
        let wanted = if shown { Display::Flex } else { Display::None };
        if node.display != wanted {
            node.display = wanted;
        }
    }
}

/// Mark what is chosen, and what cannot be used right now.
fn mark_chosen(
    mut commands: Commands,
    shell: Res<Shell>,
    game: Res<Game>,
    controls: Query<
        (
            Entity,
            Option<&PlaceButton>,
            Option<&TabButton>,
            Option<&HorseButton>,
            Option<&SlotRow>,
            Has<Selected>,
        ),
        Or<(
            With<PlaceButton>,
            With<TabButton>,
            With<HorseButton>,
            With<SlotRow>,
        )>,
    >,
    set_outs: Query<(Entity, Has<InteractionDisabled>), With<SetOut>>,
) {
    for (entity, place, tab, horse, slot, selected) in &controls {
        let wanted = place.is_some_and(|place| place.0 == shell.place() && shell.is_open())
            || tab.is_some_and(|tab| tab.tab == shell.tab(tab.place))
            || horse.is_some_and(|horse| horse.0 == shell.horse())
            || slot.is_some_and(|row| {
                shell
                    .detail(row.place)
                    .is_some_and(|detail| detail.tab == row.tab && detail.item == row.slot)
            });
        if wanted && !selected {
            commands.entity(entity).insert(Selected);
        } else if !wanted && selected {
            commands.entity(entity).remove::<Selected>();
        }
    }

    for (entity, disabled) in &set_outs {
        if game.in_town() && disabled {
            commands.entity(entity).remove::<InteractionDisabled>();
        } else if !game.in_town() && !disabled {
            commands.entity(entity).insert(InteractionDisabled);
        }
    }
}

fn show_facts(
    game: Res<Game>,
    shell: Res<Shell>,
    live: Query<(Entity, &Live)>,
    children: Query<&Children>,
    counters: Query<(), With<CounterValue>>,
    kinds: Query<&PartText>,
    mut texts: Query<&mut Text>,
) {
    for (entity, live) in &live {
        let target = if texts.contains(entity) {
            Some(entity)
        } else {
            children
                .iter_descendants(entity)
                .find(|child| counters.contains(*child))
                .or_else(|| {
                    children
                        .iter_descendants(entity)
                        .find(|child| matches!(kinds.get(*child), Ok(PartText::Dim)))
                })
        };
        let Some(mut text) = target.and_then(|target| texts.get_mut(target).ok()) else {
            continue;
        };
        let words = fact(&game, &shell, live.0);
        if text.0 != words {
            text.0 = words;
        }
    }
}

fn fact(game: &Game, shell: &Shell, fact: Fact) -> String {
    let selected = &game.party[shell.horse().min(game.party.len() - 1)];
    match fact {
        Fact::Coins => game.coins.to_string(),
        Fact::Health(horse) => match &game.screen {
            Screen::Trail(expedition) => {
                let fighter = &expedition.party[horse];
                format!("{}/{}", fighter.health, fighter.max_health)
            }
            Screen::Camp { .. } => {
                let health = game.party[horse].stats().health;
                format!("{health}/{health}")
            }
        },
        Fact::SelectedHorse => format!("Selected horse: {}", selected.name),
        Fact::SelectedStats => {
            let stats = selected.stats();
            format!(
                "{}: health {}, attack {}, defense {}, elemental {}",
                selected.name, stats.health, stats.attack, stats.defense, stats.elemental
            )
        }
        Fact::Slot(slot) => match &selected.gear[slot] {
            Some(gear) => gear.name.clone(),
            None => "nothing worn".into(),
        },
        Fact::Detail(place) => {
            let Some(detail) = shell.detail(place) else {
                return String::new();
            };
            let slot = Slot::ALL[detail.item];
            match &selected.gear[detail.item] {
                Some(gear) => format!(
                    "{}\n{} slot, worn by {}\n{}\n{}",
                    gear.name,
                    slot.name(),
                    selected.name,
                    game.data.gear.rarities[gear.rarity].name,
                    stats_line(gear.stats),
                ),
                None => format!(
                    "{} slot of {}\nNothing worn.\nGear comes from the trail.",
                    slot.name(),
                    selected.name,
                ),
            }
        }
        Fact::Report => report_text(game),
        Fact::ReportSide => report_side(game),
        Fact::Footer => match &game.screen {
            Screen::Camp { countdown, .. } => format!(
                "Ready for: a level {} trail, {} encounters. Setting out in {:.0}s.",
                game.plan.level,
                game.plan.encounters.len(),
                countdown.max(0.0).ceil(),
            ),
            Screen::Trail(expedition) => {
                let total = expedition.plan.encounters.len();
                let at = expedition.encounter + 1;
                match expedition.phase {
                    Phase::Fighting => format!("On the trail: fighting encounter {at} of {total}."),
                    _ => format!("On the trail: walking to encounter {at} of {total}."),
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// "It must stay five places" (epic #225), in the owner's order.
    #[test]
    fn the_bar_is_the_five_places_in_order() {
        let places = Places::load();
        let names: Vec<&str> = places.places.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["Horses", "Trade", "Map", "Town", "Journal"]);
    }

    /// One name for one thing: no two tabs of a place share a name, and
    /// every page says what is decided there.
    #[test]
    fn every_page_has_its_own_name_and_a_job() {
        for place in Places::load().places {
            let pages = place.pages();
            for (i, (heading, job, _)) in pages.iter().enumerate() {
                assert!(!job.is_empty(), "{heading} has no job");
                assert!(
                    pages[i + 1..].iter().all(|(other, _, _)| other != heading),
                    "two pages are called {heading}"
                );
            }
        }
    }

    #[test]
    fn town_opens_on_the_first_place() {
        let shell = Shell::new(5);
        assert_eq!((shell.place(), shell.tab(0), shell.horse()), (0, 0, 0));
    }

    /// Leaving a place and returning shows it as it was left.
    #[test]
    fn a_place_is_found_as_it_was_left() {
        let mut shell = Shell::new(5);
        shell.choose_tab(0, 2);
        shell.open(4);
        shell.choose_tab(4, 1);
        shell.open(0);
        assert_eq!(shell.tab(0), 2);
        shell.open(4);
        assert_eq!(shell.tab(4), 1);
    }

    /// A detail belongs to the tab it was opened from: it is out of the way
    /// on another tab and back on return, until it is closed.
    #[test]
    fn a_detail_stays_with_its_tab() {
        let mut shell = Shell::new(5);
        let detail = Detail {
            tab: 0,
            item: 1,
            opener: None,
        };
        shell.show_detail(0, detail);
        assert_eq!(shell.detail(0), Some(detail));
        shell.choose_tab(0, 1);
        assert_eq!(shell.detail(0), None);
        shell.open(3);
        shell.open(0);
        shell.choose_tab(0, 0);
        assert_eq!(shell.detail(0), Some(detail));
        shell.close_detail();
        assert_eq!(shell.detail(0), None);
    }

    /// Selecting a horse in one place selects it in all of them.
    #[test]
    fn one_horse_is_selected_everywhere() {
        let mut shell = Shell::new(5);
        shell.select_horse(2);
        for place in 0..5 {
            shell.open(place);
            assert_eq!(shell.horse(), 2);
        }
    }

    /// On the road a press on the open place puts it away and gives the
    /// scene its room; any press on the bar brings a place back. In town a
    /// place is always open.
    #[test]
    fn on_the_road_a_place_closes_and_opens_again() {
        let mut shell = Shell::new(5);
        shell.press(0, true);
        assert!(shell.is_open(), "a press in town never closes the place");
        shell.press(0, false);
        assert!(!shell.is_open());
        shell.press(0, false);
        assert!(shell.is_open());
        shell.press(3, false);
        assert!(shell.is_open());
        assert_eq!(shell.place(), 3);
    }

    /// A place put away on the road is found as it was left.
    #[test]
    fn a_closed_place_keeps_its_tab_and_detail() {
        let mut shell = Shell::new(5);
        let detail = Detail {
            tab: 1,
            item: 2,
            opener: None,
        };
        shell.choose_tab(0, 1);
        shell.show_detail(0, detail);
        shell.close();
        shell.open(0);
        assert_eq!(shell.tab(0), 1);
        assert_eq!(shell.detail(0), Some(detail));
    }

    #[test]
    fn a_place_that_does_not_exist_cannot_be_opened() {
        let mut shell = Shell::new(5);
        shell.open(9);
        assert_eq!(shell.place(), 0);
    }
}
