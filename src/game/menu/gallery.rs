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
    self, ConfirmAction, ConfirmCancel, CounterValue, Look, Part, Pointed, ShowLook, Stop, Unseen,
    button, card, confirmation, counter, dim, hint, panel, reason, row, scrim, side_panel,
    small_button, tab, text, title, trail_mark,
};
use super::card::{Rows, table};
use super::ink;
use super::picture;
use super::tokens::Tokens;
use crate::sim::data::{Data, GearTable, Slot, Stats};

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

/// `words` as the shell shows a live text: what comes before the first
/// colour, and each run after it a span in its colour (`ink`).
fn inked(parent: &mut ChildSpawnerCommands, t: &Tokens, gear: &GearTable, words: &str) {
    let mut rest = ink::runs(words, &ink::Names::of(gear));
    let first = match rest.first() {
        Some((_, None)) => rest.remove(0).0,
        _ => "",
    };
    parent.spawn(text(t, first)).with_children(|runs| {
        for (run, ink) in rest {
            let colour = ink.map_or(t.colour.text.0, |ink| t.colour.ink(ink, gear));
            let font = TextFont {
                font_size: FontSize::Px(t.text.body),
                ..default()
            };
            runs.spawn((TextSpan::new(run), font, TextColor(colour)));
        }
    });
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
                    // The attention mark (#232), beside and with the selected
                    // mark: a dot and a bar.
                    panel.spawn(dim(t, "attention mark: plain, unseen, unseen and selected"));
                    panel.spawn(line(0.0)).with_children(|tabs| {
                        let selected = Look {
                            selected: true,
                            ..default()
                        };
                        tabs.spawn((tab(t, "Trade", Some("T")), ShowLook(Look::default())))
                            .remove::<TabIndex>();
                        tabs.spawn((tab(t, "Journal", Some("J")), ShowLook(Look::default()), Unseen))
                            .remove::<TabIndex>();
                        tabs.spawn((tab(t, "Report", Some("1")), ShowLook(selected), Unseen))
                            .remove::<TabIndex>();
                    });
                    // Every picture a piece of gear can have (#286).
                    panel.spawn(dim(t, "gear pictures: empty, then each rarity"));
                    let gear = Data::load().gear;
                    for slot in Slot::ALL {
                        panel
                            .spawn(Node {
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(t.space.s),
                                ..default()
                            })
                            .with_children(|tiers| {
                                let tiers_of = std::iter::once(None).chain(gear.rarities.iter().map(Some));
                                for tier in tiers_of {
                                    tiers.spawn(picture::square(t, t.size.picture)).with_children(|square| {
                                        picture::draw(square, t, gear.picture(slot), tier, t.size.picture);
                                    });
                                }
                                tiers.spawn(dim(t, slot.name()));
                            });
                    }
                });

                // The side panel and the one modal, and under them the hint.
                lower
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(t.space.l),
                        flex_shrink: 0.0,
                        ..default()
                    })
                    .with_children(|middle| {
                        middle
                            .spawn(Node {
                                column_gap: Val::Px(t.space.l),
                                align_items: AlignItems::FlexStart,
                                ..default()
                            })
                            .with_children(|pair| {
                                pair
                                    .spawn(side_panel(t, "Side panel", t.size.side_panel_width))
                                    .with_children(|side| {
                                        side.spawn(text(t, "Detail about one thing,"));
                                        side.spawn(text(t, "beside its list."));
                                    });

                                pair.spawn(confirmation(
                                    t,
                                    "Retire Juniper?",
                                    "This cannot be taken back.",
                                    "Retire Juniper",
                                ));
                            });
                        // The hint (#291): its line, its two controls in every
                        // state, and its mark on the control it is about.
                        middle
                            .spawn(panel(t))
                            .insert(Node {
                                width: Val::Px(t.size.side_panel_width + t.size.confirm_width + t.space.l),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(t.space.m)),
                                row_gap: Val::Px(t.space.s),
                                border: UiRect::all(Val::Px(t.size.border)),
                                ..default()
                            })
                            .with_children(|hints| {
                                hints.spawn(dim(t, "the hint: with a key, without one, and a line too long for its room"));
                                hints.spawn(hint(t, "Equip", Some("E"), "Put the open piece on the selected horse."));
                                hints.spawn(hint(t, "Sable", None, "A press opens everything about this horse."));
                                hints.spawn(hint(
                                    t,
                                    "Journal",
                                    Some("J"),
                                    "A line too long for its room is cut where the room ends, and the two controls that put it away keep their places at the far end of it.",
                                ));
                                hints.spawn(dim(t, "its controls, in each state"));
                                hints.spawn(line(gap)).with_children(|states| {
                                    for (_, look) in parts::six() {
                                        states
                                            .spawn((small_button(t, "Got it", Some("Esc")), ShowLook(look)))
                                            .remove::<TabIndex>();
                                    }
                                });
                                hints.spawn(dim(
                                    t,
                                    "its mark on the control it is about: default, with the attention mark, a row, selected, pressed",
                                ));
                                hints.spawn(line(gap)).with_children(|marked| {
                                    let selected = Look {
                                        selected: true,
                                        ..default()
                                    };
                                    let pressed = Look {
                                        hovered: true,
                                        pressed: true,
                                        ..default()
                                    };
                                    marked
                                        .spawn((button(t, "Equip", Some("E")), ShowLook(Look::default()), Pointed))
                                        .remove::<TabIndex>();
                                    marked
                                        .spawn((tab(t, "Journal", Some("J")), ShowLook(Look::default()), Pointed, Unseen))
                                        .remove::<TabIndex>();
                                    marked.spawn(cell(150.0)).with_children(|cell| {
                                        cell.spawn((row(t, "Sable", Some("Z"), "35/35"), ShowLook(Look::default()), Pointed))
                                            .remove::<TabIndex>();
                                    });
                                    marked
                                        .spawn((tab(t, "Gear", Some("1")), ShowLook(selected), Pointed))
                                        .remove::<TabIndex>();
                                    marked
                                        .spawn((button(t, "Equip", Some("E")), ShowLook(pressed), Pointed))
                                        .remove::<TabIndex>();
                                });
                            });
                    });

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

            // A change that is a gain, a loss and neither (#313), in a
            // comparison and in a verdict, at the comparison's width.
            root.spawn(panel(t))
                .insert(Node {
                    width: Val::Px(t.size.compare_width),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(t.space.m)),
                    row_gap: Val::Px(t.space.s),
                    border: UiRect::all(Val::Px(t.size.border)),
                    ..default()
                })
                .with_children(|compared| {
                    compared.spawn(dim(t, "a comparison: gain, loss, same; then changed only"));
                    let adds = |health, attack, defense, elemental| Stats { health, attack, defense, elemental };
                    let gear = Data::load().gear;
                    for words in [
                        "health 35 to 45 (+10)\nattack 5 to 3 (-2)".to_string(),
                        table(adds(5, 3, 4, 2), adds(15, 1, 0, 2), Rows::All),
                        table(adds(5, 3, 4, 2), adds(15, 1, 0, 2), Rows::Changed),
                        "Upgrade for Sable: +11\nNo better for Sable\nWorse for Sable: -3".to_string(),
                    ] {
                        inked(compared, t, &gear, &words);
                    }
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
