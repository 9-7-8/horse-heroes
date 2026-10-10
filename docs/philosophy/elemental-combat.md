# Elemental combat

Damage types and how they combine are the central combat and build interest
(confirmed; see [vision.md](vision.md)). Almost everything else in this file
is **proposed**: it came out of the owner's design discussion and awaits her
decision. The triangle is the only rule that is built.

## Principles

### Built and kept

Fire beats Plant, Plant beats Ice, Ice beats Fire.

### Each element has an identity, not just a matchup (proposed)

| Element | Identity | Setup status | Good at |
| --- | --- | --- | --- |
| Fire | Pressure and release | Burning | Finishing targets, spreading damage |
| Ice | Control and timing | Chilled | Reducing danger, preparing bursts |
| Plant | Accumulation and sustain | Seeded | Repeated hits, growth payoffs |

An element should change how a fight goes, not only which number is
multiplied.

### Pairs react (proposed)

| Pair | Reaction | Effect |
| --- | --- | --- |
| Fire + Ice | Thermal Shock | Concentrated burst on one target |
| Fire + Plant | Wildbloom | Damage to nearby enemies |
| Ice + Plant | Wintergarden | A temporary barrier on the party |

Rules suggested for a first version, chosen to stay readable and to end:

- a pair reacts in either order;
- one setup status per enemy at a time;
- a hit of the same element refreshes or strengthens the setup;
- a hit of a different element consumes it and triggers one reaction;
- **a reaction never triggers another reaction**;
- a short cooldown per target prevents runaway chains.

Start with one reaction, then test all three.

### Mixed must not simply beat same (proposed)

A party of one element stays useful. Routes to that: a stronger Burn, a
stronger but capped slow, a stronger Plant growth payoff. If a mixed party is
always better, element is no longer a choice.

### Class, element, personality (proposed)

Three separate questions about a horse:

- **Class:** how does it act?
- **Element:** what does its magic do?
- **Personality:** who is it?

| Class | Pattern | Role in a combination |
| --- | --- | --- |
| Ranger | Frequent focused attacks | Reliable application of a setup |
| Wizard | Slower, substantial attacks | Strong payoff from a reaction |
| Mage | Wards and restorative magic | Keeping the party alive |

**Each horse has its own attunement**, in place of one element for the whole
party, so that differently attuned horses can actually cooperate. Basic
attacks may stay neutral at first, for readability.

### Automation

The baseline is competent without help. Gear and policies specialise how the
party sequences its abilities; they are never needed to fix automation that
was made poor on purpose.

## For Horse Heroes

- The slice has one element for the whole party, chosen in camp. A reaction
  needs two elements present, so individual attunement is the first step, and
  it is the step awaiting her confirmation.
- Reactions are triggered effects, and the rule that they never chain is the
  same rule as bounded effect depth in
  [procedural-generation.md](procedural-generation.md). One rule in the sim
  should serve reactions and gear effects alike.
- A reaction is a moment in the feedback chain and has to be seen: a setup
  visible on the enemy, and a reaction distinct enough to name
  ([combat-rhythm.md](combat-rhythm.md)). An invisible reaction is a
  multiplier.
- The class patterns map onto what exists: the Ranger's Focused Shot, the
  Wizard's Elemental Burst and the Mage's Ward. Under the proposal the
  Ranger sets up, the Wizard cashes in, and the Mage's kit gains restoration.
- The guardrail in the vision says no more damage types until the main loop
  works. Three elements and three reactions is the whole budget.
- The balance test (`first_trail_balance`) assumes one party element. Any of
  this changes its meaning and it must be rewritten with it, not left
  passing by accident.

## Source

Owner's design discussion summary, received 2026-10-09 (sections 7 and 8).
