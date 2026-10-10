# Juice

The sensory and timing feedback that makes an interaction feel present,
responsive and rewarding. The loot fountain is the largest case and has its
own file ([loot-fountain.md](loot-fountain.md)); hits and attacks are in
[combat-rhythm.md](combat-rhythm.md). This file is the general rules, and
covers everything else: decisions, upgrades, thresholds, returning.

## Principles

### Juice is not a substitute for mechanics

A bad loop with fireworks is still a bad loop. But a good loop with no
feedback feels dead: the player cannot tell what happened, whether it
mattered, or why to care.

### The four questions

Every meaningful action must answer, at once:

1. Did the game receive my action?
2. Did it succeed?
3. What changed because of it?
4. How important was that change?

In an idle game the "action" is often not a button mashed in combat. It is
equipping a piece, choosing a policy, claiming a report, crossing a threshold,
finding something rare, coming back after time away.

### Juice the decision, not every number

If every small gain gets a big animation, the player stops seeing any of it.
Keep three levels and never spend a higher one on a lower event.

| Level | For | Treatment |
| --- | --- | --- |
| Low | Routine gains | Quiet and aggregated: a counter ticking, small icons drifting to it, a sound at intervals, common loot in a stack |
| Medium | Decisions and thresholds | Visible: an upgrade, a new policy set, a bottleneck removed, a new enemy beaten, a possible upgrade found |
| High | Rare, game-changing, first-time | A major reveal with room around it: a top-tier drop, a set threshold, a first boss kill, a build gaining a new behaviour |

The player should feel the difference between "+12 of something" and "this
piece may rebuild my party".

### Animate the state change, not just the button

A click sound and a bounce on the button acknowledge the press. The valuable
animation shows what is now different in the game: the thing grows, the gear
appears on the character, the ability icon gains a mark, the preview numbers
move, the next fight plays out differently.

The player must believe the build changed **before** waiting for the next
result to prove it.

### Anticipation, release, aftermath

1. **Anticipation.** Something may be about to happen: a chest shakes, a
   health bar empties, a counter reads 2 of 3.
2. **Release.** The result is revealed.
3. **Aftermath.** Time and tools to understand it: the item lingers, what it
   affects is highlighted, a comparison explains the difference, and it can be
   acted on there.

Do not bury a significant result under a long summary.

### Sound gives numbers weight

Incremental systems are abstract; sound makes them physical. Sound must match
meaning:

- small gains: short and light;
- investments: weighty and deliberate;
- a top-tier discovery: unique and memorable;
- failure: clear, never humiliating or harsh;
- achievement: celebratory, but short enough to hear a hundred times;
- returning after absence: soft, not a blast.

Never a sound per tick. Aggregate repeated events, or vary them.

### Motion explains causality

Motion is most useful when it shows where something came from and where it
went: coins fly from the enemy to the counter, a scrapped piece dissolves into
the material it became, an equipped piece travels to its slot, a set piece
lights its bonus. That makes the interface a world with systems, not a
spreadsheet.

### Juice must never hide information

Effects, beams and counters become clutter. Use the three levels, aggregate
the routine, reserve spectacle. Warnings and build-relevant information must
be stronger than decoration. Provide settings to turn effects down.

## For Horse Heroes

- The three levels should be one shared scale in data that every effect reads
  (shake, flash, sound, duration), so importance is consistent across the
  fountain, camp and combat, and can be tuned in one place
  (`procedures/tune-loot-feel.txt`).
- Camp is where most actions happen and it is text with key presses. Question
  1 (did the game receive it?) is the cheapest gap to close.
- The horses are boxes. "Animate the state change" still applies with
  primitives: a new bridle is a coloured band on the head box. Gear shown on
  the horse must not depend on a model (pillar 3).
- Juice is the product here (pillar 1), which makes restraint more important,
  not less: the fountain only stays special if nothing else is spending at the
  high level.
- The owner's verdict decides feel. Nothing in this file is done on the
  strength of a build (rule 1).

## Source

Owner's notes on the MDA framework and juice applied to incremental, idle and
idle-RPG loops, received 2026-10-09 (sections 4, 5, 9.1, 11.6).
