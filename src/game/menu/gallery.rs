//! Dev screen: every menu part in every state, side by side, with a live set
//! underneath to hover, press and tab through. Opened with `--parts` or F2.
//! This is where the tokens in `assets/data/menu.ron` are tuned.

use bevy::{
    input_focus::tab_navigation::{TabGroup, TabIndex},
    prelude::*,
    ui::{InteractionDisabled, Selected},
    ui_widgets::Activate,
};

use super::parts::{
    self, ConfirmAction, ConfirmCancel, CounterValue, Part, ShowLook, Stop, button, card,
    confirmation, counter, dim, panel, reason, row, scrim, side_panel, tab, text, title,
    trail_mark,
};
use super::tokens::Tokens;

const LABEL_WIDTH: f32 = 80.0;
const CELL_WIDTH: f32 = 184.0;

pub struct GalleryPlugin {
    pub open: bool,
}

impl Plugin for GalleryPlugin {
    fn build(&self, app: &mut App) {
        if self.open {
            app.add_systems(Startup, open);
        }
        app.add_systems(Update, toggle).add_observer(close_modal);
    }
}

#[derive(Component)]
struct Gallery;

/// A confirmation opened from the live set.
#[derive(Component)]
struct Modal;

/// The live counter's number.
#[derive(Component)]
struct LiveCoins(u64);

fn open(mut commands: Commands, tokens: Res<Tokens>) {
    spawn(&mut commands, &tokens);
}

fn toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    tokens: Res<Tokens>,
    gallery: Query<Entity, With<Gallery>>,
) {
    if !keys.just_pressed(KeyCode::F2) {
        return;
    }
    match gallery.single() {
        Ok(entity) => commands.entity(entity).despawn(),
        Err(_) => spawn(&mut commands, &tokens),
    }
}

fn line(gap: f32) -> Node {
    Node {
        column_gap: Val::Px(gap),
        align_items: AlignItems::Center,
        ..default()
    }
}

fn cell(width: f32) -> Node {
    Node {
        width: Val::Px(width),
        flex_shrink: 0.0,
        ..default()
    }
}

fn sample<'a>(
    parent: &'a mut ChildSpawnerCommands,
    t: &Tokens,
    part: Part,
) -> EntityCommands<'a> {
    match part {
        Part::Button => parent.spawn(button(t, "Set out", Some("Space"))),
        Part::Tab => parent.spawn(tab(t, "Abilities", Some("2"))),
        Part::Row => parent.spawn(row(t, "Starlit Halter", None, "Head")),
        Part::Card => {
            let mut card = parent.spawn(card(t, "Starlit Halter", "Rare - Head"));
            card.with_children(|lines| {
                lines.spawn(text(t, "+3 attack"));
                lines.spawn(text(t, "+10 health"));
            });
            card
        }
    }
}

fn spawn(commands: &mut Commands, t: &Tokens) {
    let gap = t.space.m;
    commands
        .spawn((
            Gallery,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(t.space.l)),
                row_gap: Val::Px(t.space.m),
                ..default()
            },
            BackgroundColor(t.colour.screen.0),
            GlobalZIndex(10),
            TabGroup::default(),
        ))
        .with_children(|root| {
            root.spawn(line(t.space.l)).with_children(|head| {
                head.spawn(title(t, "Menu parts"));
                head.spawn(dim(
                    t,
                    "every control in every state - tokens: assets/data/menu.ron - [F2] close",
                ));
            });

            // The grid: one row per part, one column per state, drawn in that
            // state whatever the pointer does.
            root.spawn(line(gap)).with_children(|header| {
                header.spawn(cell(LABEL_WIDTH));
                for (name, _) in parts::six() {
                    header.spawn((cell(CELL_WIDTH), children![dim(t, name)]));
                }
            });
            for (name, part) in [
                ("button", Part::Button),
                ("tab", Part::Tab),
                ("list row", Part::Row),
                ("item card", Part::Card),
            ] {
                root.spawn(line(gap)).with_children(|states| {
                    states.spawn((cell(LABEL_WIDTH), children![dim(t, name)]));
                    for (_, look) in parts::six() {
                        states.spawn(cell(CELL_WIDTH)).with_children(|cell| {
                            sample(cell, t, part)
                                .insert(ShowLook(look))
                                .remove::<TabIndex>();
                        });
                    }
                });
            }

            // The parts with no states, and a live set to try.
            root.spawn(Node {
                column_gap: Val::Px(t.space.l),
                align_items: AlignItems::FlexStart,
                margin: UiRect::top(Val::Px(t.space.m)),
                ..default()
            })
            .with_children(|lower| {
                lower.spawn(panel(t)).with_children(|panel| {
                    panel.spawn(dim(t, "panel, counters"));
                    panel.spawn(counter(t, "coins", "7"));
                    panel.spawn(counter(t, "coins", "1,240"));
                    panel.spawn(counter(t, "coins", "9,876,543"));
                    panel.spawn(dim(t, "reason line"));
                    panel.spawn(reason(t, "In town only"));
                    panel.spawn(dim(t, "trail marks: behind, here, ahead, boss"));
                    panel
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(t.space.s),
                            ..default()
                        })
                        .with_children(|trail| {
                            trail.spawn(trail_mark(t, Stop::Behind, false));
                            trail.spawn(trail_mark(t, Stop::Here, false));
                            trail.spawn(trail_mark(t, Stop::Ahead, false));
                            trail.spawn(trail_mark(t, Stop::Ahead, true));
                            trail.spawn(trail_mark(t, Stop::Here, true));
                            trail.spawn(trail_mark(t, Stop::Behind, true));
                        });
                });

                lower
                    .spawn(side_panel(t, "Side panel"))
                    .with_children(|side| {
                        side.spawn(text(t, "Detail about one thing,"));
                        side.spawn(text(t, "beside its list."));
                    });

                lower.spawn(confirmation(
                    t,
                    "Retire Juniper?",
                    "This cannot be taken back.",
                    "Retire Juniper",
                ));

                lower
                    .spawn(panel(t))
                    .insert(Node {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(t.space.m)),
                        row_gap: Val::Px(t.space.m),
                        border: UiRect::all(Val::Px(t.size.border)),
                        ..default()
                    })
                    .with_children(|live| {
                        live.spawn(dim(t, "live - hover, press, Tab, Enter"));
                        live.spawn(line(gap)).with_children(|buttons| {
                            buttons
                                .spawn(button(t, "Add 999 coins", None))
                                .observe(add_coins);
                            buttons.spawn((
                                button(t, "Retire a horse", None),
                                InteractionDisabled,
                            ));
                            buttons
                                .spawn(button(t, "Retire Juniper...", None))
                                .observe(open_modal);
                        });
                        live.spawn((counter(t, "coins", "0"), LiveCoins(0)));
                        live.spawn(reason(t, "Retire a horse: opens at the second town"));
                        live.spawn(line(0.0)).with_children(|tabs| {
                            for (i, (label, key)) in
                                [("Gear", "1"), ("Abilities", "2"), ("Pet", "3"), ("Look", "4")]
                                    .into_iter()
                                    .enumerate()
                            {
                                let mut tab = tabs.spawn(tab(t, label, Some(key)));
                                tab.observe(choose);
                                if i == 0 {
                                    tab.insert(Selected);
                                }
                            }
                        });
                        live.spawn(Node {
                            flex_direction: FlexDirection::Column,
                            ..default()
                        })
                        .with_children(|rows| {
                            for (i, (name, slot)) in [
                                ("Trail-worn Halter", "Head"),
                                ("Fae-touched Blanket", "Body"),
                                ("Starlit Saddle Pad", "Saddle"),
                            ]
                            .into_iter()
                            .enumerate()
                            {
                                let mut row = rows.spawn(row(t, name, None, slot));
                                row.observe(choose);
                                if i == 1 {
                                    row.insert(Selected);
                                }
                            }
                        });
                    });
            });
        });
}

/// The one modal: over everything, and Tab stays inside it.
fn open_modal(_: On<Activate>, mut commands: Commands, tokens: Res<Tokens>) {
    commands.spawn((
        Modal,
        scrim(&tokens),
        GlobalZIndex(20),
        TabGroup::modal(),
        children![confirmation(
            &tokens,
            "Retire Juniper?",
            "This cannot be taken back.",
            "Retire Juniper",
        )],
    ));
}

fn close_modal(
    activate: On<Activate>,
    mut commands: Commands,
    answers: Query<(), Or<(With<ConfirmAction>, With<ConfirmCancel>)>>,
    modals: Query<Entity, With<Modal>>,
) {
    if answers.contains(activate.entity) {
        for modal in &modals {
            commands.entity(modal).despawn();
        }
    }
}

/// Selecting one of a set deselects the rest.
fn choose(
    activate: On<Activate>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
) {
    let Ok(parent) = parents.get(activate.entity) else {
        return;
    };
    for sibling in children.get(parent.parent()).into_iter().flatten() {
        commands.entity(*sibling).remove::<Selected>();
    }
    commands.entity(activate.entity).insert(Selected);
}

fn add_coins(
    _: On<Activate>,
    mut coins: Query<(Entity, &mut LiveCoins)>,
    children: Query<&Children>,
    mut values: Query<&mut Text, With<CounterValue>>,
) {
    for (entity, mut coins) in &mut coins {
        coins.0 += 999;
        for child in children.iter_descendants(entity) {
            if let Ok(mut value) = values.get_mut(child) {
                value.0 = coins.0.to_string();
            }
        }
    }
}
