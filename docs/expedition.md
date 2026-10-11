# Horse Heroes: the expedition loop (smallest slice)

Camp -> trail -> camp, forever. It runs with no input; input only changes the
element, equips early, or skips the wait.

## Run it

    cargo run                  # seed from the clock
    cargo run -- --seed 42     # fixed first trail; later trails are 43, 44, ...
    cargo test                 # the sim, including a balance check

Keys, in camp only: `1` Fire, `2` Ice, `3` Plant, `E` equip the drop,
`Space` set out now. Camp leaves by itself after 12 s and equips the drop
first if it is an upgrade.

A trail that is over, won or lost, ends with a walk back to town
(`Game::secs_to_town`, as long as `travel_secs`): the party is in town only
when it has reached the town gate, and it stands still there. The walk is
presentation, in `src/game/mod.rs`; the sim's trail is already finished.

## Where things are

| Part | File |
| --- | --- |
| Numbers: rules, horses, enemies, encounter pieces, gear | `assets/data/*.ron` (compiled in with `include_str!`) |
| Data types and loading | `src/sim/data.rs` |
| Trail generation (`Plan`) and the fight (`Expedition`) | `src/sim/expedition.rs` |
| Horses, gear, drop roll | `src/sim/loot.rs` |
| Seeded RNG | `src/sim/rng.rs` |
| Camp/trail loop, input, report text | `src/game/mod.rs` |
| Where the report and the controls are shown | `src/game/menu/shell.rs` ([menu-shell.md](menu-shell.md)) |
| Camera, trail, enemy cubes | `src/game/scene.rs` |
| The horses' shape | `assets/data/horse_shape.ron`, `src/sim/horse_shape.rs`, `src/game/scene.rs` ([horse-model.md](horse-model.md)) |
| A kill's coins flying out and landing | `src/fx/drops.rs` ([loot-shower.md](loot-shower.md)) |
| An enemy's death, an exhausted horse | `src/fx/death.rs`, `src/game/scene.rs` ([death-events.md](death-events.md)) |

## Rules as built

- Party: Juniper, Morrow, Sable. Fixed; there is no horse picker yet. There
  are no classes. Each horse has one ability, picked by the `class` field in
  `horses.ron` (Ranger, Wizard, Mage), which is code-only and never shown; it
  goes with the shared pool of abilities (#221).
- A trail is 3 ordinary encounters drawn from 5 hand-written pieces in a rolled
  order, then 1 of 2 finals. Each enemy's element is rolled. A quarter of
  trails have a fairy spring that heals the party after one encounter.
- Every hit is `max(attack - defense, 0) + elemental x triangle`, at least 1.
  Triangle: Fire > Plant > Ice > Fire, x1.5 strong, x0.75 resisted.
- Juniper: Focused Shot, x2.5 attack on the current target every 4 s.
  Morrow: Elemental Burst, x2 elemental on every enemy every 5 s.
  Sable: Ward, halves damage taken for 4 s every 8 s.
- One gear drop per run if at least one encounter was cleared; a win shifts
  the rarity weights up. Each piece fits one named horse and one of three
  slots (Head, Body, Saddle). On a win the piece comes out of the last
  enemy to fall (`Kill::gear`); on a loss it is only reported in town.
- A rarity is a line in `gear.ron`, lowest first, and a piece names its
  tier by its place in that list. There is no rarity enum in the code.
- A win raises the trail level: enemies gain 15% health and attack per level,
  drops gain one stat point per level.
- The sim steps in fixed 0.1 s ticks. Trail, fight and drop each follow from
  the seed (plus party and element for the fight), which the tests check.
- Each rule above has a test in `src/sim/expedition.rs` or `src/sim/loot.rs`.
  A changed rule should fail one of them.
- An ability is ready for the first time one whole cooldown into the run
  (the Ward a quarter of one), and its timer runs only while fighting.

## Balance at trail level 0, no gear (300 seeds)

Win rate 68% across all element choices; 94% with the best element for the
trail, 44% with the worst. `first_trail_balance` fails if this drifts out of
25-75% or the element stops mattering. A run takes about a minute.

## Not built yet

Horse choice, buttons (keys only), attack and hit animation, the
rest of the loot shower (gear in it, rarity, pickup), sound, saving, spending coins, a second trail, horses that
differ in shape (all three are the base horse, [horse-model.md](horse-model.md)).
