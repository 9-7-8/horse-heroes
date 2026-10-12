//! The shape of a horse, from `assets/data/horse_shape.ron`: proportions as
//! ratios of the height at the withers, turned into a `Frame` of lengths for
//! one height and then into the pieces a horse is built from
//! (docs/horse-model.md). Plain numbers; `game::scene` makes the meshes.
//!
//! Nothing varies a horse inside the windows yet (#63), so outside the tests
//! `varied` and what only checks a window are unused, and marked so.
//!
//! A piece is drawn in the horse's side view, x toward its nose and y up
//! from the ground under the middle of its body, and given a width across.

use std::ops::{Add, Mul, Sub};

use serde::Deserialize;

/// One dimension: what the base horse has, and the least and most a horse
/// may have. Ratios of the height, except where a field says otherwise.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct Dim(pub f32, pub f32, pub f32);

impl Dim {
    pub fn base(self) -> f32 {
        self.0
    }

    /// Somewhere in the window: the least at 0, the most at 1.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn at(self, t: f32) -> f32 {
        self.1 + (self.2 - self.1) * t
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HorseShape {
    /// The height at the withers, in world units.
    pub height: f32,
    pub head_length: Dim,
    pub head_width: Dim,
    pub muzzle_width: Dim,
    pub ear_length: Dim,
    pub ear_width: Dim,
    pub neck_length: Dim,
    /// Degrees above level.
    pub neck_carriage: Dim,
    pub neck_base: Dim,
    pub neck_middle: Dim,
    pub neck_throat: Dim,
    pub body_length: Dim,
    pub barrel_width: Dim,
    pub chest_width: Dim,
    pub hindquarter_width: Dim,
    pub tail_dock: Dim,
    pub body_depth: Dim,
    /// Not chosen: what the depth leaves. The window it must stay in.
    #[cfg_attr(not(test), allow(dead_code))]
    pub clearance: Dim,
    pub tail_clearance: Dim,
    /// Not chosen: what the tail's clearance leaves. The window to check.
    #[cfg_attr(not(test), allow(dead_code))]
    pub tail_length: Dim,
    pub croup: Dim,
    pub elbow: Dim,
    pub knee: Dim,
    pub hock: Dim,
    pub fetlock: Dim,
    pub hoof_top: Dim,
    /// Degrees below level.
    pub face_angle: f32,
    pub jaw_depth: f32,
    pub muzzle_depth: f32,
    pub neck_width_base: f32,
    pub neck_width_throat: f32,
    pub bone: f32,
    pub mane_fall: f32,
}

impl HorseShape {
    pub fn load() -> HorseShape {
        super::data::parse(
            "horse_shape.ron",
            include_str!("../../assets/data/horse_shape.ron"),
        )
    }

    /// The base horse at `height`.
    pub fn base(&self, height: f32) -> Frame {
        self.frame(height, &mut |dim| dim.base())
    }

    /// A horse at `height` with every dimension somewhere in its window:
    /// `roll` gives 0 to 1 for each in turn.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn varied(&self, height: f32, roll: &mut impl FnMut() -> f32) -> Frame {
        self.frame(height, &mut |dim| dim.at(roll()))
    }

    fn frame(&self, height: f32, pick: &mut dyn FnMut(Dim) -> f32) -> Frame {
        let mut of = |dim: Dim| pick(dim) * height;
        // The linked pair: the depth is chosen and the clearance is the rest.
        let body_depth = of(self.body_depth);
        Frame {
            height,
            head_length: of(self.head_length),
            head_width: of(self.head_width),
            muzzle_width: of(self.muzzle_width),
            ear_length: of(self.ear_length),
            ear_width: of(self.ear_width),
            neck_length: of(self.neck_length),
            neck_base: of(self.neck_base),
            neck_middle: of(self.neck_middle),
            neck_throat: of(self.neck_throat),
            body_length: of(self.body_length),
            barrel_width: of(self.barrel_width),
            chest_width: of(self.chest_width),
            hindquarter_width: of(self.hindquarter_width),
            tail_dock: of(self.tail_dock),
            body_depth,
            clearance: height - body_depth,
            tail_clearance: of(self.tail_clearance),
            croup: of(self.croup),
            elbow: of(self.elbow),
            knee: of(self.knee),
            hock: of(self.hock),
            fetlock: of(self.fetlock),
            hoof_top: of(self.hoof_top),
            neck_carriage: pick(self.neck_carriage).to_radians(),
            face_angle: self.face_angle.to_radians(),
            jaw_depth: self.jaw_depth * height,
            muzzle_depth: self.muzzle_depth * height,
            neck_width_base: self.neck_width_base * height,
            neck_width_throat: self.neck_width_throat * height,
            bone: self.bone * height,
            mane_fall: self.mane_fall * height,
        }
    }
}

/// One horse's dimensions, in world units; the two angles are in radians.
/// The landmark heights are where gear's sockets go.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub height: f32,
    pub head_length: f32,
    pub head_width: f32,
    pub muzzle_width: f32,
    pub ear_length: f32,
    pub ear_width: f32,
    pub neck_length: f32,
    pub neck_carriage: f32,
    pub neck_base: f32,
    pub neck_middle: f32,
    pub neck_throat: f32,
    pub body_length: f32,
    pub barrel_width: f32,
    pub chest_width: f32,
    pub hindquarter_width: f32,
    pub tail_dock: f32,
    pub body_depth: f32,
    pub clearance: f32,
    pub tail_clearance: f32,
    pub croup: f32,
    pub elbow: f32,
    pub knee: f32,
    pub hock: f32,
    pub fetlock: f32,
    pub hoof_top: f32,
    pub face_angle: f32,
    pub jaw_depth: f32,
    pub muzzle_depth: f32,
    pub neck_width_base: f32,
    pub neck_width_throat: f32,
    pub bone: f32,
    pub mane_fall: f32,
}

/// What a piece is made of. Hair is the mane, forelock, tail hair and
/// feathering: never part of the body, so it can be hidden or swapped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Body,
    Hoof,
    Hair,
}

/// What a piece moves with: the trunk, the joint the neck and head hang
/// from, or one leg's joint (`LEGS` says which).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Joint {
    Trunk,
    Neck,
    Leg(usize),
}

/// The four legs: whether it is a foreleg, and which side it is on (+1 is
/// the near side, toward the camera). Diagonal pairs step together.
pub const LEGS: [(bool, f32); 4] = [(true, 1.0), (true, -1.0), (false, 1.0), (false, -1.0)];

/// One end of a piece: an edge in the side view, from `top` to `bottom`,
/// `width` across and centred `z` off the horse's middle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Section {
    pub top: [f32; 2],
    pub bottom: [f32; 2],
    pub z: f32,
    pub width: f32,
}

/// A piece of the horse: the solid between two sections.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Part {
    pub kind: Kind,
    pub joint: Joint,
    pub from: Section,
    pub to: Section,
}

/// A point in the side view.
#[derive(Clone, Copy)]
struct V(f32, f32);

impl Add for V {
    type Output = V;
    fn add(self, other: V) -> V {
        V(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for V {
    type Output = V;
    fn sub(self, other: V) -> V {
        V(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<f32> for V {
    type Output = V;
    fn mul(self, by: f32) -> V {
        V(self.0 * by, self.1 * by)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn edge(top: V, bottom: V, z: f32, width: f32) -> Section {
    Section { top: [top.0, top.1], bottom: [bottom.0, bottom.1], z, width }
}

/// A section across the trunk at `x`.
fn upright(x: f32, top: f32, bottom: f32, width: f32) -> Section {
    edge(V(x, top), V(x, bottom), 0.0, width)
}

/// A level section through a leg, `half` to the front and back of `x`.
fn level(y: f32, x: f32, half: f32, z: f32, width: f32) -> Section {
    edge(V(x + half, y), V(x - half, y), z, width)
}

/// A section `thick` along `axis`, centred on `centre`.
fn band(centre: V, axis: V, thick: f32, z: f32, width: f32) -> Section {
    edge(centre - axis * (thick / 2.0), centre + axis * (thick / 2.0), z, width)
}

impl Frame {
    fn half_length(&self) -> f32 {
        self.body_length / 2.0
    }

    fn root(&self) -> V {
        V(0.63 * self.half_length(), 0.985 * self.height)
    }

    /// The top of the neck's root, just ahead of the withers.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn neck_root(&self) -> [f32; 2] {
        let root = self.root();
        [root.0, root.1]
    }

    /// The poll: the top of the head, between the ears.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn poll(&self) -> [f32; 2] {
        let (sin, cos) = self.neck_carriage.sin_cos();
        let poll = self.root() + V(cos, sin) * self.neck_length;
        [poll.0, poll.1]
    }

    /// The joint the neck and head hang from, in the middle of the neck's
    /// base.
    pub fn neck_joint(&self) -> [f32; 2] {
        let (sin, cos) = self.neck_carriage.sin_cos();
        let joint = self.root() + V(sin, -cos) * (self.neck_base / 2.0);
        [joint.0, joint.1]
    }

    /// Where a foreleg's cannon stands, the stifle, and a hind cannon.
    fn leg_lines(&self) -> (f32, f32, f32) {
        let half = self.half_length();
        (0.70 * half, -0.56 * half, -0.89 * half)
    }

    fn leg_z(&self, leg: usize) -> f32 {
        let (fore, side) = LEGS[leg];
        let out = if fore {
            self.chest_width / 2.0 - 0.9 * self.bone
        } else {
            self.hindquarter_width / 2.0 - 1.1 * self.bone
        };
        side * out
    }

    /// The joint a leg swings from: x, y and z.
    pub fn leg_joint(&self, leg: usize) -> [f32; 3] {
        let (fore_x, stifle_x, _) = self.leg_lines();
        let x = if LEGS[leg].0 { fore_x } else { stifle_x - 0.02 * self.height };
        [x, self.elbow + 0.05 * self.height, self.leg_z(leg)]
    }

    fn tail_points(&self) -> (V, V, V) {
        let h = self.height;
        let root = V(-self.half_length() + 0.01 * h, self.croup - 0.09 * h);
        let dock_end = root + V(-0.2, -0.98) * self.tail_dock;
        let skirt = V(dock_end.0 - 0.015 * h, self.tail_clearance + 0.225 * h);
        (root, dock_end, skirt)
    }

    /// The tail with its hair, from the root to the longest tips.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn tail_length(&self) -> f32 {
        let (_, dock_end, skirt) = self.tail_points();
        let hang = skirt - dock_end;
        self.tail_dock + hang.0.hypot(hang.1) + (skirt.1 - self.tail_clearance)
    }

    /// Every piece of the horse, standing.
    pub fn parts(&self) -> Vec<Part> {
        let h = self.height;
        let half = self.half_length();
        let (clear, depth, b) = (self.clearance, self.body_depth, self.bone);
        let mut parts = Vec::new();
        let mut add = |kind, joint, from, to| parts.push(Part { kind, joint, from, to });

        // The trunk: narrow at the chest, deepest behind the forelegs,
        // fullest through the ribs, a little in at the flank and broad again
        // behind. The back dips between the withers and the croup, and the
        // belly rises toward the flank.
        let flank = self.barrel_width.min(self.hindquarter_width) * 0.93;
        let trunk = [
            upright(half, clear + 0.74 * depth, clear + 0.17 * depth, self.chest_width),
            upright(0.52 * half, h, clear, 0.97 * self.barrel_width),
            upright(
                0.10 * half,
                (h + self.croup) / 2.0 - 0.04 * h,
                clear + 0.03 * depth,
                self.barrel_width,
            ),
            upright(-0.33 * half, self.croup - 0.035 * h, clear + 0.2 * depth, flank),
            upright(-0.67 * half, self.croup, clear + 0.17 * depth, self.hindquarter_width),
            upright(
                -half,
                self.croup - 0.09 * h,
                clear + 0.3 * depth,
                0.72 * self.hindquarter_width,
            ),
        ];
        for pair in trunk.windows(2) {
            add(Kind::Body, Joint::Trunk, pair[0], pair[1]);
        }

        // The neck: set high, arched along the crest, broad at the base and
        // light at the throat. Its first section is sunk into the shoulder.
        let (sin, cos) = self.neck_carriage.sin_cos();
        let (along, back, under) = (V(cos, sin), V(-sin, cos), V(sin, -cos));
        let root = self.root();
        let length = self.neck_length;
        let poll = root + along * length;
        let throat = poll + V(-0.01 * h, -self.neck_throat);
        let crest = [
            root - along * (0.05 * h),
            root,
            root + along * (length / 3.0) + back * (0.035 * h),
            root + along * (length * 2.0 / 3.0) + back * (0.045 * h),
            poll,
        ];
        let thick = [
            self.neck_base,
            self.neck_base,
            lerp(self.neck_base, self.neck_middle, 2.0 / 3.0),
            lerp(self.neck_middle, self.neck_throat, 1.0 / 3.0),
        ];
        let (wide, narrow) = (self.neck_width_base, self.neck_width_throat);
        let across = [
            wide,
            wide,
            lerp(wide, narrow, 1.0 / 3.0),
            lerp(wide, narrow, 2.0 / 3.0),
            narrow,
        ];
        let mut neck: Vec<Section> = (0..4)
            .map(|i| edge(crest[i], crest[i] + under * thick[i], 0.0, across[i]))
            .collect();
        neck.push(edge(poll, throat, 0.0, narrow));
        for pair in neck.windows(2) {
            add(Kind::Body, Joint::Neck, pair[0], pair[1]);
        }

        // The head: a straight profile from the poll, deepest at the jaw,
        // tapering to a muzzle that is still blunt.
        let (face_sin, face_cos) = self.face_angle.sin_cos();
        let (face, jaw, brow) = (
            V(face_cos, -face_sin),
            V(-face_sin, -face_cos),
            V(face_sin, face_cos),
        );
        let head_at = |along: f32, deep: f32, width: f32| {
            let top = poll + face * (along * self.head_length);
            edge(top, top + jaw * deep, 0.0, width)
        };
        let head = [
            edge(poll, throat, 0.0, 0.8 * self.head_width),
            head_at(0.38, self.jaw_depth, self.head_width),
            head_at(0.82, 1.1 * self.muzzle_depth, self.muzzle_width),
            head_at(1.0, 0.85 * self.muzzle_depth, 0.9 * self.muzzle_width),
        ];
        for pair in head.windows(2) {
            add(Kind::Body, Joint::Neck, pair[0], pair[1]);
        }

        // Ears: short leaves, their tips leaning toward each other.
        for side in [1.0, -1.0] {
            let base = poll + V(-0.02 * h, -0.005 * h);
            let up = V(0.2, 0.98);
            let out = 0.3 * self.head_width;
            let ear_at = |along: f32, size: f32| {
                band(
                    base + up * (along * self.ear_length),
                    V(-1.0, 0.0),
                    0.7 * size * self.ear_width,
                    side * (out - 0.15 * along * self.ear_length),
                    size * self.ear_width,
                )
            };
            let ear = [ear_at(0.0, 1.0), ear_at(0.4, 1.1), ear_at(1.0, 0.2)];
            for pair in ear.windows(2) {
                add(Kind::Body, Joint::Neck, pair[0], pair[1]);
            }
        }

        // Legs. Each row is a level section: its height, where it stands,
        // and half its thickness and its width in bones. The last piece is
        // the hoof.
        let (fore_x, stifle_x, hind_x) = self.leg_lines();
        let fore = [
            (self.elbow + 0.08 * h, fore_x, 1.16, 1.5),
            (self.knee + 0.04 * h, fore_x, 0.64, 1.07),
            (self.knee, fore_x, 0.75, 1.18),
            (self.knee - 0.04 * h, fore_x, 0.54, 0.9),
            (self.fetlock + 0.02 * h, fore_x, 0.5, 0.82),
            (self.fetlock, fore_x, 0.62, 0.96),
            (self.hoof_top, fore_x + 0.02 * h, 0.6, 1.0),
            (0.0, fore_x + 0.03 * h, 0.8, 1.25),
        ];
        // A hind leg keeps its angles: back from the stifle to the hock,
        // and down from there.
        let hind = [
            (self.elbow + 0.10 * h, stifle_x - 0.04 * h, 2.0, 1.6),
            (clear + 0.03 * h, stifle_x, 1.5, 1.4),
            (self.hock + 0.045 * h, hind_x + 0.01 * h, 0.75, 1.0),
            (self.hock, hind_x - 0.005 * h, 0.85, 1.1),
            (self.hock - 0.045 * h, hind_x, 0.58, 0.9),
            (self.fetlock + 0.02 * h, hind_x, 0.52, 0.84),
            (self.fetlock, hind_x + 0.003 * h, 0.64, 0.98),
            (self.hoof_top, hind_x + 0.022 * h, 0.6, 1.0),
            (0.0, hind_x + 0.032 * h, 0.8, 1.25),
        ];
        for (leg, &(is_fore, _)) in LEGS.iter().enumerate() {
            let z = self.leg_z(leg);
            let rows: &[(f32, f32, f32, f32)] = if is_fore { &fore } else { &hind };
            let sections: Vec<Section> = rows
                .iter()
                .map(|&(y, x, thick, width)| level(y, x, thick * b, z, width * b))
                .collect();
            for (i, pair) in sections.windows(2).enumerate() {
                let kind = if i + 2 == sections.len() { Kind::Hoof } else { Kind::Body };
                add(kind, Joint::Leg(leg), pair[0], pair[1]);
            }
            // Feathering: a skirt of hair from the cannon over the fetlock.
            let x = if is_fore { fore_x } else { hind_x };
            add(
                Kind::Hair,
                Joint::Leg(leg),
                level(self.fetlock + 0.09 * h, x - 0.35 * b, 0.45 * b, z, 0.85 * b),
                level(self.hoof_top, x + 0.02 * h - 0.5 * b, 0.95 * b, z, 1.35 * b),
            );
        }

        // The tail: a dock that tapers from the root, and hair over it that
        // falls in a curtain and ends in uneven clumps, the longest at the
        // tail's clearance above the ground.
        let (tail_root, dock_end, skirt) = self.tail_points();
        let (fall, cross, flat) = (V(-0.2, -0.98), V(0.98, -0.2), V(1.0, 0.0));
        add(
            Kind::Body,
            Joint::Trunk,
            band(tail_root, cross, 0.055 * h, 0.0, 0.05 * h),
            band(dock_end, cross, 0.03 * h, 0.0, 0.03 * h),
        );
        let hair = [
            band(tail_root + fall * (0.03 * h), cross, 0.07 * h, 0.0, 0.065 * h),
            band(dock_end, cross, 0.085 * h, 0.0, 0.085 * h),
            band(skirt, flat, 0.11 * h, 0.0, 0.10 * h),
        ];
        for pair in hair.windows(2) {
            add(Kind::Hair, Joint::Trunk, pair[0], pair[1]);
        }
        for (dx, dz, up) in [(0.0, 0.0, 0.0), (-0.032, 0.03, 0.04), (0.032, -0.03, 0.075)] {
            let tip = V(skirt.0 + 1.3 * dx * h, self.tail_clearance + up * h);
            add(
                Kind::Hair,
                Joint::Trunk,
                band(skirt + V(dx * h, 0.0), flat, 0.045 * h, dz * h, 0.04 * h),
                band(tip, flat, 0.018 * h, dz * h, 0.014 * h),
            );
        }

        // The mane: a ridge along the crest and a curtain down the near
        // side, longest in the middle of the neck. Then the forelock.
        let fall_at = [0.0, 0.8, 1.0, 0.9, 0.55];
        let ridge = |i: usize| {
            edge(crest[i] + back * (0.02 * h), crest[i] - back * (0.01 * h), 0.0, 0.5 * across[i])
        };
        let curtain = |i: usize| {
            edge(
                crest[i] + back * (0.015 * h),
                crest[i] + V(0.0, -fall_at[i] * self.mane_fall),
                across[i] / 2.0 + 0.012 * h,
                0.022 * h,
            )
        };
        for i in 1..4 {
            add(Kind::Hair, Joint::Neck, ridge(i), ridge(i + 1));
            add(Kind::Hair, Joint::Neck, curtain(i), curtain(i + 1));
        }
        let forelock = |along: f32, rise: f32, width: f32| {
            let at = poll + face * (along * self.head_length);
            edge(at + brow * (rise * h), at - brow * (0.004 * h), 0.0, width * self.head_width)
        };
        add(Kind::Hair, Joint::Neck, forelock(0.0, 0.016, 0.34), forelock(0.3, 0.008, 0.2));

        parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::rng::Rng;

    fn close(a: f32, b: f32, scale: f32) -> bool {
        (a - b).abs() <= 1e-4 * scale
    }

    fn points(part: &Part) -> [[f32; 2]; 4] {
        [part.from.top, part.from.bottom, part.to.top, part.to.bottom]
    }

    fn lowest<'a>(parts: impl Iterator<Item = &'a Part>) -> f32 {
        parts.flat_map(points).map(|p| p[1]).fold(f32::INFINITY, f32::min)
    }

    fn highest<'a>(parts: impl Iterator<Item = &'a Part>) -> f32 {
        parts.flat_map(points).map(|p| p[1]).fold(f32::NEG_INFINITY, f32::max)
    }

    fn dims(shape: &HorseShape) -> Vec<(&'static str, Dim)> {
        vec![
            ("head_length", shape.head_length),
            ("head_width", shape.head_width),
            ("muzzle_width", shape.muzzle_width),
            ("ear_length", shape.ear_length),
            ("ear_width", shape.ear_width),
            ("neck_length", shape.neck_length),
            ("neck_carriage", shape.neck_carriage),
            ("neck_base", shape.neck_base),
            ("neck_middle", shape.neck_middle),
            ("neck_throat", shape.neck_throat),
            ("body_length", shape.body_length),
            ("barrel_width", shape.barrel_width),
            ("chest_width", shape.chest_width),
            ("hindquarter_width", shape.hindquarter_width),
            ("tail_dock", shape.tail_dock),
            ("body_depth", shape.body_depth),
            ("clearance", shape.clearance),
            ("tail_clearance", shape.tail_clearance),
            ("tail_length", shape.tail_length),
            ("croup", shape.croup),
            ("elbow", shape.elbow),
            ("knee", shape.knee),
            ("hock", shape.hock),
            ("fetlock", shape.fetlock),
            ("hoof_top", shape.hoof_top),
        ]
    }

    /// The file is usable: every base is inside its own window, and the
    /// clearance's window is the depth's seen from the ground.
    #[test]
    fn the_shape_file_holds_together() {
        let shape = HorseShape::load();
        assert!(shape.height > 0.0);
        for (name, dim) in dims(&shape) {
            assert!(0.0 < dim.1 && dim.1 <= dim.0 && dim.0 <= dim.2, "{name}");
        }
        let (depth, clearance) = (shape.body_depth, shape.clearance);
        assert!(close(depth.0 + clearance.0, 1.0, 1.0));
        assert!((depth.1 + clearance.2 - 1.0).abs() < 0.002);
        assert!((depth.2 + clearance.1 - 1.0).abs() < 0.002);
        assert!(shape.bone > 0.0 && shape.mane_fall > 0.0);
        assert!(0.0 < shape.face_angle && shape.face_angle < 90.0);
    }

    /// The base horse at the brief's reference height is the brief's horse,
    /// to the centimetre it rounds to (docs/philosophy/horse-shape.md).
    #[test]
    fn the_base_horse_is_the_briefs() {
        let frame = HorseShape::load().base(165.0);
        for (got, cm) in [
            (frame.head_length, 59.0),
            (frame.head_width, 23.0),
            (frame.muzzle_width, 15.0),
            (frame.ear_length, 15.0),
            (frame.neck_length, 88.0),
            (frame.body_length, 178.0),
            (frame.barrel_width, 60.0),
            (frame.chest_width, 43.0),
            (frame.hindquarter_width, 56.0),
            (frame.body_depth, 76.0),
            (frame.clearance, 89.0),
            (frame.tail_dock, 42.0),
        ] {
            assert!((got - cm).abs() < 1.0, "{got} is not {cm} cm");
        }
        assert!(close(frame.neck_carriage.to_degrees(), 50.0, 50.0));
        let tail = frame.tail_length();
        assert!((130.0..=150.0).contains(&tail), "a tail of {tail} cm");
        assert!((5.0..=10.0).contains(&frame.tail_clearance));
    }

    /// For any height and any choice inside the windows, what is linked
    /// stays linked: depth and clearance make the height, the landmarks
    /// keep their order, and the pieces are built to them.
    #[test]
    fn linked_values_hold_for_any_height_and_any_choice() {
        let shape = HorseShape::load();
        let mut rng = Rng::new(187);
        for round in 0..3000 {
            let h = 0.4 + 3.0 * rng.unit();
            // The corners of the windows first, then anywhere inside them.
            let frame = match round {
                0 => shape.varied(h, &mut || 0.0),
                1 => shape.varied(h, &mut || 1.0),
                _ => shape.varied(h, &mut || rng.unit()),
            };

            assert!(close(frame.body_depth + frame.clearance, h, h));
            let clearance = frame.clearance / h;
            assert!(
                shape.clearance.1 - 0.002 <= clearance && clearance <= shape.clearance.2 + 0.002,
                "a clearance of {clearance} H"
            );
            let ladder = [
                0.0,
                frame.hoof_top,
                frame.fetlock,
                frame.knee,
                frame.hock,
                frame.clearance,
                frame.elbow,
                frame.croup,
            ];
            assert!(ladder.windows(2).all(|pair| pair[0] < pair[1]), "{ladder:?}");
            assert!(frame.croup <= h * 1.0001);
            assert!(frame.muzzle_width < frame.head_width);
            assert!(frame.chest_width < frame.barrel_width);
            assert!(frame.chest_width < frame.hindquarter_width);
            assert!(frame.neck_throat < frame.neck_middle && frame.neck_middle < frame.neck_base);

            let parts = frame.parts();
            for part in &parts {
                for point in points(part) {
                    assert!(point[0].is_finite() && point[1].is_finite());
                }
                assert!(part.from.width > 0.0 && part.to.width > 0.0);
            }
            let is = |kind: Kind, joint: Joint| {
                parts.iter().filter(move |part| part.kind == kind && part.joint == joint)
            };
            // The trunk runs from the clearance up to the withers.
            assert!(close(lowest(is(Kind::Body, Joint::Trunk)), frame.clearance, h));
            assert!(close(highest(is(Kind::Body, Joint::Trunk)), h, h));
            // Every leg stands on the ground, on a hoof as tall as it says.
            for leg in 0..LEGS.len() {
                assert!(close(lowest(is(Kind::Hoof, Joint::Leg(leg))), 0.0, h));
                assert!(close(highest(is(Kind::Hoof, Joint::Leg(leg))), frame.hoof_top, h));
                assert!(close(lowest(is(Kind::Body, Joint::Leg(leg))), frame.hoof_top, h));
                // Its joint is inside the trunk, so no gap opens as it swings.
                assert!(frame.leg_joint(leg)[1] > frame.clearance);
                assert!(frame.leg_joint(leg)[2].abs() < frame.barrel_width / 2.0);
            }
            // The tail's hair ends at its clearance, and nothing is lower
            // than the ground.
            assert!(close(lowest(is(Kind::Hair, Joint::Trunk)), frame.tail_clearance, h));
            assert!(lowest(parts.iter()) > -1e-4 * h);
            // The poll is the neck's length from its root, at its carriage.
            let (root, poll) = (frame.neck_root(), frame.poll());
            let (dx, dy) = (poll[0] - root[0], poll[1] - root[1]);
            assert!(close(dx.hypot(dy), frame.neck_length, h));
            assert!(close(dy.atan2(dx), frame.neck_carriage, 1.0));
            assert!(close(highest(is(Kind::Body, Joint::Neck)), poll[1] + ear_rise(&frame), h));
        }
    }

    /// How far the ear tips stand above the poll.
    fn ear_rise(frame: &Frame) -> f32 {
        0.98 * frame.ear_length - 0.005 * frame.height
    }

    /// Changing the height changes nothing else: a horse twice as tall is
    /// the same horse with every point twice as far out.
    #[test]
    fn changing_the_height_resizes_everything_together() {
        let shape = HorseShape::load();
        let (small, large) = (shape.base(1.0).parts(), shape.base(2.5).parts());
        assert_eq!(small.len(), large.len());
        for (a, b) in small.iter().zip(&large) {
            assert_eq!((a.kind, a.joint), (b.kind, b.joint));
            for (p, q) in points(a).iter().zip(points(b)) {
                assert!(close(p[0] * 2.5, q[0], 2.5) && close(p[1] * 2.5, q[1], 2.5));
            }
            assert!(close(a.from.width * 2.5, b.from.width, 2.5));
            assert!(close(a.to.z * 2.5, b.to.z, 2.5));
        }
        let (a, b) = (shape.base(1.0), shape.base(2.5));
        assert!(close(a.neck_joint()[1] * 2.5, b.neck_joint()[1], 2.5));
        assert!(close(a.leg_joint(2)[0] * 2.5, b.leg_joint(2)[0], 2.5));
    }

    /// Mane, forelock, tail hair and feathering are pieces of their own:
    /// taking the hair away leaves a whole horse, and each leg, the neck
    /// and the tail have some to take.
    #[test]
    fn hair_is_separate_from_the_body() {
        let parts = HorseShape::load().base(1.0).parts();
        let hair = |joint: Joint| {
            parts.iter().filter(|part| part.kind == Kind::Hair && part.joint == joint).count()
        };
        assert!(hair(Joint::Neck) > 0 && hair(Joint::Trunk) > 0);
        for leg in 0..LEGS.len() {
            assert_eq!(hair(Joint::Leg(leg)), 1);
        }
        // Without the hair the tail is still there: the dock is body.
        let bare: Vec<&Part> = parts.iter().filter(|part| part.kind != Kind::Hair).collect();
        let behind = bare
            .iter()
            .flat_map(|part| points(part))
            .map(|p| p[0])
            .fold(f32::INFINITY, f32::min);
        assert!(behind < -HorseShape::load().base(1.0).body_length / 2.0);
    }

    /// The tail's length is not chosen, so it is checked: wherever the
    /// croup, the dock and the hair's clearance sit in their windows, the
    /// tail that comes out is inside the brief's window for it.
    #[test]
    fn the_tail_that_comes_out_is_in_its_window() {
        let shape = HorseShape::load();
        let window = shape.tail_length;
        let in_window = |frame: &Frame| {
            let tail = frame.tail_length() / frame.height;
            assert!(window.1 <= tail && tail <= window.2, "a tail of {tail} H");
        };
        // The corners of the three windows it follows from.
        for corner in 0..8u32 {
            let at = |bit: u32| ((corner >> bit) & 1) as f32;
            let mut frame = shape.base(1.0);
            frame.croup = shape.croup.at(at(0));
            frame.tail_dock = shape.tail_dock.at(at(1));
            frame.tail_clearance = shape.tail_clearance.at(at(2));
            in_window(&frame);
        }
        let mut rng = Rng::new(63);
        for _ in 0..2000 {
            let h = 0.4 + 3.0 * rng.unit();
            in_window(&shape.varied(h, &mut || rng.unit()));
        }
    }

    /// The pieces are the ones docs/horse-model.md counts, each on the
    /// joint it moves with.
    #[test]
    fn the_pieces_are_the_ones_the_doc_counts() {
        let parts = HorseShape::load().base(1.0).parts();
        let count = |kind: Kind, joint: Joint| {
            parts.iter().filter(|part| part.kind == kind && part.joint == joint).count()
        };
        // Five pieces of trunk and the dock.
        assert_eq!(count(Kind::Body, Joint::Trunk), 5 + 1);
        // Four of neck, three of head, two ears of two.
        assert_eq!(count(Kind::Body, Joint::Neck), 4 + 3 + 2 * 2);
        // The mane's ridge and curtain, three each, and the forelock.
        assert_eq!(count(Kind::Hair, Joint::Neck), 3 + 3 + 1);
        // The tail's hair in two lengths, ending in three clumps.
        assert_eq!(count(Kind::Hair, Joint::Trunk), 2 + 3);
        // A hoof is the last piece of a leg and of nothing else.
        assert_eq!(count(Kind::Hoof, Joint::Trunk) + count(Kind::Hoof, Joint::Neck), 0);
        let mut on_legs = 0;
        for (leg, &(is_fore, _)) in LEGS.iter().enumerate() {
            assert_eq!(count(Kind::Hoof, Joint::Leg(leg)), 1);
            assert_eq!(count(Kind::Body, Joint::Leg(leg)), if is_fore { 6 } else { 7 });
            on_legs += parts.iter().filter(|part| part.joint == Joint::Leg(leg)).count();
            let last = parts
                .iter()
                .filter(|part| part.kind != Kind::Hair && part.joint == Joint::Leg(leg))
                .last()
                .unwrap();
            assert_eq!(last.kind, Kind::Hoof);
        }
        assert_eq!(parts.len(), 6 + 11 + 7 + 5 + on_legs);
    }

    /// The far side is the near side turned over: the legs of a pair and
    /// the two ears differ only in which side they are on, and the trunk
    /// is on the middle.
    #[test]
    fn the_two_sides_are_the_same_horse() {
        let frame = HorseShape::load().base(1.0);
        let parts = frame.parts();
        let mirrored = |a: &Part, b: &Part| {
            a.kind == b.kind
                && points(a) == points(b)
                && [a.from, a.to]
                    .iter()
                    .zip([b.from, b.to])
                    .all(|(p, q)| p.z == -q.z && p.width == q.width)
        };
        for (near, far) in [(0, 1), (2, 3)] {
            let of = |leg: usize| parts.iter().filter(move |part| part.joint == Joint::Leg(leg));
            assert_eq!(of(near).count(), of(far).count());
            assert!(of(near).zip(of(far)).all(|(a, b)| mirrored(a, b) && a.from.z > 0.0));
            let (a, b) = (frame.leg_joint(near), frame.leg_joint(far));
            assert_eq!((a[0], a[1], a[2]), (b[0], b[1], -b[2]));
        }
        // The only body off the middle of the neck and head is the ears.
        let ears: Vec<&Part> = parts
            .iter()
            .filter(|part| part.kind == Kind::Body && part.joint == Joint::Neck)
            .filter(|part| part.from.z != 0.0)
            .collect();
        assert_eq!(ears.len(), 4);
        assert!(mirrored(ears[0], ears[2]) && mirrored(ears[1], ears[3]));
        assert!(parts
            .iter()
            .filter(|part| part.kind == Kind::Body && part.joint == Joint::Trunk)
            .all(|part| part.from.z == 0.0 && part.to.z == 0.0));
    }

    /// A leg's sections from the top down, hair left out: the height of
    /// each and where it stands.
    fn leg_sections(parts: &[Part], leg: usize) -> Vec<(f32, f32)> {
        let pieces: Vec<&Part> = parts
            .iter()
            .filter(|part| part.kind != Kind::Hair && part.joint == Joint::Leg(leg))
            .collect();
        let at = |section: &Section| (section.top[1], (section.top[0] + section.bottom[0]) / 2.0);
        let mut sections = vec![at(&pieces[0].from)];
        sections.extend(pieces.iter().map(|part| at(&part.to)));
        sections
    }

    /// A leg is level sections at its landmarks, from the top down. A
    /// foreleg is straight as far as the fetlock; a hind leg runs back from
    /// the stifle to the hock and down from there.
    #[test]
    fn a_foreleg_is_straight_and_a_hind_leg_is_angled() {
        let shape = HorseShape::load();
        let mut rng = Rng::new(255);
        for round in 0..300 {
            let frame = match round {
                0 => shape.base(1.0),
                _ => shape.varied(1.0, &mut || rng.unit()),
            };
            let parts = frame.parts();
            for (leg, &(is_fore, _)) in LEGS.iter().enumerate() {
                let sections = leg_sections(&parts, leg);
                assert!(sections.windows(2).all(|pair| pair[0].0 > pair[1].0), "{sections:?}");
                let stands = |y: f32| {
                    sections.iter().find(|section| close(section.0, y, 1.0)).map(|section| section.1)
                };
                assert!(stands(frame.fetlock).is_some() && stands(frame.hoof_top).is_some());
                if is_fore {
                    assert!(stands(frame.knee).is_some());
                    let line = sections[0].1;
                    assert!(sections
                        .iter()
                        .filter(|section| section.0 > frame.fetlock - 1e-4)
                        .all(|section| close(section.1, line, 1.0)));
                } else {
                    let (stifle, hock) = (sections[1].1, stands(frame.hock).unwrap());
                    assert!(hock < stifle - frame.bone, "a hock at {hock}, a stifle at {stifle}");
                    assert!(sections
                        .iter()
                        .filter(|section| section.0 < frame.hock)
                        .all(|section| (section.1 - hock).abs() < frame.bone));
                }
            }
        }
    }
}
