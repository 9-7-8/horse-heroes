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

## Render errors

Bevy 0.20 meets a GPU error (wgpu's out-of-memory, validation, internal or
device-lost) without a panic, and its own handler quits the game for any of
them, so the panic hook never ran (#374). `src/render_errors.rs` puts in a
`RenderErrorHandler` of its own:

- **Out of GPU memory** - the picture stops for 2 s while the game plays on
  (Bevy skips its render schedules; the window keeps its last frame), then
  every image is marked changed, so each is sent to the GPU again, and
  drawing is tried once more. A validation or internal error within a minute
  of an out-of-memory one is its aftermath (a texture that was never made)
  and is met the same way. At most one try every 2 s, so a failure that keeps
  coming cannot flash the screen.
- **More than 8 in a minute, or any other render error** - written down and
  the game quits, as Bevy's handler did.

Each run of them is an entry in `crash.log`: "render error", the version,
time, seed, the error and its source, what the game did, and how many images,
meshes and materials it held. The first of a run is written; the rest, each
pause and each retry, are in `horse_heroes.log`.

To see either way through, run with `HORSE_HEROES_RENDER_ERROR=quit` or
`=oom`: five seconds in, the game makes a buffer wgpu refuses. `quit` is an
error with nothing before it, so the game quits; `oom` counts an
out-of-memory error as just met (one cannot be made on purpose), so the
error is its aftermath and the game pauses and goes on.
