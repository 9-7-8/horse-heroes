//! Dev aid: when `IDLE_SHOT=<path.png>` is set, save one screenshot of the
//! primary window at `IDLE_SHOT_AT` seconds (default 3) and then exit. Lets a
//! visual change be checked without anyone at the keyboard (see
//! procedures/build-and-run.txt).

use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

#[derive(Resource)]
struct Shot {
    path: String,
    at_secs: f32,
}

pub struct DevScreenshotPlugin;

impl Plugin for DevScreenshotPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(path) = std::env::var("IDLE_SHOT") {
            let at_secs = std::env::var("IDLE_SHOT_AT")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(3.0);
            app.insert_resource(Shot { path, at_secs })
                .add_systems(Update, shoot_then_exit);
        }
    }
}

fn shoot_then_exit(
    mut commands: Commands,
    time: Res<Time>,
    shot: Res<Shot>,
    mut taken: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    if !*taken && t >= shot.at_secs {
        *taken = true;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(shot.path.clone()));
    }
    if t >= shot.at_secs + 2.0 {
        exit.write(AppExit::Success);
    }
}
