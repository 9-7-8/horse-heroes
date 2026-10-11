//! Dev aid: when `IDLE_SHOT=<path.png>` is set, save one screenshot of the
//! primary window at `IDLE_SHOT_AT` seconds (default 3) and then exit. Lets a
//! visual change be checked without anyone at the keyboard (see
//! procedures/build-and-run.txt).
//!
//! `IDLE_TOUR=<folder>` walks the menus instead: it presses its way through
//! every place and tab, saves a numbered PNG of each into the folder, and
//! exits. The presses are the same `Activate` a click sends. It ends with a
//! town visit by keys alone, each key put into Bevy's keyboard state as if
//! it had gone down.
//!
//! `IDLE_WALK=<folder>` walks the click budget (#236): it does each of the
//! common jobs of `assets/data/jobs.ron` that can be done, once with the
//! presses a pointer makes and once by keys, counts the presses, checks in
//! the game that the job was done, and writes the counts to `counts.txt` in
//! the folder (procedures/walk-the-click-budget.txt).

use bevy::{
    input::InputSystems,
    input_focus::{
        FocusCause, InputFocus, InputFocusVisible,
        tab_navigation::{NavAction, TabNavigation},
    },
    picking::pointer::{Location, PointerAction, PointerId, PointerInput},
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    ui::UiGlobalTransform,
    ui_widgets::Activate,
    window::{PrimaryWindow, WindowRef},
};
use bevy::camera::NormalizedRenderTarget;
use serde::Deserialize;

use crate::game::menu::shell::{
    CarriedCell, Content, ControlButton, HorseButton, PlaceButton, Places, ReportedCell, Shell,
    SlotRow, TabButton,
};
use crate::game::{Action, Game, Hand};
use crate::sim::data::Element;

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
            app.insert_resource(Tour::new(folder))
                .add_systems(Startup, plan_tour)
                .add_systems(PreUpdate, tour_keys.after(InputSystems))
                .add_systems(Update, tour);
        } else if let Ok(folder) = std::env::var("IDLE_WALK") {
            app.insert_resource(Tour::new(folder))
                .add_systems(Startup, plan_walk)
                .add_systems(PreUpdate, tour_keys.after(InputSystems))
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
    /// A slot of the gear overview: the horse, then the slot.
    Slot(usize, usize),
    /// Press the first control of that name.
    Control(&'static str),
    /// Press a key, as the keyboard would.
    Key(KeyCode),
    /// Give the keyboard focus to the first control of that name, as Tab
    /// would; `None` takes it away again.
    Focus(Option<&'static str>),
    /// Press what has the keyboard focus, as Enter does.
    Enter,
    /// Go round every stop Tab makes and log how many there are, and how
    /// many of them are out of sight.
    TabRound,
    /// Press the carried piece where a place shows it.
    Carried(usize),
    /// Press the piece of the report that is showing, in the report.
    Reported,
    /// Move the pointer onto what the next step names, which that step
    /// then does not press: the pointer is Bevy's own, moved as the mouse
    /// moves it, so what it rests on is hovered as by hand.
    Point,
    /// Move the pointer off the menus, onto the scene.
    PointAway,
    /// Press a place and stop the scene part of the way through its move.
    PlaceMidway(usize),
    Wait(f32),
    /// Send the party off, and stop the scene part of the way through.
    SetOut,
    /// Wait for the party to come back to town.
    Town,
    Shot(String),
    /// Start counting the presses of a job of `jobs.ron`, done this way.
    Job(&'static str, By),
    /// The job is over: stop counting, and check in the game that it was
    /// done.
    Done(Check),
}

/// One of the common jobs, and the most presses it may take (#236), from
/// `assets/data/jobs.ron`.
#[derive(Clone, Debug, Deserialize)]
struct Job {
    name: String,
    /// Where the player is when the count starts.
    from: String,
    mouse: u32,
    keys: u32,
    /// The issue that will make the job possible; it cannot be walked yet.
    #[serde(default)]
    waits_on: Option<u32>,
}

#[derive(Deserialize)]
struct Jobs {
    jobs: Vec<Job>,
}

impl Jobs {
    fn load() -> Vec<Job> {
        let jobs: Jobs = ron::from_str(include_str!("../assets/data/jobs.ron"))
            .unwrap_or_else(|e| panic!("assets/data/jobs.ron: {e}"));
        jobs.jobs
    }
}

/// How a job is walked: with the presses a pointer makes, or by keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum By {
    Mouse,
    Keys,
}

/// What the game must show for a job to count as done.
#[derive(Clone, Copy, Debug)]
enum Check {
    /// The party fights with this element.
    Element(Element),
    /// The party has left town.
    OnTheRoad,
    /// The piece that was in the cargo when the job began is worn.
    Equipped,
    /// This place is open on this tab, at the latest report.
    Reading(usize, usize),
}

/// A job walked: its name, how, the presses it took, and whether the game
/// showed it done.
struct Walked {
    name: &'static str,
    by: By,
    presses: u32,
    done: bool,
}

#[derive(Resource)]
struct Tour {
    folder: String,
    steps: Vec<Step>,
    next: usize,
    wait: f32,
    shots: u32,
    /// The key to put down at the start of the next frame.
    key: Option<KeyCode>,
    /// The key put down last frame, to let go of.
    held: Option<KeyCode>,
    /// The next step points at its control and does not press it.
    pointing: bool,
    /// The job being walked, and how.
    job: Option<(&'static str, By)>,
    /// The presses it has taken so far.
    presses: u32,
    /// A press of the job found nothing in sight to press.
    missed: bool,
    /// Whether a piece was in the cargo when the job began.
    carrying: bool,
    walked: Vec<Walked>,
}

impl Tour {
    fn new(folder: String) -> Tour {
        Tour {
            folder,
            steps: Vec::new(),
            next: 0,
            wait: 1.5,
            shots: 0,
            key: None,
            held: None,
            pointing: false,
            job: None,
            presses: 0,
            missed: false,
            carrying: false,
            walked: Vec::new(),
        }
    }
}

/// Seconds for a press to show before the picture is taken: longer than the
/// scene takes to move between its layouts.
const SETTLE: f32 = 0.7;
/// Seconds after a press at which the scene is still on its way.
const MIDWAY: f32 = 0.2;

fn plan_tour(mut tour: ResMut<Tour>, places: Res<Places>) {
    let mut steps = vec![
        Step::Shot("town opens on horses".into()),
        Step::TabRound,
        Step::Horse(1),
        Step::Shot("second horse selected".into()),
        Step::Slot(0, 0),
        Step::Shot("side panel open".into()),
        Step::Control("Compare"),
        Step::Shot("a control not built says so".into()),
        Step::Slot(0, 0),
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
        Step::Place(places.settings_page()),
        Step::Shot(places.settings.name.clone()),
        Step::Control("Shake"),
        Step::Shot("a setting not built says so".into()),
        Step::Place(0),
        Step::Shot("back on horses as it was left".into()),
        Step::Control("Ice"),
        Step::Shot("ice chosen".into()),
        Step::Control("Fire"),
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
        Step::Focus(Some("Sets")),
        Step::Shot("focus brings a control into view".into()),
        Step::Focus(None),
        Step::Place(places.places.len() - 1),
        Step::Shot("on the road journal".into()),
        Step::PlaceMidway(places.places.len() - 1),
        Step::Shot("closing it midway".into()),
        Step::Wait(SETTLE),
        Step::Shot("closed and the scene expanded again".into()),
        // The marks (#232): the party comes home to a place that shows
        // neither the cargo nor the report, so both are still unseen; the
        // report shows both, and both marks go.
        Step::Place(1),
        Step::Town,
        Step::Wait(SETTLE),
        Step::Shot("back in town two places are marked".into()),
        Step::Place(places.places.len() - 1),
        Step::Shot("back in town with the report".into()),
        // The item card (#228): the drop is opened from the report, and
        // then from the cargo, and it is the same card.
        Step::Reported,
        Step::Shot("the drop's card from the report".into()),
        Step::Key(KeyCode::Escape),
        // A town visit by keys alone (#229): the keys are those printed on
        // the controls in places.ron, so one changed there is changed here.
        Step::Key(KeyCode::KeyH),
        Step::Key(KeyCode::Digit1),
        Step::Shot("by keys the drop waits in the cargo".into()),
        // The Inventory (#308): the drop is in its type's group, carried,
        // beside what the horses wear; its card is pinned from the grid.
        Step::Key(KeyCode::Digit5),
        Step::Shot("inventory with the drop carried".into()),
        Step::Key(KeyCode::ArrowDown),
        Step::Shot("inventory by arrows onto the grid".into()),
        Step::Enter,
        Step::Shot("inventory a piece's card from the grid".into()),
        Step::Key(KeyCode::Escape),
        Step::Focus(None),
        Step::Key(KeyCode::Digit1),
        // The card under the pointer (#228): resting on a piece shows its
        // card beside it, and nothing is pinned.
        Step::Point,
        Step::Carried(0),
        Step::Shot("the pointer on the drop in the cargo".into()),
        Step::PointAway,
        Step::Carried(0),
        Step::Shot("the drop's card from the cargo".into()),
        Step::Key(KeyCode::KeyX),
        Step::Shot("the card against another horse".into()),
        Step::Key(KeyCode::KeyC),
        Step::Key(KeyCode::Escape),
        // The arrow keys: onto the first slot, down to the second, open it,
        // close it, and across to the horses.
        Step::Key(KeyCode::ArrowDown),
        Step::Key(KeyCode::ArrowDown),
        Step::Shot("by arrows down the slots".into()),
        Step::Enter,
        Step::Shot("by arrows a slot opened".into()),
        Step::Key(KeyCode::Escape),
        Step::Key(KeyCode::ArrowLeft),
        Step::Shot("by arrows across to the horses".into()),
        // Past the three horses of the overview.
        Step::Key(KeyCode::ArrowRight),
        Step::Key(KeyCode::ArrowRight),
        Step::Key(KeyCode::ArrowRight),
        Step::Key(KeyCode::ArrowRight),
        Step::Shot("by arrows across to the cargo".into()),
        // Space presses what has the focus, so let go of it before Space
        // is to send the party off.
        Step::Focus(None),
        Step::Key(KeyCode::KeyE),
        Step::Shot("by keys the drop equipped".into()),
        // Worn, it is still in its group of the Inventory, under its horse.
        Step::Key(KeyCode::Digit5),
        Step::Shot("inventory with the drop worn".into()),
        Step::Key(KeyCode::Digit1),
        // The overview (#285): with the first horse selected, open what
        // the third wears on its head and equip it: the two change places.
        Step::Key(KeyCode::KeyZ),
        Step::Point,
        Step::Slot(2, 0),
        Step::Shot("the pointer on another horse's piece".into()),
        // Pressed, its card is pinned beside it and the pointer's goes.
        Step::Slot(2, 0),
        Step::Shot("another horse's piece open".into()),
        Step::PointAway,
        Step::Key(KeyCode::KeyE),
        Step::Shot("swapped onto the selected horse".into()),
        // Undo (#230): the swap is taken back, then the equip, and the
        // card goes back to the cargo with its piece.
        Step::Key(KeyCode::KeyU),
        Step::Shot("by keys the swap undone".into()),
        Step::Key(KeyCode::KeyU),
        Step::Shot("by keys the equip undone".into()),
        Step::Key(KeyCode::KeyU),
        Step::Shot("nothing left to undo".into()),
        Step::Key(KeyCode::Escape),
        Step::Key(KeyCode::KeyC),
        Step::Key(KeyCode::Digit2),
        Step::Key(KeyCode::KeyI),
        Step::Shot("by keys third horse abilities ice".into()),
        Step::Key(KeyCode::KeyF),
        Step::Key(KeyCode::KeyT),
        Step::Key(KeyCode::Digit3),
        Step::Shot("by keys trade storage".into()),
        Step::Key(KeyCode::KeyS),
        Step::Key(KeyCode::KeyK),
        Step::Shot("by keys every key listed".into()),
        Step::Key(KeyCode::Escape),
        Step::Key(KeyCode::Escape),
        Step::Shot("by keys settings and escape stays in town".into()),
        Step::Key(KeyCode::Space),
        Step::Wait(SETTLE),
        Step::Shot("by keys set out".into()),
        Step::Key(KeyCode::KeyJ),
        Step::Key(KeyCode::Digit1),
        Step::Shot("by keys journal on the road".into()),
        // The Journal (#231): the report of the trail before is still
        // there, its piece is found where it is now, and back to the latest.
        Step::Key(KeyCode::BracketLeft),
        Step::Shot("by keys an earlier report".into()),
        Step::Reported,
        Step::Shot("its piece found from the earlier report".into()),
        Step::Key(KeyCode::Escape),
        Step::Key(KeyCode::BracketRight),
        Step::Shot("by keys back to the latest report".into()),
        Step::Key(KeyCode::Escape),
        Step::Shot("by keys escape closes the place on the road".into()),
        // Home a second time, with an upgrade and a report of no note: the
        // one mark, and it goes when the cargo is in view.
        Step::Key(KeyCode::KeyT),
        Step::Town,
        Step::Wait(SETTLE),
        Step::Shot("a plain report raises no mark".into()),
        Step::Key(KeyCode::KeyH),
        Step::Shot("seen and the mark is gone".into()),
    ]);
    tour.steps = steps;
}

const EQUIP: &str = "Equip an upgrade the report just showed";
const ABILITY: &str = "Give a horse a different ability";
const SET_OUT: &str = "Set out for the next town";
const LEFT_BEHIND: &str = "Find out why a piece was left behind";

fn plan_walk(mut tour: ResMut<Tour>, places: Res<Places>) {
    tour.steps = walk_steps(&places);
}

/// The walk of the click budget (#236): each job of `jobs.ron` that can be
/// done, by pointer and by keys. Only the presses between `Job` and `Done`
/// are counted; those around them put the menus where the job starts from
/// and take back what it changed. The keys are those printed on the
/// controls in `places.ron`: one changed there is changed here, or its job
/// comes out as not done.
fn walk_steps(places: &Places) -> Vec<Step> {
    let (journal, report) = places
        .places
        .iter()
        .enumerate()
        .find_map(|(p, place)| {
            let tab = place.tabs.iter().position(|tab| tab.content == Content::Report)?;
            Some((p, tab))
        })
        .expect("a place has the report");
    vec![
        // A new game: in town, Horses open on Gear.
        Step::Job(ABILITY, By::Mouse),
        Step::Tab(0, 1),
        Step::Control("Ice"),
        Step::Done(Check::Element(Element::Ice)),
        Step::Shot("a different ability by pointer".into()),
        Step::Key(KeyCode::KeyU),
        Step::Tab(0, 0),
        Step::Job(ABILITY, By::Keys),
        Step::Key(KeyCode::KeyI),
        Step::Done(Check::Element(Element::Ice)),
        Step::Key(KeyCode::KeyU),
        Step::Job(SET_OUT, By::Mouse),
        Step::Control("Set out"),
        Step::Done(Check::OnTheRoad),
        // Home from the first trail with its drop, the report showing.
        Step::Town,
        Step::Wait(SETTLE),
        Step::Place(journal),
        Step::Tab(journal, report),
        Step::Shot("the report just showed a drop".into()),
        Step::Job(EQUIP, By::Mouse),
        Step::Reported,
        Step::Control("Equip"),
        Step::Done(Check::Equipped),
        Step::Shot("equipped from the report by pointer".into()),
        Step::Key(KeyCode::KeyU),
        Step::Key(KeyCode::Escape),
        Step::Job(EQUIP, By::Keys),
        Step::Key(KeyCode::KeyE),
        Step::Done(Check::Equipped),
        Step::Key(KeyCode::KeyU),
        Step::Place(0),
        Step::Job(LEFT_BEHIND, By::Mouse),
        Step::Place(journal),
        Step::Done(Check::Reading(journal, report)),
        Step::Shot("what becomes of the drop and why".into()),
        Step::Place(0),
        Step::Job(LEFT_BEHIND, By::Keys),
        Step::Key(KeyCode::KeyJ),
        Step::Done(Check::Reading(journal, report)),
        Step::Key(KeyCode::KeyH),
        Step::Job(SET_OUT, By::Keys),
        Step::Key(KeyCode::Space),
        Step::Done(Check::OnTheRoad),
    ]
}

/// What the walk found, as `counts.txt` says it: every job of the file, in
/// its order, with its budget beside what it took.
fn counts(jobs: &[Job], walked: &[Walked]) -> String {
    let mut text = String::new();
    for job in jobs {
        text += &format!("{}\n  from: {}\n", job.name, job.from);
        for (by, word, budget) in [(By::Mouse, "mouse", job.mouse), (By::Keys, "keys", job.keys)] {
            let took = walked.iter().find(|w| w.name == job.name && w.by == by);
            let found = match (took, job.waits_on) {
                (Some(w), _) if !w.done => format!("{} and NOT DONE", w.presses),
                (Some(w), _) if w.presses > budget => format!("{}, OVER", w.presses),
                (Some(w), _) => format!("{}", w.presses),
                (None, Some(issue)) => format!("cannot be done yet, #{issue}"),
                (None, None) => "NOT WALKED".into(),
            };
            text += &format!("  {word}: budget {budget}, took {found}\n");
        }
    }
    text
}

/// Put the tour's key down for one frame, after the real keyboard has been
/// read and before any system looks at it.
fn tour_keys(mut tour: ResMut<Tour>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    if let Some(key) = tour.held.take() {
        keys.release(key);
    }
    if let Some(key) = tour.key.take() {
        keys.press(key);
        tour.held = Some(key);
    }
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
    cells: (
        Query<(Entity, &CarriedCell)>,
        Query<(Entity, &ReportedCell)>,
    ),
    controls: Query<(Entity, &ControlButton)>,
    menus: (Res<Places>, Res<Shell>),
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    tab_stops: TabNavigation,
    nodes: Query<&ComputedNode>,
    out: (
        MessageWriter<AppExit>,
        MessageWriter<PointerInput>,
        Single<Entity, With<PrimaryWindow>>,
        Query<&UiGlobalTransform>,
    ),
) {
    let (mut exit, mut pointer, window, transforms) = out;
    let (carried, reported) = cells;
    let (map, shell) = menus;
    // Its fields one by one: the step is read while the counts are kept.
    let tour = &mut *tour;
    tour.wait -= time.delta_secs();
    if tour.wait > 0.0 {
        return;
    }
    let Some(step) = tour.steps.get(tour.next) else {
        if !tour.walked.is_empty() {
            let text = counts(&Jobs::load(), &tour.walked);
            info!("walk:\n{text}");
            if let Err(e) = std::fs::write(format!("{}/counts.txt", tour.folder), text) {
                error!("walk: counts.txt was not written: {e}");
            }
            tour.walked.clear();
        }
        exit.write(AppExit::Success);
        return;
    };
    // What is laid out and so can be pressed with a pointer.
    let seen = |entity: Entity| nodes.get(entity).is_ok_and(|node| !node.is_empty());
    if matches!(step, Step::Town) && !game.in_town() {
        return;
    }
    // The party leaves when the tour says so, not in the middle of it.
    match step {
        Step::SetOut => game.act(Action::SetOut, Hand::horse(0)),
        _ => game.hold_in_town(),
    }
    let key = match step {
        Step::Key(key) => Some(*key),
        _ => None,
    };
    let pressed = match step {
        Step::Place(p) | Step::PlaceMidway(p) => {
            places.iter().find(|(_, b)| b.0 == *p).map(|(e, _)| e)
        }
        Step::Tab(p, t) => tabs
            .iter()
            .find(|(_, b)| b.place == *p && b.tab == *t)
            .map(|(e, _)| e),
        Step::Horse(h) => horses.iter().find(|(_, b)| b.0 == *h).map(|(e, _)| e),
        // The one in sight: the Inventory shows the same pieces as the
        // overview and the cargo.
        Step::Slot(h, s) => slots
            .iter()
            .filter(|(_, b)| b.horse == *h && b.slot == *s)
            .min_by_key(|(e, _)| !seen(*e))
            .map(|(e, _)| e),
        Step::Carried(p) => carried
            .iter()
            .filter(|(_, b)| b.place == *p)
            .min_by_key(|(e, b)| (!seen(*e), b.tab.is_some()))
            .map(|(e, _)| e),
        Step::Reported => reported.iter().map(|(e, _)| e).min_by_key(|e| !seen(*e)),
        // The one in sight, where a control is in more than one place.
        Step::Control(name) => controls
            .iter()
            .filter(|(_, b)| map.control(b.id).name == *name)
            .min_by_key(|(e, b)| (!seen(*e), b.id))
            .map(|(e, _)| e),
        Step::Enter => focus.get(),
        Step::Key(_)
        | Step::Focus(_)
        | Step::TabRound
        | Step::Point
        | Step::PointAway
        | Step::Wait(_)
        | Step::SetOut
        | Step::Town
        | Step::Shot(_)
        | Step::Job(..)
        | Step::Done(_) => None,
    };
    match step {
        Step::Job(name, by) => {
            tour.job = Some((*name, *by));
            tour.presses = 0;
            tour.missed = false;
            tour.carrying = game.pending_drop().is_some();
        }
        Step::Done(check) => {
            if let Some((name, by)) = tour.job.take() {
                let done = !tour.missed
                    && match *check {
                        Check::Element(element) => game.element == element,
                        Check::OnTheRoad => !game.in_town(),
                        Check::Equipped => {
                            tour.carrying && game.in_town() && game.pending_drop().is_none()
                        }
                        Check::Reading(place, tab) => {
                            shell.is_open()
                                && shell.place() == place
                                && shell.tab(place) == tab
                                && game.reading().is_none()
                        }
                    };
                tour.walked.push(Walked {
                    name,
                    by,
                    presses: tour.presses,
                    done,
                });
            }
        }
        Step::Place(_)
        | Step::PlaceMidway(_)
        | Step::Tab(..)
        | Step::Horse(_)
        | Step::Slot(..)
        | Step::Carried(_)
        | Step::Reported
        | Step::Control(_)
        | Step::Enter
        | Step::Key(_)
            if tour.job.is_some() =>
        {
            tour.presses += 1;
            // A pointer presses only what is in sight.
            if key.is_none() && !pressed.is_some_and(seen) {
                tour.missed = true;
            }
        }
        _ => {}
    }
    if let Step::Focus(name) = step {
        let control = name.and_then(|name| {
            controls
                .iter()
                .filter(|(_, b)| map.control(b.id).name == name)
                .min_by_key(|(_, b)| b.id)
                .map(|(e, _)| e)
        });
        match control {
            Some(entity) => focus.set(entity, FocusCause::Navigated),
            None => focus.clear(),
        }
        focus_visible.0 = control.is_some();
    }
    if let Step::TabRound = step {
        let (mut stops, mut unseen) = (0, 0);
        let mut at = InputFocus::default();
        let mut first = None;
        while let Ok(next) = tab_stops.navigate(&at, NavAction::Next) {
            if first == Some(next) || stops > 1000 {
                break;
            }
            first = first.or(Some(next));
            stops += 1;
            if nodes.get(next).is_ok_and(|node| node.is_empty()) {
                unseen += 1;
            }
            at = InputFocus::from_entity(next);
        }
        info!("tour: Tab makes {stops} stops, {unseen} of them out of sight");
    }
    let mut point = |position: Vec2| {
        if let Some(window) = WindowRef::Entity(*window).normalize(None) {
            pointer.write(PointerInput::new(
                PointerId::Mouse,
                Location {
                    target: NormalizedRenderTarget::Window(window),
                    position,
                },
                PointerAction::Move { delta: Vec2::ZERO },
            ));
        }
    };
    if let Step::PointAway = step {
        point(Vec2::ONE);
    }
    if let Some(entity) = pressed {
        if !std::mem::take(&mut tour.pointing) {
            commands.trigger(Activate { entity });
        } else if let (Ok(node), Ok(transform)) = (nodes.get(entity), transforms.get(entity)) {
            let (_, _, middle) = transform.to_scale_angle_translation();
            point(middle * node.inverse_scale_factor());
        }
    }
    let mut wait = SETTLE;
    match step {
        Step::Point => {
            tour.pointing = true;
            wait = 0.0;
        }
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
    tour.key = key;
    tour.wait = wait;
    tour.next += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The walk and the file name the same jobs (#236): a job that can be
    /// done is walked by pointer and by keys, and one that waits on an issue
    /// is not walked.
    #[test]
    fn the_walk_does_every_job_that_can_be_done_both_ways() {
        let jobs = Jobs::load();
        let steps = walk_steps(&Places::load());
        let walks: Vec<(&str, By)> = steps
            .iter()
            .filter_map(|step| match step {
                Step::Job(name, by) => Some((*name, *by)),
                _ => None,
            })
            .collect();
        for job in &jobs {
            for by in [By::Mouse, By::Keys] {
                let times = walks.iter().filter(|w| **w == (job.name.as_str(), by)).count();
                let wanted = usize::from(job.waits_on.is_none());
                assert_eq!(times, wanted, "{} by {by:?}", job.name);
            }
        }
        for (name, _) in &walks {
            assert!(jobs.iter().any(|job| job.name == *name), "{name} is not in jobs.ron");
        }
    }

    /// A count is of one job: every `Job` is closed by its `Done` before
    /// the next begins, and a job by keys is made of keys alone.
    #[test]
    fn every_job_of_the_walk_is_closed_and_keys_are_keys() {
        let mut open: Option<By> = None;
        for step in walk_steps(&Places::load()) {
            match step {
                Step::Job(name, by) => {
                    assert!(open.is_none(), "{name} begins inside another job");
                    open = Some(by);
                }
                Step::Done(_) => assert!(open.take().is_some(), "a Done with no job"),
                Step::Key(_) => assert_ne!(open, Some(By::Mouse), "a key in a pointer's job"),
                Step::Shot(_) | Step::Wait(_) | Step::Town => {}
                _ => assert_ne!(open, Some(By::Keys), "a pointer's press in a job by keys"),
            }
        }
        assert!(open.is_none(), "the last job is not closed");
    }

    #[test]
    fn the_counts_say_what_is_over_not_done_and_not_yet_possible() {
        let job = |name: &str, waits_on| Job {
            name: name.into(),
            from: "In town.".into(),
            mouse: 2,
            keys: 1,
            waits_on,
        };
        let jobs = [job("Equip", None), job("List", Some(99))];
        let walked = [
            Walked { name: "Equip", by: By::Mouse, presses: 3, done: true },
            Walked { name: "Equip", by: By::Keys, presses: 1, done: false },
        ];
        let text = counts(&jobs, &walked);
        assert!(text.contains("mouse: budget 2, took 3, OVER"), "{text}");
        assert!(text.contains("keys: budget 1, took 1 and NOT DONE"), "{text}");
        assert!(text.contains("mouse: budget 2, took cannot be done yet, #99"), "{text}");
    }
}
