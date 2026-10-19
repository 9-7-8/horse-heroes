// A released build opens no console window behind the game.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        RenderPlugin,
        settings::{MemoryHints, WgpuSettings},
    },
    window::{WindowResizeConstraints, WindowResolution},
};

mod crash_log;
mod dev_screenshot;
mod fx;
mod game;
mod render_errors;
mod sim;
mod soak;

fn main() {
    // `--soak-report FOLDER` reads a soak run's logs and opens no window.
    if let Some(folder) = soak::report_folder() {
        std::process::exit(soak::write_report(&folder));
    }
    // The window cannot be made smaller than the menus need (#205).
    let layout = game::menu::tokens::Tokens::load().layout;
    let mut window = Window {
        title: "Horse Heroes".into(),
        resize_constraints: WindowResizeConstraints {
            min_width: layout.min_width,
            min_height: layout.min_height,
            ..default()
        },
        ..default()
    };
    if let Some((width, height)) = window_size() {
        window.resolution = WindowResolution::new(width, height);
    }
    // A panic and the log are written beside the exe (#364).
    let seed = seed();
    crash_log::install(seed);
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .set(LogPlugin {
                    custom_layer: crash_log::file_layer,
                    ..default()
                })
                // The GPU's memory is taken in small blocks, not wgpu's
                // default of 128 to 256 MB each: flat colours need little of
                // it, and a big block is held whole however little is in it
                // (#375, docs/memory.md).
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        memory_hints: MemoryHints::MemoryUsage,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        .add_plugins(crash_log::CrashTestPlugin)
        .add_plugins(render_errors::RenderErrorsPlugin)
        .add_plugins(game::GamePlugin {
            seed,
            bare: std::env::args().any(|arg| arg == "--bare"),
            keep_going: std::env::args().any(|arg| arg == "--keep-going"),
            cargo: cargo(),
            worn: std::env::args().any(|arg| arg == "--worn"),
            // A new game starts on the road (#292); the tour and the walk
            // begin with a town visit.
            town: std::env::args().any(|arg| arg == "--town")
                || std::env::var_os("IDLE_TOUR").is_some()
                || std::env::var_os("IDLE_WALK").is_some(),
        })
        .add_plugins(game::menu::MenuPlugin {
            show_parts: std::env::args().any(|arg| arg == "--parts"),
            hint: std::env::args().any(|arg| arg == "--hint"),
            // A lesson's pop-up stops the game (#353), except in the soak,
            // which nobody is at.
            pause: std::env::var_os("IDLE_SOAK").is_none(),
        })
        .add_plugins(fx::FxPlugin {
            burst: std::env::args().any(|arg| arg == "--burst"),
            pickup: std::env::args().any(|arg| arg == "--pickup"),
            burst_at: burst_at(),
            mute: std::env::args().any(|arg| arg == "--mute"),
        })
        .add_plugins(dev_screenshot::DevScreenshotPlugin)
        .add_plugins(soak::SoakPlugin)
        .run();
}

/// `--seed N` fixes the first trail (later ones count up from it), so a run
/// can be reproduced. Without it the seed comes from the clock.
fn seed() -> u64 {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--seed" {
            match args.next().and_then(|value| value.parse().ok()) {
                Some(seed) => return seed,
                None => eprintln!("--seed needs a whole number; using the clock instead"),
            }
        }
    }
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs())
}

/// `--cargo N` starts the game with N pieces carried, rolled from the seed.
/// A dev aid, for looking at a cargo that holds many pieces (#352).
fn cargo() -> usize {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--cargo" {
            return args.next().and_then(|value| value.parse().ok()).unwrap_or(0);
        }
    }
    0
}

/// `--burst-at SECONDS` moves the test burst of `--burst` and `--pickup` to
/// that long after launch. A dev aid, for throwing it on the road.
fn burst_at() -> Option<f32> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--burst-at" {
            return args.next()?.parse().ok();
        }
    }
    None
}

/// `--window WIDTHxHEIGHT` opens the window at that size, in the logical
/// pixels the menu sizes are written in.
/// A dev aid, for looking at the layout at the smallest size and at odd ones.
fn window_size() -> Option<(u32, u32)> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--window" {
            let size = args.next()?;
            let (width, height) = size.split_once('x')?;
            return Some((width.parse().ok()?, height.parse().ok()?));
        }
    }
    None
}
