# Horse Heroes

An idle RPG about a party of three horses. They walk a fairyland trail that is
being corrupted, fight what they meet without being told to, and bring back
horse gear that helps them get farther next time.

This is an early prototype: boxes for horses, cubes for enemies, and flat
menus under the scene, most of whose places are not built yet. It launches,
plays itself, and can be nudged. Pictures: [`docs/gallery`](docs/gallery/README.md).

## What is in it

- A party of three, each with one ability for now: Juniper's Focused Shot
  (single-target damage), Morrow's Elemental Burst (damage to every enemy)
  and Sable's Ward (halves damage taken).
- Three elements: Fire beats Plant, Plant beats Ice, Ice beats Fire. You pick
  the party's element before each expedition and can see the trail ahead.
- Generated expeditions: three encounters and a final enemy, different every
  run and reproducible from a seed.
- Horse gear: one drop per expedition, for a head, body or saddle slot, in
  three rarities.
- Idle by default: leave it alone and it keeps setting out, equipping upgrades
  as they drop. Win and the trail gets harder.

## Installing it

On Windows 10 or 11 (x64), download `HorseHeroes-Setup.exe` from the
[latest release](https://github.com/9-7-8/horse-heroes/releases/latest) and
run it. It installs for your user only and asks for no administrator rights.

The installer is not code-signed yet, so Windows SmartScreen will say it does
not recognise the app; "More info" shows the choice to run it. Each release
lists the file's SHA-256 so you can check what you downloaded.

## Building it

You need [Rust](https://rustup.rs) (stable). On Windows you also need the
Visual Studio C++ build tools; on Linux, the
[Bevy dependencies](https://bevy.org/learn/quick-start/getting-started/setup/)
for your distribution.

    cargo run                  # a new trail every launch
    cargo run -- --seed 42     # a fixed first trail
    cargo test                 # the game rules, including a balance check

The first build compiles the Bevy engine and takes several minutes. Later
builds take seconds.

## Controls

The menus under the scene work with the mouse or the keyboard: five places
in a bar, a horse to select, and Set out in the footer. Every key is printed
on the control it works. The last trail's report is in Journal, Report.

| Key | Action |
| --- | --- |
| `H` `T` `M` `N` `J` | Open Horses, Trade, Map, Town or Journal |
| `1` to `4` | Choose a tab of the open place |
| `Z` `X` `C` | Select the first, second or third horse |
| `F` `I` `P` | In town: attune the party to Fire, Ice or Plant |
| `E` | In town: equip the gear that just dropped |
| `Space` | In town: set out now, without waiting for the countdown |
| `Esc` | Close the side panel; on the road, close the open place |
| `Tab` `Enter` | Move to any control and press it |

## How it is put together

Built with [Bevy](https://bevy.org) 0.20. The game rules live in `src/sim/` as
plain Rust with no engine types, so they can be tested by running thousands of
expeditions in a unit test. `src/game/` draws the result. The numbers - horses,
enemies, encounters, gear - are in `assets/data/`. More detail is in
[`docs/expedition.md`](docs/expedition.md) and
[`docs/menu-shell.md`](docs/menu-shell.md).

## License

Horse Heroes is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

In short: you may use, study, share and change it, and if you pass it on -
changed or not - you must pass on the same freedoms and the source code with
it.

It is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE. See [`LICENSE`](LICENSE) for the full text.
