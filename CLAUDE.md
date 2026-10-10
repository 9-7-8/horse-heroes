# Horse Heroes - Rust + Bevy

An isometric incremental idle RPG built around **looting**. The hero fights on
their own; the player's attention goes to what drops. 3D graphics, simple to
start with. The thing being built is the **loot shower** - the burst of items
arcing out of a dead monster, landing, glowing by rarity, and being hoovered
up - done well enough to be satisfying by itself.

The game is called **Horse Heroes** (owner, 2026-10-09). Do not use the name
of any existing game for it, in code, docs, commits or anywhere else. It is
free software under the GPL, version 3 or later (`LICENSE`).

**Premise (owner, 2026-10-09):** a party of three horses (Ranger, Wizard,
Mage) walks right along a side-scrolling isometric fairyland trail, fights
corrupted creatures by itself, and brings back horse gear. Horse models are to
be procedurally generated. What is built so far: `docs/expedition.md`.

This file is the standing rules. Keep it short: what a session built goes in
git history, how a system works goes in `docs/`, a routine goes in
`procedures/`.

---
## Pillars - check a design call against these
1. **Loot feel is the product.** Drop arcs, bounce, rarity beams, pickup
   vacuum, sound and screen response come before content breadth. A new system
   that does not make a drop feel better waits behind one that does.
2. **Idle first.** The game progresses with no input. Input makes it faster or
   more interesting, never merely possible.
3. **Simple 3D.** Primitive meshes and flat colours until the loop is fun. Art
   is replaced late, so nothing may depend on a specific model.
4. **Numbers are data.** Items, affixes, rarities, monsters and drop tables
   live in data files, not in match arms.

---
## Stack
- **Rust** (stable, MSVC toolchain) and **Bevy**. Windows 10 desktop.
- **Pinned: Bevy `=0.20.0`** from crates.io (released 2026-10-08), built with
  Rust 1.99.0, edition 2024. Upgrade deliberately, one release at a time, from
  a committed working state and with the migration guide open.
- Bevy's API changes every release. **Pin the version in `Cargo.toml` and read
  the docs for that version** (docs.rs, the migration guides, the examples in
  the matching git tag) before writing against a system you have not touched
  this session. Do not write Bevy code from recall.
- Add a crate only after checking it supports the pinned Bevy version.

---
## Layout (intended - create as needed)
- `src/sim/` - the game model: stats, combat resolution, loot rolling, offline
  progress. Plain Rust, no Bevy rendering types, unit-tested with `cargo test`.
- `src/game/` - Bevy plugins that drive the sim and present it.
- `src/fx/` - the loot shower and every other piece of juice.
- `assets/data/` - item, affix, monster and drop-table definitions.
- `docs/` - how each system works, one file per system.
- `docs/philosophy/` - advice and guidelines for feel and design calls. Read
  the relevant file before designing or tuning what it covers; keep it current
  (`docs/philosophy/README.md`).

**Keep the sim deterministic and free of rendering.** Loot rolls take a seeded
RNG, so a drop table can be tested by rolling it a million times in a test.

---
## Rules
1. **Build before claiming it works**, and run the game before claiming it
   looks right. `cargo check` proves it compiles and nothing more.
2. **Commit working states.** Small commits, each one building.
3. **Never stop or kill a process you did not start.** A live Minecraft server
   with players on it runs on this machine. Do not stop `java` processes and
   do not touch `C:\Users\Isabel\26.1.2-neoforge-server` or
   `C:\Users\Isabel\Procedural-Horse-Coats`.
4. **Questions go through the question tool**, recommendation first (see the
   global CLAUDE.md).

---
## Planning (owner, 2026-10-09)
Bugs and future features are tracked as **GitHub issues** on
`9-7-8/horse-heroes`, not in plan documents. One feature per issue; group
related ones as sub-issues under an `epic`. Each issue says why (citing a
`docs/philosophy/` file where one applies), what, and "done when". Source
material the owner hands over becomes issues for the work and philosophy files
for the guidance. Check the open issues before starting a feature and close
the issue in the commit that finishes it.

---
## Procedures
Repeatable routines are plain text in `procedures/`; the list is
`procedures/INDEX.txt`. When the owner names one, read the file and follow it.
Add a procedure when a routine has been done twice.
