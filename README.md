# Horse Heroes

An idle RPG about a party of three horses. They walk a fairyland trail that is
being corrupted, fight what they meet without being told to, and bring back
horse gear that helps them get farther next time.

This is an early prototype: boxes for horses, cubes for enemies, white text on
a black background. It launches, plays itself, and can be nudged.

## What is in it

- A party of three: a Ranger (single-target damage), a Wizard (damage to every
  enemy) and a Mage (a ward that halves damage taken).
- Three elements: Fire beats Plant, Plant beats Ice, Ice beats Fire. You pick
  the party's element before each expedition and can see the trail ahead.
- Generated expeditions: three encounters and a final enemy, different every
  run and reproducible from a seed.
- Horse gear: one drop per expedition, for a head, body or saddle slot, in
  three rarities.
- Idle by default: leave it alone and it keeps setting out, equipping upgrades
  as they drop. Win and the trail gets harder.

## Running it

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

In camp, between expeditions:

| Key | Action |
| --- | --- |
| `1` `2` `3` | Attune the party to Fire, Ice or Plant |
| `E` | Equip the gear that just dropped |
| `Space` | Set out now, without waiting for the countdown |

## How it is put together

Built with [Bevy](https://bevy.org) 0.20. The game rules live in `src/sim/` as
plain Rust with no engine types, so they can be tested by running thousands of
expeditions in a unit test. `src/game/` draws the result. The numbers - horses,
enemies, encounters, gear - are in `assets/data/`. More detail is in
[`docs/expedition.md`](docs/expedition.md).

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
