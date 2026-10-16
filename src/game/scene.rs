//! The 3D picture: a fixed isometric-angle camera, a trail that scrolls left,
//! three horses walking right, built to the proportions in
//! `horse_shape.ron`, and cubes for the enemies. A horse at no health is
//! exhausted, not gone: it lies down while the party stands and walks on
//! with its head low.
//!
//! The trail runs along world +X, which is screen-right. Height is Y.

use bevy::{
    asset::RenderAssetUsages,
    camera::ScalingMode,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::render_resource::Face,
    window::PrimaryWindow,
};

use std::collections::HashMap;

use super::menu::picture::LookImages;
use super::menu::shell::Shell;
use super::menu::tokens::Tokens;
use super::{Game, Landed, SceneView, Screen};
use crate::sim::data::{Element, Slot};
use crate::sim::expedition::Struck;
use crate::sim::look::Look;
use crate::sim::horse_shape::{Frame, HorseShape, Joint, Kind, LEGS, Part, Section};
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
/// The town gate, an arch on the far side of the trail, facing the camera:
/// where its middle is when the party is in town, just ahead of the first
/// horse, how far along the trail each post is from that, how far back it
/// stands, and how high its beam is.
const GATE_X: f32 = 1.6;
const GATE_HALF_SPAN: f32 = 1.3;
const GATE_Z: f32 = -2.2;
const GATE_HEIGHT: f32 = 2.6;
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
/// Placeholders until horses and enemies move as their own (#365). Striking,
/// a body tips forward this far, in radians, and back, over this many
/// seconds; struck, it tips back this far and steps back this far, in world
/// units, and returns.
const LUNGE_TILT: f32 = 0.18;
const LUNGE_SECS: f32 = 0.3;
const FLINCH_TILT: f32 = 0.14;
const FLINCH_STEP: f32 = 0.25;
const FLINCH_SECS: f32 = 0.3;
/// How dark a horse's hooves and hair are, as shares of its body's shade.
const HOOF_SHADE: f32 = 0.45;
const HAIR_SHADE: f32 = 0.3;
/// How far the selected horse's outline stands out from it, in world units
/// (#356): each piece has a shell this much bigger, drawn from the inside.
const OUTLINE: f32 = 0.06;
/// How far each shell is pushed back from the camera (#369), in world units.
/// The camera is orthographic, so it does not move on the screen; it falls
/// behind the horse's own pieces, which hide the rims where they meet and
/// leave only the outline round the whole horse.
const OUTLINE_PUSH: f32 = 0.25;
/// How far worn gear's dark rim stands out from it, in world units (#359).
const WORN_RIM: f32 = 0.025;
/// How much room round a horse a press still lands on it, in world units.
const PRESS_MARGIN: f32 = 0.15;

pub struct ScenePlugin {
    /// Start with the horses' hair hidden.
    pub bare: bool,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(Bare(self.bare))
            .init_resource::<LookImages>()
            .init_resource::<Tired>()
            .init_resource::<Motion>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    frame,
                    scroll,
                    (move_with_hits, ((sync_horses, swing_legs).chain(), sync_enemies)).chain(),
                    move_gate,
                    (toggle_hair, outline_selected).chain(),
                    show_worn,
                ),
            );
    }
}

/// The box a horse stands in, in its own space: what a press on the scene
/// is tested against to find which horse it landed on (#356).
#[derive(Resource, Clone, Copy)]
pub struct HorseBox {
    min: Vec3,
    max: Vec3,
}

impl HorseBox {
    fn of(parts: &[Part]) -> Self {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        for part in parts {
            for section in [&part.from, &part.to] {
                for [x, y] in [section.top, section.bottom] {
                    for z in [section.z - section.width / 2.0, section.z + section.width / 2.0] {
                        min = min.min(Vec3::new(x, y, z));
                        max = max.max(Vec3::new(x, y, z));
                    }
                }
            }
        }
        HorseBox {
            min: min - PRESS_MARGIN,
            max: max + PRESS_MARGIN,
        }
    }

    /// The horse a ray from the camera meets first, if it meets one: where
    /// each stands to fight, as the party never leaves its places.
    pub fn horse_on(&self, ray: Ray3d) -> Option<usize> {
        (0..HORSE_X.len())
            .filter_map(|i| {
                let at = Vec3::new(HORSE_X[i], 0.0, HORSE_Z[i]);
                entry(ray, at + self.min, at + self.max).map(|distance| (i, distance))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }
}

/// The horses as the line reads across the scene, left to right (#356):
/// the last of the party first, as the party walks to the right.
pub fn line_order(horses: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..horses.min(HORSE_X.len())).collect();
    order.sort_by(|a, b| HORSE_X[*a].total_cmp(&HORSE_X[*b]));
    order
}

/// The ground in front of the horse in place `index`, level with the front
/// of the foremost horse, so the three tags hang in one line and the one
/// standing back does not cover the others' (#356).
pub fn horse_feet(index: usize) -> Vec3 {
    let front = HORSE_Z.iter().copied().fold(f32::MIN, f32::max);
    Vec3::new(HORSE_X[index], 0.0, front + 0.3)
}

/// How far along a ray it enters a box, if it does: the slab test.
fn entry(ray: Ray3d, min: Vec3, max: Vec3) -> Option<f32> {
    let (mut near, mut far) = (f32::MIN, f32::MAX);
    for axis in 0..3 {
        let (origin, direction) = (ray.origin[axis], ray.direction[axis]);
        if direction.abs() < 1e-6 {
            if origin < min[axis] || origin > max[axis] {
                return None;
            }
            continue;
        }
        let (a, b) = ((min[axis] - origin) / direction, (max[axis] - origin) / direction);
        near = near.max(a.min(b));
        far = far.min(a.max(b));
    }
    (near <= far && far >= 0.0).then_some(near)
}

/// The outline of one piece of a horse, shown while that horse is the one
/// selected (#356).
#[derive(Component)]
struct Halo {
    horse: usize,
    kind: Kind,
}

/// The selected horse is outlined in the scene, in town and on the road,
/// and the outline follows a press on another horse. Hair's outline goes
/// with the hair. Each shell is kept `OUTLINE_PUSH` behind its piece as
/// seen from the camera, whichever way its joint has turned (#369).
fn outline_selected(
    shell: Option<Res<Shell>>,
    bare: Res<Bare>,
    camera: Single<&GlobalTransform, With<Camera3d>>,
    joints: Query<&GlobalTransform, Without<Halo>>,
    mut halos: Query<(&Halo, &ChildOf, &mut Transform, &mut Visibility)>,
) {
    let Some(shell) = shell else {
        return;
    };
    let back = camera.forward() * OUTLINE_PUSH;
    for (halo, parent, mut transform, mut visibility) in &mut halos {
        let wanted = if shell.marked() == Some(halo.horse) {
            hair_visibility(halo.kind, bare.0)
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
        if wanted != Visibility::Hidden
            && let Ok(joint) = joints.get(parent.parent())
        {
            transform.translation = joint.affine().inverse().transform_vector3(back);
        }
    }
}

/// The town gate. It is what reaching town looks like: it comes up the
/// trail as the party walks back, stands in front of the party in town, and
/// is left behind when the party sets out.
#[derive(Component)]
struct TownGate;

/// Whether the horses' hair is hidden, to see the build underneath.
#[derive(Resource)]
struct Bare(bool);

/// One piece of a horse, and what it is made of.
#[derive(Component)]
struct Piece(Kind);

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

/// How each horse and each enemy's place is moving from the hits it deals
/// and takes.
#[derive(Resource)]
struct Motion {
    horse: [Pose; HORSE_X.len()],
    foe: [Pose; ENEMY_X.len()],
}

impl Default for Motion {
    fn default() -> Self {
        Self { horse: [Pose::STILL; HORSE_X.len()], foe: [Pose::STILL; ENEMY_X.len()] }
    }
}

impl Motion {
    fn of(&mut self, who: Struck) -> Option<&mut Pose> {
        match who {
            Struck::Horse(horse) => self.horse.get_mut(horse),
            Struck::Foe { slot, .. } => self.foe.get_mut(slot),
        }
    }
}

/// Seconds since a body last struck and was struck.
#[derive(Clone, Copy)]
struct Pose {
    lunge: f32,
    flinch: f32,
}

impl Pose {
    const STILL: Pose = Pose { lunge: LUNGE_SECS, flinch: FLINCH_SECS };

    /// How far it leans toward its foe, in radians, and how far it has
    /// stepped back from it, in world units.
    fn lean(&self) -> (f32, f32) {
        let bump = |since: f32, secs: f32| (since / secs * std::f32::consts::PI).sin();
        let lunge = bump(self.lunge, LUNGE_SECS);
        let flinch = bump(self.flinch, FLINCH_SECS);
        (LUNGE_TILT * lunge - FLINCH_TILT * flinch, FLINCH_STEP * flinch)
    }
}

fn move_with_hits(time: Res<Time>, mut landed: MessageReader<Landed>, mut motion: ResMut<Motion>) {
    let dt = time.delta_secs();
    let motion = &mut *motion;
    for pose in motion.horse.iter_mut().chain(motion.foe.iter_mut()) {
        pose.lunge = (pose.lunge + dt).min(LUNGE_SECS);
        pose.flinch = (pose.flinch + dt).min(FLINCH_SECS);
    }
    for Landed { hit, .. } in landed.read() {
        if let Some(pose) = motion.of(hit.by) {
            pose.lunge = 0.0;
        }
        if let Some(pose) = motion.of(hit.struck) {
            pose.flinch = 0.0;
        }
    }
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
    bare: Res<Bare>,
    tokens: Res<Tokens>,
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

    // The town gate: two posts and a beam, behind the lane.
    let timber = materials.add(Color::srgb(0.62, 0.52, 0.36));
    commands
        .spawn((TownGate, Transform::from_xyz(GATE_X, 0.0, GATE_Z), Visibility::Inherited))
        .with_children(|gate| {
            for x in [-GATE_HALF_SPAN, GATE_HALF_SPAN] {
                gate.spawn((
                    Mesh3d(cube.clone()),
                    MeshMaterial3d(timber.clone()),
                    Transform::from_xyz(x, GATE_HEIGHT / 2.0, 0.0)
                        .with_scale(Vec3::new(0.4, GATE_HEIGHT, 0.4)),
                ));
            }
            gate.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(timber.clone()),
                Transform::from_xyz(0.0, GATE_HEIGHT, 0.0).with_scale(Vec3::new(
                    2.0 * GATE_HALF_SPAN + 1.0,
                    0.4,
                    0.5,
                )),
            ));
        });

    // Every horse is the base horse for now, so one set of meshes does for
    // all three.
    let shape = HorseShape::load();
    let horse = shape.base(shape.height);
    let parts = horse.parts();
    commands.insert_resource(HorseBox::of(&parts));
    let pieces: Vec<Piece3d> = parts
        .into_iter()
        .map(|part| {
            let origin = joint_origin(&horse, part.joint);
            Piece3d {
                part,
                mesh: meshes.add(piece_mesh(&part, origin, 0.0)),
                halo: meshes.add(piece_mesh(&part, origin, OUTLINE)),
            }
        })
        .collect();
    // The outline is the inside of a shell round each piece: only its far
    // faces are drawn, so the horse covers all of it but a rim.
    let halo = materials.add(StandardMaterial {
        base_color: tokens.colour.accent.0,
        unlit: true,
        cull_mode: Some(Face::Front),
        ..default()
    });
    // What each slot's piece looks like worn (#359): its look (#361) is
    // laid over the whole slot as seen from the side.
    let mut worn = Vec::new();
    for slot in Slot::ALL {
        let parts = horse.worn(slot);
        let corners = parts.iter().flat_map(|part| {
            [part.from.top, part.from.bottom, part.to.top, part.to.bottom].map(Vec2::from)
        });
        let side = corners.fold((Vec2::MAX, Vec2::MIN), |(lo, hi), p| (lo.min(p), hi.max(p)));
        for part in parts {
            let origin = joint_origin(&horse, part.joint);
            worn.push(WornPiece {
                slot,
                joint: part.joint,
                mesh: meshes.add(side_uvs(piece_mesh(&part, origin, 0.0), origin, side)),
                rim: meshes.add(piece_mesh(&part, origin, WORN_RIM)),
            });
        }
    }
    let looks = WornLooks {
        made: HashMap::new(),
        unworn: materials.add(Color::WHITE),
        rim: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            cull_mode: Some(Face::Front),
            ..default()
        }),
    };
    for (i, &x) in HORSE_X.iter().enumerate() {
        spawn_horse(
            &mut commands,
            &mut materials,
            &horse,
            &pieces,
            &halo,
            &worn,
            &looks,
            bare.0,
            i,
            Vec3::new(x, 0.0, HORSE_Z[i]),
        );
    }
    commands.insert_resource(looks);

    let elements = ElementMaterials {
        fire: materials.add(tokens.colour.of(Element::Fire)),
        ice: materials.add(tokens.colour.of(Element::Ice)),
        plant: materials.add(tokens.colour.of(Element::Plant)),
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

/// Where a joint is, in the horse's own space.
fn joint_origin(horse: &Frame, joint: Joint) -> Vec3 {
    match joint {
        Joint::Trunk => Vec3::ZERO,
        Joint::Neck => Vec2::from(horse.neck_joint()).extend(0.0),
        Joint::Leg(leg) => Vec3::from(horse.leg_joint(leg)),
    }
}

/// One piece of the horse, with its mesh and its outline's.
struct Piece3d {
    part: Part,
    mesh: Handle<Mesh>,
    halo: Handle<Mesh>,
}

/// One piece of what a slot's gear looks like worn, with its mesh.
struct WornPiece {
    slot: Slot,
    joint: Joint,
    mesh: Handle<Mesh>,
    rim: Handle<Mesh>,
}

/// A piece of gear as the horse wears it (#359): shown while the horse
/// wears something in that slot, painted with the piece's look (#361). A
/// `rim` is its dark edge, so a pale piece still reads on a pale horse.
#[derive(Component)]
struct Worn {
    horse: usize,
    slot: Slot,
    rim: bool,
}

/// Worn gear's material for each tier and look worn so far, one for a
/// piece not yet shown, and its rim's.
#[derive(Resource)]
struct WornLooks {
    made: HashMap<(usize, Look), Handle<StandardMaterial>>,
    unworn: Handle<StandardMaterial>,
    rim: Handle<StandardMaterial>,
}

/// Each horse wears what the party says it wears, as soon as that changes:
/// an Equip, a swap, an Undo or a drag.
fn show_worn(
    game: Res<Game>,
    mut looks: ResMut<WornLooks>,
    mut images: ResMut<Assets<Image>>,
    mut pictures: ResMut<LookImages>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut worn: Query<(&Worn, &mut Visibility, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    for (worn, mut visibility, mut material) in &mut worn {
        let piece = game
            .party
            .get(worn.horse)
            .and_then(|horse| horse.gear[worn.slot.index()].as_ref())
            .map(|gear| (gear.rarity, gear.look));
        let wanted = match piece {
            Some(_) if worn.rim => Visibility::Inherited,
            Some((tier, look)) => {
                let painted = looks
                    .made
                    .entry((tier, look))
                    .or_insert_with(|| {
                        let image = pictures.worn(&mut images, &game.data, tier, look);
                        materials.add(StandardMaterial {
                            base_color_texture: Some(image),
                            ..default()
                        })
                    })
                    .clone();
                if material.0 != painted {
                    material.0 = painted;
                }
                Visibility::Inherited
            }
            None => Visibility::Hidden,
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// The corners of a piece's six faces, as indices into its eight corners:
/// the four of the section it starts from, then the four it ends at.
const FACES: [[usize; 4]; 6] = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [0, 1, 5, 4],
    [3, 2, 6, 7],
    [0, 3, 7, 4],
    [1, 2, 6, 5],
];

/// Each face's normal, pointing out of the solid whichever way round its
/// corners were listed, and whether they were listed that way round.
fn face_normals(corners: &[Vec3; 8]) -> [(Vec3, bool); 6] {
    let middle = corners.iter().sum::<Vec3>() / 8.0;
    FACES.map(|face| {
        let [p0, p1, p2, p3] = face.map(|i| corners[i]);
        let normal = (p2 - p0).cross(p3 - p1).normalize_or_zero();
        let centre = (p0 + p1 + p2 + p3) / 4.0;
        let outward = normal.dot(centre - middle) >= 0.0;
        (if outward { normal } else { -normal }, outward)
    })
}

/// A mesh's picture laid on it as seen from the side: across `side`, the
/// box from its lowest back corner to its highest front one in the horse's
/// space, the picture's left at the back and its top at the top. The mesh's
/// corners are placed relative to `origin`.
fn side_uvs(mut mesh: Mesh, origin: Vec3, (lo, hi): (Vec2, Vec2)) -> Mesh {
    let span = (hi - lo).max(Vec2::splat(1e-4));
    let uvs: Vec<[f32; 2]> = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|p| p.as_float3()) {
        Some(positions) => positions
            .iter()
            .map(|p| {
                let at = (Vec3::from(*p) + origin).truncate();
                [((at.x - lo.x) / span.x).clamp(0.0, 1.0), ((hi.y - at.y) / span.y).clamp(0.0, 1.0)]
            })
            .collect(),
        None => return mesh,
    };
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

/// The mesh of one piece: the solid between its two sections, flat-faced,
/// placed relative to the joint it moves with. `grow` moves every face out
/// by that much, for the piece's outline.
fn piece_mesh(part: &Part, origin: Vec3, grow: f32) -> Mesh {
    let corners = |section: &Section| {
        let (near, far) = (section.z + section.width / 2.0, section.z - section.width / 2.0);
        [
            Vec3::new(section.top[0], section.top[1], near) - origin,
            Vec3::new(section.top[0], section.top[1], far) - origin,
            Vec3::new(section.bottom[0], section.bottom[1], far) - origin,
            Vec3::new(section.bottom[0], section.bottom[1], near) - origin,
        ]
    };
    let (a, b) = (corners(&part.from), corners(&part.to));
    let mut solid = [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]];
    if grow > 0.0 {
        // A corner is on three faces, and moves to where it is `grow` out
        // from each of them, so each face stands that much further out.
        let normals = face_normals(&solid);
        solid = std::array::from_fn(|i| {
            let on: Vec<Vec3> = FACES
                .iter()
                .zip(normals)
                .filter(|(face, _)| face.contains(&i))
                .map(|(_, (normal, _))| normal)
                .collect();
            let rows = Mat3::from_cols(on[0], on[1], on[2]).transpose();
            let push = if rows.determinant().abs() > 1e-3 {
                rows.inverse() * Vec3::splat(grow)
            } else {
                // Two of its faces lie flat together: out along them all.
                on.iter().sum::<Vec3>().normalize_or_zero() * grow
            };
            // A sharp point would send it far out; it stops short.
            solid[i] + push.clamp_length_max(3.0 * grow)
        });
    }
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    for (face, (normal, outward)) in FACES.iter().zip(face_normals(&solid)) {
        let face = face.map(|i| solid[i]);
        let first = positions.len() as u32;
        let order = if outward { [0, 1, 2, 0, 2, 3] } else { [0, 2, 1, 0, 3, 2] };
        indices.extend(order.map(|i| first + i));
        for corner in face {
            positions.push(corner.to_array());
            normals.push(normal.to_array());
        }
    }
    let uvs = vec![[0.0, 0.0]; positions.len()];
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}

/// Build a horse from its pieces. Its shade comes from `index`, so each
/// party member has its own look, the same every launch. The neck and each
/// leg hang from a joint; hair is pieces of its own, hidden when `bare`.
fn spawn_horse(
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    horse: &Frame,
    pieces: &[Piece3d],
    halo: &Handle<StandardMaterial>,
    worn: &[WornPiece],
    looks: &WornLooks,
    bare: bool,
    index: usize,
    position: Vec3,
) {
    let mut rng = Rng::new(0x4057E + index as u64);
    let shade = 0.55 + 0.45 * rng.unit();
    let grey = |share: f32| Color::srgb(shade * share, shade * share, shade * share);
    let body = materials.add(grey(1.0));
    let hoof = materials.add(grey(HOOF_SHADE));
    let hair = materials.add(grey(HAIR_SHADE));

    // Each piece, and beside it its outline, hidden until the horse is
    // selected.
    let spawn_pieces = |parent: &mut ChildSpawnerCommands, joint: Joint| {
        for piece in pieces.iter().filter(|piece| piece.part.joint == joint) {
            let kind = piece.part.kind;
            let material = match kind {
                Kind::Body | Kind::Gear => &body,
                Kind::Hoof => &hoof,
                Kind::Hair => &hair,
            };
            parent.spawn((
                Piece(kind),
                Mesh3d(piece.mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::default(),
                hair_visibility(kind, bare),
            ));
            parent.spawn((
                Halo { horse: index, kind },
                Mesh3d(piece.halo.clone()),
                MeshMaterial3d(halo.clone()),
                Transform::default(),
                Visibility::Hidden,
            ));
        }
        // Gear over it, hidden until the horse wears something there.
        for piece in worn.iter().filter(|piece| piece.joint == joint) {
            for (rim, mesh, material) in [
                (false, &piece.mesh, &looks.unworn),
                (true, &piece.rim, &looks.rim),
            ] {
                parent.spawn((
                    Worn { horse: index, slot: piece.slot, rim },
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::default(),
                    Visibility::Hidden,
                ));
            }
        }
    };
    commands
        .spawn((
            HorseBody { index, leg_len: horse.clearance },
            Transform::from_translation(position),
            Visibility::default(),
        ))
        .with_children(|trunk| {
            spawn_pieces(trunk, Joint::Trunk);
            // Neck and head hang from a joint at the withers, which drops
            // them when the horse is exhausted.
            trunk
                .spawn((
                    Neck(index),
                    Transform::from_translation(joint_origin(horse, Joint::Neck)),
                    Visibility::default(),
                ))
                .with_children(|neck| spawn_pieces(neck, Joint::Neck));
            // Legs hang from a joint that swings; diagonal pairs move
            // together.
            for leg in 0..LEGS.len() {
                let phase = if leg == 0 || leg == 3 { 0.0 } else { std::f32::consts::PI };
                trunk
                    .spawn((
                        Leg { horse: index, phase },
                        Transform::from_translation(joint_origin(horse, Joint::Leg(leg))),
                        Visibility::default(),
                    ))
                    .with_children(|limb| spawn_pieces(limb, Joint::Leg(leg)));
            }
        });
}

fn hair_visibility(kind: Kind, bare: bool) -> Visibility {
    if kind == Kind::Hair && bare { Visibility::Hidden } else { Visibility::Inherited }
}

/// F5 hides the horses' hair, and shows it again: mane, forelock, tail hair
/// and feathering. A dev aid, for looking at the build underneath.
fn toggle_hair(
    keys: Res<ButtonInput<KeyCode>>,
    mut bare: ResMut<Bare>,
    mut pieces: Query<(&Piece, &mut Visibility)>,
) {
    if !keys.just_pressed(KeyCode::F5) {
        return;
    }
    bare.0 = !bare.0;
    for (piece, mut visibility) in &mut pieces {
        *visibility = hair_visibility(piece.0, bare.0);
    }
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
    for i in 0..HORSE_X.len() {
        let horse = horse_centre(i);
        if i == 0 || point.xz().distance(horse.xz()) < point.xz().distance(nearest.xz()) {
            nearest = horse;
        }
    }
    nearest
}

/// The middle of the body of the horse in place `index` of the party, where
/// it stands to fight.
pub fn horse_centre(index: usize) -> Vec3 {
    Vec3::new(HORSE_X[index], HORSE_MIDDLE, HORSE_Z[index])
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

/// Where the town gate is. On the way back it is as far ahead as the party
/// has left to walk, so the party stops as it reaches it; in town it stands
/// there; on the road it is left behind with the rest of the ground.
fn move_gate(
    time: Res<Time>,
    game: Res<Game>,
    gate: Single<(&mut Transform, &mut Visibility), With<TownGate>>,
) {
    let (mut transform, mut visibility) = gate.into_inner();
    let x = if let Some(secs) = game.secs_to_town() {
        GATE_X + SCROLL_SPEED * secs
    } else if game.in_town() {
        GATE_X
    } else if game.party_moving() {
        (transform.translation.x - SCROLL_SPEED * time.delta_secs()).max(-2.0 * WRAP)
    } else {
        transform.translation.x
    };
    if transform.translation.x != x {
        transform.translation.x = x;
    }
    let wanted = if x > -2.0 * WRAP { Visibility::Inherited } else { Visibility::Hidden };
    if *visibility != wanted {
        *visibility = wanted;
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
/// A horse that lands a hit tips forward and back, and one that is hit
/// flinches.
fn sync_horses(
    time: Res<Time>,
    game: Res<Game>,
    motion: Res<Motion>,
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
        // Its foes are ahead of it, along +x.
        let (lean, back) = motion.horse[body.index].lean();
        let x = HORSE_X[body.index] - back;
        if transform.translation.x != x {
            transform.translation.x = x;
        }
        let rotation = Quat::from_rotation_z(-lean);
        if transform.rotation != rotation {
            transform.rotation = rotation;
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

/// An enemy that lands a hit tips forward and back, and one that is hit
/// flinches.
fn sync_enemies(
    game: Res<Game>,
    motion: Res<Motion>,
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
            // On the way back to town the trail's enemies are behind the
            // party; `fx` shows the ones left standing drawing off.
            Screen::Trail(_) if game.secs_to_town().is_some() => None,
            Screen::Trail(expedition) => expedition.foes.get(body.0).filter(|f| f.health > 0),
            Screen::Camp { .. } => None,
        };
        let Some(foe) = foe else {
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
            continue;
        };
        // It leans and steps on its base; the horses are behind it, along -x.
        let size = enemy_size(foe.max_health);
        let (lean, back) = motion.foe[body.0].lean();
        let rotation = Quat::from_rotation_z(lean);
        let base = enemy_centre(body.0, foe.max_health) - Vec3::Y * size / 2.0;
        transform.scale = Vec3::splat(size);
        transform.rotation = rotation;
        transform.translation = base + rotation * (Vec3::Y * size / 2.0) + Vec3::X * back;
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

    /// A press finds the horse under it from the camera's angle, the nearer
    /// one where two overlap, and nothing between them or above them.
    #[test]
    fn a_press_finds_the_horse_under_it() {
        let shape = HorseShape::load();
        let parts = shape.base(shape.height).parts();
        let horses = HorseBox::of(&parts);
        let camera = camera_home();
        let down = camera.forward();
        let from = |point: Vec3| Ray3d::new(point - down * 40.0, down);
        for i in 0..HORSE_X.len() {
            assert_eq!(horses.horse_on(from(horse_centre(i))), Some(i), "horse {i}");
        }
        let between = (horse_centre(0) + horse_centre(1)) / 2.0 + Vec3::Y * 4.0;
        assert_eq!(horses.horse_on(from(between)), None);
        assert_eq!(horses.horse_on(from(Vec3::new(ENEMY_X[1], 0.5, 0.0))), None);
    }

    /// A piece's outline is the piece with every face moved out: it holds
    /// the piece, and stands out from it by the outline's width.
    #[test]
    fn an_outline_stands_out_from_its_piece() {
        let shape = HorseShape::load();
        let horse = shape.base(shape.height);
        for part in horse.parts() {
            let size = |mesh: Mesh| {
                let Some(bevy::mesh::VertexAttributeValues::Float32x3(points)) =
                    mesh.attribute(Mesh::ATTRIBUTE_POSITION).cloned()
                else {
                    panic!("no positions");
                };
                points.iter().fold((Vec3::MAX, Vec3::MIN), |(lo, hi), p| {
                    (lo.min(Vec3::from(*p)), hi.max(Vec3::from(*p)))
                })
            };
            let (lo, hi) = size(piece_mesh(&part, Vec3::ZERO, 0.0));
            let (out_lo, out_hi) = size(piece_mesh(&part, Vec3::ZERO, OUTLINE));
            assert!(out_lo.cmple(lo).all() && out_hi.cmpge(hi).all(), "{part:?}");
            assert!((out_hi - hi).max_element() >= OUTLINE * 0.99, "{part:?}");
        }
    }
}
