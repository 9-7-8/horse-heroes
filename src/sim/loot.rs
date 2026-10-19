//! Horses, their gear, and rolling a gear drop.

use super::data::{Data, FixedPiece, HorseDef, Slot, Stats};
pub use super::look::Look;
use super::rng::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Gear {
    pub name: String,
    pub slot: Slot,
    /// Its tier: the index of its rarity in `gear.ron`, lowest first.
    pub rarity: usize,
    pub stats: Stats,
    /// Its pattern and decoration, rolled with it (#361).
    pub look: Look,
}

#[derive(Clone, Debug)]
pub struct Horse {
    pub name: String,
    /// Its one ability, by its place in `abilities.ron` (#357).
    pub ability: usize,
    pub base: Stats,
    pub gear: [Option<Gear>; 3],
}

impl Horse {
    pub fn new(data: &Data, def: &HorseDef) -> Horse {
        Horse {
            name: def.name.clone(),
            ability: data.ability_index(&def.ability),
            base: def.stats,
            gear: [None, None, None],
        }
    }

    /// The starting party, as `horses.ron` has it.
    pub fn party(data: &Data) -> Vec<Horse> {
        data.horses.iter().map(|def| Horse::new(data, def)).collect()
    }

    pub fn stats(&self) -> Stats {
        self.gear
            .iter()
            .flatten()
            .fold(self.base, |total, gear| total.plus(gear.stats))
    }
}

/// One number for "is this piece better", counting health in stat points.
pub fn score(data: &Data, stats: Stats) -> i32 {
    stats.attack + stats.defense + stats.elemental + stats.speed + stats.health / data.gear.health_per_point
}

/// Two horses change what they wear in one slot. Any horse can wear any
/// piece (#250), so nothing is refused and nothing is lost: an empty slot
/// changes places like a piece does. Returns whether anything moved.
pub fn swap_worn(party: &mut [Horse], a: usize, b: usize, slot: usize) -> bool {
    let wears = |horse: usize| {
        party
            .get(horse)
            .and_then(|horse| horse.gear.get(slot))
            .map(|worn| worn.is_some())
    };
    let (Some(on_a), Some(on_b)) = (wears(a), wears(b)) else {
        return false;
    };
    if a == b || !(on_a || on_b) {
        return false;
    }
    let piece = party[a].gear[slot].take();
    party[a].gear[slot] = std::mem::replace(&mut party[b].gear[slot], piece);
    true
}

/// Which weights a piece's rarity is rolled at, from `gear.ron`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Odds {
    /// The trail's own piece: `win_weight` if the trail was won, else `weight`.
    Trail { won: bool },
    /// A piece a kill throws, the chaff: `kill_weight` (#352).
    Kill,
}

/// The trail's own piece.
pub fn roll_gear(data: &Data, rng: &mut Rng, won: bool, level: u32) -> Gear {
    roll_piece(data, rng, Odds::Trail { won }, level)
}

/// A piece given whole in the data, not rolled (#294): the same piece every
/// time, look and all.
pub fn fixed_piece(data: &Data, def: &FixedPiece) -> Gear {
    let table = &data.gear;
    let rarity = table
        .rarities
        .iter()
        .position(|r| r.name == def.rarity)
        .unwrap_or_else(|| panic!("no rarity `{}` in gear.ron", def.rarity));
    let base = table
        .bases
        .iter()
        .find(|base| base.names.contains(&def.noun))
        .unwrap_or_else(|| panic!("no slot in gear.ron has a piece named `{}`", def.noun));
    Gear {
        name: format!("{} {}", table.rarities[rarity].prefix, def.noun),
        slot: base.slot,
        rarity,
        stats: def.stats,
        look: Look::roll(&data.looks, &Rng::new(def.look_seed)),
    }
}

pub fn roll_piece(data: &Data, rng: &mut Rng, odds: Odds, level: u32) -> Gear {
    let table = &data.gear;
    let weights: Vec<u32> = table
        .rarities
        .iter()
        .map(|r| match odds {
            Odds::Trail { won: true } => r.win_weight,
            Odds::Trail { won: false } => r.weight,
            Odds::Kill => r.kill_weight,
        })
        .collect();
    let tier = rng.weighted(&weights);
    let rarity = &table.rarities[tier];
    let base = rng.pick(&table.bases);
    let noun = rng.pick(&base.names);

    let mut stats = Stats::default();
    let points = rng.range(rarity.points.0, rarity.points.1) + level as i32;
    for _ in 0..points {
        match rng.below(4) {
            0 => stats.health += table.health_per_point,
            1 => stats.attack += 1,
            2 => stats.defense += 1,
            _ => stats.elemental += 1,
        }
    }

    Gear {
        name: format!("{} {}", rarity.prefix, noun),
        slot: base.slot,
        rarity: tier,
        stats,
        look: Look::roll(&data.looks, rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROLLS: u32 = 100_000;

    /// Roll the table many times and count each rarity.
    fn rarity_counts(data: &Data, odds: Odds) -> Vec<u32> {
        let mut rng = Rng::new(11);
        let mut counts = vec![0; data.gear.rarities.len()];
        for _ in 0..ROLLS {
            counts[roll_piece(data, &mut rng, odds, 0).rarity] += 1;
        }
        counts
    }

    #[test]
    fn rarities_follow_their_weights() {
        let data = Data::load();
        for odds in [Odds::Trail { won: false }, Odds::Trail { won: true }, Odds::Kill] {
            let weight = |r: &crate::sim::data::RarityDef| match odds {
                Odds::Trail { won: true } => r.win_weight,
                Odds::Trail { won: false } => r.weight,
                Odds::Kill => r.kill_weight,
            };
            let total: u32 = data.gear.rarities.iter().map(weight).sum();
            let counts = rarity_counts(&data, odds);
            for (def, &count) in data.gear.rarities.iter().zip(&counts) {
                let expected = weight(def) as f32 / total as f32;
                let got = count as f32 / ROLLS as f32;
                assert!(
                    (got - expected).abs() < 0.01,
                    "{} {odds:?}: {got} against {expected}",
                    def.name
                );
            }
        }
    }

    #[test]
    fn a_win_drops_better_gear() {
        let data = Data::load();
        let top = data.gear.rarities.len() - 1;
        let counts = |won| rarity_counts(&data, Odds::Trail { won })[top];
        assert!(counts(true) > counts(false));
    }

    /// A kill's pieces are the chaff: no more of the top tier than the
    /// trail's own piece at its worst (#352).
    #[test]
    fn a_kills_pieces_are_no_better_than_a_lost_trails() {
        let data = Data::load();
        let top = data.gear.rarities.len() - 1;
        let kill = rarity_counts(&data, Odds::Kill)[top];
        assert!(kill <= rarity_counts(&data, Odds::Trail { won: false })[top]);
    }

    #[test]
    fn a_drop_spends_its_rarity_points_plus_the_level() {
        let data = Data::load();
        let mut rng = Rng::new(12);
        for level in [0, 3] {
            for _ in 0..10_000 {
                let gear = roll_gear(&data, &mut rng, false, level);
                let rarity = &data.gear.rarities[gear.rarity];
                let (lo, hi) = rarity.points;
                let points = score(&data, gear.stats);
                assert!((lo + level as i32..=hi + level as i32).contains(&points), "{gear:?}");
                assert_eq!(gear.stats.health % data.gear.health_per_point, 0);
                assert!(gear.name.starts_with(&rarity.prefix));
                let base = data.gear.bases.iter().find(|b| b.slot == gear.slot).unwrap();
                assert!(base.names.iter().any(|noun| gear.name.ends_with(noun.as_str())));
            }
        }
    }

    /// A slot of the gear overview holds 22 letters on one line at the
    /// smallest window (#287). A longer name needs a wider slot first.
    #[test]
    fn every_name_fits_a_slot_of_the_overview() {
        let data = Data::load();
        for rarity in &data.gear.rarities {
            for noun in data.gear.bases.iter().flat_map(|base| &base.names) {
                let name = format!("{} {}", rarity.prefix, noun);
                assert!(name.chars().count() <= 22, "{name}");
            }
        }
    }

    #[test]
    fn every_slot_gets_drops() {
        let data = Data::load();
        let mut rng = Rng::new(13);
        let mut slots = [false; 3];
        for _ in 0..1000 {
            let gear = roll_gear(&data, &mut rng, false, 0);
            slots[gear.slot.index()] = true;
        }
        assert!(slots.iter().all(|&seen| seen));
    }

    #[test]
    fn worn_gear_adds_to_a_horse() {
        let data = Data::load();
        let mut horse = Horse::new(&data, &data.horses[0]);
        assert_eq!(horse.stats(), horse.base);
        let mut rng = Rng::new(14);
        let gear = roll_gear(&data, &mut rng, true, 2);
        let expected = horse.base.plus(gear.stats);
        let slot = gear.slot.index();
        horse.gear[slot] = Some(gear);
        assert_eq!(horse.stats(), expected);
    }

    /// A swap moves a piece between two horses in its own slot, an empty
    /// slot changing places like a piece, and loses nothing. It refuses a
    /// horse with itself, two empty slots, and what is not there.
    #[test]
    fn two_horses_swap_what_they_wear_in_a_slot() {
        let data = Data::load();
        let mut party: Vec<Horse> = Horse::party(&data);
        let mut rng = Rng::new(14);
        let gear = roll_gear(&data, &mut rng, true, 2);
        let (slot, name) = (gear.slot.index(), gear.name.clone());
        let other = (slot + 1) % 3;
        party[0].gear[slot] = Some(gear);
        let worn = |party: &[Horse], horse: usize| {
            party[horse].gear[slot].as_ref().map(|gear| gear.name.clone())
        };

        assert!(swap_worn(&mut party, 0, 2, slot));
        assert_eq!(worn(&party, 0), None);
        assert_eq!(worn(&party, 2), Some(name.clone()));
        assert!(swap_worn(&mut party, 0, 2, slot), "onto a bare horse");
        assert_eq!(worn(&party, 0), Some(name.clone()));
        assert_eq!(worn(&party, 2), None);

        assert!(!swap_worn(&mut party, 0, 0, slot), "a horse with itself");
        assert!(!swap_worn(&mut party, 0, 1, other), "two empty slots");
        assert!(!swap_worn(&mut party, 0, 9, slot), "no such horse");
        assert!(!swap_worn(&mut party, 0, 1, 9), "no such slot");
        let pieces = party.iter().flat_map(|horse| &horse.gear).flatten().count();
        assert_eq!(pieces, 1);
        assert_eq!(worn(&party, 0), Some(name));
    }
}