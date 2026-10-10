use bevy::{
    prelude::*,
    window::{WindowResizeConstraints, WindowResolution},
};

mod dev_screenshot;
mod fx;
mod game;
mod sim;

fn main() {
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
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_plugins(game::GamePlugin { seed: seed() })
        .add_plugins(game::menu::MenuPlugin {
            show_parts: std::env::args().any(|arg| arg == "--parts"),
        })
        .add_plugins(fx::FxPlugin {
            burst: std::env::args().any(|arg| arg == "--burst"),
            pickup: std::env::args().any(|arg| arg == "--pickup"),
            burst_at: burst_at(),
        })
        .add_plugins(dev_screenshot::DevScreenshotPlugin)
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
