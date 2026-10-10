# Gear

The starter equipment system and how to grow it. The confirmed constraints are
in [vision.md](vision.md): no bridles, bits or reins; halters; saddles for
pets; saddlebags for carrying; every category with its own identity. The
categories beyond halter, saddle and saddlebags are **proposals to be tested
before they are fixed**.

What makes an item worth finding is in
[items-and-builds.md](items-and-builds.md); how a piece looks is in
[cosmetics.md](cosmetics.md); how one is generated is in
[fairyland-content.md](fairyland-content.md).

## Principles

### Each category owns a different question

> A slot should exist because it creates a distinct player decision, not
> merely because a model has somewhere to attach another object.

| Category | The question it owns | May carry | Not its job |
| --- | --- | --- | --- |
| Focus | What should this horse contribute? | Ability strength, elemental behaviour, taking part in reactions, how a class ability is delivered | Armour, cargo, pets |
| Protection | What happens when danger reaches this horse? | Mitigation, resistance, barriers, recovery, shortening harmful effects | Elemental application, loot quantity, attack multipliers |
| Halter | How does this horse work with its companions? | Target preference, ally support, sequencing, noticing setups, utility on the road | Flat attack or defence |
| Saddlebags | How much can the gaggle carry home? | Capacity and a few chosen carrying rules | Combat multipliers |
| Pet saddle | Where does the pet ride? | Seating; later, how the pet's help is directed | A stat slot for its own sake |
| Pet collar | How does this pet help? | Tuning the pet's own job | Horse statistics |

- **More slots do not make more depth.** If every slot rolls attack, health,
  resistance and cooldown, the player is sorting duplicates.
- **Keep universal stats off the table.** Attack speed, cooldown reduction
  and generic damage spread across every category collapse every choice into
  "pick the biggest number".
- **Crossover is for the rare and distinctive piece**, intentional and
  limited.
- **Form is free; function is not.** A blanket and plated barding share the
  protection slot and neither is worthless because the other exists.
- **A halter specialises competent behaviour.** A horse knows how to fight
  without a rare item.
- **Do not add a saddle stat because the model has a saddle.** Mechanics come
  when the pet's role is clear.

### What a first item answers

- Does this horse deal damage, protect itself, help the party, carry more, or
  support a pet?
- Why is this piece better for this horse or build?
- What decision does it create?
- Can its purpose be seen without comparing many small numbers?
- Can its look be enjoyed after its stats are obsolete?

### A small item

Name, slot, rarity, tier, **one or two stats, at most one distinctive
effect**, an appearance, a sale value, and optional tags (an element, pet, a
town of origin). Appearance and function reinforce each other and stay
separate.

Three rarities to begin with:

- **Common:** a basic stat package; an early upgrade, or something to sell.
- **Uncommon:** a better package or a small conditional bonus.
- **Rare:** a stronger package and one distinctive effect.

Every piece should have a clear home in one decision: equip it, keep it for
another build, sell it, or (if it exists) salvage it.

### Saddlebags

- A single capacity number first. Not weight and volume and bulk and
  compartments.
- Capacity is shared by the whole party (confirmed).
- **The rule for a full bag is decided:** the party carries on, keeping the
  best cargo its rules allow and leaving the rest. A full bag never ends the
  trip. This replaces the automatic return to town first suggested here. The
  whole of it is in [cargo.md](cargo.md).
- Their value is more loot per trip, never more damage per hit.

### The town

    travel and fight -> fill the saddlebags -> reach town -> compare
      -> sell what is not wanted -> use a service or buy -> choose the next journey

- Selling belongs to the town visit. It is never an inventory chore that
  interrupts a fight.
- New items shown apart; possible upgrades highlighted per horse; favourites
  can be marked; ordinary gear sold in a batch; equipped and favourite pieces
  protected; **the player keeps control of anything rare or unusual**.
- Relationships may later move prices. **Not every trip is a pricing
  mini-game.**

### Growing it

> Expand through new effects and combinations before adding slots.

1. The basic categories, three rarities, a small authored pool, buy, sell,
   equip and compare. *Test whether the player understands why each slot
   exists.*
2. One or two tested build-changing effects per category, readable in reports
   and descriptions.
3. Pet collars, once pets have recognisable abilities.
4. Generated variety inside authored rules.
5. Small collection bonuses, only after single-piece choices work. Never a
   mandatory full set.
6. A few understandable services: salvage, improving one stat within bounds,
   targeting a slot or family, recolouring. Crafting only where it creates a
   choice.
7. Later: loadouts, presets, regional motifs, looks tied to towns and
   generations.

### Not early

Ten or more slots per horse; a slot for every strap and ornament; weight and
volume together; random affixes on cosmetics; several tiers above Rare; more
than one equipment currency; durability and repair; a pet inventory managed
apart from the horses'; gear whose look cannot be overridden; large mandatory
set bonuses; generated combinations that can be invalid or unreadable.

### The ten

1. Every gear category has a job.
2. Gear is natural to a world without riders.
3. Focus for offence, protection for defence, halter for coordination,
   saddlebags for carrying.
4. Pets get a small number of useful, understandable choices.
5. Appearance is independent of power.
6. Ordinary loot is easy to sell, and unusual rewards are never hidden.
7. Expand through effects and combinations before slots.
8. Gear changes decisions, not only numbers.
9. The player never manages gear more than they enjoy finding it.
10. The first version is small enough to test on one journey and one town.

## For Horse Heroes

- The slice's Head, Body and Saddle are attachment places. By the table above
  they own no question, and every piece is the same stat points. Changing the
  slots changes what gear **is**, so it waits on her decision about the
  categories.
- Three rarities already exist and the advice is to stay there for now. That
  holds back the earlier idea of tiers above Rare: give Rare its job first
  (one distinctive effect), and add a tier only when there is a job left
  over.
- The slice equips an upgrade automatically, judged by stat total. With
  categories that own different questions there is no single total, so
  "upgrade" has to be redefined per category or the automatic choice becomes
  wrong. Idle-first still requires a default that works with no input.
- "One or two stats, at most one effect" is also a limit on text. An item
  that fits on a small card can be read during the fountain's aftermath; one
  that needs a comparison screen cannot.
- The focus is where elemental cooperation meets gear
  ([elemental-combat.md](elemental-combat.md)): which setup a horse applies,
  how it takes part in a reaction. That makes it the category most likely to
  change who travels together, which is the vision's central test.
- Saddlebags are the one place abundance and low management meet. The owner's
  full-bag rule (keep the best, carry on) is what lets a huge fountain be a
  gift and not a nuisance ([cargo.md](cargo.md)).

## Source

A proposal for a starter gear system and its expansion, received from the
owner 2026-10-09 (all sections).
