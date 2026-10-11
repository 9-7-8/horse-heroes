//! The item card (#228): the one way a piece of gear is described, wherever
//! it is shown. The summary is what a list shows of it; the pinned card is
//! what the side panel shows when it is pressed, with the comparison against
//! the selected horse and what Equip would do. Its actions are the `card`
//! controls of `assets/data/places.ron`. How it works: docs/item-card.md.

use crate::game::{Action, Game, Hand, Piece, stats_line};
use crate::sim::data::{Slot, Stats};
use crate::sim::loot::{Gear, Horse, gain};

/// What a list says where the cargo is empty.
pub const NOTHING_CARRIED: &str = "Nothing here yet.\nGear the horses bring back is carried here.";

/// What the Inventory says of a piece in the cargo, where a worn piece says
/// who wears it.
pub const CARRIED: &str = "Carried in the cargo";

/// What the Inventory says under a type the party has no piece of.
pub fn none_held(kind: &str) -> String {
    format!(
        "No {} piece is worn or carried yet. Gear comes from the trail.",
        kind.to_lowercase()
    )
}

/// Under a piece's name in a list: its rarity, its slot and what it adds.
pub fn kind(game: &Game, gear: &Gear) -> String {
    format!(
        "{}, {}  {}",
        game.data.gear.rarities[gear.rarity].name,
        gear.slot.name(),
        stats_line(gear.stats),
    )
}

/// Whether a piece matters to a horse now, short enough for a list: a word
/// and, where there is a difference, its sign.
pub fn verdict(game: &Game, horse: &Horse, gear: &Gear) -> String {
    match gain(&game.data, horse, gear) {
        change @ 1.. => format!("Upgrade for {}: +{change}", horse.name),
        0 => format!("No better for {}", horse.name),
        change => format!("Worse for {}: {change}", horse.name),
    }
}

/// A piece on a horse in place of what it wears in that slot: each stat of
/// the horse that would change, as it is and as it would be, and whether
/// that is an upgrade, in points.
pub fn compare(game: &Game, horse: &Horse, gear: &Gear) -> String {
    let head = match &horse.gear[gear.slot.index()] {
        Some(worn) => format!("On {}, in place of {}:", horse.name, worn.name),
        None => format!("On {}, who wears nothing there:", horse.name),
    };
    head + &changes(game, horse, gear)
}

/// The lines of a comparison under its heading: a line per stat that
/// changes, then the points.
fn changes(game: &Game, horse: &Horse, gear: &Gear) -> String {
    let worn = horse.gear[gear.slot.index()].as_ref();
    let mut out = String::new();
    let now = horse.stats();
    let off = worn.map_or(Stats::default(), |worn| worn.stats);
    for (label, now, off, on) in [
        ("health", now.health, off.health, gear.stats.health),
        ("attack", now.attack, off.attack, gear.stats.attack),
        ("defense", now.defense, off.defense, gear.stats.defense),
        ("elemental", now.elemental, off.elemental, gear.stats.elemental),
    ] {
        let change = on - off;
        if change != 0 {
            out += &format!("\n{label} {now} to {} ({change:+})", now + change);
        }
    }
    let change = gain(&game.data, horse, gear);
    out += &match change {
        1.. => format!("\nAn upgrade: +{change} points."),
        0 => "\nNo better: the same points.".to_string(),
        _ => format!("\nWorse: {change} points."),
    };
    out
}

/// What camp says of the drop by itself, with no card opened (#11): the
/// horse it goes on when the party sets out, what it changes there and what
/// it replaces, or why no horse takes it. Empty when nothing is carried.
pub fn appraisal(game: &Game) -> String {
    let Some(gear) = game.pending_drop() else {
        return String::new();
    };
    match game.drop_best_wearer() {
        Some(best) => {
            let horse = &game.party[best];
            let mut out = format!(
                "NEW GEAR: {}, on setting out:\n{}",
                gear.name,
                compare(game, horse, gear),
            );
            if let Some(worn) = &horse.gear[gear.slot.index()] {
                out += &format!(
                    "\n{} is thrown away: the cargo cannot hold it yet.",
                    worn.name
                );
            }
            out + "\nE puts it on the selected horse instead."
        }
        None => {
            let mut out = format!(
                "NEW GEAR: {}, left behind on setting out.\n\
                 No horse is better off with it:",
                gear.name
            );
            for horse in &game.party {
                out += &format!("\n{}", verdict(game, horse, gear));
            }
            out + "\nE puts it on the selected horse all the same."
        }
    }
}

/// What is said once a carried piece has been put on a horse, written
/// before it is: what it replaced and what it changed.
pub fn equipped(game: &Game, horse: &Horse, gear: &Gear) -> String {
    let worn = horse.gear[gear.slot.index()].as_ref();
    let mut out = match worn {
        Some(worn) => format!(
            "Equipped {} on {}, in place of {}:",
            gear.name, horse.name, worn.name
        ),
        None => format!(
            "Equipped {} on {}, who wore nothing there:",
            gear.name, horse.name
        ),
    };
    out += &changes(game, horse, gear);
    if let Some(worn) = worn {
        out += &format!(
            "\n{} was thrown away: the cargo cannot hold it yet.",
            worn.name
        );
    }
    out
}

/// What a horse wears in a slot, by name.
fn worn_name(horse: &Horse, slot: usize) -> &str {
    horse.gear[slot]
        .as_ref()
        .map_or("nothing", |gear| gear.name.as_str())
}

/// What Equip would do with the piece in the hand, and what becomes of the
/// piece it displaces. Empty when Equip cannot be done; the reason is under
/// the card's controls.
pub fn consequence(game: &Game, hand: Hand) -> String {
    if game.can(Action::Equip, hand).is_err() {
        return String::new();
    }
    let Some(selected) = game.party.get(hand.horse) else {
        return String::new();
    };
    match hand.open {
        Some(Piece::Worn { horse, slot }) => {
            let wearer = &game.party[horse];
            format!(
                "Equip puts {} on {}, and {} on {}.",
                worn_name(wearer, slot),
                selected.name,
                worn_name(selected, slot),
                wearer.name,
            )
        }
        Some(Piece::Carried) | None => {
            let Some(gear) = game.pending_drop() else {
                return String::new();
            };
            // The cargo holds one piece until it is built (#227), so there
            // is nowhere for the displaced one to go. Say so before it goes.
            match &selected.gear[gear.slot.index()] {
                Some(worn) => format!(
                    "Equip puts it on {}. {} is thrown away: the cargo cannot hold it yet.",
                    selected.name, worn.name,
                ),
                None => format!("Equip puts it on {}.", selected.name),
            }
        }
    }
}

/// The card pinned in the side panel: what the piece is and where, how it
/// compares on the selected horse, and what Equip would do.
pub fn pinned(game: &Game, piece: Piece, hand: Hand) -> String {
    let selected = &game.party[hand.horse.min(game.party.len() - 1)];
    let wearer = match piece {
        Piece::Worn { horse, .. } => game.party.get(horse),
        Piece::Carried => None,
    };
    let Some(gear) = game.piece(piece) else {
        return match (piece, wearer) {
            (Piece::Worn { slot, .. }, Some(wearer)) => {
                let mut out = format!(
                    "{} slot of {}\nNothing worn.\nGear comes from the trail.",
                    Slot::ALL[slot].name(),
                    wearer.name,
                );
                let does = consequence(game, hand);
                if !does.is_empty() {
                    out += &format!("\n\n{does}");
                }
                out
            }
            _ => NOTHING_CARRIED.into(),
        };
    };
    let mut out = face(game, gear, wearer, selected);
    let does = consequence(game, hand);
    if !does.is_empty() {
        out += &format!("\n{does}");
    }
    out
}

/// What every card says of a piece: what it is, where it is, and how it
/// compares on the selected horse.
fn face(game: &Game, gear: &Gear, wearer: Option<&Horse>, selected: &Horse) -> String {
    let mut out = format!(
        "{}\n{}\n{}",
        gear.name,
        kind(game, gear),
        match wearer {
            Some(wearer) => format!("Worn by {}.", wearer.name),
            None => "Carried in the cargo.".into(),
        },
    );
    // Against the selected horse, the one Equip puts it on. A horse is not
    // compared with what it wears already.
    if !wearer.is_some_and(|wearer| std::ptr::eq(wearer, selected)) {
        out += &format!("\n\n{}", compare(game, selected, gear));
    }
    out
}

/// Under the card the pointer shows: how to get the rest of it.
pub const PRESS_PINS: &str = "A press pins it, with its actions.";

/// The card under the pointer (#228): the pinned card's own lines of what
/// the piece is, where, and how it compares on the selected horse. What an
/// action would do is on the pinned card, beside the action, and never only
/// here. Empty where there is no piece to show.
pub fn hovered(game: &Game, piece: Piece, horse: usize) -> String {
    let Some(gear) = game.piece(piece) else {
        return String::new();
    };
    let selected = &game.party[horse.min(game.party.len() - 1)];
    let wearer = match piece {
        Piece::Worn { horse, .. } => game.party.get(horse),
        Piece::Carried => None,
    };
    format!("{}\n\n{PRESS_PINS}", face(game, gear, wearer, selected))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> Game {
        (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home")
    }

    /// The card is the same wherever the piece is (#228): carried or worn,
    /// it opens with the same name and the same line of what it is.
    #[test]
    fn a_piece_has_one_card_wherever_it_is() {
        let mut game = home();
        let gear = game.pending_drop().unwrap().clone();
        let head = format!("{}\n{}\n", gear.name, kind(&game, &gear));
        let carried = Hand {
            horse: 0,
            open: Some(Piece::Carried),
        };
        assert!(pinned(&game, Piece::Carried, carried).starts_with(&head));

        let at = game.equipped_at(carried).unwrap();
        game.act(Action::Equip, carried);
        for selected in 0..game.party.len() {
            let hand = Hand {
                horse: selected,
                open: Some(at),
            };
            let card = pinned(&game, at, hand);
            assert!(card.starts_with(&head), "{card}");
            assert!(card.contains(&format!("Worn by {}.", game.party[0].name)));
            // Compared on the selected horse, unless it is the one wearing it.
            assert_eq!(card.contains("\nOn "), selected != 0, "{card}");
        }
    }

    /// The card under the pointer (#228) is the pinned card as far as it
    /// goes: the same lines, for whichever horse is selected, and then how
    /// to pin it. What Equip would do is said only where Equip is.
    #[test]
    fn the_hovered_card_is_the_start_of_the_pinned_one() {
        let mut game = home();
        let carried = Hand {
            horse: 1,
            open: Some(Piece::Carried),
        };
        let at = game.equipped_at(carried).unwrap();
        for piece in [Piece::Carried, at] {
            for selected in 0..game.party.len() {
                let hand = Hand {
                    horse: selected,
                    open: Some(piece),
                };
                let said = hovered(&game, piece, selected);
                let lines = said.strip_suffix(&format!("\n\n{PRESS_PINS}")).expect(&said);
                assert!(pinned(&game, piece, hand).starts_with(lines), "{said}");
                assert_eq!(lines.contains("\nOn "), piece != Piece::Worn { horse: selected, slot: gear_slot(&game, piece) }, "{said}");
                assert!(!said.contains("Equip"), "{said}");
            }
            game.act(Action::Equip, carried);
        }
        // An empty slot and an empty cargo have nothing to show.
        let empty = (0..Slot::ALL.len())
            .find(|slot| game.party[0].gear[*slot].is_none())
            .unwrap();
        assert!(hovered(&game, Piece::Worn { horse: 0, slot: empty }, 0).is_empty());
        assert!(hovered(&game, Piece::Carried, 0).is_empty());
    }

    fn gear_slot(game: &Game, piece: Piece) -> usize {
        game.piece(piece).unwrap().slot.index()
    }

    /// The comparison (#11) is per stat and in the horse's own numbers: as
    /// it is and as it would be, with the sign of the difference, and only
    /// the stats that change.
    #[test]
    fn the_comparison_shows_each_stat_that_changes() {
        let mut game = home();
        let slot = game.pending_drop().unwrap().slot;
        game.party[0].gear[slot.index()] = Some(Gear {
            name: "Old Thing".into(),
            slot,
            rarity: 0,
            stats: Stats {
                attack: 2,
                defense: 1,
                ..Stats::default()
            },
        });
        let gear = Gear {
            name: "New Thing".into(),
            slot,
            rarity: 0,
            stats: Stats {
                attack: 5,
                ..Stats::default()
            },
        };
        let horse = &game.party[0];
        let now = horse.stats();
        let said = compare(&game, horse, &gear);
        assert!(said.starts_with(&format!("On {}, in place of Old Thing:", horse.name)));
        assert!(said.contains(&format!("attack {} to {} (+3)", now.attack, now.attack + 3)));
        assert!(said.contains(&format!("defense {} to {} (-1)", now.defense, now.defense - 1)));
        assert!(!said.contains("health") && !said.contains("elemental"), "{said}");
        assert!(said.ends_with("An upgrade: +2 points."), "{said}");
        assert_eq!(verdict(&game, horse, &gear), format!("Upgrade for {}: +2", horse.name));

        let bare = &game.party[1];
        assert!(compare(&game, bare, &gear).contains("who wears nothing there"));
    }

    /// A consequence is never left unsaid: the card says what Equip does
    /// exactly when Equip can be done, and names the piece that is lost or
    /// moved by it.
    #[test]
    fn the_card_says_what_equip_would_do() {
        let mut game = home();
        let name = game.pending_drop().unwrap().name.clone();
        let carried = |horse: usize| Hand {
            horse,
            open: Some(Piece::Carried),
        };
        let first = game.party[0].name.clone();
        assert_eq!(consequence(&game, carried(0)), format!("Equip puts it on {first}."));
        let at = game.equipped_at(carried(0)).unwrap();
        game.act(Action::Equip, carried(0));
        assert_eq!(consequence(&game, carried(0)), "");

        // Worn by the selected horse: nothing to do. By another: they swap.
        let open = |horse: usize| Hand {
            horse,
            open: Some(at),
        };
        assert_eq!(consequence(&game, open(0)), "");
        let second = game.party[1].name.clone();
        assert_eq!(
            consequence(&game, open(1)),
            format!("Equip puts {name} on {second}, and nothing on {first}.")
        );

        // A second drop onto the horse that wears the first: the card says
        // the worn one is thrown away.
        let mut again = home();
        let slot = again.pending_drop().unwrap().slot.index();
        again.party[0].gear[slot] = Some(Gear {
            name: "Old Thing".into(),
            slot: Slot::ALL[slot],
            rarity: 0,
            stats: Stats::default(),
        });
        assert!(consequence(&again, carried(0)).contains("Old Thing is thrown away"));
        assert!(pinned(&again, Piece::Carried, carried(0)).contains("Old Thing is thrown away"));
    }

    /// Camp appraisal (#11): with no input the report says where the drop
    /// goes and what it changes there, names the piece it replaces, and
    /// when no horse takes it says why, horse by horse. Once it is on, what
    /// it replaced is still said, and it opens the road's log.
    #[test]
    fn camp_says_what_the_drop_replaces_or_why_it_is_not_used() {
        use crate::game::{Screen, report_text};

        let game = home();
        let gear = game.pending_drop().unwrap().clone();
        let best = game.drop_best_wearer().expect("a bare party takes any piece");
        let name = game.party[best].name.clone();
        let said = appraisal(&game);
        assert!(said.starts_with(&format!(
            "NEW GEAR: {}, on setting out:\nOn {name}, who wears nothing there:",
            gear.name
        )));
        assert!(said.contains("An upgrade: +"), "{said}");
        assert!(!said.contains("thrown away"), "{said}");
        assert!(report_text(&game).contains(&said), "the report shows it by itself");

        // Every horse wears something in that slot, one of them worse.
        let old = |name: &str, points: i32| Gear {
            name: name.into(),
            slot: gear.slot,
            rarity: 0,
            stats: Stats {
                attack: points,
                ..Stats::default()
            },
        };
        let slot = gear.slot.index();
        let mut over = home();
        for horse in &mut over.party {
            horse.gear[slot] = Some(old("Fine Thing", 99));
        }
        over.party[1].gear[slot] = Some(old("Old Thing", 0));
        let said = appraisal(&over);
        let second = over.party[1].name.clone();
        assert!(said.contains(&format!("On {second}, in place of Old Thing:")), "{said}");
        assert!(said.contains("Old Thing is thrown away"), "{said}");

        // Set out with no input: it is on that horse, and the log says so.
        over.act(Action::SetOut, Hand::horse(0));
        assert_eq!(over.party[1].gear[slot].as_ref(), Some(&gear));
        let Screen::Trail(expedition) = &over.screen else {
            panic!("the party set out");
        };
        assert_eq!(
            expedition.log[1],
            format!("Equipped {} on {second}, in place of Old Thing:", gear.name)
        );
        assert!(expedition.log.iter().any(|line| line.starts_with("Old Thing was thrown away")));

        // No horse better off: left behind, and each horse's reason.
        let mut none = home();
        for horse in &mut none.party {
            horse.gear[slot] = Some(old("Fine Thing", 99));
        }
        let said = appraisal(&none);
        assert!(said.contains("left behind on setting out"), "{said}");
        for horse in &none.party {
            assert!(said.contains(&format!("Worse for {}: -", horse.name)), "{said}");
        }
        none.act(Action::SetOut, Hand::horse(0));
        assert!(none.party.iter().all(|horse| horse.gear[slot].as_ref().unwrap().name == "Fine Thing"));

        // By hand, onto a horse of the player's choosing: the note keeps
        // what it replaced after the piece is on.
        let mut by_hand = home();
        by_hand.party[2].gear[slot] = Some(old("Old Thing", 0));
        by_hand.act(Action::Equip, Hand::horse(2));
        let report = report_text(&by_hand);
        assert!(report.contains("in place of Old Thing:"), "{report}");
        assert!(report.contains("Old Thing was thrown away"), "{report}");
        assert!(!report.contains("NEW GEAR"), "{report}");
        assert_eq!(appraisal(&by_hand), "");
    }

    /// An empty slot has a card too, and an empty cargo says what belongs
    /// in it (#233).
    #[test]
    fn an_empty_place_says_what_belongs_there() {
        let hand = Hand::horse(0);
        let slot = Piece::Worn { horse: 0, slot: 0 };
        let mut bare = Game::back_from_a_trail(6);
        bare.party[0].gear[0] = None;
        assert!(pinned(&bare, slot, hand).contains("Gear comes from the trail"));
        let mut empty = home();
        empty.act(Action::Equip, hand);
        assert_eq!(pinned(&empty, Piece::Carried, hand), NOTHING_CARRIED);
    }
}
