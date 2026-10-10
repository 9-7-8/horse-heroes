# Death events

Nothing is taken out of the scene without an event (#17): an enemy that is
brought down breaks apart and leaves a mark, a horse at no health stays in
view exhausted, and enemies the party is driven back from draw off. The
resolution beat of [the loot fountain](philosophy/loot-fountain.md), and
rule 2 of [the feedback chain](philosophy/feedback-chain.md). Horses do not
die ([failure.md](philosophy/failure.md), Tone); death events are for
enemies only.

## An enemy's death

1. The sim's `Kill` carries its enemy's element and the name of its death
   (`death` in `enemies.ron`). `game::advance` passes them on in `Killed`,
   with the body's place and size.
2. The scene stops drawing the enemy in the tick its health reaches zero.
   In the same frame `fx::death` puts a body of its own in the same place,
   so nothing is seen to change.
3. **The fall** (`fall_secs`): the body is thrown `knock_back` away from the
   party, tips `tip_degrees` over, and grows to `swell` times its size,
   standing on the ground throughout (`fall_pose`, `body_transform`).
4. **The break**: the body is cut `per_side` ways along each edge and the
   pieces are thrown out from its middle, up, and away from the party
   (`plan_shards`). They tumble, fall, and stop where they first touch the
   ground. After `linger_secs` they shrink away over `fade_secs`. The
   break is heard: the death's `sound`, a recipe in `sounds.ron`
   ([loot-shower.md](loot-shower.md), Sound), sent as `Heard::Died`.
5. **The mark**: a disc in the element's colour, darkened, on the ground
   where the body broke. It comes up at once and shrinks away over the last
   `mark_fade_secs` of its `mark_secs`.

When the party walks on, the ground takes the shards and the mark with it,
as it does the drops.

## Ordinary and final

Two deaths are in `assets/data/deaths.ron`, and a new one is one more entry
there and its name in `enemies.ron`.

| | `small` | `big` |
| --- | --- | --- |
| Named by | ordinary enemies | a trail's last enemy |
| The fall | 0.1 s, knocked away | 0.3 s, rears back and swells by a quarter |
| Shards | 8 | 27, thrown higher |
| Shards gone after | 1.1 s | 1.7 s |
| Mark | 0.6 body sizes, 2.5 s | 0.9 body sizes, 6 s |
| Sound | `death_small`, a short dry crack | `death_big`, the crack and a low boom falling away under it |

The big fall is as long as the full fountain's `delay_secs`, so the drops
come out of the break, and the death's sound and the launch's start
together. An ordinary enemy's crack comes a tenth of a second after its
fountain's pop. The shards are gone before the fountain's pieces
are at rest, so the pile is read with nothing lying over it.

## An exhausted horse

A horse at no health is not hidden (`src/game/scene.rs`, `sync_horses`,
`tired_pose`). While the party stands to fight it lies down: its belly
comes to the ground, its legs fold forward and its head drops to the
trail. When the party walks on it gets up and comes along with its head
low and a shorter stride. In town every horse is on its feet again. The
pose's numbers are the `TIRED_` constants at the top of `scene.rs`.

The neck and head hang from one joint at the withers (`Neck`), which is all
the pose needs of the model; the rebuilt horse ([horse-model.md](horse-model.md)) keeps that joint,
its `Leg`s, and a leg length, which is the clearance under its chest.

## A party driven back

A lost trail goes to town at once. Enemies still standing then are handed
to `fx::death` in a `Withdrew` message and draw off to the right, slowly
and then fast (`withdraw_speed`, `withdraw_accel`), until they are out of
view.

## Tests

`cargo test` (in `src/fx/death.rs` unless said):

- the numbers are usable, and every enemy names a death there is;
- a falling body only goes back, over and bigger, starts as the body the
  scene drew, and stays on the ground;
- a body breaks into the shards its death says, which fill it, leave upward
  and away from the party, and follow from the seed;
- every shard lands on the trail, clear of the party, before it starts to
  shrink away;
- shards and marks go in their time, and a mark outlasts its shards;
- a final's death is bigger and slower than an ordinary one's in every
  number, and any death is over before the next enemies arrive;
- a body has broken by the time its fountain throws its first piece;
- a final's sound is told from an ordinary's (twice as long, an octave
  lower, and used for nothing else), and a death and its launch played
  together stay inside full scale (`src/fx/sound.rs`);
- a kill says how its enemy dies (`src/sim/expedition.rs`);
- an exhausted horse lies down only while the party stands
  (`src/game/scene.rs`).

## Not built

- Nobody has listened to a death (#241). The recipes are first guesses,
  and whether the death covers the launch is for an ear to say. A run with
  `--seed 6` has an ordinary death at 21.3 s and the final one at 64.5 s.
- A lost trail is not held on the road, so the last horse to tire is seen
  getting up in town and never lying down. Whether to hold it is #176's.
- A hit shows nothing yet: no stagger and no flash before the death.
- Nobody has watched it move. It was checked from still pictures:
  [gallery](gallery/2026-10-10-death-events/README.md).
