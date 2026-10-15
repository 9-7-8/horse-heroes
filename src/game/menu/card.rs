//! The item card (#228): the one way a piece of gear is described, wherever
//! it is shown. The summary is what a list shows of it; the comparison is
//! what the side panel shows when it is pressed, against the selected horse
//! and with what Equip would do, the same in every place (#312). Its actions
//! are the `card` controls of `assets/data/places.ron`. How it works:
//! docs/item-card.md.

use crate::game::{Action, Game, Hand, Piece, stats_line};
use crate::sim::data::{RarityDef, Slot, Stats};
use crate::sim::loot::{Gear, Horse, score};

/// What a list says where the cargo is empty.
pub const NOTHING_CARRIED: &str = "Nothing here yet.\nGear the horses bring back is carried here.";

/// What a tile of the Inventory says of a piece in the cargo, where a worn
/// piece names its horse.
pub const CARRIED: &str = "Carried";

/// Where a carried piece is, as its card and its report say it.
pub const IN_CARGO: &str = "Carried in the cargo.";

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

/// The lines of a comparison under its heading: a line per stat of the horse
/// that would change, as it is and as it would be. The numbers and nothing
/// more: the card never says a piece is better (#351).
fn changes(horse: &Horse, gear: &Gear) -> String {
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
    if out.is_empty() {
        out += "\nNo stat changes.";
    }
    out
}

/// What camp says of the drop by itself, with no card opened (#11): what
/// the trail brought back, and that it is carried until the player puts it
/// on a horse. Nothing about where it should go: the game picks no horse
/// for a piece (#351). It is said while the last piece the trail dropped is
/// the newest carried, and is empty once that piece is on a horse: what is
/// carried then is not new.
pub fn appraisal(game: &Game) -> String {
    let Some(gear) = game.pending_drop() else {
        return String::new();
    };
    let Some(drops) = game.last_report().map(|kept| &kept.outcome.drops) else {
        return String::new();
    };
    if !game.in_town() || drops.last() != Some(gear) {
        return String::new();
    }
    // What it is and what it adds is on its cell, under this.
    match drops.len() {
        1 => format!("NEW GEAR: {}\nCarried until you put it on a horse.", gear.name),
        pieces => format!(
            "NEW GEAR: {pieces} pieces, the newest {}\nCarried until you put them on a horse.",
            gear.name
        ),
    }
}

/// What is said once a carried piece has been put on a horse, written
/// before it is: what it replaced and what it changed.
pub fn equipped(horse: &Horse, gear: &Gear) -> String {
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
    out += &changes(horse, gear);
    if let Some(worn) = worn {
        out += &format!("\n{} went to the cargo.", worn.name);
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
        open => {
            let Some(gear) = game.piece(open.unwrap_or(Piece::Carried(0))) else {
                return String::new();
            };
            // What it displaces takes its place in the cargo (#352).
            match &selected.gear[gear.slot.index()] {
                Some(worn) => format!(
                    "Equip puts it on {}. {} goes to the cargo.",
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
        Piece::Carried(_) => None,
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

/// Which rows of the comparison's table are shown (#349): every stat either
/// piece adds to, or only those that differ between the two.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Rows {
    #[default]
    All,
    Changed,
}

/// The comparison's table (docs/philosophy/inventory.md): each stat either
/// piece adds to on one line, in one order, what each adds, and the change.
/// A stat only the selected piece has is new, and one only the equipped
/// piece has is lost. A change has its sign and a word for whether it is
/// good, and `ink` colours it by those (#313); every stat there is today is
/// better for being higher. With `Rows::Changed` a stat that is the same on
/// both has no row, and a line under the table says how many are left out,
/// so the table itself says it is narrowed.
pub(super) fn table(equipped: Stats, selected: Stats, rows: Rows) -> String {
    let mut out = table_row("Piece", "Equipped", "Selected", "Change");
    let amount = |value: i32| if value == 0 { "-".to_string() } else { format!("+{value}") };
    let mut left_out = 0;
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
            (off, on) if off == on && rows == Rows::Changed => {
                left_out += 1;
                continue;
            }
            (off, on) if off == on => "same".to_string(),
            (off, on) if on > off => format!("{:+} better", on - off),
            (off, on) => format!("{:+} worse", on - off),
        };
        out += &format!("
{}", table_row(label, &amount(off), &amount(on), &change));
    }
    if rows == Rows::Changed {
        out += &format!("
{}", left_out_line(left_out));
    }
    out
}

/// Under a table narrowed to the rows that differ: how many it leaves out.
fn left_out_line(rows: usize) -> String {
    match rows {
        0 => "Changed only: no row is the same.".to_string(),
        1 => "Changed only: 1 same row hidden.".to_string(),
        rows => format!("Changed only: {rows} same rows hidden."),
    }
}

/// A share of a whole as a percentage that says it is one: "9%", and
/// "under 1%" rather than a 0 that is not true.
fn percent(part: u32, whole: u32) -> String {
    if part == 0 || whole == 0 {
        return "none".into();
    }
    match (part * 100 + whole / 2) / whole {
        0 => "under 1%".into(),
        share => format!("{share}%"),
    }
}

/// A multiplier as the rules hold it: "x1.5", "x2".
fn times(multiplier: f32) -> String {
    format!("x{}", (multiplier * 100.0).round() / 100.0)
}

/// The card's third layer (#228; docs/philosophy/menus.md, "The item is the
/// handle"): the formulas and assumptions behind a piece, behind the
/// Details switch. How it was rolled, from `gear.ron`, and what each of its
/// stats does in a hit, from `rules.ron`. Every number is the data's, so a
/// change there is said here with no code.
pub fn details(game: &Game, gear: &Gear) -> String {
    let data = &game.data;
    let table = &data.gear;
    let rarity = &table.rarities[gear.rarity];
    let (lo, hi) = rarity.points;
    let points = score(data, gear.stats);
    let total = |weight: fn(&RarityDef) -> u32| table.rarities.iter().map(weight).sum();
    let share = |weight: fn(&RarityDef) -> u32| percent(weight(rarity), total(weight));
    let rules = &data.rules;
    format!(
        "DETAILS: {name}\n\
         HOW IT WAS ROLLED\n\
         A {tier} piece has {lo} to {hi} points, and 1 more for each level of its trail. This one has {points}.\n\
         Each point went to one stat at random: +1 attack, +1 defense, +1 elemental or +{per} health.\n\
         Odds of {tier}: {lost} of a lost trail's own piece, {won} of a won trail's, {kill} of a kill's pieces.\n\
         \n\
         IN A FIGHT\n\
         Each horse hits every {interval} s.\n\
         attack: a hit's physical part, less the enemy's defense, never below 0.\n\
         elemental: a hit's elemental part, {strong} where the party's element is strong, {weak} where it is resisted.\n\
         defense: taken off each enemy hit, which still does at least 1.\n\
         health: what a horse can lose before it falls.\n\
         Focused Shot hits with attack {focused}; Elemental Burst hits every enemy with elemental {burst}. A Ward takes {ward}% off enemy hits.",
        name = gear.name,
        tier = rarity.name,
        per = table.health_per_point,
        lost = share(|r| r.weight),
        won = share(|r| r.win_weight),
        kill = share(|r| r.kill_weight),
        interval = rules.attack_interval,
        strong = times(rules.strong_multiplier),
        weak = times(rules.weak_multiplier),
        focused = times(rules.focused_shot_multiplier),
        burst = times(rules.burst_multiplier),
        ward = (rules.ward_reduction * 100.0).round(),
    )
}

/// The comparison a press pins on a piece, wherever the piece is (#308,
/// #312): whose it would be and in which slot, the piece equipped there and
/// the piece selected, what equipping changes for that horse, the two pieces
/// stat by stat, and what Equip would do. The horse is the selected one;
/// another horse's key compares the same piece there, and nothing else on
/// the panel moves. Without its Details: the game asks for them by the
/// switch, through `comparison_with`.
#[cfg(test)]
pub fn comparison(game: &Game, piece: Piece, hand: Hand, rows: Rows) -> String {
    comparison_with(game, piece, hand, rows, false)
}

/// The comparison, and with `details` the card's third layer under its
/// table, before what Equip would do, so that line stays beside Equip.
pub fn comparison_with(game: &Game, piece: Piece, hand: Hand, rows: Rows, details: bool) -> String {
    let Some(mut out) = weighed(game, piece, hand.horse, rows) else {
        return empty(game, piece, hand);
    };
    if details && let Some(gear) = game.piece(piece) {
        out += &format!("\n\n{}", self::details(game, gear));
    }
    let does = consequence(game, hand);
    if !does.is_empty() {
        out += &format!("\n\n{does}");
    }
    out
}

/// The comparison under the pointer (#228, #308): the pinned one's own
/// lines, and how to pin it. What Equip would do is said beside
/// Equip, never only here. Empty where there is no piece.
pub fn pointed_comparison(game: &Game, piece: Piece, horse: usize, rows: Rows) -> String {
    weighed(game, piece, horse, rows).map_or(String::new(), |said| format!("{said}\n\n{PRESS_PINS}"))
}

/// The card a dragged piece shows over a horse or slot it may go to
/// (#228), where `hand` is what Equip would be done with: the comparison on
/// that horse and what letting go would do, the preview of the drop. Empty
/// where there is no piece or it may not go.
pub fn dragged(game: &Game, piece: Piece, hand: Hand, rows: Rows) -> String {
    let does = consequence(game, hand);
    match weighed(game, piece, hand.horse, rows) {
        Some(said) if !does.is_empty() => format!("{said}\n\n{does}\n{LET_GO}"),
        _ => String::new(),
    }
}

/// The card beside a horse of the list under Another horse (#356), where
/// `hand` is what Equip would be done with: the comparison on that horse
/// and what choosing it would do. Empty where it may not go.
pub fn offered(game: &Game, piece: Piece, hand: Hand, rows: Rows) -> String {
    let does = consequence(game, hand);
    match weighed(game, piece, hand.horse, rows) {
        Some(said) if !does.is_empty() => format!("{said}\n\n{does}\n{CHOOSE}"),
        _ => String::new(),
    }
}

/// Under the card beside a horse of that list.
pub const CHOOSE: &str = "Choose it to do it; the horse is selected after.";

/// Under the card of a dragged piece, over a place it may go.
pub const LET_GO: &str = "Let go here to do it; anywhere else, and nothing changes.";

/// A piece against what a horse wears in its slot, without what Equip
/// would do. None where there is no piece.
fn weighed(game: &Game, piece: Piece, horse: usize, rows: Rows) -> Option<String> {
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
        Piece::Carried(_) => format!("\n{IN_CARGO}"),
    };
    out += &format!("\n\nAFTER EQUIPPING, {}:{}", selected.name, changes(selected, gear));
    out += &format!(
        "\n\n{}",
        table(worn.map_or(Stats::default(), |worn| worn.stats), gear.stats, rows)
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
/// accounted for. `health` is the horse's health as its tag says it.
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

    /// A dragged piece's card over a horse (#228) is the comparison on
    /// that horse, whoever is selected, and what letting go would do.
    #[test]
    fn a_dragged_piece_previews_the_drop() {
        let game = Game::back_with_one_piece();
        let piece = Piece::Carried(0);
        let hand = game.drop_hand(piece, 2, None).unwrap();
        let said = dragged(&game, piece, hand, Rows::All);
        let name = &game.party[2].name;
        assert!(said.starts_with(&format!("For {name},")), "{said}");
        let does = consequence(&game, hand);
        assert!(!does.is_empty() && said.contains(&does) && said.ends_with(LET_GO), "{said}");
        let nothing = Piece::Carried(5);
        assert_eq!(dragged(&game, nothing, Hand { horse: 2, open: Some(nothing) }, Rows::All), "");
    }

    fn home() -> Game {
        Game::back_with_one_piece()
    }

    /// The comparison's table (#308) keeps a stat on one line with what each
    /// piece adds in the same columns, and says new, lost and same in words.
    #[test]
    fn the_comparison_lines_its_stats_up() {
        let stats = |health, attack, defense, elemental| Stats { health, attack, defense, elemental };
        let table = table(stats(5, 0, 4, 2), stats(15, 2, 0, 2), Rows::All);
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
        assert_eq!(super::table(stats(0, 1, 0, 0), stats(0, 3, 0, 0), Rows::All).lines().count(), 2);
    }

    /// The Details layer (#228) says a piece's own numbers and the data's:
    /// its rarity's points and this piece's, its odds as percentages of
    /// what, and what each stat does in a hit, each number as `gear.ron` and
    /// `rules.ron` hold it. On the card it goes under the table and before
    /// what Equip would do, which stays the last line, and only when asked.
    #[test]
    fn details_says_how_a_piece_was_rolled_and_what_its_stats_do() {
        let game = home();
        let gear = game.pending_drop().unwrap().clone();
        let rarity = &game.data.gear.rarities[gear.rarity];
        let said = details(&game, &gear);
        let (lo, hi) = rarity.points;
        assert!(said.contains(&format!("A {} piece has {lo} to {hi} points", rarity.name)), "{said}");
        assert!(said.contains(&format!("This one has {}.", score(&game.data, gear.stats))), "{said}");
        assert!(said.contains(&format!("+{} health", game.data.gear.health_per_point)), "{said}");
        let rules = &game.data.rules;
        for multiplier in [rules.strong_multiplier, rules.weak_multiplier, rules.focused_shot_multiplier] {
            assert!(said.contains(&times(multiplier)), "{said}");
        }
        for stat in ["attack:", "elemental:", "defense:", "health:"] {
            assert!(said.lines().any(|line| line.starts_with(stat)), "{stat} {said}");
        }

        assert_eq!(percent(9, 100), "9%");
        assert_eq!(percent(1, 1000), "under 1%");
        assert_eq!(percent(0, 100), "none");
        assert_eq!(times(1.5), "x1.5");
        assert_eq!(times(2.0), "x2");

        let hand = Hand { horse: 0, open: Some(Piece::Carried(0)) };
        let plain = comparison(&game, Piece::Carried(0), hand, Rows::All);
        let full = comparison_with(&game, Piece::Carried(0), hand, Rows::All, true);
        assert!(!plain.contains("DETAILS"));
        assert!(full.contains(&said), "{full}");
        assert_eq!(full.lines().last(), plain.lines().last(), "what Equip does stays last");
        let empty = Piece::Worn { horse: 0, slot: gear.slot.index() };
        assert!(game.piece(empty).is_none(), "the drop's slot is bare at home");
        assert_eq!(
            comparison_with(&game, empty, Hand { open: Some(empty), ..hand }, Rows::All, true),
            comparison(&game, empty, Hand { open: Some(empty), ..hand }, Rows::All),
            "an empty slot has no details"
        );
    }

    /// Changed only (#349): a stat that is the same on both pieces has no
    /// row and every other row is as it was, in the same order; a line under
    /// the table says how many are left out. Nothing above the table moves,
    /// and the card under the pointer is narrowed with the pinned one.
    #[test]
    fn changed_only_leaves_out_the_rows_that_are_the_same() {
        let stats = |health, attack, defense, elemental| Stats { health, attack, defense, elemental };
        let (equipped, selected) = (stats(5, 0, 4, 2), stats(15, 2, 0, 2));
        let all = table(equipped, selected, Rows::All);
        let changed = table(equipped, selected, Rows::Changed);
        let kept: Vec<&str> = all.lines().filter(|row| !row.ends_with("same")).collect();
        assert_eq!(kept.len(), 4);
        let rows: Vec<&str> = changed.lines().collect();
        assert_eq!(rows[..4], kept[..]);
        assert_eq!(rows[4..], ["Changed only: 1 same row hidden."]);

        // Nothing the same: every row, and the line says so. Nothing but
        // the same: the headings, and how many are left out.
        let differ = table(stats(1, 0, 0, 0), stats(2, 3, 0, 0), Rows::Changed);
        let every = table(stats(1, 0, 0, 0), stats(2, 3, 0, 0), Rows::All);
        assert_eq!(differ, format!("{every}\nChanged only: no row is the same."));
        let alike = table(stats(1, 2, 0, 0), stats(1, 2, 0, 0), Rows::Changed);
        assert_eq!(alike.lines().count(), 2, "{alike}");
        assert!(alike.ends_with("Changed only: 2 same rows hidden."), "{alike}");

        // On the card: a piece with one stat the same as what is worn.
        let mut game = home();
        let gear = game.pending_drop().unwrap().clone();
        let mut worn = gear.clone();
        worn.name = "Old Thing".into();
        worn.stats.health += 3;
        game.party[0].gear[gear.slot.index()] = Some(worn.clone());
        let hand = Hand { horse: 0, open: Some(Piece::Carried(0)) };
        let full = comparison(&game, Piece::Carried(0), hand, Rows::All);
        let narrow = comparison(&game, Piece::Carried(0), hand, Rows::Changed);
        let at = full.find("\nPiece ").unwrap();
        assert_eq!(full[..at], narrow[..at], "the lines above the table do not change");
        assert!(full.contains(&table(worn.stats, gear.stats, Rows::All)));
        assert!(narrow.contains(&table(worn.stats, gear.stats, Rows::Changed)));
        assert!(full.lines().any(|row| row.ends_with("  same")), "{full}");
        assert!(!narrow.lines().any(|row| row.ends_with("  same")), "{narrow}");
        assert!(narrow.ends_with(&consequence(&game, hand)));
        let pointed = pointed_comparison(&game, Piece::Carried(0), 0, Rows::Changed);
        assert!(narrow.starts_with(pointed.strip_suffix(&format!("\n\n{PRESS_PINS}")).unwrap()));
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
        let hand = Hand { horse: 0, open: Some(Piece::Carried(0)) };
        let horse = game.party[0].name.clone();
        let said = comparison(&game, Piece::Carried(0), hand, Rows::All);
        assert!(said.starts_with(&format!("For {horse}, {}\nEquipped: ", gear.slot.name())), "{said}");
        assert!(said.contains(&format!("\nSelected: {} (", gear.name)), "{said}");
        assert!(said.contains("\nCarried in the cargo."));
        assert!(said.contains(&format!("AFTER EQUIPPING, {horse}:")));
        assert!(said.contains(&table(
            game.party[0].gear[slot].as_ref().map_or(Stats::default(), |worn| worn.stats),
            gear.stats,
            Rows::All
        )));
        assert!(said.ends_with(&consequence(&game, hand)) && !consequence(&game, hand).is_empty());

        // On another horse it is the same piece against that horse.
        let other = comparison(&game, Piece::Carried(0), Hand { horse: 1, ..hand }, Rows::All);
        assert!(other.starts_with(&format!("For {}, ", game.party[1].name)));

        game.act(Action::Equip, hand);
        let worn = Piece::Worn { horse: 0, slot };
        let own = comparison(&game, worn, Hand { horse: 0, open: Some(worn) }, Rows::All);
        assert!(own.contains(&format!("{horse} wears it.")), "{own}");
        assert!(!own.contains("Selected:") && !own.contains("AFTER EQUIPPING"));
        let swap = comparison(&game, worn, Hand { horse: 1, open: Some(worn) }, Rows::All);
        assert!(swap.contains(&format!("\nWorn by {horse}.")), "{swap}");

        // Under the pointer it is the same lines, without what Equip would
        // do, and with how to pin it; and nothing where there is no piece.
        let does = consequence(&game, Hand { horse: 1, open: Some(worn) });
        assert!(!does.is_empty() && swap.ends_with(&does));
        let pointed = pointed_comparison(&game, worn, 1, Rows::All);
        assert_eq!(pointed, format!("{}\n\n{PRESS_PINS}", swap.strip_suffix(&does).unwrap().trim_end()));
        assert!(pointed_comparison(&game, Piece::Carried(0), 1, Rows::All).is_empty());
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
            open: Some(Piece::Carried(0)),
        };
        let card = comparison(&game, Piece::Carried(0), carried, Rows::All);
        assert!(card.contains(&format!("\nSelected: {named}\nCarried in the cargo.")), "{card}");

        let at = game.equipped_at(carried).unwrap();
        game.act(Action::Equip, carried);
        let wearer = game.party[0].name.clone();
        for selected in 0..game.party.len() {
            let hand = Hand {
                horse: selected,
                open: Some(at),
            };
            let card = comparison(&game, at, hand, Rows::All);
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
            open: Some(Piece::Carried(0)),
        };
        let at = game.equipped_at(carried).unwrap();
        for piece in [Piece::Carried(0), at] {
            for selected in 0..game.party.len() {
                let hand = Hand {
                    horse: selected,
                    open: Some(piece),
                };
                let said = pointed_comparison(&game, piece, selected, Rows::All);
                let lines = said.strip_suffix(&format!("\n\n{PRESS_PINS}")).expect(&said);
                assert!(comparison(&game, piece, hand, Rows::All).starts_with(lines), "{said}");
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
        assert!(pointed_comparison(&game, Piece::Worn { horse: 0, slot: empty }, 0, Rows::All).is_empty());
        assert!(pointed_comparison(&game, Piece::Carried(0), 0, Rows::All).is_empty());
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
        let said = changes(horse, &gear);
        assert_eq!(
            said,
            format!(
                "\nattack {} to {} (+3)\ndefense {} to {} (-1)",
                now.attack,
                now.attack + 3,
                now.defense,
                now.defense - 1
            )
        );
        // A piece that changes nothing says so, and no more.
        let same = horse.gear[slot.index()].clone().unwrap();
        assert_eq!(changes(horse, &same), "\nNo stat changes.");
    }

    /// The card gives the numbers and never says a piece is better (#351):
    /// no line of it, under the pointer or pinned, in the report or after
    /// Equip, is a verdict on the piece, for a better piece or a worse one.
    #[test]
    fn the_card_gives_numbers_and_no_verdict() {
        use crate::game::report_text;

        let gear = home().pending_drop().unwrap().clone();
        for worn in [-5, 0, 5] {
            let mut game = home();
            let mut old = gear.clone();
            old.name = "Old Thing".into();
            old.stats.attack += worn;
            game.party[0].gear[gear.slot.index()] = Some(old);
            let hand = Hand { horse: 0, open: Some(Piece::Carried(0)) };
            let mut said = vec![
                comparison(&game, Piece::Carried(0), hand, Rows::All),
                pointed_comparison(&game, Piece::Carried(0), 0, Rows::All),
                appraisal(&game),
                report_text(&game),
            ];
            game.act(Action::Equip, hand);
            said.push(report_text(&game));
            for said in said {
                let low = said.to_lowercase();
                for word in ["upgrade", "no better", "worse for", "points"] {
                    assert!(!low.contains(word), "`{word}` in:\n{said}");
                }
            }
        }
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
            open: Some(Piece::Carried(0)),
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
        // the worn one goes to the cargo.
        let mut again = home();
        let slot = again.pending_drop().unwrap().slot.index();
        again.party[0].gear[slot] = Some(Gear {
            name: "Old Thing".into(),
            slot: Slot::ALL[slot],
            rarity: 0,
            stats: Stats::default(),
        });
        assert!(consequence(&again, carried(0)).contains("Old Thing goes to the cargo."));
        assert!(comparison(&again, Piece::Carried(0), carried(0), Rows::All).contains("Old Thing goes to the cargo."));
    }

    /// Camp appraisal (#11, #351): with no input the report says what the
    /// trail brought back and that it is carried, and names no horse for
    /// it. Setting out with nothing pressed puts nothing on. By hand, what
    /// the piece replaced is still said once it is on.
    #[test]
    fn camp_says_what_was_brought_back_and_names_no_horse() {
        use crate::game::{Screen, report_text};

        let game = home();
        let gear = game.pending_drop().unwrap().clone();
        let said = appraisal(&game);
        assert_eq!(said, format!("NEW GEAR: {}\nCarried until you put it on a horse.", gear.name));
        for horse in &game.party {
            assert!(!said.contains(&horse.name), "{said}");
        }
        assert!(report_text(&game).contains(&said), "the report shows it by itself");

        // Several pieces brought back (#352): how many, and the newest.
        let many = Game::back_from_a_trail(6);
        let pieces = many.last_report().unwrap().outcome.drops.len();
        let newest = &many.pending_drop().unwrap().name;
        assert!(pieces > 1);
        assert_eq!(
            appraisal(&many),
            format!("NEW GEAR: {pieces} pieces, the newest {newest}\nCarried until you put them on a horse.")
        );

        // One horse wears something worse there, the others better: the
        // game still names none of them, and setting out moves nothing.
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
        for horse in &over.party {
            assert!(!said.contains(&horse.name), "{said}");
        }
        over.act(Action::SetOut, Hand::horse(0));
        assert_eq!(over.party[1].gear[slot].as_ref().unwrap().name, "Old Thing");
        assert_eq!(over.pending_drop(), Some(&gear), "and it is still carried on the road");
        let Screen::Trail(expedition) = &over.screen else {
            panic!("the party set out");
        };
        assert!(!expedition.log.iter().any(|line| line.contains(&gear.name)), "{:?}", expedition.log);

        // By hand, onto a horse of the player's choosing: the note keeps
        // what it replaced after the piece is on.
        let mut by_hand = home();
        by_hand.party[2].gear[slot] = Some(old("Old Thing", 0));
        by_hand.act(Action::Equip, Hand::horse(2));
        let report = report_text(&by_hand);
        assert!(report.contains("in place of Old Thing:"), "{report}");
        assert!(report.contains("Old Thing went to the cargo."), "{report}");
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
        assert!(comparison(&bare, slot, hand, Rows::All).contains("Gear comes from the trail"));
        let mut empty = home();
        empty.act(Action::Equip, hand);
        assert_eq!(comparison(&empty, Piece::Carried(0), hand, Rows::All), NOTHING_CARRIED);
    }
}
