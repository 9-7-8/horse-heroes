//! Drives the sim and presents it: the camp/trail loop and input. What the
//! player reads and presses is the menu shell (`menu::shell`).

use bevy::prelude::*;
use serde::Deserialize;

use crate::sim::data::{Data, Element, Stats};
use crate::sim::expedition::{Expedition, Outcome, Phase, Plan};
use crate::sim::lessons::{Event, Lessons};
use crate::sim::loot::{Gear, Horse, best_wearer, swap_worn};

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
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut game = Game::new(self.seed);
        game.keep_going = self.keep_going;
        app.insert_resource(game)
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
    /// The piece in the cargo is an upgrade for some horse.
    Upgrade,
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
    /// In the cargo: the last trail's drop, until it is equipped.
    Carried,
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
    /// What was done, as Undo says it: "Ice for the next trail".
    what: String,
    party: Vec<Horse>,
    drop: Option<Gear>,
    element: Element,
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

    /// The party, the cargo and the element as they are now, to go back to
    /// if what is about to be done is undone.
    fn before(&self, what: String, moved: Option<(Piece, Piece)>) -> Change {
        Change {
            what,
            party: self.party.clone(),
            drop: self.pending_drop().cloned(),
            element: self.element,
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
            _ => format!(
                "{} on {}",
                self.pending_drop().map_or("", |gear| gear.name.as_str()),
                name(hand.horse),
            ),
        }
    }

    /// Take back the last change of this town visit: the horses, the cargo
    /// and the element are exactly what they were before it.
    fn undo(&mut self) {
        let Some(change) = self.changes.pop() else {
            return;
        };
        self.party = change.party;
        self.element = change.element;
        if let Screen::Camp { last, note, .. } = &mut self.screen {
            if let Some(outcome) = last {
                outcome.drop = change.drop;
            }
            *note = format!("Taken back: {}.", change.what);
        }
    }

    /// Where Undo would take a piece from and to: the last change's move,
    /// the other way. A card that is open on the piece goes with it.
    pub fn undone_move(&self) -> Option<(Piece, Piece)> {
        let (from, to) = self.changes.last()?.moved?;
        Some((to, from))
    }

    /// Put the last run's drop on a horse. Any horse can wear any piece
    /// (#250). Returns what happened.
    fn equip_drop(&mut self, horse: usize) -> Option<String> {
        // Said of the horse as it is before the piece goes on (#11).
        let note = menu::card::equipped(self, self.party.get(horse)?, self.pending_drop()?);
        let Screen::Camp {
            last: Some(outcome),
            ..
        } = &mut self.screen
        else {
            return None;
        };
        let horse = self.party.get_mut(horse)?;
        let gear = outcome.drop.take()?;
        let slot = gear.slot.index();
        horse.gear[slot] = Some(gear);
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
        if self.pending_drop() == Some(gear) {
            return Some(Piece::Carried);
        }
        let slot = gear.slot.index();
        self.party
            .iter()
            .position(|horse| horse.gear[slot].as_ref() == Some(gear))
            .map(|horse| Piece::Worn { horse, slot })
    }

    /// The piece dropped by the trail whose report the Journal shows, where
    /// it is now, if the party still has it (#231). None on the road while
    /// the report is the fight's.
    pub fn reported_piece(&self) -> Option<Piece> {
        self.find(self.open_report()?.outcome.drop.as_ref()?)
    }

    /// The report of the last trail the party finished, if it has walked one.
    pub fn last_report(&self) -> Option<&Kept> {
        self.reports.last()
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
            Raise::Upgrade => self.trails > self.seen_drop && self.drop_best_wearer().is_some(),
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
                let dropped = kept.outcome.drop.as_ref().map(|gear| gear.rarity);
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
        let Screen::Camp { last, .. } = &self.screen else {
            return Err("In town only.");
        };
        match action {
            Action::Element(_)
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
                    Some(Piece::Carried) | None => match last {
                        Some(outcome) if outcome.drop.is_some() => Ok(()),
                        _ => Err("Nothing to equip."),
                    },
                }
            }
        }
    }

    /// The piece that is somewhere, if anything is there.
    pub fn piece(&self, piece: Piece) -> Option<&Gear> {
        match piece {
            Piece::Carried => self.pending_drop(),
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
            _ => self.pending_drop()?.slot.index(),
        };
        Some(Piece::Worn {
            horse: hand.horse,
            slot,
        })
    }

    /// Do an action, if it can be done. `hand` is what it is done to, for
    /// an action that is done to a horse or a piece.
    pub fn act(&mut self, action: Action, hand: Hand) {
        if self.can(action, hand).is_err() {
            return;
        }
        match action {
            // Choosing the element it already has is no change to undo.
            Action::Element(element) if element == self.element => {}
            Action::Element(element) => {
                let before = self.before(format!("{} for the next trail", element.name()), None);
                self.changes.push(before);
                self.element = element;
            }
            Action::Equip => {
                let from = hand.open.unwrap_or(Piece::Carried);
                let moved = self.equipped_at(hand).map(|to| (from, to));
                let before = self.before(self.equip_label(hand), moved);
                let done = match hand.open {
                    Some(Piece::Worn { .. }) => self.swap(hand),
                    _ => self.equip_drop(hand.horse),
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
            Action::SetOut => self.depart(),
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
            .find_map(|kept| self.find(kept.outcome.drop.as_ref()?))
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
        let taken = self
            .drop_best_wearer()
            .and_then(|horse| self.equip_drop(horse));
        self.accumulator = 0.0;
        // The visit is over: nothing done in it can be taken back now.
        self.changes.clear();
        let mut expedition =
            Expedition::new(&self.data, self.plan.clone(), &self.party, self.element);
        // What was put on at the gate opens the road's log, so the report
        // that said it would happen is not the last word on it (#11).
        if let Some(taken) = taken {
            expedition.log.extend(taken.lines().map(String::from));
        }
        self.screen = Screen::Trail(expedition);
        self.lessons.raise(Event::SetOut);
    }

    fn return_to_camp(&mut self, outcome: Outcome) {
        self.arriving = None;
        self.coins += outcome.coins;
        if outcome.drop.is_some() {
            self.lessons.raise(Event::Gear);
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

/// The report the Journal is open at: an earlier one that was kept, or the
/// latest, which is the last trail and its drop in town and the fight on the
/// road. Shown beside `report_side`.
pub fn report_text(game: &Game) -> String {
    let rules = &game.data.rules;
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
        out += &match &kept.outcome.drop {
            Some(gear) if game.find(gear).is_some() => format!("\nIt dropped {}:\n", gear.name),
            Some(gear) => format!(
                "\nIt dropped {}.\nThe party no longer has it.\n",
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
            if outcome.drop.is_some() {
                out += &format!("\n{}\n", menu::card::appraisal(game));
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

/// How a trail ended, in the two lines every report of it opens with.
fn outcome_lines(outcome: &Outcome) -> String {
    format!(
        "{}  cleared {}/{} encounters, +{} coins\n\
         {}: {} strong hits, {} resisted\n",
        if outcome.won { "VICTORY!" } else { "DEFEAT." },
        outcome.cleared,
        outcome.total,
        outcome.coins,
        outcome.element.name(),
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
            match &outcome.drop {
                Some(gear) => format!("  {}", gear.name),
                None => String::new(),
            },
        );
    }
    out
}

/// Beside the report: the trail ahead and the element to walk it with in
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
        assert_eq!(game.can(Action::Element(Element::Ice), Hand::default()), Ok(()));
        assert_eq!(game.can(Action::SetOut, Hand::default()), Ok(()));
        assert_eq!(game.can(Action::Equip, Hand::default()), Err("Nothing to equip."));

        game.act(Action::Element(Element::Ice), Hand::horse(0));
        assert_eq!(game.element, Element::Ice);
        game.act(Action::Equip, Hand::horse(0));
        assert!(game.in_town(), "a refused action changes nothing");

        game.act(Action::SetOut, Hand::horse(0));
        assert!(!game.in_town());
        for action in [
            Action::Element(Element::Fire),
            Action::Equip,
            Action::SetOut,
            Action::KeepGoing,
            Action::Undo,
        ] {
            assert_eq!(game.can(action, Hand::default()), Err("In town only."));
        }
        assert_eq!(game.can(Action::Earlier, Hand::default()), Err("No earlier report."));
        game.act(Action::Element(Element::Fire), Hand::horse(0));
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
            assert_eq!(game.can(Action::Equip, Hand::default()), Ok(()));
            let gear = game.pending_drop().unwrap();
            assert_eq!(game.piece(Piece::Carried), Some(gear));
            let (slot, name) = (gear.slot.index(), gear.name.clone());
            // With the carried piece's card open or with none: the same.
            let hand = Hand {
                horse: selected,
                open: (selected > 0).then_some(Piece::Carried),
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
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
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

    /// Undo (#230) steps back through the changes of a town visit, one at
    /// a time, and the horses, the cargo and the element are exactly what
    /// they were. It never reaches back past a departure.
    #[test]
    fn undo_steps_back_through_the_town_visit() {
        type Seen = (Vec<[Option<Gear>; 3]>, Option<Gear>, Element);
        let seen = |game: &Game| -> Seen {
            (
                game.party.iter().map(|horse| horse.gear.clone()).collect(),
                game.pending_drop().cloned(),
                game.element,
            )
        };
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let slot = game.pending_drop().unwrap().slot.index();
        let nothing = Err("Nothing changed this visit.");
        assert_eq!(game.can(Action::Undo, Hand::default()), nothing);
        assert_eq!(game.undone_move(), None);

        let mut states = vec![seen(&game)];
        let other = match game.element {
            Element::Ice => Element::Plant,
            _ => Element::Ice,
        };
        let swap = Hand {
            horse: 1,
            open: Some(Piece::Worn { horse: 0, slot }),
        };
        for (action, hand) in [
            (Action::Equip, Hand::horse(0)),
            (Action::Element(other), Hand::horse(0)),
            (Action::Equip, swap),
        ] {
            game.act(action, hand);
            assert_ne!(Some(&seen(&game)), states.last(), "{action:?} changed nothing");
            states.push(seen(&game));
        }
        // What changes nothing leaves nothing to undo: the element it has
        // already, and an Equip that is refused.
        game.act(Action::Element(other), Hand::horse(0));
        game.act(Action::Equip, Hand::horse(2));
        assert_eq!(seen(&game), states[3]);

        // The swap took the card from the first horse's slot to the second's.
        let (first, second) = (Piece::Worn { horse: 0, slot }, Piece::Worn { horse: 1, slot });
        assert_eq!(game.undone_move(), Some((second, first)));
        for step in (0..3).rev() {
            assert_eq!(game.can(Action::Undo, Hand::default()), Ok(()));
            if step == 0 {
                assert_eq!(game.undone_move(), Some((first, Piece::Carried)));
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
        let upgrade = Raise::Upgrade;
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

        // The first drop home is an upgrade: nothing is worn yet.
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
        let gear = game.pending_drop().unwrap().clone();
        let slot = gear.slot.index();
        // The latest report's piece is the one in the cargo, and is still
        // the report's piece once it is worn.
        assert_eq!(game.reported_piece(), Some(Piece::Carried));
        game.act(Action::Equip, Hand::horse(1));
        let worn = Piece::Worn { horse: 1, slot };
        assert_eq!(game.reported_piece(), Some(worn));

        game.depart();
        assert_eq!(game.reported_piece(), None, "on the road the report is the fight's");
        game.act(Action::Earlier, Hand::default());
        assert_eq!(game.reported_piece(), Some(worn));
        assert!(report_text(&game).contains(&format!("It dropped {}:", gear.name)));

        // Back in town it is acted on from that report: onto another horse.
        walk_home(&mut game);
        let open = game.reported_piece();
        assert_eq!(open, Some(worn));
        game.act(Action::Equip, Hand { horse: 0, open });
        assert_eq!(game.reported_piece(), Some(Piece::Worn { horse: 0, slot }));

        // Thrown away, it is said to be gone.
        game.party[0].gear[slot] = None;
        assert_eq!(game.reported_piece(), None);
        assert!(report_text(&game).contains("The party no longer has it."));
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
