//! Logs a player can find and send back (#364). The release build has no
//! console, so a panic's message and the game's log would otherwise go
//! nowhere. Both are written beside the game's exe:
//! - `crash.log`: every panic, appended, capped at [`CRASH_LOG_CAP`] bytes.
//! - `horse_heroes.log`: this run's log lines, replaced each run.

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::OnceLock,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::log::{BoxedLayer, tracing_subscriber::Layer};
use bevy::prelude::*;

pub const CRASH_LOG: &str = "crash.log";
pub const GAME_LOG: &str = "horse_heroes.log";
/// The crash log is cut back to its newest half once it would pass this.
pub const CRASH_LOG_CAP: usize = 256 * 1024;

static SEED: OnceLock<u64> = OnceLock::new();

/// The folder the logs go in: the exe's own if the game may write there, as
/// in a per-user install or a build; otherwise `%LOCALAPPDATA%\Horse Heroes`,
/// as under Program Files, where a player cannot write.
pub fn log_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let beside_exe = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(Path::to_path_buf))
            .unwrap_or_else(|| PathBuf::from("."));
        if writable(&beside_exe) {
            return beside_exe;
        }
        match std::env::var_os("LOCALAPPDATA") {
            Some(local) => {
                let dir = PathBuf::from(local).join("Horse Heroes");
                let _ = fs::create_dir_all(&dir);
                dir
            }
            None => beside_exe,
        }
    })
    .clone()
}

fn writable(dir: &Path) -> bool {
    let probe = dir.join(".horse_heroes_write_test");
    let ok = fs::write(&probe, b"").is_ok();
    let _ = fs::remove_file(&probe);
    ok
}

/// Writes every panic to `crash.log`, then runs the panic hook already set.
/// The seed is said in the report, so the run can be played again.
pub fn install(seed: u64) {
    let _ = SEED.set(seed);
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .map(|text| text.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "(no message)".to_string());
        let location = info
            .location()
            .map_or_else(|| "(unknown)".to_string(), |at| at.to_string());
        let thread = std::thread::current();
        let text = report(&Report {
            when: utc_now(),
            version: env!("CARGO_PKG_VERSION"),
            seed: SEED.get().copied(),
            thread: thread.name().unwrap_or("(unnamed)"),
            message: &message,
            location: &location,
            backtrace: &std::backtrace::Backtrace::force_capture().to_string(),
        });
        let _ = append_capped(&log_dir().join(CRASH_LOG), &text, CRASH_LOG_CAP);
        previous(info);
    }));
}

/// What one panic's entry in `crash.log` says.
pub struct Report<'a> {
    pub when: String,
    pub version: &'a str,
    pub seed: Option<u64>,
    pub thread: &'a str,
    pub message: &'a str,
    pub location: &'a str,
    pub backtrace: &'a str,
}

pub fn report(report: &Report) -> String {
    let seed = report
        .seed
        .map_or_else(|| "(unknown)".to_string(), |seed| seed.to_string());
    format!(
        "==== Horse Heroes {} crashed at {} ====\n\
         seed: {seed}\n\
         thread: {}\n\
         where: {}\n\
         message: {}\n\
         backtrace:\n{}\n\n",
        report.version,
        report.when,
        report.thread,
        report.location,
        report.message,
        report.backtrace.trim_end(),
    )
}

/// Writes a render error to `crash.log` (#374): Bevy meets these without a
/// panic, so the panic hook never sees them.
pub fn render_error(kind: &str, description: &str, source: &str, outcome: &str, assets: &str) {
    let text = render_report(&utc_now(), env!("CARGO_PKG_VERSION"), SEED.get().copied(), kind, description, source, outcome, assets);
    let _ = append_capped(&log_dir().join(CRASH_LOG), &text, CRASH_LOG_CAP);
}

/// What one render error's entry in `crash.log` says.
#[allow(clippy::too_many_arguments)]
pub fn render_report(
    when: &str,
    version: &str,
    seed: Option<u64>,
    kind: &str,
    description: &str,
    source: &str,
    outcome: &str,
    assets: &str,
) -> String {
    let seed = seed.map_or_else(|| "(unknown)".to_string(), |seed| seed.to_string());
    format!(
        "==== Horse Heroes {version} render error at {when} ====\n\
         seed: {seed}\n\
         error: {kind}\n\
         message: {description}\n\
         source: {source}\n\
         the game: {outcome}\n\
         assets: {assets}\n\n",
    )
}

/// Appends `text` to the file at `path`. If that would make it longer than
/// `cap` bytes, the oldest entries go first: what is kept is the newest
/// part, at most half of `cap`, cut at the start of an entry.
pub fn append_capped(path: &Path, text: &str, cap: usize) -> std::io::Result<()> {
    let old = fs::read_to_string(path).unwrap_or_default();
    if old.len() + text.len() <= cap {
        return fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
            .write_all(text.as_bytes());
    }
    let mut all = old;
    all.push_str(text);
    let keep_from = all.len().saturating_sub(cap / 2);
    // The first entry that starts inside the kept part; the newest entry
    // whole if even it is longer than that.
    let start = all[..]
        .match_indices("==== Horse Heroes ")
        .map(|(at, _)| at)
        .find(|&at| at >= keep_from)
        .unwrap_or(all.len() - text.len());
    fs::write(path, &all[start..])
}

/// A layer for Bevy's `LogPlugin` that writes the log, without colour codes,
/// to `horse_heroes.log`, made new each run.
pub fn file_layer(_app: &mut App) -> Option<BoxedLayer> {
    let file = fs::File::create(log_dir().join(GAME_LOG)).ok()?;
    Some(
        bevy::log::tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(std::sync::Mutex::new(file))
            .boxed(),
    )
}

/// `HORSE_HEROES_CRASH=1` panics on the first frame, to see the crash log
/// written. A dev aid, for the release build.
pub struct CrashTestPlugin;

impl Plugin for CrashTestPlugin {
    fn build(&self, app: &mut App) {
        if std::env::var_os("HORSE_HEROES_CRASH").is_some() {
            app.add_systems(Update, || {
                panic!("a crash asked for by HORSE_HEROES_CRASH");
            });
        }
    }
}

/// The time now, in UTC, as `2026-10-15 22:45:07 UTC`.
fn utc_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    utc(secs)
}

fn utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rest = secs % 86_400;
    // Days since 1970-01-01 to a civil date (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02} UTC",
        rest / 3_600,
        rest / 60 % 60,
        rest % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(message: &str) -> String {
        report(&Report {
            when: utc(0),
            version: "0.0.0",
            seed: Some(6),
            thread: "main",
            message,
            location: "src/here.rs:1:1",
            backtrace: "0: here\n",
        })
    }

    fn scratch(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("horse_heroes_test_{name}.log"));
        let _ = fs::remove_file(&path);
        path
    }

    #[test]
    fn a_report_says_what_where_version_seed_and_backtrace() {
        let text = entry("it broke");
        for part in [
            "Horse Heroes 0.0.0",
            "1970-01-01 00:00:00 UTC",
            "seed: 6",
            "where: src/here.rs:1:1",
            "message: it broke",
            "0: here",
        ] {
            assert!(text.contains(part), "{part:?} missing from {text}");
        }
    }

    #[test]
    fn a_render_error_says_what_it_was_and_what_the_game_did() {
        let text = render_report(&utc(0), "0.0.0", Some(6), "OutOfMemory", "", "(none)", "quit", "images=3");
        for part in ["Horse Heroes 0.0.0 render error", "seed: 6", "error: OutOfMemory", "the game: quit", "images=3"] {
            assert!(text.contains(part), "{part:?} missing from {text}");
        }
        // It is cut by the cap like a crash.
        assert!(text.starts_with("==== Horse Heroes "));
    }

    #[test]
    fn utc_dates_are_right() {
        assert_eq!(utc(0), "1970-01-01 00:00:00 UTC");
        assert_eq!(utc(951_782_400), "2000-02-29 00:00:00 UTC");
        assert_eq!(utc(1_791_762_307), "2026-10-11 23:45:07 UTC");
    }

    #[test]
    fn a_folder_that_cannot_be_written_is_not_used() {
        assert!(writable(&std::env::temp_dir()));
        assert!(!writable(&std::env::temp_dir().join("horse_heroes_no_such_folder")));
    }

    #[test]
    fn crashes_append_so_earlier_ones_are_kept() {
        let path = scratch("append");
        append_capped(&path, &entry("first"), CRASH_LOG_CAP).unwrap();
        append_capped(&path, &entry("second"), CRASH_LOG_CAP).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("message: first") && text.contains("message: second"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_log_is_capped_and_keeps_the_newest_whole_entries() {
        let path = scratch("cap");
        let cap = entry("x").len() * 6;
        for n in 0..50 {
            append_capped(&path, &entry(&format!("crash {n}")), cap).unwrap();
            let text = fs::read_to_string(&path).unwrap();
            assert!(text.len() <= cap, "{} bytes past a cap of {cap}", text.len());
            assert!(text.starts_with("==== Horse Heroes "), "cut mid-entry");
            assert!(text.contains(&format!("message: crash {n}\n")));
        }
        let text = fs::read_to_string(&path).unwrap();
        assert!(!text.contains("message: crash 0\n"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn an_entry_longer_than_the_cap_is_still_kept() {
        let path = scratch("long");
        append_capped(&path, &entry("first"), 64).unwrap();
        append_capped(&path, &entry("second"), 64).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("message: second") && !text.contains("message: first"));
        let _ = fs::remove_file(&path);
    }
}
