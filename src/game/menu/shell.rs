//! The menu shell: the bar of places, the status strip and the footer, which
//! never move, and between them the place that is open. Every later screen
//! hangs on this. How it works: docs/menu-shell.md.
//!
//! All five places are spawned once and only shown or hidden, so a place is
//! found exactly as it was left.

use bevy::{
    input_focus::{
        FocusCause, InputFocus, InputFocusVisible,
        directional_navigation::{AutoNavigationConfig, FocusableArea},
        navigator::find_best_candidate,
        tab_navigation::TabGroup,
    },
    math::CompassOctant,
    prelude::*,
    ui::{InteractionDisabled, Selected, UiGlobalTransform},
    ui_widgets::{
        Activate, ControlOrientation, ScrollArea, ScrollIntoView, Scrollbar, ScrollbarThumb,
    },
};
use serde::Deserialize;

use super::card;
use super::picture::{self, picture};
use super::parts::{
    ClosePanel, CounterValue, NotBuilt, Part, RowValue, Stop, button, cell, counter, dim, panel,
    reason, row, side_panel, tab, text, title, trail_mark,
};
use super::tokens::Tokens;
use crate::game::{
    Action, Game, Hand, Piece, SceneView, Screen, Uncollected, report_side, report_text,
    stats_line,
};
use crate::sim::data::Slot;
use crate::sim::expedition::Phase;

/// The map of the menus, from `assets/data/places.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Places {
    pub places: Vec<Place>,
    /// At the far end of the bar: a page like a place, and not one of them.
    pub settings: Place,
    /// The keys that select the horses of the party, in order.
    #[serde(default)]
    pub horse_keys: Vec<String>,
    #[serde(default)]
    pub footer: Vec<Control>,
    /// What can be done with a piece of gear, on its card (#228): the same
    /// controls wherever the piece is.
    #[serde(default)]
    pub card: Vec<Control>,
    /// Every control of the map, by its `id`.
    #[serde(skip)]
    list: Vec<Control>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Place {
    pub name: String,
    /// The key that opens it, printed on it.
    #[serde(default)]
    pub key: Option<String>,
    /// What is decided here, for a place with no tabs.
    #[serde(default)]
    pub job: String,
    /// Show the horses beside the tabs on one side and the cargo on the other.
    #[serde(default)]
    pub horse_columns: bool,
    #[serde(default)]
    pub tabs: Vec<Tab>,
    /// The controls of a place with no tabs.
    #[serde(default)]
    pub controls: Vec<Control>,
    /// The controls of the cargo column, in a place with `horse_columns`.
    #[serde(default)]
    pub cargo: Vec<Control>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tab {
    pub name: String,
    /// The key that chooses it while its place is showing, printed on it.
    #[serde(default)]
    pub key: Option<String>,
    pub job: String,
    #[serde(default)]
    pub content: Content,
    #[serde(default)]
    pub controls: Vec<Control>,
}

/// A control planned for the menus. One with an `action` or a `shows` works;
/// one with neither is not built yet, and a press on it says so.
#[derive(Clone, Debug, Deserialize)]
pub struct Control {
    pub name: String,
    /// What it does, or will do.
    pub does: String,
    /// The key that works it, printed on it.
    #[serde(default)]
    pub key: Option<String>,
    /// The issue that will make it work.
    #[serde(default)]
    pub issue: Option<u32>,
    /// The issue is a proposal the owner has not decided.
    #[serde(default)]
    pub proposed: bool,
    #[serde(default)]
    pub action: Option<Action>,
    /// What a press opens in the side panel of its page.
    #[serde(default)]
    pub shows: Option<Shows>,
    /// Its number among all the controls of the map, set on loading.
    #[serde(skip)]
    pub id: usize,
}

impl Control {
    /// Whether a press does what the control says: it acts, or it shows.
    pub fn built(&self) -> bool {
        self.action.is_some() || self.shows.is_some()
    }
}

/// What a control shows in the side panel when it is pressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Shows {
    /// Every key, and what it works.
    Keys,
}

/// What a tab shows. `Planned` is a tab whose system is not built.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
pub enum Content {
    #[default]
    Planned,
    Gear,
    /// The party's one element, the slice's stand-in for abilities.
    Element,
    Report,
}

impl Places {
    pub fn load() -> Places {
        let mut places: Places = ron::from_str(include_str!("../../../assets/data/places.ron"))
            .unwrap_or_else(|e| panic!("assets/data/places.ron: {e}"));
        let mut list = Vec::new();
        for (id, control) in places.controls_mut().into_iter().enumerate() {
            control.id = id;
            list.push(control.clone());
        }
        places.list = list;
        places
    }

    /// Every page of the menus: the places, then settings.
    pub fn pages(&self) -> impl Iterator<Item = &Place> {
        self.places.iter().chain(std::iter::once(&self.settings))
    }

    /// Which page settings is: the one after the places.
    pub fn settings_page(&self) -> usize {
        self.places.len()
    }

    /// Every control of the map, in the order of their `id`.
    #[cfg(test)]
    pub fn controls(&self) -> &[Control] {
        &self.list
    }

    pub fn control(&self, id: usize) -> &Control {
        &self.list[id]
    }

    /// The controls of the map in the groups they are drawn in: those of a
    /// tab, of a place without tabs, of a cargo column, of the footer, and
    /// of the item card.
    /// A group says under itself why any of its controls cannot be used.
    #[cfg(test)]
    pub fn groups(&self) -> Vec<&[Control]> {
        let mut groups = Vec::new();
        for place in self.pages() {
            groups.extend(place.tabs.iter().map(|tab| tab.controls.as_slice()));
            groups.push(&place.controls);
            groups.push(&place.cargo);
        }
        groups.push(&self.footer);
        groups.push(&self.card);
        groups.retain(|group| !group.is_empty());
        groups
    }

    /// The key that opens a page: a place, or settings.
    pub fn page_key(&self, page: usize) -> Option<&str> {
        self.pages().nth(page)?.key.as_deref()
    }

    pub fn tab_key(&self, page: usize, tab: usize) -> Option<&str> {
        self.pages().nth(page)?.tabs.get(tab)?.key.as_deref()
    }

    pub fn horse_key(&self, horse: usize) -> Option<&str> {
        self.horse_keys.get(horse).map(String::as_str)
    }

    fn controls_mut(&mut self) -> Vec<&mut Control> {
        let mut all = Vec::new();
        for place in self
            .places
            .iter_mut()
            .chain(std::iter::once(&mut self.settings))
        {
            for tab in &mut place.tabs {
                all.extend(tab.controls.iter_mut());
            }
            all.extend(place.controls.iter_mut());
            all.extend(place.cargo.iter_mut());
        }
        all.extend(self.footer.iter_mut());
        all.extend(self.card.iter_mut());
        all
    }
}

/// The key a control's `key` names: a letter, a digit or Space.
pub fn key_code(name: &str) -> Option<KeyCode> {
    Some(match name {
        "Space" => KeyCode::Space,
        "0" => KeyCode::Digit0,
        "1" => KeyCode::Digit1,
        "2" => KeyCode::Digit2,
        "3" => KeyCode::Digit3,
        "4" => KeyCode::Digit4,
        "5" => KeyCode::Digit5,
        "6" => KeyCode::Digit6,
        "7" => KeyCode::Digit7,
        "8" => KeyCode::Digit8,
        "9" => KeyCode::Digit9,
        "A" => KeyCode::KeyA,
        "B" => KeyCode::KeyB,
        "C" => KeyCode::KeyC,
        "D" => KeyCode::KeyD,
        "E" => KeyCode::KeyE,
        "F" => KeyCode::KeyF,
        "G" => KeyCode::KeyG,
        "H" => KeyCode::KeyH,
        "I" => KeyCode::KeyI,
        "J" => KeyCode::KeyJ,
        "K" => KeyCode::KeyK,
        "L" => KeyCode::KeyL,
        "M" => KeyCode::KeyM,
        "N" => KeyCode::KeyN,
        "O" => KeyCode::KeyO,
        "P" => KeyCode::KeyP,
        "Q" => KeyCode::KeyQ,
        "R" => KeyCode::KeyR,
        "S" => KeyCode::KeyS,
        "T" => KeyCode::KeyT,
        "U" => KeyCode::KeyU,
        "V" => KeyCode::KeyV,
        "W" => KeyCode::KeyW,
        "X" => KeyCode::KeyX,
        "Y" => KeyCode::KeyY,
        "Z" => KeyCode::KeyZ,
        _ => return None,
    })
}

impl Place {
    /// The pages of a place: one per tab, or the place itself when it has none.
    fn pages(&self) -> Vec<(String, &str, Content, &[Control])> {
        if self.tabs.is_empty() {
            return vec![(
                self.name.clone(),
                &self.job,
                Content::Planned,
                &self.controls,
            )];
        }
        self.tabs
            .iter()
            .map(|tab| {
                (
                    format!("{}, {}", self.name, tab.name),
                    tab.job.as_str(),
                    tab.content,
                    tab.controls.as_slice(),
                )
            })
            .collect()
    }
}

/// What a side panel is about.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    /// A gear slot of a horse, and the piece worn in it.
    Worn { horse: usize, slot: usize },
    /// The piece carried in the cargo.
    Carried,
    /// A control, by its `id`: what it shows, or, for one that is not
    /// built, what it will do.
    Control(usize),
}

impl Item {
    /// The piece of gear this is, when it is one: its card is what shows.
    pub fn piece(self) -> Option<Piece> {
        match self {
            Item::Worn { horse, slot } => Some(Piece::Worn { horse, slot }),
            Item::Carried => Some(Piece::Carried),
            Item::Control(_) => None,
        }
    }
}

impl From<Piece> for Item {
    fn from(piece: Piece) -> Item {
        match piece {
            Piece::Worn { horse, slot } => Item::Worn { horse, slot },
            Piece::Carried => Item::Carried,
        }
    }
}

/// Detail about one thing, open in a place's side panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Detail {
    /// The tab it was opened from; it shows on that tab only.
    pub tab: usize,
    pub item: Item,
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

    /// What an action would be done to now: the selected horse, and the
    /// piece whose card is open in the side panel that is showing.
    pub fn hand(&self) -> Hand {
        Hand {
            horse: self.horse,
            open: self
                .detail(self.place)
                .filter(|_| self.showing)
                .and_then(|detail| detail.item.piece()),
        }
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

    /// Put away whatever detail a place has open, on any of its tabs.
    pub fn drop_detail(&mut self, place: usize) {
        if place < self.details.len() {
            self.details[place] = None;
        }
    }

    /// A piece has moved: every card that was open on it stays on it, in
    /// whichever place it is open.
    pub fn piece_moved(&mut self, from: Piece, to: Piece) {
        for detail in self.details.iter_mut().flatten() {
            if detail.item == Item::from(from) {
                detail.item = to.into();
            }
        }
    }

    /// Whether a card is open on this, in any place.
    pub fn shows(&self, item: Item) -> bool {
        self.details.iter().flatten().any(|detail| detail.item == item)
    }

    /// Put away every detail about this, in every place.
    pub fn forget(&mut self, item: Item) {
        for detail in &mut self.details {
            if detail.is_some_and(|detail| detail.item == item) {
                *detail = None;
            }
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

/// A gear slot of a horse in the overview. Opens the piece in the side
/// panel, whoever is selected.
#[derive(Component)]
pub struct SlotRow {
    pub place: usize,
    pub tab: usize,
    pub horse: usize,
    pub slot: usize,
}

/// The carried piece, where a list shows it: in the cargo column, which is
/// beside every tab of its place, and in the report, which is on one. A
/// press pins its card in the side panel, as a press on a slot does.
#[derive(Component)]
pub struct CarriedCell {
    pub place: usize,
    pub tab: Option<usize>,
}

/// Something shown only while a piece is carried (true), or only while
/// none is (false).
#[derive(Component)]
struct WhenCarried(bool);

/// A horse's column of the gear overview, outlined while it is the selected
/// horse's.
#[derive(Component)]
struct Wearer(usize);

/// A control from the map of the menus. `place` is the page it is on; a
/// control of the footer is on none.
#[derive(Component)]
pub struct ControlButton {
    pub id: usize,
    pub place: Option<usize>,
}

/// The line under a group of controls that says why those of them that work
/// cannot be used now (#233). It holds their `id`s, and is hidden while all
/// of them can be used.
#[derive(Component)]
struct Blocked(Vec<usize>);

/// Something shown only while the player is there.
#[derive(Component)]
enum Page {
    Place(usize),
    Tab(usize, usize),
    Detail(usize),
    /// The cargo column of a place. The detail opens in its room (#287).
    Cargo(usize),
    /// The actions of the item card, in a place's side panel: there while
    /// the detail is a piece of gear (#228).
    Card(usize),
    /// What is left between the strip and the footer with every place closed.
    RoadView,
}

/// The whole shell: everything below the scene.
#[derive(Component)]
struct ShellRoot;

/// The trail tracker's marks, in the footer: one per fight of the trail.
#[derive(Component)]
struct TrailTracker;

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
    /// The heading of a horse's column in the overview.
    Wearer(usize),
    /// The piece a horse wears in a slot, and the line under it.
    Worn(usize, usize),
    WornLine(usize, usize),
    Detail(usize),
    /// The carried piece as a list shows it: its name, what it is, and
    /// whether it matters to the selected horse.
    CarriedName,
    CarriedKind,
    CarriedVerdict,
    Element,
    Report,
    ReportSide,
    Footer,
}

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        let places = Places::load();
        app.insert_resource(Shell::new(places.pages().count()))
            .insert_resource(places)
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    lose_hidden_focus,
                    arrows,
                    keys,
                    escape,
                    follow_the_party,
                    move_scene,
                    show_place,
                    mark_chosen,
                    mark_wearer,
                    show_carried,
                    picture::show_pictures,
                    show_reasons,
                    show_facts,
                    show_trail,
                    show_scrollbars,
                    follow_focus,
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
                    bar.spawn((tab(t, &place.name, place.key.as_deref()), PlaceButton(p)));
                }
                bar.spawn(spacer());
                bar.spawn((
                    button(t, &places.settings.name, places.settings.key.as_deref()),
                    PlaceButton(places.settings_page()),
                ));
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
                                    row(t, &horse.name, places.horse_key(h), ""),
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
                for (p, place) in places.pages().enumerate() {
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
                                        tab(t, &each.name, each.key.as_deref()),
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
                                                row(t, &horse.name, places.horse_key(h), ""),
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
                                for (i, (heading, job, content, controls)) in
                                    place.pages().into_iter().enumerate()
                                {
                                    // A tab that does not fit scrolls, with
                                    // its bar beside it (#243): nothing is
                                    // cut off without saying so.
                                    let mut frame = pages.spawn((
                                        Page::Tab(p, i),
                                        Node {
                                            width: Val::Percent(100.0),
                                            ..default()
                                        },
                                    ));
                                    let mut scrolls = Entity::PLACEHOLDER;
                                    frame.with_children(|frame| {
                                        scrolls = frame
                                            .spawn((
                                                ScrollArea,
                                                Node {
                                                    flex_grow: 1.0,
                                                    min_width: Val::Px(0.0),
                                                    flex_direction: FlexDirection::Column,
                                                    padding: UiRect::all(Val::Px(t.space.m)),
                                                    row_gap: Val::Px(t.space.s),
                                                    overflow: Overflow {
                                                        x: OverflowAxis::Clip,
                                                        y: OverflowAxis::Scroll,
                                                    },
                                                    ..default()
                                                },
                                            ))
                                            .id();
                                        frame.spawn(scrollbar(t, scrolls));
                                    });
                                    frame
                                        .commands()
                                        .entity(scrolls)
                                        .with_children(|tab_page| {
                                            tab_page.spawn(title(t, &heading));
                                            tab_page.spawn(dim(t, job));
                                            // One horse is selected in every place,
                                            // and settings is not a place.
                                            if content == Content::Planned
                                                && p != places.settings_page()
                                            {
                                                tab_page.spawn((
                                                    text(t, ""),
                                                    Live(Fact::SelectedHorse),
                                                ));
                                            }
                                            spawn_content(tab_page, t, p, i, content, game.party.len());
                                            spawn_controls(tab_page, t, controls, Some(p), true);
                                        });
                                }
                            });

                            if place.horse_columns {
                                // The detail opens in the cargo's room and
                                // at its width, so opening one moves nothing
                                // on the tab beside it (#287).
                                body.spawn((panel(t), Page::Cargo(p)))
                                    .insert(column(t.size.cargo_width))
                                    .with_children(|cargo| {
                                        cargo.spawn(dim(t, "cargo"));
                                        cargo.spawn((
                                            dim(t, card::NOTHING_CARRIED),
                                            WhenCarried(false),
                                        ));
                                        spawn_carried(cargo, t, p, None);
                                        spawn_controls(cargo, t, &place.cargo, Some(p), true);
                                    });
                            }

                            let side = if place.horse_columns {
                                t.size.cargo_width
                            } else {
                                t.size.side_panel_width
                            };
                            body.spawn((side_panel(t, "Detail", side), Page::Detail(p)))
                                .with_children(|side| {
                                    // What the detail says scrolls where it
                                    // does not fit, so that the card's
                                    // actions under it are always in view.
                                    let mut scrolls = Entity::PLACEHOLDER;
                                    side.spawn(Node {
                                        flex_grow: 1.0,
                                        min_height: Val::Px(0.0),
                                        ..default()
                                    })
                                    .with_children(|frame| {
                                        scrolls = frame
                                            .spawn((
                                                ScrollArea,
                                                Node {
                                                    flex_grow: 1.0,
                                                    min_width: Val::Px(0.0),
                                                    flex_direction: FlexDirection::Column,
                                                    overflow: Overflow {
                                                        x: OverflowAxis::Clip,
                                                        y: OverflowAxis::Scroll,
                                                    },
                                                    ..default()
                                                },
                                            ))
                                            .with_children(|words| {
                                                words.spawn((
                                                    text(t, ""),
                                                    Live(Fact::Detail(p)),
                                                ));
                                            })
                                            .id();
                                        frame.spawn(scrollbar(t, scrolls));
                                    });
                                    // The card's actions, the same in every
                                    // place a piece can be opened in (#228).
                                    if p != places.settings_page() {
                                        side.spawn((
                                            Node {
                                                flex_direction: FlexDirection::Column,
                                                row_gap: Val::Px(t.space.s),
                                                flex_shrink: 0.0,
                                                ..default()
                                            },
                                            Page::Card(p),
                                        ))
                                        .with_children(|actions| {
                                            spawn_controls(actions, t, &places.card, Some(p), false);
                                        });
                                    }
                                });
                        });
                    });
                }
            });

            // The footer: what the party is doing, and the one departure.
            root.spawn(band(UiRect::top(Val::Px(t.size.border))))
                .with_children(|footer| {
                    // The trail tracker (#253), first so that nothing beside
                    // it can move it: how many fights are left, at a glance.
                    footer
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(t.space.s),
                            flex_shrink: 0.0,
                            ..default()
                        })
                        .with_children(|trail| {
                            trail.spawn((
                                Node {
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(t.space.s),
                                    ..default()
                                },
                                TrailTracker,
                            ));
                            trail.spawn(dim(t, "boss"));
                        });
                    footer.spawn((text(t, ""), Live(Fact::Footer)));
                    footer.spawn(spacer());
                    // One above the other and never squeezed: the footer
                    // has one line's height and little width to spare.
                    footer
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            flex_shrink: 0.0,
                            ..default()
                        })
                        .with_children(|reasons| {
                            spawn_reasons(reasons, t, &places.footer, false);
                        });
                    for control in &places.footer {
                        spawn_control(footer, t, control, None);
                    }
                });
        });
}

/// The bar beside a page that scrolls. It keeps its width whether or not it
/// shows, so the page does not move when it appears.
fn scrollbar(t: &Tokens, target: Entity) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Px(t.size.scrollbar_width),
            flex_shrink: 0.0,
            ..default()
        },
        Scrollbar::new(target, ControlOrientation::Vertical, t.size.control_height),
        BackgroundColor(t.colour.panel.0),
        Visibility::Hidden,
        children![(
            ScrollbarThumb {
                border_radius: BorderRadius::all(Val::Px(t.size.radius)),
                border: UiRect::ZERO,
            },
            BackgroundColor(t.colour.control_border.0),
        )],
    )
}

/// A scrollbar shows only while its page does not fit.
fn show_scrollbars(
    mut bars: Query<(&Scrollbar, &mut Visibility)>,
    areas: Query<&ComputedNode, With<ScrollArea>>,
) {
    for (bar, mut visibility) in &mut bars {
        let Ok(area) = areas.get(bar.target) else {
            continue;
        };
        let wanted = if area.content_size().y > area.size().y + 0.5 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// A control that takes the keyboard focus is brought into view, so Tab
/// never lands on something that cannot be seen.
fn follow_focus(mut commands: Commands, focus: Res<InputFocus>) {
    if focus.is_changed()
        && let Some(entity) = focus.get()
    {
        commands.trigger(ScrollIntoView { entity });
    }
}

fn spawn_content(
    page: &mut ChildSpawnerCommands,
    t: &Tokens,
    place: usize,
    tab: usize,
    content: Content,
    horses: usize,
) {
    match content {
        Content::Planned => {
            page.spawn(reason(t, "This page is not built yet."));
        }
        Content::Element => {
            page.spawn((text(t, ""), Live(Fact::Element)));
            page.spawn(reason(
                t,
                "Abilities are not built yet: the party fights with one element.",
            ));
        }
        Content::Gear => {
            page.spawn((text(t, ""), Live(Fact::SelectedStats)));
            // The overview (#285): a column for each horse and a slot on
            // each line, so everything worn is seen without selecting.
            page.spawn(Node {
                column_gap: Val::Px(t.space.s),
                margin: UiRect::top(Val::Px(t.space.s)),
                flex_shrink: 0.0,
                ..default()
            })
            .with_children(|overview| {
                for horse in 0..horses {
                    overview
                        .spawn((
                            Node {
                                flex_grow: 1.0,
                                flex_basis: Val::Px(0.0),
                                min_width: Val::Px(0.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(t.space.s),
                                padding: UiRect::all(Val::Px(t.space.xs)),
                                border: UiRect::all(Val::Px(t.size.border)),
                                border_radius: BorderRadius::all(Val::Px(t.size.radius)),
                                ..default()
                            },
                            BorderColor::all(Color::NONE),
                            Wearer(horse),
                        ))
                        .with_children(|slots| {
                            slots.spawn((text(t, ""), Live(Fact::Wearer(horse))));
                            for slot in Slot::ALL {
                                let slot = slot.index();
                                let piece = Piece::Worn { horse, slot };
                                slots
                                    .spawn((cell(t), SlotRow { place, tab, horse, slot }))
                                    .with_children(|cell| {
                                        cell.spawn((
                                            text(t, ""),
                                            Live(Fact::Worn(horse, slot)),
                                        ));
                                        spawn_pictured(cell, t, piece, |lines| {
                                            lines.spawn((
                                                dim(t, ""),
                                                Live(Fact::WornLine(horse, slot)),
                                            ));
                                        });
                                    });
                            }
                        });
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
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(t.space.s),
                            ..default()
                        })
                        .with_children(|half| {
                            half.spawn((text(t, ""), Live(fact)));
                            // The drop, under the report of the trail that
                            // dropped it: the card the cargo shows (#228).
                            if matches!(fact, Fact::Report) {
                                spawn_carried(half, t, place, Some(tab));
                            }
                        });
                }
            });
        }
    }
}

/// The carried piece as a list shows it: the summary of its card. There
/// only while something is carried.
fn spawn_carried(
    parent: &mut ChildSpawnerCommands,
    t: &Tokens,
    place: usize,
    tab: Option<usize>,
) {
    parent
        .spawn((cell(t), CarriedCell { place, tab }, WhenCarried(true)))
        .with_children(|cell| {
            cell.spawn((text(t, ""), Live(Fact::CarriedName)));
            spawn_pictured(cell, t, Piece::Carried, |lines| {
                lines.spawn((dim(t, ""), Live(Fact::CarriedKind)));
                lines.spawn((dim(t, ""), Live(Fact::CarriedVerdict)));
            });
        });
}

/// What a cell holds under the piece's name: its picture (#286), and beside
/// it the lines that say what it is. The name has the cell's whole width,
/// which the longest one needs at the smallest window (#287).
fn spawn_pictured(
    cell: &mut ChildSpawnerCommands,
    t: &Tokens,
    piece: Piece,
    lines: impl FnOnce(&mut ChildSpawnerCommands),
) {
    cell.spawn(Node {
        align_items: AlignItems::Center,
        column_gap: Val::Px(t.space.s),
        ..default()
    })
    .with_children(|across| {
        across.spawn(picture(t, piece));
        across
            .spawn(Node {
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                min_width: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(t.space.xs),
                ..default()
            })
            .with_children(lines);
    });
}

/// A control of the map, with the key that works it printed on it. One that
/// is not built is drawn as unavailable and can still be pressed.
fn spawn_control(
    parent: &mut ChildSpawnerCommands,
    t: &Tokens,
    control: &Control,
    place: Option<usize>,
) {
    let mut entity = parent.spawn((
        button(t, &control.name, control.key.as_deref()),
        ControlButton {
            id: control.id,
            place,
        },
    ));
    if !control.built() {
        entity.insert(NotBuilt);
    }
}

/// The controls of a page, in the order of the map, wrapping as they must.
/// `hint` is whether the line about those not built says that a press says
/// more, where there is room for it.
fn spawn_controls(
    parent: &mut ChildSpawnerCommands,
    t: &Tokens,
    controls: &[Control],
    place: Option<usize>,
    hint: bool,
) {
    if controls.is_empty() {
        return;
    }
    parent
        .spawn(Node {
            flex_wrap: FlexWrap::Wrap,
            column_gap: Val::Px(t.space.s),
            row_gap: Val::Px(t.space.s),
            margin: UiRect::top(Val::Px(t.space.s)),
            flex_shrink: 0.0,
            ..default()
        })
        .with_children(|row| {
            for control in controls {
                spawn_control(row, t, control, place);
            }
        });
    spawn_reasons(parent, t, controls, hint);
}

/// Under a group of controls, why any of them cannot be used (#233): a line
/// that follows the game for those that work, and a line for those that are
/// not built, which does not change.
fn spawn_reasons(parent: &mut ChildSpawnerCommands, t: &Tokens, controls: &[Control], hint: bool) {
    let working: Vec<usize> = controls
        .iter()
        .filter(|control| control.action.is_some())
        .map(|control| control.id)
        .collect();
    if !working.is_empty() {
        parent.spawn((reason(t, ""), Blocked(working)));
    }
    let unbuilt = unbuilt_line(controls, hint);
    if !unbuilt.is_empty() {
        parent.spawn(reason(t, &unbuilt));
    }
}

/// One observer for every control of the shell: what a press does depends
/// on what the control is.
fn on_activate(
    activate: On<Activate>,
    mut shell: ResMut<Shell>,
    mut game: ResMut<Game>,
    mut focus: ResMut<InputFocus>,
    map: Res<Places>,
    places: Query<&PlaceButton>,
    tabs: Query<&TabButton>,
    horses: Query<&HorseButton>,
    slots: Query<&SlotRow>,
    carried: Query<&CarriedCell>,
    closes: Query<(), With<ClosePanel>>,
    controls: Query<&ControlButton>,
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
                item: Item::Worn {
                    horse: row.horse,
                    slot: row.slot,
                },
                opener: Some(entity),
            },
        );
    } else if let Ok(cell) = carried.get(entity) {
        let tab = cell.tab.unwrap_or(shell.tab(cell.place));
        shell.show_detail(
            cell.place,
            Detail {
                tab,
                item: Item::Carried,
                opener: Some(entity),
            },
        );
    } else if closes.contains(entity) {
        close_detail(&mut shell, &mut focus);
    } else if let Ok(button) = controls.get(entity) {
        match map.control(button.id).action {
            Some(action) => {
                let hand = shell.hand();
                // A card that is open stays on its piece when the piece
                // is moved, so the piece is still the handle (#228), and
                // goes back with it when the move is undone (#230).
                let moved = match action {
                    Action::Equip => game
                        .equipped_at(hand)
                        .map(|to| (hand.open.unwrap_or(Piece::Carried), to)),
                    Action::Undo => game.can(action, hand).ok().and(game.undone_move()),
                    _ => None,
                };
                game.act(action, hand);
                if let Some((from, to)) = moved {
                    shell.piece_moved(from, to);
                }
            }
            // It shows something, or is not built and says so: either way
            // in the side panel of the page it is on.
            None => {
                let place = button.place.unwrap_or(shell.place());
                shell.open(place);
                let tab = shell.tab(place);
                shell.show_detail(
                    place,
                    Detail {
                        tab,
                        item: Item::Control(button.id),
                        opener: Some(entity),
                    },
                );
            }
        }
    }
}

/// The key printed on a place, a horse or a control presses it, wherever the
/// player is; the key printed on a tab chooses it while its place is showing.
/// Space is the one conflict, since a focused button also answers Space:
/// while a control shows keyboard focus, Space presses that control and no
/// other.
fn keys(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    map: Res<Places>,
    shell: Res<Shell>,
    places: Query<(Entity, &PlaceButton)>,
    tabs: Query<(Entity, &TabButton)>,
    horses: Query<(Entity, &HorseButton)>,
    controls: Query<(Entity, &ControlButton)>,
) {
    if keys.get_just_pressed().len() == 0 {
        return;
    }
    let down = |key: Option<&str>| {
        key.and_then(key_code)
            .is_some_and(|code| keys.just_pressed(code))
    };
    for (entity, place) in &places {
        if down(map.page_key(place.0)) {
            commands.trigger(Activate { entity });
        }
    }
    for (entity, tab) in &tabs {
        let showing = shell.is_open() && tab.place == shell.place();
        if showing && down(map.tab_key(tab.place, tab.tab)) {
            commands.trigger(Activate { entity });
        }
    }
    // A horse is in the strip and in a column: one press, not one for each.
    for h in 0..map.horse_keys.len() {
        if down(map.horse_key(h)) {
            if let Some((entity, _)) = horses.iter().find(|(_, horse)| horse.0 == h) {
                commands.trigger(Activate { entity });
            }
        }
    }
    let on_a_control = focus_visible.0 && focus.get().is_some();
    // A control of the card is in every place's side panel: one press, not
    // one for each.
    let mut pressed: Vec<usize> = Vec::new();
    for (entity, button) in &controls {
        let key = map.control(button.id).key.as_deref();
        let Some(code) = key.and_then(key_code) else {
            continue;
        };
        if keys.just_pressed(code)
            && !(code == KeyCode::Space && on_a_control)
            && !pressed.contains(&button.id)
        {
            pressed.push(button.id);
            commands.trigger(Activate { entity });
        }
    }
}

/// A control that goes out of sight gives up the keyboard focus (#244), so
/// Enter and Space never press something on a page that is not showing.
fn lose_hidden_focus(mut focus: ResMut<InputFocus>, nodes: Query<&ComputedNode>) {
    if let Some(entity) = focus.get()
        && nodes.get(entity).is_ok_and(|node| node.is_empty())
    {
        focus.clear();
    }
}

/// The arrow keys, and the way each moves the keyboard focus.
const ARROWS: [(KeyCode, CompassOctant); 4] = [
    (KeyCode::ArrowUp, CompassOctant::North),
    (KeyCode::ArrowDown, CompassOctant::South),
    (KeyCode::ArrowLeft, CompassOctant::West),
    (KeyCode::ArrowRight, CompassOctant::East),
];

/// Where the focus goes from `from`: the nearest of `parts` that way, by the
/// rule Bevy's own directional navigation scores with. None when nothing
/// lies that way, and the focus stays where it is.
fn towards(from: Entity, way: CompassOctant, parts: &[FocusableArea]) -> Option<Entity> {
    let origin = parts.iter().find(|part| part.entity == from)?;
    find_best_candidate(origin, way, parts, &AutoNavigationConfig::default())
}

/// An arrow key moves the keyboard focus to the nearest control that way
/// (#229): down a list, along a row of buttons, across to the next column.
/// With no focus in sight, the first press puts it on the first control of
/// the page that is showing, or on the open place in the bar when none is.
fn arrows(
    keys: Res<ButtonInput<KeyCode>>,
    shell: Res<Shell>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    root: Single<Entity, With<ShellRoot>>,
    children: Query<&Children>,
    parts: Query<(&ComputedNode, &UiGlobalTransform), With<Part>>,
    pages: Query<(Entity, &Page)>,
    places: Query<(Entity, &PlaceButton)>,
) {
    let Some(&(_, way)) = ARROWS.iter().find(|(key, _)| keys.just_pressed(*key)) else {
        return;
    };
    // Only what is laid out: a control of a page that is not showing has
    // no size.
    let area = |entity: Entity| {
        let (node, transform) = parts.get(entity).ok()?;
        if node.is_empty() {
            return None;
        }
        let scale = node.inverse_scale_factor();
        let (_, _, at) = transform.to_scale_angle_translation();
        Some(FocusableArea {
            entity,
            position: at * scale,
            size: node.size() * scale,
        })
    };
    let all: Vec<FocusableArea> = children.iter_descendants(*root).filter_map(area).collect();
    let from = focus
        .get()
        .filter(|from| all.iter().any(|part| part.entity == *from));
    let next = match from {
        Some(from) => towards(from, way, &all),
        None => pages
            .iter()
            .find(|(_, page)| match **page {
                Page::Tab(place, tab) => place == shell.place() && tab == shell.tab(place),
                _ => false,
            })
            .and_then(|(page, _)| {
                children
                    .iter_descendants_depth_first(page)
                    .find(|entity| area(*entity).is_some())
            })
            .or_else(|| {
                places
                    .iter()
                    .find(|(_, place)| place.0 == shell.place())
                    .map(|(entity, _)| entity)
            }),
    };
    if let Some(next) = next {
        focus.set(next, FocusCause::Navigated);
    }
    if from.is_some() || next.is_some() {
        focus_visible.0 = true;
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
/// player decides. A detail left open over a cargo column is put away on
/// arriving, since it stands where the cargo is and the party has come
/// back with something to look at there (#287).
fn follow_the_party(
    game: Res<Game>,
    map: Res<Places>,
    mut shell: ResMut<Shell>,
    mut was_in_town: Local<bool>,
) {
    let in_town = game.in_town();
    if in_town != *was_in_town {
        *was_in_town = in_town;
        if in_town {
            for (p, place) in map.pages().enumerate() {
                if place.horse_columns {
                    shell.drop_detail(p);
                }
            }
            let place = shell.place();
            shell.open(place);
        } else {
            shell.close();
        }
    }
    // A card open on the carried piece goes when the piece does: the party
    // has left with it or without it.
    if game.pending_drop().is_none() && shell.shows(Item::Carried) {
        shell.forget(Item::Carried);
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
    mut pages: Query<(&Page, &mut Node, &mut Visibility), Without<ShellRoot>>,
) {
    // A place that is being put away stays in view until it is covered.
    let place_in_view = shell.is_open() || view.progress < 1.0;
    for (page, mut node, mut visibility) in &mut pages {
        let shown = match *page {
            Page::Place(place) => place == shell.place() && place_in_view,
            Page::Tab(place, tab) => tab == shell.tab(place),
            Page::Detail(place) => shell.detail(place).is_some(),
            Page::Cargo(place) => shell.detail(place).is_none(),
            Page::Card(place) => shell
                .detail(place)
                .is_some_and(|detail| detail.item.piece().is_some()),
            Page::RoadView => !place_in_view,
        };
        let wanted = if shown { Display::Flex } else { Display::None };
        if node.display != wanted {
            node.display = wanted;
        }
        // Hidden as well as out of the layout: Tab passes over what is
        // hidden, and stops at what is only out of the layout (#244).
        let wanted = if shown {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
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
            Option<&CarriedCell>,
            Has<Selected>,
        ),
        Or<(
            With<PlaceButton>,
            With<TabButton>,
            With<HorseButton>,
            With<SlotRow>,
            With<CarriedCell>,
        )>,
    >,
    map: Res<Places>,
    planned: Query<(
        Entity,
        &ControlButton,
        Has<Selected>,
        Has<InteractionDisabled>,
    )>,
) {
    for (entity, place, tab, horse, slot, carried, selected) in &controls {
        let wanted = place.is_some_and(|place| place.0 == shell.place() && shell.is_open())
            || tab.is_some_and(|tab| tab.tab == shell.tab(tab.place))
            || horse.is_some_and(|horse| horse.0 == shell.horse())
            || slot.is_some_and(|row| {
                shell.detail(row.place).is_some_and(|detail| {
                    detail.tab == row.tab
                        && detail.item
                            == Item::Worn {
                                horse: row.horse,
                                slot: row.slot,
                            }
                })
            })
            || carried.is_some_and(|cell| {
                shell.detail(cell.place).is_some_and(|detail| {
                    detail.item == Item::Carried && cell.tab.is_none_or(|tab| tab == detail.tab)
                })
            });
        if wanted && !selected {
            commands.entity(entity).insert(Selected);
        } else if !wanted && selected {
            commands.entity(entity).remove::<Selected>();
        }
    }

    // A control that acts is unavailable while its action cannot be done.
    // Any other is chosen while the panel it opened is showing.
    for (entity, button, selected, disabled) in &planned {
        let (chosen, blocked) = match map.control(button.id).action {
            Some(action) => (
                action == Action::Element(game.element),
                game.can(action, shell.hand()).is_err(),
            ),
            None => (
                shell
                    .detail(button.place.unwrap_or(shell.place()))
                    .is_some_and(|detail| detail.item == Item::Control(button.id)),
                false,
            ),
        };
        if chosen && !selected {
            commands.entity(entity).insert(Selected);
        } else if !chosen && selected {
            commands.entity(entity).remove::<Selected>();
        }
        if blocked && !disabled {
            commands.entity(entity).insert(InteractionDisabled);
        } else if !blocked && disabled {
            commands.entity(entity).remove::<InteractionDisabled>();
        }
    }
}

/// Outline the selected horse's column of the overview: the horse a piece
/// would go onto.
fn mark_wearer(
    shell: Res<Shell>,
    tokens: Res<Tokens>,
    mut columns: Query<(&Wearer, &mut BorderColor)>,
) {
    for (wearer, mut border) in &mut columns {
        let wanted = BorderColor::all(if wearer.0 == shell.horse() {
            tokens.colour.accent.0
        } else {
            Color::NONE
        });
        if *border != wanted {
            *border = wanted;
        }
    }
}

/// The carried piece is in its lists while there is one, and what an empty
/// list says is there while there is none.
fn show_carried(
    game: Res<Game>,
    mut parts: Query<(&WhenCarried, &mut Node, &mut Visibility)>,
) {
    let carried = game.pending_drop().is_some();
    for (when, mut node, mut visibility) in &mut parts {
        let shown = when.0 == carried;
        let wanted = if shown { Display::Flex } else { Display::None };
        if node.display != wanted {
            node.display = wanted;
        }
        // Hidden too, so that Tab passes over it (#244).
        let wanted = if shown {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// Say under each group of controls why those that work cannot be used now,
/// and put the line away while all of them can.
fn show_reasons(
    game: Res<Game>,
    shell: Res<Shell>,
    map: Res<Places>,
    mut lines: Query<(&Blocked, &mut Node, &Children)>,
    mut texts: Query<&mut Text>,
) {
    for (blocked, mut node, children) in &mut lines {
        let words = blocked_line(blocked.0.iter().map(|&id| map.control(id)), &game, shell.hand());
        let wanted = if words.is_empty() {
            Display::None
        } else {
            Display::Flex
        };
        if node.display != wanted {
            node.display = wanted;
        }
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child)
                && text.0 != words
            {
                text.0 = words.clone();
            }
        }
    }
}

fn show_facts(
    game: Res<Game>,
    uncollected: Res<Uncollected>,
    shell: Res<Shell>,
    map: Res<Places>,
    live: Query<(Entity, &Live)>,
    children: Query<&Children>,
    values: Query<(), Or<(With<CounterValue>, With<RowValue>)>>,
    mut texts: Query<&mut Text>,
) {
    for (entity, live) in &live {
        let target = if texts.contains(entity) {
            Some(entity)
        } else {
            children
                .iter_descendants(entity)
                .find(|child| values.contains(*child))
        };
        let Some(mut text) = target.and_then(|target| texts.get_mut(target).ok()) else {
            continue;
        };
        let words = fact(&game, &uncollected, &shell, &map, live.0);
        if text.0 != words {
            text.0 = words;
        }
    }
}

/// One stop per fight of a trail of `total`, with `cleared` of them behind
/// the party and the next the one it is at.
fn stops(total: usize, cleared: usize) -> Vec<Stop> {
    (0..total)
        .map(|fight| match fight.cmp(&cleared) {
            std::cmp::Ordering::Less => Stop::Behind,
            std::cmp::Ordering::Equal => Stop::Here,
            std::cmp::Ordering::Greater => Stop::Ahead,
        })
        .collect()
}

/// The trail as the tracker shows it, taken from the trail's plan: in town
/// the next trail, all of it ahead; on the road the one being walked. The
/// last stop is the boss.
fn trail_stops(game: &Game) -> Vec<Stop> {
    match &game.screen {
        Screen::Camp { .. } => vec![Stop::Ahead; game.plan.encounters.len()],
        Screen::Trail(expedition) => stops(
            expedition.plan.encounters.len(),
            match &expedition.outcome {
                Some(outcome) => outcome.cleared,
                None => expedition.encounter,
            },
        ),
    }
}

/// Draw the trail tracker again when the trail, or the party's place on it,
/// has changed.
fn show_trail(
    mut commands: Commands,
    tokens: Res<Tokens>,
    game: Res<Game>,
    tracker: Single<Entity, With<TrailTracker>>,
    mut shown: Local<Vec<Stop>>,
) {
    let stops = trail_stops(&game);
    if *shown == stops {
        return;
    }
    commands
        .entity(*tracker)
        .despawn_children()
        .with_children(|trail| {
            for (fight, stop) in stops.iter().enumerate() {
                trail.spawn(trail_mark(&tokens, *stop, fight + 1 == stops.len()));
            }
        });
    *shown = stops;
}

fn fact(game: &Game, uncollected: &Uncollected, shell: &Shell, map: &Places, fact: Fact) -> String {
    let selected = &game.party[shell.horse().min(game.party.len() - 1)];
    match fact {
        // A coin counts when it reaches the party, not when it drops.
        Fact::Coins => (game.coins_earned() - uncollected.coins).to_string(),
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
        Fact::Wearer(horse) if horse == shell.horse() => {
            format!("{}, selected", game.party[horse].name)
        }
        Fact::Wearer(horse) => game.party[horse].name.clone(),
        Fact::Worn(horse, slot) => match &game.party[horse].gear[slot] {
            Some(gear) => gear.name.clone(),
            None => "nothing worn".into(),
        },
        Fact::WornLine(horse, slot) => match &game.party[horse].gear[slot] {
            // Its picture says which slot; the room goes to what it adds.
            Some(gear) => stats_line(gear.stats),
            None => Slot::ALL[slot].name().into(),
        },
        Fact::Detail(place) => {
            let Some(detail) = shell.detail(place) else {
                return String::new();
            };
            match detail.item.piece() {
                Some(piece) => card::pinned(game, piece, shell.hand()),
                None => {
                    let Item::Control(id) = detail.item else {
                        return String::new();
                    };
                    let control = map.control(id);
                    match control.shows {
                        Some(Shows::Keys) => {
                            let horses: Vec<&str> =
                                game.party.iter().map(|horse| horse.name.as_str()).collect();
                            format!("{}\n{}", control.name, keys_list(map, &horses))
                        }
                        None => not_built(control),
                    }
                }
            }
        }
        // Against the selected horse, the one Equip puts it on.
        Fact::CarriedName => game
            .pending_drop()
            .map_or(String::new(), |gear| gear.name.clone()),
        Fact::CarriedKind => game
            .pending_drop()
            .map_or(String::new(), |gear| card::kind(game, gear)),
        Fact::CarriedVerdict => game
            .pending_drop()
            .map_or(String::new(), |gear| card::verdict(game, selected, gear)),
        Fact::Element => {
            let rules = &game.data.rules;
            format!(
                "Party element: {}\nFire beats Plant, Plant beats Ice, Ice beats Fire \
                 (strong x{}, resisted x{}).",
                game.element.name(),
                rules.strong_multiplier,
                rules.weak_multiplier,
            )
        }
        // No blank line at the end: it would push the controls down.
        Fact::Report => report_text(game).trim_end().into(),
        Fact::ReportSide => report_side(game).trim_end().into(),
        Fact::Footer => match &game.screen {
            Screen::Camp { countdown, .. } => format!(
                // The tracker beside it shows the trail's fights.
                "Ready: level {} trail. Setting out in {:.0}s.",
                game.plan.level,
                countdown.max(0.0).ceil(),
            ),
            Screen::Trail(expedition) => {
                let total = expedition.plan.encounters.len();
                let at = expedition.encounter + 1;
                match expedition.phase {
                    _ if game.secs_to_town().is_some() => "On the way back to town.".into(),
                    Phase::Fighting => format!("On the trail: fighting encounter {at} of {total}."),
                    Phase::Finished => format!("On the trail: all {total} encounters cleared."),
                    _ => format!("On the trail: walking to encounter {at} of {total}."),
                }
            }
        },
    }
}

/// The working controls among these that cannot be used now, gathered by the
/// reason, in the order of the map. The reason is `Game::can`'s, the check
/// `Game::act` makes, so what is shown is why the press would do nothing.
fn blocked<'a>(
    controls: impl IntoIterator<Item = &'a Control>,
    game: &Game,
    hand: Hand,
) -> Vec<(Vec<&'a str>, &'static str)> {
    let mut reasons: Vec<(Vec<&str>, &str)> = Vec::new();
    for control in controls {
        let Some(Err(why)) = control.action.map(|action| game.can(action, hand)) else {
            continue;
        };
        match reasons.iter_mut().find(|(_, reason)| *reason == why) {
            Some((names, _)) => names.push(&control.name),
            None => reasons.push((vec![&control.name], why)),
        }
    }
    reasons
}

/// What a group of controls says in place about those: "Fire, Ice, Plant:
/// in town only." Empty when every working control can be used.
fn blocked_line<'a>(
    controls: impl IntoIterator<Item = &'a Control>,
    game: &Game,
    hand: Hand,
) -> String {
    let lines: Vec<String> = blocked(controls, game, hand)
        .iter()
        .map(|(names, why)| reason_line(names, why))
        .collect();
    lines.join(" ")
}

/// What a group of controls says in place about those of them that are not
/// built. It names them, or, where that is shorter, the ones that work:
/// "All but Equip: not built yet." `hint` adds that a press says more,
/// where there is room for it.
fn unbuilt_line(controls: &[Control], hint: bool) -> String {
    let (working, unbuilt): (Vec<&Control>, Vec<&Control>) =
        controls.iter().partition(|control| control.built());
    if unbuilt.is_empty() {
        return String::new();
    }
    let names = |controls: &[&Control]| {
        let names: Vec<&str> = controls.iter().map(|c| c.name.as_str()).collect();
        names.join(", ")
    };
    let mut out = if working.is_empty() && unbuilt.len() > 1 {
        "None of these is built yet.".to_string()
    } else if !working.is_empty() && working.len() < unbuilt.len() {
        format!("All but {}: not built yet.", names(&working))
    } else {
        format!("{}: not built yet.", names(&unbuilt))
    };
    if hint {
        out += match unbuilt.len() {
            1 => " Press it to see what it will do.",
            _ => " Press one to see what it will do.",
        };
    }
    out
}

/// The controls a reason is about, then the reason, as one sentence.
fn reason_line(names: &[&str], why: &str) -> String {
    let mut letters = why.chars();
    let why: String = match letters.next() {
        Some(first) => first.to_lowercase().chain(letters).collect(),
        None => String::new(),
    };
    format!("{}: {}", names.join(", "), why)
}

/// What a control that is not built says when it is pressed.
fn not_built(control: &Control) -> String {
    let mut out = format!("{}\nNot built yet.\n{}", control.name, control.does);
    if let Some(issue) = control.issue {
        out += &match control.proposed {
            true => format!("\nA proposal, not decided: issue #{issue}."),
            false => format!("\nPlanned: issue #{issue}."),
        };
    }
    out
}

/// What the Keys control shows (#229): every key and what it works, taken
/// from the map, so a key changed in `places.ron` is changed here. Escape,
/// Tab and Enter are not in the file; what they do is in `escape` and Bevy.
fn keys_list(map: &Places, horses: &[&str]) -> String {
    let pair = |key: &str, name: &str| format!("{key} {name}");
    let places: Vec<String> = map
        .pages()
        .filter_map(|page| Some(pair(page.key.as_deref()?, &page.name)))
        .collect();
    // Every place numbers its own tabs, so the keys are listed once.
    let mut tabs: Vec<&str> = Vec::new();
    for key in map
        .pages()
        .flat_map(|page| &page.tabs)
        .filter_map(|tab| tab.key.as_deref())
    {
        if !tabs.contains(&key) {
            tabs.push(key);
        }
    }
    let party: Vec<String> = map
        .horse_keys
        .iter()
        .zip(horses)
        .map(|(key, name)| pair(key, name))
        .collect();
    let actions: Vec<String> = map
        .list
        .iter()
        .filter_map(|control| Some(pair(control.key.as_deref()?, &control.name)))
        .collect();
    [
        format!("Places: {}", places.join(", ")),
        format!("Tabs of a place: {}", tabs.join(", ")),
        format!("Horses: {}", party.join(", ")),
        format!("Actions: {}", actions.join(", ")),
        "Esc: back one level".into(),
        "Arrows: move focus that way".into(),
        "Tab, Shift+Tab: move focus".into(),
        "Enter: press what has focus".into(),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// "No control is silent" (#238): every control says what it does, and
    /// one that does nothing yet names the issue that will make it work.
    #[test]
    fn no_control_is_silent() {
        let places = Places::load();
        assert!(!places.controls().is_empty());
        for (id, control) in places.controls().iter().enumerate() {
            assert_eq!(control.id, id);
            assert!(!control.name.is_empty());
            assert!(!control.does.is_empty(), "{} does nothing", control.name);
            assert!(
                control.built() || control.issue.is_some(),
                "{} neither works nor names its issue",
                control.name
            );
            if !control.built() {
                assert!(not_built(control).contains("Not built yet."));
            }
        }
    }

    /// The party in each state the slice has: in town before any trail, in
    /// town with a drop to equip, and on the road.
    fn states() -> Vec<(&'static str, Game)> {
        let with_a_drop = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let mut on_the_road = Game::new(6);
        on_the_road.act(Action::SetOut, Hand::horse(0));
        vec![
            ("in town", Game::new(6)),
            ("in town with a drop", with_a_drop),
            ("on the road", on_the_road),
        ]
    }

    /// "Nothing is silently unavailable" (#233): in every state, a control
    /// that cannot be used is named in a line under its group, and for one
    /// that works the reason given is the one `Game::can` refuses it with.
    /// A control that can be used is in neither line.
    #[test]
    fn no_unavailable_control_is_without_its_reason() {
        let places = Places::load();
        let grouped: usize = places.groups().iter().map(|group| group.len()).sum();
        assert_eq!(grouped, places.controls().len(), "a control is in no group");
        for (state, game) in states() {
            for group in places.groups() {
                let reasons = blocked(group, &game, Hand::default());
                let unbuilt = unbuilt_line(group, true);
                for control in group {
                    let name = control.name.as_str();
                    let given: Vec<&str> = reasons
                        .iter()
                        .filter(|(names, _)| names.contains(&name))
                        .map(|(_, why)| *why)
                        .collect();
                    match control.action.map(|action| game.can(action, Hand::default())) {
                        Some(Ok(())) => assert!(given.is_empty(), "{state}: {name}"),
                        Some(Err(why)) => {
                            assert_eq!(given, [why], "{state}: {name}");
                            assert!(!why.is_empty());
                            let line = blocked_line(group, &game, Hand::default());
                            assert!(line.contains(name), "{state}: {line}");
                        }
                        // It shows something, whatever the party is doing:
                        // no line says it cannot be used.
                        None if control.built() => {
                            assert!(given.is_empty(), "{state}: {name}");
                            let named = unbuilt.contains(name);
                            assert_eq!(named, unbuilt.starts_with("All but "), "{unbuilt}");
                        }
                        // Named, or covered by a line that names the others.
                        None => {
                            assert!(given.is_empty(), "{state}: {name}");
                            let covered = unbuilt.starts_with("None of these")
                                || unbuilt.starts_with("All but ")
                                || unbuilt.contains(name);
                            assert!(covered, "{state}: {unbuilt}");
                            assert!(unbuilt.contains("built yet"), "{unbuilt}");
                        }
                    }
                }
            }
        }
    }

    /// The trail tracker (#253) has a mark for every fight of the plan,
    /// however long the trail, and every fight left is counted ahead.
    #[test]
    fn the_tracker_has_a_mark_for_every_fight_of_the_plan() {
        use Stop::{Ahead, Behind, Here};
        assert_eq!(stops(4, 1), [Behind, Here, Ahead, Ahead]);
        assert_eq!(stops(1, 0), [Here]);
        for total in 1..9 {
            for cleared in 0..=total {
                let stops = stops(total, cleared);
                assert_eq!(stops.len(), total);
                assert_eq!(stops.iter().filter(|s| **s == Behind).count(), cleared);
                assert!(stops.iter().filter(|s| **s == Here).count() <= 1);
            }
        }

        // In town it is the next trail, none of it walked.
        let mut game = Game::new(6);
        let total = game.plan.encounters.len();
        assert_eq!(trail_stops(&game), vec![Ahead; total]);

        // On the road it follows the party, and a longer trail has more marks.
        game.act(Action::SetOut, Hand::horse(0));
        assert_eq!(trail_stops(&game), stops(total, 0));
        let Screen::Trail(expedition) = &mut game.screen else {
            unreachable!("the party set out");
        };
        expedition.encounter = 2;
        let another = expedition.plan.encounters[0].clone();
        expedition.plan.encounters.push(another);
        assert_eq!(trail_stops(&game), stops(total + 1, 2));

        // A won trail is all behind, the boss too.
        let mut won = Game::new(6);
        won.act(Action::SetOut, Hand::horse(0));
        let Screen::Trail(expedition) = &mut won.screen else {
            unreachable!("the party set out");
        };
        while expedition.outcome.is_none() {
            expedition.tick(&won.data, 0.1);
        }
        let outcome = expedition.outcome.clone().unwrap();
        assert!(outcome.won, "seed 6 wins its first trail");
        assert_eq!(trail_stops(&won), vec![Behind; total]);
    }

    #[test]
    fn a_reason_reads_as_one_sentence() {
        assert_eq!(
            reason_line(&["Fire", "Ice"], "In town only."),
            "Fire, Ice: in town only."
        );
        let mut game = Game::new(6);
        let places = Places::load();
        assert_eq!(blocked_line(&places.footer, &game, Hand::default()), "");
        // Undo (#230) says why it has nothing to do, until something is done.
        let gear = &places.places[0].tabs[0].controls;
        assert_eq!(
            blocked_line(gear, &game, Hand::default()),
            "Undo: nothing changed this visit."
        );
        game.act(Action::Element(crate::sim::data::Element::Ice), Hand::horse(0));
        assert_eq!(blocked_line(gear, &game, Hand::default()), "");
    }

    /// The line about controls that are not built names whichever is the
    /// shorter list, and a control that works is never said to be unbuilt.
    #[test]
    fn the_unbuilt_line_names_the_shorter_list() {
        let places = Places::load();
        for group in places.groups() {
            let line = unbuilt_line(group, false);
            let working = group.iter().filter(|c| c.built());
            if line.starts_with("All but ") {
                for control in working {
                    assert!(line.contains(control.name.as_str()), "{line}");
                }
            } else if !line.starts_with("None of these") {
                for control in working {
                    assert!(!line.contains(control.name.as_str()), "{line}");
                }
            }
        }
        let cargo = &places.places[0].cargo;
        assert_eq!(unbuilt_line(cargo, false), "None of these is built yet.");
        assert!(unbuilt_line(&places.card, false).starts_with("All but Equip: "));
        assert_eq!(
            unbuilt_line(&places.footer, false),
            "Planned changes: not built yet."
        );
    }

    /// An empty list says what belongs in it and where that comes from
    /// (#233): the cargo, an empty slot and the report, before any trail.
    #[test]
    fn an_empty_list_says_what_belongs_in_it() {
        let places = Places::load();
        let game = Game::new(6);
        let uncollected = Uncollected::default();
        let mut shell = Shell::new(places.pages().count());
        shell.show_detail(
            0,
            Detail {
                tab: 0,
                item: Item::Worn { horse: 0, slot: 0 },
                opener: None,
            },
        );
        let say = |what| fact(&game, &uncollected, &shell, &places, what);
        assert!(card::NOTHING_CARRIED.contains("Gear the horses bring back"));
        assert_eq!(say(Fact::CarriedName), "");
        assert!(say(Fact::Detail(0)).contains("Gear comes from the trail"));
        assert!(say(Fact::Report).contains("is reported here"));
    }

    /// A printed key is one the game can read, works a control that works,
    /// and works only that control. The keys of places, horses and controls
    /// work everywhere, so no two of them are the same.
    #[test]
    fn every_key_is_a_real_key_and_works_one_control() {
        let places = Places::load();
        let mut seen = Vec::new();
        for control in places.controls() {
            let Some(key) = &control.key else { continue };
            assert!(
                key_code(key).is_some(),
                "{}: no key called {key}",
                control.name
            );
            assert!(
                control.built(),
                "{} has a key and does nothing",
                control.name
            );
            assert!(!seen.contains(key), "two controls answer {key}");
            seen.push(key.clone());
        }
        let pages = places.pages().filter_map(|page| page.key.as_ref());
        for key in pages.chain(&places.horse_keys) {
            assert!(key_code(key).is_some(), "no key called {key}");
            assert!(!seen.contains(key), "two things answer {key}");
            seen.push(key.clone());
        }
    }

    /// "A key for everything" (#229): every place, settings, every tab and
    /// every horse of the party has a key. A tab's key works in its place
    /// only: no other tab there has it, and nothing that works everywhere.
    #[test]
    fn every_place_tab_and_horse_has_its_key() {
        let places = Places::load();
        let mut everywhere: Vec<&str> = places
            .controls()
            .iter()
            .filter_map(|control| control.key.as_deref())
            .collect();
        everywhere.extend(places.horse_keys.iter().map(String::as_str));
        for (p, page) in places.pages().enumerate() {
            let key = places.page_key(p);
            assert!(key.is_some(), "{} has no key", page.name);
            everywhere.extend(key);
        }
        for (p, page) in places.pages().enumerate() {
            let mut seen = Vec::new();
            for (i, tab) in page.tabs.iter().enumerate() {
                let Some(key) = places.tab_key(p, i) else {
                    panic!("{}, {} has no key", page.name, tab.name);
                };
                assert!(key_code(key).is_some(), "{}: no key called {key}", tab.name);
                assert!(!seen.contains(&key), "{}: two tabs answer {key}", page.name);
                assert!(!everywhere.contains(&key), "{}: {key} is taken", tab.name);
                seen.push(key);
            }
        }
        let party = crate::sim::data::Data::load().horses.len();
        assert!(places.horse_keys.len() >= party, "a horse has no key");
    }

    /// The Keys control lists every key in the file beside what it works
    /// (#229), and is itself a control that works, on the Settings page.
    #[test]
    fn the_keys_control_lists_every_key() {
        let places = Places::load();
        let data = crate::sim::data::Data::load();
        let horses: Vec<&str> = data
            .horses
            .iter()
            .map(|horse| horse.name.as_str())
            .collect();
        let list = keys_list(&places, &horses);
        let line = |start: &str| {
            list.lines()
                .find(|line| line.starts_with(start))
                .unwrap_or_else(|| panic!("no line for {start}"))
        };
        for page in places.pages() {
            let key = page.key.as_deref().unwrap();
            assert!(
                line("Places").contains(&format!("{key} {}", page.name)),
                "{list}"
            );
            for tab in &page.tabs {
                let key = tab.key.as_deref().unwrap();
                assert!(
                    line("Tabs").split([' ', ',']).any(|word| word == key),
                    "{list}"
                );
            }
        }
        for (key, horse) in places.horse_keys.iter().zip(&horses) {
            assert!(line("Horses").contains(&format!("{key} {horse}")), "{list}");
        }
        let mut keyed = 0;
        for control in places.controls() {
            let Some(key) = &control.key else { continue };
            assert!(
                line("Actions").contains(&format!("{key} {}", control.name)),
                "{list}"
            );
            keyed += 1;
        }
        assert!(keyed > 0);

        let shows: Vec<&Control> = places
            .settings
            .controls
            .iter()
            .filter(|c| c.shows == Some(Shows::Keys))
            .collect();
        assert_eq!(shows.len(), 1, "Settings has one Keys control");
        assert!(shows[0].built() && shows[0].action.is_none());

        // Pressed, it says the list in the side panel of Settings.
        let page = places.settings_page();
        let mut shell = Shell::new(places.pages().count());
        shell.show_detail(
            page,
            Detail {
                tab: 0,
                item: Item::Control(shows[0].id),
                opener: None,
            },
        );
        let game = Game::new(6);
        let said = fact(
            &game,
            &Uncollected::default(),
            &shell,
            &places,
            Fact::Detail(page),
        );
        assert!(said.starts_with(&shows[0].name), "{said}");
        assert!(
            said.contains("Space Set out") && !said.contains("Not built"),
            "{said}"
        );
        assert!(said.contains("Arrows:"), "{said}");
    }

    /// The arrow keys (#229): down a list is the next row and up is the one
    /// before, left and right leave the list for what is beside it, and
    /// where nothing lies that way the focus stays.
    #[test]
    fn an_arrow_moves_to_the_nearest_control_that_way() {
        let mut world = World::new();
        let mut part = |x: f32, y: f32, width: f32| FocusableArea {
            entity: world.spawn_empty().id(),
            position: Vec2::new(x, y),
            size: Vec2::new(width, 30.0),
        };
        // Three rows of a list, a button beside the middle one, and a
        // button under the list. Down the screen is a greater y.
        let rows = [
            part(300.0, 100.0, 300.0),
            part(300.0, 130.0, 300.0),
            part(300.0, 160.0, 300.0),
        ];
        let beside = part(600.0, 130.0, 100.0);
        let under = part(200.0, 220.0, 100.0);
        let mut parts = rows.to_vec();
        parts.extend([beside, under]);
        let go = |from: FocusableArea, key: KeyCode| {
            let (_, way) = ARROWS.iter().find(|(arrow, _)| *arrow == key).unwrap();
            towards(from.entity, *way, &parts)
        };
        assert_eq!(go(rows[0], KeyCode::ArrowDown), Some(rows[1].entity));
        assert_eq!(go(rows[1], KeyCode::ArrowDown), Some(rows[2].entity));
        assert_eq!(go(rows[2], KeyCode::ArrowDown), Some(under.entity));
        assert_eq!(go(rows[2], KeyCode::ArrowUp), Some(rows[1].entity));
        assert_eq!(go(rows[1], KeyCode::ArrowUp), Some(rows[0].entity));
        assert_eq!(go(rows[1], KeyCode::ArrowRight), Some(beside.entity));
        assert_eq!(go(beside, KeyCode::ArrowLeft), Some(rows[1].entity));
        assert_eq!(go(rows[0], KeyCode::ArrowUp), None);
        assert_eq!(go(beside, KeyCode::ArrowRight), None);

        // A focus that is not among them moves nowhere.
        let elsewhere = world.spawn_empty().id();
        assert_eq!(towards(elsewhere, CompassOctant::South, &parts), None);
    }

    /// Everything the text overlay did is done from a control: each element,
    /// equipping the drop, and setting out, once each.
    #[test]
    fn everything_the_slice_can_do_has_its_one_control() {
        let places = Places::load();
        let mut actions: Vec<Action> = crate::sim::data::Element::ALL
            .into_iter()
            .map(Action::Element)
            .collect();
        actions.extend([Action::Equip, Action::SetOut, Action::Undo]);
        for action in actions {
            let controls = places
                .controls()
                .iter()
                .filter(|control| control.action == Some(action));
            assert_eq!(controls.count(), 1, "{action:?}");
        }
    }

    /// One name for one thing: on any one page, counting the cargo column
    /// that stays beside it, no two controls share a name.
    #[test]
    fn no_two_controls_of_a_page_share_a_name() {
        for place in Places::load().pages() {
            for (heading, _, _, controls) in place.pages() {
                let mut names: Vec<&str> = controls
                    .iter()
                    .chain(&place.cargo)
                    .map(|control| control.name.as_str())
                    .collect();
                let count = names.len();
                names.sort();
                names.dedup();
                assert_eq!(names.len(), count, "{heading}");
            }
        }
    }

    /// Every page has at least one control planned for it.
    #[test]
    fn every_page_has_its_controls() {
        for place in Places::load().pages() {
            for (heading, _, _, controls) in place.pages() {
                assert!(!controls.is_empty(), "{heading} has no controls");
            }
        }
    }

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
            for (i, (heading, job, _, _)) in pages.iter().enumerate() {
                assert!(!job.is_empty(), "{heading} has no job");
                assert!(
                    pages[i + 1..]
                        .iter()
                        .all(|(other, _, _, _)| other != heading),
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
            item: Item::Worn { horse: 0, slot: 1 },
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
            item: Item::Worn { horse: 0, slot: 2 },
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
