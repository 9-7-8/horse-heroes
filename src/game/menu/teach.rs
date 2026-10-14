//! The lessons in the menus (#290): the queue of `sim::lessons` is told what
//! the player's hands did, is asked which lesson to show, and that lesson's
//! one line goes on the hint's line with its mark on the control it is about
//! (#291). How it works: docs/lessons.md.
//!
//! Nothing here reaches the simulation or a control's availability: a
//! lesson is a line and a mark, and the game goes on under it.

use bevy::prelude::*;

use super::shell::{About, Hint, PARTING, Places, Shell, Switch};
use crate::fx::drops::Kind;
use crate::fx::sound::Heard;
use crate::game::{Game, Piece};
use crate::sim::lessons::{Event, Found, Shown, State, Target};

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
                Piece::Carried(_) => Found::Here,
                Piece::Worn { .. } => Found::Moved,
            };
            return Some((About::Piece(piece), found));
        }
    };
    Some((about, Found::Here))
}

/// What the player's hands held when the lessons last looked: whether a
/// piece's card was open, and whose, and which horse was selected. And the
/// places opened in town since an entry of the list asked for them (#303).
#[derive(Clone, Debug, Default)]
pub struct Held {
    card: bool,
    piece: Option<Piece>,
    horse: usize,
    visited: Vec<bool>,
    /// The player was where the hints are found again (#350).
    there: bool,
}

/// Keep the hint's line to the lessons. The queue hears what the player's
/// hands did here: a piece's card opened, another horse selected, a hint put
/// away, every hint skipped. Then the game is looked at again, and the lesson to show now is
/// the hint. A sample hint (the dev key's) is left alone. What the list of
/// things to try paid for what the hands did is banked (#303), and how many
/// coins that was comes back.
pub fn follow(game: &mut Game, shell: &mut Shell, map: &Places, held: &mut Held) -> i32 {
    let hand = shell.hand();
    let open = hand.open.is_some();
    if open && !held.card {
        game.lessons.raise(Event::Card);
    }
    // A carried piece other than the newest: one chosen among the rest.
    if hand.open != held.piece
        && matches!(hand.open, Some(Piece::Carried(_)))
        && hand.open != game.newest_piece()
    {
        game.lessons.raise(Event::Another);
    }
    held.piece = hand.open;
    if open && shell.is_on(Switch::Details) {
        game.lessons.raise(Event::Detailed);
    }
    if hand.horse != held.horse {
        game.lessons.raise(Event::Selected);
        // The card stayed open over the change: it is read on two horses.
        if open && held.card {
            game.lessons.raise(Event::Compared);
        }
    }
    (held.card, held.horse) = (open, hand.horse);
    // Every place of the bar, opened in town since the list asked.
    if !game.lessons.wants(Event::Toured) {
        held.visited.clear();
    } else if game.in_town() && shell.is_open() {
        held.visited.resize(map.places.len(), false);
        if let Some(visited) = held.visited.get_mut(shell.place()) {
            *visited = true;
        }
        if held.visited.iter().all(|visited| *visited) {
            game.lessons.raise(Event::Toured);
        }
    }
    let paid = game.bank_rewards();

    // What is put away, and whether hints are off, is the shell's to say.
    let away: Vec<bool> = game.lessons.defs().iter().map(|def| shell.put_away(&def.id)).collect();
    for (lesson, away) in away.into_iter().enumerate() {
        game.lessons.put_away(lesson, away);
    }
    if shell.hints_off() != game.lessons.is_off() {
        game.lessons.turn_off(shell.hints_off());
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
            // One the player asked for is shown though hints are off.
            match game.lessons.asked().is_some() {
                true => shell.show_again(hint),
                false => {
                    shell.raise_hint(hint);
                }
            }
        }
        None if ours => shell.finish_hint(),
        _ => {}
    }

    // Skip all hints is answered once with where the hints went (#350), on
    // the line it emptied. The answer goes when the player gets there.
    let parting = game.lessons.parting().and_then(|parting| {
        let (about, _) = find(&parting.target, map, game, hand.horse)?;
        Some((about, parting.words.clone()))
    });
    if let Some((about, words)) = parting {
        let there = shell.at(about);
        if shell.parting() && there && !held.there {
            shell.finish_hint();
        } else {
            shell.say_parting(Hint { lesson: PARTING.into(), about, words });
        }
        held.there = there;
    }
    paid
}

/// What a lesson's target is called in the list of hints, there or not.
fn target_name(target: &Target, game: &Game) -> String {
    match target {
        Target::Place(name) | Target::Control(name) | Target::Tab(_, name) => name.clone(),
        Target::Horse(horse) => match game.party.get(*horse) {
            Some(horse) => horse.name.clone(),
            None => "a horse".into(),
        },
        Target::Selected => "the selected horse".into(),
        Target::Drop => "the newest piece".into(),
    }
}

/// A hint as the Journal's list has it (#300).
pub struct Listed {
    /// Its one line, as it would be said now.
    pub words: String,
    /// What became of it, where it applies, and whether a press shows it.
    pub line: String,
    /// It is the one showing.
    pub showing: bool,
    /// A press shows it again now.
    pub can: bool,
}

/// The hint at a row of the list: the lessons that have become due, in the
/// order they came. None when the list is not that long.
pub fn listed(game: &Game, map: &Places, selected: usize, row: usize) -> Option<Listed> {
    let lesson = *game.lessons.listed().get(row)?;
    let def = &game.lessons.defs()[lesson];
    let found = find(&def.target, map, game, selected).map(|(_, found)| found);
    let here = game.lessons.shown_here(lesson, game.in_town());
    let state = game.lessons.state(lesson);
    let showing = state == State::Showing;
    let became = match state {
        State::Showing => "Showing now",
        State::Done => "Done",
        State::PutAway => "Put away",
        State::Skipped => "Skipped, with hints off",
        State::Waiting => "Waiting its turn",
    };
    let place = match def.shown {
        Shown::Anywhere => "Anywhere",
        Shown::Town => "In town",
        Shown::Road => "On the road",
    };
    let press = match (showing, here, found) {
        (true, ..) => "",
        (_, false, _) if def.shown == Shown::Town => " A press shows it again in town.",
        (_, false, _) => " A press shows it again on the road.",
        (_, _, None) => " What it is about is not there now.",
        _ => " A press shows it again.",
    };
    Some(Listed {
        words: game.lessons.words(lesson, found.unwrap_or(Found::Here)).into(),
        line: format!("{became}. {place}, on {}.{press}", target_name(&def.target, game)),
        showing,
        can: !showing && here && found.is_some(),
    })
}

/// A press on a row of the list (#300): its hint is shown again, on its
/// control, if this is where it is shown and that control is there now.
/// Otherwise nothing changes, and the row's line has said why.
pub fn again(game: &mut Game, shell: &mut Shell, map: &Places, row: usize) -> bool {
    let Some(&lesson) = game.lessons.listed().get(row) else {
        return false;
    };
    let def = &game.lessons.defs()[lesson];
    let Some((about, found)) = find(&def.target, map, game, shell.horse()) else {
        return false;
    };
    let in_town = game.in_town();
    if !game.lessons.again(lesson, in_town, found) {
        return false;
    }
    shell.show_again(Hint {
        lesson: game.lessons.defs()[lesson].id.clone(),
        about,
        words: game.lessons.words(lesson, found).into(),
    });
    true
}

pub fn teach(
    mut game: ResMut<Game>,
    mut shell: ResMut<Shell>,
    map: Res<Places>,
    mut held: Local<Held>,
    mut heard: MessageWriter<Heard>,
) {
    // A coin the list pays is heard as a coin reaching the party is.
    for _ in 0..follow(&mut game, &mut shell, &map, &mut held) {
        heard.write(Heard::Collected(Kind::Coin));
    }
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

    /// Put away the horse's view a press on a horse opened.
    fn close_view(shell: &mut Shell) {
        shell.close_detail();
    }

    /// Pin the card of the piece in the cargo, as a press on it does.
    fn open_carried(shell: &mut Shell) {
        let place = shell.place();
        let tab = shell.tab(place);
        shell.show_detail(place, Detail { tab, item: Item::Carried(0), opener: None });
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

        let mut game = Game::back_with_one_piece();
        let slot = game.pending_drop().unwrap().slot.index();
        let carried = (About::Piece(Piece::Carried(0)), Found::Here);
        assert_eq!(find(&Target::Drop, &map, &game, 0), Some(carried));
        game.act(Action::Equip, Hand::horse(1));
        let worn = (About::Piece(Piece::Worn { horse: 1, slot }), Found::Moved);
        assert_eq!(find(&Target::Drop, &map, &game, 0), Some(worn));
        game.party[1].gear[slot] = None;
        assert_eq!(find(&Target::Drop, &map, &game, 0), None, "the party no longer has it");
    }

    /// Every event a lesson can wait for is one the game raises: the trail
    /// and the town by themselves, and the player's hands for the rest.
    /// So every lesson in the file names a trigger that can happen.
    #[test]
    fn the_game_raises_every_event_a_lesson_can_wait_for() {
        let map = Places::load();
        let by_hand = [
            Event::Card,
            Event::Equipped,
            Event::Selected,
            Event::Swapped,
            Event::Compared,
            Event::Toured,
            Event::SentOut,
            Event::Another,
            Event::Detailed,
        ];
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
        // Keep going's own departure is not a press of Set out.
        assert!(by_hand.iter().all(|event| !game.lessons.happened(*event)));

        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let (mut shell, mut held) = (shell(&map), Held::default());
        follow(&mut game, &mut shell, &map, &mut held);
        // Its first trail was sent out by hand; nothing else was done.
        assert!(by_hand.iter().all(|event| *event == Event::SentOut || !game.lessons.happened(*event)));
        open_carried(&mut shell);
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(game.lessons.happened(Event::Card));
        shell.inspect(1, None);
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(game.lessons.happened(Event::Selected));
        assert!(game.lessons.happened(Event::Compared), "the card was open over it");
        assert!(!game.lessons.happened(Event::Another), "the newest piece's card");
        let place = shell.place();
        let tab = shell.tab(place);
        shell.show_detail(place, Detail { tab, item: Item::Carried(1), opener: None });
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(game.lessons.happened(Event::Another));
        open_carried(&mut shell);
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(!game.lessons.happened(Event::Detailed));
        shell.press_switch(Switch::Details);
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(game.lessons.happened(Event::Detailed));
        game.act(Action::Equip, shell.hand());
        assert!(game.lessons.happened(Event::Equipped));
        // Putting the carried piece on is not a swap; taking it from the
        // horse that wears it is.
        assert!(!game.lessons.happened(Event::Swapped));
        let slot = game.party[1].gear.iter().position(|worn| worn.is_some()).unwrap();
        game.act(Action::Equip, Hand { horse: 0, open: Some(Piece::Worn { horse: 1, slot }) });
        assert!(game.lessons.happened(Event::Swapped));

        // Every place opened, once the list asks; and Set out by hand.
        assert!(game.lessons.wants(Event::Toured), "the file's list asks for no tour");
        for place in 0..map.places.len() {
            assert!(!game.lessons.happened(Event::Toured), "before place {place}");
            shell.open(place);
            follow(&mut game, &mut shell, &map, &mut held);
        }
        assert!(game.lessons.happened(Event::Toured));
        assert!(game.lessons.happened(Event::SentOut));

        for def in game.lessons.defs() {
            assert!(Event::ALL.contains(&def.due), "{}", def.id);
        }
        for def in game.lessons.entries() {
            assert!(Event::ALL.contains(&def.due) && Event::ALL.contains(&def.done), "{}", def.id);
        }
    }

    /// The list of the file itself (#303), in the first town with a piece,
    /// by what each entry waits for and never by its name or words. Nothing
    /// is listed before a trail brings a piece back. Each entry is ticked
    /// by doing its thing and pays once, into the party's count. Setting
    /// out with entries undone loses nothing. A party that leaves by Keep
    /// going has not pressed Set out. Nobody at it earns nothing.
    #[test]
    fn the_list_is_ticked_by_doing_and_pays_into_the_count() {
        let map = Places::load();
        let fresh = Game::new(6);
        assert!(!fresh.lessons.list_showing(), "before any trail");
        let home = || {
            (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some())
                .expect("no seed under 50 brings a drop home")
        };
        let done = |game: &Game, event: Event| {
            let entries = game.lessons.entries().iter().enumerate();
            let mut waiting = entries.filter(|(_, def)| def.done == event).peekable();
            assert!(waiting.peek().is_some(), "no entry waits for {event:?}");
            waiting.all(|(entry, _)| game.lessons.tick(entry).done)
        };
        let all = [Event::Selected, Event::Card, Event::Compared, Event::Equipped, Event::Toured, Event::SentOut];
        let price = |game: &Game, event: Event| -> i32 {
            let entries = game.lessons.entries().iter();
            entries.filter(|def| def.done == event).map(|def| def.coins).sum()
        };

        // Nobody at it: the list is there, nothing is ticked, nothing paid,
        // and the party leaves by Keep going with Set out still to try.
        let mut game = home();
        let (mut shell, mut held) = (shell(&map), Held::default());
        let coins = game.coins_earned();
        game.act(Action::KeepGoing, Hand::default());
        while game.in_town() {
            assert_eq!(follow(&mut game, &mut shell, &map, &mut held), 0);
            assert!(game.lessons.list_showing());
            pass(&mut game);
        }
        assert_eq!(game.coins_earned(), coins);
        assert!(all.iter().all(|event| !done(&game, *event)));

        // With a player at it, one thing at a time.
        let mut game = home();
        (shell, held) = (Shell::new(map.pages().count()), Held::default());
        let mut coins = game.coins_earned();
        assert_eq!(follow(&mut game, &mut shell, &map, &mut held), 0);
        let mut paid = |game: &mut Game, shell: &mut Shell, event: Event| {
            let got = follow(game, shell, &map, &mut held);
            assert!(done(game, event), "{event:?}");
            assert_eq!(got, price(game, event), "{event:?}");
            coins += got;
            assert_eq!(game.coins_earned(), coins, "{event:?}");
        };
        shell.inspect(1, None);
        paid(&mut game, &mut shell, Event::Selected);
        assert!(!done(&game, Event::Compared), "no card was open");
        close_view(&mut shell);
        open_carried(&mut shell);
        paid(&mut game, &mut shell, Event::Card);
        shell.inspect(2, None);
        paid(&mut game, &mut shell, Event::Compared);
        game.act(Action::Equip, shell.hand());
        paid(&mut game, &mut shell, Event::Equipped);

        // Away with two undone, and back: they are still there to do, and
        // what was done is not paid again.
        game.act(Action::KeepGoing, Hand::default());
        let mut left = false;
        for _ in 0..20_000 {
            pass(&mut game);
            assert_eq!(follow(&mut game, &mut shell, &map, &mut held), 0);
            left |= !game.in_town();
            if left && game.in_town() {
                break;
            }
        }
        assert!(left && game.in_town(), "the party did not go and come back");
        coins = game.coins_earned();
        assert!(game.lessons.list_showing());
        assert!(!done(&game, Event::Toured) && !done(&game, Event::SentOut));
        for place in 0..map.places.len() {
            assert!(!done(&game, Event::Toured), "before place {place}");
            shell.open(place);
            follow(&mut game, &mut shell, &map, &mut held);
        }
        assert!(done(&game, Event::Toured));
        coins += price(&game, Event::Toured);
        assert_eq!(game.coins_earned(), coins);
        game.act(Action::SetOut, shell.hand());
        assert_eq!(follow(&mut game, &mut shell, &map, &mut held), price(&game, Event::SentOut));
        assert!(all.iter().all(|event| done(&game, *event)));
        assert!(!game.lessons.list_showing(), "on the road");
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
            let hands = [Hand::default(), Hand { horse: 1, open: Some(Piece::Carried(0)) }];
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
        assert_eq!(shell.hint().unwrap().about, About::Piece(Piece::Carried(0)));
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
    /// then finds its piece where it is now: on the horse the player put it
    /// on with its card unopened, with the line for that. The dev key's
    /// sample is left alone.
    #[test]
    fn a_lesson_left_in_town_finds_its_piece_again() {
        let map = Places::load();
        let (mut shell, mut held) = (shell(&map), Held::default());
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        game.lessons = lessons();
        game.lessons.raise(Event::Gear);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("piece"));
        assert_eq!(shell.hint().unwrap().words, "piece.");

        // Equip by its key, the card never opened, and away.
        game.act(Action::Equip, Hand::horse(1));
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
        outcome.drops.clear();
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

    /// The Journal's list (#300), with a player at it. No hint is listed
    /// before its event has happened. One put away is listed and a press
    /// shows it again; one done too. With hints turned off none is shown,
    /// the list is still read, and a press still shows one. A row says why
    /// a press cannot show it, and then the press changes nothing.
    #[test]
    fn a_hint_is_found_again_in_the_journal() {
        let map = Places::load();
        let (mut shell, mut held) = (shell(&map), Held::default());
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        game.lessons = lessons();
        let row = |game: &Game, row: usize| listed(game, &map, 0, row);
        assert!(row(&game, 0).is_none(), "nothing has happened");

        // The piece's lesson comes due, shows, and is put away.
        game.lessons.raise(Event::Gear);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("piece"));
        let first = row(&game, 0).expect("it is listed once it is due");
        assert!(first.showing && !first.can);
        assert!(first.line.starts_with("Showing now. In town, on "), "{}", first.line);
        assert!(row(&game, 1).is_none(), "one event, one lesson");
        assert!(shell.dismiss_hint());
        follow(&mut game, &mut shell, &map, &mut held);
        let first = row(&game, 0).unwrap();
        assert_eq!(first.words, "piece.");
        assert!(first.can && first.line.starts_with("Put away. "), "{}", first.line);
        assert!(first.line.ends_with("A press shows it again."), "{}", first.line);

        // A press: it is on the piece again, and is no longer put away.
        assert!(again(&mut game, &mut shell, &map, 0));
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("piece"));
        assert_eq!(shell.hint().unwrap().about, About::Piece(Piece::Carried(0)));
        assert!(!shell.put_away("piece"));

        // Its thing done: the next comes due, and the first is listed done.
        open_carried(&mut shell);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("equip"));
        assert!(row(&game, 0).unwrap().line.starts_with("Done. "));
        assert!(row(&game, 1).unwrap().showing);

        // Hints turned off: none shows, the list is read, a press shows one.
        shell.press_switch(crate::game::menu::shell::Switch::Hints);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);
        assert!(game.lessons.is_off());
        let second = row(&game, 1).unwrap();
        assert!(second.can && second.line.starts_with("Skipped, with hints off. "), "{}", second.line);
        game.lessons.raise(Event::Fight);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None, "a new hint with hints off");
        assert!(row(&game, 2).is_some(), "and it is listed");
        assert!(again(&mut game, &mut shell, &map, 0), "one that was done");
        follow(&mut game, &mut shell, &map, &mut held);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("piece"), "asked for, though hints are off");
        assert!(shell.dismiss_hint());
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);

        // The road's lesson cannot be shown in town, and says so.
        let road = row(&game, 2).unwrap();
        assert!(!road.can && road.line.ends_with("A press shows it again on the road."), "{}", road.line);
        let before = shell.clone();
        assert!(!again(&mut game, &mut shell, &map, 2));
        assert!(!again(&mut game, &mut shell, &map, 9), "no such row");
        assert_eq!(shell, before);

        // On the road it can, and the town's cannot.
        game.act(Action::SetOut, Hand::default());
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(row(&game, 2).unwrap().can);
        let town = row(&game, 1).unwrap();
        assert!(!town.can && town.line.ends_with("A press shows it again in town."), "{}", town.line);
        assert!(again(&mut game, &mut shell, &map, 2));
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some("fight"));
    }

    /// Skip all hints is answered once (#350), by the file's own words and
    /// on the file's own target, which exists. It is on the line though
    /// hints are off, the game's lessons are off under it, and it goes when
    /// the player gets to where it points. It is not said a second time, and
    /// the switch in Settings never raises it. With no answer in the file
    /// nothing is said.
    #[test]
    fn skip_all_hints_is_answered_once_with_where_they_went() {
        let map = Places::load();
        let home = || {
            (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some())
                .expect("no seed under 50 brings a drop home")
        };
        let mut game = home();
        let (mut shell, mut held) = (shell(&map), Held::default());
        let parting = game.lessons.parting().expect("the file has no answer").clone();
        let (about, _) = find(&parting.target, &map, &game, 0).expect("its target is not in places.ron");
        assert!(!shell.at(about), "the walk starts where the hints are kept");

        follow(&mut game, &mut shell, &map, &mut held);
        assert!(shell.hint().is_some() && !shell.parting(), "a lesson shows in the first town");
        shell.skip_hints();
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some(PARTING));
        assert_eq!(shell.hint().unwrap().about, about);
        assert_eq!(shell.hint().unwrap().words, parting.words);
        assert!(game.lessons.is_off() && shell.hints_off());
        for _ in 0..3 {
            follow(&mut game, &mut shell, &map, &mut held);
            assert_eq!(hinted(&shell), Some(PARTING), "it stays until it is seen");
        }

        // The player goes there: it has been seen.
        let About::Tab(place, tab) = about else {
            panic!("this walk is of a tab: {about:?}");
        };
        shell.open(place);
        shell.choose_tab(place, tab);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);

        // Hints on again, and skipped again: not a second time.
        shell.open(0);
        shell.press_switch(crate::game::menu::shell::Switch::Hints);
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(!game.lessons.is_off() && !shell.parting());
        shell.skip_hints();
        for _ in 0..3 {
            follow(&mut game, &mut shell, &map, &mut held);
            assert_eq!(hinted(&shell), None);
        }

        // Turned off in Settings: never.
        let mut game = home();
        (shell, held) = (Shell::new(map.pages().count()), Held::default());
        follow(&mut game, &mut shell, &map, &mut held);
        shell.press_switch(crate::game::menu::shell::Switch::Hints);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);

        // Skipped while already there: it is said, and Got it puts it away.
        let mut game = home();
        (shell, held) = (Shell::new(map.pages().count()), Held::default());
        shell.open(place);
        shell.choose_tab(place, tab);
        follow(&mut game, &mut shell, &map, &mut held);
        shell.skip_hints();
        follow(&mut game, &mut shell, &map, &mut held);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some(PARTING));
        assert!(shell.dismiss_hint());
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);

        // A file with no answer says nothing.
        let mut game = home();
        game.lessons = lessons();
        (shell, held) = (Shell::new(map.pages().count()), Held::default());
        shell.skip_hints();
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), None);
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
        assert_eq!(shell.hint().unwrap().about, About::Piece(Piece::Carried(0)));

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
        assert_eq!(hand, Hand { horse: 2, open: Some(Piece::Carried(0)) });
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

    /// A trail that brings back several pieces (#352) makes a choice among
    /// them, and the file's lesson for it is taught by making one: it comes
    /// after the card's own, and goes when the card of a carried piece other
    /// than the newest is opened, and not for the newest's. One piece home
    /// does not make it due.
    #[test]
    fn several_pieces_are_a_choice_taught_by_opening_another() {
        let map = Places::load();
        let mut game = Game::back_from_a_trail(6);
        assert!(game.cargo().len() > 1, "seed 6's first trail brings back several pieces");
        assert!(game.lessons.happened(Event::Several));
        let choose = game
            .lessons
            .defs()
            .iter()
            .position(|def| def.due == Event::Several)
            .expect("lessons.ron: no lesson for several pieces");
        assert_eq!(game.lessons.defs()[choose].done, Done::Did(Event::Another));
        assert_eq!(game.newest_piece(), Some(Piece::Carried(0)));

        // The newest piece's card, on another horse: the card's lessons.
        let (mut shell, mut held) = (shell(&map), Held::default());
        follow(&mut game, &mut shell, &map, &mut held);
        open_carried(&mut shell);
        follow(&mut game, &mut shell, &map, &mut held);
        shell.inspect(1, None);
        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(game.lessons.showing().map(|def| def.due), Some(Event::Several));
        let inventory = map.page_named("Inventory").unwrap();
        assert_eq!(shell.hint().unwrap().about, About::Place(inventory));
        assert!(!game.lessons.happened(Event::Another), "the newest is no choice among them");

        // Another piece's card: the choice is made, and the lesson goes.
        let place = shell.place();
        let tab = shell.tab(place);
        shell.show_detail(place, Detail { tab, item: Item::Carried(1), opener: None });
        follow(&mut game, &mut shell, &map, &mut held);
        assert!(game.lessons.happened(Event::Another));
        assert!(game.lessons.progress(choose).done);
        assert_ne!(game.lessons.showing().map(|def| def.due), Some(Event::Several));

        // One piece home: nothing to choose among.
        let one = Game::back_with_one_piece().last_report().unwrap().outcome.clone();
        let mut game = Game::new(0);
        game.return_to_camp(one);
        assert!(game.lessons.happened(Event::Gear) && !game.lessons.happened(Event::Several));
    }
}
