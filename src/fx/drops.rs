//! Drops as physical objects: a kill's drops leave the body one after
//! another, fanned out, each on its own arc with spin; they bounce, squash a
//! little and come to rest. Every number is in `assets/data/loot_shower.ron`.
//!
//! The flight is plain arithmetic (`Body`, `plan_burst`), so it is unit-tested;
//! the systems below it only draw it.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;
use serde::Deserialize;

use crate::game::menu::tokens::Hex;
use crate::game::scene::{ENEMY_X, SCENE_WIDTH, SCROLL_SPEED};
use crate::game::{Game, Killed};
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
    pub keep_clear: f32,
    pub burst_pieces: usize,
    pub burst_seed: u64,
}

impl Shower {
    pub fn load() -> Shower {
        ron::from_str(include_str!("../../assets/data/loot_shower.ron"))
            .unwrap_or_else(|e| panic!("assets/data/loot_shower.ron: {e}"))
    }
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
    model: Entity,
}

#[derive(Component)]
struct Model;

#[derive(Resource)]
struct Look {
    coin: Handle<Mesh>,
    gold: Handle<StandardMaterial>,
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
    shower: Res<Shower>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Look {
        coin: meshes.add(Cylinder::new(shower.coin_radius, shower.coin_thickness)),
        gold: materials.add(shower.coin_colour.0),
    });
}

fn spawn_burst(
    commands: &mut Commands,
    look: &Look,
    shower: &Shower,
    origin: Vec3,
    launches: &[Launch],
    pinned: bool,
) {
    for launch in launches {
        let mut model = Entity::PLACEHOLDER;
        let mut piece = commands.spawn((Transform::from_translation(origin), Visibility::Hidden));
        piece.with_children(|piece| {
            model = piece
                .spawn((
                    Model,
                    Mesh3d(look.coin.clone()),
                    MeshMaterial3d(look.gold.clone()),
                ))
                .id();
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
            model,
        });
    }
}

/// A kill's coins leave its body, one piece a coin up to the most shown.
fn launch_kills(
    mut commands: Commands,
    mut killed: MessageReader<Killed>,
    look: Res<Look>,
    shower: Res<Shower>,
    mut bursts: Local<u64>,
) {
    for kill in killed.read() {
        let count = (kill.coins.max(0) as usize).min(shower.most_pieces);
        let launches = plan_burst(&shower, *bursts, count, kill.at, &kill.standing);
        *bursts += 1;
        spawn_burst(&mut commands, &look, &shower, kill.at, &launches, false);
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
    let launches = plan_burst(&shower, shower.burst_seed, shower.burst_pieces, origin, &[]);
    spawn_burst(&mut commands, &look, &shower, origin, &launches, true);
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
        let height = shower.coin_radius
            + (shower.coin_thickness / 2.0 - shower.coin_radius) * lying;
        transform.translation = piece.body.position;
        transform.translation.y += height * (1.0 - flattened) - shower.coin_radius;
        transform.scale = Vec3::new(1.0 + flattened / 2.0, 1.0 - flattened, 1.0 + flattened / 2.0);
        if let Ok(mut model) = models.get_mut(piece.model) {
            model.rotation = piece.tumble.slerp(piece.flat, lying);
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
        // Enemies stand further apart than the room kept round each, or the
        // way out sideways would not be clear.
        assert!(s.keep_clear < ENEMY_X[1] - ENEMY_X[0]);
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
