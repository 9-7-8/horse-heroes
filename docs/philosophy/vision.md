# Vision

What Horse Heroes is, in the owner's design as it stood on 2026-10-09. This
file outranks the other philosophy files where they disagree: they are advice
drawn from studying other games; this is what she wants built.

Three kinds of statement are kept apart here, as her summary kept them apart.
**Confirmed** is decided. **Proposed** came out of discussion and awaits her
decision. **Open** is unanswered. Do not build on a proposal as if it were
confirmed; the open questions are filed as issues labelled `decision`.

## The game in one paragraph

The player organises an autonomous gaggle of horses travelling through a
solarpunk fairy world threatened by one greedy dragon. They choose the party,
equipment, pets, pet assignments, appearances, town trades, and which horses
stay in each town. Automatic combat centres on distinct elemental damage types
and their combinations. Victories produce abundant horse loot; town visits
turn that loot into build improvements, sales, customisation and new party
decisions. Restoration, recruitment, pets, residents and remembered events
make the journey personal.

## Confirmed

### World

- A solarpunk fairy world. One particular greedy, evil black dragon is
  spreading infestation, corruption and harmful industrialisation.
- **Dragons are not inherently evil.** This individual is the antagonist.
- Restoration is the theme of world progress: places are helped and restored.

### The loop

    horses walk -> fight monsters -> huge loot drops -> equip what is useful
      -> sell the rest in towns -> fight stronger monsters

Recruitment, pets, helping people, restoration and customisation give that
loop emotional meaning. **They are not replacements for the combat-and-loot
engine.**

### The player is the organiser

The player does not ride a horse or control combat. Horses walk and fight by
themselves. The player controls:

- who is in the party;
- what each horse wears;
- what is bought and sold in each town;
- which horses are left behind in each town;
- which pets travel, and which horse each pet is with;
- appearance and outfits.

No avatar and no explanation for the organiser is required.

### Horses, pets, towns

- Horses can stay in a town until the player returns, and generate resources
  while there.
- **The world respects horse bodies.** A horse cannot simply be given
  human-style farm work: it has no hands. Either the task suits a horse, or
  the world says how magic, tools, pets or other inhabitants make it possible.
- Pets are collectible companions assigned to individual horses. What they
  do in play is open; the recommendation she was given is in
  [pets.md](pets.md).
- Selling in towns has a small shop-simulator side. Relationships with a
  town's horses affect selling prices.

### Equipment

- **No bridles, bits or reins.** They imply riders, and there are none. Use
  halters.
- **Saddles are for pets**, not riders.
- **Saddlebags** let a horse carry more.
- **Each equipment category has its own identity**, as armour is defence,
  weapons offence and trinkets utility. No slot may become the same generic
  mix of attack, health and cooldown as every other. The categories should
  feel natural for autonomous horses.

### The caravan shop

Confirmed direction in her caravan shop document; the detail is in
[town.md](town.md).

- The caravan is the party's **travelling home and shop**.
- The player lists gear for sale; local horses and townspeople **buy it over
  time**.
- The player **sets prices as a percentage** above or below a baseline.
- **Lower prices improve relationships** with townsfolk.
- **Unsold stock comes with the party** when the caravan leaves.
- The shop is idle-friendly and simpler than a business simulator.

### Loot and cargo

Recorded as her confirmed decisions in her loot and saddlebags document; the
detail is in [cargo.md](cargo.md).

- Loot belongs to the **whole party**; no horse has a personal inventory.
- **A full bag does not stop the journey.** The party leaves a new item
  behind or abandons a carried one for a better one. Capacity never sends the
  party home or skips the boss.
- The player can tune what is kept. The default is highest rarity first.
- Rare gear is exciting: its own rarity colour, a more distinctive look, a
  more impressive reveal.

### Travel

Established in discussion and listed as such in her travel document; the
detail built on them is in [travel.md](travel.md).

- The world is procedurally generated and **finite**.
- The player chooses a **destination city**.
- Distance determines the number of travel days; horses cover a limited
  distance per day.
- **Every journey ends with a boss encounter.**
- More dangerous areas have more and stronger encounters and stronger bosses.
- Local danger depends on the **purity of nearby world tiles**.
- Towns and regions update when visited or when the world map is opened.

### What the long game is for

- The central combat and build interest is **damage types and how they
  combine**.
- Long-term rewards are companions, pets, attractive gear and appearance: **a
  fashion-focused endgame**. Huge loot drops and good-looking equipment are
  core goals.
- The game should produce **memorable stories** through persistent
  characters, procedural situations and remembered consequences, as an idle
  RPG and not a colony simulator.

## The design priority

> Complexity through depth, not breadth.

Supporting features reinforce the journey and the loot-and-build loop. None
may become an independent management game.

**The central test for any addition:**

> Does a new horse, pet assignment or item make the player want to change who
> travels together, and can they see that choice matter on the road?

**Put depth into:** party selection, elemental cooperation, differentiated
equipment, routes, town residency, commerce, appearance collection.

**Stay away from, until the main loop works:** breeding, detailed survival
needs, extensive crafting, town construction, and many more damage types.

## Proposed, awaiting her decision

- **Road and town as two phases.** Road: automatic action and the occasional
  discovery. Town: concentrated decisions about builds, trade, companions and
  looks. Developed in [town.md](town.md); the caravan itself is now
  confirmed, above.
- **A screen layout:** the top third a fixed-view 3D scene, the lower
  two-thirds menus. Four of her documents treat this as established; the document
  that decided it has not been received, so it is listed here until she
  confirms it.
- **Equipment as five categories:** focus (offence), protection (defence),
  halter (coordination and utility), pet saddle (companion), saddlebags
  (carrying). She confirmed halters, pet saddles and saddlebags; the
  focus/protection/halter split is a recommendation. Where a piece is drawn
  need not be its category: a focus may sit on the halter. Developed in
  [gear.md](gear.md).
- **Utility gear specialises behaviour; it never repairs it.** The baseline
  automation is already competent.
- **Elements, classes and reactions:** see
  [elemental-combat.md](elemental-combat.md).
- **An appearance library:** a piece's look is kept for good once collected,
  so the piece can be sold or replaced without losing the look, and a loved
  outfit stays viable. Developed in [cosmetics.md](cosmetics.md).
- **Loot handling:** collect routine rewards automatically, aggregate the
  ordinary, spotlight upgrades, rare looks and build-changing pieces, then
  keep, equip or sell. Favourites and gear in saved loadouts are protected
  from automatic disposal.
- **What a major victory does:** corruption recedes, a town opens or
  recovers, routes become safer, residents offer aid, and horses, pets,
  styles and equipment become available. Restored places offer **patrols**
  against remaining incursions, so farming does not need a place to be
  corrupted again.
- **Harmful extraction and domination against restorative, communal
  technology** as the line the world draws, in keeping with solarpunk.
- **Stories start small:** persistent characters, contextual events and
  remembered consequences, with a journey journal first. Bonds, rivalries and
  personality systems only after that.
- **Narration reports what happened.** It never invents an accomplishment the
  simulation did not produce.
- **A horse's identity:** a name, an appearance, a class, one modest
  signature trait, and a record of notable events. Class is how it acts,
  element is what its magic does, personality is who it is: three separate
  questions.

## Her proposal: retirement and generations

A horse who leaves the party retires into a town and stays a named character
there. Healthy, restored towns support families; their children may later be
recruited, so the party is one generation in a longer effort. Towns advance
only when visited or when the world map is opened. The campaign has a finite
goal, cleansing every region, and endless play is optional. The idea is hers;
the details are recommendations. All of it is in
[generations.md](generations.md).

This touches the guardrail above against breeding. It is not a genetics or
pairing system, and its own advice is to prototype it small and only after the
main loop works. Both hold.

## Open

Party size and duplicates; whether class or element can change; what makes a
recruit valuable without replacing a favourite; the shape of the map and how
routes are picked; how the
shop works and how relationships are earned; what work a horse can do in
town; the exact reaction rules; what pets do; what failure costs; whether
restoration is permanent; whether corrupted creatures are killed or cured;
what ends the journey and what follows it. Each group is a `decision` issue.

## Against the game as built

- The slice is one fixed party of three on a repeating trail with a camp. The
  loop's first half exists; "sell the rest in towns" does not, and neither do
  towns, recruits, pets, residents or stories.
- `assets/data/gear.ron` has a Bridle and a Browband in the Head slot. That
  contradicts a confirmed rule and is filed to be fixed.
- The three slots (Head, Body, Saddle) are places on the horse, not
  identities: every piece is the same stat points. That is exactly the
  sameness the equipment rule forbids.
- One element is chosen for the whole party. Her emphasis on combinations
  needs more than one element in play at once.
- The pillars in `CLAUDE.md` still hold and agree with this file: the loot
  shower first, idle first, simple 3D, numbers as data. "Depth, not breadth"
  is the same judgment as pillar 1's "a new system waits behind one that
  makes a drop feel better".
- The guardrail against extensive crafting bears on the salvage and material
  issues filed earlier: selling in towns is confirmed, a crafting economy is
  not.

## Source

Owner's design discussion summary, received 2026-10-09 (all sections); the
retirement section is from her later proposal, same date. The summary's
header says it follows a document about a Windows installer and updater, which
has not been received.
