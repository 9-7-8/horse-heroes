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
- **Horses have telekinetic magic**, and resident horses use it to help build
  their towns.
- **When a horse retires, all its gear and pets return to the party.** It can
  be invited back, and is then given gear and a pet again.
- **Pets cannot die.** The player chooses pets and assigns them to horses;
  they may ride in saddles; pet equipment stays small, a collar and
  optionally armour.
- **Breeding exists as background flavour.** It has genetics, superficial and
  for colour only for now, borrowed from the horse breeding mod she has
  already made. It is not an important mechanic and everything it affects is
  aesthetic.
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
- **Her slot mapping** (her latest, replacing the five categories proposed
  earlier): halter for accuracy; barding for defence and damage reduction;
  boots for defence; breastcollar for varied utility; horseshoes for
  critical-hit chance; a mouth-held weapon; a mane or tail braid for charms
  and build interactions. Saddlebags and pet saddles stay. No stirrups
  either. The rules built on it are in [gear.md](gear.md).

### Failure

Confirmed in her document on failure and stakes; the detail is in
[failure.md](failure.md).

- **Horses and pets cannot die.**
- When health and usable potions run out, the party returns to the previous
  city, loses a configurable amount of loot, and pays a modest cost in money
  or materials.
- It then **waits for the player to send it out, and never retries the
  failed route by itself.**

### The ending

Her endpoint, set out in the endgame note; the detail is in
[endgame.md](endgame.md).

- **The world is actually saved, not farmed for ever.** The corruption and
  the dragon are finite and stay defeated.
- A region is cleared when its corruption source is removed, its town
  restored and its routes secured. A cleared region has no corrupted
  creatures and no boss, and none are brought back for the sake of loot.
- *(Revised by her, 2026-10-10.)* A cleared road is still a road. Horses are
  prey animals, and bandits and predators are a fact of travel, before and
  after the dragon. See "The roads and the trade", below. This replaces the
  earlier lines that success makes combat unnecessary and that there is no
  combat after the dragon.
- The campaign is finite by design. After the dragon the save stays open,
  and trading between towns along roads that are never wholly safe is what
  the party does there.

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

## Decided on 2026-10-10

Her answers to the `decision` issues, each recorded in full on the issue
named. Where another philosophy file still words one of these as a
recommendation, this section is the newer statement.

### Combat (#80)

- Tagged Primer and Finisher skills. One Primer per enemy, which a different
  element does not overwrite. Pair reactions: Thermal Shock, Wildbloom,
  Wintergarden. Same-element Crescendos: Flare, Deep Frost, Bloom. No
  recursive secondary effects and no permanent boss freeze.
- **Attunement belongs to the horse.** A weapon modifies it, never overrides.
- Each class keeps one fixed role at first: Ranger primes, Wizard finishes,
  Mage supports.
- One accuracy system for attacks and spells. Reactions never miss or crit; a
  miss does not consume a Primer.
- Impairments are a short fixed set: Slowed, Staggered, Rooted.

### The party (#86)

- Starts at three and **grows to five**, a place at a time, by restoration.
- Duplicate classes are allowed. Class and element never change.
- A horse's identity: name, appearance, class, one modest signature trait, a
  record of notable events. Personality is expressive only; a horse always
  goes where it is sent.

### The road (#82, #153, #88)

- A hex tile map holds purity; cities are joined by routes across it. The
  player picks from a few route options; a custom path exists, and the game
  is never balanced around it.
- The route is fixed on departure. Gear and pets change in town only. The
  party does not stop on the way; discoveries go in the journal.
- Events come from seeded daily checks. Health carries between fights; quiet
  days restore part, potions the rest. Danger is purity alone. Nothing about
  a route is hidden but its particular events.
- Failure: only gear picked up on that trip can be lost (a tenth, lowest
  rarity first, at most three); a small capped coin cost; nothing else. A
  retreat by choice is free. **The stall opens on any return to town.**

### Time and idling (#114, #200)

- **One world clock in days.** It runs while the party travels or has its
  stall open. Time away continues what the party was doing, up to **8
  hours**. Towns and families advance by world days, on a visit or when the
  map opens.
- **The party follows an itinerary** of destinations the player queues. After
  a success it sells until sold out or a set time passes, then leaves. With
  an empty itinerary, or after a failure, it waits.

### Cargo, the stall and the caravan (#83, #84, #144)

- One piece, one slot. Equipped gear, looks and pets take no cargo; coins and
  materials are counters. Saddlebags carry no combat bonuses.
- **Each town has storage.** The caravan stores nothing.
- **Listing is the only way to sell.** Prices from -50% to +50% of baseline.
  Sales stop at once on departure. No offer needs an answer. Demand is
  abstract first; named buyers, and townsfolk wearing what they bought, come
  later.
- **One standing number per town: goodwill.** Earned on completed sales and
  by helping; it gently improves prices and unlocks welcome, recruits and
  looks. Never construction speed or combat power.
- The caravan rolls on sun-charged wheels, has an exterior only, and its
  decoration is expression only.
- **Telekinesis is the answer for hands everywhere.**

### Towns, retirement and families (#112, #85, #113)

- A horse is either travelling or retired; there is no other "left in town".
  Any uncorrupted town the party has reached will do, with no limit on
  places. An invitation back is always accepted and costs the town nothing.
- Residents give construction progress to a few town projects. The party
  gains from finished projects only; no income accumulates.
- Partnerships form by themselves; the player only shapes conditions. Shown
  by journal and presence. Three life stages, advanced by the world clock.
  Young horses stay in town. The family tree keeps the whole campaign.

### Pets (#87)

- One automatic signature ability each, and nothing beside it. One collar
  slot specialises it. The pet saddle is seating and looks only.
- Reassigning is free in town. Pets never own a Finisher, never handle loot,
  are never captured by defeat, and are not exhausted to begin with.

### Gear and looks (#81, #128)

- The martingale is renamed the **breastcollar**. One braid slot. Slots are
  staged: mouth weapon, barding and saddlebags first.
- Gear specialises and never repairs; an item is small; no universal stats;
  equipment modifies abilities and does not replace them.
- **The appearance library is confirmed.** A look unlocks when its item is
  collected, for the whole save. Changing looks is free; gear can be hidden;
  dyes are reusable; presets are apart from loadouts; no collection bonuses.
  Look slots: one per gear slot, plus the other braid, a rug and leg wraps.

### Horses (#200)

- The base horse is **built by code** from the brief's proportions: the
  Friesian build, in whatever colours the genetics give. Foal proportions are
  derived by code for her approval.
- The player defines the **genotype of the starting horses**. Every later
  horse has a rolled look, and there is no other coat customisation.
- The breeding mod may be read for its natural coat genetics and nothing
  else, and never changed.

### The world and the ending (#89)

- Extraction and domination against restorative, communal technology.
- Defeated living creatures are freed and leave restored; the dragon's
  machines are broken.
- The dragon can be faced only when every other region is cleared.

### The screen and the release (#145, #189)

- In town the scene is the top third and menus fill the rest. **On the road
  the scene expands** when there is nothing to decide. Fixed proportion, with
  a minimum window size.
- Windows 10 and 11, x64. Application ID `9-7-8.HorseHeroes`, publisher
  `9-7-8`. Published from this repository, one stable channel, updates
  offered and chosen. Unsigned until a decision before 1.0. The manifest
  signing key is a GitHub Actions secret.
- No social features. No timed content, with one exception she made the same
  day: trade prices change each real-world day (below).

## The roads and the trade (owner, 2026-10-10)

Two changes she made after the decisions above, "to keep it interesting and
to provide the skeleton for the endgame after you kill the dragon". Where
they disagree with anything earlier in this file, they are newer.

### There is always combat on the roads

- Horses are prey animals, even in this world, and **bandits and predators
  are out on every road**. Every road has a chance of rolling some kind of
  combat; a journey may still pass without any.
- They are **not usually very dangerous**, and much easier than the dragon.
- **Longer distances between towns have more of them.**
- This is separate from corruption. Purity still sets how many and how
  strong the corrupted creatures and the boss are; a restored road has
  neither, and keeps its bandits and predators.
- **Drops: coins and trade goods, rarely gear.** Corrupted creatures and
  bosses remain the source of Rare gear and new looks, so restoring a region
  still closes its best loot.

### Towns trade in goods

- Towns buy and sell **non-combat materials**, such as wood, iron or apples,
  **at different prices**. The party buys in one town, loads the caravan,
  and picks a destination.
- **Prices change every real-world day, within a trend.** One city might
  always pay above average for apples; the exact amount fluctuates.
- The player can set up **automatic buying and a set of routes**, with
  conditions such as "only buy if their price is less than X".
- The model is Tiny Rails, taken almost exactly, apart from the daily price
  change ([inspirations.md](inspirations.md)).
- **Goods ride in a caravan hold** of their own, counted in units.
  Saddlebags stay for gear. This replaces "the caravan stores nothing" for
  trade goods only.
- **Trading opens at the second town** and is part of the main game; after
  the dragon it is the main activity.
- **A failed journey loses a bounded share of the goods aboard**, never all
  of them, beside the existing gear and coin rule.

What this changes elsewhere: the earlier loop-closing rule that buying and
reselling makes no money applies to gear at the stall, not to trade goods;
materials dropped on the road are goods in the hold, not bare counters; and
daily prices run on the real calendar, the one thing that does not follow
the world clock.

## The design priority

> Complexity through depth, not breadth.

Supporting features reinforce the journey and the loot-and-build loop. None
may become an independent management game.

**The central test for any addition:**

> Does a new horse, pet assignment or item make the player want to change who
> travels together, and can they see that choice matter on the road?

**Put depth into:** party selection, elemental cooperation, differentiated
equipment, routes, town residency, commerce, appearance collection.

**Stay away from, until the main loop works:** detailed survival needs,
extensive crafting, town construction, and many more damage types. The
summary's list also had breeding; she has since said breeding is in, as
background flavour with colour-only genetics, so what remains of that
guardrail is that it must never become a system the player has to work.
"Town construction" here means the player building a town as a management
game; residents rebuilding their own town by magic is confirmed and is not
that.

## Proposed, awaiting her decision

Most of what stood here was confirmed on 2026-10-10 and is in the section
above. What remains proposed:

- **Loot handling:** collect routine rewards automatically, aggregate the
  ordinary, spotlight upgrades, rare looks and build-changing pieces, then
  keep, equip or sell. Favourites and gear in saved loadouts are protected
  from automatic disposal.
- **What a major victory does:** corruption recedes, a town opens or
  recovers, routes become safer, residents offer aid, and horses, pets,
  styles and equipment become available.
- **Stories start small:** persistent characters, contextual events and
  remembered consequences, with a journey journal first.
- **Narration reports what happened.** It never invents an accomplishment the
  simulation did not produce.

## Her proposal: retirement and generations

A horse who leaves the party retires into a town and stays a named character
there. Healthy, restored towns support families; their children may later be
recruited, so the party is one generation in a longer effort. Towns advance
only when visited or when the world map is opened. The campaign has a finite
goal, cleansing every region. The idea is hers;
the details are recommendations. All of it is in
[generations.md](generations.md).

Her later statement settles how far it goes: coat colour is inherited through
a superficial genetics system, nothing inherited affects play, and the whole
thing is flavour. Its own advice still holds: prototype it small, and only
after the main loop works.

## Open

Every `decision` issue was answered on 2026-10-10. What is left is tuning and
content, and lives on the implementing issues: reaction timings, journey
length, purity gained per boss, which pet abilities come first.

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
header says it follows a document about a Windows installer and updater; that
arrived later the same day and is in [distribution.md](distribution.md).
