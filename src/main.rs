use bevy::prelude::*;

mod dev_screenshot;
mod game;
mod sim;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Horse Heroes".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(game::GamePlugin { seed: seed() })
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
