# Horse Heroes: the expedition loop (smallest slice)

Camp -> trail -> camp, forever. It runs with no input; input only changes the
element, equips early, or skips the wait.

## Run it

    cargo run                  # seed from the clock
    cargo run -- --seed 42     # fixed first trail; later trails are 43, 44, ...
    cargo test                 # the sim, including a balance check

Keys, in camp only: `1` Fire, `2` Ice, `3` Plant, `E` equip the drop,
`Space` set out. No town leaves by itself (#305): the party waits for
Space unless Keep going (`G`, on the Map) is on, and then it leaves after
12 s. Either way nothing is equipped at the gate (#351): a piece is worn
only because the player put it on a horse. A lost trail turns Keep going
off.

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
- Each horse hits at its own cadence (#354): `attack_interval` per horse in
  `horses.ron` (Juniper 1.4 s, Sable 1.55, Morrow 1.6), each swing up to
  `attack_jitter` (10%) sooner or later from the trail's own seeded RNG.
  Each fight opens out of step: the first hits come `opening` (0.8 s) in
  and `opening_stagger` (0.35 s) apart, in an order rolled per fight; the
  abilities' timers run on from the last fight. Over trails the horses hit
  on the same tick about one time in twenty (test
  `over_a_trail_the_horses_rarely_hit_together`).
- Every hit is `max(attack - defense, 0) + elemental x triangle`, at least 1.
  Triangle: Fire > Plant > Ice > Fire, x1.5 strong, x0.75 resisted.
- Juniper: Focused Shot, x2.5 attack on the current target every 4 s.
  Morrow: Elemental Burst, x2 elemental on every enemy every 5 s.
  Sable: Ward, halves damage taken for 4 s every 8 s.
- Every kill throws gear (#352): a least and a most per enemy, `gear` in
  `enemies.ron` (1-2 for the small ones, 2-4 for a final), the same at
  every trail level, rolled at each rarity's `kill_weight` in `gear.ron`,
  which is Normal and Magic only: the chaff. On top of it the trail rolls
  its own piece if at least one encounter was cleared, at `weight`, or
  `win_weight` on a win; it is the only piece that can be Rare or higher,
  so the higher tiers come per trail as often as when it was the only
  piece. A won trail brings back ten pieces or so (seed 6's first: 10). `Outcome::drops`
  holds them all in the order they dropped, the trail's own last
  (`trail_piece`); `best` is the one the report names, and the report
  counts them. The kills' pieces are rolled from their own seeded RNG, so
  the fight goes as it did. Each piece fits one of three slots (Head, Body,
  Saddle) and any horse can wear it (owner, 2026-10-11, #250): Equip puts
  it on the selected horse. Nothing equips by itself (owner, 2026-10-12,
  #351): a party that leaves town with the piece still in cargo wears what
  it wore and still carries the piece, and the game names no horse for it
  (`setting_out_equips_nothing`). The report says what was brought back and
  that it is carried (item-card.md). The cargo (`Game::cargo`) keeps every
  piece, newest first, on the road as in town (#352): a piece left alone
  stays carried, a worn piece that a carried one replaces takes its
  place in the cargo, and nothing leaves the cargo but onto a horse. It
  has no capacity yet (#34), so it only grows until selling and salvage
  exist. A kill's pieces come out of its body (`Kill::gear`); on a win the
  trail's piece comes out of the last enemy to fall, after its chaff; on a
  loss the trail's piece is only reported in town.
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
