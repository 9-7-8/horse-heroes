# Crash log

The release build opens no console, so without this a panic's message and the
game's log go nowhere (#364). `src/crash_log.rs` writes two files beside
`horse_heroes.exe` when the game may write there (a build, or a per-user
install), and otherwise in `%LOCALAPPDATA%\Horse Heroes` - the default
install is under Program Files, where a player cannot write. The game has no
save folder yet; when it has one, 0.3.2, they move there. The uninstaller
removes the two beside the exe and leaves those in `%LOCALAPPDATA%`:

- **`crash.log`** - one entry per panic, appended, so earlier crashes are kept:
  the version, the time in UTC, the seed (`--seed N` plays the run again), the
  thread, where it panicked, the message and a full backtrace. Once it would
  pass 256 KiB it is cut back to its newest whole entries, at most half that.
  Written by a panic hook set in `main` before the app is built; the hook
  already set (Rust's own, which prints to stderr) still runs after it.
- **`horse_heroes.log`** - Bevy's log for this run, the same lines a console
  would show, without colour codes. Made new each run. A crash that is not a
  panic (a GPU or driver error) still leaves its last lines here. It is added
  to Bevy's `LogPlugin` as a `custom_layer`, so `RUST_LOG` filters it too.

Nothing shows in the game. To see it work, run the release build with
`HORSE_HEROES_CRASH=1`: it panics on the first frame.
