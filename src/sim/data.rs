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

/// Which one ability a horse has. Never shown to the player: there are no
/// classes, and this goes when abilities are bought from a shared pool (#221).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Class {
    Ranger,
    Wizard,
    Mage,
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
    /// How much sooner or later a horse's swing may come, as a fraction of
    /// its interval (#354).
    pub attack_jitter: f32,
    /// When a fight's first hit comes, and how far apart the horses' first
    /// hits are (#354).
    pub opening: f32,
    pub opening_stagger: f32,
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
    /// How many trails' reports the Journal keeps (#231).
    pub reports_kept: usize,
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
    /// Seconds between its ordinary hits (#354).
    pub attack_interval: f32,
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
    /// How many pieces of gear its kill throws, least and most (#352).
    pub gear: (u32, u32),
    /// The kind of kill it is: the name of the fountain its drops leave in,
    /// one of those in `loot_shower.ron`. The sim only carries the name.
    pub fountain: String,
    /// How it dies: the name of one of the deaths in `deaths.ron`. The sim
    /// only carries the name.
    pub death: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Encounters {
    pub ordinary: Vec<Vec<String>>,
    pub finals: Vec<Vec<String>>,
}

/// One rarity tier. A piece of gear names its tier by its place in
/// `GearTable::rarities`, which lists them lowest first, so a tier is added
/// in `gear.ron` alone.
#[derive(Clone, Debug, Deserialize)]
pub struct RarityDef {
    pub name: String,
    pub prefix: String,
    pub weight: u32,
    pub win_weight: u32,
    /// Its weight for the pieces every kill throws, which are the trail's
    /// chaff: the higher tiers come from the trail's own piece (#352).
    pub kill_weight: u32,
    pub points: (i32, i32),
    pub look: RarityLook,
}

/// How a tier's drops are told apart in the loot shower. Plain values; `fx`
/// makes the materials from them.
#[derive(Clone, Debug, Deserialize)]
pub struct RarityLook {
    /// sRGB hex, `"rrggbb"`.
    pub colour: String,
    /// How strongly the piece gives off its own colour; 0 is none.
    pub glow: f32,
    /// What the piece multiplies its `shape` by.
    pub size: f32,
    /// The solids the piece is built of, as it lies flat on the ground.
    pub shape: Vec<PartDef>,
    /// The column of light over a drop of this tier, if it has one.
    #[serde(default)]
    pub beam: Option<BeamDef>,
    /// What the screen does when a burst erupts with this tier the best in it.
    #[serde(default)]
    pub response: ResponseDef,
    /// The sound a drop of this tier makes as it is thrown, by its name in
    /// `sounds.ron`. Empty is none.
    #[serde(default)]
    pub sound: String,
}

impl RarityLook {
    /// How far the piece reaches sideways from its middle, at its `size`.
    pub fn reach(&self) -> f32 {
        self.size * self.shape.iter().map(PartDef::reach).fold(0.0, f32::max)
    }

    /// How thick the piece is as it lies flat, at its `size`.
    pub fn thickness(&self) -> f32 {
        2.0 * self.size * self.shape.iter().map(PartDef::rise).fold(0.0, f32::max)
    }
}

/// One solid of a tier's shape. Lengths are world units before the tier's
/// `size`; `fx` makes the meshes.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct PartDef {
    pub solid: Solid,
    /// Where its middle is from the piece's middle: along, up and across.
    #[serde(default)]
    pub at: (f32, f32, f32),
    /// Degrees it is turned about the upright axis.
    #[serde(default)]
    pub turn: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum Solid {
    /// Long, thick and deep.
    Box(f32, f32, f32),
    /// Its radius.
    Ball(f32),
    /// A ring lying flat: the radius of the circle its tube follows, and the
    /// tube's radius.
    Ring(f32, f32),
    /// A flat piece with this many sides: its radius to a corner, and its
    /// thickness. Many sides make a disc.
    Prism(u32, f32, f32),
}

impl PartDef {
    /// How far it reaches sideways from the piece's middle. A box is taken
    /// by its longer side, not its corners.
    pub fn reach(&self) -> f32 {
        let own = match self.solid {
            Solid::Box(long, _, deep) => long.max(deep) / 2.0,
            Solid::Ball(radius) | Solid::Prism(_, radius, _) => radius,
            Solid::Ring(radius, tube) => radius + tube,
        };
        self.at.0.hypot(self.at.2) + own
    }

    /// How far it reaches above or below the piece's middle.
    pub fn rise(&self) -> f32 {
        let own = match self.solid {
            Solid::Box(_, thick, _) | Solid::Prism(_, _, thick) => thick / 2.0,
            Solid::Ball(radius) => radius,
            Solid::Ring(_, tube) => tube,
        };
        self.at.1.abs() + own
    }
}

/// How strongly the screen answers a burst: none of either is 0.
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize)]
pub struct ResponseDef {
    /// How far the view is thrown, in world units.
    pub shake: f32,
    /// How solid the flash is at its start: 0 clear, 1 opaque.
    pub flash: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct BeamDef {
    pub height: f32,
    pub width: f32,
    /// How solid it is: 0 clear, 1 opaque.
    pub alpha: f32,
    /// Seconds it takes to rise to its height once the piece is thrown.
    pub rise_secs: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BaseDef {
    pub slot: Slot,
    pub names: Vec<String>,
    pub picture: PictureDef,
}

/// The stand-in picture of a slot's gear: one flat shape. Plain values; the
/// menus draw it.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct PictureDef {
    /// Its width and height, as shares of the picture's square.
    pub wide: f32,
    pub tall: f32,
    /// How far each corner is rounded, as a share of the square: top left,
    /// top right, bottom right, bottom left.
    pub round: (f32, f32, f32, f32),
}

#[derive(Clone, Debug, Deserialize)]
pub struct GearTable {
    pub health_per_point: i32,
    pub rarities: Vec<RarityDef>,
    pub bases: Vec<BaseDef>,
}

impl GearTable {
    /// The stand-in picture of a slot's gear.
    pub fn picture(&self, slot: Slot) -> PictureDef {
        self.bases
            .iter()
            .find(|base| base.slot == slot)
            .unwrap_or_else(|| panic!("gear.ron: no base for {slot:?}"))
            .picture
    }
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

pub(super) fn parse<T: DeserializeOwned>(file: &str, text: &str) -> T {
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
            assert!(horse.stats.health > 0 && horse.attack_interval > 0.0, "{}", horse.name);
            let Stats { attack, defense, elemental, .. } = horse.stats;
            assert!(attack >= 0 && defense >= 0 && elemental >= 0, "{}", horse.name);
            let named = data.horses.iter().filter(|h| h.name == horse.name).count();
            assert_eq!(named, 1, "horses.ron: two horses named `{}`", horse.name);
        }

        let mut ids: Vec<&str> = data.enemies.iter().map(|e| e.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), data.enemies.len(), "an enemy id is used twice");
        for enemy in &data.enemies {
            assert!(enemy.health > 0 && enemy.attack_interval > 0.0, "{}", enemy.id);
            assert!(enemy.attack >= 0 && enemy.defense >= 0, "{}", enemy.id);
            assert!(0 <= enemy.coins.0 && enemy.coins.0 <= enemy.coins.1, "{}", enemy.id);
            assert!(1 <= enemy.gear.0 && enemy.gear.0 <= enemy.gear.1, "{}", enemy.id);
            assert!(!enemy.fountain.is_empty(), "{}", enemy.id);
            assert!(!enemy.death.is_empty(), "{}", enemy.id);
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
        assert!(gear.rarities.iter().any(|r| r.kill_weight > 0));
        for rarity in &gear.rarities {
            assert!(0 < rarity.points.0 && rarity.points.0 <= rarity.points.1);
            assert!(!rarity.name.is_empty() && !rarity.prefix.is_empty());
            let named = gear.rarities.iter().filter(|r| r.name == rarity.name).count();
            assert_eq!(named, 1, "gear.ron: two rarities named `{}`", rarity.name);
            let look = &rarity.look;
            assert!(look.glow >= 0.0 && look.size > 0.0, "{}", rarity.name);
            assert!(!look.shape.is_empty(), "{}", rarity.name);
            for part in &look.shape {
                let sound = match part.solid {
                    Solid::Box(long, thick, deep) => long > 0.0 && thick > 0.0 && deep > 0.0,
                    Solid::Ball(radius) => radius > 0.0,
                    Solid::Ring(radius, tube) => 0.0 < tube && tube < radius,
                    Solid::Prism(sides, radius, thick) => sides >= 3 && radius > 0.0 && thick > 0.0,
                };
                assert!(sound, "{}: {part:?}", rarity.name);
            }
            if let Some(beam) = look.beam {
                assert!(beam.height > 0.0 && beam.width > 0.0, "{}", rarity.name);
                assert!(0.0 < beam.alpha && beam.alpha <= 1.0, "{}", rarity.name);
                assert!(beam.rise_secs >= 0.0, "{}", rarity.name);
            }
            let response = look.response;
            assert!(response.shake >= 0.0, "{}", rarity.name);
            assert!((0.0..=1.0).contains(&response.flash), "{}", rarity.name);
        }
        // Listed lowest first: a higher tier never spends fewer points.
        for pair in gear.rarities.windows(2) {
            assert!(pair[0].points.1 <= pair[1].points.0, "{}", pair[1].name);
        }
        assert!(!gear.bases.is_empty());
        for base in &gear.bases {
            assert!(!base.names.is_empty(), "{:?}", base.slot);
            let picture = base.picture;
            assert!(0.0 < picture.wide && picture.wide <= 1.0, "{:?}", base.slot);
            assert!(0.0 < picture.tall && picture.tall <= 1.0, "{:?}", base.slot);
            let (a, b, c, d) = picture.round;
            for round in [a, b, c, d] {
                assert!((0.0..=0.5).contains(&round), "{:?}", base.slot);
            }
        }
        // Every slot has a picture, and no two slots the same one: a slot
        // is told by its outline with no word beside it (#286).
        for (i, slot) in Slot::ALL.into_iter().enumerate() {
            for other in &Slot::ALL[i + 1..] {
                assert_ne!(gear.picture(slot), gear.picture(*other), "{slot:?} and {other:?}");
            }
        }

        let rules = &data.rules;
        assert!(rules.ordinary_encounters > 0);
        assert!(rules.opening > 0.0 && rules.opening_stagger > 0.0 && rules.travel_secs > 0.0);
        // A swing is never due at once or before the last.
        assert!((0.0..1.0).contains(&rules.attack_jitter));
        assert!(rules.focused_shot_cooldown > 0.0 && rules.burst_cooldown > 0.0);
        assert!(rules.ward_cooldown > 0.0);
        assert!((0.0..=1.0).contains(&rules.bonus_chance));
        assert!((0.0..=1.0).contains(&rules.ward_reduction));
        assert!((0.0..=1.0).contains(&rules.bonus_heal));
        assert!(0.0 < rules.weak_multiplier);
        assert!(rules.weak_multiplier < 1.0 && 1.0 < rules.strong_multiplier);
        // An ability is worth using, and the Ward is down before it is up again.
        assert!(rules.focused_shot_multiplier > 1.0 && rules.burst_multiplier > 0.0);
        assert!(0.0 < rules.ward_duration && rules.ward_duration < rules.ward_cooldown);
        assert!(rules.level_scaling >= 0.0);
    }

    /// A piece's reach and thickness cover every solid in it, wherever the
    /// solid sits, and grow with the tier's size.
    #[test]
    fn a_shape_is_measured_over_all_its_parts() {
        let part = |solid, at| PartDef { solid, at, turn: 0.0 };
        let mut look = Data::load().gear.rarities[0].look.clone();
        look.size = 1.0;
        look.shape = vec![part(Solid::Box(0.4, 0.2, 0.3), (0.0, 0.0, 0.0))];
        assert_eq!((look.reach(), look.thickness()), (0.2, 0.2));
        look.shape.push(part(Solid::Ball(0.1), (0.3, 0.0, 0.4)));
        assert!((look.reach() - 0.6).abs() < 1e-6 && look.thickness() == 0.2);
        look.shape.push(part(Solid::Ring(0.2, 0.05), (0.0, 0.25, 0.0)));
        assert!((look.thickness() - 0.6).abs() < 1e-6);
        look.shape.push(part(Solid::Prism(3, 0.7, 0.1), (0.0, 0.0, 0.0)));
        assert!((look.reach() - 0.7).abs() < 1e-6);
        look.size = 2.0;
        assert!((look.reach() - 1.4).abs() < 1e-6 && (look.thickness() - 1.2).abs() < 1e-6);
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