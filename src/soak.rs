//! Dev aid: the soak run, the game left to play itself for hours and its
//! record read afterwards (procedures/overnight-soak.txt).
//!
//! `IDLE_SOAK=<file>` writes what happens to that file as it happens, a line
//! each: every departure with the party's numbers, every line of the fight's
//! log (so every hit and its damage), every kill and what it dropped, every
//! trail's outcome, every arrival in town with the coins and the drops'
//! pieces accounted for, and once a minute the frame times and how many
//! entities there are. A line is the game's seconds, the kind, then
//! `key=value` fields, tab-separated. `IDLE_SOAK_SECS` exits the game by
//! itself after that long. The soak keeps Keep going on, turning it on
//! again after a lost trail has turned it off, so that the game plays on
//! with nobody at it (#305). Nothing else about the game changes.
//!
//! `--soak-report FOLDER` opens no window: it reads the run's files in the
//! folder, checks them against the rules in `assets/data/`, walks every
//! trail again in the sim to see that the game did what the sim says, and
//! writes `report.txt` there. `check` is the whole of it.

use std::{
    collections::HashMap,
    fs::File,
    io::{LineWriter, Write},
    path::Path,
    time::Instant,
};

use bevy::prelude::*;

use crate::game::{Action, CAMP_SECS, Game, Hand, Killed, Screen, TICK, Uncollected};
use crate::sim::data::{Data, Element, Stats};
use crate::sim::expedition::{Expedition, Outcome, Plan};
use crate::sim::loot::Horse;

/// Seconds between two lines of frame times.
const BEAT_SECS: f64 = 60.0;

pub struct SoakPlugin;

impl Plugin for SoakPlugin {
    fn build(&self, app: &mut App) {
        let Ok(path) = std::env::var("IDLE_SOAK") else {
            return;
        };
        let file = File::create(&path).unwrap_or_else(|error| panic!("IDLE_SOAK {path}: {error}"));
        let limit = std::env::var("IDLE_SOAK_SECS")
            .ok()
            .and_then(|value| value.parse().ok());
        app.insert_resource(Soak {
            out: LineWriter::new(file),
            started: Instant::now(),
            limit,
            begun: false,
            on_road: false,
            lines: 0,
            finished: None,
            in_town_since: 0.0,
            reported: 0,
            next_beat: BEAT_SECS,
            frames: 0,
            secs: 0.0,
            worst: 0.0,
            ended: false,
        })
        // After everything of the frame, the drops' arrivals included.
        .add_systems(Last, record);
    }
}

#[derive(Resource)]
struct Soak {
    out: LineWriter<File>,
    started: Instant,
    /// Seconds by the clock after which the game exits by itself.
    limit: Option<f64>,
    begun: bool,
    on_road: bool,
    /// How many lines of the trail's log are written.
    lines: usize,
    /// When the trail's outcome was written, until the party is in town.
    finished: Option<f32>,
    in_town_since: f32,
    /// The number of the last trail whose arrival is written.
    reported: u32,
    next_beat: f64,
    frames: u32,
    secs: f32,
    worst: f32,
    ended: bool,
}

impl Soak {
    fn write(&mut self, t: f32, line: &str) {
        // A full disk is not worth stopping the game for.
        let _ = writeln!(self.out, "{t:.2}\t{line}");
    }
}

fn stats_text(stats: Stats) -> String {
    format!(
        "{}/{}/{}/{}",
        stats.health, stats.attack, stats.defense, stats.elemental
    )
}

fn depart_line(trail: u32, expedition: &Expedition, waited: f32) -> String {
    let party: Vec<String> = expedition
        .party
        .iter()
        .map(|fighter| {
            format!(
                "{}:{}/{}/{}/{}",
                fighter.name, fighter.max_health, fighter.attack, fighter.defense, fighter.elemental
            )
        })
        .collect();
    format!(
        "depart\ttrail={trail}\tseed={}\tlevel={}\telement={}\twaited={waited:.2}\tparty={}",
        expedition.plan.seed,
        expedition.plan.level,
        expedition.element.name(),
        party.join(","),
    )
}

/// A kill and its gear, each piece by its tier, in the order they dropped.
fn kill_line(coins: i32, fountain: &str, gear: &[usize]) -> String {
    let tiers: Vec<String> = gear.iter().map(|tier| tier.to_string()).collect();
    let gear = if tiers.is_empty() { "-".into() } else { tiers.join(",") };
    format!("kill\tcoins={coins}\tfountain={fountain}\tgear={gear}")
}

fn outcome_line(trail: u32, outcome: &Outcome) -> String {
    let drop = match outcome.trail_piece() {
        Some(gear) => format!(
            "{}|{}|{}|{}",
            gear.name,
            gear.rarity,
            gear.slot.name(),
            stats_text(gear.stats)
        ),
        None => "-".into(),
    };
    let deeds: Vec<String> = outcome
        .deeds
        .iter()
        .map(|deeds| {
            format!(
                "{}/{}/{}/{}",
                deeds.dealt, deeds.taken, deeds.kills, deeds.fell as u8
            )
        })
        .collect();
    format!(
        "outcome\ttrail={trail}\twon={}\tcleared={}\ttotal={}\tcoins={}\tstrong={}\tresisted={}\tdrop={drop}\tpieces={}\tdeeds={}",
        outcome.won as u8,
        outcome.cleared,
        outcome.total,
        outcome.coins,
        outcome.strong_hits,
        outcome.resisted_hits,
        outcome.drops.len(),
        deeds.join(","),
    )
}

fn town_line(trail: u32, level: u32, coins: i32, walked: f32, drops: &Uncollected, entities: usize) -> String {
    format!(
        "town\ttrail={trail}\tlevel={level}\tcoins={coins}\twalked={walked:.2}\tuncollected={}\tpieces={}\tthrown={}\tcollected={}\tleft={}\tentities={entities}",
        drops.coins, drops.pieces, drops.thrown, drops.collected, drops.left,
    )
}

fn record(
    time: Res<Time>,
    mut game: ResMut<Game>,
    uncollected: Res<Uncollected>,
    mut killed: MessageReader<Killed>,
    entities: Query<Entity>,
    mut soak: ResMut<Soak>,
    mut exit: MessageWriter<AppExit>,
) {
    let soak = &mut *soak;
    let t = time.elapsed_secs();
    let wall = soak.started.elapsed().as_secs_f64();
    // Nobody is here to set out: at the start, and after a lost trail.
    if game.in_town() && !game.keep_going() {
        game.act(Action::KeepGoing, Hand::default());
    }
    if !soak.begun {
        soak.begun = true;
        let line = format!(
            "start\tversion={}\tseed={}\tlimit={}",
            env!("CARGO_PKG_VERSION"),
            game.plan.seed,
            soak.limit.unwrap_or(0.0),
        );
        soak.write(t, &line);
    }
    let dt = time.delta_secs();
    soak.frames += 1;
    soak.secs += dt;
    soak.worst = soak.worst.max(dt);

    if let Some(kept) = game.last_report()
        && kept.number > soak.reported
    {
        soak.reported = kept.number;
        let walked = soak.finished.take().map_or(0.0, |at| t - at);
        let line = town_line(
            kept.number,
            game.level,
            game.coins,
            walked,
            &uncollected,
            entities.iter().count(),
        );
        soak.write(t, &line);
        soak.in_town_since = t;
    }
    if let Screen::Trail(expedition) = &game.screen {
        if !soak.on_road {
            let line = depart_line(soak.reported + 1, expedition, t - soak.in_town_since);
            soak.write(t, &line);
            soak.lines = 0;
            soak.finished = None;
        }
        for line in &expedition.log[soak.lines.min(expedition.log.len())..] {
            soak.write(t, &format!("line\ttext={line}"));
        }
        soak.lines = expedition.log.len();
        if soak.finished.is_none()
            && let Some(outcome) = &expedition.outcome
        {
            soak.write(t, &outcome_line(soak.reported + 1, outcome));
            soak.finished = Some(t);
        }
    }
    soak.on_road = !game.in_town();
    for kill in killed.read() {
        soak.write(t, &kill_line(kill.coins, &kill.fountain, &kill.gear));
    }

    if wall >= soak.next_beat {
        soak.next_beat += BEAT_SECS;
        let place = if game.in_town() {
            "town"
        } else if game.secs_to_town().is_some() {
            "walking"
        } else {
            "road"
        };
        let line = format!(
            "beat\twall={wall:.1}\tframes={}\tmean_ms={:.2}\tworst_ms={:.1}\tentities={}\tpieces={}\tplace={place}",
            soak.frames,
            1000.0 * soak.secs / soak.frames.max(1) as f32,
            1000.0 * soak.worst,
            entities.iter().count(),
            uncollected.pieces,
        );
        soak.write(t, &line);
        (soak.frames, soak.secs, soak.worst) = (0, 0.0, 0.0);
    }
    if let Some(limit) = soak.limit
        && wall >= limit
        && !soak.ended
    {
        soak.ended = true;
        soak.write(t, &format!("end\treason=limit\twall={wall:.1}"));
        exit.write(AppExit::Success);
    }
}

/// The folder named by `--soak-report FOLDER`.
pub fn report_folder() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--soak-report" {
            return args.next();
        }
    }
    None
}

/// Read a soak run's files in a folder, write `report.txt` beside them and
/// print it. Returns the exit code: 0 when nothing is wrong, 1 when
/// something is, 2 when there is no run to read.
pub fn write_report(folder: &str) -> i32 {
    let folder = Path::new(folder);
    let read = |name: &str| std::fs::read_to_string(folder.join(name)).unwrap_or_default();
    let events = read("events.tsv");
    if events.is_empty() {
        eprintln!("{}: no events.tsv, or an empty one", folder.display());
        return 2;
    }
    let logs = read("game.log") + &read("game.out.log");
    let report = check(&Data::load(), &events, &logs, &read("memory.tsv"), &read("run.txt"));
    if let Err(error) = std::fs::write(folder.join("report.txt"), &report.text) {
        eprintln!("report.txt: {error}");
    }
    println!("{}", report.text);
    i32::from(!report.problems.is_empty())
}

struct Event {
    t: f32,
    kind: String,
    fields: HashMap<String, String>,
}

impl Event {
    fn text(&self, key: &str) -> &str {
        self.fields.get(key).map_or("", String::as_str)
    }

    fn num<T: std::str::FromStr + Default>(&self, key: &str) -> T {
        self.text(key).parse().unwrap_or_default()
    }
}

fn parse(log: &str) -> Vec<Event> {
    log.lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let t = parts.next()?.parse().ok()?;
            let kind = parts.next()?.to_string();
            let fields = parts
                .filter_map(|part| part.split_once('='))
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();
            Some(Event { t, kind, fields })
        })
        .collect()
}

fn parse_stats(text: &str) -> Option<Stats> {
    let mut numbers = text.split('/').map(|number| number.parse().ok());
    Some(Stats {
        health: numbers.next()??,
        attack: numbers.next()??,
        defense: numbers.next()??,
        elemental: numbers.next()??,
    })
}

/// What is wrong, by kind, so that one fault repeated all night is one entry.
#[derive(Default)]
struct Problems(Vec<(String, Vec<String>)>);

impl Problems {
    fn add(&mut self, kind: &str, detail: String) {
        match self.0.iter_mut().find(|(known, _)| known == kind) {
            Some((_, details)) => details.push(detail),
            None => self.0.push((kind.to_string(), vec![detail])),
        }
    }
}

#[derive(Default)]
struct Trail<'a> {
    depart: Option<&'a Event>,
    lines: Vec<&'a str>,
    kills: Vec<&'a Event>,
    outcome: Option<&'a Event>,
    town: Option<&'a Event>,
}

pub struct Report {
    /// Each kind of thing that is wrong, with every case of it.
    pub problems: Vec<(String, Vec<String>)>,
    pub text: String,
}

/// The number a hit's line ends in: "... for 12  Strong!".
fn damage_of(line: &str) -> Option<i32> {
    let (_, rest) = line.rsplit_once(" for ")?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

fn median(values: &[f32]) -> f32 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f32::total_cmp);
    sorted.get(sorted.len() / 2).copied().unwrap_or(0.0)
}

/// Check a soak run: `events` is what `IDLE_SOAK` wrote, `logs` what the game
/// printed, `memory` the launcher's samples of its memory (seconds, megabytes)
/// and `run` the launcher's own `key=value` notes. The last three may be empty.
pub fn check(data: &Data, events: &str, logs: &str, memory: &str, run: &str) -> Report {
    let events = parse(events);
    let rules = &data.rules;
    let mut problems = Problems::default();

    let mut trails: Vec<Trail> = Vec::new();
    let mut beats: Vec<&Event> = Vec::new();
    let (mut start, mut end) = (None, None);
    for event in &events {
        let trail = trails.last_mut();
        match (event.kind.as_str(), trail) {
            ("start", _) => start = Some(event),
            ("end", _) => end = Some(event),
            ("beat", _) => beats.push(event),
            ("depart", _) => trails.push(Trail {
                depart: Some(event),
                ..default()
            }),
            ("line", Some(trail)) => trail.lines.push(event.text("text")),
            ("kill", Some(trail)) => trail.kills.push(event),
            ("outcome", Some(trail)) => trail.outcome = Some(event),
            ("town", Some(trail)) => trail.town = Some(event),
            (kind, _) => problems.add(
                "a line the report does not know, or one before the first departure",
                format!("{kind} at {:.0} s", event.t),
            ),
        }
    }

    // The run itself.
    let last_t = events.last().map_or(0.0, |event| event.t);
    let limit: f64 = start.map_or(0.0, |start| start.num("limit"));
    if start.is_none() {
        problems.add("the log has no start line", String::new());
    }
    match end {
        Some(_) => {}
        None => problems.add(
            "the game did not end by itself at its limit",
            format!("the last line is at {last_t:.0} s of {limit:.0}: it crashed, hung or was closed"),
        ),
    }
    let notes: HashMap<&str, &str> = run.lines().filter_map(|line| line.split_once('=')).collect();
    if let Some(closed) = notes.get("closed")
        && *closed != "itself"
    {
        problems.add("the launcher had to close the game", format!("closed={closed}"));
    }
    if let Some(code) = notes.get("exit")
        && *code != "0"
    {
        problems.add("the game's exit code was not 0", format!("exit={code}"));
    }

    // Every trail, in order.
    let n_tiers = data.gear.rarities.len();
    let (mut won_tiers, mut lost_tiers) = (vec![0u32; n_tiers], vec![0u32; n_tiers]);
    let (mut wins, mut losses, mut no_drop) = (0u32, 0u32, 0u32);
    let mut coins_total = 0i64;
    let mut previous: Option<(u32, u64, u32, bool)> = None;
    let mut previous_power = 0;
    let mut waits: Vec<f32> = Vec::new();
    let mut walks_won: Vec<f32> = Vec::new();
    let mut trail_secs: Vec<f32> = Vec::new();
    let mut town_entities: Vec<f32> = Vec::new();
    let mut deeds_total: Vec<(String, [i64; 4])> = Vec::new();
    let (mut hits_dealt, mut hits_taken, mut biggest_dealt, mut biggest_taken) = (0u64, 0u64, 0, 0);
    let (mut strong, mut resisted, mut kills_total) = (0u64, 0u64, 0u64);
    let mut by_level: Vec<(u32, u32, u32)> = Vec::new();
    let mut last_party = String::new();
    let mut hours: Vec<[f64; 4]> = Vec::new(); // trails, wins, level, coins
    let mut last_drops: Option<&Event> = None;

    let count = trails.len();
    for (index, trail) in trails.iter().enumerate() {
        let depart = trail.depart.expect("a trail begins with its departure");
        let number: u32 = depart.num("trail");
        let seed: u64 = depart.num("seed");
        let level: u32 = depart.num("level");
        let at = format!("trail {number} (seed {seed}, level {level})");

        // Town to town: one after another, each by itself after the wait.
        if let Some((before, seed_before, level_before, won_before)) = previous {
            if number != before + 1 {
                problems.add("trail numbers do not follow on", format!("{at} after trail {before}"));
            }
            if seed != seed_before.wrapping_add(1) {
                problems.add("a trail's seed is not the one after the last", at.clone());
            }
            if level != level_before + u32::from(won_before) {
                problems.add(
                    "the trail level did not follow the last result",
                    format!("{at}: it was {level_before}, won {won_before}"),
                );
            }
        }
        let waited: f32 = depart.num("waited");
        waits.push(waited);
        if (waited - CAMP_SECS).abs() > 0.5 {
            problems.add(
                "the party did not leave town by itself on time",
                format!("{at}: waited {waited:.2} s, not {CAMP_SECS}"),
            );
        }

        // The party as it set out. Nothing it wears is ever worse than before.
        let party: Vec<(String, Stats)> = depart
            .text("party")
            .split(',')
            .filter_map(|horse| {
                let (name, stats) = horse.split_once(':')?;
                Some((name.to_string(), parse_stats(stats)?))
            })
            .collect();
        if party.len() != data.horses.len() {
            problems.add("the party is not the horses of horses.ron", at.clone());
        }
        let power: i32 = party
            .iter()
            .map(|(_, stats)| crate::sim::loot::score(data, *stats))
            .sum();
        if power < previous_power {
            problems.add(
                "the party set out weaker than the time before",
                format!("{at}: {power} points after {previous_power}"),
            );
        }
        previous_power = power;
        last_party = depart.text("party").replace(',', "   ");

        // Damage: every hit takes at least 1.
        for line in &trail.lines {
            let dealt = party.iter().any(|(name, _)| line.starts_with(name.as_str()));
            let hit = [" hits ", " Focused Shot on ", " Elemental Burst on "];
            if !hit.iter().any(|verb| line.contains(verb)) {
                continue;
            }
            match damage_of(line) {
                Some(damage) if damage >= 1 => {
                    if dealt {
                        hits_dealt += 1;
                        biggest_dealt = biggest_dealt.max(damage);
                    } else {
                        hits_taken += 1;
                        biggest_taken = biggest_taken.max(damage);
                    }
                }
                _ => problems.add("a hit with no damage, or none that can be read", format!("{at}: {line}")),
            }
        }

        // Kills and their coins.
        let mut defeated = 0;
        for line in &trail.lines {
            let Some((name, rest)) = line.split_once(" is defeated (+") else {
                continue;
            };
            let coins: i32 = rest.trim_end_matches(" coins)").parse().unwrap_or(-1);
            match data.enemies.iter().find(|enemy| enemy.name == name) {
                Some(enemy) if coins >= enemy.coins.0 && coins <= enemy.coins.1 => {}
                Some(enemy) => problems.add(
                    "a kill's coins are outside its enemy's range",
                    format!("{at}: {name} dropped {coins}, range {:?}", enemy.coins),
                ),
                None => problems.add("a kill of an enemy not in enemies.ron", format!("{at}: {name}")),
            }
            if trail.kills.get(defeated).map(|kill| kill.num::<i32>("coins")) != Some(coins) {
                problems.add(
                    "a kill in the fight's log was not shown with its coins",
                    format!("{at}: kill {} ({name}, {coins} coins)", defeated + 1),
                );
            }
            defeated += 1;
        }
        if defeated != trail.kills.len() {
            problems.add(
                "kills shown and kills in the fight's log differ in number",
                format!("{at}: {} shown, {defeated} in the log", trail.kills.len()),
            );
        }
        kills_total += trail.kills.len() as u64;

        let Some(outcome) = trail.outcome else {
            if index + 1 < count {
                problems.add("a trail with no outcome", at.clone());
            }
            continue;
        };
        let won = outcome.num::<u8>("won") == 1;
        let cleared: usize = outcome.num("cleared");
        let coins: i32 = outcome.num("coins");
        previous = Some((number, seed, level, won));
        strong += outcome.num::<u64>("strong");
        resisted += outcome.num::<u64>("resisted");
        if won { wins += 1 } else { losses += 1 }
        match by_level.iter_mut().find(|(known, _, _)| *known == level) {
            Some((_, won_there, walked)) => {
                *won_there += u32::from(won);
                *walked += 1;
            }
            None => by_level.push((level, u32::from(won), 1)),
        }
        trail_secs.push(outcome.t - depart.t);
        if trail.kills.iter().map(|kill| kill.num::<i32>("coins")).sum::<i32>() != coins {
            problems.add("a trail's coins are not the sum of its kills'", at.clone());
        }
        for (horse, deeds) in outcome.text("deeds").split(',').enumerate() {
            let numbers: Vec<i64> = deeds.split('/').filter_map(|n| n.parse().ok()).collect();
            let Some((name, _)) = party.get(horse) else { break };
            if deeds_total.len() <= horse {
                deeds_total.push((name.clone(), [0; 4]));
            }
            for (sum, number) in deeds_total[horse].1.iter_mut().zip(numbers) {
                *sum += number;
            }
        }

        // The trail's own piece: one when an encounter was cleared, of a rarity and
        // with stat points the table allows.
        let drop = outcome.text("drop");
        let mut tier_dropped = None;
        if drop == "-" {
            no_drop += 1;
            if cleared > 0 {
                problems.add("no piece dropped though an encounter was cleared", at.clone());
            }
        } else {
            let parts: Vec<&str> = drop.split('|').collect();
            let tier = parts.get(1).and_then(|tier| tier.parse::<usize>().ok());
            let stats = parts.get(3).and_then(|stats| parse_stats(stats));
            match (tier, stats) {
                (Some(tier), Some(stats)) if tier < n_tiers => {
                    tier_dropped = Some(tier);
                    if won { won_tiers[tier] += 1 } else { lost_tiers[tier] += 1 }
                    let rarity = &data.gear.rarities[tier];
                    let points = stats.attack + stats.defense + stats.elemental
                        + stats.health / data.gear.health_per_point;
                    let allowed = (rarity.points.0 + level as i32, rarity.points.1 + level as i32);
                    if points < allowed.0 || points > allowed.1 {
                        problems.add(
                            "a piece's stat points are outside its rarity's range",
                            format!("{at}: {drop} has {points}, range {allowed:?}"),
                        );
                    }
                    if !parts[0].starts_with(&rarity.prefix) {
                        problems.add("a piece's name does not match its rarity", format!("{at}: {drop}"));
                    }
                }
                _ => problems.add("a drop that cannot be read", format!("{at}: {drop}")),
            }
            if cleared == 0 {
                problems.add("a piece dropped though no encounter was cleared", at.clone());
            }
        }
        // Every kill throws a piece or more (#352), of the tiers a kill can
        // throw; the kill that wins the trail throws the trail's piece last;
        // and the outcome counts every piece thrown, and the trail's.
        let pieces: usize = outcome.num("pieces");
        let thrown: Vec<Vec<usize>> = trail
            .kills
            .iter()
            .map(|kill| kill.text("gear").split(',').filter_map(|tier| tier.parse().ok()).collect())
            .collect();
        let mut chaff: Vec<&[usize]> = thrown.iter().map(Vec::as_slice).collect();
        if won && let Some(last) = chaff.last_mut() {
            let tier_thrown = last.last().copied();
            *last = &last[..last.len().saturating_sub(1)];
            if tier_thrown != tier_dropped {
                problems.add("the trail's piece was not thrown by the winning kill as it dropped", at.clone());
            }
        }
        if chaff.iter().any(|gear| gear.is_empty()) {
            problems.add("a kill threw no gear", at.clone());
        }
        if chaff.iter().flat_map(|gear| gear.iter()).any(|&tier| {
            data.gear.rarities.get(tier).is_none_or(|rarity| rarity.kill_weight == 0)
        }) {
            problems.add("a kill threw a piece of a tier kills do not throw", at.clone());
        }
        let counted = chaff.iter().map(|gear| gear.len()).sum::<usize>() + usize::from(cleared > 0);
        if pieces != counted {
            problems.add(
                "the pieces counted are not the pieces thrown",
                format!("{at}: {pieces} counted, {counted} thrown"),
            );
        }

        // The same trail walked again in the sim: line for line the same.
        let element = Element::ALL
            .into_iter()
            .find(|element| element.name() == depart.text("element"));
        let horses: Option<Vec<Horse>> = party
            .iter()
            .map(|(name, stats)| {
                let def = data.horses.iter().find(|def| def.name == *name)?;
                Some(Horse {
                    name: name.clone(),
                    class: def.class,
                    base: *stats,
                    gear: [None, None, None],
                })
            })
            .collect();
        match (element, horses) {
            (Some(element), Some(horses)) => {
                let plan = Plan::generate(data, seed, level);
                let mut again = Expedition::new(data, plan, &horses, element);
                for _ in 0..1_000_000 {
                    if again.outcome.is_some() {
                        break;
                    }
                    again.tick(data, TICK);
                }
                // Lines the game adds after the first are allowed for.
                let extra = trail.lines.len().saturating_sub(again.log.len());
                let same_lines = trail.lines.len() >= again.log.len()
                    && trail.lines.first() == again.log.first().map(String::as_str).as_ref()
                    && trail.lines[(1 + extra).min(trail.lines.len())..]
                        .iter()
                        .eq(again.log[1..].iter().map(String::as_str).collect::<Vec<_>>().iter());
                let same_end = again.outcome.as_ref().is_some_and(|sim| {
                    sim.won == won
                        && sim.cleared == cleared
                        && sim.coins == coins
                        && sim.trail_piece().map(|gear| gear.rarity) == tier_dropped
                        && sim.drops.len() == pieces
                });
                if !same_lines || !same_end {
                    problems.add(
                        "the game's trail is not the one the sim walks from the same seed and party",
                        format!(
                            "{at}: {} lines in the game, {} in the sim, same ending {same_end}",
                            trail.lines.len(),
                            again.log.len()
                        ),
                    );
                }
            }
            _ => problems.add("a departure that cannot be read", at.clone()),
        }

        // Back in town: the coins are banked, the level moved, the drops are in.
        let Some(town) = trail.town else {
            if index + 1 < count {
                problems.add("a finished trail that never reached town", at.clone());
            }
            continue;
        };
        coins_total += coins as i64;
        if town.num::<i64>("coins") != coins_total {
            problems.add(
                "the coins in town are not the sum of the trails'",
                format!("{at}: {} in town, {coins_total} earned", town.text("coins")),
            );
        }
        if town.num::<u32>("level") != level + u32::from(won) {
            problems.add("the level in town did not follow the result", at.clone());
        }
        let walked: f32 = town.num("walked");
        let travel = rules.travel_secs;
        if won {
            walks_won.push(walked);
        }
        if walked < travel - 0.1 || (!won && walked > travel + 0.5) || walked > travel + 30.0 {
            problems.add(
                "the walk back to town was not the length it should be",
                format!("{at}: {walked:.2} s, won {won}, the walk is {travel} s"),
            );
        }
        let (pieces, thrown, collected, left): (i64, i64, i64, i64) = (
            town.num("pieces"),
            town.num("thrown"),
            town.num("collected"),
            town.num("left"),
        );
        if thrown != collected + left + pieces {
            problems.add(
                "drops' pieces unaccounted for",
                format!("{at}: {thrown} thrown, {collected} collected, {left} left behind, {pieces} still out"),
            );
        }
        if won && (pieces != 0 || town.num::<i64>("uncollected") != 0) {
            problems.add(
                "the party reached town from a won trail with drops still on the road",
                format!("{at}: {pieces} pieces, {} coins", town.text("uncollected")),
            );
        }
        town_entities.push(town.num("entities"));
        last_drops = Some(town);
        let hour = (town.t / 3600.0) as usize;
        if hours.len() <= hour {
            hours.resize(hour + 1, [0.0; 4]);
        }
        hours[hour][0] += 1.0;
        hours[hour][1] += f64::from(u8::from(won));
        hours[hour][2] = f64::from(level + u32::from(won));
        hours[hour][3] = coins_total as f64;
    }
    if count == 0 {
        problems.add("the party never left town", String::new());
    }

    // Rarities against their weights, where there are enough drops to say.
    let mut rarity_lines = Vec::new();
    for (label, counts, won) in [("won", &won_tiers, true), ("lost", &lost_tiers, false)] {
        let total: u32 = counts.iter().sum();
        let weights: Vec<u32> = data
            .gear
            .rarities
            .iter()
            .map(|rarity| if won { rarity.win_weight } else { rarity.weight })
            .collect();
        let weight_sum: u32 = weights.iter().sum();
        let mut shares = Vec::new();
        for (tier, rarity) in data.gear.rarities.iter().enumerate() {
            let chance = weights[tier] as f32 / weight_sum.max(1) as f32;
            let share = counts[tier] as f32 / total.max(1) as f32;
            shares.push(format!(
                "{} {} ({:.0}%, table {:.0}%)",
                rarity.name,
                counts[tier],
                100.0 * share,
                100.0 * chance
            ));
            let sigma = (chance * (1.0 - chance) / total.max(1) as f32).sqrt();
            if total >= 50 && (share - chance).abs() > 4.0 * sigma {
                problems.add(
                    "a rarity drops far from its weight",
                    format!("{label} trails: {} is {:.1}% of {total}, table {:.1}%", rarity.name, 100.0 * share, 100.0 * chance),
                );
            }
        }
        rarity_lines.push(format!("  {label} trails, {total} pieces: {}", shares.join(", ")));
    }

    // The minute beats: no stall, no slowing, nothing piling up.
    let mut previous_wall = 0.0f64;
    let means: Vec<f32> = beats.iter().map(|beat| beat.num("mean_ms")).collect();
    for beat in &beats {
        let wall: f64 = beat.num("wall");
        if wall - previous_wall > 2.0 * BEAT_SECS {
            problems.add(
                "the game stalled",
                format!("no frame-time line between {previous_wall:.0} s and {wall:.0} s"),
            );
        }
        previous_wall = wall;
        let mean: f32 = beat.num("mean_ms");
        if mean > 50.0 {
            problems.add("under 20 frames a second for a minute", format!("at {wall:.0} s: {mean:.1} ms a frame"));
        }
    }
    let quarter = (means.len() / 4).max(1).min(means.len());
    let (early, late) = (median(&means[..quarter]), median(&means[means.len() - quarter..]));
    if late > early * 1.5 && late - early > 2.0 {
        problems.add("frames got slower over the run", format!("{early:.2} ms early, {late:.2} ms late"));
    }
    let wall_end: f64 = end.map_or(previous_wall, |end| end.num("wall"));
    if wall_end > 60.0 && (wall_end - f64::from(last_t)).abs() > wall_end * 0.01 {
        problems.add(
            "the game's clock fell behind the wall clock",
            format!("{last_t:.0} s in the game, {wall_end:.0} s by the clock: long frames are being cut short"),
        );
    }
    let tenth = (town_entities.len() / 10).clamp(1, 10).min(town_entities.len());
    let (first_entities, last_entities) = (
        median(&town_entities[..tenth]),
        median(&town_entities[town_entities.len() - tenth..]),
    );
    if last_entities > first_entities * 1.25 && last_entities - first_entities > 50.0 {
        problems.add(
            "entities pile up",
            format!("{first_entities:.0} on arriving in town early in the run, {last_entities:.0} late"),
        );
    }

    // What the game printed.
    let mut printed: Vec<(String, u32)> = Vec::new();
    let mut noise = 0;
    let lines: Vec<&str> = logs.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        if !(line.contains("WARN") || line.contains("ERROR") || line.contains("panicked")) {
            continue;
        }
        // The GOG Galaxy overlay's Vulkan layer complains at every launch:
        // a loader message, what it is about on the next line, and the
        // object it concerns on the one after.
        let about = |back: usize, ahead: usize| {
            let from = index.saturating_sub(back);
            let to = (index + ahead + 1).min(lines.len());
            lines[from..to].iter().any(|near| near.to_lowercase().contains("galaxy"))
        };
        if (line.contains("Loader Message") && about(0, 1)) || (line.contains("objects: (") && about(1, 0)) {
            noise += 1;
            continue;
        }
        let message = match line.find("WARN").or_else(|| line.find("ERROR")) {
            Some(at) => &line[at..],
            None => line,
        };
        match printed.iter_mut().find(|(known, _)| known == message) {
            Some((_, times)) => *times += 1,
            None => printed.push((message.to_string(), 1)),
        }
    }
    for (message, times) in &printed {
        problems.add("warnings and errors the game printed", format!("{times} times: {message}"));
    }

    // Memory, as the launcher sampled it.
    let samples: Vec<(f32, f32)> = memory
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
        })
        .collect();
    let settled = samples.iter().find(|(secs, _)| *secs >= 300.0);
    let memory_line = match (settled, samples.last()) {
        (Some((_, first)), Some((_, last))) => {
            let most = samples.iter().map(|(_, mb)| *mb).fold(0.0, f32::max);
            if *last > first * 1.5 && last - first > 100.0 {
                problems.add("memory grows", format!("{first:.0} MB after five minutes, {last:.0} MB at the end"));
            }
            format!("{first:.0} MB after five minutes, {last:.0} MB at the end, {most:.0} MB at most")
        }
        _ => "not sampled, or the run was under five minutes".into(),
    };

    // The report.
    let spread = |values: &[f32]| {
        let low = values.iter().copied().fold(f32::INFINITY, f32::min);
        let high = values.iter().copied().fold(0.0, f32::max);
        if values.is_empty() { "none".to_string() } else { format!("{low:.2} to {high:.2}") }
    };
    let mut text = String::new();
    let mut say = |line: String| {
        text += &line;
        text.push('\n');
    };
    let total_problems: usize = problems.0.iter().map(|(_, cases)| cases.len().max(1)).sum();
    say("SOAK REPORT".into());
    say(format!(
        "Horse Heroes {}, first seed {}, {:.2} hours by the clock ({:.0} s of a {:.0} s limit)",
        start.map_or("?", |start| start.text("version")),
        start.map_or("?", |start| start.text("seed")),
        wall_end / 3600.0,
        wall_end,
        limit,
    ));
    say(String::new());
    if problems.0.is_empty() {
        say("VERDICT: nothing wrong found.".into());
    } else {
        say(format!(
            "VERDICT: {} kinds of problem, {total_problems} cases.",
            problems.0.len()
        ));
        for (kind, cases) in &problems.0 {
            say(format!("\n  {kind}  [{}]", cases.len()));
            for case in cases.iter().take(5).filter(|case| !case.is_empty()) {
                say(format!("    {case}"));
            }
            if cases.len() > 5 {
                say(format!("    and {} more", cases.len() - 5));
            }
        }
    }
    say("\nTOWN TO TOWN".into());
    say(format!(
        "  {count} departures, {} arrivals; {wins} trails won, {losses} lost ({:.0}% won)",
        town_entities.len(),
        100.0 * wins as f32 / (wins + losses).max(1) as f32,
    ));
    say(format!(
        "  waited in town before leaving by itself: {} s (the wait is {CAMP_SECS} s)",
        spread(&waits)
    ));
    say(format!(
        "  a trail, departure to its end: {} s, median {:.1}",
        spread(&trail_secs),
        median(&trail_secs)
    ));
    say(format!(
        "  walk back after a won trail, its last drops' pickup included: {} s",
        spread(&walks_won)
    ));
    say("\nBY THE HOUR   trails  won  level  coins".into());
    for (hour, [trails, won, level, coins]) in hours.iter().enumerate() {
        say(format!("  hour {:<2}      {trails:>4.0} {won:>4.0} {level:>6.0} {coins:>6.0}", hour + 1));
    }
    by_level.sort();
    // In at most a dozen bands of levels, so a long climb is still a line.
    let top = by_level.last().map_or(0, |(level, _, _)| *level);
    let band = top / 12 + 1;
    let mut bands: Vec<(u32, u32, u32)> = Vec::new();
    for (level, won, walked) in &by_level {
        match bands.last_mut() {
            Some((from, won_there, walked_there)) if level / band == *from / band => {
                *won_there += won;
                *walked_there += walked;
            }
            _ => bands.push((level / band * band, *won, *walked)),
        }
    }
    let levels: Vec<String> = bands
        .iter()
        .map(|(from, won, walked)| match band {
            1 => format!("{from}: {won}/{walked}"),
            _ => format!("{from}-{}: {won}/{walked}", (from + band - 1).min(top)),
        })
        .collect();
    say(format!("\nWON OF WALKED, BY TRAIL LEVEL\n  {}", levels.join("   ")));
    say("\nDAMAGE".into());
    say(format!(
        "  {hits_dealt} hits by the party (biggest {biggest_dealt}), {hits_taken} taken (biggest {biggest_taken}); \
         {strong} strong, {resisted} resisted; {kills_total} kills"
    ));
    for (name, [dealt, taken, kills, fell]) in &deeds_total {
        say(format!("  {name:<10} dealt {dealt:>8}  took {taken:>7}  kills {kills:>5}  fell {fell:>4} times"));
    }
    say(format!("  the party at its last departure (health/attack/defense/elemental):\n    {last_party}"));
    say("\nDROPS".into());
    for line in rarity_lines {
        say(line);
    }
    say(format!("  trails that brought nothing back: {no_drop}"));
    if let Some(town) = last_drops {
        say(format!(
            "  pieces in the loot shower: {} thrown, {} collected, {} carried off by the road, {} still out",
            town.text("thrown"),
            town.text("collected"),
            town.text("left"),
            town.text("pieces"),
        ));
    }
    say(format!("  coins banked: {coins_total}"));
    say("\nTHE MACHINE".into());
    say(format!(
        "  frame time, a minute's mean: {early:.2} ms in the first quarter, {late:.2} ms in the last; worst single frame {:.0} ms",
        beats.iter().map(|beat| beat.num::<f32>("worst_ms")).fold(0.0, f32::max)
    ));
    say(format!(
        "  entities on arriving in town: {first_entities:.0} early, {last_entities:.0} late ({})",
        spread(&town_entities)
    ));
    say(format!("  memory: {memory_line}"));
    say(format!(
        "  printed by the game: {} kinds of warning or error, and {noise} lines of the GOG Galaxy overlay's (ignored)",
        printed.len()
    ));
    Report {
        problems: problems.0,
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A run as `record` writes it, from the sim alone: trail after trail
    /// from town, with nothing worn.
    fn written(data: &Data, trails: u32) -> String {
        let party: Vec<Horse> = data.horses.iter().map(Horse::new).collect();
        let mut drops = Uncollected::default();
        let (mut t, mut level, mut coins) = (0.0, 0, 0);
        let mut out = "0.00\tstart\tversion=test\tseed=6\tlimit=0\n".to_string();
        let mut write = |t: f32, line: String| out += &format!("{t:.2}\t{line}\n");
        for number in 1..=trails {
            t += CAMP_SECS;
            let plan = Plan::generate(data, 5 + u64::from(number), level);
            let mut expedition = Expedition::new(data, plan, &party, Element::Fire);
            write(t, depart_line(number, &expedition, CAMP_SECS));
            let mut lines = 0;
            let outcome = loop {
                expedition.tick(data, TICK);
                t += TICK;
                for line in &expedition.log[lines..] {
                    write(t, format!("line\ttext={line}"));
                }
                lines = expedition.log.len();
                for kill in std::mem::take(&mut expedition.kills) {
                    let gear: Vec<usize> = kill.gear.iter().map(|gear| gear.rarity).collect();
                    write(t, kill_line(kill.coins, &kill.fountain, &gear));
                    drops.thrown += 1;
                    drops.collected += 1;
                }
                if let Some(outcome) = expedition.outcome.clone() {
                    break outcome;
                }
            };
            write(t, outcome_line(number, &outcome));
            t += data.rules.travel_secs;
            level += u32::from(outcome.won);
            coins += outcome.coins;
            write(t, town_line(number, level, coins, data.rules.travel_secs, &drops, 100));
        }
        write(t, format!("end\treason=limit\twall={t:.1}"));
        out
    }

    /// A run that went as the rules say has nothing wrong with it, and a
    /// hit, a wait or a coin that did not is found.
    #[test]
    fn the_report_finds_what_is_wrong_and_nothing_else() {
        let data = Data::load();
        let log = written(&data, 6);
        let report = check(&data, &log, "", "", "closed=itself\nexit=0\n");
        assert!(report.problems.is_empty(), "{}", report.text);
        assert!(report.text.contains("6 departures, 6 arrivals"), "{}", report.text);

        let kinds = |log: &str, logs: &str| -> Vec<String> {
            let report = check(&data, log, logs, "", "");
            report.problems.into_iter().map(|(kind, _)| kind).collect()
        };
        // A hit that did other damage than the sim's.
        let hit = log.lines().find(|line| line.contains(" hits ")).unwrap();
        let (head, _) = hit.rsplit_once(" for ").unwrap();
        let changed = log.replacen(hit, &format!("{head} for 999"), 1);
        assert!(kinds(&changed, "")[0].contains("not the one the sim walks"));
        // A party that stayed in town.
        let late = log.replacen("waited=12.00", "waited=40.00", 1);
        assert_eq!(kinds(&late, ""), ["the party did not leave town by itself on time"]);
        // A run that stops in the middle, and what the game printed.
        let cut = &log[..log.len() / 2];
        assert!(kinds(cut, "")[0].contains("did not end by itself"));
        let printed = kinds(&log, "2026 WARN bevy: something\n2026 ERROR Loader Message\n\tgalaxy_overlay\n");
        assert_eq!(printed, ["warnings and errors the game printed"]);
    }
}
