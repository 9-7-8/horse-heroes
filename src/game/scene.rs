//! The 3D picture: a fixed isometric-angle camera, a trail that scrolls left,
//! three box-built horses walking right, and cubes for the enemies. A horse
//! at no health is exhausted, not gone: it lies down while the party stands
//! and walks on with its head low.
//!
//! The trail runs along world +X, which is screen-right. Height is Y.

use bevy::{camera::ScalingMode, prelude::*, window::PrimaryWindow};

use super::menu::tokens::Tokens;
use super::{Game, SceneView, Screen};
use crate::sim::data::Element;
use crate::sim::rng::Rng;

/// World units of height visible in the scene: in the town layout, and in
/// the road's, which is taller so that a drop has room to fly.
const SCENE_HEIGHT: f32 = 7.0;
const ROAD_HEIGHT: f32 = 12.0;
/// How far up the scene the aim point sits, from its bottom edge. Lower on
/// the road: the room gained goes above the party, where the loot goes.
const SCENE_ANCHOR: f32 = 0.5;
const ROAD_ANCHOR: f32 = 0.32;
/// World units of width that stay in view however narrow the window is: the
/// last horse to the furthest enemy, with a margin.
pub const SCENE_WIDTH: f32 = 22.0;
pub const SCROLL_SPEED: f32 = 2.5;
/// The trail's depth, from its far edge to its near one.
pub const TRAIL_DEPTH: f32 = 5.0;
/// Scenery wraps around once it is this far left of the origin.
const WRAP: f32 = 20.0;
const HORSE_X: [f32; 3] = [-1.5, -4.5, -7.5];
const HORSE_Z: [f32; 3] = [0.6, -0.6, 0.6];
/// About how high the middle of a horse's body is.
const HORSE_MIDDLE: f32 = 1.2;
pub const ENEMY_X: [f32; 3] = [3.5, 6.0, 8.5];
/// An exhausted horse: seconds to lie down or get up, how far its neck drops
/// while it is on its feet and how much further lying down, in radians, how
/// far forward its legs fold, how high its belly rests, and the share of
/// its stride it keeps when it walks on.
const TIRED_SECS: f32 = 0.5;
const TIRED_HANG: f32 = 1.05;
const TIRED_LIE_HANG: f32 = 0.45;
const TIRED_FOLD: f32 = std::f32::consts::FRAC_PI_2;
const TIRED_BELLY: f32 = 0.08;
const TIRED_STRIDE: f32 = 0.5;

/// The colour an enemy of each element is drawn in.
pub fn element_colour(element: Element) -> Color {
    match element {
        Element::Fire => Color::srgb(0.9, 0.25, 0.15),
        Element::Ice => Color::srgb(0.35, 0.65, 0.95),
        Element::Plant => Color::srgb(0.3, 0.75, 0.3),
    }
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .init_resource::<Tired>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (frame, scroll, (sync_horses, swing_legs).chain(), sync_enemies),
            );
    }
}

#[derive(Component)]
struct Scrolling;

#[derive(Component)]
struct HorseBody {
    index: usize,
    leg_len: f32,
}

/// The joint at the withers that a horse's neck and head hang from.
#[derive(Component)]
struct Neck(usize);

#[derive(Component)]
struct Leg {
    horse: usize,
    phase: f32,
}

/// How each horse is held, 0 to 1: `hang` is how far its head has dropped
/// and `lie` how far it has lain down. Both stay 0 for a horse that can
/// fight.
#[derive(Resource, Default)]
struct Tired {
    hang: [f32; HORSE_X.len()],
    lie: [f32; HORSE_X.len()],
}

/// What an exhausted horse does: its head hangs, and it lies down unless the
/// party is walking, when it gets up and comes along. Returns (hang, lie).
pub fn tired_pose(exhausted: bool, moving: bool) -> (bool, bool) {
    (exhausted, exhausted && !moving)
}

/// Eased, so a horse starts and stops moving gently.
fn eased(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

#[derive(Component)]
struct EnemyBody(usize);

#[derive(Resource)]
struct ElementMaterials {
    fire: Handle<StandardMaterial>,
    ice: Handle<StandardMaterial>,
    plant: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera at the isometric elevation (about 35 degrees), looking across
    // the trail. Its projection covers the whole window and the menu shell
    // is drawn over the lower part; `frame` sizes it every frame.
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection::default_3d()),
        camera_home(),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-4.0, 10.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // Trail
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(4.0 * WRAP, TRAIL_DEPTH))),
        MeshMaterial3d(materials.add(Color::srgb(0.10, 0.10, 0.10))),
    ));

    // Marker stones along both edges, so the scrolling can be seen.
    let stone = materials.add(Color::srgb(0.45, 0.45, 0.45));
    for i in 0..16 {
        let x = -WRAP + i as f32 * (2.0 * WRAP / 16.0);
        let z = if i % 2 == 0 { -2.2 } else { 2.2 };
        commands.spawn((
            Scrolling,
            Mesh3d(cube.clone()),
            MeshMaterial3d(stone.clone()),
            Transform::from_xyz(x, 0.1, z)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_4))
                .with_scale(Vec3::new(0.7, 0.2, 0.7)),
        ));
    }

    for (i, &x) in HORSE_X.iter().enumerate() {
        spawn_horse(
            &mut commands,
            &mut materials,
            &cube,
            i,
            Vec3::new(x, 0.0, HORSE_Z[i]),
        );
    }

    let elements = ElementMaterials {
        fire: materials.add(element_colour(Element::Fire)),
        ice: materials.add(element_colour(Element::Ice)),
        plant: materials.add(element_colour(Element::Plant)),
    };
    for (i, &x) in ENEMY_X.iter().enumerate() {
        commands.spawn((
            EnemyBody(i),
            Mesh3d(cube.clone()),
            MeshMaterial3d(elements.fire.clone()),
            Transform::from_xyz(x, 0.5, 0.0),
            Visibility::Hidden,
        ));
    }
    commands.insert_resource(elements);
}

/// Build a horse out of boxes. Its proportions and shade come from `index`,
/// so each party member has its own look, the same every launch.
fn spawn_horse(
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    cube: &Handle<Mesh>,
    index: usize,
    position: Vec3,
) {
    let mut rng = Rng::new(0x4057E + index as u64);
    let mut roll = |lo: f32, hi: f32| lo + (hi - lo) * rng.unit();

    let body_len = roll(1.5, 2.0);
    let body_h = roll(0.55, 0.75);
    let body_w = roll(0.45, 0.6);
    let leg_len = roll(0.75, 1.05);
    let neck_len = roll(0.7, 1.0);
    let head_len = roll(0.5, 0.7);
    let shade = roll(0.55, 1.0);

    let material = materials.add(Color::srgb(shade, shade, shade));
    let part = |translation: Vec3, tilt: f32, scale: Vec3| {
        (
            Mesh3d(cube.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_z(tilt))
                .with_scale(scale),
        )
    };

    let back = leg_len + body_h;
    let front = body_len / 2.0;
    commands
        .spawn((
            HorseBody { index, leg_len },
            Transform::from_translation(position),
            Visibility::default(),
        ))
        .with_children(|horse| {
            // Body
            horse.spawn(part(
                Vec3::new(0.0, leg_len + body_h / 2.0, 0.0),
                0.0,
                Vec3::new(body_len, body_h, body_w),
            ));
            // Neck and head hang from a joint at the withers, which drops
            // them when the horse is exhausted.
            horse
                .spawn((
                    Neck(index),
                    Transform::from_xyz(front, back - 0.15, 0.0),
                    Visibility::default(),
                ))
                .with_children(|neck| {
                    // Neck, leaning forward
                    neck.spawn(part(
                        Vec3::new(0.05, neck_len / 2.0, 0.0),
                        -0.5,
                        Vec3::new(0.3, neck_len, 0.28),
                    ));
                    // Head
                    neck.spawn(part(
                        Vec3::new(0.3 + head_len / 2.0, neck_len * 0.8 + 0.15, 0.0),
                        -0.3,
                        Vec3::new(head_len, 0.3, 0.26),
                    ));
                });
            // Tail
            horse.spawn(part(
                Vec3::new(-front - 0.1, back - 0.35, 0.0),
                -0.35,
                Vec3::new(0.12, 0.7, 0.12),
            ));
            // Legs hang from a hip that swings; diagonal pairs move together.
            for (x, z, phase) in [
                (1.0, 1.0, 0.0),
                (1.0, -1.0, std::f32::consts::PI),
                (-1.0, 1.0, std::f32::consts::PI),
                (-1.0, -1.0, 0.0),
            ] {
                horse
                    .spawn((
                        Leg { horse: index, phase },
                        Transform::from_xyz(x * (front - 0.15), leg_len, z * (body_w / 2.0 - 0.09)),
                        Visibility::default(),
                    ))
                    .with_children(|leg| {
                        leg.spawn(part(
                            Vec3::new(0.0, -leg_len / 2.0, 0.0),
                            0.0,
                            Vec3::new(0.16, leg_len, 0.16),
                        ));
                    });
            }
        });
}

/// Keep the scene in its share of the window. The share is the top of the
/// window; the projection is sized so that share is as many world units tall
/// as the layout asks, with the aim point at its anchor. Between the town
/// layout and the road's everything is mixed by `SceneView`, so the move is
/// a glide and never a cut.
fn frame(
    view: Res<SceneView>,
    tokens: Res<Tokens>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut projection: Single<&mut Projection, With<Camera3d>>,
) {
    let expanded = view.expanded();
    let mix = |town: f32, road: f32| town + (road - town) * expanded;
    let share = tokens.layout.share(expanded);
    let aspect = window.width() / window.height().max(1.0);
    // A window too narrow for the party and its enemies shows more height
    // instead of cutting them off.
    let viewport_height = (mix(SCENE_HEIGHT, ROAD_HEIGHT) / share).max(SCENE_WIDTH / aspect);
    let viewport_origin = Vec2::new(0.5, 1.0 - share + share * mix(SCENE_ANCHOR, ROAD_ANCHOR));

    let Projection::Orthographic(current) = &**projection else {
        return;
    };
    let same = current.viewport_origin == viewport_origin
        && matches!(
            current.scaling_mode,
            ScalingMode::FixedVertical { viewport_height: h } if h == viewport_height
        );
    if same {
        return;
    }
    if let Projection::Orthographic(ortho) = &mut **projection {
        ortho.scaling_mode = ScalingMode::FixedVertical { viewport_height };
        ortho.viewport_origin = viewport_origin;
    }
}

/// Where the camera stands when nothing is shaking it.
pub fn camera_home() -> Transform {
    let aim = Vec3::new(0.0, 1.6, 0.0);
    Transform::from_translation(aim + Vec3::new(0.0, 20.0, 28.28)).looking_at(aim, Vec3::Y)
}

/// The side of an enemy's cube. Bigger enemies have more health.
pub fn enemy_size(max_health: i32) -> f32 {
    (max_health as f32).cbrt() / 3.5
}

/// The middle of the body of the horse nearest `point` along the ground:
/// where a drop goes when the party collects it.
pub fn nearest_horse(point: Vec3) -> Vec3 {
    let mut nearest = Vec3::ZERO;
    for (i, &x) in HORSE_X.iter().enumerate() {
        let horse = Vec3::new(x, HORSE_MIDDLE, HORSE_Z[i]);
        if i == 0 || point.xz().distance(horse.xz()) < point.xz().distance(nearest.xz()) {
            nearest = horse;
        }
    }
    nearest
}

/// The middle of the body of the enemy standing in place `slot`.
pub fn enemy_centre(slot: usize, max_health: i32) -> Vec3 {
    Vec3::new(ENEMY_X[slot], enemy_size(max_health) / 2.0, 0.0)
}

fn scroll(
    time: Res<Time>,
    game: Res<Game>,
    mut scenery: Query<&mut Transform, With<Scrolling>>,
) {
    if !game.party_moving() {
        return;
    }
    for mut transform in &mut scenery {
        transform.translation.x -= SCROLL_SPEED * time.delta_secs();
        if transform.translation.x < -WRAP {
            transform.translation.x += 2.0 * WRAP;
        }
    }
}

fn swing_legs(
    time: Res<Time>,
    game: Res<Game>,
    tired: Res<Tired>,
    mut legs: Query<(&Leg, &mut Transform)>,
) {
    // Standing to fight, the legs come to rest under the horse.
    let swing = if game.party_moving() { 0.45 } else { 0.0 };
    let ease = (time.delta_secs() * 12.0).min(1.0);
    for (leg, mut transform) in &mut legs {
        // An exhausted horse takes shorter steps, and lying down its legs
        // fold forward along the ground.
        let stride = 1.0 - (1.0 - TIRED_STRIDE) * eased(tired.hang[leg.horse]);
        let lie = eased(tired.lie[leg.horse]);
        let step = (time.elapsed_secs() * 7.0 + leg.phase).sin() * swing * stride;
        let angle = step * (1.0 - lie) + TIRED_FOLD * lie;
        transform.rotation = transform.rotation.slerp(Quat::from_rotation_z(angle), ease);
    }
}

/// A horse at no health stays in view, exhausted: its head hangs, and it
/// lies down while the party stands. It is on its feet again in town.
fn sync_horses(
    time: Res<Time>,
    game: Res<Game>,
    mut tired: ResMut<Tired>,
    mut horses: Query<(&HorseBody, &mut Transform), Without<Neck>>,
    mut necks: Query<(&Neck, &mut Transform), Without<HorseBody>>,
) {
    let step = time.delta_secs() / TIRED_SECS;
    let towards = |now: f32, wanted: bool| {
        if wanted { (now + step).min(1.0) } else { (now - step).max(0.0) }
    };
    for (body, mut transform) in &mut horses {
        let exhausted = match &game.screen {
            Screen::Trail(expedition) => expedition.party[body.index].health == 0,
            Screen::Camp { .. } => false,
        };
        let (hang, lie) = tired_pose(exhausted, game.party_moving());
        tired.hang[body.index] = towards(tired.hang[body.index], hang);
        tired.lie[body.index] = towards(tired.lie[body.index], lie);
        // Lying, its belly comes down to just above the ground.
        let y = -(body.leg_len - TIRED_BELLY) * eased(tired.lie[body.index]);
        if transform.translation.y != y {
            transform.translation.y = y;
        }
    }
    for (neck, mut transform) in &mut necks {
        let drop = TIRED_HANG * eased(tired.hang[neck.0]) + TIRED_LIE_HANG * eased(tired.lie[neck.0]);
        let rotation = Quat::from_rotation_z(-drop);
        if transform.rotation != rotation {
            transform.rotation = rotation;
        }
    }
}

fn sync_enemies(
    game: Res<Game>,
    elements: Res<ElementMaterials>,
    mut enemies: Query<(
        &EnemyBody,
        &mut Visibility,
        &mut Transform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
) {
    for (body, mut visibility, mut transform, mut material) in &mut enemies {
        let foe = match &game.screen {
            Screen::Trail(expedition) => expedition.foes.get(body.0).filter(|f| f.health > 0),
            Screen::Camp { .. } => None,
        };
        let Some(foe) = foe else {
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
            continue;
        };
        transform.scale = Vec3::splat(enemy_size(foe.max_health));
        transform.translation = enemy_centre(body.0, foe.max_health);
        let wanted = match foe.element {
            Element::Fire => &elements.fire,
            Element::Ice => &elements.ice,
            Element::Plant => &elements.plant,
        };
        if material.0 != *wanted {
            material.0 = wanted.clone();
        }
        if *visibility != Visibility::Inherited {
            *visibility = Visibility::Inherited;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A horse that can fight is never held as an exhausted one; an
    /// exhausted one hangs its head, lies down while the party stands, and
    /// is on its feet whenever the party walks.
    #[test]
    fn an_exhausted_horse_lies_down_only_while_the_party_stands() {
        assert_eq!(tired_pose(false, false), (false, false));
        assert_eq!(tired_pose(false, true), (false, false));
        assert_eq!(tired_pose(true, false), (true, true));
        assert_eq!(tired_pose(true, true), (true, false));
    }
}
