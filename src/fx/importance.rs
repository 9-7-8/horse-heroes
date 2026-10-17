//! One importance scale for every effect (#39): three levels, each with the
//! budget an effect at it may spend, and which event sits at which level.
//! The numbers are in `assets/data/importance.ron`; the table of events is
//! docs/importance.md.

use std::collections::BTreeMap;

use bevy::prelude::*;
use serde::Deserialize;

use crate::sim::data::RarityDef;

/// How important something that happens is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Level {
    Low,
    Medium,
    High,
}

/// What an effect at one level may spend.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct Budget {
    /// How far the view is thrown, in world units.
    pub shake: f32,
    /// How solid the flash is at its start: 0 clear, 1 opaque.
    pub flash: f32,
    pub shake_secs: f32,
    pub flash_secs: f32,
    /// How loud a sound is played against its recipe, 0 to 1.
    pub sound: f32,
}

/// `importance.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Importance {
    low: Budget,
    medium: Budget,
    high: Budget,
    /// By rarity name in `gear.ron`.
    pub rarities: BTreeMap<String, Level>,
    /// A burst with no gear in it.
    pub coins: Level,
    /// By fountain name in `loot_shower.ron`.
    pub fountains: BTreeMap<String, Level>,
    /// By death name in `deaths.ron`.
    pub deaths: BTreeMap<String, Level>,
    pub landing: Level,
    pub pickup: Level,
    pub hit: Level,
    pub ability: Level,
}

impl Importance {
    pub fn load() -> Importance {
        ron::from_str(include_str!("../../assets/data/importance.ron"))
            .unwrap_or_else(|e| panic!("assets/data/importance.ron: {e}"))
    }

    pub fn budget(&self, level: Level) -> &Budget {
        match level {
            Level::Low => &self.low,
            Level::Medium => &self.medium,
            Level::High => &self.high,
        }
    }

    #[cfg(test)]
    pub fn set(&mut self, level: Level, budget: Budget) {
        match level {
            Level::Low => self.low = budget,
            Level::Medium => self.medium = budget,
            Level::High => self.high = budget,
        }
    }

    fn named(map: &BTreeMap<String, Level>, what: &str, name: &str) -> Level {
        *map.get(name)
            .unwrap_or_else(|| panic!("importance.ron: no level for {what} `{name}`"))
    }

    pub fn rarity(&self, rarity: &RarityDef) -> Level {
        Self::named(&self.rarities, "rarity", &rarity.name)
    }

    pub fn fountain(&self, name: &str) -> Level {
        Self::named(&self.fountains, "fountain", name)
    }

    pub fn death(&self, name: &str) -> Level {
        Self::named(&self.deaths, "death", name)
    }
}

pub struct ImportancePlugin;

impl Plugin for ImportancePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Importance::load());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::death::Deaths;
    use crate::fx::drops::Shower;
    use crate::sim::data::Data;

    const ALL: [Level; 3] = [Level::Low, Level::Medium, Level::High];

    /// Every level's numbers can be used as they are, and a higher level
    /// never spends less of anything than a lower one, so a routine event
    /// can never outshine a rare one.
    #[test]
    fn the_levels_climb() {
        let importance = Importance::load();
        for level in ALL {
            let budget = importance.budget(level);
            assert!(budget.shake >= 0.0 && budget.shake <= 0.5, "{level:?}");
            assert!((0.0..=0.5).contains(&budget.flash), "{level:?}");
            assert!(budget.shake_secs > 0.0 && budget.shake_secs <= 1.0, "{level:?}");
            assert!(budget.flash_secs > 0.0 && budget.flash_secs <= 1.0, "{level:?}");
            assert!(budget.sound > 0.0 && budget.sound <= 1.0, "{level:?}");
        }
        for pair in ALL.windows(2) {
            let (lower, higher) = (importance.budget(pair[0]), importance.budget(pair[1]));
            assert!(lower.shake <= higher.shake, "{:?}", pair[1]);
            assert!(lower.flash <= higher.flash, "{:?}", pair[1]);
            assert!(lower.shake_secs <= higher.shake_secs, "{:?}", pair[1]);
            assert!(lower.flash_secs <= higher.flash_secs, "{:?}", pair[1]);
            assert!(lower.sound < higher.sound, "{:?}", pair[1]);
        }
        // High is felt: it shakes and flashes, where Low does neither.
        let (low, high) = (importance.budget(Level::Low), importance.budget(Level::High));
        assert!(high.shake > low.shake && high.flash > low.flash);
    }

    /// Every rarity, fountain and death has a level, and nothing has one
    /// that is not in the game; a better rarity is never at a lower level,
    /// and a trail's last enemy is never below an ordinary one.
    #[test]
    fn everything_has_its_level() {
        let (importance, data) = (Importance::load(), Data::load());
        let (shower, deaths) = (Shower::load(), Deaths::load());
        let rarities = &data.gear.rarities;
        assert_eq!(importance.rarities.len(), rarities.len());
        for pair in rarities.windows(2) {
            assert!(importance.rarity(&pair[0]) <= importance.rarity(&pair[1]), "{}", pair[1].name);
        }
        assert!(importance.coins <= importance.rarity(&rarities[0]));
        assert_eq!(importance.fountains.len(), shower.fountains.len());
        assert_eq!(importance.deaths.len(), deaths.deaths.len());
        let of = |ids: &Vec<Vec<String>>| -> Vec<(Level, Level)> {
            ids.iter()
                .flatten()
                .map(|id| {
                    let enemy = &data.enemies[data.enemy_index(id)];
                    (importance.fountain(&enemy.fountain), importance.death(&enemy.death))
                })
                .collect()
        };
        for small in of(&data.encounters.ordinary) {
            for big in of(&data.encounters.finals) {
                assert!(small.0 <= big.0 && small.1 <= big.1);
            }
        }
        // The routine is never above Medium: High is kept for what is rare.
        for routine in [importance.landing, importance.pickup, importance.hit, importance.ability] {
            assert!(routine < Level::High);
        }
    }
}
