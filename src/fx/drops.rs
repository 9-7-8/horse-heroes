//! Drops as physical objects: a kill's drops leave the body one after
//! another, fanned out, each on its own arc with spin; they bounce, squash a
//! little and come to rest. Every number is in `assets/data/loot_shower.ron`.
//! A piece of gear is told apart by its rarity: colour, glow, size, and for
//! the tiers that have one a beam of light, all in `assets/data/gear.ron`.
//!
//! The flight is plain arithmetic (`Body`, `plan_burst`), so it is unit-tested;
//! the systems below it only draw it.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;
use serde::Deserialize;

use crate::game::menu::tokens::Hex;
use crate::game::scene::{ENEMY_X, SCENE_WIDTH, SCROLL_SPEED};
use crate::game::{Game, Killed};
use crate::sim::data::{BeamDef, GearTable, RarityDef};
use crate::sim::rng::Rng;

/// Seconds after launch at which `--burst` throws the test burst.
const BURST_AT: f32 = 1.0;
/// Directions tried for a piece before it is sent straight out sideways.
const TRIES: u32 = 8;
/// A drop is forgotten once it is this far left of the party's middle.
const GONE: f32 = SCENE_WIDTH / 2.0 + 4.0;

/// The feel numbers in `assets/data/loot_shower.ron`, compiled into the binary.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Shower {
    pub stagger_secs: f32,
    pub fan_degrees: f32,
    pub up_speed: (f32, f32),
    pub out_speed: (f32, f32),
    pub gravity: f32,
    pub spin: (f32, f32),
    pub bounces: u32,
    pub bounce: f32,
    pub grip: f32,
    pub squash: f32,
    pub squash_secs: f32,
    pub settle_secs: f32,
    pub coin_radius: f32,
    pub coin_thickness: f32,
    pub coin_colour: Hex,
    pub most_pieces: usize,
    pub gear_size: (f32, f32, f32),
    pub keep_clear: f32,
    pub burst_pieces: usize,
    pub burst_gear: Vec<String>,
    pub burst_seed: u64,
}

impl Shower {
    pub fn load() -> Shower {
        ron::from_str(include_str!("../../assets/data/loot_shower.ron"))
            .unwrap_or_else(|e| panic!("assets/data/loot_shower.ron: {e}"))
    }

    /// The test burst's gear as rarity tiers.
    pub fn burst_tiers(&self, gear: &GearTable) -> Vec<usize> {
        self.burst_gear
            .iter()
            .map(|name| {
                gear.rarities
                    .iter()
                    .position(|rarity| &rarity.name == name)
                    .unwrap_or_else(|| panic!("loot_shower.ron: no rarity named `{name}`"))
            })
            .collect()
    }
}

/// A rarity's colour. `gear.ron` holds it as text; the sim knows no colours.
pub fn rarity_colour(rarity: &RarityDef) -> Srgba {
    let text = &rarity.look.colour;
    Srgba::hex(text).unwrap_or_else(|e| panic!("gear.ron: {} colour `{text}`: {e}", rarity.name))
}

/// What a piece is: a coin, or a piece of gear of a rarity tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Coin,
    Gear(usize),
}

/// What a burst is made of, one entry for each launch of `plan_burst`. Each
/// launch has its own slice of the fan, so the seed puts the gear among the
/// coins and it is not thrown the same way every time.
pub fn burst_kinds(seed: u64, coins: usize, gear: &[usize]) -> Vec<Kind> {
    let mut kinds = vec![Kind::Coin; coins];
    kinds.extend(gear.iter().map(|&tier| Kind::Gear(tier)));
    Rng::new(seed ^ 0x6EA2_6EA2_6EA2_6EA2).shuffle(&mut kinds);
    kinds
}

/// A piece in flight. Its position is its middle; it touches the ground when
/// that is `coin_radius` up, so a piece on edge does not sink in.
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub position: Vec3,
    pub velocity: Vec3,
    pub bounces_left: u32,
    pub resting: bool,
}

impl Body {
    /// Move on by `dt`. True when it touched the ground in this step.
    pub fn step(&mut self, shower: &Shower, dt: f32) -> bool {
        if self.resting {
            return false;
        }
        self.velocity.y -= shower.gravity * dt;
        self.position += self.velocity * dt;
        if self.position.y > shower.coin_radius || self.velocity.y >= 0.0 {
            return false;
        }
        self.position.y = shower.coin_radius;
        if self.bounces_left == 0 {
            self.velocity = Vec3::ZERO;
            self.resting = true;
        } else {
            self.bounces_left -= 1;
            self.velocity.y *= -shower.bounce;
            self.velocity.x *= shower.grip;
            self.velocity.z *= shower.grip;
        }
        true
    }
}

/// How one piece leaves the body.
#[derive(Clone, Debug, PartialEq)]
pub struct Launch {
    /// Seconds after the kill at which it leaves.
    pub delay: f32,
    pub velocity: Vec3,
    /// The axis it tumbles about, and how fast, in radians a second.
    pub axis: Vec3,
    pub spin: f32,
    /// Which way it faces once it lies flat.
    pub yaw: f32,
}

impl Launch {
    pub fn body(&self, shower: &Shower, origin: Vec3) -> Body {
        Body {
            position: origin,
            velocity: self.velocity,
            bounces_left: shower.bounces,
            resting: false,
        }
    }
}

/// Where a launch comes to rest, found by flying it.
pub fn rest_point(shower: &Shower, origin: Vec3, launch: &Launch) -> Vec3 {
    let mut body = launch.body(shower, origin);
    // A minute of flight is far past any real one.
    for _ in 0..7200 {
        if body.resting {
            break;
        }
        body.step(shower, 1.0 / 120.0);
    }
    body.position
}

fn is_clear(point: Vec3, standing: &[Vec3], keep_clear: f32) -> bool {
    standing
        .iter()
        .all(|enemy| point.xz().distance(enemy.xz()) > keep_clear)
}

/// Plan a burst of `count` pieces from `origin`. Everything follows from the
/// seed. Each piece gets its own slice of the fan, so a burst is spread out
/// however few pieces it has, and no piece is sent to rest on an enemy still
/// `standing`.
pub fn plan_burst(
    shower: &Shower,
    seed: u64,
    count: usize,
    origin: Vec3,
    standing: &[Vec3],
) -> Vec<Launch> {
    let mut rng = Rng::new(seed);
    let mut order: Vec<usize> = (0..count).collect();
    rng.shuffle(&mut order);
    let fan = shower.fan_degrees.to_radians();
    let between = |rng: &mut Rng, (lo, hi): (f32, f32)| lo + (hi - lo) * rng.unit();

    (0..count)
        .map(|i| {
            let up = between(&mut rng, shower.up_speed);
            let out = between(&mut rng, shower.out_speed);
            let tumble = rng.unit() * TAU;
            let mut launch = Launch {
                delay: order[i] as f32 * shower.stagger_secs,
                velocity: Vec3::ZERO,
                axis: Vec3::new(tumble.cos(), 0.0, tumble.sin()),
                spin: between(&mut rng, shower.spin) * TAU,
                yaw: rng.unit() * TAU,
            };
            // Angle 0 points at the party, which is to the left.
            let slice = fan * ((i as f32 + rng.unit()) / count as f32 - 0.5);
            let mut angle = slice;
            let mut tries = 0;
            loop {
                launch.velocity = Vec3::new(-angle.cos() * out, up, angle.sin() * out);
                let rest = rest_point(shower, origin, &launch);
                if tries == TRIES || is_clear(rest, standing, shower.keep_clear) {
                    break;
                }
                tries += 1;
                angle = if tries < TRIES {
                    fan * (rng.unit() - 0.5)
                } else {
                    // Enemies stand in a line along the trail, so straight
                    // across it is clear of them.
                    FRAC_PI_2.copysign(slice)
                };
            }
            launch
        })
        .collect()
}

pub struct DropsPlugin {
    pub burst: bool,
}

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Shower::load())
            .insert_resource(TestBurst {
                due: self.burst.then_some(BURST_AT),
                watch: None,
                calm: Watch::default(),
            })
            .add_systems(Startup, setup)
            .add_systems(Update, (launch_kills, test_burst, fly).chain());
    }
}

/// One drop in the world. The entity carries where it is and its squash; its
/// child, the model, carries its tumble.
#[derive(Component)]
struct Piece {
    /// Seconds until it leaves the body. It is hidden until then.
    delay: f32,
    body: Body,
    axis: Vec3,
    spin: f32,
    tumble: Quat,
    /// The way it lies once it has stopped, set when it stops.
    flat: Quat,
    yaw: f32,
    /// 1 at a landing, falling back to 0.
    squash: f32,
    /// 0 when it stops, rising to 1 as it lies down.
    settle: f32,
    /// Part of the test burst: it stays where it lands, to be looked at.
    pinned: bool,
    /// Half its length, which is how high its middle is while it tumbles,
    /// and its thickness, which it lies on.
    radius: f32,
    thickness: f32,
    model: Entity,
    beam: Option<Beam>,
}

/// The beam of light over a piece: a child of the piece, like its model.
struct Beam {
    model: Entity,
    def: BeamDef,
    /// 0 when the piece is thrown, rising to 1 as the beam reaches its height.
    risen: f32,
}

/// A drawn part of a piece: its body, or its beam.
#[derive(Component)]
struct Model;

#[derive(Resource)]
struct Look {
    coin: Handle<Mesh>,
    gold: Handle<StandardMaterial>,
    /// A piece of gear, before its rarity's size.
    gear: Handle<Mesh>,
    /// A beam one unit wide and one tall.
    beam: Handle<Mesh>,
    /// One for each rarity, in the order of `gear.ron`.
    tiers: Vec<Tier>,
    burst_gear: Vec<usize>,
}

struct Tier {
    material: Handle<StandardMaterial>,
    size: f32,
    beam: Option<(Handle<StandardMaterial>, BeamDef)>,
}

/// The test burst: the same pieces from the same seed every time, so one
/// run can be compared with another.
#[derive(Resource)]
struct TestBurst {
    /// Seconds until `--burst` throws it.
    due: Option<f32>,
    /// Frame times while its pieces are in the air, reported when they rest.
    watch: Option<Watch>,
    /// Frame times with no test burst in the air, to compare them with.
    calm: Watch,
}

#[derive(Default)]
struct Watch {
    frames: u32,
    secs: f32,
    worst: f32,
}

impl Watch {
    fn add(&mut self, dt: f32) {
        self.frames += 1;
        self.secs += dt;
        self.worst = self.worst.max(dt);
    }

    fn mean_ms(&self) -> f32 {
        1000.0 * self.secs / self.frames.max(1) as f32
    }
}

fn setup(
    mut commands: Commands,
    game: Res<Game>,
    shower: Res<Shower>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tiers = game
        .data
        .gear
        .rarities
        .iter()
        .map(|rarity| {
            let colour = rarity_colour(rarity);
            Tier {
                material: materials.add(StandardMaterial {
                    base_color: colour.into(),
                    emissive: LinearRgba::from(colour) * rarity.look.glow,
                    ..default()
                }),
                size: rarity.look.size,
                beam: rarity.look.beam.map(|beam| {
                    let light = materials.add(StandardMaterial {
                        base_color: colour.with_alpha(beam.alpha).into(),
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    });
                    (light, beam)
                }),
            }
        })
        .collect();
    let (length, thickness, depth) = shower.gear_size;
    commands.insert_resource(Look {
        coin: meshes.add(Cylinder::new(shower.coin_radius, shower.coin_thickness)),
        gold: materials.add(shower.coin_colour.0),
        gear: meshes.add(Cuboid::new(length, thickness, depth)),
        beam: meshes.add(Cylinder::new(0.5, 1.0)),
        tiers,
        burst_gear: shower.burst_tiers(&game.data.gear),
    });
}

fn spawn_burst(
    commands: &mut Commands,
    look: &Look,
    shower: &Shower,
    origin: Vec3,
    launches: &[Launch],
    kinds: &[Kind],
    pinned: bool,
) {
    for (launch, kind) in launches.iter().zip(kinds) {
        let tier = match kind {
            Kind::Coin => None,
            Kind::Gear(tier) => Some(&look.tiers[*tier]),
        };
        let (mesh, material, size) = match tier {
            None => (&look.coin, &look.gold, 1.0),
            Some(tier) => (&look.gear, &tier.material, tier.size),
        };
        let (radius, thickness) = match tier {
            None => (shower.coin_radius, shower.coin_thickness),
            Some(tier) => (shower.gear_size.0 / 2.0 * tier.size, shower.gear_size.1 * tier.size),
        };
        let mut model = Entity::PLACEHOLDER;
        let mut beam = None;
        let mut piece = commands.spawn((Transform::from_translation(origin), Visibility::Hidden));
        piece.with_children(|piece| {
            model = piece
                .spawn((
                    Model,
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_scale(Vec3::splat(size)),
                ))
                .id();
            // The beam is hidden with the piece, so it appears as the piece
            // is thrown, and rises from nothing.
            if let Some((light, def)) = tier.and_then(|tier| tier.beam.as_ref()) {
                let model = piece
                    .spawn((
                        Model,
                        Mesh3d(look.beam.clone()),
                        MeshMaterial3d(light.clone()),
                        Transform::from_scale(Vec3::ZERO),
                    ))
                    .id();
                beam = Some(Beam {
                    model,
                    def: *def,
                    risen: 0.0,
                });
            }
        });
        piece.insert(Piece {
            delay: launch.delay,
            body: launch.body(shower, origin),
            axis: launch.axis,
            spin: launch.spin,
            tumble: Quat::from_rotation_y(launch.yaw),
            flat: Quat::IDENTITY,
            yaw: launch.yaw,
            squash: 0.0,
            settle: 0.0,
            pinned,
            radius,
            thickness,
            model,
            beam,
        });
    }
}

/// A kill's drops leave its body: one piece a coin up to the most shown, and
/// its gear if it dropped any.
fn launch_kills(
    mut commands: Commands,
    mut killed: MessageReader<Killed>,
    look: Res<Look>,
    shower: Res<Shower>,
    mut bursts: Local<u64>,
) {
    for kill in killed.read() {
        let coins = (kill.coins.max(0) as usize).min(shower.most_pieces);
        let gear: Vec<usize> = kill.gear.into_iter().collect();
        let kinds = burst_kinds(*bursts, coins, &gear);
        let launches = plan_burst(&shower, *bursts, kinds.len(), kill.at, &kill.standing);
        *bursts += 1;
        spawn_burst(&mut commands, &look, &shower, kill.at, &launches, &kinds, false);
    }
}

/// F3, or `--burst` a moment after launch, throws the test burst from where
/// the first enemy stands, in place of the last one thrown.
fn test_burst(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    look: Res<Look>,
    shower: Res<Shower>,
    mut test: ResMut<TestBurst>,
    pieces: Query<(Entity, &Piece)>,
) {
    let mut throw = keys.just_pressed(KeyCode::F3);
    if let Some(due) = &mut test.due {
        *due -= time.delta_secs();
        if *due <= 0.0 {
            test.due = None;
            throw = true;
        }
    }
    if !throw {
        return;
    }
    for (entity, piece) in &pieces {
        if piece.pinned {
            commands.entity(entity).despawn();
        }
    }
    let origin = Vec3::new(ENEMY_X[0], 0.5, 0.0);
    let kinds = burst_kinds(shower.burst_seed, shower.burst_pieces, &look.burst_gear);
    let launches = plan_burst(&shower, shower.burst_seed, kinds.len(), origin, &[]);
    spawn_burst(&mut commands, &look, &shower, origin, &launches, &kinds, true);
    test.watch = Some(Watch::default());
}

fn fly(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<Game>,
    shower: Res<Shower>,
    mut test: ResMut<TestBurst>,
    mut pieces: Query<(Entity, &mut Piece, &mut Transform, &mut Visibility), Without<Model>>,
    mut models: Query<&mut Transform, With<Model>>,
) {
    let dt = time.delta_secs();
    let drift = if game.party_moving() { SCROLL_SPEED * dt } else { 0.0 };
    let (mut pinned, mut pinned_moving) = (0, 0);

    for (entity, mut piece, mut transform, mut visibility) in &mut pieces {
        let piece = &mut *piece;
        if piece.pinned {
            pinned += 1;
            pinned_moving += !piece.body.resting as u32;
        }
        if piece.delay > 0.0 {
            piece.delay -= dt;
            if piece.delay > 0.0 {
                continue;
            }
            *visibility = Visibility::Inherited;
        }

        let was_resting = piece.body.resting;
        if piece.body.step(&shower, dt) {
            piece.squash = 1.0;
            piece.spin *= shower.grip;
        }
        if piece.body.resting {
            if !was_resting {
                // A coin lies on whichever face is nearer the ground.
                let face_down = (piece.tumble * Vec3::Y).y < 0.0;
                let turn = if face_down { PI } else { 0.0 };
                piece.flat = Quat::from_rotation_y(piece.yaw) * Quat::from_rotation_x(turn);
            }
            piece.settle = (piece.settle + dt / shower.settle_secs.max(0.001)).min(1.0);
        } else {
            piece.tumble = Quat::from_axis_angle(piece.axis, piece.spin * dt) * piece.tumble;
        }
        piece.squash = (piece.squash - dt / shower.squash_secs.max(0.001)).max(0.0);

        // The ground moves under the party as it walks, and takes with it
        // what lies on it and what is falling to it.
        if !piece.pinned {
            piece.body.position.x -= drift;
            if piece.body.position.x < -GONE {
                commands.entity(entity).despawn();
                continue;
            }
        }

        // Lying down brings its middle from a radius up to half its thickness.
        let lying = piece.settle * piece.settle * (3.0 - 2.0 * piece.settle);
        let flattened = piece.squash * shower.squash;
        let height = piece.radius + (piece.thickness / 2.0 - piece.radius) * lying;
        transform.translation = piece.body.position;
        transform.translation.y += height * (1.0 - flattened) - shower.coin_radius;
        transform.scale = Vec3::new(1.0 + flattened / 2.0, 1.0 - flattened, 1.0 + flattened / 2.0);
        if let Ok(mut model) = models.get_mut(piece.model) {
            model.rotation = piece.tumble.slerp(piece.flat, lying);
        }
        // The beam stands on the ground where the piece is, upright and its
        // own size whatever the piece is doing: undo the piece's height and
        // squash.
        if let Some(beam) = &mut piece.beam {
            beam.risen = (beam.risen + dt / beam.def.rise_secs.max(0.001)).min(1.0);
            if let Ok(mut model) = models.get_mut(beam.model) {
                let height = beam.def.height * beam.risen;
                let middle = height / 2.0 - transform.translation.y;
                model.translation = Vec3::new(0.0, middle / transform.scale.y, 0.0);
                model.scale = Vec3::new(beam.def.width, height, beam.def.width) / transform.scale;
            }
        }
    }

    // Say how the test burst went, once: its frame times, beside those with
    // no burst in the air, are what "holds frame rate" is checked against.
    let test = &mut *test;
    let Some(watch) = &mut test.watch else {
        // The first frames of a launch are slow for reasons of their own.
        if time.elapsed_secs() > 0.5 {
            test.calm.add(dt);
        }
        return;
    };
    watch.add(dt);
    if pinned > 0 && pinned_moving == 0 {
        info!(
            "test burst: {pinned} pieces at rest after {:.2} s, mean frame {:.1} ms, worst {:.1} ms \
             (with none in the air: mean {:.1} ms, worst {:.1} ms)",
            watch.secs,
            watch.mean_ms(),
            1000.0 * watch.worst,
            test.calm.mean_ms(),
            1000.0 * test.calm.worst,
        );
        test.watch = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::scene::{TRAIL_DEPTH, element_colour};
    use crate::sim::data::{Data, Element};

    const ORIGIN: Vec3 = Vec3::new(ENEMY_X[0], 0.5, 0.0);

    /// The numbers hold together: a piece flies, comes down and stops.
    #[test]
    fn the_feel_numbers_are_usable() {
        let s = Shower::load();
        assert!(s.gravity > 0.0 && s.stagger_secs >= 0.0);
        for (lo, hi) in [s.up_speed, s.out_speed, s.spin] {
            assert!(0.0 <= lo && lo <= hi);
        }
        assert!(s.up_speed.0 > 0.0);
        assert!((0.0..1.0).contains(&s.bounce) && (0.0..=1.0).contains(&s.grip));
        assert!((0.0..1.0).contains(&s.squash));
        assert!(0.0 < s.fan_degrees && s.fan_degrees <= 360.0);
        assert!(0.0 < s.coin_thickness && s.coin_thickness <= 2.0 * s.coin_radius);
        assert!(s.most_pieces > 0 && s.burst_pieces > 0);
        let (length, thickness, depth) = s.gear_size;
        assert!(0.0 < thickness && thickness <= depth && depth <= length);
        // Enemies stand further apart than the room kept round each, or the
        // way out sideways would not be clear.
        assert!(s.keep_clear < ENEMY_X[1] - ENEMY_X[0]);
    }

    /// The test burst is a mixed one: it throws gear of every rarity.
    #[test]
    fn the_test_burst_has_every_rarity() {
        let (s, data) = (Shower::load(), Data::load());
        let tiers = s.burst_tiers(&data.gear);
        for tier in 0..data.gear.rarities.len() {
            assert!(tiers.contains(&tier), "no {}", data.gear.rarities[tier].name);
        }
    }

    /// A burst holds what it was given, and the seed decides where in the
    /// fan the gear goes.
    #[test]
    fn gear_is_put_among_the_coins_by_the_seed() {
        let gear = [0, 2];
        let mut places = Vec::new();
        for seed in 0..50 {
            let kinds = burst_kinds(seed, 12, &gear);
            assert_eq!(kinds, burst_kinds(seed, 12, &gear));
            assert_eq!(kinds.len(), 14);
            assert_eq!(kinds.iter().filter(|&&kind| kind == Kind::Coin).count(), 12);
            for tier in gear {
                assert!(kinds.contains(&Kind::Gear(tier)));
            }
            places.push(kinds.iter().position(|&kind| kind == Kind::Gear(2)).unwrap());
        }
        places.sort();
        places.dedup();
        assert!(places.len() > 7, "the Rare was thrown only {} ways", places.len());
    }

    /// Rarities are told apart without colour, lighter up the tiers, and
    /// their colours are not an element's colour or the coins' gold. The top
    /// tier has a beam.
    #[test]
    fn rarity_colours_are_their_own() {
        let (s, data) = (Shower::load(), Data::load());
        let rarities = &data.gear.rarities;
        let luma = |c: Srgba| 0.299 * c.red + 0.587 * c.green + 0.114 * c.blue;
        for pair in rarities.windows(2) {
            let step = luma(rarity_colour(&pair[1])) - luma(rarity_colour(&pair[0]));
            assert!(step >= 0.1, "{} is only {step} lighter", pair[1].name);
            assert!(pair[1].look.glow >= pair[0].look.glow && pair[1].look.size >= pair[0].look.size);
        }
        assert!(rarities.last().unwrap().look.beam.is_some());

        let mut taken: Vec<(String, Hsla)> = Element::ALL
            .iter()
            .map(|&element| (element.name().into(), element_colour(element).into()))
            .collect();
        taken.push(("coin gold".into(), s.coin_colour.0.into()));
        for rarity in rarities {
            let colour = Hsla::from(rarity_colour(rarity));
            // A colour near grey has no hue to mistake.
            if colour.saturation < 0.15 {
                continue;
            }
            for (name, other) in &taken {
                let apart = (colour.hue - other.hue).abs();
                let apart = apart.min(360.0 - apart);
                assert!(apart >= 40.0, "{} is {apart} degrees from {name}", rarity.name);
            }
        }
    }

    #[test]
    fn same_seed_same_burst() {
        let s = Shower::load();
        let a = plan_burst(&s, 7, 24, ORIGIN, &[]);
        assert_eq!(a, plan_burst(&s, 7, 24, ORIGIN, &[]));
        assert_ne!(a, plan_burst(&s, 8, 24, ORIGIN, &[]));
    }

    /// No two pieces leave together, and each leaves upwards within the
    /// speeds the data gives.
    #[test]
    fn launches_are_staggered_and_within_their_speeds() {
        let s = Shower::load();
        let burst = plan_burst(&s, 1, 40, ORIGIN, &[]);
        let mut delays: Vec<f32> = burst.iter().map(|l| l.delay).collect();
        delays.sort_by(f32::total_cmp);
        for (i, delay) in delays.iter().enumerate() {
            assert!((delay - i as f32 * s.stagger_secs).abs() < 1e-5);
        }
        for launch in &burst {
            let out = launch.velocity.xz().length();
            assert!((s.up_speed.0..=s.up_speed.1).contains(&launch.velocity.y));
            assert!((s.out_speed.0 - 1e-4..=s.out_speed.1 + 1e-4).contains(&out));
            assert!((launch.axis.length() - 1.0).abs() < 1e-4);
        }
    }

    /// A burst fans out: it is not all thrown the same way.
    #[test]
    fn a_burst_is_spread_over_the_fan() {
        let s = Shower::load();
        let burst = plan_burst(&s, 2, 40, ORIGIN, &[]);
        let near = burst.iter().filter(|l| l.velocity.z > 0.0).count();
        assert!((10..=30).contains(&near), "{near} of 40 towards the camera");
        let towards_party = burst.iter().filter(|l| l.velocity.x < 0.0).count();
        assert!(towards_party > 20);
    }

    /// Every piece bounces as often as the data says, then rests on the
    /// ground, on the trail.
    #[test]
    fn every_piece_bounces_and_settles_on_the_trail() {
        let s = Shower::load();
        for seed in 0..50 {
            for launch in plan_burst(&s, seed, 40, ORIGIN, &[]) {
                let mut body = launch.body(&s, ORIGIN);
                let (mut touches, mut top) = (0, 0.0f32);
                for _ in 0..1200 {
                    touches += body.step(&s, 1.0 / 120.0) as u32;
                    top = top.max(body.position.y);
                }
                assert!(body.resting, "seed {seed}: still moving after 10 s");
                assert_eq!(touches, s.bounces + 1);
                assert!(top > ORIGIN.y + 0.5, "seed {seed}: no arc");
                assert_eq!(body.position.y, s.coin_radius);
                assert!(body.position.z.abs() < TRAIL_DEPTH / 2.0, "seed {seed}: off the trail");
                assert_eq!(body.position, rest_point(&s, ORIGIN, &launch));
            }
        }
    }

    /// Drops do not land on enemies still alive, whichever of them died.
    #[test]
    fn no_piece_rests_on_a_standing_enemy() {
        let s = Shower::load();
        let places: Vec<Vec3> = ENEMY_X.iter().map(|&x| Vec3::new(x, 0.5, 0.0)).collect();
        for seed in 0..200 {
            for dead in 0..places.len() {
                let standing: Vec<Vec3> = (0..places.len())
                    .filter(|&i| i != dead)
                    .map(|i| places[i])
                    .collect();
                for launch in plan_burst(&s, seed, s.most_pieces, places[dead], &standing) {
                    let rest = rest_point(&s, places[dead], &launch);
                    assert!(
                        is_clear(rest, &standing, s.keep_clear),
                        "seed {seed}, enemy {dead}: rests at {rest}"
                    );
                }
            }
        }
    }
}
