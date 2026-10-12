//! The item card (#228): the one way a piece of gear is described, wherever
//! it is shown. The summary is what a list shows of it; the comparison is
//! what the side panel shows when it is pressed, against the selected horse
//! and with what Equip would do, the same in every place (#312). Its actions
//! are the `card` controls of `assets/data/places.ron`. How it works:
//! docs/item-card.md.

use crate::game::{Action, Game, Hand, Piece, stats_line};
use crate::sim::data::{Slot, Stats};
use crate::sim::loot::{Gear, Horse, gain};

/// What a list says where the cargo is empty.
pub const NOTHING_CARRIED: &str = "Nothing here yet.\nGear the horses bring back is carried here.";

/// What a tile of the Inventory says of a piece in the cargo, where a worn
/// piece names its horse.
pub const CARRIED: &str = "Carried";

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

/// What the side panel says where there is no piece to compare: whose
/// empty slot it is and what Equip would put there, or what belongs in an
/// empty cargo.
fn empty(game: &Game, piece: Piece, hand: Hand) -> String {
    let wearer = match piece {
        Piece::Worn { horse, .. } => game.party.get(horse),
        Piece::Carried => None,
    };
    match (piece, wearer) {
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
    }
}

/// What the comparison's room says in the Inventory while no piece is open.
pub fn compare_hint(selected: &Horse) -> String {
    format!(
        "Point at a piece to compare it with what {} wears there. A press pins it here, with Equip. Another horse's key compares it on that horse.",
        selected.name
    )
}

/// One row of the comparison's table: a label, then what the equipped
/// piece and the selected piece add, right-aligned, then the change.
fn table_row(label: &str, equipped: &str, selected: &str, change: &str) -> String {
    format!("{label:<9}{equipped:>10}{selected:>10}  {change}").trim_end().to_string()
}

/// The comparison's table (docs/philosophy/inventory.md): each stat either
/// piece adds to on one line, in one order, what each adds, and the change.
/// A stat only the selected piece has is new, and one only the equipped
/// piece has is lost. A change has its sign and a word for whether it is
/// good, and `ink` colours it by those (#313); every stat there is today is
/// better for being higher.
pub(super) fn table(equipped: Stats, selected: Stats) -> String {
    let mut out = table_row("Piece", "Equipped", "Selected", "Change");
    let amount = |value: i32| if value == 0 { "-".to_string() } else { format!("+{value}") };
    for (label, off, on) in [
        ("health", equipped.health, selected.health),
        ("attack", equipped.attack, selected.attack),
        ("defense", equipped.defense, selected.defense),
        ("elemental", equipped.elemental, selected.elemental),
    ] {
        let change = match (off, on) {
            (0, 0) => continue,
            (0, on) => format!("{on:+} new"),
            (off, 0) => format!("{:+} lost", -off),
            (off, on) if off == on => "same".to_string(),
            (off, on) if on > off => format!("{:+} better", on - off),
            (off, on) => format!("{:+} worse", on - off),
        };
        out += &format!("\n{}", table_row(label, &amount(off), &amount(on), &change));
    }
    out
}

/// The comparison a press pins on a piece, wherever the piece is (#308,
/// #312): whose it would be and in which slot, the piece equipped there and
/// the piece selected, what equipping changes for that horse, the two pieces
/// stat by stat, and what Equip would do. The horse is the selected one;
/// another horse's key compares the same piece there, and nothing else on
/// the panel moves.
pub fn comparison(game: &Game, piece: Piece, hand: Hand) -> String {
    let Some(mut out) = weighed(game, piece, hand.horse) else {
        return empty(game, piece, hand);
    };
    let does = consequence(game, hand);
    if !does.is_empty() {
        out += &format!("\n\n{does}");
    }
    out
}

/// The comparison under the pointer (#228, #308): the pinned one's own
/// lines, and how to pin it. What Equip would do is said beside
/// Equip, never only here. Empty where there is no piece.
pub fn pointed_comparison(game: &Game, piece: Piece, horse: usize) -> String {
    weighed(game, piece, horse).map_or(String::new(), |said| format!("{said}\n\n{PRESS_PINS}"))
}

/// A piece against what a horse wears in its slot, without what Equip
/// would do. None where there is no piece.
fn weighed(game: &Game, piece: Piece, horse: usize) -> Option<String> {
    let gear = game.piece(piece)?;
    let selected = &game.party[horse.min(game.party.len() - 1)];
    let rarity = |gear: &Gear| game.data.gear.rarities[gear.rarity].name.clone();
    let worn = selected.gear[gear.slot.index()].as_ref();
    let mut out = format!("For {}, {}", selected.name, gear.slot.name());
    if worn.is_some_and(|worn| std::ptr::eq(worn, gear)) {
        out += &format!(
            "\nEquipped: {} ({})\n{} wears it. Another horse's key compares it there.\n\n{}",
            gear.name,
            rarity(gear),
            selected.name,
            table_row("Piece", "Equipped", "", ""),
        );
        for (label, value) in [
            ("health", gear.stats.health),
            ("attack", gear.stats.attack),
            ("defense", gear.stats.defense),
            ("elemental", gear.stats.elemental),
        ] {
            if value != 0 {
                out += &format!("\n{}", table_row(label, &format!("+{value}"), "", ""));
            }
        }
        return Some(out);
    }
    out += &match worn {
        Some(worn) => format!("\nEquipped: {} ({})", worn.name, rarity(worn)),
        None => "\nEquipped: nothing".to_string(),
    };
    out += &format!("\nSelected: {} ({})", gear.name, rarity(gear));
    out += &match piece {
        Piece::Worn { horse, .. } => format!("\nWorn by {}.", game.party[horse].name),
        Piece::Carried => "\nCarried in the cargo.".to_string(),
    };
    out += &format!("\n\nAFTER EQUIPPING, {}:{}", selected.name, changes(game, selected, gear));
    out += &format!(
        "\n\n{}",
        table(worn.map_or(Stats::default(), |worn| worn.stats), gear.stats)
    );
    Some(out)
}

/// One row of a horse's table: a label, then what the horse has by
/// itself, what its gear adds, and the two together, right-aligned. It is
/// 25 letters wide, which the narrowest side panel holds.
fn sheet_row(label: &str, horse: &str, gear: &str, now: &str) -> String {
    format!("{label:<9}{horse:>6}{gear:>5}{now:>5}")
}

/// The head of a horse's view (#311): its name, its health now and at
/// most, and every stat with where its points come from, so each total is
/// accounted for. `health` is the horse's health as the strip says it.
pub fn horse_sheet(game: &Game, horse: usize, health: &str) -> String {
    let horse = &game.party[horse.min(game.party.len() - 1)];
    let gear = horse
        .gear
        .iter()
        .flatten()
        .fold(Stats::default(), |sum, gear| sum.plus(gear.stats));
    let now = horse.stats();
    let mut out = format!("{}\nHealth {health}\n\n{}", horse.name, sheet_row("", "horse", "gear", "now"));
    for (label, base, gear, now) in [
        ("health", horse.base.health, gear.health, now.health),
        ("attack", horse.base.attack, gear.attack, now.attack),
        ("defense", horse.base.defense, gear.defense, now.defense),
        ("elemental", horse.base.elemental, gear.elemental, now.elemental),
    ] {
        let gear = if gear == 0 { "-".to_string() } else { format!("{gear:+}") };
        out += &format!("\n{}", sheet_row(label, &base.to_string(), &gear, &now.to_string()));
    }
    out
}

/// What a horse wears, slot by slot, with what each piece adds: the gear
/// column of its table, piece by piece.
pub fn horse_wears(game: &Game, horse: usize) -> String {
    let horse = &game.party[horse.min(game.party.len() - 1)];
    let lines: Vec<String> = Slot::ALL
        .iter()
        .map(|slot| match &horse.gear[slot.index()] {
            Some(gear) => format!("{}: {}\n  {}", slot.name(), gear.name, stats_line(gear.stats)),
            None => format!("{}: nothing worn", slot.name()),
        })
        .collect();
    lines.join("\n")
}

/// Under the card the pointer shows: how to get the rest of it.
pub const PRESS_PINS: &str = "A press pins it, with its actions.";

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> Game {
        (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home")
    }

    /// The comparison's table (#308) keeps a stat on one line with what each
    /// piece adds in the same columns, and says new, lost and same in words.
    #[test]
    fn the_comparison_lines_its_stats_up() {
        let stats = |health, attack, defense, elemental| Stats { health, attack, defense, elemental };
        let table = table(stats(5, 0, 4, 2), stats(15, 2, 0, 2));
        let rows: Vec<&str> = table.lines().collect();
        assert_eq!(rows.len(), 5);
        assert!(rows[0].starts_with("Piece") && rows[0].ends_with("Change"));
        for (row, (label, change)) in rows[1..].iter().zip([
            ("health", "+10 better"),
            ("attack", "+2 new"),
            ("defense", "-4 lost"),
            ("elemental", "same"),
        ]) {
            assert!(row.starts_with(label), "{row}");
            assert_eq!(&row[31..], change, "{row}");
            // Right-aligned: each amount ends where its heading ends.
            assert_ne!(&row[18..19], " ", "{row}");
            assert_ne!(&row[28..29], " ", "{row}");
        }
        // A stat neither piece adds to has no row.
        assert_eq!(super::table(stats(0, 1, 0, 0), stats(0, 3, 0, 0)).lines().count(), 2);
    }

    /// What the Inventory opens on a piece (#308): whose it would be and
    /// where, what is equipped there and what is selected, what changes,
    /// and what Equip would do; and a horse's own piece is not compared
    /// with itself.
    #[test]
    fn the_comparison_names_the_horse_and_both_pieces() {
        let mut game = home();
        let gear = game.pending_drop().unwrap().clone();
        let slot = gear.slot.index();
        let hand = Hand { horse: 0, open: Some(Piece::Carried) };
        let horse = game.party[0].name.clone();
        let said = comparison(&game, Piece::Carried, hand);
        assert!(said.starts_with(&format!("For {horse}, {}\nEquipped: ", gear.slot.name())), "{said}");
        assert!(said.contains(&format!("\nSelected: {} (", gear.name)), "{said}");
        assert!(said.contains("\nCarried in the cargo."));
        assert!(said.contains(&format!("AFTER EQUIPPING, {horse}:")));
        assert!(said.contains(&table(
            game.party[0].gear[slot].as_ref().map_or(Stats::default(), |worn| worn.stats),
            gear.stats
        )));
        assert!(said.ends_with(&consequence(&game, hand)) && !consequence(&game, hand).is_empty());

        // On another horse it is the same piece against that horse.
        let other = comparison(&game, Piece::Carried, Hand { horse: 1, ..hand });
        assert!(other.starts_with(&format!("For {}, ", game.party[1].name)));

        game.act(Action::Equip, hand);
        let worn = Piece::Worn { horse: 0, slot };
        let own = comparison(&game, worn, Hand { horse: 0, open: Some(worn) });
        assert!(own.contains(&format!("{horse} wears it.")), "{own}");
        assert!(!own.contains("Selected:") && !own.contains("AFTER EQUIPPING"));
        let swap = comparison(&game, worn, Hand { horse: 1, open: Some(worn) });
        assert!(swap.contains(&format!("\nWorn by {horse}.")), "{swap}");

        // Under the pointer it is the same lines, without what Equip would
        // do, and with how to pin it; and nothing where there is no piece.
        let does = consequence(&game, Hand { horse: 1, open: Some(worn) });
        assert!(!does.is_empty() && swap.ends_with(&does));
        let pointed = pointed_comparison(&game, worn, 1);
        assert_eq!(pointed, format!("{}\n\n{PRESS_PINS}", swap.strip_suffix(&does).unwrap().trim_end()));
        assert!(pointed_comparison(&game, Piece::Carried, 1).is_empty());
    }

    /// A horse's view (#311) accounts for every total: what the horse has
    /// and what its gear adds come to what it has now, and each piece that
    /// adds is named under its slot.
    #[test]
    fn a_horses_view_accounts_for_every_total() {
        let mut game = home();
        let gear = game.pending_drop().unwrap().clone();
        game.act(Action::Equip, Hand::horse(0));
        let sheet = horse_sheet(&game, 0, "7/9");
        let rows: Vec<&str> = sheet.lines().collect();
        assert_eq!(rows[0], game.party[0].name);
        assert_eq!(rows[1], "Health 7/9");
        assert_eq!(rows.len(), 8, "{sheet}");
        for row in &rows[4..] {
            assert!(row.len() <= 25, "{row}");
            let cells: Vec<&str> = row.split_whitespace().collect();
            let number = |cell: &str| if cell == "-" { 0 } else { cell.parse::<i32>().unwrap() };
            assert_eq!(number(cells[1]) + number(cells[2]), number(cells[3]), "{row}");
        }
        let now = game.party[0].stats();
        assert!(rows[4].ends_with(&now.health.to_string()) && rows[5].ends_with(&now.attack.to_string()));
        let wears = horse_wears(&game, 0);
        assert_eq!(wears.matches("nothing worn").count() + wears.matches("\n  ").count(), Slot::ALL.len());
        assert!(wears.contains(&format!("{}: {}\n  {}", gear.slot.name(), gear.name, stats_line(gear.stats))));
    }

    /// The card is the same wherever the piece is (#228, #312): carried or
    /// worn, a press shows its comparison, which names it with its rarity,
    /// says where it is and whose it would be.
    #[test]
    fn a_piece_has_one_card_wherever_it_is() {
        let mut game = home();
        let gear = game.pending_drop().unwrap().clone();
        let named = format!("{} ({})", gear.name, game.data.gear.rarities[gear.rarity].name);
        let carried = Hand {
            horse: 0,
            open: Some(Piece::Carried),
        };
        let card = comparison(&game, Piece::Carried, carried);
        assert!(card.contains(&format!("\nSelected: {named}\nCarried in the cargo.")), "{card}");

        let at = game.equipped_at(carried).unwrap();
        game.act(Action::Equip, carried);
        let wearer = game.party[0].name.clone();
        for selected in 0..game.party.len() {
            let hand = Hand {
                horse: selected,
                open: Some(at),
            };
            let card = comparison(&game, at, hand);
            let head = format!("For {}, {}\n", game.party[selected].name, gear.slot.name());
            assert!(card.starts_with(&head), "{card}");
            // Compared on the selected horse, unless it is the one wearing it.
            let own = format!("\nEquipped: {named}\n{wearer} wears it.");
            let other = format!("\nSelected: {named}\nWorn by {wearer}.");
            assert_eq!(card.contains(&own), selected == 0, "{card}");
            assert_eq!(card.contains(&other), selected != 0, "{card}");
            assert_eq!(card.contains("AFTER EQUIPPING"), selected != 0, "{card}");
        }
    }

    /// The card under the pointer (#228) is the pinned one as far as it
    /// goes: the same lines, for whichever horse is selected, and then how
    /// to pin it. What Equip would do is said only where Equip is.
    #[test]
    fn the_pointed_card_is_the_start_of_the_pinned_one() {
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
                let said = pointed_comparison(&game, piece, selected);
                let lines = said.strip_suffix(&format!("\n\n{PRESS_PINS}")).expect(&said);
                assert!(comparison(&game, piece, hand).starts_with(lines), "{said}");
                let own = piece == Piece::Worn { horse: selected, slot: gear_slot(&game, piece) };
                assert_eq!(lines.contains("AFTER EQUIPPING"), !own, "{said}");
                assert!(!said.contains("Equip puts"), "{said}");
            }
            game.act(Action::Equip, carried);
        }
        // An empty slot and an empty cargo have nothing to show.
        let empty = (0..Slot::ALL.len())
            .find(|slot| game.party[0].gear[*slot].is_none())
            .unwrap();
        assert!(pointed_comparison(&game, Piece::Worn { horse: 0, slot: empty }, 0).is_empty());
        assert!(pointed_comparison(&game, Piece::Carried, 0).is_empty());
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
        assert!(comparison(&again, Piece::Carried, carried(0)).contains("Old Thing is thrown away"));
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
        assert!(comparison(&bare, slot, hand).contains("Gear comes from the trail"));
        let mut empty = home();
        empty.act(Action::Equip, hand);
        assert_eq!(comparison(&empty, Piece::Carried, hand), NOTHING_CARRIED);
    }
}
