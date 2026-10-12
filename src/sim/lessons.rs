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
}

impl Event {
    #[cfg(test)]
    pub const ALL: [Event; 8] = [
        Event::Fight,
        Event::Ability,
        Event::Kill,
        Event::Resisted,
        Event::Win,
        Event::Gear,
        Event::Card,
        Event::Equipped,
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
    /// The newest piece a trail brought back that the party still has,
    /// where it is now.
    Drop,
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

/// The lessons and how far the player is with each. Kept in memory only:
/// saving it is #44.
#[derive(Clone, Debug)]
pub struct Lessons {
    defs: Vec<LessonDef>,
    progress: Vec<Progress>,
    /// Every event that has happened, once.
    raised: Vec<Event>,
    /// The lesson that is showing: one at a time.
    showing: Option<usize>,
    /// The player skipped them all: none is shown.
    off: bool,
}

impl Lessons {
    pub fn load() -> Lessons {
        Lessons::new(parse("lessons.ron", include_str!("../../assets/data/lessons.ron")))
    }

    pub fn new(defs: Vec<LessonDef>) -> Lessons {
        Lessons {
            progress: vec![Progress::default(); defs.len()],
            defs,
            raised: Vec::new(),
            showing: None,
            off: false,
        }
    }

    pub fn defs(&self) -> &[LessonDef] {
        &self.defs
    }

    /// How far the player is with a lesson. The Journal's list of them is
    /// #300; until then only the tests ask.
    #[cfg(test)]
    pub fn progress(&self, lesson: usize) -> Progress {
        self.progress[lesson]
    }

    /// Whether a lesson of this name is in the file.
    pub fn knows(&self, id: &str) -> bool {
        self.defs.iter().any(|def| def.id == id)
    }

    /// The lesson that is showing.
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
            if !self.progress[lesson].due || self.progress[lesson].done {
                continue;
            }
            let finished = match self.defs[lesson].done {
                Done::Did(wanted) => wanted == event,
                Done::Watched(wanted) => wanted == event && self.showing == Some(lesson),
            };
            if finished {
                self.progress[lesson].done = true;
                if self.showing == Some(lesson) {
                    self.showing = None;
                }
            }
        }
        if !self.raised.contains(&event) {
            self.raised.push(event);
            for (def, progress) in self.defs.iter().zip(&mut self.progress) {
                progress.due |= def.due == event;
            }
        }
    }

    /// Whether a lesson could be shown now: it is due, neither done nor put
    /// away, this is where it is shown, and its target is there.
    fn fits(&self, lesson: usize, in_town: bool, found: &[Found]) -> bool {
        let progress = self.progress[lesson];
        let place = match self.defs[lesson].shown {
            Shown::Anywhere => true,
            Shown::Town => in_town,
            Shown::Road => !in_town,
        };
        !self.off
            && progress.due
            && !progress.done
            && !progress.skipped
            && place
            && found.get(lesson).is_some_and(|found| *found != Found::Gone)
    }

    /// The lesson to show now, by its place in the file. `found` is where
    /// each lesson's target is at this moment, in the file's order: the game
    /// is looked at again every time. One that is showing and no longer
    /// fits goes back to waiting: a town lesson when the party sets out, or
    /// one whose target has gone. Then, with none showing, the first that
    /// fits is shown. One showing is never pushed aside by another.
    pub fn show(&mut self, in_town: bool, found: &[Found]) -> Option<usize> {
        if self.showing.is_some_and(|lesson| !self.fits(lesson, in_town, found)) {
            self.showing = None;
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
    pub fn dismiss(&mut self) {
        if let Some(lesson) = self.showing.take() {
            self.progress[lesson].skipped = true;
        }
    }

    /// The player skipped them all: the one showing is put away and none is
    /// shown after it. They still become due and done.
    pub fn skip_all(&mut self) {
        self.dismiss();
        self.off = true;
    }

    /// Lessons are shown again, those put away too (#300).
    pub fn restore(&mut self) {
        self.off = false;
        for progress in &mut self.progress {
            progress.skipped = false;
        }
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

        lessons.restore();
        assert_eq!(lessons.show(false, &HERE), Some(0), "the one put away is shown again");
        lessons.raise(Event::Card);
        assert_eq!(lessons.show(false, &HERE), None, "the other was done meanwhile");
    }
}
