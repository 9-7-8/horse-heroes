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

/// What an ability does when it fires (#357), with its numbers.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum AbilityKind {
    /// One hit on the first enemy standing: attack times this, past
    /// defense, and the horse's elemental in the ability's element.
    Shot(f32),
    /// The horse's elemental times this, in the ability's element, on every
    /// enemy standing.
    Burst(f32),
    /// The party takes this share less from enemy hits for these seconds.
    Ward(f32, f32),
}

/// One ability, from `abilities.ron` (#357). Its element is its own: a
/// horse's plain hits have none.
#[derive(Clone, Debug, Deserialize)]
pub struct AbilityDef {
    pub id: String,
    pub name: String,
    pub element: Option<Element>,
    pub kind: AbilityKind,
    pub cooldown: f32,
    /// Seconds to its first use on a trail; the cooldown when not given.
    #[serde(default)]
    pub first: Option<f32>,
    pub does: String,
    /// The sound it makes as it is used, by its name in `sounds.ron`
    /// (#358). Its hits sound by their element, apart from this.
    pub sound: String,
}

impl AbilityDef {
    /// Its name with its element, as the menus and the fight log say it:
    /// "Ember Shot (Fire)".
    pub fn named(&self) -> String {
        match self.element {
            Some(element) => format!("{} ({})", self.name, element.name()),
            None => self.name.clone(),
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct Stats {
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub elemental: i32,
    /// How often a horse hits and how quickly the party walks between
    /// encounters (#355), by `attack_tempo` and `travel_distance` in
    /// `rules.ron`.
    pub speed: i32,
}

impl Stats {
    pub fn plus(self, other: Stats) -> Stats {
        Stats {
            health: self.health + other.health,
            attack: self.attack + other.attack,
            defense: self.defense + other.defense,
            elemental: self.elemental + other.elemental,
            speed: self.speed + other.speed,
        }
    }

    /// Every stat with the word the menus name it by, in the one order
    /// they are listed in.
    pub fn named(self) -> [(&'static str, i32); 5] {
        [
            ("health", self.health),
            ("attack", self.attack),
            ("defense", self.defense),
            ("elemental", self.elemental),
            ("speed", self.speed),
        ]
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
    /// Seconds the trail's final enemy takes to come up before its fight's
    /// first blow (#19): nothing is struck or cast while it lasts.
    pub final_approach: f32,
    /// Speed (#355): a horse's ordinary hits come `attack_tempo / speed`
    /// seconds apart, and the road between encounters takes
    /// `travel_distance / speed` at the party's average speed.
    pub attack_tempo: f32,
    pub travel_distance: f32,
    pub ordinary_encounters: usize,
    pub bonus_chance: f32,
    pub bonus_heal: f32,
    pub level_scaling: f32,
    /// How many trails' reports the Journal keeps (#231).
    pub reports_kept: usize,
}

impl Rules {
    /// Damage multiplier for an elemental attack against a target's element.
    /// A hit with no element is neutral.
    pub fn multiplier(&self, attack: Option<Element>, target: Element) -> f32 {
        let Some(attack) = attack else {
            return 1.0;
        };
        if attack.beats() == target {
            self.strong_multiplier
        } else if target.beats() == attack {
            self.weak_multiplier
        } else {
            1.0
        }
    }

    /// Seconds between a horse's ordinary hits at its speed, before each
    /// swing's jitter (#355): the faster, the more often.
    pub fn attack_interval(&self, speed: i32) -> f32 {
        self.attack_tempo / speed.max(1) as f32
    }

    /// Seconds the road between two encounters takes a party of these
    /// speeds, which walks at their average (vision.md, Gaits).
    pub fn travel_secs(&self, speeds: impl IntoIterator<Item = i32>) -> f32 {
        let (sum, count) = speeds.into_iter().fold((0, 0), |(sum, count), speed| (sum + speed, count + 1));
        let average = if count == 0 { 1.0 } else { sum as f32 / count as f32 };
        self.travel_distance / average.max(1.0)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HorseDef {
    pub name: String,
    /// The ability it starts with, by its id in `abilities.ron`.
    pub ability: String,
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

/// A flat mark a piece's look is drawn with (`looks.ron`).
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum Mark {
    Bar,
    Disc,
    /// An ellipse's rim: the share of its radius that is rim.
    Ring(f32),
}

/// One mark of a decoration, in its box from -1 to 1 each way.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct MarkDef {
    pub mark: Mark,
    pub size: (f32, f32),
    #[serde(default)]
    pub at: (f32, f32),
    #[serde(default)]
    pub turn: f32,
}

/// A pattern: one mark repeated in a grid over a whole piece.
#[derive(Clone, Debug, Deserialize)]
pub struct PatternDef {
    #[allow(dead_code)]
    pub name: String,
    pub mark: Mark,
    pub size: (f32, f32),
    pub step: (f32, f32),
    #[serde(default)]
    pub stagger: f32,
    #[serde(default)]
    pub turn: f32,
    #[serde(default)]
    pub spin: f32,
}

/// A decoration: a few marks, drawn on top of the pattern.
#[derive(Clone, Debug, Deserialize)]
pub struct DecorationDef {
    #[allow(dead_code)]
    pub name: String,
    pub marks: Vec<MarkDef>,
}

/// What every piece's look is rolled from (#361).
#[derive(Clone, Debug, Deserialize)]
pub struct LookTable {
    /// Colours as hex text; the sim knows no colours.
    pub inks: Vec<String>,
    pub patterns: Vec<PatternDef>,
    pub decorations: Vec<DecorationDef>,
    /// Where the decoration goes: its middle as shares of the piece's width
    /// and height, and its side as a share of the shorter side.
    pub picture_decoration: (f32, f32, f32),
    pub worn_decoration: (f32, f32, f32),
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
    pub abilities: Vec<AbilityDef>,
    pub enemies: Vec<EnemyDef>,
    pub encounters: Encounters,
    pub gear: GearTable,
    pub looks: LookTable,
}

impl Data {
    pub fn load() -> Data {
        Data {
            rules: parse("rules.ron", include_str!("../../assets/data/rules.ron")),
            horses: parse("horses.ron", include_str!("../../assets/data/horses.ron")),
            abilities: parse("abilities.ron", include_str!("../../assets/data/abilities.ron")),
            enemies: parse("enemies.ron", include_str!("../../assets/data/enemies.ron")),
            encounters: parse(
                "encounters.ron",
                include_str!("../../assets/data/encounters.ron"),
            ),
            gear: parse("gear.ron", include_str!("../../assets/data/gear.ron")),
            looks: parse("looks.ron", include_str!("../../assets/data/looks.ron")),
        }
    }

    pub fn enemy_index(&self, id: &str) -> usize {
        self.enemies
            .iter()
            .position(|e| e.id == id)
            .unwrap_or_else(|| panic!("encounters.ron names unknown enemy `{id}`"))
    }

    pub fn ability_index(&self, id: &str) -> usize {
        self.abilities
            .iter()
            .position(|a| a.id == id)
            .unwrap_or_else(|| panic!("no ability `{id}` in abilities.ron"))
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
            assert!(horse.stats.health > 0 && horse.stats.speed > 0, "{}", horse.name);
            let Stats { attack, defense, elemental, .. } = horse.stats;
            assert!(attack >= 0 && defense >= 0 && elemental >= 0, "{}", horse.name);
            let named = data.horses.iter().filter(|h| h.name == horse.name).count();
            assert_eq!(named, 1, "horses.ron: two horses named `{}`", horse.name);
            data.ability_index(&horse.ability);
        }

        // Abilities (#357): each element has one that deals damage, and the
        // numbers make each worth using.
        for ability in &data.abilities {
            let id = &ability.id;
            assert_eq!(data.abilities.iter().filter(|a| a.id == *id).count(), 1, "{id}");
            assert_eq!(data.abilities.iter().filter(|a| a.name == ability.name).count(), 1, "{id}");
            assert!(!ability.name.is_empty() && !ability.does.is_empty(), "{id}");
            assert!(ability.cooldown > 0.0 && ability.first.is_none_or(|first| first > 0.0), "{id}");
            match ability.kind {
                AbilityKind::Shot(multiplier) => assert!(multiplier > 1.0, "{id}"),
                AbilityKind::Burst(multiplier) => assert!(multiplier > 0.0, "{id}"),
                // The Ward is down before it is up again.
                AbilityKind::Ward(secs, reduction) => {
                    assert!(0.0 < secs && secs < ability.cooldown, "{id}");
                    assert!((0.0..=1.0).contains(&reduction), "{id}");
                }
            }
        }
        for element in Element::ALL {
            let damage = |a: &&AbilityDef| !matches!(a.kind, AbilityKind::Ward(..));
            assert!(
                data.abilities.iter().filter(damage).any(|a| a.element == Some(element)),
                "no ability deals {element:?}"
            );
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
        assert!(rules.opening > 0.0 && rules.opening_stagger > 0.0);
        assert!(rules.final_approach > 0.0 && rules.final_approach < 5.0);
        assert!(rules.attack_tempo > 0.0 && rules.travel_distance > 0.0);
        // A swing is never due at once or before the last.
        assert!((0.0..1.0).contains(&rules.attack_jitter));
        assert!((0.0..=1.0).contains(&rules.bonus_chance));
        assert!((0.0..=1.0).contains(&rules.bonus_heal));
        assert!(0.0 < rules.weak_multiplier);
        assert!(rules.weak_multiplier < 1.0 && 1.0 < rules.strong_multiplier);
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

    /// The party walks the road at its horses' average speed (vision.md,
    /// Gaits): a slow horse slows it and a fast one hurries it, by the mean.
    #[test]
    fn the_road_takes_the_party_s_average_speed() {
        let rules = Data::load().rules;
        let close = |a: f32, b: f32| (a - b).abs() < 1e-4;
        let at = |speed: f32| rules.travel_distance / speed;
        assert!(close(rules.travel_secs([4]), at(4.0)));
        assert!(close(rules.travel_secs([2, 4, 6]), at(4.0)));
        assert!(close(rules.travel_secs([1, 2]), at(1.5)));
        assert!(rules.travel_secs([8, 8, 8]) < rules.travel_secs([4, 4, 4]));
        // No horses, or no speed, is a walk at speed 1, never a division by 0.
        assert!(close(rules.travel_secs([]), at(1.0)));
        assert!(close(rules.travel_secs([0, 0]), at(1.0)));
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