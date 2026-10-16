//! Hit reactions (#16): whatever is struck sheds sparks off the face that
//! was struck, back toward what struck it, while the body recoils the other
//! way; and its health bar drops at once while a trailing piece
//! holds where it was and then follows it down. More sparks, faster and
//! bigger, for a strong hit; a few small grey ones for a resisted hit. The
//! flash and the flinch are the body's own and are drawn with it
//! (`game::scene`). Every number is in `assets/data/impacts.ron`.
//!
//! The sparks and the trailing piece are plain arithmetic (`plan_sparks`,
//! `Spark::step`, `trail_after`), so they are unit-tested; the systems below
//! them only draw them. How it works: docs/hit-reactions.md.

use std::collections::HashMap;

use bevy::color::Mix;
use bevy::prelude::*;
use serde::Deserialize;

use super::numbers::{self, Feel};
use crate::game::menu::tokens::{Hex, Tokens};
use crate::game::scene::{ENEMY_X, HORSE_PLACES, enemy_centre, enemy_size, horse_centre};
use crate::game::{Game, Landed, Screen};
use crate::sim::expedition::{HitKind, Struck};
use crate::sim::rng::Rng;

/// The numbers in `impacts.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Impacts {
    pub sparks: Kinds,
    pub lift: f32,
    pub gravity: f32,
    pub spark_secs: f32,
    pub resisted_grey: f32,
    pub horse_chest: f32,
    pub bar_width: f32,
    pub bar_height: f32,
    pub horse_above: f32,
    pub enemy_above: f32,
    pub hold_secs: f32,
    pub drain: f32,
    pub back: Hex,
    pub trail: Hex,
    pub foe: Hex,
    pub horse: Hex,
}

/// The sparks of each kind of hit.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Kinds {
    pub strong: Burst,
    pub plain: Burst,
    pub resisted: Burst,
}

impl Kinds {
    /// A warded hit throws a resisted one's sparks.
    pub fn of(&self, kind: HitKind) -> Burst {
        match kind {
            HitKind::Strong => self.strong,
            HitKind::Plain => self.plain,
            HitKind::Resisted | HitKind::Warded => self.resisted,
        }
    }
}

/// How many sparks a hit throws, how fast, and how big.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Burst {
    pub count: usize,
    pub speed: (f32, f32),
    pub size: f32,
}

impl Impacts {
    pub fn load() -> Impacts {
        ron::from_str(include_str!("../../assets/data/impacts.ron"))
            .unwrap_or_else(|e| panic!("assets/data/impacts.ron: {e}"))
    }
}

/// One spark in the air.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spark {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Spark {
    pub fn step(&mut self, gravity: f32, dt: f32) {
        self.velocity.y -= gravity * dt;
        self.position += self.velocity * dt;
    }
}

/// The sparks a hit throws from `at`, on the face it struck: out along
/// `out` (+1 or -1 on x, off that face, toward what struck it), up by as
/// much as `lift` of each one's flight, a little toward or away from the
/// camera. Everything follows from the seed.
pub fn plan_sparks(impacts: &Impacts, burst: Burst, seed: u64, at: Vec3, out: f32) -> Vec<Spark> {
    let mut rng = Rng::new(seed ^ 0x5DA2_4C11_5DA2_4C11);
    (0..burst.count)
        .map(|_| {
            let up = impacts.lift * rng.unit();
            let across = (rng.unit() - 0.5) * 0.8;
            let way = Vec3::new(out * (1.0 - up), up, across).normalize();
            let speed = burst.speed.0 + (burst.speed.1 - burst.speed.0) * rng.unit();
            Spark { position: at, velocity: way * speed }
        })
        .collect()
}

/// How much of its size a spark is `age` seconds after it was thrown: all of
/// it, shrinking to nothing at `spark_secs`.
pub fn spark_scale(impacts: &Impacts, age: f32) -> f32 {
    (1.0 - age / impacts.spark_secs).clamp(0.0, 1.0)
}

/// Where a bar's trailing piece is, as a share of the bar, `dt` seconds on:
/// never below the health, `fill`; held where it was until `since` (the
/// seconds since the health last dropped) passes `hold_secs`; then falling
/// at `drain` of a bar a second.
pub fn trail_after(impacts: &Impacts, trail: f32, fill: f32, since: f32, dt: f32) -> f32 {
    if trail <= fill {
        fill
    } else if since < impacts.hold_secs {
        trail
    } else {
        (trail - impacts.drain * dt).max(fill)
    }
}

pub struct ImpactPlugin;

impl Plugin for ImpactPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Impacts::load())
            .init_resource::<SparkLooks>()
            .add_systems(Startup, spawn_bars)
            .add_systems(Update, (throw, fly, bars).after(crate::game::advance));
    }
}

/// A spark's flight and how long it has been in the air.
#[derive(Component)]
struct Flying {
    spark: Spark,
    size: f32,
    age: f32,
}

/// The cube every spark is, and a material for each colour thrown so far.
#[derive(Resource, Default)]
struct SparkLooks {
    cube: Option<Handle<Mesh>>,
    made: HashMap<[u8; 4], Handle<StandardMaterial>>,
}

/// Each hit that lands throws its sparks.
#[allow(clippy::too_many_arguments)]
fn throw(
    mut commands: Commands,
    impacts: Res<Impacts>,
    feel: Res<Feel>,
    tokens: Res<Tokens>,
    mut looks: ResMut<SparkLooks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut landed: MessageReader<Landed>,
    mut count: Local<u64>,
) {
    for Landed { hit, at } in landed.read() {
        let burst = impacts.sparks.of(hit.kind);
        let mut colour = numbers::colour(&feel, &tokens, hit);
        if matches!(hit.kind, HitKind::Resisted | HitKind::Warded) {
            colour = colour.mix(&Color::srgb(0.5, 0.5, 0.5), impacts.resisted_grey);
        }
        let cube = looks.cube.get_or_insert_with(|| meshes.add(Cuboid::new(1.0, 1.0, 1.0))).clone();
        let material = looks
            .made
            .entry(colour.to_srgba().to_u8_array())
            .or_insert_with(|| {
                materials.add(StandardMaterial {
                    base_color: colour,
                    unlit: true,
                    ..default()
                })
            })
            .clone();
        // Horses stand to the left of their foes, so a foe is struck on its
        // left face and a horse on its chest. The sparks start there, on
        // the camera's side of the middle so the body does not hide them.
        let (out, reach, near) = match hit.struck {
            Struck::Foe { max_health, .. } => (-1.0, enemy_size(max_health) / 2.0, enemy_size(max_health) / 2.0),
            Struck::Horse(_) => (1.0, impacts.horse_chest, 0.3),
        };
        let from = *at + Vec3::new(out * reach, 0.0, near);
        for spark in plan_sparks(&impacts, burst, *count, from, out) {
            commands.spawn((
                Flying { spark, size: burst.size, age: 0.0 },
                Mesh3d(cube.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(spark.position).with_scale(Vec3::splat(burst.size)),
            ));
        }
        *count += 1;
    }
}

fn fly(
    mut commands: Commands,
    time: Res<Time>,
    impacts: Res<Impacts>,
    mut sparks: Query<(Entity, &mut Flying, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut flying, mut transform) in &mut sparks {
        flying.age += dt;
        let scale = spark_scale(&impacts, flying.age);
        if scale <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        flying.spark.step(impacts.gravity, dt);
        transform.translation = flying.spark.position;
        transform.scale = Vec3::splat(flying.size * scale);
    }
}

/// Whose health a bar shows.
#[derive(Clone, Copy, PartialEq)]
enum Whose {
    Horse(usize),
    Foe(usize),
}

/// A health bar: the share of health it shows, where its trailing piece is,
/// seconds since the health last dropped, and the full health it is a share
/// of, so a new enemy in its place starts a new bar.
#[derive(Component)]
struct Bar {
    whose: Whose,
    fill: f32,
    trail: f32,
    since: f32,
    max: i32,
    fill_node: Entity,
    trail_node: Entity,
}

fn spawn_bars(mut commands: Commands, impacts: Res<Impacts>) {
    let whose = (0..HORSE_PLACES).map(Whose::Horse).chain((0..ENEMY_X.len()).map(Whose::Foe));
    for whose in whose {
        let piece = |colour: Color| {
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(colour),
                Pickable::IGNORE,
            )
        };
        let fill_colour = match whose {
            Whose::Horse(_) => impacts.horse.0,
            Whose::Foe(_) => impacts.foe.0,
        };
        let trail_node = commands.spawn(piece(impacts.trail.0)).id();
        let fill_node = commands.spawn(piece(fill_colour)).id();
        commands
            .spawn((
                Bar { whose, fill: 1.0, trail: 1.0, since: f32::INFINITY, max: 0, fill_node, trail_node },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(impacts.bar_width),
                    height: Val::Px(impacts.bar_height),
                    ..default()
                },
                BackgroundColor(impacts.back.0),
                // Under the menus, so a bar never covers a control.
                GlobalZIndex(-1),
                Visibility::Hidden,
                Pickable::IGNORE,
            ))
            .add_children(&[trail_node, fill_node]);
    }
}

/// Each bar follows its body's health, over it, on a trail; hidden in town
/// and over a place with nobody standing in it.
fn bars(
    time: Res<Time>,
    game: Res<Game>,
    impacts: Res<Impacts>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut bars: Query<(&mut Bar, &mut Node, &mut Visibility)>,
    mut pieces: Query<&mut Node, Without<Bar>>,
) {
    let dt = time.delta_secs();
    let (camera, view) = *camera;
    for (mut bar, mut node, mut visibility) in &mut bars {
        // Its health, full health, and where over the body it sits.
        let shown = match (&game.screen, bar.whose) {
            (Screen::Trail(expedition), Whose::Horse(i)) => expedition
                .party
                .get(i)
                .map(|horse| (horse.health, horse.max_health, horse_centre(i) + Vec3::Y * impacts.horse_above)),
            // As the scene draws them: not on the way back to town.
            (Screen::Trail(_), Whose::Foe(_)) if game.secs_to_town().is_some() => None,
            (Screen::Trail(expedition), Whose::Foe(slot)) => {
                expedition.foes.get(slot).filter(|foe| foe.health > 0).map(|foe| {
                    let top = enemy_size(foe.max_health) / 2.0 + impacts.enemy_above;
                    (foe.health, foe.max_health, enemy_centre(slot, foe.max_health) + Vec3::Y * top)
                })
            }
            (Screen::Camp { .. }, _) => None,
        };
        let spot = shown.and_then(|(_, _, over)| camera.world_to_viewport(view, over).ok());
        let (Some((health, max, _)), Some(spot)) = (shown, spot) else {
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
            continue;
        };
        let fill = (health.max(0) as f32 / max.max(1) as f32).clamp(0.0, 1.0);
        if max != bar.max {
            (bar.max, bar.trail, bar.since) = (max, fill, f32::INFINITY);
        } else if fill < bar.fill {
            bar.since = 0.0;
        } else {
            bar.since += dt;
        }
        bar.fill = fill;
        bar.trail = trail_after(&impacts, bar.trail, fill, bar.since, dt);

        let (left, top) = (
            Val::Px(spot.x - impacts.bar_width / 2.0),
            Val::Px(spot.y - impacts.bar_height / 2.0),
        );
        if (node.left, node.top) != (left, top) {
            (node.left, node.top) = (left, top);
        }
        for (piece, share) in [(bar.fill_node, bar.fill), (bar.trail_node, bar.trail)] {
            if let Ok(mut piece) = pieces.get_mut(piece) {
                let width = Val::Percent(share * 100.0);
                if piece.width != width {
                    piece.width = width;
                }
            }
        }
        if *visibility != Visibility::Inherited {
            *visibility = Visibility::Inherited;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The numbers can be used as they are, and a strong hit throws more
    /// sparks, faster and bigger, than a plain one, and a plain one than a
    /// resisted or warded one, so they are told apart without the number.
    #[test]
    fn a_stronger_hit_throws_more_sparks() {
        let impacts = Impacts::load();
        assert!(impacts.gravity > 0.0 && impacts.spark_secs > 0.0 && impacts.spark_secs < 1.0);
        assert!((0.0..1.0).contains(&impacts.lift) && (0.0..=1.0).contains(&impacts.resisted_grey));
        assert!(impacts.bar_width > impacts.bar_height && impacts.bar_height > 0.0);
        assert!(impacts.hold_secs >= 0.0 && impacts.drain > 0.0);
        let order = [HitKind::Resisted, HitKind::Plain, HitKind::Strong].map(|kind| impacts.sparks.of(kind));
        for pair in order.windows(2) {
            let (weak, strong) = (pair[0], pair[1]);
            assert!(weak.count < strong.count && weak.size < strong.size, "{weak:?} {strong:?}");
            assert!(weak.speed.1 < strong.speed.1, "{weak:?} {strong:?}");
        }
        for burst in order {
            assert!(burst.count > 0 && 0.0 < burst.speed.0 && burst.speed.0 <= burst.speed.1);
        }
        assert_eq!(impacts.sparks.of(HitKind::Warded).count, impacts.sparks.resisted.count);
    }

    /// A hit's sparks start where it struck, all leave off the struck face
    /// and none downward, and the same seed throws them the same way.
    #[test]
    fn sparks_fly_off_the_struck_face() {
        let impacts = Impacts::load();
        let at = Vec3::new(3.0, 1.0, 0.0);
        for burst in [impacts.sparks.strong, impacts.sparks.plain, impacts.sparks.resisted] {
            for away in [1.0, -1.0] {
                for seed in 0..50 {
                    let sparks = plan_sparks(&impacts, burst, seed, at, away);
                    assert_eq!(sparks.len(), burst.count);
                    assert_eq!(sparks, plan_sparks(&impacts, burst, seed, at, away));
                    for spark in &sparks {
                        assert_eq!(spark.position, at);
                        assert!(spark.velocity.x * away > 0.0, "{spark:?}");
                        assert!(spark.velocity.y >= 0.0, "{spark:?}");
                        let speed = spark.velocity.length();
                        assert!(speed >= burst.speed.0 - 1e-3 && speed <= burst.speed.1 + 1e-3);
                    }
                }
            }
        }
        // And a spark falls once thrown.
        let mut spark = plan_sparks(&impacts, impacts.sparks.plain, 1, at, 1.0)[0];
        let up = spark.velocity.y;
        spark.step(impacts.gravity, 0.1);
        assert!(spark.velocity.y < up);
        assert_eq!(spark_scale(&impacts, 0.0), 1.0);
        assert_eq!(spark_scale(&impacts, impacts.spark_secs), 0.0);
    }

    /// The trailing piece never shows less than the health; it holds where
    /// the health was, then falls to it and stops there; and it is never
    /// left above health that has come back up.
    #[test]
    fn the_trail_holds_then_follows_the_health_down() {
        let impacts = Impacts::load();
        let dt = 1.0 / 60.0;
        assert_eq!(trail_after(&impacts, 1.0, 0.6, 0.0, dt), 1.0);
        assert_eq!(trail_after(&impacts, 1.0, 0.6, impacts.hold_secs * 0.99, dt), 1.0);
        let (mut trail, mut since, mut frames) = (1.0, 0.0, 0);
        while trail > 0.6 {
            trail = trail_after(&impacts, trail, 0.6, since, dt);
            assert!(trail >= 0.6);
            since += dt;
            frames += 1;
            assert!(frames < 600, "the trail never caught up");
        }
        assert_eq!(trail, 0.6);
        let secs = frames as f32 * dt;
        assert!(secs > impacts.hold_secs && secs < impacts.hold_secs + 0.4 / impacts.drain + 0.05, "{secs}");
        // Health back up: the trail is at it at once.
        assert_eq!(trail_after(&impacts, 0.3, 0.8, 0.0, dt), 0.8);
    }
}
