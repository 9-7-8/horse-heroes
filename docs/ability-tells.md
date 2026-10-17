# Ability tells

Nobody presses a button to announce an ability, so each one shows itself
coming and going, and can be named from the motion alone with the HUD
hidden (#18; `docs/philosophy/combat-rhythm.md`). Each has the four phases
of an attack: a wind-up, a release that lands on the step the sim uses it,
an impact (the hit reactions of [hit-reactions.md](hit-reactions.md)) and a
recovery.

## When

The sim decides when an ability is used and tells the presentation ahead
of time: `Expedition::cast_in(horse, dt)` counts the steps to it as
`swing_in` does for an ordinary hit, and a test holds that every ability
is used on the step it said. Each use is told once, with the horse that
used it (`Used`, the `Cast` message). The steps run `Game::behind()`
seconds behind the clock, which the presentation takes off.

## The horse

Each ability has its own swing in `assets/data/attacks.ron`, `casts`, by
its id: `wind_up`, `release`, `recovery`, `rear` and `lunge`, as an
ordinary hit's ([horse-model.md](horse-model.md)). A horse goes into it once
its ability is closer than its wind-up. Every ability's wind-up is longer
and its rear deeper than an ordinary hit's (a test), so a horse drawing
far back is read as an ability coming.

- **A shot** (Ember Shot, Thorn Shot) draws far back and drives in.
- **A burst** (Frost Burst) rears high and stamps down.
- **The Ward** rears and settles upright as it is raised; no lunge.

## What it throws

`fx::tells`, every number in `assets/data/tells.ron`, timed by the same
swing. A shot and a burst take their ability's element colour.

- **Orb (a shot).** Through the wind-up an orb grows in front of the
  horse's chest, pulsing; as the release starts it is thrown in an arc at
  the first enemy standing and reaches it as the hit lands, where the
  hit's sparks take over.
- **Ring (a burst).** Through the wind-up a ring of shards closes in on
  the ground round the horse; on the release it rushes out to just past
  the furthest enemy standing, through them as the hits land, and runs on
  `beyond` further as it shrinks away. It spreads no further across the
  trail than `across`, so a wide ring is a long ellipse on the ground (the
  first try left the trail and filled the sky).
- **Ward.** Through the wind-up a pale glow swells round the horse raising
  it. As it is raised a bubble closes over every horse standing, growing
  in over `grow_secs`, and holds while the Ward lasts (it is frozen with
  the Ward between encounters). In its last `warn_secs` it flickers
  between whole and faint; when the Ward ends, or the trail does, it
  swells by `pop` and fades in `pop_secs`.

Where each is at a moment is plain arithmetic (`orb_at`, `ring_closing`,
`ring_after`, `Ring::shard`, `bubble_at`, `popped`), unit-tested; the
system only draws it.

## Seen

`--seed 6`. The log line `tell: <ability> by horse N at T`
(`RUST_LOG=info,horse_heroes::fx::tells=debug`) says when each is used:
the Ward at 6.0, Ember Shot at 8.1, Frost Burst at 9.1. At 5.85 the Ward's
glow round Sable; at 7.0 the bubbles; at 7.8 the orb gathering in front of
Juniper and at 8.03 in flight; at 8.7 the ring closed round Morrow; at 9.07
rushing out; at 9.5 the bubbles faint in their flicker; by 10.35 gone.
Pictures: `docs/gallery/2026-10-16-ability-tells`.
