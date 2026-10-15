//! A dragged piece dangling from the pointer (#363): a damped pendulum hung
//! from the pointer and swung by the pointer's changes of speed, so a quick
//! move swings it, a stop lets it settle and a slow move barely tilts it.
//! Plain Rust, stepped in fixed steps, so the same moves give the same swing
//! at any frame rate. How it is shown: docs/item-card.md, "Dragging".

use serde::Deserialize;

/// The swing's tuning, from `swing` in `assets/data/menu.ron`.
#[derive(Clone, Debug, Deserialize)]
pub struct Swing {
    /// The side of the piece in the hand, in pixels.
    pub side: f32,
    /// How far above the piece's top edge the point it hangs by is.
    pub hang: f32,
    /// What pulls it back to hanging straight down, in pixels a second
    /// squared.
    pub gravity: f32,
    /// How much of the pointer's change of speed reaches the piece.
    pub drive: f32,
    /// How quickly a swing dies away, a second.
    pub damping: f32,
    /// The furthest it swings from straight down, in degrees.
    pub limit: f32,
    /// The fixed step it is worked out in, in seconds.
    pub step: f32,
    /// How long a piece let go takes to fly to its cell, in seconds.
    pub fly: f32,
}

impl Swing {
    /// From the point it hangs by to the piece's middle.
    pub fn length(&self) -> f32 {
        self.hang + self.side / 2.0
    }
}

/// The piece's swing as it hangs from the pointer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Dangle {
    /// Radians from hanging straight down; positive is the piece out to the
    /// right of the pointer.
    pub angle: f32,
    /// How fast the angle changes, radians a second.
    spin: f32,
    /// The pointer when last seen, and its speed then.
    at: Option<(f32, f32)>,
    speed: (f32, f32),
    /// Time seen and not yet stepped.
    owed: f32,
}

impl Dangle {
    /// The pointer is at `pointer` (screen pixels, y down) after `dt`
    /// seconds. A change in its speed kicks the piece the other way, as the
    /// point it hangs by is jerked; then the swing is stepped through `dt`.
    pub fn update(&mut self, swing: &Swing, pointer: (f32, f32), dt: f32) {
        if dt <= 0.0 {
            return;
        }
        let speed = self
            .at
            .map_or((0.0, 0.0), |at| ((pointer.0 - at.0) / dt, (pointer.1 - at.1) / dt));
        self.at = Some(pointer);
        let jerk = (
            (speed.0 - self.speed.0) * swing.drive,
            (speed.1 - self.speed.1) * swing.drive,
        );
        self.speed = speed;
        let length = swing.length();
        // Seen from the point it hangs by, a jerk is a push the other way: a
        // jerk sideways swings it back, a jerk downwards lifts it.
        self.spin += (-jerk.0 * self.angle.cos() + jerk.1 * self.angle.sin()) / length;
        let limit = swing.limit.to_radians();
        self.owed += dt;
        while self.owed >= swing.step {
            self.owed -= swing.step;
            let pull = -(swing.gravity / length) * self.angle.sin() - swing.damping * self.spin;
            self.spin += pull * swing.step;
            self.angle += self.spin * swing.step;
            if self.angle.abs() > limit {
                self.angle = limit.copysign(self.angle);
                self.spin = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::menu::tokens::Tokens;

    /// The pointer moves right at `speed` pixels a second until `stop`, then
    /// rests; the swing is sampled at `rate` frames a second, and the angle
    /// read at each of `at`.
    fn swing_of(speed: f32, stop: f32, rate: f32, at: &[f32]) -> (Vec<f32>, f32) {
        let swing = Tokens::load().swing;
        let mut dangle = Dangle::default();
        let mut seen = Vec::new();
        let mut most = 0.0f32;
        let frames = (at.iter().cloned().fold(0.0, f32::max) * rate).round() as usize;
        for frame in 0..=frames {
            let t = frame as f32 / rate;
            let x = speed * t.min(stop);
            dangle.update(&swing, (x, 0.0), 1.0 / rate);
            most = most.max(dangle.angle.abs());
            if at.iter().any(|a| (a * rate).round() as usize == frame) {
                seen.push(dangle.angle);
            }
        }
        (seen, most)
    }

    /// A quick move swings it back, against the move, and once the pointer
    /// stops it settles hanging straight down.
    #[test]
    fn it_settles_from_a_push() {
        let (seen, most) = swing_of(900.0, 0.5, 60.0, &[0.2, 3.0]);
        assert!(seen[0] < -0.1, "it trails the pointer: {}", seen[0]);
        assert!(most > 0.3, "a quick move swings it: {most}");
        assert!(seen[1].abs() < 0.01, "it settles: {}", seen[1]);
    }

    /// A slow move barely tilts it.
    #[test]
    fn a_slow_move_barely_tilts_it() {
        let (_, most) = swing_of(40.0, 2.0, 60.0, &[2.5]);
        assert!(most.to_degrees() < 4.0, "{}", most.to_degrees());
    }

    /// The same moves give the same swing at 30 and 144 frames a second.
    #[test]
    fn the_swing_is_the_same_at_any_frame_rate() {
        let at = [1.0 / 6.0, 0.5, 2.0 / 3.0, 1.0, 1.5];
        let (slow, _) = swing_of(900.0, 0.5, 30.0, &at);
        let (fast, _) = swing_of(900.0, 0.5, 144.0, &at);
        assert_eq!(slow.len(), at.len());
        for ((a, b), t) in slow.iter().zip(&fast).zip(at) {
            assert!((a - b).abs() < 0.03, "at {t} s: {a} at 30, {b} at 144");
        }
    }
}
