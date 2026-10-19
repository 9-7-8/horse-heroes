//! What the game does when the GPU fails it (#374). Bevy's own handler quits
//! on any render error, with no panic, so the crash log never heard of it.
//!
//! - Out of GPU memory: the picture stops for [`PAUSE_SECS`] while the game
//!   plays on, every image is sent to the GPU again, and drawing is tried
//!   once more. Other programs often give memory back, so this is often
//!   enough. A render error soon after one of these is its aftermath (a
//!   texture that was never made) and is treated the same way.
//! - More than [`MOST_IN_A_MINUTE`] of them in a minute, or any other render
//!   error (a lost device, a bug in how the GPU is used): `crash.log` says
//!   so, and the game quits as Bevy's would.
//!
//! The pause is long so that a failure that keeps coming cannot flash the
//! screen: at most one try every [`PAUSE_SECS`].

use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::render::error_handler::{ErrorType, RenderError, RenderErrorHandler, RenderErrorPolicy};

use crate::crash_log;

/// How long the picture stops after an out-of-memory error.
pub const PAUSE_SECS: f32 = 2.0;
/// Out-of-memory errors (and their aftermath) survived in a minute, at most.
pub const MOST_IN_A_MINUTE: usize = 8;
const MINUTE: Duration = Duration::from_secs(60);

pub struct RenderErrorsPlugin;

impl Plugin for RenderErrorsPlugin {
    fn build(&self, app: &mut App) {
        // Replaces the one RenderPlugin put in, so it is added after it.
        app.insert_resource(RenderErrorHandler(handle))
            .init_resource::<Trouble>();
        // A dev aid, to see both ways through in the running game: a real
        // render error, made on purpose a few seconds in. `quit` is one with
        // no out-of-memory before it; `oom` counts one as just met, which
        // cannot be made on purpose, so the error is its aftermath.
        match std::env::var("HORSE_HEROES_RENDER_ERROR").as_deref() {
            Ok("oom") => {
                app.insert_resource(Trouble {
                    last_oom: Some(Instant::now()),
                    ..default()
                });
            }
            Ok("quit") => {}
            _ => return,
        }
        app.add_systems(Update, make_a_render_error);
    }
}

/// Makes a buffer wgpu refuses, once, five seconds in: a validation error.
fn make_a_render_error(time: Res<Time<Real>>,device: Res<bevy::render::renderer::RenderDevice>, mut done: Local<bool>) {
    if *done || time.elapsed_secs() < 5.0 {
        return;
    }
    *done = true;
    warn!("HORSE_HEROES_RENDER_ERROR: making a render error");
    device.create_buffer(&bevy::render::render_resource::BufferDescriptor {
        label: Some("render error on purpose"),
        size: 16,
        usage: bevy::render::render_resource::BufferUsages::MAP_READ | bevy::render::render_resource::BufferUsages::VERTEX,
        mapped_at_creation: false,
    });
}

/// The render errors met lately, and what is being done about them.
#[derive(Resource, Default, Debug)]
pub struct Trouble {
    /// When each error was met, the last minute's.
    recent: Vec<Instant>,
    /// When the last out-of-memory error was met.
    last_oom: Option<Instant>,
    /// The picture is stopped until then.
    paused_until: Option<Instant>,
    quitting: bool,
}

/// What to do about a render error, or about the one still standing.
#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    /// A new error to live through: stop the picture for a while.
    Pause,
    /// Still stopped.
    Wait,
    /// The pause is over: send the images again and draw.
    Retry,
    /// Write it down and quit.
    Quit,
    /// Quitting is under way.
    Quitting,
}

impl Trouble {
    /// Bevy asks every frame while the picture is stopped, with the same
    /// error; a new error comes only after a frame was let through.
    pub fn decide(&mut self, kind: ErrorType, now: Instant) -> Decision {
        if self.quitting {
            return Decision::Quitting;
        }
        if let Some(until) = self.paused_until {
            if now < until {
                return Decision::Wait;
            }
            self.paused_until = None;
            return Decision::Retry;
        }
        self.recent.retain(|at| now.duration_since(*at) < MINUTE);
        self.recent.push(now);
        let aftermath = self.last_oom.is_some_and(|at| now.duration_since(at) < MINUTE);
        let survivable = match kind {
            ErrorType::OutOfMemory => {
                self.last_oom = Some(now);
                true
            }
            ErrorType::Validation | ErrorType::Internal => aftermath,
            ErrorType::DeviceLost => false,
        };
        if survivable && self.recent.len() <= MOST_IN_A_MINUTE {
            self.paused_until = Some(now + Duration::from_secs_f32(PAUSE_SECS));
            Decision::Pause
        } else {
            self.quitting = true;
            Decision::Quit
        }
    }

    /// Errors met in the last minute, this one included.
    pub fn in_the_last_minute(&self) -> usize {
        self.recent.len()
    }
}

fn handle(error: &RenderError, main_world: &mut World, _render_world: &mut World) -> RenderErrorPolicy {
    let now = Instant::now();
    let decision = main_world
        .get_resource_or_insert_with(Trouble::default)
        .decide(error.ty, now);
    match decision {
        Decision::Wait | Decision::Quitting => RenderErrorPolicy::StopRendering,
        Decision::Retry => {
            // A texture that could not be made is broken for good unless it
            // is made again: every image goes to the GPU once more.
            if let Some(mut images) = main_world.get_resource_mut::<Assets<Image>>() {
                for _ in images.iter_mut() {}
            }
            info!("Drawing again after a render error");
            RenderErrorPolicy::Ignore
        }
        Decision::Pause => {
            let count = main_world.resource::<Trouble>().in_the_last_minute();
            warn!(
                "Render error {:?}; the picture stops for {PAUSE_SECS} s and is tried again ({count} of {MOST_IN_A_MINUTE} in a minute)",
                error.ty
            );
            // The first of a run of them is written down; the rest are in the log.
            if count == 1 {
                write(error, main_world, &format!("stopped the picture for {PAUSE_SECS} s and went on"));
            }
            RenderErrorPolicy::StopRendering
        }
        Decision::Quit => {
            let count = main_world.resource::<Trouble>().in_the_last_minute();
            error!("Render error {:?}, {count} in a minute; quitting", error.ty);
            write(
                error,
                main_world,
                &format!("quit ({count} render errors in the last minute)"),
            );
            main_world.write_message(AppExit::error());
            RenderErrorPolicy::StopRendering
        }
    }
}

fn write(error: &RenderError, world: &World, outcome: &str) {
    let count = |len: Option<usize>| len.unwrap_or(0);
    let assets = crate::soak::asset_fields(
        count(world.get_resource::<Assets<Image>>().map(|assets| assets.len())),
        count(world.get_resource::<Assets<Mesh>>().map(|assets| assets.len())),
        count(world.get_resource::<Assets<StandardMaterial>>().map(|assets| assets.len())),
    )
    .replace('\t', " ");
    let source = error.source.as_ref().map_or_else(String::new, |source| format!("{source:?}"));
    crash_log::render_error(&format!("{:?}", error.ty), &error.description, &source, outcome, &assets);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secs(start: Instant, secs: f32) -> Instant {
        start + Duration::from_secs_f32(secs)
    }

    #[test]
    fn out_of_memory_pauses_then_tries_again() {
        let start = Instant::now();
        let mut trouble = Trouble::default();
        assert_eq!(trouble.decide(ErrorType::OutOfMemory, start), Decision::Pause);
        assert_eq!(trouble.decide(ErrorType::OutOfMemory, secs(start, 1.0)), Decision::Wait);
        assert_eq!(trouble.decide(ErrorType::OutOfMemory, secs(start, PAUSE_SECS)), Decision::Retry);
        // A texture lost to it fails validation on the next frame: survived too.
        assert_eq!(trouble.decide(ErrorType::Validation, secs(start, 2.1)), Decision::Pause);
    }

    #[test]
    fn a_render_error_with_no_out_of_memory_before_it_quits() {
        let start = Instant::now();
        for kind in [ErrorType::Validation, ErrorType::Internal, ErrorType::DeviceLost] {
            let mut trouble = Trouble::default();
            assert_eq!(trouble.decide(kind, start), Decision::Quit, "{kind:?}");
            assert_eq!(trouble.decide(kind, secs(start, 0.1)), Decision::Quitting);
        }
    }

    #[test]
    fn too_many_in_a_minute_quits_and_fewer_spread_out_do_not() {
        let start = Instant::now();
        let mut trouble = Trouble::default();
        let mut t = 0.0;
        for _ in 0..MOST_IN_A_MINUTE {
            assert_eq!(trouble.decide(ErrorType::OutOfMemory, secs(start, t)), Decision::Pause);
            t += PAUSE_SECS;
            assert_eq!(trouble.decide(ErrorType::OutOfMemory, secs(start, t)), Decision::Retry);
        }
        assert_eq!(trouble.decide(ErrorType::OutOfMemory, secs(start, t)), Decision::Quit);

        // One every ten seconds goes on for good.
        let mut trouble = Trouble::default();
        for minute_tenth in 0..60 {
            let at = secs(start, 10.0 * minute_tenth as f32);
            assert_eq!(trouble.decide(ErrorType::OutOfMemory, at), Decision::Pause);
            assert_eq!(trouble.decide(ErrorType::OutOfMemory, secs(at, PAUSE_SECS)), Decision::Retry);
        }
    }

    #[test]
    fn a_validation_error_long_after_out_of_memory_quits() {
        let start = Instant::now();
        let mut trouble = Trouble::default();
        trouble.decide(ErrorType::OutOfMemory, start);
        trouble.decide(ErrorType::OutOfMemory, secs(start, PAUSE_SECS));
        assert_eq!(trouble.decide(ErrorType::Validation, secs(start, 90.0)), Decision::Quit);
    }
}
