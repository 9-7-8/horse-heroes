//! The lessons in the menus (#290): the queue of `sim::lessons` is told what
//! the player's hands did, is asked which lesson to show, and that lesson's
//! one line goes on the hint's line with its mark on the control it is about
//! (#291). How it works: docs/lessons.md.
//!
//! Nothing here reaches the simulation or a control's availability: a
//! lesson is a line and a mark, and the game goes on under it.

use bevy::prelude::*;

use super::shell::{About, Hint, Places, Shell};
use crate::game::{Game, Piece};
use crate::sim::lessons::{Event, Found, Target};

/// Where a lesson's target is now: the control its mark goes on, and
/// whether that is where the lesson's words expect it. None when it is not
/// there, and the lesson waits.
pub fn find(target: &Target, map: &Places, game: &Game, selected: usize) -> Option<(About, Found)> {
    let about = match target {
        Target::Place(name) => About::Place(map.page_named(name)?),
        Target::Tab(place, tab) => {
            let (place, tab) = map.tab_named(place, tab)?;
            About::Tab(place, tab)
        }
        Target::Control(name) => About::Control(map.control_named(name)?),
        Target::Horse(horse) if *horse < game.party.len() => About::Horse(*horse),
        Target::Horse(_) => return None,
        Target::Selected if selected < game.party.len() => About::Horse(selected),
        Target::Selected => return None,
        // The piece is found where it is now: carried, worn, or not at all.
        Target::Drop => {
            let piece = game.newest_piece()?;
            let found = match piece {
                Piece::Carried => Found::Here,
                Piece::Worn { .. } => Found::Moved,
            };
            return Some((About::Piece(piece), found));
        }
    };
    Some((about, Found::Here))
}

/// What the player's hands held when the lessons last looked: whether a
/// piece's card was open, and which horse was selected.
#[derive(Clone, Copy, Debug, Default)]
pub struct Held {
    card: bool,
    horse: usize,
}

/// Keep the hint's line to the lessons. The queue hears what the player's
/// hands did here: a piece's card opened, another horse selected, a hint put
/// away, every hint skipped. Then the game is looked at again, and the lesson to show now is
/// the hint. A sample hint (the dev key's) is left alone.
pub fn follow(game: &mut Game, shell: &mut Shell, map: &Places, held: &mut Held) {
    let hand = shell.hand();
    let open = hand.open.is_some();
    if open && !held.card {
        game.lessons.raise(Event::Card);
    }
    if hand.horse != held.horse {
        game.lessons.raise(Event::Selected);
    }
    *held = Held { card: open, horse: hand.horse };

    if shell.hints_off() != game.lessons.is_off() {
        match shell.hints_off() {
            true => game.lessons.skip_all(),
            false => game.lessons.restore(),
        }
    }
    if game.lessons.showing().is_some_and(|def| shell.put_away(&def.id)) {
        game.lessons.dismiss();
    }

    let found: Vec<Option<(About, Found)>> = game
        .lessons
        .defs()
        .iter()
        .map(|def| find(&def.target, map, game, hand.horse))
        .collect();
    let places: Vec<Found> = found
        .iter()
        .map(|found| found.map_or(Found::Gone, |(_, found)| found))
        .collect();
    let in_town = game.in_town();
    let wanted = game.lessons.show(in_town, &places).and_then(|lesson| {
        let (about, at) = found[lesson]?;
        Some(Hint {
            lesson: game.lessons.defs()[lesson].id.clone(),
            about,
            words: game.lessons.words(lesson, at).into(),
        })
    });

    let ours = shell.hint().is_some_and(|hint| game.lessons.knows(&hint.lesson));
    match wanted {
        Some(hint) if shell.hint() != Some(&hint) && (ours || shell.hint().is_none()) => {
            shell.finish_hint();
            shell.raise_hint(hint);
        }
        None if ours => shell.finish_hint(),
        _ => {}
    }
}

pub fn teach(
    mut game: ResMut<Game>,
    mut shell: ResMut<Shell>,
    map: Res<Places>,
    mut held: Local<Held>,
) {
    follow(&mut game, &mut shell, &map, &mut held);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::menu::shell::{Detail, Item, SAMPLE};
    use crate::game::{Action, CAMP_SECS, Hand, Screen, TICK, step};
    use crate::sim::data::Element;
    use crate::sim::lessons::{Done, LessonDef, Lessons, Progress, Shown};

    /// Four lessons of the tests' own, so that none of these tests holds a
    /// word of `lessons.ron`: two for the road, and in town one about the
    /// piece and one about a control.
    fn lessons() -> Lessons {
        let lesson = |id: &str, due, shown, target, done| LessonDef {
            id: id.into(),
            due,
            shown,
            target,
            words: format!("{id}."),
            done,
            instead: Some(format!("{id}, moved.")),
        };
        Lessons::new(vec![
            lesson("fight", Event::Fight, Shown::Road, Target::Horse(0), Done::Watched(Event::Kill)),
            lesson("ability", Event::Ability, Shown::Road, Target::Horse(2), Done::Watched(Event::Ability)),
            lesson("piece", Event::Gear, Shown::Town, Target::Drop, Done::Did(Event::Card)),
            lesson("equip", Event::Card, Shown::Town, Target::Control("Equip".into()), Done::Did(Event::Equipped)),
        ])
    }

    fn shell(map: &Places) -> Shell {
        Shell::new(map.pages().count())
    }

    /// A step of the game with nobody at it: time in town, or on the trail,
    /// and home the moment the trail is over. The walk back and the last
    /// fountain are the presentation's and are not waited for here.
    fn pass(game: &mut Game) {
        match &mut game.screen {
            Screen::Camp { .. } => game.wait_in_town(TICK),
            Screen::Trail(expedition) => {
                step(expedition, &game.data, &mut game.lessons);
                if let Some(outcome) = expedition.outcome.clone() {
                    game.return_to_camp(outcome);
                }
            }
        }
    }

    /// The lesson whose hint is showing.
    fn hinted(shell: &Shell) -> Option<&str> {
        shell.hint().map(|hint| hint.lesson.as_str())
    }

    /// Pin the card of the piece in the cargo, as a press on it does.
    fn open_carried(shell: &mut Shell) {
        let place = shell.place();
        let tab = shell.tab(place);
        shell.show_detail(place, Detail { tab, item: Item::Carried, opener: None });
    }

    /// Every lesson in the file names a target that exists: a place, a tab
    /// or a control of `places.ron` by its name, and no control's name that
    /// two controls share; a horse the party has. A piece is looked for in
    /// the game, and found in the cargo and on a horse.
    #[test]
    fn every_lesson_names_a_target_that_exists() {
        let map = Places::load();
        let game = Game::new(6);
        for def in game.lessons.defs() {
            match &def.target {
                Target::Drop => {}
                Target::Control(name) => {
                    let named = map.controls().iter().filter(|control| control.name == *name);
                    assert_eq!(named.count(), 1, "{}: the control `{name}`", def.id);
                    let id = map.control_named(name).unwrap();
                    assert!(map.control(id).built(), "{}: `{name}` is not built", def.id);
                }
                target => assert!(find(target, &map, &game, 0).is_some(), "{}: {target:?}", def.id),
            }
        }

        assert_eq!(find(&Target::Drop, &map, &game, 0), None, "no trail walked");
        assert_eq!(find(&Target::Horse(3), &map, &game, 0), None);
        assert_eq!(find(&Target::Place("Nowhere".into()), &map, &game, 0), None);
        assert_eq!(find(&Target::Tab("Horses".into(), "Nowhere".into()), &map, &game, 0), None);
        assert_eq!(find(&Target::Control("Nothing".into()), &map, &game, 0), None);

        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let slot = game.pending_drop().unwrap().slot.index();
        let carried = (About::Piece(Piece::Carried), Found::Here);
        assert_eq!(find(&Target::Drop, &map, &game, 0), Some(carried));
        game.act(Action::Equip, Hand::horse(1));
        let worn = (About::Piece(Piece::Worn { horse: 1, slot }), Found::Moved);
        assert_eq!(find(&Target::Drop, &map, &game, 0), Some(worn));
        game.party[1].gear[slot] = None;
        assert_eq!(find(&Target::Drop, &map, &game, 0), None, "left behind");
    }

    /// Every event a lesson can wait for is one the game raises: the trail
    /// and the town by themselves, and the player's hands for the rest.
    /// So every lesson in the file names a trigger that can happen.
    #[test]
    fn the_game_raises_every_event_a_lesson_can_wait_for() {
        let map = Places::load();
        let by_hand = [Event::Card, Event::Equipped, Event::Selected, Event::Swapped];
        let by_itself = |event: &Event| !by_hand.contains(event);
        let mut game = Game::new(0);
        'seeds: for seed in 0..40 {
            game = Game::new(seed);
            game.keep_going = true;
            // Two trails and the town between them.
            for _ in 0..6000 {
                pass(&mut game);
                if Event::ALL.iter().filter(|event| by_itself(event)).all(|e| game.lessons.happened(*e)) {
                    break 'seeds;
                }
            }
        }
        for event in Event::ALL.iter().filter(|event| by_itself(event)) {
            assert!(game.lessons.happened(*event), "{event:?} never happened by itself");
        }

        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let (mut shell, mut held) = (shell(&map), Held::default());
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(by_hand.iter().all(|event| !game.lessons.happened(*event)));
        open_carried(&mut shell);
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(game.lessons.happened(Event::Card));
        shell.inspect(1, None);
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(game.lessons.happened(Event::Selected));
        game.act(Action::Equip, shell.hand());
        assert!(game.lessons.happened(Event::Equipped));
        // Putting the carried piece on is not a swap; taking it from the
        // horse that wears it is.
        assert!(!game.lessons.happened(Event::Swapped));
        let slot = game.party[1].gear.iter().position(|worn| worn.is_some()).unwrap();
        game.act(Action::Equip, Hand { horse: 0, open: Some(Piece::Worn { horse: 1, slot }) });
        assert!(game.lessons.happened(Event::Swapped));

        for def in game.lessons.defs() {
            assert!(Event::ALL.contains(&def.due), "{}", def.id);
        }
    }

    /// A whole first trail and the town visit after it, to the next departure
    /// or, after a lost trail, as long in town as a visit lasts, with no
    /// input: the party is where it would be with every lesson skipped,
    /// tick for tick, while lessons come and go over it. The party never
    /// waits on a lesson.
    #[test]
    fn the_party_never_waits_on_a_lesson() {
        let map = Places::load();
        let mut left_again = 0;
        for (seed, own) in [(6, true), (1, true), (3, true), (6, false)] {
            let mut taught = Game::new(seed);
            if own {
                taught.lessons = lessons();
            }
            let mut untaught = Game::new(seed);
            untaught.lessons.skip_all();
            let (mut shell, mut held) = (shell(&map), Held::default());
            let mut shown: Vec<String> = Vec::new();
            let (mut departed, mut in_town_for) = (false, 0.0);
            for game in [&mut taught, &mut untaught] {
                game.act(Action::KeepGoing, Hand::default());
                game.act(Action::SetOut, Hand::default());
            }
            for tick in 0..20_000 {
                let was_in_town = taught.in_town();
                pass(&mut taught);
                pass(&mut untaught);
                follow(&mut taught, &mut shell, &map, &mut held);
                if let Some(lesson) = hinted(&shell)
                    && shown.last().map(String::as_str) != Some(lesson)
                {
                    shown.push(lesson.into());
                }

                assert_eq!(taught.in_town(), untaught.in_town(), "seed {seed}, tick {tick}");
                assert_eq!(taught.leaving_in(), untaught.leaving_in(), "seed {seed}, tick {tick}");
                let worn = |game: &Game| game.party.iter().map(|horse| horse.gear.clone()).collect::<Vec<_>>();
                assert_eq!(worn(&taught), worn(&untaught), "seed {seed}, tick {tick}");
                assert_eq!(taught.coins_earned(), untaught.coins_earned());
                if let (Screen::Trail(a), Screen::Trail(b)) = (&taught.screen, &untaught.screen) {
                    assert_eq!(a.log, b.log, "seed {seed}, tick {tick}");
                }
                departed |= was_in_town && !taught.in_town();
                in_town_for = if taught.in_town() { in_town_for + TICK } else { 0.0 };
                if departed || in_town_for > 2.0 * CAMP_SECS {
                    break;
                }
            }
            left_again += usize::from(departed);
            assert!(taught.last_report().is_some(), "seed {seed}: the trail was not walked");
            assert_eq!(taught.last_report(), untaught.last_report());
            // Not for want of lessons: the road's were shown on the road.
            if own {
                assert_eq!(shown.first().map(String::as_str), Some("fight"));
                assert!(shown.len() > 1, "seed {seed}: {shown:?}");
            }
        }
        assert!(left_again > 0, "no party left its first town again");
    }

    /// No control's availability depends on a lesson: whatever the lessons'
    /// state, in town and on the road, every action can be done or not for
    /// the same reason, and a hint leaves every control as it was.
    #[test]
    fn no_controls_availability_depends_on_a_lesson() {
        let map = Places::load();
        let actions = [
            Action::Element(Element::Ice),
            Action::Equip,
            Action::SetOut,
            Action::KeepGoing,
            Action::Undo,
            Action::Earlier,
            Action::Later,
        ];
        let home = || {
            (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some())
                .expect("no seed under 50 brings a drop home")
        };
        let on_the_road = || {
            let mut game = home();
            game.act(Action::SetOut, Hand::default());
            game
        };
        for (place, make) in [("in town", &home as &dyn Fn() -> Game), ("on the road", &on_the_road)] {
            let plain = make();
            let hands = [Hand::default(), Hand { horse: 1, open: Some(Piece::Carried) }];
            let can = |game: &Game| {
                hands.map(|hand| actions.map(|action| game.can(action, hand)))
            };
            let wanted = can(&plain);

            // Lessons due and none seen; one showing; all skipped; none
            // in the file at all.
            let mut game = make();
            game.lessons = lessons();
            for event in [Event::Fight, Event::Ability, Event::Gear, Event::Card] {
                game.lessons.raise(event);
            }
            assert_eq!(can(&game), wanted, "{place}: lessons due");
            let (mut shell, mut held) = (shell(&map), Held::default());
            let before = shell.clone();
            follow(&mut game, &mut shell, &map, &mut held);
            assert!(shell.hint().is_some(), "{place}: no lesson shows");
            assert_eq!(can(&game), wanted, "{place}: a lesson showing");
            // The hint is all that changed in the menus.
            let mut without = shell.clone();
            without.finish_hint();
            assert_eq!(without, before, "{place}");

            shell.skip_hints();
            follow(&mut game, &mut shell, &map, &mut held);
            assert!(shell.hint().is_none() && game.lessons.is_off());
            assert_eq!(can(&game), wanted, "{place}: every lesson skipped");

            let mut fresh = make();
            fresh.lessons = Lessons::new(Vec::new());
            assert_eq!(can(&fresh), wanted, "{place}: no lessons at all");
        }
    }

    /// The first trail and town, with a player at it: the road's lessons
    /// show on the road, one at a time, and go when what they are about has
    /// been watched; the piece's lesson waits for town, is finished by a
    /// press on the piece, and Equip's follows it.
    #[test]
    fn the_first_trail_and_town_are_taught_one_lesson_at_a_time() {
        let map = Places::load();
        let (mut shell, mut held) = (shell(&map), Held::default());
        let seed = (0..50)
            .find(|seed| Game::back_from_a_trail(*seed).pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let mut game = Game::new(seed);
        game.lessons = lessons();
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None, "nothing has happened yet");

        game.act(Action::SetOut, Hand::default());
        let mut shown: Vec<String> = Vec::new();
        while !game.in_town() {
            pass(&mut game);
            follow(&mut game, &mut shell, &map, &mut held);
            if let Some(lesson) = hinted(&shell)
                && shown.last().map(String::as_str) != Some(lesson)
            {
                shown.push(lesson.into());
            }
        }
        assert_eq!(shown[..2], ["fight", "ability"]);
        let first = game.lessons.progress(0);
        assert_eq!(first, Progress { due: true, seen: true, done: true, skipped: false });

        // In town: the piece, in the cargo.
        assert_eq!(hinted(&shell), Some("piece"));
        assert_eq!(shell.hint().unwrap().about, About::Piece(Piece::Carried));
        open_carried(&mut shell);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("equip"));
        let equip = map.control_named("Equip").unwrap();
        assert_eq!(shell.hint().unwrap().about, About::Control(equip));

        // Got it puts it away, and it does not come back.
        assert!(shell.dismiss_hint());
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);
        assert_eq!(game.lessons.progress(3), Progress { due: true, seen: true, done: false, skipped: true });
    }

    /// A town lesson the party walks away from waits for the next town, and
    /// then finds its piece where it is now: on the horse it went on at the
    /// gate, with the line for that. The dev key's sample is left alone.
    #[test]
    fn a_lesson_left_in_town_finds_its_piece_again() {
        let map = Places::load();
        let (mut shell, mut held) = (shell(&map), Held::default());
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.drop_best_wearer().is_some())
            .expect("no seed under 50 brings an upgrade home");
        game.lessons = lessons();
        game.lessons.raise(Event::Gear);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("piece"));
        assert_eq!(shell.hint().unwrap().words, "piece.");

        game.act(Action::SetOut, Hand::default());
        follow(&mut game, &mut shell, &map, &mut held);
        assert_ne!(hinted(&shell), Some("piece"), "a town lesson on the road");
        assert!(!game.lessons.progress(2).skipped && !game.lessons.progress(2).done);

        // Home again with nothing new: the piece is on a horse now.
        let Screen::Trail(expedition) = &mut game.screen else {
            unreachable!("the party set out");
        };
        let mut outcome = loop {
            step(expedition, &game.data, &mut game.lessons);
            if let Some(outcome) = expedition.outcome.clone() {
                break outcome;
            }
        };
        outcome.drop = None;
        game.return_to_camp(outcome);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("piece"));
        assert!(matches!(shell.hint().unwrap().about, About::Piece(Piece::Worn { .. })));
        assert_eq!(shell.hint().unwrap().words, "piece, moved.");

        // A sample hint stands over a lesson's and is not replaced by it.
        shell.finish_hint();
        let sample = Hint { lesson: SAMPLE.into(), about: About::Place(0), words: "A sample.".into() };
        assert!(shell.raise_hint(sample.clone()));
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(shell.hint(), Some(&sample));
        shell.finish_hint();
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("piece"));
    }

    /// The lessons of the file itself, in a first town with a piece (#296),
    /// by what each waits for and never by its name or words. Nothing about
    /// a swap is said before a swap is done; nothing about Equip's result
    /// before Equip; a lesson on the selected horse follows the selection;
    /// and a player who presses nothing is shown one lesson and no second.
    #[test]
    fn the_first_item_is_taught_by_what_the_player_does() {
        let map = Places::load();
        let home = || {
            let mut game = (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some())
                .expect("no seed under 50 brings a drop home");
            // Whatever the road taught is not this test's.
            game.lessons = Lessons::load();
            game.lessons.raise(Event::Gear);
            game
        };
        let due = |game: &Game| game.lessons.showing().map(|def| def.due);
        let waits_for = |game: &Game, event: Event| game.lessons.defs().iter().any(|def| def.due == event);

        // Nobody at it: one lesson, about the piece, for as long as the
        // party is in town, and the party leaves as it would have.
        let mut game = home();
        let (mut shell, mut held) = (shell(&map), Held::default());
        game.act(Action::KeepGoing, Hand::default());
        let mut shown: Vec<String> = Vec::new();
        while game.in_town() {
            follow(&mut game, &mut shell, &map, &mut held);
            if let Some(lesson) = hinted(&shell)
                && shown.last().map(String::as_str) != Some(lesson)
            {
                shown.push(lesson.into());
            }
            pass(&mut game);
        }
        assert_eq!(shown.len(), 1, "{shown:?}");
        assert!(game.lessons.defs().iter().all(|def| def.id != shown[0] || def.target == Target::Drop));

        // With a player at it.
        let mut game = home();
        (shell, held) = (Shell::new(map.pages().count()), Held::default());
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(due(&game), Some(Event::Gear));
        assert_eq!(shell.hint().unwrap().about, About::Piece(Piece::Carried));

        // The card opens: whose stats it is about, on the selected horse,
        // and on the next one when that is selected.
        open_carried(&mut shell);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(due(&game), Some(Event::Card));
        assert_eq!(shell.hint().unwrap().about, About::Horse(0));
        assert!(game.lessons.showing().is_some_and(|def| def.done == Done::Did(Event::Selected)));
        shell.inspect(2, None);
        follow(&mut game, &mut shell, &map, &mut held);
        let equip = map.control_named("Equip").unwrap();
        assert_eq!(shell.hint().unwrap().about, About::Control(equip));

        // Equip: the carried piece goes on. That is no swap.
        let hand = shell.hand();
        assert_eq!(hand, Hand { horse: 2, open: Some(Piece::Carried) });
        game.act(Action::Equip, hand);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(due(&game), Some(Event::Equipped));
        assert!(waits_for(&game, Event::Swapped), "the swap's lesson is gone from the file");

        // Put away, and nothing about a swap comes after it.
        assert!(shell.dismiss_hint());
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);

        // A piece taken from the horse that wears it: now it is said, on
        // the horse that has it.
        let slot = game.party[2].gear.iter().position(|worn| worn.is_some()).unwrap();
        shell.inspect(0, None);
        game.act(Action::Equip, Hand { horse: 0, open: Some(Piece::Worn { horse: 2, slot }) });
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(due(&game), Some(Event::Swapped));
        assert_eq!(shell.hint().unwrap().about, About::Horse(0));

        // Setting out finishes what was only to be seen in town.
        game.act(Action::SetOut, Hand::default());
        follow(&mut game, &mut shell, &map, &mut held);
        for (lesson, def) in game.lessons.defs().iter().enumerate() {
            if def.done == Done::Did(Event::SetOut) {
                assert!(game.lessons.progress(lesson).done, "{}", def.id);
            }
        }
    }
}
