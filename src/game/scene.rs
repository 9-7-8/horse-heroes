//! The 3D picture: a fixed isometric-angle camera, a trail that scrolls left,
//! three box-built horses walking right, and cubes for the enemies.
//!
//! The trail runs along world +X, which is screen-right. Height is Y.

use bevy::{camera::ScalingMode, prelude::*};

use super::{Game, Screen};
use crate::sim::data::Element;
use crate::sim::rng::Rng;

const SCROLL_SPEED: f32 = 2.5;
/// Scenery wraps around once it is this far left of the origin.
const WRAP: f32 = 20.0;
const HORSE_X: [f32; 3] = [-1.5, -4.5, -7.5];
const ENEMY_X: [f32; 3] = [3.5, 6.0, 8.5];

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, setup)
            .add_systems(Update, (scroll, swing_legs, sync_horses, sync_enemies));
    }
}

#[derive(Component)]
struct Scrolling;

#[derive(Component)]
struct HorseBody(usize);

#[derive(Component)]
struct Leg {
    phase: f32,
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
    // the trail. Aimed above the ground so the trail sits low on screen and
    // leaves the top for text.
    let aim = Vec3::new(0.0, 6.5, 0.0);
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 20.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(aim + Vec3::new(0.0, 20.0, 28.28)).looking_at(aim, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-4.0, 10.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // Trail
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(4.0 * WRAP, 5.0))),
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
        let z = [0.6, -0.6, 0.6][i];
        spawn_horse(
            &mut commands,
            &mut materials,
            &cube,
            i,
            Vec3::new(x, 0.0, z),
        );
    }

    let elements = ElementMaterials {
        fire: materials.add(Color::srgb(0.9, 0.25, 0.15)),
        ice: materials.add(Color::srgb(0.35, 0.65, 0.95)),
        plant: materials.add(Color::srgb(0.3, 0.75, 0.3)),
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
            HorseBody(index),
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
            // Neck, leaning forward
            horse.spawn(part(
                Vec3::new(front + 0.05, back + neck_len / 2.0 - 0.15, 0.0),
                -0.5,
                Vec3::new(0.3, neck_len, 0.28),
            ));
            // Head
            horse.spawn(part(
                Vec3::new(front + 0.3 + head_len / 2.0, back + neck_len * 0.8, 0.0),
                -0.3,
                Vec3::new(head_len, 0.3, 0.26),
            ));
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
                        Leg { phase },
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

fn scroll(time: Res<Time>, mut scenery: Query<&mut Transform, With<Scrolling>>) {
    for mut transform in &mut scenery {
        transform.translation.x -= SCROLL_SPEED * time.delta_secs();
        if transform.translation.x < -WRAP {
            transform.translation.x += 2.0 * WRAP;
        }
    }
}

fn swing_legs(time: Res<Time>, mut legs: Query<(&Leg, &mut Transform)>) {
    for (leg, mut transform) in &mut legs {
        let angle = (time.elapsed_secs() * 7.0 + leg.phase).sin() * 0.45;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

/// A horse that is down drops out of view until the next run.
fn sync_horses(game: Res<Game>, mut horses: Query<(&HorseBody, &mut Visibility)>) {
    for (body, mut visibility) in &mut horses {
        let down = match &game.screen {
            Screen::Trail(expedition) => expedition.party[body.0].health == 0,
            Screen::Camp { .. } => false,
        };
        let wanted = if down {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        if *visibility != wanted {
            *visibility = wanted;
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
        // Bigger enemies have more health.
        let size = (foe.max_health as f32).cbrt() / 3.5;
        transform.scale = Vec3::splat(size);
        transform.translation.y = size / 2.0;
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
