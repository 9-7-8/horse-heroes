# Gear

What each piece of horse equipment is for, and the rules that keep the slots
from blurring into one another. **The slot mapping is the owner's choice.**
The formulas, example items and modifier rules built on it are proposals.

This file was first written around a proposal of five categories (focus,
protection, a halter for coordination, pet saddle, saddlebags). Her later
mapping replaces that; what changed is listed at the end.

What makes an item worth finding is in
[items-and-builds.md](items-and-builds.md); how a piece looks is in
[cosmetics.md](cosmetics.md); how one is generated is in
[fairyland-content.md](fairyland-content.md); how it is carried is in
[cargo.md](cargo.md).

> Every item should improve a recognisable contribution without becoming
> another version of every other item. Restricted pools create identity;
> supporting overlap creates choice; exceptional powers create experiments.

## The owner's slot mapping

Each slot is the horse-world translation of a familiar RPG slot.

| Familiar slot | Horse slot | What it is for |
| --- | --- | --- |
| Helmet | **Halter** | Accuracy |
| Breastplate | **Barding** | Defence and damage reduction |
| Trousers | **Boots** | Defence |
| Belt | **Breastcollar** | Varied utility |
| Gloves | **Horseshoes** | Critical-hit chance |
| Weapon hand | **Mouth** | A mouth-held weapon |
| Trinkets | **Mane or tail braid** | Charms and build interactions |

Also kept: **saddlebags** for the party's shared carrying capacity, and
**saddles** for pets. No bridles, bits, reins, stirrups or riders.

## Principles

### Three layers per slot

1. **An intrinsic property:** why the base item exists.
2. **Supporting modifiers:** a small pool related to that property.
3. **Exceptional powers:** authored and bounded, never unrestricted random
   effects.

**A slot's identity must survive at high rarity.** If every rare piece says
"plus all damage", the slot's purpose has gone.

### What each slot owns

| Slot | The question it answers | Intrinsic | Supporting pool | Never an ordinary roll |
| --- | --- | --- | --- | --- |
| Halter | Will this horse deliver its ability reliably? | Accuracy | Conditional accuracy (against evasive or primed targets), narrow reliability | Weapon damage, crit, bulk armour, cargo |
| Barding | How much punishment can this horse take? | A protection rating | More protection on the piece, health, elemental resistance, bounded mitigation of heavy hits | Attack speed, crit, cargo, unrestricted reaction damage |
| Boots | Can this horse keep acting under pressure? | A smaller defensive rating | Defence, shorter slows and staggers, protection against repeated small hits | Crit, raw damage, cargo |
| Horseshoes | How often does this horse land an exceptional hit? | Critical-hit chance | Crit and narrow conditions that support it; later, limited crit damage | Accuracy, bulk armour, cargo, travel speed |
| Breastcollar | What everyday part of the build runs more smoothly? | One modest property from a named utility family | Resource economy, recovery timing, a situational support benefit | Copies of accuracy, weapon damage, armour, crit or capacity |
| Mouth | What does this horse's action do? | Weapon or ability power, and a weapon family | Local output, elemental expression, a compatible delivery modifier | Bulk armour, cargo, pets, anything about trade |
| Braid | What distinctive interaction does this build emphasise? | A modest charm property | A named cooperation effect, a class or status interaction, conditional ally support | Large unconditional copies of the weapon's power or another slot's stat |
| Saddlebags | How much can the gaggle carry home? | Shared cargo slots | Capacity; later, carrying rules that create a choice | Any combat bonus |

**Barding reduces damage; boots preserve stability.** Some overlap between
them is intended, but they must not be two interchangeable armour numbers.

**The breastcollar improves general operation; the braid changes a named
synergy.** Keep those pools apart unless an exceptional piece deliberately
bridges them.

### Controlled overlap

Some overlap is useful. What matters is who owns the strongest version:

    the weapon owns output            the halter owns accuracy
    the shoes own crit chance         barding owns substantial mitigation
    boots own defensive stability     the breastcollar owns bounded utility
    the braid owns named synergy      saddlebags own carrying

A small health roll on boots does not erase barding. A large generic attack
multiplier on boots probably does. **Balance the ranges as well as the
eligibility.**

### What to take from other loot games, and what not to

Take: recognisable item bases; ordinary modifiers restricted by slot;
selective overlap; authored rare powers that extend a slot's theme; and,
internally, the difference between improving a piece's own property and
improving that property everywhere.

Leave: their full modifier complexity. This game needs clear consequences in
automatic combat and little inventory management.

The slot meanings above are this game's own. Do not describe accuracy on a
halter, or anything else here, as copied from another game.

### Accuracy (proposed rules)

- Direct offensive hits can miss. **Healing and wards cannot.**
- A miss does not consume a Primer.
- Damage over time does not reroll accuracy each tick.
- A reaction that has triggered does not roll a second miss.
- **Baseline hit chance is high** against an equal enemy. Gear improves
  reliability; it never repairs a horse that keeps failing to act.
- If spells use accuracy too, say so plainly.

### Critical hits (proposed rules)

- One roll, on a successful direct hit. A fixed, modest multiplier.
- A crit adds no extra Primer charges.
- Reaction damage does not crit to begin with.
- An effect triggered by a crit can never trigger itself.
- The shoes are enchanted: they apply to eligible attacks and spells, not
  only to kicks.

### Mitigation

Do not roll percentage damage reduction freely on every piece. Use a formula
from protection to mitigation with a cap, or a few explicit conditional
effects, so additive stacking can never reach near-invulnerability.

### The awkward slots

- **Halter.** In the fiction, an enchanted aid to attention or a chosen
  signalling ornament, never rider control. No bit, no reins.
- **Boots and horseshoes.** Boots are lower-leg coverings; shoes are the
  hoof's contact with the ground. **One item equips the whole set** of four;
  no matching chore. World travel speed is a separate matter and is on
  neither to begin with.
- **Breastcollar.** "Varied benefits" means a few tested utility families, not
  every statistic in one pool. **Never generate a bonus to a resource that
  does not exist yet.** The owner renamed this slot from "martingale" on
  2026-10-10: a real martingale attaches to reins, and a breastcollar sits on
  the chest and implies no reins or rider. Its families are timing and
  recovery: ability timing, potion effect, rest on quiet days.
- **Mouth.** The weapon replaces the earlier focus. Rods and crystal focuses
  are weapon families beside compact mouth-grip weapons; **no horse is
  required to swing a human-sized sword.** The grip must be safe and
  readable, and the strike or spell must come from a point that makes sense.
  Class decides delivery, element decides magical behaviour, and neither
  needs its own duplicate damage stat.
- **Braid.** Start with one braid slot shown on mane **or** tail. Two slots
  is an open choice, not implied by two places to hang a charm. A horse with
  a short mane or another hairstyle gets a fallback socket, so a cosmetic
  choice never locks it out of the slot.
- **Material is not power.** Blankets, woven coverings and plate are
  alternative bases and looks for barding. Metal never invalidates a
  favourite blanket.

### Pets

The pet saddle is seating and appearance to begin with, later perhaps a way
of directing the pet. It is never an extra horse-stat slot. A collar tunes
the pet's signature ability. Pet armour only if pets can be in danger. One or
two pet slots at most.

### A small item

- **Common:** the base property and a modest improvement.
- **Uncommon:** the base and one supporting modifier.
- **Rare:** the base and two supporting modifiers, or one distinctive effect.

Keep descriptions short and comparisons legible. **Always show the resulting
value or explain the breakdown** ("Accuracy 12, +4, +3 against primed
targets").

Rarity does not make a piece best for every horse, and a simple look stays
usable through the wardrobe.

### Exceptional pieces extend their slot

- Halter: a miss briefly improves the next setup attempt.
- Barding: a heavy hit raises a capped emergency shield.
- Boots: resisting an impairment grants a short recovery.
- Shoes: a crit empowers one following direct attack.
- Breastcollar: changes one resource or recovery loop.
- Weapon: changes one skill's delivery.
- Braid: specialises one cooperation pattern.

**Author these deliberately.** Rolling arbitrary triggers and effects
together and calling the result a feature is not design.

### Generating a piece

    slot -> base family -> tier -> rarity -> valid modifier pool
      -> authored special power if eligible -> visual recipe -> name -> validation

A modifier's definition carries its allowed slots, supporting tags, power
cost, exclusions, stacking cap, and trigger eligibility and cooldown.

Hard validation: no bonus to a resource or status that does not exist; no
interaction with an unbuilt pet system; no unbounded trigger chain; no
equivalent effects under two names; no cargo modifier that multiplies
damage; no decoration that needs a horn an ordinary horse lacks.

### Comparing and carrying

- **Compare by purpose, not by one universal power score.** Show accuracy,
  protection, stability, crit chance, utility, output, synergy or capacity as
  fits the slot.
- Equipped gear is separate from shared cargo. Favourites and loadout spares
  are protected. The default keep rule is rarity first, and no hidden upgrade
  score overrides it.
- Gameplay loadouts and wardrobe presets are separate. A halter's look is
  not its accuracy. **A mouth weapon's chosen look must stay compatible with
  its animation family.**

### Order of work

Two options, for the owner to choose between:

- **Stage the mechanics.** First: mouth weapon, halter, barding, horseshoes,
  saddlebags. Then boots once impairments matter, the breastcollar once a
  utility loop exists, the braid once reactions work, pet gear once pets have
  abilities.
- **Show every slot from the start.** Give each one simple property and
  defer conditional effects. Label any piece that is cosmetic only.

Either way: **do not invent a subsystem merely to fill a slot.**

### Still holds from the first version of this file

- A slot exists because it creates a distinct decision, not because a model
  has somewhere to hang another object.
- Keep universal stats off the table: attack speed, cooldown reduction and
  generic damage spread across every slot collapse every choice into "pick
  the biggest number".
- Every piece has a home in one decision: equip it, keep it for another
  build, sell it.
- Expand through new effects and combinations before adding slots.
- Not early: weight and volume together, random modifiers on cosmetics,
  several tiers above Rare, more than one equipment currency, durability, a
  separate pet inventory, gear whose look cannot be overridden, large
  mandatory set bonuses.
- The player never manages gear more than they enjoy finding it.

## For Horse Heroes

- The slice has three slots (Head, Body, Saddle) and stat points. The mapping
  gives seven horse slots, plus saddlebags and the pet saddle. That is a
  large step, and the staged order above exists to take it in pieces.
- **Three slots need mechanics the game does not have.** Nothing can miss, so
  accuracy means nothing yet. There are no critical hits. There are no slows
  or staggers for boots to resist, and no resources for a breastcollar to
  ease. Each is a real subsystem, to be built because the owner chose the
  slot and not as filler; and the warning above applies to the two that have
  nothing obvious to attach to yet.
- Accuracy is a new way for an automatic fight to feel bad. A miss the player
  did not cause and cannot see reads as the game cheating. If it is built, a
  miss must be shown as clearly as a hit
  ([combat-rhythm.md](combat-rhythm.md)), and the baseline must be high.
- The halter no longer carries targeting. In the earlier proposal it
  specialised how a horse cooperates; now that job has no slot. Targeting
  preferences stay as settings in town, and named cooperation effects move to
  the braid.
- The mouth weapon is where the elemental system meets gear
  ([elemental-combat.md](elemental-combat.md)): which setup a horse applies,
  how hard its Finisher lands. It raises one question that must be settled:
  whether a weapon sets the horse's element or modifies an attunement the
  horse already has. There cannot be two sources that disagree.
- Automatic equipping judged by stat total no longer means anything. Each
  slot needs its own notion of "better", shown in that slot's own terms, and
  the idle default has to be right slot by slot.
- "One or two stats, at most one effect" still limits the text. A piece has
  to be readable on a small card during the fountain's aftermath.
- Nine categories of gear multiply drops and looks. That is good for the
  fountain and for the wardrobe, and it is exactly the volume the cargo rules
  were made to absorb.

## What changed from the first version

- **Focus is gone.** Its job is the mouth weapon's.
- **Protection is split** into barding (mitigation) and boots (stability).
- **The halter is accuracy**, not coordination and targeting.
- **New slots:** horseshoes for crit, the breastcollar for utility, the braid
  for build-defining charms.
- The earlier advice against many slots is overridden by the owner's mapping;
  the reasoning behind it survives as the rule that each slot keeps its own
  question.

## Source

A proposal for a starter gear system, received from the owner 2026-10-09, and
her later design document on equipment slot identities and modifier rules,
same date, which states her slot mapping and replaces the earlier categories.
