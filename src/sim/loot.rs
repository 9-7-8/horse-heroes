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
