# Damage numbers

Each hit that lands in a fight shows its damage as a number that rises from
what it struck and fades (#249). Guidance: `docs/philosophy/juice.md`,
`docs/philosophy/combat-rhythm.md`.

## Where they come from

The sim tells every hit as it lands: `Expedition::hits` in
`src/sim/expedition.rs`, a `Hit` with who was struck (`Struck::Foe` with its
place and full health, or `Struck::Horse`), the damage the fight log says, the
element of its elemental part if it had one, and its kind: `Plain`, `Strong`,
`Resisted`, or `Warded` for an enemy's hit softened by the Ward. Nothing reads
them in the sim; `game::advance` takes them each tick, as it takes the kills,
and sends a `Landed` message with the middle of the body struck.
`every_hit_is_told_as_the_log_says_it` holds the hits to the log line for line.

## How they look

`src/fx/numbers.rs` draws each as UI text placed over the scene, under the
menus (`GlobalZIndex(-1)`), so a number never covers a control.

- **Size** grows with the square root of the damage, from `small` at 1 to
  `large` at `big_damage`. A Strong hit is `strong_scale` bigger and ends with
  "!"; a Resisted or Warded one is `weak_scale` smaller and `weak_alpha` solid.
- **Colour**: a horse's hit is its element's colour, lightened, when it had an
  elemental part, and `physical` when not. An enemy's hit on a horse is
  `taken`, a hue no element has, and starts with a minus.
- **Motion**: it pops in `pop` times its size and settles over `pop_secs`,
  rises `rise` world units, slowing, over `life_secs`, and fades over the last
  `fade_secs`.
- **Many at once**: hits on one thing within half a number's life are set
  side by side, `spread` pixels apart, `row` to a row, the next row
  `row_height` higher. A Burst puts one on each enemy. A plain hit has no element (#357), so
only an ability's hit is in an element's colour. Past `most`
  numbers the oldest go first.

Every value is in `assets/data/damage_numbers.ron`.

## Turning them off

**Damage numbers** in Settings (key D) is a switch, on in a new game. Off, the
numbers in the air go and no new one is shown. It changes nothing in the game.
When the feel settings are built (#45) it may move there.

## Its lesson

"the numbers are hits" in `assets/data/lessons.ron`: due at the first kill,
shown on the road, done by watching the next kill. At `--seed 6
--keep-going` it is on the hint's line at 32 s.

## Looking at it

`--seed 6` with `IDLE_SHOT_AT=5.4` has the first fight's opening:
Sable's 6! and then Morrow's 11! over the Mire Toad, one after the other
(#354); at 6.2 Juniper's 8! has come, and -3 over Morrow. (Before #354 the
three came on one tick, side by side; checked 2026-10-15.)
