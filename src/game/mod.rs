//! Drives the sim and presents it: the camp/trail loop and input. What the
//! player reads and presses is the menu shell (`menu::shell`).

use bevy::{
    input_focus::{InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::sim::data::{Data, Element, Stats};
use crate::sim::expedition::{Expedition, Outcome, Phase, Plan};
use crate::sim::loot::{Horse, score};

pub mod menu;
pub mod scene;

/// Sim step. Fixed, so a run does not depend on the frame rate.
const TICK: f32 = 0.1;
/// Seconds the camp screen waits before setting out by itself.
const CAMP_SECS: f32 = 12.0;
const LOG_LINES: usize = 8;

/// Which of the two layouts the window is in, and how far between them: 0 is
/// the town layout, with the scene in the top third, and 1 the road's, with
/// the scene expanded. The menu shell moves it; the scene and the shell both
/// lay themselves out from it, so they cannot disagree. The shares are in
/// `assets/data/menu.ron`.
#[derive(Resource, Default)]
pub struct SceneView {
    /// Linear progress, 0 to 1. Read `expanded` for the eased value.
    pub progress: f32,
}

impl SceneView {
    /// Eased, so the scene starts and stops moving gently.
    pub fn expanded(&self) -> f32 {
        let t = self.progress.clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Move towards one layout at a steady rate, taking `secs` end to end.
    pub fn step_towards(&mut self, expanded: bool, dt: f32, secs: f32) {
        let step = if secs > 0.0 { dt / secs } else { 1.0 };
        self.progress = if expanded {
            (self.progress + step).min(1.0)
        } else {
            (self.progress - step).max(0.0)
        };
    }
}

/// An enemy went down: where its body was, the coins it dropped, the rarity
/// tier of the gear it dropped if it dropped any, and where the enemies
/// still standing are. `fx` turns it into drops.
#[derive(Message)]
pub struct Killed {
    pub at: Vec3,
    pub coins: i32,
    pub gear: Option<usize>,
    pub standing: Vec<Vec3>,
}

/// Coins that have dropped and not yet reached the party: in the air, on
/// the ground, or on their way. `advance` adds a kill's coins as it reports
/// the kill and `fx` takes each piece's share off as it arrives, so the
/// coins counter climbs one arrival at a time.
#[derive(Resource, Default)]
pub struct Uncollected {
    pub coins: i32,
}

pub struct GamePlugin {
    pub seed: u64,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::new(self.seed))
            .init_resource::<SceneView>()
            .init_resource::<Uncollected>()
            .add_message::<Killed>()
            .add_plugins(scene::ScenePlugin)
            .add_systems(Update, (input, advance).chain());
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

    /// Every coin the party has earned, the current trail's included.
    pub fn coins_earned(&self) -> i32 {
        match &self.screen {
            Screen::Trail(expedition) => self.coins + expedition.coins,
            Screen::Camp { .. } => self.coins,
        }
    }

    pub fn in_town(&self) -> bool {
        matches!(self.screen, Screen::Camp { .. })
    }

    /// Whether the party is walking. It stands still to fight, and the
    /// ground and whatever lies on it stand still with it.
    pub fn party_moving(&self) -> bool {
        !matches!(&self.screen, Screen::Trail(expedition) if expedition.phase == Phase::Fighting)
    }

    /// Dev aid: start the wait in town again, so the menu tour can finish
    /// its walk round town before the party leaves by itself.
    pub fn hold_in_town(&mut self) {
        if let Screen::Camp { countdown, .. } = &mut self.screen {
            *countdown = CAMP_SECS;
        }
    }

    /// The departure action. Does nothing on the road.
    pub fn set_out(&mut self) {
        if self.in_town() {
            self.depart();
        }
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

fn input(
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    mut game: ResMut<Game>,
) {
    if !game.in_town() {
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
    // Space sets out, except while a control shows keyboard focus: there
    // Space presses that control, and must not also send the party off.
    let on_a_control = focus_visible.0 && focus.get().is_some();
    if keys.just_pressed(KeyCode::Space) && !on_a_control {
        game.set_out();
    }
}

fn advance(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut uncollected: ResMut<Uncollected>,
    mut killed: MessageWriter<Killed>,
) {
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
                for kill in std::mem::take(&mut expedition.kills) {
                    uncollected.coins += kill.coins.max(0);
                    killed.write(Killed {
                        at: scene::enemy_centre(kill.foe, kill.max_health),
                        coins: kill.coins,
                        gear: kill.gear.as_ref().map(|gear| gear.rarity),
                        standing: expedition
                            .foes
                            .iter()
                            .enumerate()
                            .filter(|(_, foe)| foe.health > 0)
                            .map(|(i, foe)| scene::enemy_centre(i, foe.max_health))
                            .collect(),
                    });
                }
            }
            if let Some(outcome) = expedition.outcome.clone() {
                game.return_to_camp(outcome);
            }
        }
    }
}

pub fn stats_line(stats: Stats) -> String {
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

/// The latest report: the last trail and its drop in town, the fight on the
/// road. Shown in the Journal, beside `report_side`.
pub fn report_text(game: &Game) -> String {
    let rules = &game.data.rules;
    let mut out = String::new();

    match &game.screen {
        Screen::Camp { last, note, .. } => {
            let Some(outcome) = last else {
                return "No trail walked yet.".into();
            };
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
                    game.data.gear.rarities[gear.rarity].name,
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
            out += &format!("    element: {}\n", expedition.element.name());
            if expedition.ward >= 0.05 {
                out += &format!("Ward up: damage halved for {:.1}s\n", expedition.ward);
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
        }
    }
    out
}

/// Beside the report: the trail ahead and the element to walk it with in
/// town, the log of the fight on the road.
pub fn report_side(game: &Game) -> String {
    let rules = &game.data.rules;
    match &game.screen {
        Screen::Camp { .. } => {
            let mut out = format!(
                "NEXT TRAIL    level {}   seed {}\n",
                game.level, game.plan.seed
            );
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
                 Fire beats Plant, Plant beats Ice, Ice beats Fire\n\
                 (strong x{}, resisted x{})\n",
                game.element.name(),
                rules.strong_multiplier,
                rules.weak_multiplier,
            );
            out
        }
        Screen::Trail(expedition) => {
            let start = expedition.log.len().saturating_sub(LOG_LINES);
            expedition.log[start..].join("\n")
        }
    }
}
