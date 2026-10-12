//! Coloured text (#251): which parts of what the menus say are about an
//! element or a stat, or are the name of a piece of gear, so that each is
//! shown in its colour wherever it is said. The colours are `colour.element`
//! and `colour.stat` in `assets/data/menu.ron`, and a piece's name is in the
//! colour of its rarity in `assets/data/gear.ron` (#309). A coloured run
//! always holds its own word, so the text reads the same with the colour
//! taken away. How it works: docs/item-card.md, "Colour".

use crate::sim::data::{Element, GearTable};

/// What a piece of text is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ink {
    Element(Element),
    Stat(Stat),
    /// The name of a piece of gear, by its tier's place in
    /// `GearTable::rarities`.
    Rarity(usize),
    /// A change for the better (#313).
    Gain,
    /// A change for the worse.
    Loss,
}

/// Every name a piece of gear can have, each with its tier.
pub struct Names(Vec<(String, usize)>);

impl Names {
    pub fn of(gear: &GearTable) -> Self {
        let mut names = Vec::new();
        for (tier, rarity) in gear.rarities.iter().enumerate() {
            for base in &gear.bases {
                names.extend(base.names.iter().map(|noun| (format!("{} {noun}", rarity.prefix), tier)));
            }
        }
        Names(names)
    }

    /// The first name of a piece in `words`, as a whole name: where it
    /// starts, where it ends and its tier.
    fn first_in(&self, words: &str) -> Option<(usize, usize, usize)> {
        let whole = |from: usize, to: usize| {
            let before = words[..from].chars().next_back();
            let after = words[to..].chars().next();
            !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric)
        };
        self.0
            .iter()
            .filter_map(|(name, tier)| {
                let to = |from: usize| from + name.len();
                let (from, _) = words.match_indices(name.as_str()).find(|(from, _)| whole(*from, to(*from)))?;
                Some((from, to(from), *tier))
            })
            // The earliest; of two that start together the longer, so that
            // a "Starlit Saddle Pad" is not taken for a "Starlit Saddle".
            .min_by_key(|(from, to, _)| (*from, std::cmp::Reverse(*to)))
    }
}

/// A stat of a horse or of a piece of gear, as the menus name it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stat {
    Health,
    Attack,
    Defense,
    Elemental,
}

impl Stat {
    pub const ALL: [Stat; 4] = [Stat::Health, Stat::Attack, Stat::Defense, Stat::Elemental];

    pub fn name(self) -> &'static str {
        match self {
            Stat::Health => "health",
            Stat::Attack => "attack",
            Stat::Defense => "defense",
            Stat::Elemental => "elemental",
        }
    }
}

/// What a word names, if it names an element or a stat. An element is
/// written with a capital and a stat without, as the game writes them.
fn ink(word: &str) -> Option<Ink> {
    let element = Element::ALL.into_iter().find(|element| element.name() == word);
    let stat = Stat::ALL.into_iter().find(|stat| stat.name() == word);
    element.map(Ink::Element).or(stat.map(Ink::Stat))
}

/// How many bytes of digits `text` ends with.
fn digits_before(text: &[u8]) -> usize {
    text.iter().rev().take_while(|byte| byte.is_ascii_digit()).count()
}

/// How many bytes of digits `text` starts with, a minus sign before them
/// counted.
fn number_after(text: &[u8]) -> usize {
    let sign = usize::from(text.first() == Some(&b'-'));
    let digits = text[sign..].iter().take_while(|byte| byte.is_ascii_digit()).count();
    if digits == 0 { 0 } else { sign + digits }
}

/// Where a stat's run starts when its word starts at `at`: at the signed
/// number before it, as in "+15 health", or at the word.
fn stat_start(text: &[u8], at: usize) -> usize {
    if at == 0 || text[at - 1] != b' ' {
        return at;
    }
    let digits = digits_before(&text[..at - 1]);
    let sign = (at - 1).wrapping_sub(digits + 1);
    let signed = digits > 0 && matches!(text.get(sign), Some(b'+' | b'-'));
    // The sign opens the number: "+15", not the end of "3-15".
    let opens = sign == 0 || !text.get(sign - 1).is_some_and(u8::is_ascii_alphanumeric);
    if signed && opens { sign } else { at }
}

/// Where the number ends that follows `before` at `at`, if one does.
fn amount(text: &[u8], at: usize, before: &[u8]) -> Option<usize> {
    let rest = text.get(at..)?.strip_prefix(before)?;
    match number_after(rest) {
        0 => None,
        digits => Some(at + before.len() + digits),
    }
}

/// Where a stat's run ends when its word ends at `at`: after the amount
/// that follows it, as in "health 35", or at the word.
fn stat_end(text: &[u8], at: usize) -> usize {
    amount(text, at, b" ").unwrap_or(at)
}

/// The change of a horse's stat whose word ends at `at`, as in "health 35
/// to 50 (+15)": where its "(+15)" starts and ends, and whether it is a
/// gain. Every stat there is today is better for being higher.
fn horse_change(text: &[u8], at: usize) -> Option<(usize, usize, Ink)> {
    let now = amount(text, at, b" ")?;
    let then = amount(text, now, b" to ")?;
    let gain = amount(text, then, b" (+").map(|by| (by, Ink::Gain));
    let loss = amount(text, then, b" (-").map(|by| (by, Ink::Loss));
    let (by, ink) = gain.or(loss)?;
    (text.get(by) == Some(&b')')).then_some((then + 1, by + 1, ink))
}

/// The change column of a row of the comparison's table, whose stat's word
/// is the start of a line and ends at `end`: after what the two pieces add,
/// a signed amount and the word the card gives it ("+10 better", "+2 new",
/// "-4 lost"). A change that is neither ("same") has none.
fn piece_change(words: &str, start: usize, end: usize) -> Option<(usize, usize, Ink)> {
    if start > 0 && words.as_bytes()[start - 1] != b'\n' {
        return None;
    }
    let row = words[end..].lines().next()?;
    let mut rest = row;
    for _ in 0..2 {
        rest = rest.trim_start_matches(' ');
        let adds = rest.split(' ').next().filter(|adds| !adds.is_empty())?;
        rest = &rest[adds.len()..];
    }
    let said = rest.trim_matches(' ');
    let ends = |with: [&str; 2]| with.iter().any(|word| said.ends_with(word));
    let ink = if said.starts_with('+') && ends([" better", " new"]) {
        Ink::Gain
    } else if said.starts_with('-') && ends([" worse", " lost"]) {
        Ink::Loss
    } else {
        return None;
    };
    let at = end + row.len() - rest.trim_start_matches(' ').len();
    Some((at, at + said.len(), ink))
}

/// `words` cut into runs in order, each with what it is about. The runs put
/// end to end are `words` exactly. A piece's name is one run, whatever words
/// are in it. No line is coloured as a verdict on a piece: the card gives
/// none (#351).
pub fn runs<'a>(words: &'a str, names: &Names) -> Vec<(&'a str, Option<Ink>)> {
    let mut out = Vec::new();
    let mut rest = words;
    while let Some((from, to, tier)) = names.first_in(rest) {
        out.extend(plain_runs(&rest[..from]));
        out.push((&rest[from..to], Some(Ink::Rarity(tier))));
        rest = &rest[to..];
    }
    out.extend(plain_runs(rest));
    out
}

/// The runs of `words` that have no piece's name in them.
fn plain_runs(words: &str) -> Vec<(&str, Option<Ink>)> {
    let text = words.as_bytes();
    let mut out = Vec::new();
    // Up to here is in `out`.
    let mut done = 0;
    let mut starts = words.char_indices().peekable();
    while let Some((start, first)) = starts.next() {
        if !first.is_alphanumeric() {
            continue;
        }
        let mut end = start + first.len_utf8();
        while let Some((at, next)) = starts.next_if(|(_, next)| next.is_alphanumeric()) {
            end = at + next.len_utf8();
        }
        let Some(ink) = ink(&words[start..end]) else {
            continue;
        };
        let (from, to) = match ink {
            Ink::Stat(_) => (stat_start(text, start).max(done), stat_end(text, end)),
            _ => (start, end),
        };
        // A stat that is compared keeps its colour on its word alone: the
        // amounts are plain, and the change after them is a gain or a loss
        // (#313).
        let change = match ink {
            Ink::Stat(_) => horse_change(text, end).or_else(|| piece_change(words, start, end)),
            _ => None,
        };
        let to = if change.is_some() { end } else { to };
        if from > done {
            out.push((&words[done..from], None));
        }
        out.push((&words[from..to], Some(ink)));
        done = to;
        if let Some((at, until, judged)) = change {
            out.push((&words[done..at], None));
            out.push((&words[at..until], Some(judged)));
            done = until;
        }
        // The amount after a stat is part of its run, not the next word.
        while starts.next_if(|(at, _)| *at < done).is_some() {}
    }
    if done < words.len() {
        out.push((&words[done..], None));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::menu::card;
    use crate::game::menu::tokens::Tokens;
    use crate::game::{Game, report_side, report_text, stats_line};
    use crate::sim::data::{Data, Stats};
    use bevy::prelude::*;

    fn inked(words: &str) -> Vec<(&str, Ink)> {
        let names = Names::of(&Data::load().gear);
        runs(words, &names)
            .into_iter()
            .filter_map(|(run, ink)| Some((run, ink?)))
            .collect()
    }

    /// Whatever is coloured, the words are the same words: every coloured
    /// line is still readable with the colour taken away (#251).
    #[test]
    fn the_runs_are_the_words() {
        let game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        for words in [
            report_text(&game),
            report_side(&game),
            card::appraisal(&game),
            "attack".into(),
            "+2 attack".into(),
            "health 3".into(),
            "Fée +1 health, à Ice".into(),
            String::new(),
        ] {
            let names = Names::of(&game.data.gear);
            let joined: String = runs(&words, &names).iter().map(|(run, _)| *run).collect();
            assert_eq!(joined, words);
            for (run, ink) in inked(&words) {
                let named = match ink {
                    Ink::Element(element) => element.name(),
                    Ink::Stat(stat) => stat.name(),
                    Ink::Rarity(tier) => &game.data.gear.rarities[tier].prefix,
                    Ink::Gain => "+",
                    Ink::Loss => "-",
                };
                assert!(run.contains(named), "`{run}` does not say {named}");
            }
        }
    }

    /// What a piece adds, as every list says it, is a run per stat, and each
    /// run takes its number with it.
    #[test]
    fn each_stat_a_piece_adds_is_its_own_run() {
        let line = stats_line(Stats {
            health: 15,
            attack: 2,
            defense: 4,
            elemental: 2,
        });
        assert_eq!(
            inked(&format!("Legendary, Head  {line}")),
            [
                ("+15 health", Ink::Stat(Stat::Health)),
                ("+2 attack", Ink::Stat(Stat::Attack)),
                ("+4 defense", Ink::Stat(Stat::Defense)),
                ("+2 elemental", Ink::Stat(Stat::Elemental)),
            ]
        );
    }

    /// A horse's stats are a run each with the amount. In a line of a
    /// comparison the stat is its word alone, the two amounts are plain, and
    /// the change is a gain or a loss (#313).
    #[test]
    fn a_stat_takes_its_amount_and_a_change_is_judged() {
        assert_eq!(
            inked("On Sable:\nhealth 35 to 50 (+15)\ndefense 5 to 2 (-3)"),
            [
                ("health", Ink::Stat(Stat::Health)),
                ("(+15)", Ink::Gain),
                ("defense", Ink::Stat(Stat::Defense)),
                ("(-3)", Ink::Loss),
            ]
        );
        assert_eq!(
            inked("Sable: health 35, attack 3"),
            [
                ("health 35", Ink::Stat(Stat::Health)),
                ("attack 3", Ink::Stat(Stat::Attack)),
            ]
        );
        // A number that is not the stat's is left alone.
        assert_eq!(inked("3-15 attack"), [("attack", Ink::Stat(Stat::Attack))]);
        assert_eq!(inked("health 35 to town"), [("health 35", Ink::Stat(Stat::Health))]);
    }

    /// The comparison's table (#313): the stat's word in its colour, what
    /// the two pieces add plain, and the change a gain or a loss by its sign
    /// and its word. A change that is neither stays plain, and so does a
    /// horse's own table, which has no change.
    #[test]
    fn the_change_column_is_a_gain_or_a_loss() {
        let stats = |health, attack, defense, elemental| Stats { health, attack, defense, elemental };
        let table = card::table(stats(5, 0, 4, 2), stats(15, 2, 0, 2), card::Rows::All);
        assert_eq!(
            inked(&table),
            [
                ("health", Ink::Stat(Stat::Health)),
                ("+10 better", Ink::Gain),
                ("attack", Ink::Stat(Stat::Attack)),
                ("+2 new", Ink::Gain),
                ("defense", Ink::Stat(Stat::Defense)),
                ("-4 lost", Ink::Loss),
                ("elemental", Ink::Stat(Stat::Elemental)),
            ]
        );
        let table = card::table(stats(15, 0, 0, 0), stats(5, 0, 0, 0), card::Rows::All);
        assert_eq!(inked(&table), [("health", Ink::Stat(Stat::Health)), ("-10 worse", Ink::Loss)]);

        let mut game = Game::back_from_a_trail(6);
        game.act(crate::game::Action::Equip, crate::game::Hand::horse(0));
        let judged = |ink: &(&str, Ink)| matches!(ink.1, Ink::Gain | Ink::Loss);
        assert!(!inked(&card::horse_sheet(&game, 0, "7/9")).iter().any(judged));
    }

    /// No line is a verdict on a piece (#351): what the card used to say of
    /// one is plain text now, were it ever said again.
    #[test]
    fn no_line_is_coloured_as_a_verdict() {
        for said in ["Upgrade for Sable: +11", "An upgrade: +6 points.", "Worse for Sable: -3"] {
            assert_eq!(inked(said), [], "{said}");
        }
    }

    /// An element is its word wherever it is said, and only the whole word.
    #[test]
    fn an_element_is_its_word() {
        assert_eq!(
            inked("Thornling (Ice), Mire Toad (Plant)\nELEMENT: Fire"),
            [
                ("Ice", Ink::Element(Element::Ice)),
                ("Plant", Ink::Element(Element::Plant)),
                ("Fire", Ink::Element(Element::Fire)),
            ]
        );
        assert_eq!(inked("Fireside, Iced, Plants, healthy, elementals"), []);
    }

    /// No name in the game is taken for an element or a stat: a horse, an
    /// enemy and a rarity stay in the colour of their line.
    #[test]
    fn no_other_name_is_coloured() {
        let data = Data::load();
        let mut names: Vec<String> = Vec::new();
        names.extend(data.horses.iter().map(|horse| horse.name.clone()));
        names.extend(data.enemies.iter().map(|enemy| enemy.name.clone()));
        names.extend(data.gear.rarities.iter().map(|rarity| rarity.name.clone()));
        for name in names {
            assert_eq!(inked(&name), [], "{name}");
        }
    }

    /// The name of every piece the game can roll is one run in its rarity,
    /// all of it, wherever it is said (#309).
    #[test]
    fn a_piece_is_named_in_its_rarity() {
        let data = Data::load();
        for (tier, rarity) in data.gear.rarities.iter().enumerate() {
            for noun in data.gear.bases.iter().flat_map(|base| &base.names) {
                let name = format!("{} {noun}", rarity.prefix);
                assert_eq!(inked(&name), [(name.as_str(), Ink::Rarity(tier))]);
                assert_eq!(
                    inked(&format!("Juniper wears {name}: +2 attack.")),
                    [(name.as_str(), Ink::Rarity(tier)), ("+2 attack", Ink::Stat(Stat::Attack))]
                );
            }
        }
        // Part of a longer word is not a piece.
        let name = format!("{} {}", data.gear.rarities[0].prefix, data.gear.bases[0].names[0]);
        assert_eq!(inked(&format!("{name}s")), []);
    }

    /// The colours can be told from each other, from the text they sit in
    /// and from the rarity colours beside them (#251): no two are close.
    #[test]
    fn the_colours_are_apart() {
        let tokens = Tokens::load();
        let data = Data::load();
        let mut inks: Vec<(String, Color)> = Vec::new();
        for element in Element::ALL {
            inks.push((element.name().into(), tokens.colour.of(element)));
        }
        for stat in Stat::ALL {
            inks.push((stat.name().into(), tokens.colour.ink(Ink::Stat(stat), &data.gear)));
        }
        for (name, ink) in [("gain", Ink::Gain), ("loss", Ink::Loss)] {
            inks.push((name.into(), tokens.colour.ink(ink, &data.gear)));
        }
        // What an ink is seen beside: the others, the text it sits in, and
        // the rarities.
        let mut beside = inks.clone();
        beside.push(("text".into(), tokens.colour.text.0));
        beside.push(("dim text".into(), tokens.colour.text_dim.0));
        beside.push(("a reason".into(), tokens.colour.reason.0));
        for rarity in &data.gear.rarities {
            let colour = Srgba::hex(&rarity.look.colour).expect("a rarity's colour");
            beside.push((rarity.name.clone(), colour.into()));
        }
        for (i, (name, colour)) in inks.iter().enumerate() {
            for (other_name, other) in &beside[i + 1..] {
                let (a, b) = (Oklaba::from(*colour), Oklaba::from(*other));
                let apart = ((a.lightness - b.lightness).powi(2)
                    + (a.a - b.a).powi(2)
                    + (a.b - b.b).powi(2))
                .sqrt();
                assert!(apart > 0.08, "{name} and {other_name} are {apart:.3} apart");
            }
        }
    }

    /// A gain and a loss (#313) are read on every surface a comparison
    /// rests on, at 4.5 to 1, and are told from each other with the
    /// colour taken away, as well as by their sign and word.
    #[test]
    fn a_gain_and_a_loss_stand_out_from_their_surface() {
        let tokens = Tokens::load();
        let data = Data::load();
        let colours = &tokens.colour;
        let light = |colour: Color| {
            let c = colour.to_linear();
            0.2126 * c.red + 0.7152 * c.green + 0.0722 * c.blue
        };
        let [gain, loss] = [Ink::Gain, Ink::Loss].map(|ink| colours.ink(ink, &data.gear));
        for (name, ink) in [("gain", gain), ("loss", loss)] {
            for (on, surface) in [
                ("the screen", colours.screen.0),
                ("a panel", colours.panel.0),
                ("a control", colours.control.0),
                ("a selected control", colours.selected.0),
            ] {
                let contrast = (light(ink) + 0.05) / (light(surface) + 0.05);
                assert!(contrast >= 4.5, "{name} on {on}: {contrast:.2} to 1");
            }
        }
        let grey = |colour: Color| {
            let c = colour.to_srgba();
            0.299 * c.red + 0.587 * c.green + 0.114 * c.blue
        };
        assert!((grey(gain) - grey(loss)).abs() >= 0.1);
    }
}
