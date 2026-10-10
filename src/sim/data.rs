//! The definitions in `assets/data/`, compiled into the binary.

use serde::{Deserialize, de::DeserializeOwned};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Element {
    Fire,
    Ice,
    Plant,
}

impl Element {
    pub const ALL: [Element; 3] = [Element::Fire, Element::Ice, Element::Plant];

    pub fn beats(self) -> Element {
        match self {
            Element::Fire => Element::Plant,
            Element::Plant => Element::Ice,
            Element::Ice => Element::Fire,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Element::Fire => "Fire",
            Element::Ice => "Ice",
            Element::Plant => "Plant",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Class {
    Ranger,
    Wizard,
    Mage,
}

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::Ranger => "Ranger",
            Class::Wizard => "Wizard",
            Class::Mage => "Mage",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Slot {
    Head,
    Body,
    Saddle,
}

impl Slot {
    pub const ALL: [Slot; 3] = [Slot::Head, Slot::Body, Slot::Saddle];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn name(self) -> &'static str {
        match self {
            Slot::Head => "Head",
            Slot::Body => "Body",
            Slot::Saddle => "Saddle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}

impl Rarity {
    pub fn name(self) -> &'static str {
        match self {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct Stats {
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub elemental: i32,
}

impl Stats {
    pub fn plus(self, other: Stats) -> Stats {
        Stats {
            health: self.health + other.health,
            attack: self.attack + other.attack,
            defense: self.defense + other.defense,
            elemental: self.elemental + other.elemental,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rules {
    pub strong_multiplier: f32,
    pub weak_multiplier: f32,
    pub attack_interval: f32,
    pub focused_shot_cooldown: f32,
    pub focused_shot_multiplier: f32,
    pub burst_cooldown: f32,
    pub burst_multiplier: f32,
    pub ward_cooldown: f32,
    pub ward_duration: f32,
    pub ward_reduction: f32,
    pub travel_secs: f32,
    pub ordinary_encounters: usize,
    pub bonus_chance: f32,
    pub bonus_heal: f32,
    pub level_scaling: f32,
}

impl Rules {
    /// Damage multiplier for an elemental attack against a target's element.
    pub fn multiplier(&self, attack: Element, target: Element) -> f32 {
        if attack.beats() == target {
            self.strong_multiplier
        } else if target.beats() == attack {
            self.weak_multiplier
        } else {
            1.0
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HorseDef {
    pub name: String,
    pub class: Class,
    pub stats: Stats,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EnemyDef {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub attack: i32,
    #[serde(default)]
    pub defense: i32,
    pub attack_interval: f32,
    pub coins: (i32, i32),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Encounters {
    pub ordinary: Vec<Vec<String>>,
    pub finals: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RarityDef {
    pub rarity: Rarity,
    pub prefix: String,
    pub weight: u32,
    pub win_weight: u32,
    pub points: (i32, i32),
}

#[derive(Clone, Debug, Deserialize)]
pub struct BaseDef {
    pub slot: Slot,
    pub names: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GearTable {
    pub health_per_point: i32,
    pub rarities: Vec<RarityDef>,
    pub bases: Vec<BaseDef>,
}

#[derive(Clone, Debug)]
pub struct Data {
    pub rules: Rules,
    pub horses: Vec<HorseDef>,
    pub enemies: Vec<EnemyDef>,
    pub encounters: Encounters,
    pub gear: GearTable,
}

impl Data {
    pub fn load() -> Data {
        Data {
            rules: parse("rules.ron", include_str!("../../assets/data/rules.ron")),
            horses: parse("horses.ron", include_str!("../../assets/data/horses.ron")),
            enemies: parse("enemies.ron", include_str!("../../assets/data/enemies.ron")),
            encounters: parse(
                "encounters.ron",
                include_str!("../../assets/data/encounters.ron"),
            ),
            gear: parse("gear.ron", include_str!("../../assets/data/gear.ron")),
        }
    }

    pub fn enemy_index(&self, id: &str) -> usize {
        self.enemies
            .iter()
            .position(|e| e.id == id)
            .unwrap_or_else(|| panic!("encounters.ron names unknown enemy `{id}`"))
    }
}

fn parse<T: DeserializeOwned>(file: &str, text: &str) -> T {
    ron::from_str(text).unwrap_or_else(|e| panic!("assets/data/{file}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The data files hold together: every reference resolves and no number
    /// would make the sim divide by zero, loop or roll an empty table.
    #[test]
    fn data_files_are_consistent() {
        let data = Data::load();

        assert!(!data.horses.is_empty());
        for horse in &data.horses {
            assert!(horse.stats.health > 0, "{}", horse.name);
        }

        let mut ids: Vec<&str> = data.enemies.iter().map(|e| e.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), data.enemies.len(), "an enemy id is used twice");
        for enemy in &data.enemies {
            assert!(enemy.health > 0 && enemy.attack_interval > 0.0, "{}", enemy.id);
            assert!(0 <= enemy.coins.0 && enemy.coins.0 <= enemy.coins.1, "{}", enemy.id);
        }

        let encounters = &data.encounters;
        assert!(!encounters.ordinary.is_empty() && !encounters.finals.is_empty());
        for encounter in encounters.ordinary.iter().chain(&encounters.finals) {
            assert!(!encounter.is_empty());
            for id in encounter {
                data.enemy_index(id);
            }
        }

        let gear = &data.gear;
        assert!(gear.health_per_point > 0);
        assert!(gear.rarities.iter().any(|r| r.weight > 0));
        assert!(gear.rarities.iter().any(|r| r.win_weight > 0));
        for rarity in &gear.rarities {
            assert!(0 < rarity.points.0 && rarity.points.0 <= rarity.points.1);
        }
        assert!(!gear.bases.is_empty());
        for base in &gear.bases {
            assert!(!base.names.is_empty(), "{:?}", base.slot);
        }

        let rules = &data.rules;
        assert!(rules.ordinary_encounters > 0);
        assert!(rules.attack_interval > 0.0 && rules.travel_secs > 0.0);
        assert!(rules.focused_shot_cooldown > 0.0 && rules.burst_cooldown > 0.0);
        assert!(rules.ward_cooldown > 0.0);
        assert!((0.0..=1.0).contains(&rules.bonus_chance));
        assert!((0.0..=1.0).contains(&rules.ward_reduction));
        assert!(rules.weak_multiplier < 1.0 && 1.0 < rules.strong_multiplier);
    }

    /// Every element beats one other and is beaten by one other.
    #[test]
    fn the_triangle_is_a_cycle() {
        for element in Element::ALL {
            assert_ne!(element.beats(), element);
            assert_eq!(element.beats().beats().beats(), element);
        }
    }

    /// No bridles, bits or reins: horses wear halters (CLAUDE.md, Vision).
    #[test]
    fn no_gear_is_a_bridle_bit_or_reins() {
        let data = Data::load();
        for name in data.gear.bases.iter().flat_map(|b| &b.names) {
            let lower = name.to_lowercase();
            for banned in ["bridle", "bit", "rein"] {
                assert!(
                    !lower.split_whitespace().any(|word| word.starts_with(banned)),
                    "gear.ron: `{name}`"
                );
            }
        }
    }
}