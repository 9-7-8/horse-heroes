//! Coloured text (#251): which parts of what the menus say are about an
//! element or a stat, so that each is shown in its colour wherever it is
//! said. The colours are `colour.element` and `colour.stat` in
//! `assets/data/menu.ron`. A coloured run always holds its own word, so the
//! text reads the same with the colour taken away. How it works:
//! docs/item-card.md, "Colour".

use crate::sim::data::Element;

/// What a piece of text is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ink {
    Element(Element),
    Stat(Stat),
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

/// Where a stat's run ends when its word ends at `at`: after the amount
/// that follows it, as in "health 35", or the change that follows that, as
/// in "health 35 to 50 (+15)", or at the word.
fn stat_end(text: &[u8], at: usize) -> usize {
    let amount = |at: usize, before: &[u8]| {
        let rest = text.get(at..)?.strip_prefix(before)?;
        match number_after(rest) {
            0 => None,
            digits => Some(at + before.len() + digits),
        }
    };
    let Some(now) = amount(at, b" ") else {
        return at;
    };
    let change = amount(now, b" to ").and_then(|then| {
        let by = [amount(then, b" (+"), amount(then, b" (")].into_iter().flatten().next()?;
        (text.get(by) == Some(&b')')).then_some(by + 1)
    });
    change.unwrap_or(now)
}

/// `words` cut into runs in order, each with what it is about. The runs put
/// end to end are `words` exactly.
pub fn runs(words: &str) -> Vec<(&str, Option<Ink>)> {
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
            Ink::Element(_) => (start, end),
            Ink::Stat(_) => (stat_start(text, start).max(done), stat_end(text, end)),
        };
        if from > done {
            out.push((&words[done..from], None));
        }
        out.push((&words[from..to], Some(ink)));
        done = to;
        // The amount after a stat is part of its run, not the next word.
        while starts.next_if(|(at, _)| *at < to).is_some() {}
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
        runs(words)
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
            let joined: String = runs(&words).iter().map(|(run, _)| *run).collect();
            assert_eq!(joined, words);
            for (run, ink) in inked(&words) {
                let named = match ink {
                    Ink::Element(element) => element.name(),
                    Ink::Stat(stat) => stat.name(),
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

    /// A line of a comparison is one run, the change with it, and a horse's
    /// stats are a run each with the amount.
    #[test]
    fn a_stat_takes_its_amount_and_its_change() {
        assert_eq!(
            inked("On Sable:\nhealth 35 to 50 (+15)\ndefense 5 to 2 (-3)\nAn upgrade: +11 points."),
            [
                ("health 35 to 50 (+15)", Ink::Stat(Stat::Health)),
                ("defense 5 to 2 (-3)", Ink::Stat(Stat::Defense)),
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
        assert_eq!(inked("Upgrade for Sable: +11"), []);
        assert_eq!(inked("3-15 attack"), [("attack", Ink::Stat(Stat::Attack))]);
        assert_eq!(inked("health 35 to town"), [("health 35", Ink::Stat(Stat::Health))]);
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
    /// enemy and a piece of gear stay in the colour of their line.
    #[test]
    fn no_name_is_coloured() {
        let data = Data::load();
        let mut names: Vec<String> = Vec::new();
        names.extend(data.horses.iter().map(|horse| horse.name.clone()));
        names.extend(data.enemies.iter().map(|enemy| enemy.name.clone()));
        for rarity in &data.gear.rarities {
            names.push(rarity.name.clone());
            for base in &data.gear.bases {
                names.extend(base.names.iter().map(|name| format!("{} {name}", rarity.prefix)));
            }
        }
        for name in names {
            assert_eq!(inked(&name), [], "{name}");
        }
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
            inks.push((stat.name().into(), tokens.colour.ink(Ink::Stat(stat))));
        }
        // What an ink is seen beside: the others, the text it sits in, and
        // the rarities.
        let mut beside = inks.clone();
        beside.push(("text".into(), tokens.colour.text.0));
        beside.push(("dim text".into(), tokens.colour.text_dim.0));
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
}
