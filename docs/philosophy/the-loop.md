# The loop

The whole game seen at once: what the player does, in what order, and what
they should feel. Use this file to judge whether a feature belongs, and the
checklist at the end before adding content.

## Principles

### Four layers

    Mechanics create the possibility of satisfaction.
    Dynamics create the repeated player behaviour.
    Aesthetics create the emotion.
    Juice makes every meaningful step felt.

- **Mechanics** are the rules and data: formulas, tables, slots, cooldowns.
- **Dynamics** are what happens when those rules meet a player's choices over
  time: farming a lower level because it clears twice as fast, changing a
  policy to fix a weakness.
- **Aesthetics** are the feelings that result.

**We build from mechanics outward; the player experiences from aesthetics
inward.** The developer thinks "I added a set bonus". The player feels "my
party finally has its identity". Nobody cares about a mechanic in isolation,
so design each one backwards from the feeling and the behaviour it is for.

### What the player should be able to say

1. I understand why I progressed.
2. My decision caused this change.
3. That drop gives me a new idea.
4. My build behaves differently now.
5. I have a clear reason to return and make the next decision.

A game that only produces numbers while the player is away has failed, however
large the numbers.

### The feelings to aim for

- **Sensation:** this drop, this upgrade, this result feels good.
- **Discovery:** I did not know those two effects could combine.
- **Expression:** this build is mine.
- **Challenge:** I can see why this level beats me, and I have a plan.
- **Fantasy:** I am not raising a damage number; I am becoming something.
- **Submission:** I can check in comfortably, decide, and let it run.
- **Anticipation:** the next chest, boss or threshold could matter.
- **Mastery:** I understand it well enough to make an odd build work.
- **Relief and pride:** the boss that blocked me is now a farm target.

### The behaviour to produce, and to prevent

Wanted: comparing a drop against the build; noticing a drop that suggests a
different build and keeping it; choosing between farming and pushing; changing
a policy to fix a real weakness; changing the build for a boss or for
unattended reliability; targeting a place for a needed piece; turning
duplicates into progress; returning because the report poses a decision;
experimenting instead of following one answer.

Not wanted: equipping whatever has the highest score without thought; ignoring
item text; one build for everything; leaving the game running because there is
no reason to come back; sorting junk; repetitive claim buttons; feeling
punished for being away; every system being one more currency; needing an
outside guide because the game does not explain its own interactions.

### The loop

Not this:

    wait -> collect -> wait

This:

    intention -> build choice -> automated test -> visible result
      -> rewarding reveal -> understandable lesson -> new hypothesis
      -> a stronger or stranger next attempt

1. **Choose an intention.** A run is for something: a missing piece, testing
   a new effect, pushing a level, checking that a change fixed a failure. The
   player chooses a purpose, not merely "start".
2. **Configure the build.** Gear, abilities, policies, formation, level,
   place. The screen shows consequences: what the build is good and bad at.
   It guides without replacing experiment.
3. **Observe or delegate.** Watched or not, the run leaves evidence: where
   the damage came from, what fired, what was prevented, what killed the
   party. For an unwatched run, a few highlights, not every second.
4. **Reveal and appraise.** The reward ceremony
   ([loot-fountain.md](loot-fountain.md)). A highlighted item explains why it
   is relevant. It is a design prompt, not a stat sheet.
5. **Form the next hypothesis.** "This fixes my weakness", "this changes my
   build", "one more piece", "I can push a level". If a reward leaves the
   player with no such thought, it was a dead end.

**Progress is authorship.** The player forms an intention, creates a build,
delegates a plan, sees evidence, finds something interesting and makes a new
guess.

## The checklist

Run this before adding content. A "no" is worth more than a new feature.

**Mechanics**
- Can the player identify their current bottleneck?
- Can they explain what an upgrade changes?
- Does every currency have a recognisable source and purpose?
- Does every item category have a use?
- Can automated behaviour be configured?
- Do different builds perform measurably differently?
- Can a needed reward be targeted?
- Is offline progress bounded and explainable?

**Dynamics**
- Is there a meaningful decision after most major rewards?
- Do players sometimes farm instead of push, and push instead of farm?
- Do drops create ideas for other builds?
- Do failures expose a real weakness?
- Does a new build change which activity the player wants to run?
- Do players return with a clear next objective?
- Can an unconventional build succeed somewhere?

**Aesthetics**
- Does a rare item feel different from a common one?
- Does the player feel they own their build?
- Does a result produce anticipation, relief, excitement or useful
  frustration?
- Does progress feel like power, discovery, identity or mastery, not only
  larger numbers?
- Is the game comfortable to leave and return to?

**Juice**
- Is every important action acknowledged at once?
- Do effects communicate what changed?
- Do high-value events get space and time to land?
- Are routine events restrained, to the eye and the ear?
- Can a result be understood without watching all of it?
- Does equipping an item visibly demonstrate its consequence?
- Do sound, animation, colour, motion and UI agree about importance?

## For Horse Heroes

- Against the five statements today: 1 and 2 are weak (a trail is won or lost
  with no account of why), 3 and 4 are not yet possible (gear is stat points),
  5 is absent (camp leaves by itself and nothing is posed).
- The loop as built is close to `wait -> collect -> wait` with an element
  choice. The missing steps are 1, 3 and 5: intention, evidence, hypothesis.
- Pillar 2 and "choose an intention" must both hold. The answer is a default:
  the game picks a sensible goal and states it, and the player may pick
  another. An intention that has to be chosen before anything happens would
  break idle-first.
- Fantasy is already supplied by the premise: three horses, a fairyland trail,
  corrupted creatures. A build should read as that party's character, so gear
  and effects are named and shown in that world, not as bare modifiers.
- Fellowship is out of scope. There are no social features planned.

## Source

Owner's notes on the MDA framework and juice applied to incremental, idle and
idle-RPG loops, received 2026-10-09 (purpose, sections 1, 6, 7, 8, 12, 13).
