//! Dev aid: when `IDLE_SHOT=<path.png>` is set, save one screenshot of the
//! primary window at `IDLE_SHOT_AT` seconds (default 3) and then exit. Lets a
//! visual change be checked without anyone at the keyboard (see
//! procedures/build-and-run.txt).
//!
//! `IDLE_TOUR=<folder>` walks the menus instead: it presses its way through
//! every place and tab, saves a numbered PNG of each into the folder, and
//! exits. The presses are the same `Activate` a click sends.

use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    ui_widgets::Activate,
};

use crate::game::Game;
use crate::game::menu::shell::{HorseButton, PlaceButton, Places, SlotRow, TabButton};

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
        if let Ok(folder) = std::env::var("IDLE_TOUR") {
            app.insert_resource(Tour {
                folder,
                steps: Vec::new(),
                next: 0,
                wait: 1.5,
                shots: 0,
            })
            .add_systems(Startup, plan_tour)
            .add_systems(Update, tour);
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

enum Step {
    Place(usize),
    Tab(usize, usize),
    Horse(usize),
    Slot(usize),
    /// Press a place and stop the scene part of the way through its move.
    PlaceMidway(usize),
    Wait(f32),
    /// Send the party off, and stop the scene part of the way through.
    SetOut,
    /// Wait for the party to come back to town.
    Town,
    Shot(String),
}

#[derive(Resource)]
struct Tour {
    folder: String,
    steps: Vec<Step>,
    next: usize,
    wait: f32,
    shots: u32,
}

/// Seconds for a press to show before the picture is taken: longer than the
/// scene takes to move between its layouts.
const SETTLE: f32 = 0.7;
/// Seconds after a press at which the scene is still on its way.
const MIDWAY: f32 = 0.2;

fn plan_tour(mut tour: ResMut<Tour>, places: Res<Places>) {
    let mut steps = vec![
        Step::Shot("town opens on horses".into()),
        Step::Horse(1),
        Step::Shot("second horse selected".into()),
        Step::Slot(0),
        Step::Shot("side panel open".into()),
    ];
    for (p, place) in places.places.iter().enumerate() {
        steps.push(Step::Place(p));
        if place.tabs.is_empty() {
            steps.push(Step::Shot(place.name.clone()));
        }
        // The first place is left on its second tab, to come back to.
        let mut order: Vec<usize> = (0..place.tabs.len()).collect();
        if p == 0 {
            order[1..].reverse();
        }
        for t in order {
            steps.push(Step::Tab(p, t));
            steps.push(Step::Shot(format!("{} {}", place.name, place.tabs[t].name)));
        }
    }
    steps.extend([
        Step::Place(0),
        Step::Shot("back on horses as it was left".into()),
        Step::Tab(0, 0),
        Step::Shot("side panel still open on its tab".into()),
        Step::Place(places.places.len() - 1),
        Step::Tab(places.places.len() - 1, 0),
        Step::SetOut,
        Step::Shot("setting out midway".into()),
        Step::Wait(SETTLE),
        Step::Shot("on the road the scene expands".into()),
        Step::Wait(5.0),
        Step::Shot("a fight in the road view".into()),
        Step::Horse(2),
        Step::PlaceMidway(0),
        Step::Shot("opening a place on the road midway".into()),
        Step::Wait(SETTLE),
        Step::Shot("on the road horses".into()),
        Step::Place(places.places.len() - 1),
        Step::Shot("on the road journal".into()),
        Step::PlaceMidway(places.places.len() - 1),
        Step::Shot("closing it midway".into()),
        Step::Wait(SETTLE),
        Step::Shot("closed and the scene expanded again".into()),
        Step::Town,
        Step::Wait(SETTLE),
        Step::Shot("back in town with the report".into()),
    ]);
    tour.steps = steps;
}

fn tour(
    mut commands: Commands,
    time: Res<Time>,
    mut tour: ResMut<Tour>,
    mut game: ResMut<Game>,
    places: Query<(Entity, &PlaceButton)>,
    tabs: Query<(Entity, &TabButton)>,
    horses: Query<(Entity, &HorseButton)>,
    slots: Query<(Entity, &SlotRow)>,
    mut exit: MessageWriter<AppExit>,
) {
    tour.wait -= time.delta_secs();
    if tour.wait > 0.0 {
        return;
    }
    let Some(step) = tour.steps.get(tour.next) else {
        exit.write(AppExit::Success);
        return;
    };
    if matches!(step, Step::Town) && !game.in_town() {
        return;
    }
    // The party leaves when the tour says so, not in the middle of it.
    match step {
        Step::SetOut => game.set_out(),
        _ => game.hold_in_town(),
    }
    let pressed = match step {
        Step::Place(p) | Step::PlaceMidway(p) => {
            places.iter().find(|(_, b)| b.0 == *p).map(|(e, _)| e)
        }
        Step::Tab(p, t) => tabs
            .iter()
            .find(|(_, b)| b.place == *p && b.tab == *t)
            .map(|(e, _)| e),
        Step::Horse(h) => horses.iter().find(|(_, b)| b.0 == *h).map(|(e, _)| e),
        Step::Slot(s) => slots.iter().find(|(_, b)| b.slot == *s).map(|(e, _)| e),
        Step::Wait(_) | Step::SetOut | Step::Town | Step::Shot(_) => None,
    };
    if let Some(entity) = pressed {
        commands.trigger(Activate { entity });
    }
    let mut wait = SETTLE;
    match step {
        Step::Wait(secs) => wait = *secs,
        Step::PlaceMidway(_) | Step::SetOut => wait = MIDWAY,
        Step::Shot(name) => {
            let path = format!(
                "{}/{:02}-{}.png",
                tour.folder,
                tour.shots,
                name.to_lowercase().replace(' ', "-")
            );
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));
            tour.shots += 1;
        }
        _ => {}
    }
    tour.wait = wait;
    tour.next += 1;
}
