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
    let worn = horse.gear[gear.slot.index()].as_ref();
    let mut out = match worn {
        Some(worn) => format!("On {}, in place of {}:", horse.name, worn.name),
        None => format!("On {}, who wears nothing there:", horse.name),
    };
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
    let does = consequence(game, hand);
    if !does.is_empty() {
        out += &format!("\n{does}");
    }
    out
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
