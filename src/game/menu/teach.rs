//! The lessons in the menus (#290): the queue of `sim::lessons` is told what
//! the player's hands did, is asked which lesson to show, and that lesson's
//! one line goes in the hint's pop-up beside the control it is about (#353)
//! (#291). How it works: docs/lessons.md.
//!
//! Nothing here reaches the simulation or a control's availability. What a
//! lesson's pop-up does to the game is to stop its clock while it is up, as
//! the owner decided (2026-10-12): the fight, the walk and the town's wait
//! go on from where they were when it is answered.

use bevy::prelude::*;

use super::shell::{About, Hint, PARTING, Places, Shell, Switch};
use crate::fx::drops::Kind;
use crate::fx::sound::Heard;
use crate::game::{Game, Piece, Uncollected, change_line};
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
        Target::Party => About::Party,
        // On the boss in the scene, while its fight is on (#360).
        Target::Boss => About::Boss(game.boss()?.0),
        // On the horse whose ability was resisted the most last trail (#357).
        Target::Resisted => About::Horse(game.most_resisted()?),
        // On the horse a piece was last put on, while that stands (#298).
        Target::Equipped => About::Horse(game.last_equipped()?.0),
    };
    Some((about, Found::Here))
}

/// A lesson's line as it is said now: with what the game fills in (#298),
/// `{change}` the last-equipped horse's stat as it was and as it is, and
/// `{level}` the trail level.
fn say(game: &Game, lesson: usize, found: Found) -> String {
    let words = game.lessons.words(lesson, found);
    let change = match game.last_equipped() {
        Some((_, was, now)) => change_line(&game.data, was, now),
        None => "no stat changed".into(),
    };
    words.replace("{change}", &change).replace("{level}", &game.plan.level.to_string())
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

/// Keep the hint to the lessons. The queue hears what the player's
/// hands did here: a piece's card opened, another horse selected, a hint put
/// away, every hint skipped. Then the game is looked at again, and the lesson to show now is
/// the hint. A sample hint (the dev key's) is left alone. What the list of
/// things to try paid for what the hands did is banked (#303), and how many
/// coins that was comes back.
#[cfg(test)]
pub fn follow(game: &mut Game, shell: &mut Shell, map: &Places, held: &mut Held) -> i32 {
    follow_over(game, shell, map, held, false)
}

/// `follow`, with a kill's drops still in the world (`aloft`): in the air,
/// on the ground or on their way to the party. No new lesson comes up then,
/// so that none stops a fountain where it is or covers it (#353); it is
/// asked for again once every piece has arrived.
pub fn follow_over(game: &mut Game, shell: &mut Shell, map: &Places, held: &mut Held, aloft: bool) -> i32 {
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
    let ours = shell.hint().is_some_and(|hint| game.lessons.knows(&hint.lesson));
    // The queue is not asked while drops are aloft, so no lesson counts as
    // showing, and none is watched, while it is not on the screen.
    let wanted = match aloft && !ours {
        true => None,
        false => game.lessons.show(in_town, &places).and_then(|lesson| {
            let (about, at) = found[lesson]?;
            Some(Hint {
                lesson: game.lessons.defs()[lesson].id.clone(),
                about,
                words: say(game, lesson, at),
            })
        }),
    };

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
        Target::Party => "the party".into(),
        Target::Boss => "the boss".into(),
        Target::Resisted => "the horse whose ability was resisted".into(),
        Target::Equipped => "the horse the piece went on".into(),
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
        State::Left => "Left in town twice",
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
        words: say(game, lesson, found.unwrap_or(Found::Here)),
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
        words: say(game, lesson, found),
    });
    true
}

/// Whether a lesson's pop-up stops the game while it is up (#353). On in
/// the game; off for a run nobody is at that keeps the times it had before
/// the pause (`--no-pause`, and the soak).
#[derive(Resource)]
pub struct Pause(pub bool);

/// Whether the game waits on the pop-up that is up: a lesson's, shown by
/// itself or asked for again. The dev key's sample and the answer to Skip
/// all hints are not lessons, and hold nothing up.
pub fn waits_on(game: &Game, shell: &Shell) -> bool {
    shell.hint().is_some_and(|hint| game.lessons.knows(&hint.lesson))
}

pub fn teach(
    mut game: ResMut<Game>,
    mut shell: ResMut<Shell>,
    map: Res<Places>,
    uncollected: Res<Uncollected>,
    pause: Res<Pause>,
    mut clock: ResMut<Time<Virtual>>,
    mut held: Local<Held>,
    mut heard: MessageWriter<Heard>,
) {
    let aloft = uncollected.pieces > 0;
    // A coin the list pays is heard as a coin reaching the party is.
    for _ in 0..follow_over(&mut game, &mut shell, &map, &mut held, aloft) {
        heard.write(Heard::Collected(Kind::Coin));
    }
    // The game's clock: the sim, the scene and the drops run on it. The
    // menus run on the real one, so the pop-up and its controls still move.
    match pause.0 && waits_on(&game, &shell) {
        true if !clock.is_paused() => clock.pause(),
        false if clock.is_paused() => clock.unpause(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::menu::shell::{Detail, Item, SAMPLE, sample_hint};
    use crate::game::{Action, CAMP_SECS, Hand, Screen, TICK, step};
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

    /// The lesson of a resisted ability (#357) is on the horse whose ability
    /// was resisted the most on the last trail, in town only, and is done by
    /// giving a horse another ability. Before any trail it is nowhere.
    #[test]
    fn a_resisted_ability_is_taught_on_its_horse() {
        let map = Places::load();
        assert_eq!(find(&Target::Resisted, &map, &Game::new(6), 0), None, "no trail walked");
        let def = Game::new(6)
            .lessons
            .defs()
            .iter()
            .find(|def| def.target == Target::Resisted)
            .cloned()
            .expect("a lesson for a resisted ability");
        assert_eq!((def.due, def.shown, def.done), (Event::Resisted, Shown::Town, Done::Did(Event::Chose)));
        let game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.last_report().is_some_and(|kept| kept.outcome.resisted_hits > 0))
            .expect("no seed under 50 has a resisted hit");
        let deeds = &game.last_report().unwrap().outcome.deeds;
        let horse = game.most_resisted().unwrap();
        let most = deeds[horse].resisted_hits;
        assert!(most > 0 && deeds.iter().all(|deeds| deeds.resisted_hits <= most));
        assert_eq!(find(&Target::Resisted, &map, &game, 0), Some((About::Horse(horse), Found::Here)));
        assert!(game.lessons.happened(Event::Resisted));
        // Only an ability's hit is ever resisted: the Ward's horse never is.
        let ward = game.data.ability_index("ward");
        for (fighter, deeds) in game.party.iter().zip(deeds) {
            assert!(fighter.ability != ward || deeds.resisted_hits == 0);
        }
    }

    /// After an Equip the lesson is on the horse the piece went on, and says
    /// what it changed there as it was and as it is (#298), from the stats
    /// themselves. Undo takes the change back, and the lesson's horse with
    /// it; the trail level is said as the footer says it.
    #[test]
    fn the_equip_lesson_says_what_it_changed() {
        let map = Places::load();
        let mut game = Game::back_with_one_piece();
        let lesson = |id: &str, target, words: &str| LessonDef {
            id: id.into(),
            due: Event::Gear,
            shown: Shown::Town,
            target,
            words: words.into(),
            done: Done::Did(Event::SetOut),
            instead: None,
        };
        game.lessons = Lessons::new(vec![
            lesson("on", Target::Equipped, "{change}, with it on."),
            lesson("level", Target::Party, "Level {level}."),
        ]);
        assert_eq!(find(&Target::Equipped, &map, &game, 0), None, "nothing put on yet");
        let was = game.party[1].stats();
        game.act(Action::Equip, Hand::horse(1));
        let now = game.party[1].stats();
        assert_ne!(was, now);
        assert_eq!(game.last_equipped(), Some((1, was, now)));
        assert_eq!(find(&Target::Equipped, &map, &game, 0), Some((About::Horse(1), Found::Here)));
        let line = say(&game, 0, Found::Here);
        assert_eq!(line, format!("{}, with it on.", change_line(&game.data, was, now)));
        assert_ne!(line, "no stat changed, with it on.");
        assert_eq!(say(&game, 1, Found::Here), format!("Level {}.", game.plan.level));

        game.act(Action::Undo, Hand::default());
        assert_eq!(game.last_equipped(), None);
        assert_eq!(find(&Target::Equipped, &map, &game, 0), None);
    }

    /// The change is said by the stat that moved the most, health counted
    /// in stat points, and how many more moved; numbers only.
    #[test]
    fn a_change_is_said_by_its_biggest_stat() {
        use crate::sim::data::{Data, Stats};
        let data = Data::load();
        let per = data.gear.health_per_point;
        let was = Stats { health: 40, attack: 10, defense: 2, elemental: 0, speed: 3 };
        assert_eq!(change_line(&data, was, was), "no stat changed");
        let attack = Stats { attack: 13, ..was };
        assert_eq!(change_line(&data, was, attack), "attack 10 to 13");
        // Health by its points: one point's worth less than attack's three.
        let both = Stats { health: 40 + per, ..attack };
        assert_eq!(change_line(&data, was, both), "attack 10 to 13 and 1 more");
        let more = Stats { health: 40 + per * 4, ..both };
        assert_eq!(change_line(&data, was, more), format!("health 40 to {} and 1 more", 40 + per * 4));
        // A loss is said as plainly as a gain.
        let less = Stats { defense: 0, ..was };
        assert_eq!(change_line(&data, was, less), "defense 2 to 0");
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
                // Found in a boss fight: `the_boss_is_met_once`.
                // Found after a resisted trail: `a_resisted_ability_is_taught_on_its_horse`.
                // Found after an Equip: `the_equip_lesson_says_what_it_changed`.
                Target::Drop | Target::Boss | Target::Resisted | Target::Equipped => {}
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

    /// A new game (#292) is on the road from the first frame, with no wait
    /// and nothing to press. Until the first fight the only line said is
    /// on no control, so nothing looks as if it wants a press; the fight
    /// beginning puts it away.
    #[test]
    fn a_new_game_opens_on_the_road_with_nothing_marked() {
        let map = Places::load();
        let mut game = Game::opening(6);
        let mut shell = shell(&map);
        let mut held = Held::default();
        assert!(!game.in_town() && game.party_moving(), "walking from the first frame");
        assert_eq!(game.leaving_in(), None);
        let at = game.lessons.defs().iter().position(|def| def.target == Target::Party);
        let at = at.expect("lessons.ron has the opening's line");
        let opening = game.lessons.defs()[at].id.clone();

        follow(&mut game, &mut shell, &map, &mut held);
        assert_eq!(hinted(&shell), Some(opening.as_str()), "said from the first frame");
        while !game.lessons.happened(Event::Fight) {
            assert_eq!(shell.hint().map(|hint| hint.about), Some(About::Party));
            pass(&mut game);
            follow(&mut game, &mut shell, &map, &mut held);
        }
        assert_ne!(hinted(&shell), Some(opening.as_str()), "the fight puts it away");
        assert_eq!(game.lessons.state(at), State::Done);
    }

    /// The boss's lesson (#360), from `lessons.ron`: it comes up beside the
    /// boss as its fight begins, with nothing aloft, and the game waits on
    /// it until Got it; the fight then goes on. At the next trail's boss it
    /// is not shown again.
    #[test]
    fn the_boss_is_met_once() {
        let map = Places::load();
        let mut game = Game::opening(6);
        game.keep_going = true;
        let (mut shell, mut held) = (shell(&map), Held::default());
        let boss = game.lessons.defs().iter().position(|def| def.target == Target::Boss);
        let boss = game.lessons.defs()[boss.expect("lessons.ron has the boss's lesson")].id.clone();
        assert_eq!(find(&Target::Boss, &map, &game, 0), None, "no boss on the way out");

        let mut met = 0;
        let mut fought = false;
        for _ in 0..20000 {
            follow(&mut game, &mut shell, &map, &mut held);
            let at_boss = game.boss().is_some();
            if at_boss && !fought {
                // The fight has just begun: nothing has been struck yet.
                met += 1;
                if met == 1 {
                    let (slot, _) = game.boss().unwrap();
                    assert_eq!(hinted(&shell), Some(boss.as_str()), "said as the fight begins");
                    assert_eq!(shell.hint().unwrap().about, About::Boss(slot));
                    assert!(waits_on(&game, &shell), "the game waits on it");
                } else {
                    assert_ne!(hinted(&shell), Some(boss.as_str()), "said at a later boss");
                }
            }
            fought = at_boss;
            // Anything showing is answered, as a player would.
            if waits_on(&game, &shell) {
                shell.dismiss_hint();
                follow(&mut game, &mut shell, &map, &mut held);
            }
            if met >= 2 && !at_boss {
                break;
            }
            pass(&mut game);
        }
        assert!(met >= 2, "two boss fights were met, {met}");
        let at = game.lessons.defs().iter().position(|def| def.id == boss).unwrap();
        assert_eq!(game.lessons.state(at), State::PutAway, "answered by Got it");
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
            Event::Dragged,
            Event::Chose,
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
        // Giving a horse another ability (#357).
        assert!(!game.lessons.happened(Event::Chose));
        let other = (game.party[0].ability + 1) % game.data.abilities.len();
        game.act(Action::Ability(other), Hand::horse(0));
        assert!(game.lessons.happened(Event::Chose));
        // Putting the carried piece on is not a swap; taking it from the
        // horse that wears it is.
        assert!(!game.lessons.happened(Event::Swapped));
        let slot = game.party[1].gear.iter().position(|worn| worn.is_some()).unwrap();
        game.act(Action::Equip, Hand { horse: 0, open: Some(Piece::Worn { horse: 1, slot }) });
        assert!(game.lessons.happened(Event::Swapped));
        // A drag is its own event as well as an Equip (#228).
        assert!(!game.lessons.happened(Event::Dragged));
        assert!(game.drop_piece(Piece::Worn { horse: 0, slot }, 1, Some(slot)).is_some());
        assert!(game.lessons.happened(Event::Dragged));

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
            Action::Ability(1),
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
        // Who wears it is chosen: on Another horse, the list beside Equip.
        let others = map.control_named("Another horse").unwrap();
        assert_eq!(shell.hint().unwrap().about, About::Control(others));

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

    /// The party leaving town by Keep going at each step of the first item
    /// (#297), the card open for some of them, by the file's own lessons and
    /// never their names. It leaves on the tick a game without lessons
    /// leaves on, with every piece it had where it was, and no town lesson
    /// is left up. A lesson it cut off is waiting, not put
    /// away; it is offered at the next town, and not at the one after when
    /// the party leaves it again.
    #[test]
    fn setting_out_at_any_step_of_a_town_lesson_loses_nothing() {
        let map = Places::load();
        let home = |lessons: Lessons| {
            let mut game = (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some())
                .expect("no seed under 50 brings a drop home");
            game.lessons = lessons;
            game.lessons.raise(Event::Gear);
            game
        };
        // A player's way through the first item, a step at a time.
        let steps: [fn(&mut Game, &mut Shell); 4] = [
            |_, _| {},
            |_, shell| open_carried(shell),
            |_, shell| shell.inspect(2, None),
            |game, shell| {
                game.act(Action::Equip, shell.hand());
            },
        ];
        let pieces = |game: &Game| {
            let worn = game.party.iter().map(|horse| horse.gear.clone()).collect::<Vec<_>>();
            (game.cargo().to_vec(), worn)
        };
        // Home again from a trail that brings nothing new.
        let walk_home = |game: &mut Game| {
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
        };
        let town = |game: &Game, lesson: usize| game.lessons.defs()[lesson].shown == Shown::Town;
        let mut cut_off = 0;
        for upto in 0..steps.len() {
            let mut game = home(Lessons::load());
            let mut plain = home(Lessons::new(Vec::new()));
            let (mut shell, mut held) = (shell(&map), Held::default());
            let (mut plain_shell, mut plain_held) = (Shell::new(map.pages().count()), Held::default());
            follow(&mut game, &mut shell, &map, &mut held);
            for step in &steps[..=upto] {
                step(&mut game, &mut shell);
                step(&mut plain, &mut plain_shell);
                follow(&mut game, &mut shell, &map, &mut held);
                follow(&mut plain, &mut plain_shell, &map, &mut plain_held);
            }
            let showing = game.lessons.defs().iter().position(|def| Some(def) == game.lessons.showing());
            let had = pieces(&game);

            // Keep going, and the countdown runs out with the card as it is.
            game.act(Action::KeepGoing, Hand::default());
            plain.act(Action::KeepGoing, Hand::default());
            while game.in_town() || plain.in_town() {
                assert_eq!(game.in_town(), plain.in_town(), "step {upto}: not on time");
                pass(&mut game);
                pass(&mut plain);
                follow(&mut game, &mut shell, &map, &mut held);
            }
            assert_eq!(pieces(&game), had, "step {upto}: something was lost");
            if let Some(hint) = shell.hint() {
                let lesson = game.lessons.defs().iter().position(|def| def.id == hint.lesson).unwrap();
                assert!(!town(&game, lesson), "step {upto}: {} on the road", hint.lesson);
            }
            assert!((0..game.lessons.defs().len()).all(|lesson| !game.lessons.progress(lesson).skipped));
            let Some(left) = showing.filter(|&lesson| town(&game, lesson) && !game.lessons.progress(lesson).done)
            else {
                continue;
            };
            cut_off += 1;
            assert_eq!(game.lessons.state(left), State::Waiting, "step {upto}");

            // The next town offers it again; left again, the one after does not.
            walk_home(&mut game);
            follow(&mut game, &mut shell, &map, &mut held);
            assert_eq!(game.lessons.showing(), Some(&game.lessons.defs()[left]), "step {upto}");
            game.act(Action::SetOut, Hand::default());
            follow(&mut game, &mut shell, &map, &mut held);
            assert_eq!(game.lessons.state(left), State::Left, "step {upto}");
            walk_home(&mut game);
            follow(&mut game, &mut shell, &map, &mut held);
            assert_ne!(game.lessons.showing(), Some(&game.lessons.defs()[left]), "step {upto}");
            let row = game.lessons.listed().iter().position(|&lesson| lesson == left).unwrap();
            let line = listed(&game, &map, 0, row).unwrap();
            assert!(line.can && line.line.starts_with("Left in town twice. "), "{}", line.line);
        }
        assert!(cut_off > 1, "no step of the first item was cut off");
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

    /// While a kill's drops are in the world no lesson comes up (#353), and
    /// the queue counts none as showing, so none is watched unseen. Once
    /// they have arrived it comes up. One already up stays up.
    #[test]
    fn no_lesson_comes_up_over_a_fountain() {
        let map = Places::load();
        let mut game = Game::opening(6);
        let (mut shell, mut held) = (shell(&map), Held::default());
        let opening = game.lessons.defs().iter().position(|def| def.target == Target::Party).unwrap();

        for _ in 0..5 {
            follow_over(&mut game, &mut shell, &map, &mut held, true);
            assert_eq!(shell.hint(), None);
            assert_eq!(game.lessons.showing(), None);
        }
        // Its fight begins while it is held back: it was never seen, so it
        // was not watched.
        while !game.lessons.happened(Event::Fight) {
            pass(&mut game);
            follow_over(&mut game, &mut shell, &map, &mut held, true);
        }
        assert!(!game.lessons.progress(opening).done && !game.lessons.progress(opening).seen);

        follow_over(&mut game, &mut shell, &map, &mut held, false);
        let up = shell.hint().cloned().expect("a lesson comes up once the drops are in");
        follow_over(&mut game, &mut shell, &map, &mut held, true);
        assert_eq!(shell.hint(), Some(&up), "one up stays up");
    }

    /// While a lesson's pop-up is up the game's clock stops (#353): frame
    /// after frame no sim time passes, on the road and in town. Answered,
    /// the game goes on from where it was. With the pause off (`--no-pause`)
    /// it never stops; the dev key's sample stops nothing.
    #[test]
    fn a_lessons_popup_stops_the_game_until_it_is_answered() {
        use crate::fx::sound::Heard;
        use crate::game::{Cast, Killed, Landed, Withdrew, advance};
        use bevy::time::{TimePlugin, TimeUpdateStrategy};
        use std::time::Duration;

        let app = |game: Game, pause: bool| {
            let map = Places::load();
            let mut app = App::new();
            app.add_plugins(TimePlugin)
                .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(TICK)))
                .insert_resource(shell(&map))
                .insert_resource(map)
                .insert_resource(game)
                .insert_resource(Uncollected::default())
                .insert_resource(Pause(pause))
                .add_message::<Heard>()
                .add_message::<Killed>()
                .add_message::<Withdrew>()
                .add_message::<Landed>()
                .add_message::<Cast>()
                .add_systems(Update, (advance, teach).chain());
            app
        };
        // Where the game is: the trail to the last detail, the time not yet
        // stepped, and the town's wait.
        let state = |app: &App| {
            let game = app.world().resource::<Game>();
            let screen = match &game.screen {
                Screen::Trail(expedition) => format!("{expedition:?}"),
                Screen::Camp { countdown, .. } => format!("town {countdown}"),
            };
            (screen, game.accumulator, game.arriving)
        };
        let up = |app: &App| waits_on(app.world().resource::<Game>(), app.world().resource::<Shell>());

        // On the road: the opening comes up on the first frame.
        let mut road = app(Game::opening(6), true);
        road.update();
        assert!(up(&road));
        let before = state(&road);
        for _ in 0..100 {
            road.update();
            assert_eq!(state(&road), before, "ten seconds with the pop-up up");
        }
        assert!(road.world().resource::<Time<Virtual>>().is_paused());

        // Got it: the walk goes on from there, a tick a frame.
        road.world_mut().resource_mut::<Shell>().dismiss_hint();
        road.update();
        road.update();
        assert!(!up(&road) && !road.world().resource::<Time<Virtual>>().is_paused());
        assert_ne!(state(&road), before, "going again");

        // In town: the wait to leave stops too.
        let mut town = Game::back_with_one_piece();
        town.keep_going = true;
        let mut town = app(town, true);
        town.update();
        assert!(up(&town), "the piece's lesson");
        let before = state(&town);
        for _ in 0..200 {
            town.update();
        }
        assert_eq!(state(&town), before);
        assert!(town.world().resource::<Game>().in_town(), "it did not leave by itself");

        // With the pause off nothing stops.
        let mut free = app(Game::opening(6), false);
        free.update();
        assert!(up(&free));
        let before = state(&free);
        free.update();
        free.update();
        assert_ne!(state(&free), before);

        // The sample is not a lesson.
        let mut sample = app(Game::opening(6), true);
        sample.world_mut().resource_mut::<Shell>().skip_hints();
        let hint = sample_hint(&Places::load(), 0);
        sample.world_mut().resource_mut::<Shell>().show_again(hint);
        sample.update();
        let before = state(&sample);
        sample.update();
        sample.update();
        assert_ne!(state(&sample), before);
    }
}
