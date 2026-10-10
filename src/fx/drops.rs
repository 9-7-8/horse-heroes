//! Drops as physical objects: a kill's drops leave the body one after
//! another, fanned out, each on its own arc with spin; they bounce, squash a
//! little and come to rest. Every number is in `assets/data/loot_shower.ron`.
//! How big the burst is depends on the kind of kill: each enemy names its
//! fountain, small for an ordinary one and the full one for a trail's last.
//! A piece of gear is told apart by its rarity: colour, glow, size, and for
//! the tiers that have one a beam of light, all in `assets/data/gear.ron`.
//! A burst lies still for a moment, then its pieces go to the party one after
//! another, and the coins counter climbs as each coin arrives.
//!
//! The flight is plain arithmetic (`Body`, `plan_burst`), so it is unit-tested;
//! the systems below it only draw it.

use std::collections::BTreeMap;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;
use serde::Deserialize;

use crate::game::menu::tokens::Hex;
use crate::game::scene::{ENEMY_X, SCENE_WIDTH, SCROLL_SPEED, nearest_horse};
use crate::game::{Game, Killed, Uncollected};
use crate::sim::data::{BeamDef, GearTable, RarityDef};
use crate::sim::rng::Rng;

/// Seconds after launch at which `--burst` throws the test burst, unless
/// `--burst-at` says otherwise.
pub const BURST_AT: f32 = 1.0;
/// Directions tried for a piece before it is sent straight out sideways.
const TRIES: u32 = 8;
/// A drop is forgotten once it is this far left of the party's middle.
const GONE: f32 = SCENE_WIDTH / 2.0 + 4.0;

/// The feel numbers in `assets/data/loot_shower.ron`, compiled into the binary.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Shower {
    /// One for each kind of kill, by the name an enemy gives in
    /// `enemies.ron`. A new kind is one more entry there and no code.
    pub fountains: BTreeMap<String, Fountain>,
    pub gravity: f32,
    pub bounces: u32,
    pub bounce: f32,
    pub grip: f32,
    pub squash: f32,
    pub squash_secs: f32,
    pub settle_secs: f32,
    pub coin_radius: f32,
    pub coin_thickness: f32,
    pub coin_colour: Hex,
    pub gear_size: (f32, f32, f32),
    pub keep_clear: f32,
    pub pickup_pause_secs: f32,
    pub pickup_stagger_secs: f32,
    pub pickup_speed: f32,
    pub pickup_accel: f32,
    pub pickup_shrink: f32,
    pub burst_fountain: String,
    pub burst_pieces: usize,
    pub burst_gear: Vec<String>,
    pub burst_seed: u64,
}

/// How one kind of kill throws its drops: how long after the death, how far
/// apart, how wide, how high and how far, and how many coin pieces at most.
#[derive(Clone, Debug, Deserialize)]
pub struct Fountain {
    pub delay_secs: f32,
    pub stagger_secs: f32,
    pub fan_degrees: f32,
    pub up_speed: (f32, f32),
    pub out_speed: (f32, f32),
    pub spin: (f32, f32),
    pub most_pieces: usize,
}

impl Shower {
    pub fn load() -> Shower {
        ron::from_str(include_str!("../../assets/data/loot_shower.ron"))
            .unwrap_or_else(|e| panic!("assets/data/loot_shower.ron: {e}"))
    }

    pub fn fountain(&self, name: &str) -> &Fountain {
        self.fountains
            .get(name)
            .unwrap_or_else(|| panic!("loot_shower.ron: no fountain named `{name}`"))
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

/// Where a launch comes to rest and how long it flies, found by flying it.
fn flight(shower: &Shower, origin: Vec3, launch: &Launch) -> (Vec3, f32) {
    let mut body = launch.body(shower, origin);
    let mut steps = 0;
    // A minute of flight is far past any real one.
    while !body.resting && steps < 7200 {
        body.step(shower, 1.0 / 120.0);
        steps += 1;
    }
    (body.position, steps as f32 / 120.0)
}

/// Where a launch comes to rest.
pub fn rest_point(shower: &Shower, origin: Vec3, launch: &Launch) -> Vec3 {
    flight(shower, origin, launch).0
}

/// The coins each coin piece of a burst is worth: `coins` shared out over
/// `pieces` as evenly as whole coins allow, so they add up to `coins`.
pub fn coin_values(coins: i32, pieces: usize) -> Vec<i32> {
    let (coins, count) = (coins.max(0), pieces as i32);
    (0..count)
        .map(|i| coins / count + (i < coins % count) as i32)
        .collect()
}

/// When each piece of a burst leaves the ground for the party, in seconds
/// after the kill. The whole burst lies still for the pause, so it can be
/// read, and then the pieces go one after another, the nearest to the party
/// first.
pub fn plan_pickup(shower: &Shower, origin: Vec3, launches: &[Launch]) -> Vec<f32> {
    let flights: Vec<(Vec3, f32)> = launches
        .iter()
        .map(|launch| flight(shower, origin, launch))
        .collect();
    let at_rest = launches
        .iter()
        .zip(&flights)
        .map(|(launch, (_, secs))| launch.delay + secs)
        .fold(0.0, f32::max);
    let away = |i: usize| flights[i].0.xz().distance(nearest_horse(flights[i].0).xz());
    let mut order: Vec<usize> = (0..launches.len()).collect();
    order.sort_by(|&a, &b| away(a).total_cmp(&away(b)));
    let mut leaves = vec![0.0; launches.len()];
    for (turn, &i) in order.iter().enumerate() {
        leaves[i] = at_rest + shower.pickup_pause_secs + turn as f32 * shower.pickup_stagger_secs;
    }
    leaves
}

/// One step of a piece's way to the party: it gains speed all the way to
/// `to`. True when it has arrived.
pub fn step_home(shower: &Shower, position: &mut Vec3, speed: &mut f32, to: Vec3, dt: f32) -> bool {
    *speed += shower.pickup_accel * dt;
    let left = to - *position;
    let step = *speed * dt;
    if left.length() <= step {
        *position = to;
        return true;
    }
    *position += left.normalize() * step;
    false
}

fn is_clear(point: Vec3, standing: &[Vec3], keep_clear: f32) -> bool {
    standing
        .iter()
        .all(|enemy| point.xz().distance(enemy.xz()) > keep_clear)
}

/// Plan a burst of `count` pieces from `origin`, thrown the way `fountain`
/// says. Everything follows from the seed. Each piece gets its own slice of
/// the fan, so a burst is spread out however few pieces it has, and no piece
/// is sent to rest on an enemy still `standing`.
pub fn plan_burst(
    shower: &Shower,
    fountain: &Fountain,
    seed: u64,
    count: usize,
    origin: Vec3,
    standing: &[Vec3],
) -> Vec<Launch> {
    let mut rng = Rng::new(seed);
    let mut order: Vec<usize> = (0..count).collect();
    rng.shuffle(&mut order);
    let fan = fountain.fan_degrees.to_radians();
    let between = |rng: &mut Rng, (lo, hi): (f32, f32)| lo + (hi - lo) * rng.unit();

    (0..count)
        .map(|i| {
            let up = between(&mut rng, fountain.up_speed);
            let out = between(&mut rng, fountain.out_speed);
            let tumble = rng.unit() * TAU;
            let mut launch = Launch {
                delay: fountain.delay_secs + order[i] as f32 * fountain.stagger_secs,
                velocity: Vec3::ZERO,
                axis: Vec3::new(tumble.cos(), 0.0, tumble.sin()),
                spin: between(&mut rng, fountain.spin) * TAU,
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
    pub pickup: bool,
    pub burst_at: f32,
}

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Shower::load())
            .insert_resource(TestBurst {
                due: (self.burst || self.pickup).then_some(self.burst_at),
                pickup: self.pickup,
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
    /// Part of the test burst: the ground does not carry it off.
    pinned: bool,
    /// Seconds until it leaves the ground for the party. A test burst
    /// thrown to be looked at has none, and stays where it lands.
    pickup: Option<f32>,
    /// Its speed on its way to the party, once it has left the ground.
    homing: Option<f32>,
    /// The coins the counter gains when it arrives.
    value: i32,
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

/// What becomes of a burst once it has landed.
#[derive(Clone, Copy)]
enum After {
    /// A kill's drops: the party collects them, and they are worth this
    /// many coins in all.
    Kill(i32),
    /// The test burst: collected, or left to be looked at. Worth nothing.
    Test { pickup: bool },
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
    /// Seconds until `--burst` or `--pickup` throws it.
    due: Option<f32>,
    /// Whether the one thrown at launch is picked up afterwards.
    pickup: bool,
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
    after: After,
) {
    let (coins, pinned, collected) = match after {
        After::Kill(coins) => (coins, false, true),
        After::Test { pickup } => (0, true, pickup),
    };
    let coin_pieces = kinds.iter().filter(|&&kind| kind == Kind::Coin).count();
    let mut values = coin_values(coins, coin_pieces).into_iter();
    let pickups = plan_pickup(shower, origin, launches);
    for ((launch, kind), pickup) in launches.iter().zip(kinds).zip(pickups) {
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
            pickup: collected.then_some(pickup),
            homing: None,
            value: match kind {
                Kind::Coin => values.next().unwrap_or(0),
                Kind::Gear(_) => 0,
            },
            radius,
            thickness,
            model,
            beam,
        });
    }
}

/// A kill's drops leave its body in the fountain its enemy names: one piece
/// a coin up to the most that fountain shows, and its gear if it dropped any.
fn launch_kills(
    mut commands: Commands,
    mut killed: MessageReader<Killed>,
    look: Res<Look>,
    shower: Res<Shower>,
    mut bursts: Local<u64>,
) {
    for kill in killed.read() {
        let fountain = shower.fountain(&kill.fountain);
        let coins = (kill.coins.max(0) as usize).min(fountain.most_pieces);
        let gear: Vec<usize> = kill.gear.into_iter().collect();
        let kinds = burst_kinds(*bursts, coins, &gear);
        let launches = plan_burst(&shower, fountain, *bursts, kinds.len(), kill.at, &kill.standing);
        *bursts += 1;
        let after = After::Kill(kill.coins);
        spawn_burst(&mut commands, &look, &shower, kill.at, &launches, &kinds, after);
    }
}

/// F3, or `--burst` a moment after launch, throws the test burst from where
/// the first enemy stands, in place of the last one thrown. F4 and
/// `--pickup` throw the same burst and let the party collect it.
fn test_burst(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    look: Res<Look>,
    shower: Res<Shower>,
    mut test: ResMut<TestBurst>,
    pieces: Query<(Entity, &Piece)>,
) {
    let mut throw = if keys.just_pressed(KeyCode::F4) {
        Some(After::Test { pickup: true })
    } else if keys.just_pressed(KeyCode::F3) {
        Some(After::Test { pickup: false })
    } else {
        None
    };
    if let Some(due) = &mut test.due {
        *due -= time.delta_secs();
        if *due <= 0.0 {
            test.due = None;
            throw = Some(After::Test { pickup: test.pickup });
        }
    }
    let Some(after) = throw else {
        return;
    };
    for (entity, piece) in &pieces {
        if piece.pinned {
            commands.entity(entity).despawn();
        }
    }
    let origin = Vec3::new(ENEMY_X[0], 0.5, 0.0);
    let kinds = burst_kinds(shower.burst_seed, shower.burst_pieces, &look.burst_gear);
    let fountain = shower.fountain(&shower.burst_fountain);
    let launches = plan_burst(&shower, fountain, shower.burst_seed, kinds.len(), origin, &[]);
    spawn_burst(&mut commands, &look, &shower, origin, &launches, &kinds, after);
    test.watch = Some(Watch::default());
}

fn fly(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<Game>,
    shower: Res<Shower>,
    mut uncollected: ResMut<Uncollected>,
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
        if let Some(pickup) = &mut piece.pickup {
            *pickup -= dt;
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

        // Its turn has come: it leaves the ground for the nearest horse.
        let due = piece.pickup.is_some_and(|pickup| pickup <= 0.0);
        if piece.homing.is_none() && due && piece.settle >= 1.0 {
            piece.homing = Some(shower.pickup_speed);
        }
        let mut shrink = 1.0;
        if let Some(speed) = &mut piece.homing {
            let to = nearest_horse(piece.body.position);
            if step_home(&shower, &mut piece.body.position, speed, to, dt) {
                uncollected.coins -= piece.value;
                commands.entity(entity).despawn();
                continue;
            }
            let left = piece.body.position.distance(to);
            shrink = (left / shower.pickup_shrink.max(0.001)).clamp(0.2, 1.0);
        } else if !piece.pinned {
            // The ground moves under the party as it walks, and takes with
            // it what lies on it and what is falling to it.
            piece.body.position.x -= drift;
            if piece.body.position.x < -GONE {
                uncollected.coins -= piece.value;
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
        transform.scale =
            shrink * Vec3::new(1.0 + flattened / 2.0, 1.0 - flattened, 1.0 + flattened / 2.0);
        if let Ok(mut model) = models.get_mut(piece.model) {
            model.rotation = piece.tumble.slerp(piece.flat, lying);
        }
        // The beam stands on the ground where the piece is, upright and its
        // own size whatever the piece is doing: undo the piece's height and
        // squash. It sinks again as the piece leaves.
        if let Some(beam) = &mut piece.beam {
            let rise = dt / beam.def.rise_secs.max(0.001);
            let rise = if piece.homing.is_some() { -rise } else { rise };
            beam.risen = (beam.risen + rise).clamp(0.0, 1.0);
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
    use crate::game::scene::TRAIL_DEPTH;
    use crate::sim::data::Data;

    const ORIGIN: Vec3 = Vec3::new(ENEMY_X[0], 0.5, 0.0);

    /// The numbers hold together: a piece flies, comes down and stops.
    #[test]
    fn the_feel_numbers_are_usable() {
        let s = Shower::load();
        assert!(s.gravity > 0.0 && !s.fountains.is_empty());
        for (name, f) in &s.fountains {
            assert!(f.delay_secs >= 0.0 && f.stagger_secs >= 0.0, "{name}");
            for (lo, hi) in [f.up_speed, f.out_speed, f.spin] {
                assert!(0.0 <= lo && lo <= hi, "{name}");
            }
            assert!(f.up_speed.0 > 0.0 && f.most_pieces > 0, "{name}");
            assert!(0.0 < f.fan_degrees && f.fan_degrees <= 360.0, "{name}");
        }
        assert!((0.0..1.0).contains(&s.bounce) && (0.0..=1.0).contains(&s.grip));
        assert!((0.0..1.0).contains(&s.squash));
        assert!(0.0 < s.coin_thickness && s.coin_thickness <= 2.0 * s.coin_radius);
        assert!(s.burst_pieces > 0);
        assert!(s.pickup_pause_secs > 0.0 && s.pickup_stagger_secs > 0.0);
        assert!(s.pickup_speed >= 0.0 && s.pickup_accel > 0.0 && s.pickup_shrink > 0.0);
        let (length, thickness, depth) = s.gear_size;
        assert!(0.0 < thickness && thickness <= depth && depth <= length);
        // Enemies stand further apart than the room kept round each, or the
        // way out sideways would not be clear.
        assert!(s.keep_clear < ENEMY_X[1] - ENEMY_X[0]);
    }

    /// Every enemy names a fountain there is, and so does the test burst.
    #[test]
    fn every_kind_of_kill_has_its_fountain() {
        let (s, data) = (Shower::load(), Data::load());
        for enemy in &data.enemies {
            assert!(s.fountains.contains_key(&enemy.fountain), "{}: `{}`", enemy.id, enemy.fountain);
        }
        s.fountain(&s.burst_fountain);
    }

    /// An ordinary kill and a trail's last are different events: the last
    /// enemy's fountain waits longer, shows more pieces, takes longer to
    /// empty, opens wider, and its lowest arc tops an ordinary kill's highest.
    #[test]
    fn a_final_kill_is_a_bigger_event_than_an_ordinary_one() {
        let (s, data) = (Shower::load(), Data::load());
        let of = |ids: &Vec<Vec<String>>| -> Vec<&Fountain> {
            ids.iter()
                .flatten()
                .map(|id| s.fountain(&data.enemies[data.enemy_index(id)].fountain))
                .collect()
        };
        let top = |up: f32| up * up / (2.0 * s.gravity);
        for small in of(&data.encounters.ordinary) {
            for full in of(&data.encounters.finals) {
                assert!(full.delay_secs > small.delay_secs);
                assert!(full.most_pieces >= 2 * small.most_pieces);
                assert!(full.stagger_secs > small.stagger_secs);
                assert!(full.fan_degrees > small.fan_degrees);
                assert!(top(full.up_speed.0) > top(small.up_speed.1));
                assert!(full.out_speed.1 > small.out_speed.1);
            }
        }
        // The wait before the big one is a release, not a stall.
        for fountain in s.fountains.values() {
            assert!(fountain.delay_secs <= 0.6);
        }
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

    /// No two rarities share a colour, glow and size never fall going up the
    /// tiers, and the top tier has a beam. Rarity colours need not differ
    /// from the element colours: rarity shows on drops and in menus only.
    #[test]
    fn rarity_colours_are_told_apart() {
        let rarities = Data::load().gear.rarities;
        for (i, a) in rarities.iter().enumerate() {
            for b in &rarities[i + 1..] {
                let (x, y) = (rarity_colour(a), rarity_colour(b));
                let apart = (x.red - y.red).abs() + (x.green - y.green).abs() + (x.blue - y.blue).abs();
                assert!(apart >= 0.4, "{} and {} are {apart} apart", a.name, b.name);
            }
        }
        for pair in rarities.windows(2) {
            assert!(pair[1].look.glow >= pair[0].look.glow && pair[1].look.size >= pair[0].look.size);
        }
        assert!(rarities.last().unwrap().look.beam.is_some());
    }

    #[test]
    fn same_seed_same_burst() {
        let s = Shower::load();
        for f in s.fountains.values() {
            let a = plan_burst(&s, f, 7, 24, ORIGIN, &[]);
            assert_eq!(a, plan_burst(&s, f, 7, 24, ORIGIN, &[]));
            assert_ne!(a, plan_burst(&s, f, 8, 24, ORIGIN, &[]));
        }
    }

    /// The first piece leaves after the fountain's delay, no two leave
    /// together, and each leaves upwards within the speeds the data gives.
    #[test]
    fn launches_are_staggered_and_within_their_speeds() {
        let s = Shower::load();
        for (name, f) in &s.fountains {
            let burst = plan_burst(&s, f, 1, 40, ORIGIN, &[]);
            let mut delays: Vec<f32> = burst.iter().map(|l| l.delay).collect();
            delays.sort_by(f32::total_cmp);
            for (i, delay) in delays.iter().enumerate() {
                let due = f.delay_secs + i as f32 * f.stagger_secs;
                assert!((delay - due).abs() < 1e-5, "{name}");
            }
            for launch in &burst {
                let out = launch.velocity.xz().length();
                assert!((f.up_speed.0..=f.up_speed.1).contains(&launch.velocity.y), "{name}");
                assert!((f.out_speed.0 - 1e-4..=f.out_speed.1 + 1e-4).contains(&out), "{name}");
                assert!((launch.axis.length() - 1.0).abs() < 1e-4);
            }
        }
    }

    /// A burst fans out: it is not all thrown the same way.
    #[test]
    fn a_burst_is_spread_over_the_fan() {
        let s = Shower::load();
        for (name, f) in &s.fountains {
            let burst = plan_burst(&s, f, 2, 40, ORIGIN, &[]);
            let near = burst.iter().filter(|l| l.velocity.z > 0.0).count();
            assert!((10..=30).contains(&near), "{name}: {near} of 40 towards the camera");
            let towards_party = burst.iter().filter(|l| l.velocity.x < 0.0).count();
            assert!(towards_party > 20, "{name}");
        }
    }

    /// Every piece bounces as often as the data says, then rests on the
    /// ground, on the trail.
    #[test]
    fn every_piece_bounces_and_settles_on_the_trail() {
        let s = Shower::load();
        for (name, f) in &s.fountains {
            for seed in 0..50 {
                for launch in plan_burst(&s, f, seed, 40, ORIGIN, &[]) {
                    let mut body = launch.body(&s, ORIGIN);
                    let (mut touches, mut top) = (0, 0.0f32);
                    for _ in 0..1200 {
                        touches += body.step(&s, 1.0 / 120.0) as u32;
                        top = top.max(body.position.y);
                    }
                    assert!(body.resting, "{name}, seed {seed}: still moving after 10 s");
                    assert_eq!(touches, s.bounces + 1);
                    assert!(top > ORIGIN.y + 0.5, "{name}, seed {seed}: no arc");
                    assert_eq!(body.position.y, s.coin_radius);
                    let off = body.position.z.abs();
                    assert!(off < TRAIL_DEPTH / 2.0, "{name}, seed {seed}: off the trail");
                    assert_eq!(body.position, rest_point(&s, ORIGIN, &launch));
                }
            }
        }
    }

    /// A kill's coins are shared over its coin pieces: nothing is lost when
    /// there are more coins than pieces, and no piece is worth much more
    /// than another.
    #[test]
    fn coin_pieces_add_up_to_the_coins() {
        for coins in 0..200 {
            for pieces in 1..=20 {
                let values = coin_values(coins, pieces);
                assert_eq!(values.len(), pieces);
                assert_eq!(values.iter().sum::<i32>(), coins);
                let (least, most) = (values.iter().min().unwrap(), values.iter().max().unwrap());
                assert!(most - least <= 1 && *least >= 0);
            }
        }
        assert!(coin_values(5, 0).is_empty());
    }

    /// The whole burst lies still for the pause before the first piece
    /// leaves, and the pieces leave one at a time, the nearest to the
    /// party first.
    #[test]
    fn pickup_waits_then_takes_one_piece_at_a_time() {
        let s = Shower::load();
        for (f, seed) in s.fountains.values().flat_map(|f| (0..50).map(move |seed| (f, seed))) {
            let burst = plan_burst(&s, f, seed, 24, ORIGIN, &[]);
            let leaves = plan_pickup(&s, ORIGIN, &burst);
            let at_rest = burst
                .iter()
                .map(|launch| launch.delay + flight(&s, ORIGIN, launch).1)
                .fold(0.0, f32::max);
            let mut order: Vec<usize> = (0..burst.len()).collect();
            order.sort_by(|&a, &b| leaves[a].total_cmp(&leaves[b]));
            assert!((leaves[order[0]] - at_rest - s.pickup_pause_secs).abs() < 1e-4);
            let away = |i: usize| {
                let rest = rest_point(&s, ORIGIN, &burst[i]);
                rest.xz().distance(nearest_horse(rest).xz())
            };
            for pair in order.windows(2) {
                let gap = leaves[pair[1]] - leaves[pair[0]];
                assert!((gap - s.pickup_stagger_secs).abs() < 1e-4, "seed {seed}: {gap} apart");
                assert!(away(pair[0]) <= away(pair[1]), "seed {seed}: a further piece went first");
            }
        }
    }

    /// A piece on its way to the party only gains speed, and gets there
    /// soon, at any frame rate.
    #[test]
    fn a_collected_piece_speeds_up_and_arrives() {
        let s = Shower::load();
        for dt in [1.0 / 240.0, 1.0 / 60.0, 1.0 / 20.0] {
            let full = s.fountain(&s.burst_fountain);
            for launch in plan_burst(&s, full, 3, 40, ORIGIN, &[]) {
                let mut position = rest_point(&s, ORIGIN, &launch);
                let to = nearest_horse(position);
                let (mut speed, mut secs, mut arrived) = (s.pickup_speed, 0.0, false);
                let mut left = position.distance(to);
                while !arrived && secs < 3.0 {
                    let before = speed;
                    arrived = step_home(&s, &mut position, &mut speed, to, dt);
                    assert!(speed > before && position.distance(to) < left + 1e-4);
                    left = position.distance(to);
                    secs += dt;
                }
                assert!(arrived && position == to, "not home after {secs} s");
            }
        }
    }

    /// Drops do not land on enemies still alive, whichever of them died.
    #[test]
    fn no_piece_rests_on_a_standing_enemy() {
        let s = Shower::load();
        let places: Vec<Vec3> = ENEMY_X.iter().map(|&x| Vec3::new(x, 0.5, 0.0)).collect();
        for (f, seed) in s.fountains.values().flat_map(|f| (0..200).map(move |seed| (f, seed))) {
            for dead in 0..places.len() {
                let standing: Vec<Vec3> = (0..places.len())
                    .filter(|&i| i != dead)
                    .map(|i| places[i])
                    .collect();
                for launch in plan_burst(&s, f, seed, f.most_pieces, places[dead], &standing) {
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
