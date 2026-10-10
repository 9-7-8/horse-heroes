# Pets

What pets are and how they fit beside the horses. A few points are confirmed
and listed first. The rest is recommendation, from two documents the owner
handed over; where the second refines the first, this file follows the
second.

> One pet, one clear signature ability, one specialising collar, and
> optional defensive armour. The pet helps the horses do something
> interesting without becoming another full adventurer to equip and manage.

## Confirmed by the owner

1. Pets are collectible companions, **chosen by the player and assigned to
   individual horses**.
2. **Pets cannot die.**
3. Pets **may ride in saddles**.
4. Pet equipment **stays small: a collar, and optionally armour**.
5. **When a horse retires, its pets return to the party** along with its
   gear ([generations.md](generations.md)).

## Principles

### A pet enriches decisions the player already makes

    travel -> meet or adopt a pet -> assign it -> watch its signature help
      -> find a collar variant -> adjust the party -> travel again

**No separate full build screen.** A pet adds to party composition and gear
choices. It does not open a second game beside them. The horses already
carry the build complexity.

### One signature ability each

Recommended: a pet has **one automatic signature ability**, plus cosmetic
and personality behaviour. Not also a passive and a set of attacks. One
ability can be enough to open a new option for the party, and **its value
should be identifiable from its behaviour, not buried in small bonuses.**

An ability is built from four parts: a trigger, a target rule, an effect and
a cooldown.

Examples to choose from, not a roster:

| Pet | Ability | Why the party wants it |
| --- | --- | --- |
| Moth | Now and then reinforces an eligible elemental setup | Keeping an opening alive |
| Rabbit | A small recovery pulse between encounters | Lasting a long trip |
| Fox | Marks a dangerous enemy for its horse's next direct attack | Coordination |
| Turtle | Gives its horse a small barrier after a heavy hit | Survival |

Start with two or three.

### What a pet stores

A stable ID; a name; species and appearance; its signature ability; a collar;
optional armour; which horse it is with, or none; a temporary recovery
state; a few remembered events.

### Three levels of identity

- The horse's **class** defines how it fights.
- The horse's **element** defines its elemental contribution.
- The **pet** defines one more kind of support.

A pet is never a fourth class or a second damage system.

### Supporting reactions without replacing horses

Horses own damage and Finishers ([elemental-combat.md](elemental-combat.md)).
The safest things for a pet to do:

- refresh a Primer once, within a bounded duration;
- add one small charge, up to the normal cap;
- mark a valid target for better coordination;
- protect the horse that is setting up from interruption.

Later, perhaps: apply a weak Primer when its horse has no setup, on a long
cooldown. It opens a build without replacing a horse whose class is about
frequent setup.

Rules:

- **No pet Finisher**, to begin with.
- No reaction from a pet's ordinary attack.
- No pet effect triggered by the reaction it helped enable.
- One activation is one event, never one per particle.
- No unlimited refresh and no stacking barriers.
- Different pets should serve same-element and mixed parties. **No one
  elemental pet is mandatory for every strong party.**

### The collar

The collar specialises the pet's existing ability:

- its own horse only, or a nearby eligible ally;
- a modest change of cooldown;
- a longer setup;
- a bounded variant of the trigger.

So a moth that refreshes a Primer on its horse's target might, with one
collar, refresh at a better moment, and with another prefer a target an ally
can finish.

- **No collars that roll horse damage, horse armour, loot rarity or
  capacity.** That makes every collar the same comparison.
- **A collar never turns a pet into something else.** A healing rabbit does
  not become a damage turret.
- **A pet works without one.** There is a free basic collar, and adopting a
  pet gives its ability at once; it never needs a rare drop to switch on.
- A collar is a charm, bell, wreath or tag: nothing that implies a handler.

### Armour, and gentle risk

Recommended first model:

- Pets are **not** targeted by ordinary attacks. Certain clearly identified
  area threats can **exhaust** them.
- A tired pet retreats onto or beside its horse and recovers by itself.
- While exhausted, its signature ability pauses. Following, appearance and
  any shared conveniences continue.
- **No death, injury, permanent loss of stats, or removal.**
- Armour owns resilience: resisting these interruptions, recovering sooner,
  a limited protective response to one kind of threat.
- **Armour improves consistency. It never repairs a pet that is useless
  without it**, and it must not be compulsory.
- No manual revival, no pet potions, no feeding.
- **Pet exhaustion alone can never defeat the party.** That is decided by the
  horses ([failure.md](failure.md)).

The simpler alternative, which may be the right place to start: pets cannot
be exhausted yet, armour is cosmetic only and **labelled as such**. Never
claim a defensive function before the risk exists. Add the second slot only
after testing that pets are enjoyable with and without it.

### Assignment is meaningful and forgiving

- Which horse a pet goes with is a small team decision.
- **No permanent bonds and no penalty for rearranging.** Free reassignment
  in town; the current assignment always visible.
- Explain effects in plain language: "this pet helps the horse it goes
  with", never a coefficient.
- **Retirement clears the assignment.** The pet and its gear go back to the
  party's roster; the history between horse and pet is kept. A recalled
  horse is given a pet again.

### Favourites stay viable

- Species gives identity; generated appearance gives variation; gear gives
  small specialisations.
- **Never roll one litter as permanently much stronger than another.**
- Several useful roles, not one best companion.
- Pet progression, if any, is bounded and easy to catch up, so adopting a
  new favourite is never a punishment.
- Pet looks and armour looks are separate from mechanics
  ([cosmetics.md](cosmetics.md)). Fashion never forces a look for the
  optimal build.

### Pets are not a loot system

- **Basic loot collection is a shared convenience of the game**, not a
  reward for owning the right pet.
- **A pet never salvages or disposes of gear.** The party's keep rules
  decide what is carried ([cargo.md](cargo.md)), and no pet may quietly undo
  them.

### Adoption

Pets are **rescued, befriended or adopted**. Never captured by defeating
them. An encountered creature can bring a distinctive ability and a
recognisable identity. No randomly generated modifiers on a pet without a
tested list of what is allowed.

### Cuteness is part of the presentation, and is not a stat

The pet is visibly in the world and reacts to it: it follows or rides with
its horse; reacts to a victory, a rare drop, its horse being hurt; shows its
ability clearly enough to be noticed (fluttering toward a setup, a shield
pulse); appears beside its horse in town and in the party view; has a name,
a few animations of its own, optional accessories.

The ability is named on inspection and in summaries, without filling the
screen with text. **A decorative gesture is never counted as a gameplay
event.** A report may list Primers preserved, useful healing and barriers,
signature activations and time spent exhausted.

**A player must be able to keep a favourite pet because they love it**, even
when another is more efficient.

### What to take from other games, and what to leave

Take: distinct companion abilities; equipment that specialises a
contribution; temporary, recoverable downtime in place of permanent loss;
shared conveniences in place of one compulsory species; collectible
followers with visible behaviour; the idea that a creature met on the road
has an identity of its own.

Leave: large modification grids, separate pet weapon builds, upkeep, layered
survivability, automatic salvage, defeating an animal in order to adopt it,
and any presentation of a pet dying.

### First version, and what comes after

Start: two or three pets; one signature ability each; one collar slot; one
pet per active horse; cosmetic armour; clear reassignment.

Then: one telegraphed area hazard causing short, recoverable exhaustion, and
only then functional armour.

**Not in it:** pet death, breeding of pets, complex commands, pet skill
trees.

## For Horse Heroes

- Most of this is still recommendation toward an open decision. Until she
  decides, a pet is a companion with no combat effect, which is how the
  first pet issue is filed.
- The reactions (to a rare drop, to a victory) are loot feel. A pet that
  visibly delights in a top-tier drop is part of the fountain's aftermath,
  and is worth building before any pet ability.
- Pet abilities go through the same bounded pipeline as reactions and gear
  effects, with "pet ability" as an origin of its own. The rules above are
  that pipeline's settings for pets, not a second system.
- Setup support needs Primers to exist first, so the moth is not one of the
  first pets unless elemental cooperation is already built.
- "Not targeted by ordinary attacks" matches the sim as it is: there are no
  positions and nothing aims at a pet. A telegraphed area hazard that can
  exhaust pets belongs with boss telegraphs
  ([travel.md](travel.md)), which is also where it would be readable.
- "Keep a favourite even if another is more efficient" needs abilities that
  differ in kind. If pets can be ranked, favourites lose. The same reasoning
  as signature traits for horses.
- Retirement returning gear to the party means a retiring horse can push
  shared cargo over its limit. That is settled in town before leaving, like
  any other change of capacity.
- Idle-first: a pet never needs commanding.

## Source

A recommendation on pet design, and a later design document comparing pet
mechanics in three other games and setting out a proposal for this one, both
received from the owner 2026-10-09. The second also corrects how the
retirement rule had been read. Only principles are taken from the games it
studied.
