//! A look of its own for every piece of gear (#361): a pattern and a
//! decoration over the rarity's colour, each in an ink of its own, rolled
//! from the lists in `assets/data/looks.ron` and kept with the piece. Here it
//! is rolled, and painted into a grid of layers; `game` turns a grid into
//! pixels. Plain numbers, so a look is tested without drawing it. How it
//! works: docs/item-card.md, "The picture".

use super::data::{LookTable, Mark, PatternDef};
use super::rng::Rng;

/// What a piece looks like, by place in `looks.ron`'s lists.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Look {
    pub pattern: usize,
    pub decoration: usize,
    pub pattern_ink: usize,
    pub decoration_ink: usize,
}

/// Mixed into the piece's own roll, so a look is its own stream.
const LOOK_SALT: u64 = 0x100C_5EED;

impl Look {
    /// The look of a piece rolled at `rng`'s state. It does not move `rng`
    /// (it borrows it unchanged), so every other roll, and every seeded time
    /// in the docs, is what it was before pieces had looks.
    pub fn roll(table: &LookTable, rng: &Rng) -> Look {
        let mut rng = Rng::new(rng.clone().next_u64() ^ LOOK_SALT);
        let inks = table.inks.len() as u32;
        let pattern_ink = rng.below(inks) as usize;
        // The decoration's ink is any but the pattern's.
        let decoration_ink = (pattern_ink + 1 + rng.below(inks - 1) as usize) % inks as usize;
        Look {
            pattern: rng.below(table.patterns.len() as u32) as usize,
            decoration: rng.below(table.decorations.len() as u32) as usize,
            pattern_ink,
            decoration_ink,
        }
    }
}

/// What a pixel of a painted look shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    /// Outside the piece's shape: nothing.
    Outside,
    /// The rarity's colour.
    Ground,
    Pattern,
    Decoration,
}

/// Whether a point, from a mark's middle in units of the mark's size and
/// not yet turned, is on the mark.
fn on_mark(mark: Mark, size: (f32, f32), turn: f32, x: f32, y: f32) -> bool {
    let (sin, cos) = (-turn.to_radians()).sin_cos();
    let (x, y) = (x * cos - y * sin, x * sin + y * cos);
    let (a, b) = (x / (size.0 / 2.0), y / (size.1 / 2.0));
    match mark {
        Mark::Bar => a.abs() <= 1.0 && b.abs() <= 1.0,
        Mark::Disc => a * a + b * b <= 1.0,
        Mark::Ring(band) => {
            let r = (a * a + b * b).sqrt();
            r <= 1.0 && r >= 1.0 - band
        }
    }
}

/// Whether a point, in units of the shorter side, is on a pattern's marks.
fn on_pattern(pattern: &PatternDef, u: f32, v: f32) -> bool {
    let (sin, cos) = (-pattern.turn.to_radians()).sin_cos();
    let (u, v) = (u * cos - v * sin, u * sin + v * cos);
    let (sx, sy) = pattern.step;
    let row = (v / sy).floor();
    let shift = if row.rem_euclid(2.0) == 1.0 { pattern.stagger * sx } else { 0.0 };
    let column = ((u - shift) / sx).floor();
    let (cx, cy) = ((column + 0.5) * sx + shift, (row + 0.5) * sy);
    on_mark(pattern.mark, pattern.size, pattern.spin, u - cx, v - cy)
}

/// Whether a pixel's middle is inside a rectangle `w` by `h` with its
/// corners rounded by `round` (top left, top right, bottom right, bottom
/// left), all in pixels.
fn inside(w: f32, h: f32, round: [f32; 4], x: f32, y: f32) -> bool {
    let corners = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];
    corners.iter().zip(round).all(|(&(cx, cy), r)| {
        let r = r.min(w / 2.0).min(h / 2.0);
        // The corner's circle's middle, and whether the point is in the
        // square it rounds.
        let (mx, my) = (if cx > 0.0 { cx - r } else { r }, if cy > 0.0 { cy - r } else { r });
        let in_corner = (x - mx) * (cx - mx) > 0.0 && (y - my) * (cy - my) > 0.0;
        !in_corner || (x - mx).hypot(y - my) <= r
    })
}

/// Paint a look into `w` by `h` pixels, row by row from the top left: the
/// piece's shape with its corners rounded by `round` (pixels), the pattern
/// over it, and the decoration where `place` says (as `looks.ron` has it).
pub fn paint(
    table: &LookTable,
    look: Look,
    (w, h): (usize, usize),
    round: [f32; 4],
    place: (f32, f32, f32),
) -> Vec<Layer> {
    let unit = w.min(h).max(1) as f32;
    let pattern = &table.patterns[look.pattern % table.patterns.len()];
    let decoration = &table.decorations[look.decoration % table.decorations.len()];
    let (dx, dy, side) = (place.0 * w as f32, place.1 * h as f32, place.2 * unit / 2.0);
    let mut out = Vec::with_capacity(w * h);
    for row in 0..h {
        for column in 0..w {
            let (x, y) = (column as f32 + 0.5, row as f32 + 0.5);
            let layer = if !inside(w as f32, h as f32, round, x, y) {
                Layer::Outside
            } else if decoration.marks.iter().any(|mark| {
                let (bx, by) = ((x - dx) / side - mark.at.0, (y - dy) / side - mark.at.1);
                on_mark(mark.mark, mark.size, mark.turn, bx, by)
            }) {
                Layer::Decoration
            } else if on_pattern(pattern, x / unit, y / unit) {
                Layer::Pattern
            } else {
                Layer::Ground
            };
            out.push(layer);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::data::{Data, Slot};
    use crate::sim::loot::{Odds, roll_piece};

    fn count(painted: &[Layer], layer: Layer) -> usize {
        painted.iter().filter(|&&l| l == layer).count()
    }

    /// The lists are there to roll from, each pattern and decoration leaves
    /// a mark of its own, and every ink is a colour.
    #[test]
    fn the_looks_file_holds_together() {
        let table = Data::load().looks;
        assert!(table.inks.len() >= 2 && !table.patterns.is_empty() && !table.decorations.is_empty());
        for ink in &table.inks {
            assert!(ink.len() == 6 && u32::from_str_radix(ink, 16).is_ok(), "{ink}");
        }
        let painted = |pattern, decoration| {
            let look = Look { pattern, decoration, pattern_ink: 0, decoration_ink: 1 };
            paint(&table, look, (64, 64), [0.0; 4], table.picture_decoration)
        };
        for (i, _) in table.patterns.iter().enumerate() {
            let a = painted(i, 0);
            assert!(count(&a, Layer::Pattern) > 64 * 64 / 20, "pattern {i} barely shows");
            assert!(count(&a, Layer::Ground) > 64 * 64 / 5, "pattern {i} covers the ground");
            for j in i + 1..table.patterns.len() {
                assert_ne!(a, painted(j, 0), "patterns {i} and {j}");
            }
        }
        for (i, _) in table.decorations.iter().enumerate() {
            let a = painted(0, i);
            assert!(count(&a, Layer::Decoration) > 64, "decoration {i} barely shows");
            for j in i + 1..table.decorations.len() {
                assert_ne!(a, painted(0, j), "decorations {i} and {j}");
            }
        }
    }

    /// The same seed rolls the same look; the two inks always differ; and
    /// two pieces of one slot and rarity are told apart by their look in
    /// most rolls.
    #[test]
    fn a_look_is_the_seeds_and_tells_pieces_apart() {
        let data = Data::load();
        let roll = |seed: u64| {
            let mut rng = Rng::new(seed);
            (0..40).map(|_| roll_piece(&data, &mut rng, Odds::Kill, 0)).collect::<Vec<_>>()
        };
        assert_eq!(roll(361), roll(361));

        let mut rng = Rng::new(17);
        let mut by_kind: Vec<Vec<Look>> = vec![Vec::new(); 3 * data.gear.rarities.len()];
        for _ in 0..20_000 {
            let gear = roll_piece(&data, &mut rng, Odds::Kill, 0);
            assert_ne!(gear.look.pattern_ink, gear.look.decoration_ink);
            by_kind[gear.slot.index() * data.gear.rarities.len() + gear.rarity].push(gear.look);
        }
        let (mut pairs, mut apart) = (0, 0);
        for looks in by_kind.iter().filter(|looks| looks.len() > 1) {
            for pair in looks.windows(2) {
                pairs += 1;
                let same = |l: &Look| (l.pattern, l.decoration);
                apart += (same(&pair[0]) != same(&pair[1])) as usize;
            }
        }
        assert!(pairs > 1000);
        assert!(apart * 10 >= pairs * 9, "{apart} of {pairs} told apart");
        // Every slot is rolled, so every slot has looks.
        assert!(Slot::ALL.iter().all(|slot| !by_kind[slot.index() * data.gear.rarities.len()].is_empty()));
    }

    /// The shape's rounded corners are outside it, and its middle inside.
    #[test]
    fn a_rounded_shape_leaves_its_corners_out() {
        let table = Data::load().looks;
        let painted = paint(&table, Look::default(), (40, 20), [8.0, 0.0, 8.0, 0.0], (0.5, 0.5, 0.1));
        let at = |x: usize, y: usize| painted[y * 40 + x];
        assert_eq!(at(0, 0), Layer::Outside);
        assert_eq!(at(39, 19), Layer::Outside);
        assert_ne!(at(39, 0), Layer::Outside);
        assert_ne!(at(0, 19), Layer::Outside);
        assert_ne!(at(20, 10), Layer::Outside);
    }
}
