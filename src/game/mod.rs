//! Drives the sim and presents it: the camp/trail loop and input. What the
//! player reads and presses is the menu shell (`menu::shell`).

use bevy::prelude::*;
use serde::Deserialize;

use crate::sim::data::{Data, Element, Stats};
use crate::sim::expedition::{Expedition, Foe, Hit, Outcome, Phase, Plan, Struck};
use crate::sim::lessons::{Event, Lessons};
use crate::sim::loot::{Gear, Horse, roll_gear, swap_worn};
use crate::sim::rng::Rng;

pub mod menu;
pub mod scene;

/// Sim step. Fixed, so a run does not depend on the frame rate.
pub const TICK: f32 = 0.1;
/// Seconds a town visit lasts before the party sets out by itself, while
/// Keep going is on (#305). With it off no town leaves by itself.
pub const CAMP_SECS: f32 = 12.0;

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
    pub gear: Vec<usize>,
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

/// A hit landed (#249): the sim's word for it, and the middle of the body it
/// struck. `fx` shows its number.
#[derive(Message)]
pub struct Landed {
    pub hit: Hit,
    pub at: Vec3,
}

/// An ability was used (#358), by its place in `abilities.ron`. `fx` plays
/// its sound.
#[derive(Message)]
pub struct Cast {
    pub ability: usize,
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
    /// Kills' pieces since launch: thrown, taken by the party, and carried
    /// off by the ground before it could take them. Counts for the soak log
    /// (procedures/overnight-soak.txt); nothing in the game reads them.
    pub thrown: usize,
    pub collected: usize,
    pub left: usize,
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
    /// Start with Keep going on (`--keep-going`), so that a run nobody is
    /// at leaves town at a known time. A new game has it off.
    pub keep_going: bool,
    /// Start with this many pieces carried (`--cargo N`), rolled from the
    /// seed. A new game carries nothing.
    pub cargo: usize,
    /// Start with every horse wearing a piece in each slot (`--worn`),
    /// rolled from the seed, to see worn gear (#359). A new game wears
    /// nothing.
    pub worn: bool,
    /// Start in town instead of on the road (`--town`, and the menu tour
    /// and the click budget's walk, which begin with a town visit).
    pub town: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut game = match self.town {
            true => Game::new(self.seed),
            false => Game::opening(self.seed),
        };
        game.keep_going = self.keep_going;
        let mut rng = Rng::new(self.seed);
        for _ in 0..self.cargo {
            let gear = roll_gear(&game.data, &mut rng, false, 0);
            game.cargo.insert(0, gear);
        }
        if self.worn {
            for horse in 0..game.party.len() {
                for slot in 0..3 {
                    // Rolled until one fits; a slot nothing fits stays empty.
                    let fits = (0..100)
                        .map(|_| roll_gear(&game.data, &mut rng, true, 0))
                        .find(|gear| gear.slot.index() == slot);
                    game.party[horse].gear[slot] = fits;
                }
            }
        }
        app.insert_resource(game)
            .init_resource::<SceneView>()
            .init_resource::<Uncollected>()
            .add_message::<Killed>()
            .add_message::<Withdrew>()
            .add_message::<Landed>()
            .add_message::<Cast>()
            .add_plugins(scene::ScenePlugin { bare: self.bare })
            .add_systems(Update, advance);
    }
}

/// Something the player can do from a control of the menus. Which control
/// does which is `action` in `assets/data/places.ron`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Action {
    /// Give the selected horse this ability, by its place in
    /// `abilities.ron` (#357). Its element comes with it.
    Ability(usize),
    /// Put a piece on the selected horse (#228): the piece whose card is
    /// open, or the one carried when none is. A carried piece is put on; a
    /// piece another horse wears changes places with what the selected
    /// horse wears in that slot.
    Equip,
    SetOut,
    /// Turn Keep going on or off (#305): on, the party leaves town by
    /// itself after the usual time, until it is turned off or a trail is
    /// lost. It stands in for the itinerary (#201) and goes when that ships.
    KeepGoing,
    /// Take back the last change made during this town visit (#230).
    Undo,
    /// Open the report before the one the Journal shows, or the one after
    /// it (#231).
    Earlier,
    Later,
}

/// What raises a mark on a place (#232): something there the player has not
/// seen. Which place it marks is `marks` in `assets/data/places.ron`.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Raise {
    /// The last trail brought back a piece that is still carried. Whether
    /// it is any good is the player's to say (#351).
    NewGear,
    /// The newest report kept is worth opening: its trail was lost (`lost`),
    /// or dropped a piece of at least this rarity, by its name in `gear.ron`.
    /// Only the newest, so a long absence leaves one thing to read, and it is
    /// never more than a press of Earlier away.
    Report {
        #[serde(default)]
        lost: bool,
        #[serde(default)]
        rarity: Option<String>,
    },
}

/// A trail's report, kept for the Journal after the party has moved on
/// (#231): which trail it was, and how it ended, with what it dropped.
#[derive(Clone, Debug, PartialEq)]
pub struct Kept {
    /// The trail's number, counted from the first the party walked.
    pub number: u32,
    pub level: u32,
    pub outcome: Outcome,
}

/// Where a piece of gear is. One card shows it wherever that is (#228).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    /// In the cargo, counted from the newest piece: 0 is the last one a
    /// trail brought back or a horse took off (#352).
    Carried(usize),
    /// On a horse, in a slot. The slot may be empty.
    Worn { horse: usize, slot: usize },
}

/// What an action is done to: the selected horse, and the piece whose card
/// is open in the side panel, when one is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Hand {
    pub horse: usize,
    pub open: Option<Piece>,
}

impl Hand {
    /// A horse selected and nothing open.
    pub fn horse(horse: usize) -> Hand {
        Hand { horse, open: None }
    }
}

/// A change made in town, kept so that it can be taken back (#230): what it
/// was, and everything it could touch as it was before it.
struct Change {
    /// What was done, as Undo says it: "Frost Burst for Juniper".
    what: String,
    party: Vec<Horse>,
    cargo: Vec<Gear>,
    /// Where the change took a piece from and to, when it moved one.
    moved: Option<(Piece, Piece)>,
}

pub enum Screen {
    /// Between runs: last result, the drop, and the next trail.
    Camp {
        last: Option<Outcome>,
        note: String,
        /// Seconds before the party leaves by itself. It runs only while
        /// Keep going is on.
        countdown: f32,
    },
    Trail(Expedition),
}

#[derive(Resource)]
pub struct Game {
    pub data: Data,
    pub party: Vec<Horse>,
    /// Every piece carried and not worn, newest first. Nothing leaves it
    /// but onto a horse (#352); capacity and a keep rule are #34 and #167.
    cargo: Vec<Gear>,
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
    /// The changes made during this town visit, oldest first. Undo takes
    /// back the last; setting out empties it, so nothing reaches back past
    /// a departure (#230).
    changes: Vec<Change>,
    /// How many trails the party has walked.
    trails: u32,
    /// The reports of the trails walked, oldest first, as many as
    /// `rules.reports_kept` (#231).
    reports: Vec<Kept>,
    /// The number of the earlier report the Journal is open at. None is the
    /// latest: the last trail in town, the fight on the road.
    reading: Option<u32>,
    /// The trail whose drop the player has had in view, and the newest
    /// report read. Neither raises a mark again (#232).
    seen_drop: u32,
    seen_report: u32,
    /// Whether the party leaves town by itself (#305). Off in a new game.
    keep_going: bool,
    /// What the game explains, and how far the player is with each lesson
    /// (#290). It is told what happens; nothing here asks it anything, so no
    /// lesson can hold the party up or keep a control from being used.
    pub lessons: Lessons,
}

impl Game {
    fn new(seed: u64) -> Game {
        let data = Data::load();
        Game {
            party: Horse::party(&data),
            cargo: Vec::new(),
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
            changes: Vec::new(),
            trails: 0,
            reports: Vec::new(),
            reading: None,
            seen_drop: 0,
            seen_report: 0,
            keep_going: false,
            lessons: Lessons::load(),
            data,
        }
    }

    /// A new game (#292): the party is already walking out of the gate
    /// towards its first fight, with nothing to press and no wait. The first
    /// town visit is the one after the first trail, with a drop to look at.
    pub fn opening(seed: u64) -> Game {
        let mut game = Game::new(seed);
        game.depart();
        game
    }

    /// The party, its gear and abilities, and the cargo as they are now, to go back to
    /// if what is about to be done is undone.
    fn before(&self, what: String, moved: Option<(Piece, Piece)>) -> Change {
        Change {
            what,
            party: self.party.clone(),
            cargo: self.cargo.clone(),
            moved,
        }
    }

    /// What Equip would do, as Undo says it afterwards.
    fn equip_label(&self, hand: Hand) -> String {
        let name = |horse: usize| self.party.get(horse).map_or("", |horse| horse.name.as_str());
        match hand.open {
            Some(Piece::Worn { horse, .. }) => {
                format!("the swap between {} and {}", name(hand.horse), name(horse))
            }
            open => format!(
                "{} on {}",
                self.piece(open.unwrap_or(Piece::Carried(0)))
                    .map_or("", |gear| gear.name.as_str()),
                name(hand.horse),
            ),
        }
    }

    /// Take back the last change of this town visit: the horses, the cargo
    /// and their abilities are exactly what they were before it.
    fn undo(&mut self) {
        let Some(change) = self.changes.pop() else {
            return;
        };
        self.party = change.party;
        self.cargo = change.cargo;
        if let Screen::Camp { note, .. } = &mut self.screen {
            *note = format!("Taken back: {}.", change.what);
        }
    }

    /// Where Undo would take a piece from and to: the last change's move,
    /// the other way. A card that is open on the piece goes with it.
    pub fn undone_move(&self) -> Option<(Piece, Piece)> {
        let (from, to) = self.changes.last()?.moved?;
        Some((to, from))
    }

    /// Put a piece of the cargo on a horse. Any horse can wear any piece
    /// (#250). What the horse wore there takes the piece's place in the
    /// cargo, so nothing is lost (#352). Returns what happened.
    fn equip_carried(&mut self, horse: usize, at: usize) -> Option<String> {
        // Said of the horse as it is before the piece goes on (#11).
        let note = menu::card::equipped(self.party.get(horse)?, self.cargo.get(at)?);
        let slot = self.cargo[at].slot.index();
        let gear = match self.party[horse].gear[slot].take() {
            Some(worn) => std::mem::replace(&mut self.cargo[at], worn),
            None => self.cargo.remove(at),
        };
        self.party[horse].gear[slot] = Some(gear);
        Some(note)
    }

    /// The open piece and what the selected horse wears in its slot change
    /// places (#285). Returns what happened.
    fn swap(&mut self, hand: Hand) -> Option<String> {
        let Some(Piece::Worn { horse: other, slot }) = hand.open else {
            return None;
        };
        if !swap_worn(&mut self.party, other, hand.horse, slot) {
            return None;
        }
        let wears = |horse: &Horse| match &horse.gear[slot] {
            Some(gear) => gear.name.clone(),
            None => "nothing".into(),
        };
        let (selected, other) = (&self.party[hand.horse], &self.party[other]);
        Some(format!(
            "{} now wears {}, and {} wears {}.",
            selected.name,
            wears(selected),
            other.name,
            wears(other),
        ))
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

    /// The boss, while its fight is on (#360): the place it stands in among
    /// the foes, and the foe. The trail's last encounter is its boss fight,
    /// and the boss the biggest of it left standing.
    pub fn boss(&self) -> Option<(usize, &Foe)> {
        let Screen::Trail(expedition) = &self.screen else {
            return None;
        };
        let last = expedition.encounter + 1 == expedition.plan.encounters.len();
        if !last || expedition.phase != Phase::Fighting || self.arriving.is_some() {
            return None;
        }
        expedition
            .foes
            .iter()
            .enumerate()
            .filter(|(_, foe)| foe.health > 0)
            .max_by_key(|(slot, foe)| (foe.max_health, std::cmp::Reverse(*slot)))
    }

    /// Seconds of walking left before the party reaches town, while it is on
    /// its way back from a finished trail. The walk is as long as the one
    /// between two encounters.
    pub fn secs_to_town(&self) -> Option<f32> {
        self.arriving
            .map(|walked| (self.road_secs() - walked).max(0.0))
    }

    /// Seconds the road between two encounters takes the party, at its
    /// average speed (#355); the walk back to town is as long.
    pub fn road_secs(&self) -> f32 {
        match &self.screen {
            Screen::Trail(expedition) => expedition.road_secs(),
            Screen::Camp { .. } => self.data.rules.travel_secs(self.party.iter().map(|horse| horse.stats().speed)),
        }
    }

    /// Whether Keep going is on: the party leaves town by itself.
    pub fn keep_going(&self) -> bool {
        self.keep_going
    }

    /// Seconds before the party leaves town by itself. None on the road,
    /// and while Keep going is off: it waits for the player.
    pub fn leaving_in(&self) -> Option<f32> {
        match &self.screen {
            Screen::Camp { countdown, .. } if self.keep_going => Some(countdown.max(0.0)),
            _ => None,
        }
    }

    /// Time passing in town. No town leaves by itself (#305): the party
    /// stands at the gate until the player sets out, unless Keep going is
    /// on, and then it leaves after the usual time.
    fn wait_in_town(&mut self, dt: f32) {
        let keep_going = self.keep_going;
        if let Screen::Camp { countdown, .. } = &mut self.screen
            && keep_going
        {
            *countdown -= dt;
            if *countdown <= 0.0 {
                self.depart();
            }
        }
    }

    /// Dev aid: start the wait in town again, so the menu tour can finish
    /// its walk round town before the party leaves by itself.
    pub fn hold_in_town(&mut self) {
        if let Screen::Camp { countdown, .. } = &mut self.screen {
            *countdown = CAMP_SECS;
        }
    }

    /// The reports behind the latest, oldest first. In town the newest one
    /// kept is the latest itself, the last trail's, and is not among them.
    pub fn earlier(&self) -> &[Kept] {
        let latest = usize::from(self.in_town() && !self.reports.is_empty());
        &self.reports[..self.reports.len() - latest]
    }

    /// The earlier report the Journal is open at. None when it shows the
    /// latest, and when the one it was open at is no longer kept.
    pub fn reading(&self) -> Option<&Kept> {
        let number = self.reading?;
        self.earlier().iter().find(|kept| kept.number == number)
    }

    /// Where the Journal is among the earlier reports: the index of the one
    /// it is open at, or one past the last when it shows the latest.
    fn reading_at(&self) -> usize {
        let earlier = self.earlier();
        self.reading()
            .and_then(|open| earlier.iter().position(|kept| kept.number == open.number))
            .unwrap_or(earlier.len())
    }

    /// Where a piece is now, if the party has it: in the cargo, or on the
    /// horse that wears it. Pieces have no identity of their own yet, so a
    /// piece exactly like it counts as it.
    pub fn find(&self, gear: &Gear) -> Option<Piece> {
        if let Some(at) = self.cargo.iter().position(|carried| carried == gear) {
            return Some(Piece::Carried(at));
        }
        let slot = gear.slot.index();
        self.party
            .iter()
            .position(|horse| horse.gear[slot].as_ref() == Some(gear))
            .map(|horse| Piece::Worn { horse, slot })
    }

    /// The best piece dropped by the trail whose report the Journal shows,
    /// where it is now, if the party still has it (#231). None on the road
    /// while the report is the fight's.
    pub fn reported_piece(&self) -> Option<Piece> {
        self.find(self.open_report()?.outcome.best()?)
    }

    /// The report of the last trail the party finished, if it has walked one.
    pub fn last_report(&self) -> Option<&Kept> {
        self.reports.last()
    }

    /// The horse whose ability was resisted the most on the last trail, if
    /// any was (#357): the first of them, when two were alike.
    pub fn most_resisted(&self) -> Option<usize> {
        let deeds = &self.last_report()?.outcome.deeds;
        let most = deeds.iter().map(|deeds| deeds.resisted_hits).max()?;
        (most > 0).then(|| deeds.iter().position(|deeds| deeds.resisted_hits == most))?
    }

    /// The horse a piece was last put on this town visit, while that change
    /// stands, and what it changed there (#298): its stats before that
    /// Equip and as they are now.
    pub fn last_equipped(&self) -> Option<(usize, Stats, Stats)> {
        self.changes.iter().rev().find_map(|change| match change.moved? {
            (_, Piece::Worn { horse, .. }) => {
                let before = change.party.get(horse)?.stats();
                Some((horse, before, self.party.get(horse)?.stats()))
            }
            _ => None,
        })
    }

    /// The kept report the Journal is open at: an earlier one, or in town
    /// the last trail's. None on the road while the report is the fight's.
    fn open_report(&self) -> Option<&Kept> {
        match self.reading() {
            Some(kept) => Some(kept),
            None if self.in_town() => self.reports.last(),
            None => None,
        }
    }

    /// Whether there is something of this kind the player has not seen
    /// (#232). It stops when the thing is seen, and does not come back for
    /// the same thing.
    pub fn wants(&self, raise: &Raise) -> bool {
        match raise {
            Raise::NewGear => {
                let brought = self.last_report().map_or(&[][..], |kept| &kept.outcome.drops);
                self.trails > self.seen_drop
                    && brought.iter().any(|gear| self.cargo.contains(gear))
            }
            Raise::Report { lost, rarity } => {
                let Some(kept) = self.reports.last() else {
                    return false;
                };
                if kept.number <= self.seen_report {
                    return false;
                }
                let tiers = &self.data.gear.rarities;
                let tier = rarity
                    .as_ref()
                    .and_then(|name| tiers.iter().position(|tier| tier.name == *name));
                let dropped = kept.outcome.best().map(|gear| gear.rarity);
                (*lost && !kept.outcome.won)
                    || tier.is_some_and(|tier| dropped.is_some_and(|dropped| dropped >= tier))
            }
        }
    }

    /// The piece in the cargo has been in view.
    pub fn see_drop(&mut self) {
        if self.pending_drop().is_some() {
            self.seen_drop = self.trails;
        }
    }

    /// The report the Journal is open at has been in view.
    pub fn see_report(&mut self) {
        if let Some(number) = self.open_report().map(|kept| kept.number) {
            self.seen_report = self.seen_report.max(number);
        }
    }

    /// Whether an action can be done now, and if not, why not in plain
    /// words. `act` uses the same check, so the reason a control shows is
    /// the reason the action would fail.
    pub fn can(&self, action: Action, hand: Hand) -> Result<(), &'static str> {
        // The Journal is read on the road as in town.
        match action {
            Action::Earlier if self.earlier().is_empty() => return Err("No earlier report."),
            Action::Earlier if self.reading_at() == 0 => {
                return Err("This is the earliest report kept.");
            }
            Action::Later if self.reading().is_none() => {
                return Err("This is the latest report.");
            }
            Action::Earlier | Action::Later => return Ok(()),
            _ => {}
        }
        if !self.in_town() {
            return Err("In town only.");
        }
        match action {
            Action::Ability(ability) if ability >= self.data.abilities.len() => Err("No such ability."),
            Action::Ability(_) if hand.horse >= self.party.len() => Err("No horse is selected."),
            Action::Ability(_)
            | Action::SetOut
            | Action::KeepGoing
            | Action::Earlier
            | Action::Later => Ok(()),
            Action::Undo if self.changes.is_empty() => Err("Nothing changed this visit."),
            Action::Undo => Ok(()),
            Action::Equip => {
                let worn = |horse: usize, slot: usize| {
                    self.party
                        .get(horse)
                        .and_then(|horse| horse.gear.get(slot))
                        .is_some_and(|worn| worn.is_some())
                };
                match hand.open {
                    Some(Piece::Worn { horse, slot }) if horse == hand.horse => {
                        if worn(horse, slot) {
                            Err("The selected horse wears it already.")
                        } else {
                            Err("Nothing is worn there.")
                        }
                    }
                    Some(Piece::Worn { horse, slot }) => {
                        if worn(horse, slot) || worn(hand.horse, slot) {
                            Ok(())
                        } else {
                            Err("Neither horse wears anything there.")
                        }
                    }
                    Some(Piece::Carried(at)) if at < self.cargo.len() => Ok(()),
                    None if !self.cargo.is_empty() => Ok(()),
                    Some(Piece::Carried(_)) | None => Err("Nothing to equip."),
                }
            }
        }
    }

    /// The piece that is somewhere, if anything is there.
    pub fn piece(&self, piece: Piece) -> Option<&Gear> {
        match piece {
            Piece::Carried(at) => self.cargo.get(at),
            Piece::Worn { horse, slot } => self.party.get(horse)?.gear.get(slot)?.as_ref(),
        }
    }

    /// Where the card in the hand would be after Equip: with its piece, on
    /// the selected horse. A card open on an empty slot stays there, since
    /// that is where the selected horse's piece goes. None when Equip
    /// cannot be done.
    pub fn equipped_at(&self, hand: Hand) -> Option<Piece> {
        self.can(Action::Equip, hand).ok()?;
        let slot = match hand.open {
            Some(open @ Piece::Worn { slot, .. }) => {
                if self.piece(open).is_none() {
                    return Some(open);
                }
                slot
            }
            open => self.piece(open.unwrap_or(Piece::Carried(0)))?.slot.index(),
        };
        Some(Piece::Worn {
            horse: hand.horse,
            slot,
        })
    }

    /// What Equip would be done with if a dragged piece were let go on a
    /// horse (#228): on one of its slots, or on the horse itself. None where
    /// it may not go: a slot of another type, the horse that wears it, or
    /// anywhere while Equip cannot be done.
    pub fn drop_hand(&self, piece: Piece, horse: usize, slot: Option<usize>) -> Option<Hand> {
        let fits = self.piece(piece)?.slot.index();
        if slot.is_some_and(|slot| slot != fits) || horse >= self.party.len() {
            return None;
        }
        if matches!(piece, Piece::Worn { horse: wearer, .. } if wearer == horse) {
            return None;
        }
        let hand = Hand {
            horse,
            open: Some(piece),
        };
        self.can(Action::Equip, hand).ok().map(|_| hand)
    }

    /// Let a dragged piece go on a horse: Equip, done with that horse as
    /// the one it is for, so the drag and the button and key are one action
    /// with one Undo. Returns where the piece went from and to.
    pub fn drop_piece(&mut self, piece: Piece, horse: usize, slot: Option<usize>) -> Option<(Piece, Piece)> {
        let hand = self.drop_hand(piece, horse, slot)?;
        let to = self.equipped_at(hand)?;
        self.act(Action::Equip, hand);
        self.lessons.raise(Event::Dragged);
        Some((piece, to))
    }

    /// Do an action, if it can be done. `hand` is what it is done to, for
    /// an action that is done to a horse or a piece.
    pub fn act(&mut self, action: Action, hand: Hand) {
        if self.can(action, hand).is_err() {
            return;
        }
        match action {
            // Choosing the ability it already has is no change to undo.
            Action::Ability(ability) if ability == self.party[hand.horse].ability => {}
            Action::Ability(ability) => {
                let horse = &self.party[hand.horse];
                let what = format!("{} for {}", self.data.abilities[ability].name, horse.name);
                let before = self.before(what, None);
                self.changes.push(before);
                self.party[hand.horse].ability = ability;
                self.lessons.raise(Event::Chose);
            }
            Action::Equip => {
                let from = hand.open.unwrap_or(Piece::Carried(0));
                let moved = self.equipped_at(hand).map(|to| (from, to));
                let before = self.before(self.equip_label(hand), moved);
                let done = match hand.open {
                    Some(Piece::Worn { .. }) => self.swap(hand),
                    Some(Piece::Carried(at)) => self.equip_carried(hand.horse, at),
                    None => self.equip_carried(hand.horse, 0),
                };
                if let Some(text) = done {
                    self.lessons.raise(Event::Equipped);
                    if matches!(hand.open, Some(Piece::Worn { .. })) {
                        self.lessons.raise(Event::Swapped);
                    }
                    self.changes.push(before);
                    if let Screen::Camp { note, .. } = &mut self.screen {
                        *note = text;
                    }
                }
            }
            // By the player's hand, which Keep going's departure is not.
            Action::SetOut => {
                self.lessons.raise(Event::SentOut);
                self.depart();
            }
            // Not a change to undo: a second press takes it back. The
            // usual time is counted from the press that turns it on.
            Action::KeepGoing => {
                self.keep_going = !self.keep_going;
                self.hold_in_town();
            }
            Action::Undo => self.undo(),
            // By its number, so the report being read stays the one being
            // read when the party leaves or arrives.
            Action::Earlier => {
                let at = self.reading_at();
                self.reading = Some(self.earlier()[at - 1].number);
            }
            Action::Later => {
                let at = self.reading_at();
                self.reading = self.earlier().get(at + 1).map(|kept| kept.number);
            }
        }
    }

    /// The newest piece a trail brought back that the party still has, where
    /// it is now: in the cargo, or on the horse that wears it. What a lesson
    /// about a piece is about (#290).
    pub fn newest_piece(&self) -> Option<Piece> {
        self.reports
            .iter()
            .rev()
            .find_map(|kept| kept.outcome.drops.iter().rev().find_map(|gear| self.find(gear)))
    }

    /// The newest piece in the cargo: the last a trail brought back or a
    /// horse took off. It is what a list with room for one piece shows.
    pub fn pending_drop(&self) -> Option<&Gear> {
        self.cargo.first()
    }

    /// Every piece carried and not worn, newest first.
    pub fn cargo(&self) -> &[Gear] {
        &self.cargo
    }

    fn depart(&mut self) {
        // Nothing is put on at the gate (#351): a piece is worn only because
        // the player put it on a horse, and one left alone stays carried.
        self.accumulator = 0.0;
        // The visit is over: nothing done in it can be taken back now.
        self.changes.clear();
        let expedition =
            Expedition::new(&self.data, self.plan.clone(), &self.party);
        self.screen = Screen::Trail(expedition);
        self.lessons.raise(Event::SetOut);
        self.lessons.leave();
    }

    /// What the list of things to try has paid since this was last asked
    /// (#303) goes into the party's count. Returns how many coins that was.
    pub fn bank_rewards(&mut self) -> i32 {
        let coins = self.lessons.collect();
        self.coins += coins;
        coins
    }

    fn return_to_camp(&mut self, outcome: Outcome) {
        self.arriving = None;
        self.lessons.arrive();
        self.coins += outcome.coins;
        // What the trail brought back goes into the cargo, and stays there
        // until the player puts it on a horse (#352).
        // Newest first, so the last piece to drop is first.
        for gear in &outcome.drops {
            self.cargo.insert(0, gear.clone());
        }
        if !outcome.drops.is_empty() {
            self.lessons.raise(Event::Gear);
        }
        if outcome.drops.len() > 1 {
            self.lessons.raise(Event::Several);
        }
        if outcome.won {
            self.level += 1;
        } else {
            // After a failed journey the party waits in town (pillar 2).
            self.keep_going = false;
        }
        // The report is kept: it is in the Journal after the party has
        // moved on, and never has to be read first (#231).
        self.trails += 1;
        self.reports.push(Kept {
            number: self.trails,
            level: self.plan.level,
            outcome: outcome.clone(),
        });
        let over = self.reports.len().saturating_sub(self.data.rules.reports_kept);
        self.reports.drain(..over);
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
        game.act(Action::SetOut, Hand::horse(0));
        let outcome = loop {
            let Screen::Trail(expedition) = &mut game.screen else {
                unreachable!("the party set out");
            };
            step(expedition, &game.data, &mut game.lessons);
            if let Some(outcome) = expedition.outcome.clone() {
                break outcome;
            }
        };
        game.return_to_camp(outcome);
        game
    }

    /// The first of the seeds from 0 whose first trail brings a piece home,
    /// back in town with that trail's own piece alone, as if it were all
    /// the trail dropped: for a test of what is done with one piece, where
    /// every kill's chaff (#352) would only be in the way.
    pub fn back_with_one_piece() -> Game {
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.last_report().is_some_and(|kept| kept.outcome.cleared > 0))
            .expect("no seed under 50 clears an encounter");
        let kept = game.reports.last_mut().unwrap();
        let piece = kept.outcome.trail_piece().cloned().unwrap();
        kept.outcome.drops = vec![piece.clone()];
        if let Screen::Camp { last: Some(outcome), .. } = &mut game.screen {
            outcome.drops = vec![piece.clone()];
        }
        game.cargo = vec![piece];
        game
    }
}

/// One step of a trail. What happened on it is told to the lessons, which
/// are told and never asked (#290).
fn step(expedition: &mut Expedition, data: &Data, lessons: &mut Lessons) {
    expedition.tick(data, TICK);
    for event in expedition.events.drain(..) {
        lessons.raise(event);
    }
}

pub fn advance(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut uncollected: ResMut<Uncollected>,
    mut killed: MessageWriter<Killed>,
    mut withdrew: MessageWriter<Withdrew>,
    mut landed: MessageWriter<Landed>,
    mut cast: MessageWriter<Cast>,
) {
    let game = &mut *game;
    let dt = time.delta_secs();
    match &mut game.screen {
        Screen::Camp { .. } => game.wait_in_town(dt),
        Screen::Trail(expedition) => {
            let mut fell = false;
            game.accumulator += dt;
            while game.accumulator >= TICK && expedition.phase != Phase::Finished {
                game.accumulator -= TICK;
                step(expedition, &game.data, &mut game.lessons);
                for hit in expedition.hits.drain(..) {
                    let at = match hit.struck {
                        Struck::Foe { slot, max_health } => scene::enemy_centre(slot, max_health),
                        Struck::Horse(horse) => scene::horse_centre(horse),
                    };
                    landed.write(Landed { hit, at });
                }
                for ability in expedition.casts.drain(..) {
                    cast.write(Cast { ability });
                }
                for kill in std::mem::take(&mut expedition.kills) {
                    fell = true;
                    uncollected.coins += kill.coins.max(0);
                    killed.write(Killed {
                        at: scene::enemy_centre(kill.foe, kill.max_health),
                        size: scene::enemy_size(kill.max_health),
                        element: kill.element,
                        death: kill.death,
                        coins: kill.coins,
                        gear: kill.gear.iter().map(|gear| gear.rarity).collect(),
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
                if walked < game.road_secs() {
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
    for (label, value) in stats.named() {
        if value != 0 {
            parts.push(format!("+{value} {label}"));
        }
    }
    parts.join("  ")
}

/// A horse's stats as they were and as they are, in a few words (#298):
/// the stat that moved the most, counting health in stat points, as
/// "attack 12 to 15", and how many more moved. Numbers only: never whether
/// it is better.
pub fn change_line(data: &Data, was: Stats, now: Stats) -> String {
    let weight = |label: &str, by: i32| match label {
        "health" => by.abs() as f32 / data.gear.health_per_point.max(1) as f32,
        _ => by.abs() as f32,
    };
    let mut moved: Vec<_> = was
        .named()
        .into_iter()
        .zip(now.named())
        .filter(|((_, was), (_, now))| was != now)
        .map(|((label, was), (_, now))| (label, was, now))
        .collect();
    // The biggest first; of two alike, the first named.
    moved.sort_by(|a, b| weight(b.0, b.2 - b.1).total_cmp(&weight(a.0, a.2 - a.1)));
    match moved.as_slice() {
        [] => "no stat changed".into(),
        [(label, was, now)] => format!("{label} {was} to {now}"),
        [(label, was, now), rest @ ..] => format!("{label} {was} to {now} and {} more", rest.len()),
    }
}

/// The report the Journal is open at: an earlier one that was kept, or the
/// latest, which is the last trail and its drop in town and the fight on the
/// road. Shown beside `report_side`.
pub fn report_text(game: &Game) -> String {
    let mut out = String::new();

    if let Some(kept) = game.reading() {
        let earlier = game.earlier();
        let at = earlier.iter().position(|other| other.number == kept.number);
        out += &format!(
            "EARLIER REPORT {} of {}: trail {}, level {}\n{}",
            at.map_or(0, |at| at + 1),
            earlier.len(),
            kept.number,
            kept.level,
            outcome_lines(&kept.outcome),
        );
        // Where the piece is now is said by its card, under this, while
        // the party has it.
        let dropped = dropped(&kept.outcome);
        out += &match kept.outcome.best() {
            Some(gear) if game.find(gear).is_some() => format!("\nIt dropped {dropped}:\n"),
            Some(gear) => format!(
                "\nIt dropped {dropped}.\nThe party no longer has {}.\n",
                gear.name
            ),
            None => "\nIt dropped no gear.\n".into(),
        };
        return out;
    }

    match &game.screen {
        Screen::Camp { last, note, .. } => {
            let Some(outcome) = last else {
                return "No trail walked yet.\n\
                        What happens on a trail is reported here when the party is back."
                    .into();
            };
            out += &outcome_lines(outcome);
            // The appraisal, with no input (#11). The piece itself is under
            // this, as its card (#228).
            let appraisal = menu::card::appraisal(game);
            if !appraisal.is_empty() {
                out += &format!("\n{appraisal}\n");
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
            out += "\n";
            if expedition.ward >= 0.05 {
                out += &format!("Ward up: damage halved for {:.1}s\n", expedition.ward);
            } else {
                out += "\n";
            }

            out += "\nENEMIES\n";
            for foe in &expedition.foes {
                let matchup = matchup(game, foe.element);
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

/// What a trail dropped, by its best piece: "Starlit Halter", or
/// "14 pieces, the best Starlit Halter".
fn dropped(outcome: &Outcome) -> String {
    match (outcome.drops.len(), outcome.best()) {
        (_, None) => "no gear".into(),
        (1, Some(best)) => best.name.clone(),
        (pieces, Some(best)) => format!("{pieces} pieces, the best {}", best.name),
    }
}

/// How a trail ended, in the two lines every report of it opens with.
fn outcome_lines(outcome: &Outcome) -> String {
    format!(
        "{}  cleared {}/{} encounters, +{} coins, {} gear\n\
         {} strong hits, {} resisted\n",
        if outcome.won { "VICTORY!" } else { "DEFEAT." },
        outcome.cleared,
        outcome.total,
        outcome.coins,
        outcome.drops.len(),
        outcome.strong_hits,
        outcome.resisted_hits,
    )
}

/// The reports kept, newest first, with a mark on the one that is open:
/// where the player is among them (#231).
fn reports_list(game: &Game, open: &Kept) -> String {
    let mut out = format!(
        "REPORTS KEPT, newest first\n  {}\n",
        match &game.screen {
            Screen::Camp { .. } => format!("Latest: trail {}", game.trails),
            Screen::Trail(_) => format!("Latest: trail {}, being walked", game.trails + 1),
        }
    );
    for kept in game.earlier().iter().rev() {
        let outcome = &kept.outcome;
        out += &format!(
            "{} Trail {}  {} {}/{}  +{}{}\n",
            if kept.number == open.number { ">" } else { " " },
            kept.number,
            if outcome.won { "won" } else { "lost" },
            outcome.cleared,
            outcome.total,
            outcome.coins,
            match outcome.best() {
                Some(gear) if outcome.drops.len() > 1 => {
                    format!("  {} +{}", gear.name, outcome.drops.len() - 1)
                }
                Some(gear) => format!("  {}", gear.name),
                None => String::new(),
            },
        );
    }
    out
}

/// Which horses' abilities are strong against an enemy of this element, and
/// which are resisted by it (#357): "  <- strong: Juniper; resisted: Morrow".
fn matchup(game: &Game, element: Element) -> String {
    let rules = &game.data.rules;
    let by = |strong: bool| -> Vec<&str> {
        game.party
            .iter()
            .filter(|horse| {
                let multiplier = rules.multiplier(game.data.abilities[horse.ability].element, element);
                if strong { multiplier > 1.0 } else { multiplier < 1.0 }
            })
            .map(|horse| horse.name.as_str())
            .collect()
    };
    let parts: Vec<String> = [("strong", by(true)), ("resisted", by(false))]
        .into_iter()
        .filter(|(_, names)| !names.is_empty())
        .map(|(said, names)| format!("{said}: {}", names.join(", ")))
        .collect();
    if parts.is_empty() {
        String::new()
    } else {
        format!("  <- {}", parts.join("; "))
    }
}

/// Beside the report: the trail ahead and each horse's ability to walk it with in
/// town, where the log of the fight is on the road; beside an earlier
/// report, the list of those kept.
pub fn report_side(game: &Game) -> String {
    let rules = &game.data.rules;
    if let Some(open) = game.reading() {
        return reports_list(game, open);
    }
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
            out += "\nABILITIES\n";
            for horse in &game.party {
                out += &format!("  {:<8} {}\n", horse.name, game.data.abilities[horse.ability].named());
            }
            out += &format!(
                "Fire beats Plant, Plant beats Ice, Ice beats Fire\n\
                 (strong x{}, resisted x{})\n",
                rules.strong_multiplier,
                rules.weak_multiplier,
            );
            out
        }
        // The log of the fight is where the fight is watched (#306).
        Screen::Trail(_) => "THE FIGHT'S LOG\n\
                             It lies over the road, on the left, while every place is closed."
            .into(),
    }
}

/// The latest `lines` of the fight's log, the newest last: what lies over
/// the road scene (#306). Nothing in town.
pub fn fight_log(game: &Game, lines: usize) -> Vec<&str> {
    match &game.screen {
        Screen::Camp { .. } => Vec::new(),
        Screen::Trail(expedition) => {
            let start = expedition.log.len().saturating_sub(lines);
            expedition.log[start..].iter().map(String::as_str).collect()
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
        let frost = game.data.ability_index("frost_burst");
        assert_eq!(game.can(Action::Ability(frost), Hand::default()), Ok(()));
        let none = game.data.abilities.len();
        assert_eq!(game.can(Action::Ability(none), Hand::default()), Err("No such ability."));
        assert_eq!(game.can(Action::Ability(frost), Hand::horse(3)), Err("No horse is selected."));
        assert_eq!(game.can(Action::SetOut, Hand::default()), Ok(()));
        assert_eq!(game.can(Action::Equip, Hand::default()), Err("Nothing to equip."));

        // An ability is the selected horse's and no other's (#357).
        let before: Vec<usize> = game.party.iter().map(|horse| horse.ability).collect();
        game.act(Action::Ability(frost), Hand::horse(0));
        assert_eq!(game.party[0].ability, frost);
        assert_eq!(game.party[2].ability, before[2]);
        game.act(Action::Equip, Hand::horse(0));
        assert!(game.in_town(), "a refused action changes nothing");

        game.act(Action::SetOut, Hand::horse(0));
        assert!(!game.in_town());
        for action in [
            Action::Ability(0),
            Action::Equip,
            Action::SetOut,
            Action::KeepGoing,
            Action::Undo,
        ] {
            assert_eq!(game.can(action, Hand::default()), Err("In town only."));
        }
        assert_eq!(game.can(Action::Earlier, Hand::default()), Err("No earlier report."));
        game.act(Action::Ability(0), Hand::horse(0));
        assert_eq!(game.party[0].ability, frost, "no change on the road");
    }

    /// A press does something exactly when `can` allows it (#233): with a
    /// drop home, Equip is allowed and puts it on the selected horse,
    /// whichever that is (#250); after that it is refused.
    #[test]
    fn what_can_be_done_is_done() {
        let home = Game::back_with_one_piece;
        for selected in 0..3 {
            let mut game = home();
            assert_eq!(game.can(Action::Equip, Hand::default()), Ok(()));
            let gear = game.pending_drop().unwrap();
            assert_eq!(game.piece(Piece::Carried(0)), Some(gear));
            let (slot, name) = (gear.slot.index(), gear.name.clone());
            // With the carried piece's card open or with none: the same.
            let hand = Hand {
                horse: selected,
                open: (selected > 0).then_some(Piece::Carried(0)),
            };
            let at = Piece::Worn { horse: selected, slot };
            assert_eq!(game.equipped_at(hand), Some(at));
            game.act(Action::Equip, hand);
            assert_eq!(game.piece(at).map(|gear| gear.name.as_str()), Some(name.as_str()));
            assert!(game.pending_drop().is_none());
            for (index, horse) in game.party.iter().enumerate() {
                let worn = horse.gear[slot].as_ref().map(|gear| gear.name.as_str());
                assert_eq!(worn, (index == selected).then_some(name.as_str()));
            }
            assert_eq!(game.can(Action::Equip, Hand::default()), Err("Nothing to equip."));
        }

        let mut game = home();
        assert_eq!(game.can(Action::SetOut, Hand::default()), Ok(()));
        game.act(Action::SetOut, Hand::horse(0));
        assert!(!game.in_town());
    }

    /// Swap (#285): the open piece goes onto the selected horse and what
    /// that horse wore goes back the other way; it needs another horse's
    /// slot open, something worn in it by one of the two, and town.
    #[test]
    fn a_worn_piece_is_swapped_onto_the_selected_horse() {
        let mut game = Game::back_with_one_piece();
        let slot = game.pending_drop().unwrap().slot.index();
        let name = game.pending_drop().unwrap().name.clone();
        game.act(Action::Equip, Hand::horse(0));
        let open = |horse: usize, selected: usize| Hand {
            horse: selected,
            open: Some(Piece::Worn { horse, slot }),
        };

        // With no card open Equip is about the cargo, which is empty now.
        assert_eq!(game.can(Action::Equip, Hand::horse(1)), Err("Nothing to equip."));
        assert_eq!(
            game.can(Action::Equip, open(0, 0)),
            Err("The selected horse wears it already.")
        );
        assert_eq!(game.can(Action::Equip, open(1, 1)), Err("Nothing is worn there."));
        assert_eq!(
            game.can(Action::Equip, open(1, 2)),
            Err("Neither horse wears anything there.")
        );
        assert_eq!(game.equipped_at(open(1, 2)), None);
        assert_eq!(
            game.equipped_at(open(0, 1)),
            Some(Piece::Worn { horse: 1, slot })
        );

        assert_eq!(game.can(Action::Equip, open(0, 1)), Ok(()));
        game.act(Action::Equip, open(0, 1));
        let worn = |game: &Game, horse: usize| {
            game.party[horse].gear[slot].as_ref().map(|gear| gear.name.clone())
        };
        assert_eq!(worn(&game, 0), None);
        assert_eq!(worn(&game, 1), Some(name.clone()));
        // From an empty slot too: the selected horse's piece goes over.
        assert_eq!(game.can(Action::Equip, open(2, 1)), Ok(()));
        assert_eq!(
            game.equipped_at(open(2, 1)),
            Some(Piece::Worn { horse: 2, slot }),
            "a card on an empty slot stays on it"
        );
        game.act(Action::Equip, open(2, 1));
        assert_eq!(worn(&game, 1), None);
        assert_eq!(worn(&game, 2), Some(name));

        game.act(Action::SetOut, Hand::horse(0));
        assert_eq!(game.can(Action::Equip, open(2, 1)), Err("In town only."));
    }

    /// Dragging (#228): a piece may be let go on a horse, or on a slot of
    /// its own type, and nowhere else; doing so is Equip for that horse,
    /// whichever horse is selected, and one Undo takes it back.
    #[test]
    fn a_dragged_piece_goes_where_equip_would_put_it() {
        let mut game = Game::back_with_one_piece();
        let slot = game.pending_drop().unwrap().slot.index();
        let other = (slot + 1) % crate::sim::data::Slot::ALL.len();
        let name = game.pending_drop().unwrap().name.clone();
        let carried = Piece::Carried(0);
        let hand = |horse| Hand { horse, open: Some(carried) };
        assert_eq!(game.drop_hand(carried, 2, Some(slot)), Some(hand(2)));
        assert_eq!(game.drop_hand(carried, 2, None), Some(hand(2)), "on the horse itself");
        assert_eq!(game.drop_hand(carried, 2, Some(other)), None, "a slot of another type");
        assert_eq!(game.drop_hand(carried, 9, None), None, "no such horse");
        assert_eq!(game.drop_hand(Piece::Carried(1), 0, None), None, "nothing there");
        assert_eq!(game.drop_hand(Piece::Worn { horse: 0, slot }, 1, None), None, "an empty slot");

        let worn = Piece::Worn { horse: 2, slot };
        assert_eq!(game.drop_piece(carried, 2, Some(slot)), Some((carried, worn)));
        assert_eq!(game.piece(worn).map(|gear| gear.name.as_str()), Some(name.as_str()));
        assert!(game.pending_drop().is_none());

        // A worn piece may go to another horse, not to its own.
        assert_eq!(game.drop_hand(worn, 2, None), None);
        assert_eq!(game.drop_hand(worn, 0, Some(slot)), Some(Hand { horse: 0, open: Some(worn) }));
        let moved = Piece::Worn { horse: 0, slot };
        assert_eq!(game.drop_piece(worn, 0, None), Some((worn, moved)));
        assert_eq!(game.piece(moved).map(|gear| gear.name.as_str()), Some(name.as_str()));
        assert_eq!(game.piece(worn), None);

        game.act(Action::Undo, Hand::horse(1));
        assert_eq!(game.piece(worn).map(|gear| gear.name.as_str()), Some(name.as_str()));
        game.act(Action::SetOut, Hand::horse(0));
        assert_eq!(game.drop_hand(worn, 0, None), None, "in town only");
    }

    /// Undo (#230) steps back through the changes of a town visit, one at
    /// a time, and the horses, the cargo and the abilities are exactly what
    /// they were. It never reaches back past a departure.
    #[test]
    fn undo_steps_back_through_the_town_visit() {
        type Seen = (Vec<[Option<Gear>; 3]>, Option<Gear>, Vec<usize>);
        let seen = |game: &Game| -> Seen {
            (
                game.party.iter().map(|horse| horse.gear.clone()).collect(),
                game.pending_drop().cloned(),
                game.party.iter().map(|horse| horse.ability).collect(),
            )
        };
        let mut game = Game::back_with_one_piece();
        let slot = game.pending_drop().unwrap().slot.index();
        let nothing = Err("Nothing changed this visit.");
        assert_eq!(game.can(Action::Undo, Hand::default()), nothing);
        assert_eq!(game.undone_move(), None);

        let mut states = vec![seen(&game)];
        let other = (game.party[0].ability + 1) % game.data.abilities.len();
        let swap = Hand {
            horse: 1,
            open: Some(Piece::Worn { horse: 0, slot }),
        };
        for (action, hand) in [
            (Action::Equip, Hand::horse(0)),
            (Action::Ability(other), Hand::horse(0)),
            (Action::Equip, swap),
        ] {
            game.act(action, hand);
            assert_ne!(Some(&seen(&game)), states.last(), "{action:?} changed nothing");
            states.push(seen(&game));
        }
        // What changes nothing leaves nothing to undo: the ability it has
        // already, and an Equip that is refused.
        game.act(Action::Ability(other), Hand::horse(0));
        game.act(Action::Equip, Hand::horse(2));
        assert_eq!(seen(&game), states[3]);

        // The swap took the card from the first horse's slot to the second's.
        let (first, second) = (Piece::Worn { horse: 0, slot }, Piece::Worn { horse: 1, slot });
        assert_eq!(game.undone_move(), Some((second, first)));
        for step in (0..3).rev() {
            assert_eq!(game.can(Action::Undo, Hand::default()), Ok(()));
            if step == 0 {
                assert_eq!(game.undone_move(), Some((first, Piece::Carried(0))));
            }
            game.act(Action::Undo, Hand::default());
            assert_eq!(seen(&game), states[step], "step {step}");
            let Screen::Camp { note, .. } = &game.screen else {
                unreachable!("still in town");
            };
            assert!(note.starts_with("Taken back: "), "{note}");
        }
        assert_eq!(game.can(Action::Undo, Hand::default()), nothing);
        game.act(Action::Undo, Hand::default());
        assert_eq!(seen(&game), states[0], "a press too many does nothing");

        // A piece put on over another is not lost: undo brings the one
        // under it back. And a visit that is over is out of reach.
        game.act(Action::Equip, Hand::horse(0));
        let over = loop {
            game.act(Action::SetOut, Hand::horse(0));
            assert_eq!(game.can(Action::Undo, Hand::default()), Err("In town only."));
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
            assert_eq!(game.can(Action::Undo, Hand::default()), nothing);
            assert!(game.plan.seed < 500, "no later trail drops a piece for that slot");
            match game.pending_drop() {
                Some(gear) if gear.slot.index() == slot => break gear.clone(),
                _ => {}
            }
        };
        // Whichever horse wears a piece there: at least the first did.
        let wearer = (0..game.party.len())
            .find(|&horse| game.party[horse].gear[slot].is_some())
            .expect("the first drop is still worn");
        let under = game.party[wearer].gear[slot].clone();
        game.act(Action::Equip, Hand::horse(wearer));
        assert_eq!(game.party[wearer].gear[slot], Some(over.clone()));
        game.act(Action::Undo, Hand::default());
        assert_eq!(game.party[wearer].gear[slot], under);
        assert_eq!(game.pending_drop(), Some(&over));
    }

    /// Nothing equips by itself (#351). A party that sets out with a piece
    /// carried and nothing pressed wears what it wore before and still
    /// carries the piece, trail after trail, whatever the piece would do
    /// for a horse; and the road's log says nothing of gear at the gate.
    #[test]
    fn setting_out_equips_nothing() {
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let worn = |game: &Game| -> Vec<[Option<Gear>; 3]> {
            game.party.iter().map(|horse| horse.gear.clone()).collect()
        };
        let bare = worn(&game);
        assert!(bare.iter().flatten().all(|slot| slot.is_none()), "nothing is worn at first");
        for by_itself in [false, true] {
            for _ in 0..10 {
                let carried = game.cargo().to_vec();
                assert!(!carried.is_empty());
                if by_itself {
                    // Keep going's own departure, with nobody at it.
                    game.depart();
                } else {
                    game.act(Action::SetOut, Hand::default());
                }
                assert_eq!(worn(&game), bare, "a piece went on at the gate");
                assert_eq!(game.cargo(), carried, "the cargo changed at the gate");
                let Screen::Trail(expedition) = &game.screen else {
                    unreachable!("the party set out");
                };
                assert!(!expedition.log.iter().any(|line| line.starts_with("Equipped ")));
                walk_home(&mut game);
                assert_eq!(worn(&game), bare);
            }
        }
        assert!(game.cargo().len() > 1, "and what the trails brought back is all carried");
    }

    /// The cargo keeps every piece (#352). What a trail brings back is
    /// carried, newest first, until it is put on a horse; setting out loses
    /// nothing; a piece taken off for another takes that one's place in the
    /// cargo; and any carried piece can be put on, not only the newest.
    #[test]
    fn the_cargo_keeps_every_piece() {
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let held = |game: &Game| {
            let worn = game.party.iter().flat_map(|horse| &horse.gear).flatten().count();
            game.cargo().len() + worn
        };
        let mut brought = game.last_report().unwrap().outcome.drops.len();
        assert_eq!(held(&game), brought);
        for _ in 0..2 {
            game.act(Action::SetOut, Hand::horse(0));
            assert_eq!(held(&game), brought, "setting out lost a piece");
            walk_home(&mut game);
            let drops = game.last_report().unwrap().outcome.drops.clone();
            brought += drops.len();
            if !drops.is_empty() {
                assert_eq!(game.pending_drop(), drops.last(), "the newest is first");
                let newest: Vec<&Gear> = drops.iter().rev().collect();
                assert_eq!(game.cargo()[..drops.len()].iter().collect::<Vec<_>>(), newest);
            }
            assert_eq!(held(&game), brought);
        }
        assert!(game.cargo().len() > 1);

        // The oldest carried piece, onto the last horse.
        let carried = game.cargo().len();
        let at = carried - 1;
        let piece = game.cargo()[at].clone();
        let slot = piece.slot.index();
        let worn = game.party[2].gear[slot].clone();
        let hand = Hand { horse: 2, open: Some(Piece::Carried(at)) };
        assert_eq!(game.piece(Piece::Carried(at)), Some(&piece));
        assert_eq!(game.can(Action::Equip, hand), Ok(()));
        assert_eq!(game.equipped_at(hand), Some(Piece::Worn { horse: 2, slot }));
        game.act(Action::Equip, hand);
        assert_eq!(game.party[2].gear[slot], Some(piece.clone()));
        match &worn {
            Some(worn) => assert_eq!(game.cargo().get(at), Some(worn), "it takes the piece's place"),
            None => assert_eq!(game.cargo().len(), carried - 1),
        }
        assert_eq!(held(&game), brought);

        game.act(Action::Undo, Hand::default());
        assert_eq!(game.party[2].gear[slot], worn);
        assert_eq!(game.cargo().len(), carried);
        assert_eq!(game.cargo()[at], piece);

        let past = Hand { horse: 0, open: Some(Piece::Carried(carried)) };
        assert_eq!(game.can(Action::Equip, past), Err("Nothing to equip."));
        assert_eq!(game.piece(Piece::Carried(carried)), None);
    }

    /// A won first trail at seed 6, the tour's, brings back a piece or more
    /// for every enemy on it, and every one of them is carried (#352).
    #[test]
    fn the_first_trail_brings_back_a_piece_for_every_enemy() {
        let game = Game::back_from_a_trail(6);
        let outcome = &game.last_report().unwrap().outcome;
        assert!(outcome.won, "the tour's first trail is won");
        let killed: u32 = outcome.deeds.iter().map(|deeds| deeds.kills).sum();
        assert!(outcome.drops.len() as u32 > killed, "{} pieces, {killed} kills", outcome.drops.len());
        assert_eq!(game.cargo().len(), outcome.drops.len());
        assert!(outcome.drops.iter().all(|gear| game.cargo().contains(gear)));
    }

    /// Walk the trail the party is on to its end and bring it home.
    fn walk_home(game: &mut Game) {
        let outcome = loop {
            let Screen::Trail(expedition) = &mut game.screen else {
                unreachable!("the party set out");
            };
            step(expedition, &game.data, &mut game.lessons);
            if let Some(outcome) = expedition.outcome.clone() {
                break outcome;
            }
        };
        game.return_to_camp(outcome);
    }

    /// The Journal (#231): with no input the party walks trail after trail
    /// and every report is kept, as many as the rules say; Earlier and
    /// Later step through them, and the one being read stays the one being
    /// read when the party leaves and arrives.
    #[test]
    fn a_report_is_kept_and_opened_afterwards() {
        let none = Hand::default();
        let mut game = Game::new(6);
        assert!(game.earlier().is_empty());
        assert_eq!(game.can(Action::Earlier, none), Err("No earlier report."));
        assert_eq!(game.can(Action::Later, none), Err("This is the latest report."));

        // No input: the countdown's own departure, three trails running.
        let mut outcomes = Vec::new();
        for trail in 1..=3 {
            game.depart();
            assert_eq!(game.earlier().len(), trail - 1, "on the road every one kept is earlier");
            walk_home(&mut game);
            let Screen::Camp { last: Some(outcome), .. } = &game.screen else {
                unreachable!("back in town");
            };
            outcomes.push(outcome.clone());
            // In town the newest kept is the latest, not an earlier one.
            assert_eq!(game.earlier().len(), trail - 1);
        }
        assert!(game.reading().is_none());
        assert!(report_text(&game).contains("cleared"));

        // Earlier, twice, to the first trail's report, as it was.
        game.act(Action::Earlier, none);
        assert_eq!(game.reading().map(|kept| kept.number), Some(2));
        assert_eq!(game.reading().unwrap().outcome, outcomes[1]);
        game.act(Action::Earlier, none);
        assert_eq!(game.reading().unwrap().outcome, outcomes[0]);
        assert!(report_text(&game).starts_with("EARLIER REPORT 1 of 2: trail 1, level 0"));
        assert!(report_side(&game).contains("> Trail 1 "), "{}", report_side(&game));
        assert_eq!(
            game.can(Action::Earlier, none),
            Err("This is the earliest report kept.")
        );
        game.act(Action::Earlier, none);
        assert_eq!(game.reading().map(|kept| kept.number), Some(1));

        // The party leaves and comes back; the report being read holds.
        game.depart();
        assert_eq!(game.reading().map(|kept| kept.number), Some(1));
        assert_eq!(game.earlier().len(), 3);
        walk_home(&mut game);
        assert_eq!(game.reading().map(|kept| kept.number), Some(1));

        // Later, up to the latest, and no further.
        for number in [2, 3] {
            game.act(Action::Later, none);
            assert_eq!(game.reading().map(|kept| kept.number), Some(number));
        }
        game.act(Action::Later, none);
        assert!(game.reading().is_none());
        assert_eq!(game.can(Action::Later, none), Err("This is the latest report."));
        // Turning the pages is nothing to undo.
        assert_eq!(game.can(Action::Undo, none), Err("Nothing changed this visit."));

        // Only so many are kept: the oldest goes, and a report that is no
        // longer kept is no longer being read.
        for _ in 0..3 {
            game.act(Action::Earlier, none);
        }
        assert_eq!(game.reading().map(|kept| kept.number), Some(1));
        let limit = game.data.rules.reports_kept;
        for _ in 0..limit {
            game.depart();
            walk_home(&mut game);
        }
        assert_eq!(game.reports.len(), limit);
        assert_eq!(game.reports.first().map(|kept| kept.number), Some(5));
        assert!(game.reading().is_none());
    }

    /// A mark (#232) is raised by something not yet seen, stops when it is
    /// seen, and does not come back for the same thing. Only the newest
    /// report raises one, so a long absence leaves one thing to read.
    #[test]
    fn a_mark_is_raised_until_its_thing_is_seen() {
        let upgrade = Raise::NewGear;
        let lost = Raise::Report { lost: true, rarity: None };
        let any_drop = Raise::Report { lost: false, rarity: Some("Normal".into()) };
        let unknown = Raise::Report { lost: false, rarity: Some("No such".into()) };
        let none = Hand::default();

        let mut game = Game::new(6);
        game.see_drop();
        game.see_report();
        for raise in [&upgrade, &lost, &any_drop] {
            assert!(!game.wants(raise), "{raise:?} before any trail");
        }

        // The first drop home is new gear, carried.
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        assert!(game.wants(&upgrade) && game.wants(&any_drop));
        assert!(!game.wants(&unknown));
        assert_eq!(game.wants(&lost), !game.reports[0].outcome.won);
        game.see_drop();
        assert!(!game.wants(&upgrade));
        assert!(game.wants(&any_drop), "the report is another thing");
        // Equipped and taken back it is the same piece: no mark again.
        game.act(Action::Equip, Hand::horse(0));
        game.act(Action::Undo, none);
        assert!(game.pending_drop().is_some() && !game.wants(&upgrade));
        game.see_report();
        assert!(!game.wants(&any_drop));
        game.depart();
        assert!(!game.wants(&any_drop) && !game.wants(&upgrade));

        // A report not read in town still wants reading on the road. The
        // fight's report is not it; Earlier opens it.
        for trail in 0.. {
            assert!(trail < 100, "no later trail drops a piece");
            walk_home(&mut game);
            let dropped = game.pending_drop().is_some();
            game.depart();
            if dropped {
                break;
            }
        }
        assert!(game.wants(&any_drop));
        game.see_report();
        assert!(game.wants(&any_drop), "the fight was in view, not the report");
        game.act(Action::Earlier, none);
        game.see_report();
        assert!(!game.wants(&any_drop));
        game.act(Action::Later, none);

        // A long absence: trail after trail with nobody looking. A lost
        // trail that another has followed raises nothing.
        let mut followed = false;
        for _ in 0..300 {
            walk_home(&mut game);
            let newest = game.reports.len() - 1;
            let lost_now = !game.reports[newest].outcome.won;
            assert_eq!(game.wants(&lost), lost_now);
            followed |= !lost_now && newest > 0 && !game.reports[newest - 1].outcome.won;
            game.depart();
        }
        assert!(followed, "no lost trail was followed by a won one");
    }

    /// A piece an earlier report shows is found where it is now and acted
    /// on there, while the party has it (#231).
    #[test]
    fn a_piece_in_an_earlier_report_is_found_where_it_is() {
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let outcome = &game.last_report().unwrap().outcome;
        let (gear, pieces) = (outcome.best().unwrap().clone(), outcome.drops.len());
        let slot = gear.slot.index();
        // The latest report's piece is its best, in the cargo, and is still
        // the report's piece once it is worn.
        let carried = game.reported_piece();
        let Some(Piece::Carried(at)) = carried else {
            panic!("the report's piece is not carried: {carried:?}");
        };
        assert_eq!(game.piece(Piece::Carried(at)), Some(&gear));
        game.act(Action::Equip, Hand { horse: 1, open: carried });
        let worn = Piece::Worn { horse: 1, slot };
        assert_eq!(game.reported_piece(), Some(worn));

        game.depart();
        assert_eq!(game.reported_piece(), None, "on the road the report is the fight's");
        game.act(Action::Earlier, Hand::default());
        assert_eq!(game.reported_piece(), Some(worn));
        let dropped = format!("It dropped {pieces} pieces, the best {}:", gear.name);
        assert!(report_text(&game).contains(&dropped), "{}", report_text(&game));

        // Back in town it is acted on from that report: onto another horse.
        walk_home(&mut game);
        let open = game.reported_piece();
        assert_eq!(open, Some(worn));
        game.act(Action::Equip, Hand { horse: 0, open });
        assert_eq!(game.reported_piece(), Some(Piece::Worn { horse: 0, slot }));

        // Thrown away, it is said to be gone.
        game.party[0].gear[slot] = None;
        assert_eq!(game.reported_piece(), None);
        let gone = format!("The party no longer has {}.", gear.name);
        assert!(report_text(&game).contains(&gone));
    }

    /// Keep going (#305): off in a new game, so no town leaves by itself;
    /// on, the usual time is counted from the press; a lost trail turns it
    /// off, and a won one leaves it on. It is nothing to undo.
    #[test]
    fn keep_going_is_off_until_pressed_and_after_a_lost_trail() {
        let none = Hand::default();
        let mut game = Game::new(6);
        assert!(!game.keep_going());
        assert_eq!(game.leaving_in(), None);
        assert_eq!(game.can(Action::KeepGoing, none), Ok(()));

        // Left for an hour, the party is still at the gate.
        for _ in 0..36_000 {
            game.wait_in_town(TICK);
        }
        assert!(game.in_town());

        game.act(Action::KeepGoing, none);
        assert!(game.keep_going());
        assert_eq!(game.leaving_in(), Some(CAMP_SECS));
        // On, it leaves after the usual time and not before.
        let mut on = Game::new(6);
        on.act(Action::KeepGoing, none);
        on.wait_in_town(CAMP_SECS - 0.5);
        assert!(on.in_town());
        on.wait_in_town(1.0);
        assert!(!on.in_town());
        assert_eq!(game.can(Action::Undo, none), Err("Nothing changed this visit."));
        game.act(Action::KeepGoing, none);
        assert!(!game.keep_going());
        assert_eq!(game.leaving_in(), None);

        // Trail after trail, from one seed and the next, until one has
        // been won and one lost.
        let (mut won, mut lost) = (false, false);
        for trail in 0..3000 {
            if won && lost {
                break;
            }
            if trail % 300 == 0 {
                game = Game::new(trail / 300);
                game.act(Action::KeepGoing, none);
            }
            game.depart();
            assert_eq!(game.leaving_in(), None, "on the road");
            game.act(Action::KeepGoing, none);
            assert!(game.keep_going(), "it is pressed in town only");
            walk_home(&mut game);
            if game.reports.last().unwrap().outcome.won {
                won = true;
                assert_eq!(game.leaving_in(), Some(CAMP_SECS));
            } else {
                lost = true;
                assert!(!game.keep_going());
                assert_eq!(game.leaving_in(), None);
                game.act(Action::KeepGoing, none);
            }
        }
        assert!(won && lost, "no trail was won, or none lost: {won} {lost}");
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
