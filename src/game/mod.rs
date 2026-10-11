//! Drives the sim and presents it: the camp/trail loop and input. What the
//! player reads and presses is the menu shell (`menu::shell`).

use bevy::prelude::*;
use serde::Deserialize;

use crate::sim::data::{Data, Element, Stats};
use crate::sim::expedition::{Expedition, Outcome, Phase, Plan};
use crate::sim::loot::{Gear, Horse, best_wearer};

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

/// An enemy went down: where its body was, its side and its element, the
/// coins it dropped, the kind of kill it was (the name of its fountain), how
/// it dies (the name of its death), the rarity tier of the gear it dropped if
/// it dropped any, and where the enemies still standing are. `fx` turns it
/// into a death and drops.
#[derive(Message)]
pub struct Killed {
    pub at: Vec3,
    pub size: f32,
    pub element: Element,
    pub death: String,
    pub coins: i32,
    pub fountain: String,
    pub gear: Option<usize>,
    pub standing: Vec<Vec3>,
}

/// The party was driven back from an enemy still standing: where its body
/// is, its side and its element. `fx` has it draw off, so that it is not
/// simply gone.
#[derive(Message)]
pub struct Withdrew {
    pub at: Vec3,
    pub size: f32,
    pub element: Element,
}

/// Coins that have dropped and not yet reached the party: in the air, on
/// the ground, or on their way. `advance` adds a kill's coins as it reports
/// the kill and `fx` takes each piece's share off as it arrives, so the
/// coins counter climbs one arrival at a time.
#[derive(Resource, Default)]
pub struct Uncollected {
    pub coins: i32,
    /// Pieces of kills' drops still in the world, coins and gear alike. `fx`
    /// counts them in as it throws them and out as they arrive.
    pub pieces: usize,
}

/// The calm after the kill: a trail that was won keeps the party standing on
/// the road, with nothing else going on, from the tick its last enemy `fell`
/// until every piece of its drops has reached the party. A lost trail has
/// no fountain to wait for.
pub fn road_is_held(won: bool, fell: bool, pieces: usize) -> bool {
    won && (fell || pieces > 0)
}

pub struct GamePlugin {
    pub seed: u64,
    /// Start with the horses' hair hidden (`--bare`). F5 toggles it.
    pub bare: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::new(self.seed))
            .init_resource::<SceneView>()
            .init_resource::<Uncollected>()
            .add_message::<Killed>()
            .add_message::<Withdrew>()
            .add_plugins(scene::ScenePlugin { bare: self.bare })
            .add_systems(Update, advance);
    }
}

/// Something the player can do from a control of the menus. Which control
/// does which is `action` in `assets/data/places.ron`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Action {
    /// Fight the next trail with this element.
    Element(Element),
    /// Put the last trail's drop on the selected horse.
    EquipDrop,
    SetOut,
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
    /// Seconds walked towards town since a finished trail let the party go.
    /// The trail is over and town has not begun: the menus stay as they are
    /// until the party is seen to arrive (#246).
    arriving: Option<f32>,
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
            arriving: None,
            data,
        }
    }

    /// Put the last run's drop on a horse. Any horse can wear any piece
    /// (#250). Returns what happened.
    fn equip_drop(&mut self, horse: usize) -> Option<String> {
        let Screen::Camp {
            last: Some(outcome),
            ..
        } = &mut self.screen
        else {
            return None;
        };
        let horse = self.party.get_mut(horse)?;
        let gear = outcome.drop.take()?;
        let note = format!("Equipped {} on {}.", gear.name, horse.name);
        let slot = gear.slot.index();
        horse.gear[slot] = Some(gear);
        Some(note)
    }

    /// The horse the pending drop does most for, if it betters any of them.
    pub fn drop_best_wearer(&self) -> Option<usize> {
        best_wearer(&self.data, &self.party, self.pending_drop()?)
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

    /// Whether the party is walking. It stands still in town, to fight, and
    /// while the last fountain of a won trail plays, and the ground and
    /// whatever lies on it stand still with it. It walks between encounters
    /// and on the way back to town.
    pub fn party_moving(&self) -> bool {
        match &self.screen {
            Screen::Camp { .. } => false,
            Screen::Trail(expedition) => {
                self.arriving.is_some() || expedition.phase == Phase::Travelling
            }
        }
    }

    /// Seconds of walking left before the party reaches town, while it is on
    /// its way back from a finished trail. The walk is as long as the one
    /// between two encounters.
    pub fn secs_to_town(&self) -> Option<f32> {
        self.arriving
            .map(|walked| (self.data.rules.travel_secs - walked).max(0.0))
    }

    /// Dev aid: start the wait in town again, so the menu tour can finish
    /// its walk round town before the party leaves by itself.
    pub fn hold_in_town(&mut self) {
        if let Screen::Camp { countdown, .. } = &mut self.screen {
            *countdown = CAMP_SECS;
        }
    }

    /// Whether an action can be done now, and if not, why not in plain
    /// words. `act` uses the same check, so the reason a control shows is
    /// the reason the action would fail.
    pub fn can(&self, action: Action) -> Result<(), &'static str> {
        let Screen::Camp { last, .. } = &self.screen else {
            return Err("In town only.");
        };
        match action {
            Action::Element(_) | Action::SetOut => Ok(()),
            Action::EquipDrop => match last {
                Some(outcome) if outcome.drop.is_some() => Ok(()),
                _ => Err("Nothing to equip."),
            },
        }
    }

    /// Do an action, if it can be done. `horse` is the selected horse, for
    /// an action that is done to one.
    pub fn act(&mut self, action: Action, horse: usize) {
        if self.can(action).is_err() {
            return;
        }
        match action {
            Action::Element(element) => self.element = element,
            Action::EquipDrop => {
                if let Some(text) = self.equip_drop(horse)
                    && let Screen::Camp { note, .. } = &mut self.screen
                {
                    *note = text;
                }
            }
            Action::SetOut => self.depart(),
        }
    }

    /// The last trail's drop, while it waits in town to be equipped.
    pub fn pending_drop(&self) -> Option<&Gear> {
        match &self.screen {
            Screen::Camp {
                last: Some(outcome),
                ..
            } => outcome.drop.as_ref(),
            _ => None,
        }
    }

    fn depart(&mut self) {
        // Idle first: an unattended camp still takes the upgrade, on the
        // horse it does most for.
        if let Some(horse) = self.drop_best_wearer() {
            self.equip_drop(horse);
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
        self.arriving = None;
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

#[cfg(test)]
impl Game {
    /// A game back in town from its first trail, with whatever it dropped.
    pub fn back_from_a_trail(seed: u64) -> Game {
        let mut game = Game::new(seed);
        game.act(Action::SetOut, 0);
        let outcome = loop {
            let Screen::Trail(expedition) = &mut game.screen else {
                unreachable!("the party set out");
            };
            expedition.tick(&game.data, TICK);
            if let Some(outcome) = expedition.outcome.clone() {
                break outcome;
            }
        };
        game.return_to_camp(outcome);
        game
    }
}

pub fn advance(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut uncollected: ResMut<Uncollected>,
    mut killed: MessageWriter<Killed>,
    mut withdrew: MessageWriter<Withdrew>,
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
            let mut fell = false;
            game.accumulator += dt;
            while game.accumulator >= TICK && expedition.phase != Phase::Finished {
                game.accumulator -= TICK;
                expedition.tick(&game.data, TICK);
                for kill in std::mem::take(&mut expedition.kills) {
                    fell = true;
                    uncollected.coins += kill.coins.max(0);
                    killed.write(Killed {
                        at: scene::enemy_centre(kill.foe, kill.max_health),
                        size: scene::enemy_size(kill.max_health),
                        element: kill.element,
                        death: kill.death,
                        coins: kill.coins,
                        gear: kill.gear.as_ref().map(|gear| gear.rarity),
                        fountain: kill.fountain,
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
            if let Some(outcome) = expedition.outcome.clone()
                && !road_is_held(outcome.won, fell, uncollected.pieces)
            {
                let walked = match game.arriving {
                    Some(walked) => walked + dt,
                    None => {
                        // Enemies the party leaves standing are handed on,
                        // not dropped.
                        for (i, foe) in expedition.foes.iter().enumerate() {
                            if foe.health > 0 {
                                withdrew.write(Withdrew {
                                    at: scene::enemy_centre(i, foe.max_health),
                                    size: scene::enemy_size(foe.max_health),
                                    element: foe.element,
                                });
                            }
                        }
                        0.0
                    }
                };
                // The party walks to town before it is in town, so the
                // menus move when it is seen to arrive and not before.
                if walked < game.data.rules.travel_secs {
                    game.arriving = Some(walked);
                } else {
                    game.return_to_camp(outcome);
                }
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
                return "No trail walked yet.\n\
                        What happens on a trail is reported here when the party is back."
                    .into();
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
                out += &format!(
                    "\nNEW GEAR: {} [{}, {}]  {}\n",
                    gear.name,
                    game.data.gear.rarities[gear.rarity].name,
                    gear.slot.name(),
                    stats_line(gear.stats),
                );
                out += &match game.drop_best_wearer() {
                    Some(best) => {
                        let horse = &game.party[best];
                        let worn = match &horse.gear[gear.slot.index()] {
                            Some(worn) => format!("{}  {}", worn.name, stats_line(worn.stats)),
                            None => "nothing".into(),
                        };
                        format!("  does most for {}, who wears: {worn}\n", horse.name)
                    }
                    None => "  no horse is better off in it\n".into(),
                };
            }
            if !note.is_empty() {
                out += &format!("\n{note}\n");
            }
        }

        Screen::Trail(expedition) => {
            let total = expedition.plan.encounters.len();
            out += &match expedition.phase {
                Phase::Fighting => format!("ENCOUNTER {}/{}", expedition.encounter + 1, total),
                Phase::Finished => format!("TRAIL CLEARED {total}/{total}"),
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
                "\nELEMENT: {}\n\
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

#[cfg(test)]
mod tests {
    use super::*;

    /// What the slice can do is done in town, and a refusal says why.
    #[test]
    fn an_action_that_cannot_be_done_says_why() {
        let mut game = Game::new(6);
        assert_eq!(game.can(Action::Element(Element::Ice)), Ok(()));
        assert_eq!(game.can(Action::SetOut), Ok(()));
        assert_eq!(game.can(Action::EquipDrop), Err("Nothing to equip."));

        game.act(Action::Element(Element::Ice), 0);
        assert_eq!(game.element, Element::Ice);
        game.act(Action::EquipDrop, 0);
        assert!(game.in_town(), "a refused action changes nothing");

        game.act(Action::SetOut, 0);
        assert!(!game.in_town());
        for action in [
            Action::Element(Element::Fire),
            Action::EquipDrop,
            Action::SetOut,
        ] {
            assert_eq!(game.can(action), Err("In town only."));
        }
        game.act(Action::Element(Element::Fire), 0);
        assert_eq!(game.element, Element::Ice, "no change on the road");
    }

    /// A press does something exactly when `can` allows it (#233): with a
    /// drop home, Equip is allowed and puts it on the selected horse,
    /// whichever that is (#250); after that it is refused.
    #[test]
    fn what_can_be_done_is_done() {
        let home = || {
            (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some())
                .expect("no seed under 50 brings a drop home")
        };
        for selected in 0..3 {
            let mut game = home();
            assert_eq!(game.can(Action::EquipDrop), Ok(()));
            let gear = game.pending_drop().unwrap();
            let (slot, name) = (gear.slot.index(), gear.name.clone());
            game.act(Action::EquipDrop, selected);
            assert!(game.pending_drop().is_none());
            for (index, horse) in game.party.iter().enumerate() {
                let worn = horse.gear[slot].as_ref().map(|gear| gear.name.as_str());
                assert_eq!(worn, (index == selected).then_some(name.as_str()));
            }
            assert_eq!(game.can(Action::EquipDrop), Err("Nothing to equip."));
        }

        let mut game = home();
        assert_eq!(game.can(Action::SetOut), Ok(()));
        game.act(Action::SetOut, 0);
        assert!(!game.in_town());
    }

    /// The road is held for a won trail's last drops and for nothing else.
    #[test]
    fn the_road_is_held_until_the_last_drops_are_in() {
        // The tick the last enemy falls, before `fx` has thrown anything.
        assert!(road_is_held(true, true, 0));
        assert!(road_is_held(true, false, 3));
        assert!(!road_is_held(true, false, 0));
        // A lost trail goes home at once, whatever still lies on the road.
        assert!(!road_is_held(false, true, 0));
        assert!(!road_is_held(false, false, 3));
    }
}
