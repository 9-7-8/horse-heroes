//! Death events: no enemy is taken out of the scene without one. A body that
//! is brought down is knocked back and tips over, breaks into shards that
//! are thrown out and fall, and leaves a mark on the ground for a while. The
//! break is heard, by the sound its death names (`fx::sound`). A trail's
//! last enemy names a bigger, slower death than an ordinary one.
//! Enemies still standing when the party is driven back draw off out of
//! view. Every number is in `assets/data/deaths.ron`.
//!
//! The fall, the shards and the mark are plain arithmetic (`fall_pose`,
//! `plan_shards`, `Shard::step`, `shard_scale`, `mark_scale`), so they are
//! unit-tested; the systems below them only draw them.

use std::collections::BTreeMap;
use std::f32::consts::TAU;

use bevy::prelude::*;
use serde::Deserialize;

use super::sound::Heard;
use crate::game::scene::{SCROLL_SPEED, element_colour};
use crate::game::{Game, Killed, Withdrew};
use crate::sim::data::Element;
use crate::sim::rng::Rng;

/// A shard, a mark or an enemy drawing off is forgotten once it is this far
/// from the party's middle, which is out of view in the widest window.
const GONE: f32 = 40.0;
/// How far above the ground a mark lies, so that it does not flicker in it.
const MARK_HEIGHT: f32 = 0.02;

/// The numbers in `assets/data/deaths.ron`, compiled into the binary.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Deaths {
    /// One for each kind of death, by the name an enemy gives in
    /// `enemies.ron`. A new kind is one more entry there and no code.
    pub deaths: BTreeMap<String, Death>,
    pub gravity: f32,
    pub mark_shade: f32,
    pub mark_grow_secs: f32,
    pub withdraw_speed: f32,
    pub withdraw_accel: f32,
}

/// How one kind of enemy dies: how long it falls, how far it is thrown and
/// tips, how many shards it breaks into and how they fly, and its mark.
#[derive(Clone, Debug, Deserialize)]
pub struct Death {
    pub fall_secs: f32,
    pub knock_back: f32,
    pub tip_degrees: f32,
    pub swell: f32,
    pub per_side: u32,
    pub out_speed: (f32, f32),
    pub up_speed: (f32, f32),
    pub back_speed: f32,
    pub spin: (f32, f32),
    pub linger_secs: f32,
    pub fade_secs: f32,
    pub mark_radius: f32,
    pub mark_secs: f32,
    pub mark_fade_secs: f32,
    /// Heard as the body breaks, by its name in `sounds.ron`.
    pub sound: String,
}

impl Deaths {
    pub fn load() -> Deaths {
        ron::from_str(include_str!("../../assets/data/deaths.ron"))
            .unwrap_or_else(|e| panic!("assets/data/deaths.ron: {e}"))
    }

    pub fn death(&self, name: &str) -> &Death {
        self.deaths
            .get(name)
            .unwrap_or_else(|| panic!("deaths.ron: no death named `{name}`"))
    }
}

impl Death {
    pub fn shards(&self) -> usize {
        self.per_side.pow(3) as usize
    }
}

/// How a falling body is held: how far back it has been thrown, how far over
/// it has tipped, in radians, and how many times its size it is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub back: f32,
    pub tip: f32,
    pub scale: f32,
}

/// A body `age` seconds after the blow. It is thrown and tips fast at first
/// and slows, and swells slowly at first and then fast, up to the break.
pub fn fall_pose(death: &Death, age: f32) -> Pose {
    let t = if death.fall_secs > 0.0 {
        (age / death.fall_secs).clamp(0.0, 1.0)
    } else {
        1.0
    };
    let thrown = 1.0 - (1.0 - t) * (1.0 - t);
    Pose {
        back: death.knock_back * thrown,
        tip: death.tip_degrees.to_radians() * thrown,
        scale: 1.0 + (death.swell - 1.0) * t * t,
    }
}

/// Where a body of side `size` that stood at `centre` is in `pose`. It tips
/// away from the party and stays standing on the ground as it swells.
pub fn body_transform(centre: Vec3, size: f32, pose: Pose) -> Transform {
    let side = size * pose.scale;
    Transform::from_xyz(centre.x + pose.back, centre.y + (side - size) / 2.0, centre.z)
        .with_rotation(Quat::from_rotation_z(-pose.tip))
        .with_scale(Vec3::splat(side))
}

/// One piece of a broken body. Its position is its middle.
#[derive(Clone, Debug, PartialEq)]
pub struct Shard {
    pub position: Vec3,
    pub velocity: Vec3,
    /// The axis it tumbles about, and how fast, in radians a second.
    pub axis: Vec3,
    pub spin: f32,
    pub resting: bool,
}

impl Shard {
    /// Move on by `dt`. A shard of half side `half` stops where it first
    /// touches the ground.
    pub fn step(&mut self, gravity: f32, half: f32, dt: f32) {
        if self.resting {
            return;
        }
        self.velocity.y -= gravity * dt;
        self.position += self.velocity * dt;
        if self.position.y <= half && self.velocity.y < 0.0 {
            self.position.y = half;
            self.velocity = Vec3::ZERO;
            self.resting = true;
        }
    }
}

/// The side of each shard of a body of side `size`.
pub fn shard_side(death: &Death, size: f32) -> f32 {
    size * death.swell / death.per_side.max(1) as f32
}

/// Break a body of side `size` that stood at `centre`: the shards are the
/// body as it was at the end of its fall, cut `per_side` ways along each
/// edge, each thrown out from the middle, up, and away from the party.
/// Everything follows from the seed.
pub fn plan_shards(death: &Death, seed: u64, centre: Vec3, size: f32) -> Vec<Shard> {
    let mut rng = Rng::new(seed ^ 0xDEAD_5A4D_DEAD_5A4D);
    let between = |rng: &mut Rng, (lo, hi): (f32, f32)| lo + (hi - lo) * rng.unit();
    let body = body_transform(centre, size, fall_pose(death, death.fall_secs));
    let n = death.per_side.max(1);
    let mut shards = Vec::with_capacity(death.shards());
    for i in 0..n.pow(3) {
        // Its place in the body, from -0.5 to 0.5 along each edge.
        let cell = Vec3::new((i % n) as f32, (i / n % n) as f32, (i / (n * n)) as f32);
        let place = (cell + 0.5) / n as f32 - 0.5;
        let position = body.transform_point(place);
        let tumble = rng.unit() * TAU;
        let lift = rng.unit();
        // The one in the very middle has no way out of its own; it goes up.
        let out = (body.rotation * place).try_normalize().unwrap_or(Vec3::Y);
        let out = Vec3::new(out.x, out.y.max(0.0), out.z) * between(&mut rng, death.out_speed);
        let up = death.up_speed.0 + (death.up_speed.1 - death.up_speed.0) * lift;
        shards.push(Shard {
            position,
            velocity: out + Vec3::new(death.back_speed, up, 0.0),
            axis: Vec3::new(tumble.cos(), 0.0, tumble.sin()),
            spin: between(&mut rng, death.spin) * TAU,
            resting: false,
        });
    }
    shards
}

/// How much of its size a shard is `age` seconds after the break: all of it
/// while it lingers, then shrinking to nothing.
pub fn shard_scale(death: &Death, age: f32) -> f32 {
    if age >= death.linger_secs + death.fade_secs {
        return 0.0;
    }
    if death.fade_secs <= 0.0 {
        return 1.0;
    }
    (1.0 - (age - death.linger_secs) / death.fade_secs).clamp(0.0, 1.0)
}

/// How much of its size a mark is `age` seconds after the break: it comes up
/// at once, stays, and shrinks away over the last of its time.
pub fn mark_scale(deaths: &Deaths, death: &Death, age: f32) -> f32 {
    if age < 0.0 || age >= death.mark_secs {
        return 0.0;
    }
    let grown = if deaths.mark_grow_secs > 0.0 {
        (age / deaths.mark_grow_secs).min(1.0)
    } else {
        1.0
    };
    let left = if death.mark_fade_secs > 0.0 {
        ((death.mark_secs - age) / death.mark_fade_secs).min(1.0)
    } else {
        1.0
    };
    grown * left
}

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Deaths::load())
            .add_systems(Startup, setup)
            // After the kills are reported, so a body's death begins in the
            // frame the scene stops drawing it.
            .add_systems(Update, (begin, play).chain().after(crate::game::advance));
    }
}

/// A body falling: thrown back and tipping, until it breaks.
#[derive(Component)]
struct Falling {
    death: String,
    element: Element,
    centre: Vec3,
    size: f32,
    age: f32,
    seed: u64,
}

/// A piece of a broken body, lying or in the air.
#[derive(Component)]
struct Debris {
    death: String,
    shard: Shard,
    side: f32,
    tumble: Quat,
    age: f32,
}

/// The mark a death leaves on the ground.
#[derive(Component)]
struct Mark {
    death: String,
    radius: f32,
    age: f32,
}

/// An enemy the party was driven back from, drawing off.
#[derive(Component)]
struct Withdrawing {
    speed: f32,
}

#[derive(Resource)]
struct Look {
    cube: Handle<Mesh>,
    /// A mark one unit in radius.
    disc: Handle<Mesh>,
    fire: Shades,
    ice: Shades,
    plant: Shades,
}

/// An element's two materials: a body's, and a mark's.
struct Shades {
    body: Handle<StandardMaterial>,
    mark: Handle<StandardMaterial>,
}

impl Look {
    fn of(&self, element: Element) -> &Shades {
        match element {
            Element::Fire => &self.fire,
            Element::Ice => &self.ice,
            Element::Plant => &self.plant,
        }
    }
}

fn setup(
    mut commands: Commands,
    deaths: Res<Deaths>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut shades = |element: Element| {
        let colour = element_colour(element);
        let dark = colour.to_linear() * deaths.mark_shade;
        Shades {
            body: materials.add(colour),
            mark: materials.add(Color::from(dark.with_alpha(1.0))),
        }
    };
    let (fire, ice, plant) = (
        shades(Element::Fire),
        shades(Element::Ice),
        shades(Element::Plant),
    );
    commands.insert_resource(Look {
        cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        disc: meshes.add(Cylinder::new(1.0, MARK_HEIGHT)),
        fire,
        ice,
        plant,
    });
}

/// A kill's body starts to fall where the scene has just stopped drawing it,
/// and an enemy left standing by a party driven back starts to draw off.
fn begin(
    mut commands: Commands,
    mut killed: MessageReader<Killed>,
    mut withdrew: MessageReader<Withdrew>,
    deaths: Res<Deaths>,
    look: Res<Look>,
    mut count: Local<u64>,
) {
    for kill in killed.read() {
        let death = deaths.death(&kill.death);
        commands.spawn((
            Falling {
                death: kill.death.clone(),
                element: kill.element,
                centre: kill.at,
                size: kill.size,
                age: 0.0,
                seed: *count,
            },
            Mesh3d(look.cube.clone()),
            MeshMaterial3d(look.of(kill.element).body.clone()),
            body_transform(kill.at, kill.size, fall_pose(death, 0.0)),
        ));
        *count += 1;
    }
    for enemy in withdrew.read() {
        commands.spawn((
            Withdrawing {
                speed: deaths.withdraw_speed,
            },
            Mesh3d(look.cube.clone()),
            MeshMaterial3d(look.of(enemy.element).body.clone()),
            Transform::from_translation(enemy.at).with_scale(Vec3::splat(enemy.size)),
        ));
    }
}

fn play(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<Game>,
    deaths: Res<Deaths>,
    look: Res<Look>,
    mut falling: Query<(Entity, &mut Falling, &mut Transform)>,
    mut debris: Query<(Entity, &mut Debris, &mut Transform), Without<Falling>>,
    mut marks: Query<(Entity, &mut Mark, &mut Transform), (Without<Falling>, Without<Debris>)>,
    mut withdrawing: Query<
        (Entity, &mut Withdrawing, &mut Transform),
        (Without<Falling>, Without<Debris>, Without<Mark>),
    >,
    mut heard: MessageWriter<Heard>,
) {
    let dt = time.delta_secs();
    // The ground moves under the party as it walks, and takes with it what
    // lies on it and what is falling to it.
    let drift = if game.party_moving() { SCROLL_SPEED * dt } else { 0.0 };

    for (entity, mut body, mut transform) in &mut falling {
        let death = deaths.death(&body.death);
        body.age += dt;
        body.centre.x -= drift;
        if body.age < death.fall_secs {
            *transform = body_transform(body.centre, body.size, fall_pose(death, body.age));
            continue;
        }
        // It breaks: the body goes, and its shards and its mark take its place.
        commands.entity(entity).despawn();
        heard.write(Heard::Died(death.sound.clone()));
        let shades = look.of(body.element);
        let side = shard_side(death, body.size);
        for shard in plan_shards(death, body.seed, body.centre, body.size) {
            commands.spawn((
                Mesh3d(look.cube.clone()),
                MeshMaterial3d(shades.body.clone()),
                Transform::from_translation(shard.position).with_scale(Vec3::splat(side)),
                Debris {
                    death: body.death.clone(),
                    shard,
                    side,
                    tumble: Quat::from_rotation_z(-death.tip_degrees.to_radians()),
                    age: 0.0,
                },
            ));
        }
        let pose = fall_pose(death, death.fall_secs);
        commands.spawn((
            Mesh3d(look.disc.clone()),
            MeshMaterial3d(shades.mark.clone()),
            Transform::from_xyz(body.centre.x + pose.back, MARK_HEIGHT / 2.0, body.centre.z)
                .with_scale(Vec3::new(0.0, 1.0, 0.0)),
            Mark {
                death: body.death.clone(),
                radius: death.mark_radius * body.size,
                age: 0.0,
            },
        ));
    }

    for (entity, mut piece, mut transform) in &mut debris {
        let piece = &mut *piece;
        let death = deaths.death(&piece.death);
        piece.age += dt;
        let scale = shard_scale(death, piece.age);
        piece.shard.position.x -= drift;
        if scale <= 0.0 || piece.shard.position.x < -GONE {
            commands.entity(entity).despawn();
            continue;
        }
        if !piece.shard.resting {
            let turn = Quat::from_axis_angle(piece.shard.axis, piece.shard.spin * dt);
            piece.tumble = turn * piece.tumble;
        }
        piece.shard.step(deaths.gravity, piece.side / 2.0, dt);
        transform.translation = piece.shard.position;
        transform.rotation = piece.tumble;
        transform.scale = Vec3::splat(piece.side * scale);
    }

    for (entity, mut mark, mut transform) in &mut marks {
        let death = deaths.death(&mark.death);
        mark.age += dt;
        transform.translation.x -= drift;
        if mark.age >= death.mark_secs || transform.translation.x < -GONE {
            commands.entity(entity).despawn();
            continue;
        }
        let radius = mark.radius * mark_scale(&deaths, death, mark.age);
        transform.scale = Vec3::new(radius, 1.0, radius);
    }

    for (entity, mut enemy, mut transform) in &mut withdrawing {
        enemy.speed += deaths.withdraw_accel * dt;
        transform.translation.x += enemy.speed * dt;
        if transform.translation.x > GONE {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::drops::Shower;
    use crate::game::scene::{ENEMY_X, TRAIL_DEPTH, enemy_centre, enemy_size};
    use crate::sim::data::Data;

    /// How long a shard is in the air, found by flying it.
    fn flight_secs(deaths: &Deaths, shard: &Shard, half: f32) -> f32 {
        let mut shard = shard.clone();
        let mut steps = 0;
        while !shard.resting && steps < 7200 {
            shard.step(deaths.gravity, half, 1.0 / 120.0);
            steps += 1;
        }
        assert!(shard.resting, "a shard never came down");
        steps as f32 / 120.0
    }

    /// The numbers hold together: a body falls, breaks, and all of it goes.
    #[test]
    fn the_death_numbers_are_usable() {
        let deaths = Deaths::load();
        assert!(deaths.gravity > 0.0 && !deaths.deaths.is_empty());
        assert!((0.0..=1.0).contains(&deaths.mark_shade) && deaths.mark_grow_secs >= 0.0);
        assert!(deaths.withdraw_speed >= 0.0 && deaths.withdraw_accel > 0.0);
        for (name, d) in &deaths.deaths {
            assert!(d.fall_secs >= 0.0 && d.knock_back >= 0.0, "{name}");
            assert!((0.0..90.0).contains(&d.tip_degrees) && d.swell >= 1.0, "{name}");
            assert!((1..=4).contains(&d.per_side), "{name}");
            for (lo, hi) in [d.out_speed, d.up_speed, d.spin] {
                assert!(0.0 <= lo && lo <= hi, "{name}");
            }
            assert!(d.up_speed.0 > 0.0 && d.back_speed >= 0.0, "{name}");
            assert!(d.linger_secs > 0.0 && d.fade_secs > 0.0, "{name}");
            assert!(d.mark_radius > 0.0, "{name}");
            // The mark has time to come up before it starts to go.
            assert!(d.mark_fade_secs > 0.0, "{name}");
            assert!(deaths.mark_grow_secs + d.mark_fade_secs <= d.mark_secs, "{name}");
        }
    }

    /// Every enemy names a death there is.
    #[test]
    fn every_enemy_has_its_death() {
        let (deaths, data) = (Deaths::load(), Data::load());
        for enemy in &data.enemies {
            assert!(deaths.deaths.contains_key(&enemy.death), "{}: `{}`", enemy.id, enemy.death);
        }
    }

    /// A falling body only goes one way: back, over and bigger, never past
    /// what its death says, and it starts as the body the scene drew.
    #[test]
    fn a_body_is_thrown_back_and_tips_until_it_breaks() {
        let deaths = Deaths::load();
        for (name, death) in &deaths.deaths {
            let start = fall_pose(death, 0.0);
            assert_eq!((start.back, start.tip, start.scale), (0.0, 0.0, 1.0), "{name}");
            let centre = enemy_centre(0, 60);
            let size = enemy_size(60);
            let drawn = body_transform(centre, size, start);
            assert_eq!(drawn.translation, centre, "{name}");
            assert_eq!(drawn.scale, Vec3::splat(size), "{name}");

            let mut last = start;
            for step in 1..=60 {
                let pose = fall_pose(death, death.fall_secs * step as f32 / 60.0);
                assert!(pose.back >= last.back && pose.tip >= last.tip, "{name}");
                assert!(pose.scale >= last.scale, "{name}");
                // It stays standing on the ground as it swells.
                let body = body_transform(centre, size, pose);
                assert!((body.translation.y - body.scale.y / 2.0).abs() < 1e-4, "{name}");
                last = pose;
            }
            let end = fall_pose(death, death.fall_secs);
            assert!((end.back - death.knock_back).abs() < 1e-5, "{name}");
            assert!((end.scale - death.swell).abs() < 1e-5, "{name}");
            assert_eq!(fall_pose(death, death.fall_secs + 5.0), end, "{name}");
        }
    }

    /// A body breaks into the shards its death says, which start inside it,
    /// together fill it, leave upward and away from the party, and the same
    /// seed breaks it the same way.
    #[test]
    fn a_body_breaks_into_its_shards() {
        let (deaths, data) = (Deaths::load(), Data::load());
        for enemy in &data.enemies {
            let death = deaths.death(&enemy.death);
            let (centre, size) = (enemy_centre(0, enemy.health), enemy_size(enemy.health));
            let shards = plan_shards(death, 7, centre, size);
            assert_eq!(shards.len(), death.shards(), "{}", enemy.id);
            assert_eq!(shards, plan_shards(death, 7, centre, size), "{}", enemy.id);
            assert_ne!(shards, plan_shards(death, 8, centre, size), "{}", enemy.id);

            let side = shard_side(death, size);
            let volume = side.powi(3) * shards.len() as f32;
            let body = (size * death.swell).powi(3);
            assert!((volume - body).abs() < 1e-3 * body, "{}", enemy.id);
            let middle = centre.x + death.knock_back;
            for shard in &shards {
                assert!(shard.position.distance(Vec3::new(middle, centre.y, centre.z)) <= size * death.swell, "{}", enemy.id);
                assert!(shard.velocity.y >= death.up_speed.0, "{}", enemy.id);
                assert!(shard.spin >= 0.0 && shard.axis.is_normalized(), "{}", enemy.id);
            }
            // Thrown away from the party on the whole, not into it.
            let mean = shards.iter().map(|s| s.velocity.x).sum::<f32>() / shards.len() as f32;
            assert!(mean > 0.0, "{}: {mean}", enemy.id);
        }
    }

    /// Every shard comes down on the trail, stops, and has stopped before it
    /// starts to shrink away: nothing disappears in the air. None lands among
    /// the party.
    #[test]
    fn every_shard_lands_before_it_goes() {
        let (deaths, data) = (Deaths::load(), Data::load());
        for enemy in &data.enemies {
            let death = deaths.death(&enemy.death);
            let size = enemy_size(enemy.health);
            let half = shard_side(death, size) / 2.0;
            for slot in 0..ENEMY_X.len() {
                for seed in 0..40 {
                    for shard in plan_shards(death, seed, enemy_centre(slot, enemy.health), size) {
                        let secs = flight_secs(&deaths, &shard, half);
                        assert!(secs < death.linger_secs, "{}: {secs} s in the air", enemy.id);
                        let mut landed = shard.clone();
                        while !landed.resting {
                            landed.step(deaths.gravity, half, 1.0 / 120.0);
                        }
                        assert_eq!(landed.position.y, half, "{}", enemy.id);
                        assert!(landed.position.z.abs() < TRAIL_DEPTH / 2.0 + 1.0, "{}", enemy.id);
                        assert!(landed.position.x > 0.0, "{}: {}", enemy.id, landed.position.x);
                    }
                }
            }
        }
    }

    /// A shard is whole while it lingers and gone when its death says; a
    /// mark comes up, stays and is gone when its death says, and outlasts
    /// the shards.
    #[test]
    fn shards_and_marks_go_in_their_time() {
        let deaths = Deaths::load();
        for (name, death) in &deaths.deaths {
            assert_eq!(shard_scale(death, 0.0), 1.0, "{name}");
            assert_eq!(shard_scale(death, death.linger_secs), 1.0, "{name}");
            assert_eq!(shard_scale(death, death.linger_secs + death.fade_secs), 0.0, "{name}");
            let half = shard_scale(death, death.linger_secs + death.fade_secs / 2.0);
            assert!((half - 0.5).abs() < 1e-4, "{name}");

            assert_eq!(mark_scale(&deaths, death, 0.0), 0.0, "{name}");
            assert_eq!(mark_scale(&deaths, death, death.mark_secs), 0.0, "{name}");
            let held = (deaths.mark_grow_secs + death.mark_secs - death.mark_fade_secs) / 2.0;
            assert_eq!(mark_scale(&deaths, death, held), 1.0, "{name}");
            for step in 0..=200 {
                let scale = mark_scale(&deaths, death, death.mark_secs * step as f32 / 200.0);
                assert!((0.0..=1.0).contains(&scale), "{name}");
            }
            assert!(death.mark_secs > death.linger_secs + death.fade_secs, "{name}");
        }
    }

    /// A trail's last enemy dies a bigger, slower death than an ordinary
    /// one: it falls for longer, breaks into more pieces that go higher and
    /// lie longer, and its mark is wider and stays longer. And the whole of
    /// any death is over before the next enemies can stand where it was.
    #[test]
    fn a_final_death_is_bigger_and_slower_than_an_ordinary_one() {
        let (deaths, data) = (Deaths::load(), Data::load());
        let of = |ids: &Vec<Vec<String>>| -> Vec<(&Death, f32)> {
            ids.iter()
                .flatten()
                .map(|id| {
                    let enemy = &data.enemies[data.enemy_index(id)];
                    (deaths.death(&enemy.death), enemy_size(enemy.health))
                })
                .collect()
        };
        let top = |up: f32| up * up / (2.0 * deaths.gravity);
        // From the blow to the last shard being gone.
        let body_secs = |death: &Death| death.fall_secs + death.linger_secs + death.fade_secs;
        for (small, small_size) in of(&data.encounters.ordinary) {
            for (big, big_size) in of(&data.encounters.finals) {
                assert!(big.fall_secs > small.fall_secs);
                assert!(big.shards() > small.shards());
                assert!(big.swell > small.swell);
                assert!(top(big.up_speed.0) > top(small.up_speed.1));
                assert!(big.linger_secs > small.linger_secs);
                assert!(body_secs(big) > body_secs(small));
                assert!(big.mark_secs > small.mark_secs);
                assert!(big.mark_radius * big_size > small.mark_radius * small_size);
            }
        }
        for death in deaths.deaths.values() {
            assert!(body_secs(death) < data.rules.travel_secs);
        }
    }

    /// The drops come out of the death: a body has broken, or all but, by
    /// the time its enemy's fountain throws its first piece.
    #[test]
    fn a_fountain_erupts_from_the_break() {
        let (deaths, shower, data) = (Deaths::load(), Shower::load(), Data::load());
        for enemy in &data.enemies {
            let death = deaths.death(&enemy.death);
            let fountain = shower.fountain(&enemy.fountain);
            assert!(death.fall_secs <= fountain.delay_secs + 0.1, "{}", enemy.id);
            // And the fountain's place is still inside the body thrown back.
            let size = enemy_size(enemy.health);
            assert!(death.knock_back < size * death.swell / 2.0, "{}", enemy.id);
        }
    }
}
