//! Horses, their gear, and rolling a gear drop.

use super::data::{Class, Data, HorseDef, Slot, Stats};
use super::rng::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Gear {
    pub name: String,
    pub slot: Slot,
    /// Its tier: the index of its rarity in `gear.ron`, lowest first.
    pub rarity: usize,
    pub stats: Stats,
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

/// What a horse gains by wearing a piece in place of what it wears in that
/// slot now, in `score` points. Any horse can wear any piece (#250).
pub fn gain(data: &Data, horse: &Horse, gear: &Gear) -> i32 {
    let worn = horse.gear[gear.slot.index()]
        .as_ref()
        .map_or(Stats::default(), |worn| worn.stats);
    score(data, gear.stats) - score(data, worn)
}

/// The horse a piece does most for, if it is an upgrade for any of them. The
/// earlier horse takes a tie.
pub fn best_wearer(data: &Data, party: &[Horse], gear: &Gear) -> Option<usize> {
    let mut best: Option<(usize, i32)> = None;
    for (index, horse) in party.iter().enumerate() {
        let gain = gain(data, horse, gear);
        if gain > best.map_or(0, |(_, most)| most) {
            best = Some((index, gain));
        }
    }
    best.map(|(index, _)| index)
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

pub fn roll_gear(data: &Data, rng: &mut Rng, won: bool, level: u32) -> Gear {
    let table = &data.gear;
    let weights: Vec<u32> = table
        .rarities
        .iter()
        .map(|r| if won { r.win_weight } else { r.weight })
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROLLS: u32 = 100_000;

    /// Roll the table many times and count each rarity.
    fn rarity_counts(data: &Data, won: bool) -> Vec<u32> {
        let mut rng = Rng::new(11);
        let mut counts = vec![0; data.gear.rarities.len()];
        for _ in 0..ROLLS {
            counts[roll_gear(data, &mut rng, won, 0).rarity] += 1;
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
                    "{} won={won}: {got} against {expected}",
                    def.name
                );
            }
        }
    }

    #[test]
    fn a_win_drops_better_gear() {
        let data = Data::load();
        let top = data.gear.rarities.len() - 1;
        assert!(rarity_counts(&data, true)[top] > rarity_counts(&data, false)[top]);
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

    /// Any horse can wear a piece, and by itself it goes to the horse it
    /// does most for: never to one already wearing better.
    #[test]
    fn a_piece_goes_where_it_does_most() {
        let data = Data::load();
        let mut party: Vec<Horse> = data.horses.iter().map(Horse::new).collect();
        let piece = |points: i32| Gear {
            name: "Test Halter".into(),
            slot: Slot::Head,
            rarity: 0,
            stats: Stats {
                attack: points,
                ..Stats::default()
            },
        };
        let head = Slot::Head.index();
        assert_eq!(best_wearer(&data, &party, &piece(3)), Some(0), "a tie goes to the first");
        party[0].gear[head] = Some(piece(5));
        party[1].gear[head] = Some(piece(1));
        assert_eq!(gain(&data, &party[0], &piece(3)), -2);
        assert_eq!(gain(&data, &party[1], &piece(3)), 2);
        assert_eq!(gain(&data, &party[2], &piece(3)), 3);
        assert_eq!(best_wearer(&data, &party, &piece(3)), Some(2));
        party[2].gear[head] = Some(piece(5));
        assert_eq!(best_wearer(&data, &party, &piece(3)), Some(1));
        assert_eq!(best_wearer(&data, &party, &piece(1)), None, "an upgrade for nobody");
    }

    #[test]
    fn worn_gear_adds_to_a_horse() {
        let data = Data::load();
        let mut horse = Horse::new(&data.horses[0]);
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
        let mut party: Vec<Horse> = data.horses.iter().map(Horse::new).collect();
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