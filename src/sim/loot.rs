//! Horses, their gear, and rolling a gear drop.

use super::data::{Class, Data, HorseDef, Rarity, Slot, Stats};
use super::rng::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Gear {
    pub name: String,
    pub slot: Slot,
    pub rarity: Rarity,
    pub stats: Stats,
    /// Index of the party horse this piece was made for.
    pub fits: usize,
}

#[derive(Clone, Debug)]
pub struct Horse {
    pub name: String,
    pub class: Class,
    pub base: Stats,
    pub gear: [Option<Gear>; 3],
}

impl Horse {
    pub fn new(def: &HorseDef) -> Horse {
        Horse {
            name: def.name.clone(),
            class: def.class,
            base: def.stats,
            gear: [None, None, None],
        }
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
    stats.attack + stats.defense + stats.elemental + stats.health / data.gear.health_per_point
}

pub fn roll_gear(data: &Data, rng: &mut Rng, won: bool, level: u32, party_size: usize) -> Gear {
    let table = &data.gear;
    let weights: Vec<u32> = table
        .rarities
        .iter()
        .map(|r| if won { r.win_weight } else { r.weight })
        .collect();
    let rarity = &table.rarities[rng.weighted(&weights)];
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
        rarity: rarity.rarity,
        stats,
        fits: rng.below(party_size as u32) as usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROLLS: u32 = 100_000;

    fn rarity_def(data: &Data, rarity: Rarity) -> &crate::sim::data::RarityDef {
        data.gear.rarities.iter().find(|r| r.rarity == rarity).unwrap()
    }

    /// Roll the table many times and count each rarity.
    fn rarity_counts(data: &Data, won: bool) -> Vec<u32> {
        let mut rng = Rng::new(11);
        let mut counts = vec![0; data.gear.rarities.len()];
        for _ in 0..ROLLS {
            let gear = roll_gear(data, &mut rng, won, 0, 3);
            let i = data.gear.rarities.iter().position(|r| r.rarity == gear.rarity);
            counts[i.unwrap()] += 1;
        }
        counts
    }

    #[test]
    fn rarities_follow_their_weights() {
        let data = Data::load();
        for won in [false, true] {
            let weight = |r: &crate::sim::data::RarityDef| if won { r.win_weight } else { r.weight };
            let total: u32 = data.gear.rarities.iter().map(weight).sum();
            let counts = rarity_counts(&data, won);
            for (def, &count) in data.gear.rarities.iter().zip(&counts) {
                let expected = weight(def) as f32 / total as f32;
                let got = count as f32 / ROLLS as f32;
                assert!(
                    (got - expected).abs() < 0.01,
                    "{:?} won={won}: {got} against {expected}",
                    def.rarity
                );
            }
        }
    }

    #[test]
    fn a_win_drops_better_gear() {
        let data = Data::load();
        let rare = data.gear.rarities.iter().position(|r| r.rarity == Rarity::Rare).unwrap();
        assert!(rarity_counts(&data, true)[rare] > rarity_counts(&data, false)[rare]);
    }

    #[test]
    fn a_drop_spends_its_rarity_points_plus_the_level() {
        let data = Data::load();
        let mut rng = Rng::new(12);
        for level in [0, 3] {
            for _ in 0..10_000 {
                let gear = roll_gear(&data, &mut rng, false, level, 3);
                let (lo, hi) = rarity_def(&data, gear.rarity).points;
                let points = score(&data, gear.stats);
                assert!((lo + level as i32..=hi + level as i32).contains(&points), "{gear:?}");
                assert_eq!(gear.stats.health % data.gear.health_per_point, 0);
                assert!(gear.fits < 3);
                assert!(gear.name.starts_with(&rarity_def(&data, gear.rarity).prefix));
                let base = data.gear.bases.iter().find(|b| b.slot == gear.slot).unwrap();
                assert!(base.names.iter().any(|noun| gear.name.ends_with(noun.as_str())));
            }
        }
    }

    #[test]
    fn every_slot_and_horse_gets_drops() {
        let data = Data::load();
        let mut rng = Rng::new(13);
        let (mut slots, mut horses) = ([false; 3], [false; 3]);
        for _ in 0..1000 {
            let gear = roll_gear(&data, &mut rng, false, 0, 3);
            slots[gear.slot.index()] = true;
            horses[gear.fits] = true;
        }
        assert!(slots.iter().chain(&horses).all(|&seen| seen));
    }

    #[test]
    fn worn_gear_adds_to_a_horse() {
        let data = Data::load();
        let mut horse = Horse::new(&data.horses[0]);
        assert_eq!(horse.stats(), horse.base);
        let mut rng = Rng::new(14);
        let gear = roll_gear(&data, &mut rng, true, 2, 3);
        let expected = horse.base.plus(gear.stats);
        let slot = gear.slot.index();
        horse.gear[slot] = Some(gear);
        assert_eq!(horse.stats(), expected);
    }
}