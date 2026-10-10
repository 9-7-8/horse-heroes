# Elemental combat

> **Revised by the owner, 2026-10-10.** There are no classes and no horse
> attunements. Elements belong to abilities; horses buy abilities with points
> from one shared pool, equip a few, and fire whichever suits the enemy.
> Primer, Finisher and Support are tags on abilities. Where this file speaks
> of a class's role or a horse's attunement, read "the ability's". The newer
> statement is in [vision.md](vision.md), "Abilities, levels and no classes".
> Also changed that day, in "Combat rules settled": an enemy holds one Primer
> of each element; a failed accuracy check glances and never misses; every
> hit can crit, reactions included; every payoff also deals damage.

Damage types and how they combine are the central combat and build interest
(confirmed; see [vision.md](vision.md)). The triangle is built. **Everything
else here is a proposal awaiting the owner's confirmation**, with prototype
numbers.

This file follows her later, more detailed document on elemental cooperation.
It replaces the first sketch from the design summary where they differ; the
differences are listed at the end.

> The horses should visibly cooperate without the player conducting every
> attack. One prepares, another pays it off, and the party feels stronger
> because of how its members fit together, not merely because it has three
> colours.

## Principles

### The answers first

- **How does one horse set up another?** A designated skill leaves a
  short-lived **Primer** on an enemy: an element, who left it, how strong.
- **What triggers a reaction?** Only a skill tagged as a **Finisher**. Not
  every hit of another element. Ordinary attacks, damage over time and
  cosmetic effects never consume a Primer.
- **What is consumed?** The whole Primer, with its charges. The Finisher's
  normal damage still happens, and it leaves no new Primer on that hit.
- **How are endless chains prevented?** A reaction can never apply a Primer,
  finish one, or trigger another on-hit effect. One Finisher cast, at most
  one reaction.
- **How do same-element parties work?** They build a Primer's charges and
  cash it out with a same-element Finisher as a **Crescendo**.

### What to borrow, and what not to claim

From action RPGs with rich skill systems, take the principles:

- every attack works even when no combination is available;
- timing and presentation make power legible;
- builds alter familiar skills, not an endless supply of new buttons;
- secondary triggers have deliberate eligibility and frequency limits, which
  account for how often a skill hits and how many it hits;
- big payoffs contrast with smaller repeated actions.

The reaction system itself is original to this game. Do not describe it as
anyone else's.

### Three things with three names

| Name | Meaning |
| --- | --- |
| Enemy element | The enemy's defensive matchup |
| Skill element | This hit's damage type |
| Primer element | A temporary setup for cooperation |

They are independent. A Fire enemy can carry a Plant Primer; an Ice Finisher
then deals Ice damage against Fire, and the reaction is Plant plus Ice.

The triangle stays: Fire beats Plant, Plant beats Ice, Ice beats Fire;
1.5 favoured, 0.75 disadvantaged, 1.0 neutral.

**A Primer is setup and nothing else.** It is not also a burn, chill or seed
meter with effects of its own. Element-flavoured behaviour belongs to a skill
or to a payoff. That keeps hidden state and balance small.

### Skill roles

Every horse has a neutral basic attack and one automatic signature skill,
tagged **Primer**, **Finisher** or **Support**.

A party to teach with: the Ranger has a fast, focused Primer; the Wizard a
slower Finisher; the Mage a ward or heal.

- **Tags are not locked to classes for ever.** Class decides how a skill is
  delivered; gear and attunement decide its elemental expression. A later
  mouth weapon or variant may let a Mage prime or a Ranger finish.
- **A party without a Finisher still fights.** It loses a specialised payoff,
  not the ability to function.

### Primers

Prototype values: lasts 5 seconds, up to 3 charges, a 1 second reaction
lockout on the target.

- On an unprimed enemy: one charge.
- Same element: another charge, up to the cap, and the timer refreshes.
- **A different element does not overwrite.** A third horse must never erase
  an ally's setup just before the Finisher lands.
- The skill's normal damage happens regardless.
- Contributors are recorded. Strength comes from the skill's intended setup
  budget, never from each damage tick.
- A Primer dies with its enemy, but **a lethal Finisher still pays off**: the
  reaction is resolved before the enemy is removed.

### Resolving a Finisher

1. The target was alive at impact and the hit is valid.
2. Read its Primer and lockout as they were before the hit.
3. Deal the normal damage.
4. If eligible, remove the Primer in one step and create one payoff.
5. Apply the lockout.
6. Resolve the payoff, then deaths and rewards, in a defined order.

Different element: a pair **Reaction**. Same element: a **Crescendo**. No
Primer, an expired one, or a lockout: just the skill.

Hits resolve in a deterministic order. Two Finishers never consume one
Primer. A skill's impact is one event, however many particles it shows. One
cast consumes one target's Primer; an area skill still does its ordinary area
damage.

### Three pair reactions

| Pair | Reaction | Role | Limit |
| --- | --- | --- | --- |
| Fire + Ice | Thermal Shock | Burst on one dangerous target | One target |
| Fire + Plant | Wildbloom | Eruption into nearby enemies | Small radius, a few secondary targets |
| Ice + Plant | Wintergarden | A barrier for the party | Refreshes or replaces; never stacks without bound |

Either order, to begin with. A damaging reaction uses the Finisher's element.
They need not be equal in raw damage: compare useful outcomes (removing a
dangerous target, clearing a group, surviving, reaching the boss).

### Three Crescendos

| Element | Crescendo | Payoff |
| --- | --- | --- |
| Fire | Flare | A bounded burn on the target |
| Ice | Deep Frost | A burst and a brief, capped slow |
| Plant | Bloom | A modest hit and recovery for the most injured horse |

- No permanent freezing. Bosses take a reduced slow.
- Healing has a cap and triggers nothing.
- More charges strengthen a payoff with diminishing returns (for example
  1.0, 1.35, 1.6). Never multiplied again by the number of horses.

**Mixed must not simply beat same.** A same-element party concentrates the
type advantage: strong on favourable routes, with an understandable weakness
to its counter. A mixed party gains coverage and distinct pair effects, and
sometimes loses consistency. Tune both against groups and against bosses.
**Never claim viability from formulas alone.**

### Scaling without confusion

- Two separate values: **attack power** (a skill's normal damage) and **setup
  power** (its contribution to a Primer).
- A payoff scales from the bounded setup contribution plus the Finisher's
  power, times the charge modifier.
- Apply the Finisher's element matchup **once**. Not both elements', and not
  the opening hit's again.
- No critical reactions to begin with.
- Barriers and healing have their own coefficients and caps, and an enemy's
  resistance never applies to them.
- **The report credits the horse that set up as well as the one that
  finished.**

### Anti-chain safety

Every effect has an origin: direct skill, basic attack, damage over time,
reaction, item effect, pet ability.

- Only designated direct Primer skills create Primers.
- Only designated direct Finisher skills consume them.
- Reactions, damage over time and generic item effects are ineligible by
  default.
- One reaction per cast. State is consumed before secondary effects are
  dispatched. Stacks, barriers, target counts and triggered events are all
  capped.
- Not in a first version: cooldown resets from reactions, reactions that
  spawn a Finisher, items granting extra full-strength hits.
- A pet may later apply a small Primer, through the same bounded pipeline.

**A chain may look like a chain without being one.** Wildbloom can ripple
visibly through a group while resolving as one limited event.

### Gear changes its own part

The elemental document was written for an earlier set of gear categories. In
the owner's slot mapping ([gear.md](gear.md)) the same jobs fall like this:

- **Mouth weapon:** setup strength, Finisher power, elemental expression, a
  tested skill variant.
- **Braid:** a named cooperation effect, such as a reaction reaching one
  more enemy within its cap.
- **Barding and boots:** resistance, mitigation, staying on its feet.
- **Halter:** accuracy, including against primed targets. A miss never
  consumes a Primer.
- **Horseshoes:** crit chance. A crit adds no Primer charges, and reaction
  damage does not crit to begin with.
- **Saddlebags:** cargo, nothing to do with reactions.
- **Pet saddle and collar:** the pet's own role, never a universal reaction
  multiplier.

Which target or ally to favour, and whether a Finisher waits briefly, are no
longer a gear slot's job; they are settings, with a competent default.
**No piece of gear is ever required for the party to cooperate.** A piece's
look never forces its rule; looks are separate
([cosmetics.md](cosmetics.md)).

### Automation

The default already understands the system:

- a Primer skill targets a living enemy with no strong Primer;
- a Finisher prefers an eligible primed target and **waits only a brief,
  bounded window**, then fires normally;
- nothing ever halts the party waiting on a dead or disabled Primer source;
- major skills resolve in short predictable cycles, so setup exists before
  the payoff looks for it, and animation timing matches real impact.

The party is previewed in plain words: "Plant setup, Fire Finisher:
Wildbloom", or "No Finisher: normal attacks work, but no reaction payoff".

An unwatched fight runs the same rules without rendering and reports counts
and outcomes: reactions triggered, useful healing and barriers, clear speed,
the main cause of a loss.

### Showing it

- **Setup:** a modest element-shaped marker on the target (ember, crystal,
  leaf), with charges as small segments, not three more particle systems.
- **Finisher:** a distinct wind-up and release; sound, enemy response and
  damage land together.
- **Reaction:** the marker is visibly consumed; the payoff plays; its name
  appears occasionally. Never a modal message or a giant label every time.
- **Aftermath:** enemies recoil or die, a barrier shows on protected horses,
  healing shows on whoever received it.
- Shape and icon as well as colour. A reaction never covers enemy danger, a
  horse's silhouette, or dropped gear. **The strongest presentation is kept
  for boss outcomes and exceptional loot, not for a setup.**

### Build order and tests

Build: neutral attacks and elemental damage; one Plant Primer and one Fire
Finisher; a visible, bounded Wildbloom; atomic consumption, expiry and
lockout; the three Crescendos; the other two reactions; gear variants and a
limited pet setup.

Test: two Finishers cannot consume one setup; reactions create no Primers and
trigger no reactions; an expired setup cannot be consumed; a lethal Finisher
gives one payoff and one reward; removing the setup horse does not deadlock
the party; same-element parties have routes and bosses they are good at;
mixed is not always best; a multi-hit skill does not add a charge per
particle; watched and unwatched fights agree.

### To confirm before making many skills or items

Tagged Primer and Finisher skills (not a reaction on every cross-element
hit); individual attunements; one Primer per enemy; symmetric pair
reactions; same-element Crescendos; no recursive secondary effects; no
permanent boss freeze; whether each class keeps one fixed skill role at first
or allows early variants.

## For Horse Heroes

- The slice has one element for the party and three abilities on cooldown. In
  these terms the Ranger's Focused Shot is the natural Primer, the Wizard's
  Elemental Burst the Finisher, and the Mage's Ward the Support, so the
  teaching party is the party that exists. Elemental Burst hits every enemy
  today; as a Finisher it would still do that damage and consume one Primer.
- "Reserve the strongest presentation for boss outcomes and exceptional
  loot" is pillar 1 stated from the combat side. A reaction sits below the
  fountain on the shared importance scale ([juice.md](juice.md)).
- The effect origins and caps are one mechanism for three systems: reactions,
  gear effects and pet abilities. Build it once
  ([procedural-generation.md](procedural-generation.md), bounded effects).
- The sim is deterministic with fixed ticks, which is what atomic consumption
  and a defined resolution order need. The "short predictable cycles" are a
  scheduling rule in the sim, not an animation trick.
- `first_trail_balance` assumes one party element and must be rewritten with
  attunements, not left passing by accident.
- The vision's guardrail holds: three elements, three reactions, three
  Crescendos, and no more damage types until the loop works.

## What changed from the first sketch

- **Statuses became Primers.** The summary proposed Burning, Chilled and
  Seeded as statuses with effects of their own. Here a Primer is setup only;
  burn, slow and growth live in the Crescendos.
- **Not every cross-element hit reacts.** Only a tagged Finisher does.
- **A different element no longer consumes on contact**, and does not
  overwrite an existing Primer.
- **Same-element play has a mechanism**, the Crescendo, where the summary had
  only "stronger Burn, capped slow, growth payoff".
- **Classes are a starting assignment of roles**, not their definition.

## Source

Owner's design document on elemental cooperation, received 2026-10-09 (all
sections), following the design discussion summary of the same date
(sections 7 and 8).
