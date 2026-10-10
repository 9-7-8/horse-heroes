# Combat rhythm

Why a hit looks and feels heavy. The goal is communication and timing, not
realism: readable exaggeration beats naturalistic motion.

**Incomplete.** The source file was cut off partway through its section on
responsiveness. Extend this file when the rest arrives.

## Principles

### What the viewer must be able to read

- When an action has begun.
- When it can deal damage.
- Whether it connected, and what it affected.
- Whether the character can act again.
- What changed in the enemy and the world.

### The four phases of an attack

1. **Anticipation.** A brief wind-up sets direction and power. Too much feels
   unresponsive; too little makes the attack illegible.
2. **Release.** The swing, the launch, the dash, the spell expanding. This
   must line up tightly with the moment damage is applied.
3. **Impact.** The target flashes, recoils, sheds particles, makes a sound,
   loses health, or dies. This is the "yes, it worked" frame.
4. **Recovery.** The attacker returns to a state where it can act, blends into
   movement, or chains into the next action.

Animation can be large and dramatic as long as it never makes the character
wait through recovery it does not need.

### Keep the timings separate

One animation clip must not decide all of these. Each is its own number:

- input acceptance time
- animation start time
- damage (hit) frame
- movement lock duration
- cancellation window
- cooldown start
- visual effect lifetime

### Response before polish

- A beautiful attack that hesitates feels worse than a plain one that starts
  at once. Snappiness is the tight link between input, targeting, action start
  and hit feedback.
- Buffer input briefly: a valid command given just before the current action
  ends runs at the earliest proper moment.
- Allow intentional cancels for movement and defensive actions.
- Protect important commitments. Heavy attacks and long casts may keep their
  commitment as a tactical cost.

## For Horse Heroes

- The horses attack by themselves on sim ticks, so the "hit frame" is fixed by
  the sim. Presentation starts its anticipation *before* the tick so that
  release lands on it. The sim stays the source of truth and stays free of
  rendering (see `CLAUDE.md`, Layout).
- The separate timings become separate named fields in data (pillar 4), per
  attack and per ability: wind-up lead, release, impact effect lifetime,
  recovery. No timing is derived from a model or clip, since art is replaced
  late (pillar 3).
- With primitive meshes, the four phases are done with transforms: lean back,
  lunge, squash, settle. That is enough to judge the rhythm.
- The three abilities (Focused Shot, Elemental Burst, Ward) each need a
  distinct, readable anticipation, since nobody pressed a button to announce
  them.
- Input buffering and cancels barely apply: the only input is in camp. What
  carries over is that a key press there responds on the same frame.

## Source

Owner's notes on loot fountains and combat feedback in a reference action RPG,
received 2026-10-09 (sections 3 to 3.2; the file ends mid-sentence in 3.2).
