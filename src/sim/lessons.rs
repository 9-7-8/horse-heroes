//! The lessons (#290): what the game explains, in the order of
//! `assets/data/lessons.ron`, and the queue that shows one at a time.
//!
//! Three things are kept apart (docs/philosophy/onboarding.md, 11): whether
//! a mechanic exists, which is not asked here at all; whether something has
//! happened that makes its lesson due; and whether the player has seen it,
//! done it or put it away. The queue is told what happened as plain facts
//! and is asked what to show. It is handed nothing it could hold the party
//! up with. How it works: docs/lessons.md.

use serde::Deserialize;

use super::data::parse;

/// How many town visits a town lesson is offered at, when the party leaves
/// each one with it showing (#297): the one it was cut off at, and the next.
const CUTS: u8 = 2;

/// Something that happened, as a plain fact: on the trail, in town, or by
/// the player's hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Event {
    /// An encounter began.
    Fight,
    /// A horse's ability fired.
    Ability,
    /// An enemy went down.
    Kill,
    /// A hit was resisted by its enemy's element.
    Resisted,
    /// A trail was won.
    Win,
    /// The party came back to town with a piece.
    Gear,
    /// The player opened a piece's card.
    Card,
    /// The player put a piece on a horse.
    Equipped,
    /// The player selected another horse.
    Selected,
    /// The player's Equip took a piece from the horse that wore it.
    Swapped,
    /// The party left town.
    SetOut,
    /// The player selected another horse with a piece's card open.
    Compared,
    /// The player has opened every place in the bar.
    Toured,
    /// The player sent the party out: Set out, by its button or its key.
    SentOut,
    /// The party came back to town with more than one piece (#352).
    Several,
    /// The player opened the card of a carried piece that is not the
    /// newest: one among the others.
    Another,
    /// The player read a piece's card with its Details (#228).
    Detailed,
    /// The player put a piece on a horse by dragging it there (#228).
    Dragged,
}

impl Event {
    #[cfg(test)]
    pub const ALL: [Event; 18] = [
        Event::Fight,
        Event::Ability,
        Event::Kill,
        Event::Resisted,
        Event::Win,
        Event::Gear,
        Event::Card,
        Event::Equipped,
        Event::Selected,
        Event::Swapped,
        Event::SetOut,
        Event::Compared,
        Event::Toured,
        Event::SentOut,
        Event::Several,
        Event::Another,
        Event::Detailed,
        Event::Dragged,
    ];
}

/// Where a lesson is shown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
pub enum Shown {
    #[default]
    Anywhere,
    Town,
    Road,
}

/// The control a lesson's mark is on, named as `places.ron` names it. The
/// menus find it; the sim only carries the name.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum Target {
    Place(String),
    Tab(String, String),
    Control(String),
    /// A horse of the party, by its place in it.
    Horse(usize),
    /// The horse that is selected, whichever it is now.
    Selected,
    /// The newest piece a trail brought back that the party still has,
    /// where it is now.
    Drop,
    /// The party as a whole, on no control: the line is said with no mark,
    /// so nothing is shown as wanting a press (#292).
    Party,
}

/// What finishes a lesson: the player watched something happen, or did it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Done {
    /// It happens while the lesson is showing.
    Watched(Event),
    /// It happens once the lesson is due, shown or not.
    Did(Event),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct LessonDef {
    pub id: String,
    pub due: Event,
    #[serde(default)]
    pub shown: Shown,
    pub target: Target,
    pub words: String,
    pub done: Done,
    /// What to say when the state has moved on.
    #[serde(default)]
    pub instead: Option<String>,
}

/// An entry of the list of things to try in town (#303): one thing to do on
/// the real controls, which pays a few coins the first time it is done.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct EntryDef {
    pub id: String,
    /// What has to happen before it is on the list.
    pub due: Event,
    /// Its one line: the thing to do.
    pub words: String,
    /// The player's doing this, in town, ticks it.
    pub done: Event,
    /// What it pays, once.
    pub coins: i32,
}

/// What Skip all hints is answered with, once (#350): where the hints
/// went, on the control they are found again by.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Parting {
    pub target: Target,
    /// Its one line: where they are, and what turns them back on.
    pub words: String,
}

/// `lessons.ron`: the lessons, the list of things to try, and what Skip all
/// hints is answered with.
#[derive(Deserialize)]
struct File {
    lessons: Vec<LessonDef>,
    #[serde(default)]
    list: Vec<EntryDef>,
    #[serde(default)]
    skipped: Option<Parting>,
}

/// How far the player is with an entry of the list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tick {
    /// It is on the list.
    pub listed: bool,
    /// The player did it, and was paid.
    pub done: bool,
}

/// Where a lesson's target is, looked for again each time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Found {
    /// Where the lesson's words expect it.
    Here,
    /// There, and not where it was: the lesson says its `instead`.
    Moved,
    /// Not there. The lesson waits.
    Gone,
}

/// How far the player is with one lesson. Each is its own fact: a lesson
/// can be done without having been seen, and seen without being done.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Progress {
    /// What it waits for has happened.
    pub due: bool,
    /// It has been shown.
    pub seen: bool,
    /// The player watched or did what it is about.
    pub done: bool,
    /// The player put it away.
    pub skipped: bool,
}

/// What the list of lessons says of one (#300), in a word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// It is the one showing.
    Showing,
    /// The player watched or did what it is about.
    Done,
    /// The player put it away.
    PutAway,
    /// It became due with the lessons turned off, and is not shown.
    Skipped,
    /// The party left town with it showing, at two visits running (#297),
    /// and it is not offered again by itself.
    Left,
    /// It is shown when its turn comes and its target is there.
    Waiting,
}

/// The lessons and how far the player is with each. Kept in memory only:
/// saving it is #44.
#[derive(Clone, Debug)]
pub struct Lessons {
    defs: Vec<LessonDef>,
    progress: Vec<Progress>,
    /// How often each town lesson was showing when the party left (#297).
    cut: Vec<u8>,
    /// Every event that has happened, once.
    raised: Vec<Event>,
    /// The lessons that have become due, in the order they came (#300).
    came: Vec<usize>,
    /// The lesson that is showing: one at a time.
    showing: Option<usize>,
    /// The lesson the player asked to see again, while it shows (#300).
    asked: Option<usize>,
    /// The player turned them off: none is shown unless asked for.
    off: bool,
    /// The list of things to try (#303), and how far the player is with
    /// each entry.
    entries: Vec<EntryDef>,
    ticks: Vec<Tick>,
    /// Coins the list has paid that the party has not been given yet.
    owed: i32,
    /// The party is in town: an entry is ticked there and nowhere else.
    town: bool,
    /// An entry was ticked during this town visit.
    fresh: bool,
    /// What Skip all hints is answered with (#350).
    parting: Option<Parting>,
}

impl Lessons {
    pub fn load() -> Lessons {
        let file: File = parse("lessons.ron", include_str!("../../assets/data/lessons.ron"));
        let mut lessons = Lessons::new(file.lessons).with_list(file.list);
        lessons.parting = file.skipped;
        lessons
    }

    pub fn new(defs: Vec<LessonDef>) -> Lessons {
        Lessons {
            progress: vec![Progress::default(); defs.len()],
            cut: vec![0; defs.len()],
            defs,
            raised: Vec::new(),
            came: Vec::new(),
            showing: None,
            asked: None,
            off: false,
            entries: Vec::new(),
            ticks: Vec::new(),
            owed: 0,
            town: true,
            fresh: false,
            parting: None,
        }
    }

    /// The same lessons with this list of things to try (#303).
    pub fn with_list(mut self, entries: Vec<EntryDef>) -> Lessons {
        self.ticks = vec![Tick::default(); entries.len()];
        self.entries = entries;
        self
    }

    /// What Skip all hints is answered with, if the file says.
    pub fn parting(&self) -> Option<&Parting> {
        self.parting.as_ref()
    }

    /// The list of things to try, in the file's order.
    pub fn entries(&self) -> &[EntryDef] {
        &self.entries
    }

    /// How far the player is with an entry of the list.
    pub fn tick(&self, entry: usize) -> Tick {
        self.ticks[entry]
    }

    /// Whether the list is to be seen: in town, while an entry on it is
    /// not done, and for the rest of the visit in which the last one was.
    pub fn list_showing(&self) -> bool {
        let open = self.ticks.iter().any(|tick| tick.listed && !tick.done);
        self.town && self.ticks.iter().any(|tick| tick.listed) && (open || self.fresh)
    }

    /// Whether an entry on the list is still waiting for this to be done.
    pub fn wants(&self, event: Event) -> bool {
        let waiting = |(def, tick): (&EntryDef, &Tick)| tick.listed && !tick.done && def.done == event;
        self.entries.iter().zip(&self.ticks).any(waiting)
    }

    /// The party is in town.
    pub fn arrive(&mut self) {
        self.town = true;
    }

    /// The party has left town.
    pub fn leave(&mut self) {
        self.town = false;
        self.fresh = false;
    }

    /// The coins the list has paid since this was last asked: they are the
    /// party's now, and are not handed over twice.
    pub fn collect(&mut self) -> i32 {
        std::mem::take(&mut self.owed)
    }

    pub fn defs(&self) -> &[LessonDef] {
        &self.defs
    }

    /// How far the player is with a lesson.
    #[cfg(test)]
    pub fn progress(&self, lesson: usize) -> Progress {
        self.progress[lesson]
    }

    /// The lessons that have become due, in the order they came, whatever
    /// became of each (#300). None is here before its event has happened.
    pub fn listed(&self) -> &[usize] {
        &self.came
    }

    /// What the list says of a lesson.
    pub fn state(&self, lesson: usize) -> State {
        let progress = self.progress[lesson];
        if self.showing == Some(lesson) {
            State::Showing
        } else if progress.done {
            State::Done
        } else if progress.skipped {
            State::PutAway
        } else if self.cut[lesson] >= CUTS {
            State::Left
        } else if self.off {
            State::Skipped
        } else {
            State::Waiting
        }
    }

    /// The lesson showing because the player asked for it again.
    pub fn asked(&self) -> Option<usize> {
        self.asked
    }

    /// Whether a lesson of this name is in the file.
    pub fn knows(&self, id: &str) -> bool {
        self.defs.iter().any(|def| def.id == id)
    }

    /// The lesson that is showing.
    #[cfg(test)]
    pub fn showing(&self) -> Option<&LessonDef> {
        self.showing.map(|lesson| &self.defs[lesson])
    }

    pub fn is_off(&self) -> bool {
        self.off
    }

    /// Whether this has happened yet.
    #[cfg(test)]
    pub fn happened(&self, event: Event) -> bool {
        self.raised.contains(&event)
    }

    /// Something happened. A lesson that was waiting for it is finished;
    /// and, the first time only, the lessons it makes due are due. It can
    /// be told the same thing any number of times.
    pub fn raise(&mut self, event: Event) {
        // Before anything becomes due: what makes a lesson due is never
        // what finishes it, in the same breath.
        for lesson in 0..self.defs.len() {
            // One asked for again is finished again by the same thing.
            let again = self.asked == Some(lesson);
            if !self.progress[lesson].due || (self.progress[lesson].done && !again) {
                continue;
            }
            let finished = match self.defs[lesson].done {
                Done::Did(wanted) => wanted == event,
                Done::Watched(wanted) => wanted == event && self.showing == Some(lesson),
            };
            if finished {
                self.progress[lesson].done = true;
                if self.showing == Some(lesson) {
                    self.put_down();
                }
            }
        }
        // The list (#303): an entry on it is ticked by its thing being done
        // in town, and pays once. Nothing else ticks one.
        if self.town {
            for (def, tick) in self.entries.iter().zip(&mut self.ticks) {
                if tick.listed && !tick.done && def.done == event {
                    tick.done = true;
                    self.owed += def.coins;
                    self.fresh = true;
                }
            }
        }
        if !self.raised.contains(&event) {
            self.raised.push(event);
            for (def, tick) in self.entries.iter().zip(&mut self.ticks) {
                tick.listed |= def.due == event;
            }
            for (lesson, def) in self.defs.iter().enumerate() {
                if def.due == event {
                    self.progress[lesson].due = true;
                    self.came.push(lesson);
                }
            }
        }
    }

    /// Nothing is showing.
    fn put_down(&mut self) {
        self.showing = None;
        self.asked = None;
    }

    /// Whether this is where a lesson is shown.
    pub fn shown_here(&self, lesson: usize, in_town: bool) -> bool {
        match self.defs[lesson].shown {
            Shown::Anywhere => true,
            Shown::Town => in_town,
            Shown::Road => !in_town,
        }
    }

    /// Whether a lesson could be shown now: it is due, this is where it is
    /// shown, and its target is there; and it is neither done nor put away
    /// nor turned off, unless the player asked for it.
    fn fits(&self, lesson: usize, in_town: bool, found: &[Found]) -> bool {
        let progress = self.progress[lesson];
        let wanted = self.asked == Some(lesson)
            || (!self.off && !progress.done && !progress.skipped && self.cut[lesson] < CUTS);
        progress.due
            && wanted
            && self.shown_here(lesson, in_town)
            && found.get(lesson).is_some_and(|found| *found != Found::Gone)
    }

    /// The lesson to show now, by its place in the file. `found` is where
    /// each lesson's target is at this moment, in the file's order: the game
    /// is looked at again every time. One that is showing and no longer
    /// fits goes back to waiting: a town lesson when the party sets out, or
    /// one whose target has gone. A town lesson the party leaves is offered
    /// at the next town visit, and left there if the party leaves it again
    /// (#297). Then, with none showing, the first that fits is shown. One
    /// showing is never pushed aside by another.
    pub fn show(&mut self, in_town: bool, found: &[Found]) -> Option<usize> {
        if let Some(lesson) = self.showing
            && !self.fits(lesson, in_town, found)
        {
            // One asked for again is the player's, and goes as it came.
            if !in_town && self.defs[lesson].shown == Shown::Town && self.asked != Some(lesson) {
                self.cut[lesson] += 1;
            }
            self.put_down();
        }
        if self.showing.is_none() {
            self.showing = (0..self.defs.len()).find(|&lesson| self.fits(lesson, in_town, found));
            if let Some(lesson) = self.showing {
                self.progress[lesson].seen = true;
            }
        }
        self.showing
    }

    /// What a lesson says with its target where it is.
    pub fn words(&self, lesson: usize, found: Found) -> &str {
        let def = &self.defs[lesson];
        match (&def.instead, found) {
            (Some(instead), Found::Moved) => instead,
            _ => &def.words,
        }
    }

    /// The player put the lesson that is showing away. It is not shown
    /// again by itself, and is not counted as done.
    #[cfg(test)]
    pub fn dismiss(&mut self) {
        if let Some(lesson) = self.showing {
            self.put_away(lesson, true);
        }
    }

    /// Whether a lesson is put away. One put away is not shown by itself;
    /// one taken out again waits its turn as it did.
    pub fn put_away(&mut self, lesson: usize, away: bool) {
        self.progress[lesson].skipped = away;
        if away && self.showing == Some(lesson) {
            self.put_down();
        }
    }

    /// The player skipped them all: the one showing is put away and none is
    /// shown after it. They still become due and done.
    #[cfg(test)]
    pub fn skip_all(&mut self) {
        self.dismiss();
        self.off = true;
    }

    /// Lessons are turned off, or on again (#300). Off, the one showing
    /// goes back to waiting and none is shown by itself; they still become
    /// due and done, and are listed.
    pub fn turn_off(&mut self, off: bool) {
        self.off = off;
        if off {
            self.put_down();
        }
    }

    /// The player asks for a lesson again, from the list (#300): whether it
    /// was done or put away, and with the lessons turned off. It is shown
    /// now, in the place of any other, if this is where it is shown and its
    /// target is there; otherwise nothing changes and false comes back. It
    /// goes as any lesson does: when its thing is done, when it is put away,
    /// or when its target is no longer there.
    pub fn again(&mut self, lesson: usize, in_town: bool, found: Found) -> bool {
        if !self.progress[lesson].due || !self.shown_here(lesson, in_town) || found == Found::Gone {
            return false;
        }
        self.progress[lesson].skipped = false;
        self.progress[lesson].seen = true;
        self.showing = Some(lesson);
        self.asked = Some(lesson);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lesson(id: &str, due: Event, shown: Shown, done: Done) -> LessonDef {
        LessonDef {
            id: id.into(),
            due,
            shown,
            target: Target::Horse(0),
            words: format!("{id}."),
            done,
            instead: Some(format!("{id}, moved.")),
        }
    }

    const HERE: [Found; 4] = [Found::Here; 4];

    fn id(lessons: &Lessons) -> Option<&str> {
        lessons.showing().map(|def| def.id.as_str())
    }

    /// The file holds together: no two lessons of one name, and each says
    /// one line.
    #[test]
    fn the_lessons_file_is_consistent() {
        let lessons = Lessons::load();
        assert!(!lessons.defs().is_empty());
        for def in lessons.defs() {
            let named = lessons.defs().iter().filter(|other| other.id == def.id).count();
            assert_eq!(named, 1, "lessons.ron: two lessons named `{}`", def.id);
            for words in [Some(&def.words), def.instead.as_ref()].into_iter().flatten() {
                assert!(!words.trim().is_empty(), "{}", def.id);
                // One line, never a page (the owner, 2026-10-11), and
                // short enough for the hint's line at the smallest window.
                assert!(!words.contains('\n') && words.chars().count() <= 85, "{}: {words}", def.id);
            }
            // A Drop is the only target that moves.
            assert!(def.instead.is_none() || def.target == Target::Drop, "{}", def.id);
        }
        // What Skip all hints is answered with (#350) is one line too.
        let parting = lessons.parting().expect("lessons.ron: nothing answers Skip all hints");
        let words = parting.words.trim();
        assert!(!words.is_empty() && !words.contains('\n') && words.chars().count() <= 85, "{words}");
        assert!(Lessons::new(Vec::new()).parting().is_none());
    }

    fn entry(id: &str, due: Event, done: Event, coins: i32) -> EntryDef {
        EntryDef { id: id.into(), due, words: format!("{id}."), done, coins }
    }

    /// The list in the file holds together (#303): no two entries of one
    /// name, each one short line, and each pays something.
    #[test]
    fn the_list_in_the_file_is_consistent() {
        let lessons = Lessons::load();
        assert!(!lessons.entries().is_empty());
        for def in lessons.entries() {
            let named = lessons.entries().iter().filter(|other| other.id == def.id).count();
            assert_eq!(named, 1, "lessons.ron: two entries named `{}`", def.id);
            let words = def.words.trim();
            assert!(!words.is_empty() && !words.contains('\n') && words.chars().count() <= 32, "{}", def.id);
            assert!(def.coins > 0, "{}", def.id);
        }
    }

    /// An entry is on the list once its event has happened, is ticked by
    /// its thing being done after that, and pays once however often the
    /// thing is done again. What put it on the list does not tick it in the
    /// same breath, and what was done before it was listed does not count.
    #[test]
    fn an_entry_is_ticked_by_doing_it_and_pays_once() {
        let mut lessons = Lessons::new(Vec::new()).with_list(vec![
            entry("select", Event::Gear, Event::Selected, 2),
            entry("equip", Event::Gear, Event::Equipped, 3),
            entry("again", Event::Card, Event::Card, 5),
        ]);
        let ticks = |lessons: &Lessons| [0, 1, 2].map(|entry| lessons.tick(entry));
        lessons.raise(Event::Selected);
        assert_eq!(ticks(&lessons), [Tick::default(); 3], "done before it was listed");
        assert!(!lessons.list_showing() && !lessons.wants(Event::Selected));

        lessons.raise(Event::Gear);
        let listed = Tick { listed: true, done: false };
        assert_eq!(ticks(&lessons), [listed, listed, Tick::default()]);
        assert!(lessons.list_showing() && lessons.wants(Event::Selected));
        assert_eq!(lessons.collect(), 0);

        lessons.raise(Event::Selected);
        let done = Tick { listed: true, done: true };
        assert_eq!(ticks(&lessons), [done, listed, Tick::default()]);
        assert!(!lessons.wants(Event::Selected));
        assert_eq!(lessons.collect(), 2);
        assert_eq!(lessons.collect(), 0, "handed over once");
        for _ in 0..3 {
            lessons.raise(Event::Selected);
        }
        assert_eq!(lessons.collect(), 0, "paid once");

        lessons.raise(Event::Card);
        assert_eq!(lessons.tick(2), listed, "listed and ticked in the same breath");
        lessons.raise(Event::Card);
        lessons.raise(Event::Equipped);
        assert_eq!(ticks(&lessons), [done; 3]);
        assert_eq!(lessons.collect(), 8);
    }

    /// An entry is ticked in town and nowhere else. Setting out with entries
    /// undone loses nothing: they are there at the next town. The list
    /// stays for the visit in which its last entry is ticked, and is gone
    /// at the next.
    #[test]
    fn the_list_is_the_towns_and_waits_for_the_next_one() {
        let mut lessons = Lessons::new(Vec::new()).with_list(vec![
            entry("select", Event::Gear, Event::Selected, 2),
            entry("equip", Event::Gear, Event::Equipped, 3),
        ]);
        lessons.raise(Event::Gear);
        lessons.raise(Event::Selected);
        assert_eq!(lessons.collect(), 2);

        lessons.leave();
        assert!(!lessons.list_showing(), "on the road");
        lessons.raise(Event::Equipped);
        assert!(!lessons.tick(1).done, "ticked on the road");
        assert_eq!(lessons.collect(), 0);

        lessons.arrive();
        assert!(lessons.list_showing());
        assert_eq!([lessons.tick(0).done, lessons.tick(1).done], [true, false]);
        lessons.raise(Event::Equipped);
        assert_eq!(lessons.collect(), 3);
        assert!(lessons.list_showing(), "its last tick is seen");
        lessons.leave();
        lessons.arrive();
        assert!(!lessons.list_showing(), "nothing is left to try");
    }

    /// The list is not the hints: with every hint skipped an entry is still
    /// ticked and paid, and a lesson's state changes nothing on it.
    #[test]
    fn the_list_does_not_depend_on_the_hints() {
        let mut lessons = Lessons::new(vec![lesson("a", Event::Gear, Shown::Town, Done::Did(Event::Card))])
            .with_list(vec![entry("card", Event::Gear, Event::Card, 4)]);
        lessons.raise(Event::Gear);
        lessons.show(true, &HERE);
        lessons.skip_all();
        lessons.raise(Event::Card);
        assert!(lessons.tick(0).done);
        assert_eq!(lessons.collect(), 4);
    }

    /// Nothing is due until its event has happened, and with every lesson
    /// due at once one shows and the rest wait, in the file's order.
    #[test]
    fn with_every_lesson_due_one_shows_and_the_rest_wait() {
        let mut lessons = Lessons::new(vec![
            lesson("a", Event::Fight, Shown::Anywhere, Done::Did(Event::Card)),
            lesson("b", Event::Kill, Shown::Anywhere, Done::Did(Event::Equipped)),
            lesson("c", Event::Win, Shown::Anywhere, Done::Watched(Event::Gear)),
        ]);
        assert_eq!(lessons.show(false, &HERE), None, "nothing has happened");

        for event in [Event::Win, Event::Kill, Event::Fight] {
            lessons.raise(event);
        }
        assert!((0..3).all(|lesson| lessons.progress(lesson).due));
        assert_eq!(lessons.show(false, &HERE), Some(0));
        assert_eq!(lessons.show(false, &HERE), Some(0), "it stays");
        let seen: Vec<bool> = (0..3).map(|lesson| lessons.progress(lesson).seen).collect();
        assert_eq!(seen, [true, false, false]);

        // Each goes when its thing is done, and the next takes its place.
        lessons.raise(Event::Card);
        assert_eq!(id(&lessons), None);
        assert_eq!(lessons.show(false, &HERE), Some(1));
        lessons.raise(Event::Equipped);
        assert_eq!(lessons.show(false, &HERE), Some(2));
        lessons.raise(Event::Gear);
        assert_eq!(lessons.show(false, &HERE), None);
        assert!((0..3).all(|lesson| lessons.progress(lesson).done));
    }

    /// The order is the file's and nothing else's: the same lessons the
    /// other way round are shown the other way round. One that is showing
    /// is not pushed aside by one above it that becomes due.
    #[test]
    fn the_order_is_the_files() {
        let a = lesson("a", Event::Fight, Shown::Anywhere, Done::Did(Event::Card));
        let b = lesson("b", Event::Kill, Shown::Anywhere, Done::Did(Event::Card));
        for defs in [vec![a.clone(), b.clone()], vec![b.clone(), a.clone()]] {
            let first = defs[0].id.clone();
            let mut lessons = Lessons::new(defs);
            lessons.raise(Event::Fight);
            lessons.raise(Event::Kill);
            lessons.show(true, &HERE);
            assert_eq!(id(&lessons), Some(first.as_str()));
        }

        let mut lessons = Lessons::new(vec![a, b]);
        lessons.raise(Event::Kill);
        lessons.show(true, &HERE);
        lessons.raise(Event::Fight);
        lessons.show(true, &HERE);
        assert_eq!(id(&lessons), Some("b"), "one focus at a time");
    }

    /// A trigger fires once: a lesson that is done or put away is not due
    /// again when its event happens again.
    #[test]
    fn a_trigger_fires_once() {
        let mut lessons = Lessons::new(vec![
            lesson("gear", Event::Gear, Shown::Town, Done::Did(Event::Card)),
            lesson("win", Event::Win, Shown::Town, Done::Did(Event::Equipped)),
        ]);
        lessons.raise(Event::Gear);
        assert!(lessons.happened(Event::Gear) && !lessons.happened(Event::Win));
        assert_eq!(lessons.show(true, &HERE), Some(0));
        lessons.raise(Event::Card);
        lessons.raise(Event::Win);
        assert_eq!(lessons.show(true, &HERE), Some(1));
        lessons.dismiss();

        // Back in town with a piece, after another win.
        for _ in 0..3 {
            lessons.raise(Event::Gear);
            lessons.raise(Event::Win);
            assert_eq!(lessons.show(true, &HERE), None, "the opening played again");
        }
    }

    /// Seen, done and skipped are three facts. What makes a lesson due does
    /// not finish it in the same breath; a thing done needs no telling; a
    /// thing watched counts only while its lesson shows.
    #[test]
    fn seen_done_and_skipped_are_kept_apart() {
        let mut lessons = Lessons::new(vec![
            lesson("watch", Event::Ability, Shown::Anywhere, Done::Watched(Event::Ability)),
            lesson("do", Event::Gear, Shown::Anywhere, Done::Did(Event::Equipped)),
            lesson("skip", Event::Fight, Shown::Anywhere, Done::Did(Event::Card)),
        ]);
        let all = |lessons: &Lessons| [0, 1, 2].map(|lesson| lessons.progress(lesson));
        let none = Progress::default();

        // Due, and its own event again before it shows: not watched yet.
        lessons.raise(Event::Ability);
        lessons.raise(Event::Ability);
        let due = Progress { due: true, ..none };
        assert_eq!(all(&lessons), [due, none, none]);
        lessons.show(false, &HERE);
        let seen = Progress { seen: true, ..due };
        assert_eq!(all(&lessons), [seen, none, none]);
        lessons.raise(Event::Ability);
        let watched = Progress { done: true, ..seen };
        assert_eq!(all(&lessons), [watched, none, none]);

        // Done unprompted: never shown, and never will be.
        lessons.raise(Event::Gear);
        lessons.raise(Event::Equipped);
        let unprompted = Progress { done: true, ..due };
        assert_eq!(all(&lessons), [watched, unprompted, none]);
        assert_eq!(lessons.show(false, &HERE), None);

        // Put away: seen, not done. Doing it afterwards is still done.
        lessons.raise(Event::Fight);
        assert_eq!(lessons.show(false, &HERE), Some(2));
        lessons.dismiss();
        let skipped = Progress { skipped: true, ..seen };
        assert_eq!(all(&lessons), [watched, unprompted, skipped]);
        assert_eq!(lessons.show(false, &HERE), None);
        lessons.raise(Event::Card);
        assert_eq!(lessons.progress(2), Progress { done: true, ..skipped });
    }

    /// A town lesson that is showing when the party sets out waits for the
    /// next town, and a road lesson for the next road. Neither is counted
    /// as put away.
    #[test]
    fn a_town_lesson_waits_for_the_next_town() {
        let mut lessons = Lessons::new(vec![
            lesson("town", Event::Gear, Shown::Town, Done::Did(Event::Card)),
            lesson("road", Event::Gear, Shown::Road, Done::Did(Event::Equipped)),
        ]);
        lessons.raise(Event::Gear);
        assert_eq!(lessons.show(true, &HERE), Some(0));
        assert_eq!(lessons.show(false, &HERE), Some(1), "the party set out");
        assert_eq!(lessons.show(true, &HERE), Some(0), "and came back");
        assert!(!lessons.progress(0).skipped && !lessons.progress(1).skipped);
        lessons.raise(Event::Card);
        assert_eq!(lessons.show(true, &HERE), None, "the road's is not shown in town");
        assert_eq!(lessons.show(false, &HERE), Some(1));
    }

    /// A town lesson the party leaves is offered at the next town visit,
    /// once (#297): left a second time, it is not offered by itself again,
    /// and the next waiting takes its place. One done is never offered
    /// again, one asked for from the list is not counted as cut off, and
    /// one that only waited behind another was not cut off at all.
    #[test]
    fn a_lesson_cut_off_by_setting_out_is_offered_once_more() {
        let mut lessons = Lessons::new(vec![
            lesson("left", Event::Gear, Shown::Town, Done::Did(Event::Card)),
            lesson("next", Event::Gear, Shown::Town, Done::Did(Event::Equipped)),
            lesson("done", Event::Win, Shown::Town, Done::Did(Event::Selected)),
        ]);
        lessons.raise(Event::Gear);
        assert_eq!(lessons.show(true, &HERE), Some(0));
        assert_eq!(lessons.show(false, &HERE), None, "the party set out");
        assert_eq!(lessons.state(0), State::Waiting);
        assert_eq!(lessons.show(true, &HERE), Some(0), "offered at the next town");
        assert_eq!(lessons.show(false, &HERE), None, "and left again");
        assert_eq!(lessons.state(0), State::Left);
        assert!(!lessons.progress(0).skipped && !lessons.progress(0).done, "nothing was pressed");
        assert_eq!(lessons.show(true, &HERE), Some(1), "the next waited behind it");
        lessons.show(false, &HERE);
        assert_eq!(lessons.show(true, &HERE), Some(1), "its own first cut");

        // The one left: the list still shows it, and doing it still counts.
        assert!(lessons.again(0, true, Found::Here));
        assert_eq!(lessons.show(true, &HERE), Some(0));
        lessons.raise(Event::Card);
        assert_eq!(lessons.state(0), State::Done);

        // Asked for from the list and left: that was not a second cut.
        assert!(lessons.again(1, true, Found::Here));
        assert_eq!(lessons.show(false, &HERE), None);
        assert_eq!(lessons.show(true, &HERE), Some(1), "asked for, it was not cut off");

        // One cut off once and then done is not offered again.
        lessons.raise(Event::Card);
        lessons.raise(Event::Equipped);
        lessons.raise(Event::Win);
        assert_eq!(lessons.show(true, &HERE), Some(2));
        lessons.show(false, &HERE);
        lessons.raise(Event::Selected);
        assert_eq!(lessons.show(true, &HERE), None);
        assert_eq!(lessons.state(2), State::Done);
    }

    /// The game is looked at again every time: a lesson whose target is
    /// gone waits and lets the next one by, and one whose target has moved
    /// says its other line.
    #[test]
    fn a_lesson_follows_its_target() {
        let mut lessons = Lessons::new(vec![
            lesson("piece", Event::Gear, Shown::Anywhere, Done::Did(Event::Card)),
            lesson("other", Event::Gear, Shown::Anywhere, Done::Did(Event::Equipped)),
        ]);
        lessons.raise(Event::Gear);
        assert_eq!(lessons.show(true, &[Found::Here, Found::Here]), Some(0));
        assert_eq!(lessons.words(0, Found::Here), "piece.");
        assert_eq!(lessons.show(true, &[Found::Moved, Found::Here]), Some(0));
        assert_eq!(lessons.words(0, Found::Moved), "piece, moved.");

        assert_eq!(lessons.show(true, &[Found::Gone, Found::Here]), Some(1));
        assert_eq!(lessons.show(true, &[Found::Gone, Found::Gone]), None);
        assert_eq!(lessons.show(true, &[]), None, "nothing looked for is nothing found");
        assert_eq!(lessons.show(true, &[Found::Here, Found::Here]), Some(0), "it is back");
        assert!(!lessons.progress(0).skipped);

        let mut plain = lesson("plain", Event::Gear, Shown::Anywhere, Done::Did(Event::Card));
        plain.instead = None;
        assert_eq!(Lessons::new(vec![plain]).words(0, Found::Moved), "plain.");
    }

    /// Skip all puts the one showing away and shows none after it; they
    /// still become due and done behind it, and come back when asked for.
    #[test]
    fn skipped_lessons_come_back_when_asked_for() {
        let mut lessons = Lessons::new(vec![
            lesson("a", Event::Fight, Shown::Anywhere, Done::Did(Event::Card)),
            lesson("b", Event::Kill, Shown::Anywhere, Done::Did(Event::Equipped)),
        ]);
        lessons.raise(Event::Fight);
        lessons.show(false, &HERE);
        lessons.skip_all();
        assert!(lessons.is_off() && lessons.progress(0).skipped);
        lessons.raise(Event::Kill);
        assert_eq!(lessons.show(false, &HERE), None);
        assert!(lessons.progress(1).due && !lessons.progress(1).seen);
        lessons.raise(Event::Equipped);

        // Turned on again, the one put away stays away until it is taken
        // out, and the other was done meanwhile.
        lessons.turn_off(false);
        assert_eq!(lessons.show(false, &HERE), None);
        lessons.put_away(0, false);
        assert_eq!(lessons.show(false, &HERE), Some(0), "the one put away is shown again");
        lessons.raise(Event::Card);
        assert_eq!(lessons.show(false, &HERE), None, "the other was done meanwhile");
    }

    /// The list (#300): no lesson is in it before its event has happened;
    /// each is there from then on, in the order it came and not the file's,
    /// whatever became of it; and it says which of them was done, put away
    /// or skipped.
    #[test]
    fn the_list_has_every_lesson_that_came_due_in_the_order_it_came() {
        let mut lessons = Lessons::new(vec![
            lesson("a", Event::Fight, Shown::Anywhere, Done::Did(Event::Card)),
            lesson("b", Event::Kill, Shown::Anywhere, Done::Did(Event::Equipped)),
            lesson("c", Event::Win, Shown::Anywhere, Done::Did(Event::Gear)),
            lesson("d", Event::SetOut, Shown::Anywhere, Done::Did(Event::Gear)),
        ]);
        assert!(lessons.listed().is_empty(), "nothing has happened");
        lessons.raise(Event::Kill);
        lessons.raise(Event::Fight);
        lessons.raise(Event::Kill);
        assert_eq!(lessons.listed(), [1, 0], "in the order they came, once each");

        assert_eq!(lessons.show(false, &HERE), Some(0));
        assert_eq!([0, 1].map(|l| lessons.state(l)), [State::Showing, State::Waiting]);
        lessons.dismiss();
        lessons.raise(Event::Equipped);
        assert_eq!([0, 1].map(|l| lessons.state(l)), [State::PutAway, State::Done]);

        // Turned off: what comes due is listed and not shown.
        lessons.turn_off(true);
        lessons.raise(Event::Win);
        assert_eq!(lessons.show(false, &HERE), None);
        assert_eq!(lessons.listed(), [1, 0, 2]);
        assert_eq!(lessons.state(2), State::Skipped);
        assert_eq!([0, 1].map(|l| lessons.state(l)), [State::PutAway, State::Done], "the rest is readable");
        lessons.turn_off(false);
        assert_eq!(lessons.state(2), State::Waiting);
        assert!(!lessons.listed().contains(&3), "its event has not happened");
    }

    /// A lesson asked for again (#300) is shown whether it was done or put
    /// away, with the lessons off too, in the place of the one showing; not
    /// before it is due, not where it is not shown, and not with its target
    /// gone. It goes when its thing is done or it is put away, and does not
    /// come back by itself.
    #[test]
    fn a_lesson_asked_for_is_shown_again() {
        let mut lessons = Lessons::new(vec![
            lesson("done", Event::Fight, Shown::Anywhere, Done::Did(Event::Card)),
            lesson("away", Event::Fight, Shown::Town, Done::Did(Event::Equipped)),
            lesson("later", Event::Win, Shown::Anywhere, Done::Did(Event::Gear)),
            lesson("other", Event::Fight, Shown::Anywhere, Done::Did(Event::Gear)),
        ]);
        lessons.raise(Event::Fight);
        lessons.raise(Event::Card);
        assert_eq!(lessons.show(true, &HERE), Some(1));
        lessons.dismiss();
        assert_eq!(lessons.show(true, &HERE), Some(3));

        assert!(!lessons.again(2, true, Found::Here), "not before it is due");
        assert!(!lessons.again(1, false, Found::Here), "a town lesson on the road");
        assert!(!lessons.again(0, true, Found::Gone), "its target is not there");
        assert_eq!(lessons.show(true, &HERE), Some(3), "and nothing changed");

        // One that was done: shown in the other's place, and still done.
        assert!(lessons.again(0, true, Found::Here));
        assert_eq!(lessons.show(true, &HERE), Some(0));
        assert_eq!(lessons.state(0), State::Showing);
        assert!(lessons.progress(0).done);
        // Its thing done again finishes it again, and the other is back.
        lessons.raise(Event::Card);
        assert_eq!(lessons.show(true, &HERE), Some(3));
        assert_eq!(lessons.state(0), State::Done);

        // One put away, with the lessons off: shown, and put away again.
        lessons.turn_off(true);
        assert_eq!(lessons.show(true, &HERE), None);
        assert!(lessons.again(1, true, Found::Here));
        assert_eq!(lessons.show(true, &HERE), Some(1));
        lessons.dismiss();
        assert_eq!(lessons.show(true, &HERE), None);
        assert_eq!(lessons.state(1), State::PutAway);

        // It leaves with its target, and being done does not come back.
        assert!(lessons.again(0, true, Found::Here));
        assert_eq!(lessons.show(true, &[Found::Gone, Found::Here, Found::Here, Found::Here]), None);
        assert_eq!(lessons.show(true, &HERE), None);
        assert_eq!(lessons.asked(), None);
    }
}
