//! Drives the sim and presents it: the camp/trail loop, input and the text HUD.

use bevy::{prelude::*, text::FontSize};

use crate::sim::data::{Data, Element, Stats};
use crate::sim::expedition::{Expedition, Outcome, Phase, Plan};
use crate::sim::loot::{Horse, score};

pub mod menu;
mod scene;

/// Sim step. Fixed, so a run does not depend on the frame rate.
const TICK: f32 = 0.1;
/// Seconds the camp screen waits before setting out by itself.
const CAMP_SECS: f32 = 12.0;
const LOG_LINES: usize = 8;

pub struct GamePlugin {
    pub seed: u64,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::new(self.seed))
            .add_plugins(scene::ScenePlugin)
            .add_systems(Startup, spawn_hud)
            .add_systems(Update, (input, advance, update_hud).chain());
    }
}

pub enum Screen {
    /// Between runs: last result, the drop, and the next trail.
    Camp {
        last: Option<Outcome>,
        note: String,
        countdown: f32,
    },
    Trail(Expedition),
}

#[derive(Resource)]
pub struct Game {
    pub data: Data,
    pub party: Vec<Horse>,
    pub element: Element,
    pub level: u32,
    pub coins: i32,
    /// The next trail while in camp, the current one while on it.
    pub plan: Plan,
    pub screen: Screen,
    accumulator: f32,
}

impl Game {
    fn new(seed: u64) -> Game {
        let data = Data::load();
        Game {
            party: data.horses.iter().map(Horse::new).collect(),
            element: Element::Fire,
            level: 0,
            coins: 0,
            plan: Plan::generate(&data, seed, 0),
            screen: Screen::Camp {
                last: None,
                note: String::new(),
                countdown: CAMP_SECS,
            },
            accumulator: 0.0,
            data,
        }
    }

    /// Put the last run's drop on the horse it fits. Returns what happened.
    fn equip_drop(&mut self) -> Option<String> {
        let Screen::Camp {
            last: Some(outcome),
            ..
        } = &mut self.screen
        else {
            return None;
        };
        let gear = outcome.drop.take()?;
        let horse = &mut self.party[gear.fits];
        let note = format!("Equipped {} on {}.", gear.name, horse.name);
        let slot = gear.slot.index();
        horse.gear[slot] = Some(gear);
        Some(note)
    }

    /// Whether the pending drop beats what its horse wears in that slot.
    fn drop_is_upgrade(&self) -> bool {
        let Screen::Camp {
            last: Some(outcome),
            ..
        } = &self.screen
        else {
            return false;
        };
        let Some(gear) = &outcome.drop else {
            return false;
        };
        let worn = self.party[gear.fits].gear[gear.slot.index()]
            .as_ref()
            .map_or(Stats::default(), |g| g.stats);
        score(&self.data, gear.stats) > score(&self.data, worn)
    }

    fn depart(&mut self) {
        // Idle first: an unattended camp still takes the upgrade.
        if self.drop_is_upgrade() {
            self.equip_drop();
        }
        self.accumulator = 0.0;
        self.screen = Screen::Trail(Expedition::new(
            &self.data,
            self.plan.clone(),
            &self.party,
            self.element,
        ));
    }

    fn return_to_camp(&mut self, outcome: Outcome) {
        self.coins += outcome.coins;
        if outcome.won {
            self.level += 1;
        }
        self.plan = Plan::generate(&self.data, self.plan.seed.wrapping_add(1), self.level);
        self.screen = Screen::Camp {
            last: Some(outcome),
            note: String::new(),
            countdown: CAMP_SECS,
        };
    }
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    if !matches!(game.screen, Screen::Camp { .. }) {
        return;
    }
    for (key, element) in [
        (KeyCode::Digit1, Element::Fire),
        (KeyCode::Digit2, Element::Ice),
        (KeyCode::Digit3, Element::Plant),
    ] {
        if keys.just_pressed(key) {
            game.element = element;
        }
    }
    if keys.just_pressed(KeyCode::KeyE) {
        if let Some(text) = game.equip_drop() {
            if let Screen::Camp { note, .. } = &mut game.screen {
                *note = text;
            }
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        game.depart();
    }
}

fn advance(time: Res<Time>, mut game: ResMut<Game>) {
    let game = &mut *game;
    let dt = time.delta_secs();
    match &mut game.screen {
        Screen::Camp { countdown, .. } => {
            *countdown -= dt;
            if *countdown <= 0.0 {
                game.depart();
            }
        }
        Screen::Trail(expedition) => {
            game.accumulator += dt;
            while game.accumulator >= TICK && expedition.phase != Phase::Finished {
                game.accumulator -= TICK;
                expedition.tick(&game.data, TICK);
            }
            if let Some(outcome) = expedition.outcome.clone() {
                game.return_to_camp(outcome);
            }
        }
    }
}

#[derive(Component)]
struct Hud;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Hud,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            left: px(14),
            ..default()
        },
    ));
}

fn update_hud(game: Res<Game>, mut hud: Single<&mut Text, With<Hud>>) {
    let text = hud_text(&game);
    if hud.0 != text {
        hud.0 = text;
    }
}

fn stats_line(stats: Stats) -> String {
    let mut parts = Vec::new();
    for (value, label) in [
        (stats.health, "health"),
        (stats.attack, "attack"),
        (stats.defense, "defense"),
        (stats.elemental, "elemental"),
    ] {
        if value != 0 {
            parts.push(format!("+{value} {label}"));
        }
    }
    parts.join("  ")
}

fn hud_text(game: &Game) -> String {
    let rules = &game.data.rules;
    let mut out = format!(
        "HORSE HEROES    seed {}   trail level {}   coins {}\n\
         Fire beats Plant.  Plant beats Ice.  Ice beats Fire.   (strong x{}, resisted x{})\n\n",
        game.plan.seed, game.level, game.coins, rules.strong_multiplier, rules.weak_multiplier,
    );

    match &game.screen {
        Screen::Camp {
            last,
            note,
            countdown,
        } => {
            if let Some(outcome) = last {
                out += &format!(
                    "{}  cleared {}/{} encounters, +{} coins\n\
                     {}: {} strong hits, {} resisted\n",
                    if outcome.won { "VICTORY!" } else { "DEFEAT." },
                    outcome.cleared,
                    outcome.total,
                    outcome.coins,
                    outcome.element.name(),
                    outcome.strong_hits,
                    outcome.resisted_hits,
                );
                if let Some(gear) = &outcome.drop {
                    let horse = &game.party[gear.fits];
                    let worn = match &horse.gear[gear.slot.index()] {
                        Some(worn) => format!("{}  {}", worn.name, stats_line(worn.stats)),
                        None => "nothing".into(),
                    };
                    out += &format!(
                        "\nNEW GEAR: {} [{}, {}]  {}\n\
                         \x20 fits {} the {}, who wears: {}\n\
                         \x20 [E] equip{}\n",
                        gear.name,
                        gear.rarity.name(),
                        gear.slot.name(),
                        stats_line(gear.stats),
                        horse.name,
                        horse.class.name(),
                        worn,
                        if game.drop_is_upgrade() {
                            "  (an upgrade)"
                        } else {
                            ""
                        },
                    );
                }
                if !note.is_empty() {
                    out += &format!("\n{note}\n");
                }
                out += "\n";
            }

            out += "PARTY\n";
            for horse in &game.party {
                let stats = horse.stats();
                out += &format!(
                    "  {:<8} {:<7} health {:<3} attack {:<2} defense {:<2} elemental {}\n",
                    horse.name,
                    horse.class.name(),
                    stats.health,
                    stats.attack,
                    stats.defense,
                    stats.elemental,
                );
            }

            out += "\nNEXT TRAIL\n";
            for (i, encounter) in game.plan.encounters.iter().enumerate() {
                let names: Vec<String> = encounter
                    .iter()
                    .map(|spawn| {
                        format!(
                            "{} ({})",
                            game.data.enemies[spawn.enemy].name,
                            spawn.element.name()
                        )
                    })
                    .collect();
                out += &format!("  {}. {}\n", i + 1, names.join(", "));
            }

            out += &format!(
                "\nELEMENT: {}    [1] Fire  [2] Ice  [3] Plant\n\
                 [Space] set out now - leaving in {:.0}s\n",
                game.element.name(),
                countdown.max(0.0).ceil(),
            );
        }

        Screen::Trail(expedition) => {
            let total = expedition.plan.encounters.len();
            out += &match expedition.phase {
                Phase::Fighting => format!("ENCOUNTER {}/{}", expedition.encounter + 1, total),
                _ => format!(
                    "On the trail to encounter {}/{}",
                    expedition.encounter + 1,
                    total
                ),
            };
            out += &format!("    element: {}\n\nPARTY\n", expedition.element.name());
            for fighter in &expedition.party {
                out += &format!(
                    "  {:<8} {:<7} {:>3}/{}{}\n",
                    fighter.name,
                    fighter.class.name(),
                    fighter.health,
                    fighter.max_health,
                    if fighter.health == 0 { "  down" } else { "" },
                );
            }
            if expedition.ward >= 0.05 {
                out += &format!("  Ward up: damage halved for {:.1}s\n", expedition.ward);
            } else {
                out += "\n";
            }

            out += "\nENEMIES\n";
            for foe in &expedition.foes {
                let multiplier = rules.multiplier(expedition.element, foe.element);
                let matchup = if multiplier > 1.0 {
                    "  <- strong against it"
                } else if multiplier < 1.0 {
                    "  <- resisted"
                } else {
                    ""
                };
                out += &format!(
                    "  {:<15} {:<5} {:>3}/{}{}\n",
                    foe.name,
                    foe.element.name(),
                    foe.health,
                    foe.max_health,
                    matchup,
                );
            }

            out += "\n";
            let start = expedition.log.len().saturating_sub(LOG_LINES);
            for line in &expedition.log[start..] {
                out += &format!("  {line}\n");
            }
        }
    }
    out
}
