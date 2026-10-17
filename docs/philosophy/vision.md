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
  aesthetic. *(Revised 2026-10-11: a foal's two signatures and its build
  are inherited as well. Still optional flavour. See "Signatures, breeds
  and gaits".)*
- **There will be breeds, later, and a breed decides a horse's colours and
  its gaits** (owner, 2026-10-11: "We will later have different breeds of
  horses in the game, which will dictate their colors and gaits"). How a
  horse moves: `horse-movement.md`. *(Later the same day she settled that a
  gait is more than looks, and that a breed also has its own build and its
  own pool of signature parts: see "Signatures, breeds and gaits". How a
  breed meets the colour genetics is still open, #267.)*
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
  more impressive reveal. (The tiers and colours were named on 2026-10-10:
  see "Rarity tiers and their colours" below.)

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
- *(Replaced the same day; see "Abilities, levels and no classes".)*
  Attunement belonged to the horse, and each class kept one fixed role.
- One accuracy system for attacks and spells. Reactions never miss or crit; a
  miss does not consume a Primer.
- Impairments are a short fixed set: Slowed, Staggered, Rooted.

### The party (#86)

- Starts at three and **grows to five**, a place at a time, by restoration.
- *(Replaced the same day: there are no classes and no innate element.)*
- A horse's identity: name, appearance, a stat lean, one modest signature
  trait, a record of notable events. Personality is expressive only; a horse
  always goes where it is sent. *(2026-10-11: the trait is read as replaced
  by two generated signatures; see "Signatures, breeds and gaits" and #316.)*

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
  *2026-10-11: it is set on the world map and spans more than one town;
  see "Leaving town".*

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
  *(2026-10-11: the gait of the horse a pet is with now boosts it; see
  "Signatures, breeds and gaits".)*
- Reassigning is free in town. Pets never own a Finisher, are never captured
  by defeat, and are not exhausted to begin with. *(Revised the same day:
  pets do fetch the loot; see "The loop as she tuned it".)*

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
- **It installs to Program Files by default, not the user's application
  data (owner, 2026-10-11, after installing 0.2.0).** That replaces "per
  user, no administrator rights" from the installer guide: Windows asks for
  administrator rights at each install and each update. Saves and settings
  still live under the user's local application data.
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
  still closes its best loot. *(2026-10-12: she has since asked for far
  more drops of all loot, #352; "rarely gear" here is to be read against
  that, and how often a restored road drops gear is tuned then.)*

### Towns trade in goods

- Towns buy and sell **non-combat materials**, such as wood, iron or apples,
  **at different prices**. The party buys in one town, loads the caravan,
  and picks a destination.
- **Prices change every real-world day, within a trend.** One city might
  always pay above average for apples; the exact amount fluctuates.
- The player can set up **automatic buying and a set of routes**, with
  conditions such as "only buy if their price is less than X". Selling takes
  the same kind of limit: a price floor below which the goods stay aboard.
- **Goodwill moves market prices**, as it does stall prices.
- The model is Tiny Rails, taken almost exactly, apart from the daily price
  change ([inspirations.md](inspirations.md)).
- *(Revised the same day: goods are sold through the stall over time, as
  gear is, not instantly.)*
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

## The generated world (owner, 2026-10-10)

Her design for the map, given after the decisions above. Where it disagrees
with anything earlier in this file, it is newer. It replaces two of them:
"danger is purity alone" and "a boss victory raises purity nearby".

### How the world is made, in order

1. **A finite world of hexes, exactly like RimWorld's, except that it is
   always one huge landmass**, so horses never have to swim.
2. **The party starts on a random hex in a far corner. The dragon is on the
   opposite hex.**
3. **Biomes** are laid out around the map. Mountains and lakes are obstacles.
   A biome determines **travel speed**, **what goods can be foraged**, and
   **how many and what kind of predators** there are.
4. **Towns are scattered** around the map, and **roads are built along the
   shortest distance between them**, going round mountains and lakes.
5. **The reach of the dragon's corruption** is set. It can be changed when
   setting up a run. In a **classic run the entire world except the starting
   town is corrupted**.

**The size of the world can also be set when starting a run.** Larger worlds
mean longer runs. **The "classic" setting is the one being created and
balanced now**; the others exist and are not balance targets yet.

### Travelling it

- A hex with no road can be crossed, **slower and more dangerously**.
- **The party forages goods while moving through a hex**, and the biome
  determines which. Foraged goods are trade goods and ride in the hold.
- Corruption is **weaker the further it is from the dragon**.
- **Fighting on the road clears nothing by itself.** Corruption is cleared
  only when the party reaches a corrupt town and beats the boss at its gate.
  That town is then cleared, and the corruption on nearby hexes is weakened.
  *(She first described an idle task in the town, such as picking up trash
  or tearing down a factory, and removed it the same day: "killing the boss
  is cleansing enough".)*

### Purity against corruption

- The starting town provides a **purity score**; the dragon provides a
  **corruption score**; every liberated town raises the purity of the hexes
  around it.
- **A hex's danger comes from comparing its purity and corruption scores.**
- **Liberated towns have a pure aura; corrupt towns have a corrupt aura.**

### Settled by her the same day

- **The dragon can be faced only when every town is liberated.**
- **A journey ends with a boss when it arrives at a corrupt town**: the boss
  is the fight at its gate. Journeys between liberated towns have ordinary
  fights only.
- **Beating that boss liberates the town at once.** There is no cleansing
  wait, so a town the party is inside is always open.

"Cleared" therefore now means a town liberated, in place of the earlier
three-part rule for a region. Not yet stated by her: how large the classic
world is.

## The loop as she tuned it (owner, 2026-10-10)

Her changes after reading the whole loop back. Newer than everything above.

- **Town is mostly about assigning gear.** The player is meant to really
  invest in defining and tuning builds for their horses.
- **The three starting horses are expected to reach the endgame.** Retiring a
  horse is a very rare event. The party still grows to five; the starters
  stay throughout and recruits fill the new places.
- **Health returns to full over time while the party waits in a town.** This
  replaces instant healing on return.
- **Town building projects** improve a town's healing rate, its trade prices,
  or what goods it sells. The party takes part by **delivering goods and
  coins**; residents do the building.
- **Pets grab the loot after the shower and put it in the bags.** This is how
  collection looks; what is kept is still decided by the keep rules and bag
  space.
- **Trade goods do not sell instantly. They sell the way gear does**, listed
  at the stall and bought over time. **A larger town with more residents
  sells faster, because it gets more visitors.**
- **No cleansing wait.** Killing the boss is cleansing enough.
- **Classic is a single finite experience, like completing a story mode.**
  The classic save stays open to trade afterwards. An **endless mode**, where
  the dragon's corruption spreads by itself and towns may need liberating
  again, may be made later and is not classic.

## Abilities, levels and no classes (owner, 2026-10-10)

Her change after reviewing the combat system. Newer than everything above.

- **Horses level up and unlock new abilities by levelling**, as in Diablo III
  and Path of Exile 2.
- **The game is classless.** Points buy abilities. Ranger, Wizard and Mage
  are gone as classes.
- **Elements are tied to abilities, not to horses.** In her words, tying the
  element to the horse "might end up hampering my ability to create more
  abilities to unlock". No horse has an innate element.
- **Each horse has a range of abilities and uses the one that is a good
  match-up** against what it is fighting, by itself.

Settled by her the same day:

- **A few loadout slots per horse**, set in town; the best equipped ability
  fires.
- **One shared pool of abilities** for every horse. Horses differ by a **base
  stat lean** and one signature trait.
- **Points are reassigned freely in town.**
- **One shared party level.** A horse who joins late has the same points to
  spend.

Primer, Finisher and Support remain, as tags on abilities. The reaction and
Crescendo rules stand unchanged.

## Combat rules settled (owner, 2026-10-10)

Her answers after the combat review. Newer than everything above, and they
replace four lines of the morning's "Combat (#80)" section: one Primer per
enemy, misses, reactions never critting, and flat defence.

- **Armour can protect against a specific element, and gear can add to an
  element's damage.** Resistance is on **barding only**, capped at **75%**.
  The damage bonus is on the **mouth weapon and the braid only**.
- **An enemy's element is fixed by its creature kind.** It attacks with that
  element. Horses have none of their own; their elemental defence is gear.
- **Defence is a percentage with diminishing returns**, not a subtraction.
- **Nothing misses.** A failed accuracy check is a glancing hit for reduced
  damage; a glancing Finisher still consumes its Primer, for less.
- **Every hit can crit**, reactions included.
- **Bonuses of one kind add; different kinds multiply.** The kinds are gear
  bonus, match-up, crit and Primer charges.
- **An enemy holds one Primer of each element.** A Finisher consumes exactly
  one, the best payoff, and leaves the rest.
- **Every payoff also hits.** Wintergarden's barrier bursts for damage; no
  reaction is purely defensive.
- **Reactions are for hard fights.** An easy fight may end before one fires.
- **Four enemy kinds:** Armoured, Evasive, Swarm, Warded.
- **Basic attacks are neutral unless the mouth weapon gives them an
  element.** They never apply or consume a Primer.

## Cadence and speed (owner, testing 0.2.3, 2026-10-14)

- **The horses attack at different cadences**, "to make the combat feel
  more natural": a party never hits in step.
- **Speed is a stat, and it governs two things:** how quickly the horses
  move between encounters and how often each attacks ("let's add a speed
  stat that governs both"). Gaits set it later (Gaits, below). How strongly
  is tuning, and a number in data.

## Every piece looks its own (owner, 2026-10-14)

- **Three layers**: a background in the piece's rarity colour, in its
  slot's shape (the same for every piece of that slot); a pattern on it
  (spots, diamonds...); a decoration on top (stars, vines, flames...).
- **Cosmetic for now**: pattern and decoration say nothing about stats or
  element. Drawn very simply, in the menus and on the horse (#361, #359).

## Menus and navigation (owner, 2026-10-10)

Her judgment: "This game is going to live and die on how easy and intuitive
the menus are to navigate." Four choices she made from options put to her
the same day. The advice built on them is in [menus.md](menus.md).

- **Five places, horse first:** Horses, Trade, Map, Town, Journal, in one bar
  that is always visible. Town opens on Horses. Gear, abilities, pet and look
  are tabs on the selected horse. This replaces the four places (Party,
  Trade, Wardrobe, Town) proposed in town.md. On 2026-10-11 she gave the
  Inventory a place of its own, after Horses ([inventory.md](inventory.md)).
- **Mouse first, with a key for everything.** The whole game can be played
  without the mouse. A controller is not a design target.
- **On the road, look and plan.** Every place opens on the road. Gear,
  ability and pet changes made there are queued and applied on arrival, so
  "gear and pets change in town only" still holds. **Looks queue too**
  (owner, 2026-10-10, #239): a look changed on the road is planned and
  applied on arrival like gear, so every change to a horse follows one rule.
- **Menus are built first.** "The menus should be one of the first things we
  build, so we can tune it throughout. All buttons should be there, even if
  they're not functional, as early as possible."

## Rarity tiers and their colours (owner, 2026-10-10)

"Use the same rarity tiers and coloring as Diablo 3."

- Five tiers, lowest first: **Normal** (white), **Magic** (blue), **Rare**
  (yellow), **Legendary** (orange), **Set** (green). They replace Common,
  Uncommon and Rare.
- "Rarity tiers don't need to stay distinguished from element colors,
  because rarity tiers apply on drops and in menus only." This replaces the
  rule in [cargo.md](cargo.md) that rarity colours must be distinguishable
  from the Fire, Ice and Plant colours.
- "All five drop" now, though Legendary powers and set bonuses are not
  built: until they are, those two tiers are more stat points and a beam.
  Giving each tier its job is #33.

Mine, not hers: the tier names are that game's; the exact hex values, drop
weights, stat points and name prefixes in `gear.ron`; and that Legendary and
Set are the tiers with a beam.

## Tutorials (owner, 2026-10-11)

- **Tutorials are interactive and taught through gameplay.** "Never, ever,
  ever just a wall of text a player is expected to read."
- **Once 0.2.2 has shipped, every new feature has a tutorial somewhere**, as
  it has a test.
- **The game starts on the road.** The player learns the basics of combat by
  watching them happen. In 0.2.2 that is what is built, the three abilities
  and the loot; "they see the setup, they see the finishers" holds once
  0.3.3 ships them, and they join the watched opening then.
- **The first town does not leave by itself.** "You need to give the player
  a chance to breathe, to get acquainted... that point in the town will
  really be their first chance to explore the UI, so we need to not rush
  them through it." The party waits there until the player sets out (#302).
  This is a third case of waiting in town, beside a failed journey and an
  empty itinerary; later towns leave by themselves as before.
  *Replaced 2026-10-11 by "Leaving town" below: no town leaves by itself,
  so the first is no longer a special case (#302 closed for #305).*
- **A first-town list of things to try**, each paying a small amount of
  coins when done, "like clicking or equipping an item" (#303). A short list
  for the first town, kept as data so a later feature can add its entry; not
  a general quest system.
- **The first trail of a new game is authored** (confirmed 2026-10-11, #289
  question 2). One fixed plan kept as data: the same fights, enemy elements
  and first drop for every new game, whatever the seed, under the ordinary
  combat rules, ordered so each of the three abilities is seen and the drop
  is a plain upgrade. The generator takes over from the second trail (#294,
  designed in #293). She chose this over a first trail rolled from a gentle
  pool and over one rolled like any other. ~~Whether it can be lost is a
  separate question and still open (#289, question 3).~~ *Answered below.*
- **The tutorial trail cannot be lost, by tuning alone** (confirmed
  2026-10-16, #289 question 3). It is tuned under the ordinary combat rules
  so the starting party wins it with any abilities it may carry and on any
  seed, and a test holds that. There is no tutorial-only rule: no health
  floor, no rescue, no forced win. Failure is taught later, once there is a
  preparation choice to learn from (#178). She chose this over a tutorial-only
  safety net, over a trail that can be lost like any other, and over one
  whose loss costs nothing.
- **Until saving exists (#44), the lessons play again on every launch**
  (confirmed 2026-10-17, #289 question 5), as they do now: Skip all hints
  puts them all away for that run. She chose this over remembering them
  only for a first launch and over hints off by default.
- The opening is releases 0.2.2 to 0.2.4 (#288). The advice on how to teach is
  [onboarding.md](onboarding.md); those two lines are hers and outrank it.

## Leaving town (owner, testing 0.2.1, 2026-10-11)

- **No town leaves by itself by default.** "They shouldn't automatically
  leave town by default unless the user has set up an itinerary that spans
  through multiple towns, which they have to do on the world map." The
  countdown goes; the party waits at the gate until the player sets out.
- **An itinerary is set on the world map and spans more than one town**
  (#201). It is the only thing that makes the party leave a town by itself.
- **Until the itinerary exists, a Keep going switch stands in for it**
  (#305): off by default, in town, for the one repeating trail. She chose
  this over the game stopping at every town until #201. It is a stopgap and
  is removed when the itinerary ships.
- **The game starts and the horses are instantly walking** (#292, said
  again).
- **The fight log lies over the road scene, on the left**, beside the damage
  and heal numbers, and moves out of the Journal (#306).
- **A press on the road where the horses are closes the open place** (#307).

## Signatures, breeds and gaits (owner, 2026-10-11)

From her working design on procedural abilities and breed mathematics,
which marks these as settled. Newer than everything above. It replaces
three earlier lines: "one modest signature trait", "everything breeding
affects is aesthetic", and the open question of whether a gait is only
looks. The reasoning is in [signatures.md](signatures.md),
[horse-shape.md](horse-shape.md) and [horse-movement.md](horse-movement.md).

### Two signatures on every horse

- **Every horse has exactly two permanent, procedurally generated
  signatures: one for combat and one for town.** They are assembled from a
  small vocabulary of parts, never chosen from a list of finished
  abilities. Neither takes a loadout slot, and a town benefit never
  replaces the combat signature.
- **Town means everything that is not combat:** travel, forage, recovery,
  construction, trading and other systems to come.
- **A town signature is active for every horse physically in the town**,
  resident or visitor. Permanent residency or an assignment is not needed.
- **Town signatures stack.** Tuning residents can be optional endgame
  content and must not become necessary for the main adventure. No
  strongest-only rule and no global diminishing return without her say.
- **Each breed has its own pool of parts.** Pools may overlap; "breed is a
  way to deliberately search for particular predicates".

### What a foal inherits

- **Each part of each signature comes from one parent or the other**, half
  and half, after a 1% chance that the part mutates. A foal does not copy a
  whole signature or the pair.
- **Numbers are exact parental values**, never blended, placed only where
  they mean the same thing. The foal rerolls until both signatures are
  valid.
- **A clone of a parent is allowed.** No forced mutation and no novelty
  requirement.
- **Crossbreeds follow inherited genetics, not an average of two breeds.**
  The real coat genetics stay, and beside them are abstract heritable loci
  for body type, limb proportions, height and related dimensions.
- **Breeding is optional flavour**: not needed for the main loop, and no
  guarantee of novelty.

### Gaits

- **Breed gaits affect travel speed and give a boost to pets.** There is no
  travel-efficiency stat.
- **The party travels at the arithmetic average** of its horses' speeds,
  not the slowest horse's and not a chosen leader's.
- **Different gaits boost different aspects of pets.** Each boost is an
  explicit rule, never a side effect of how fast the legs are drawn.

Mine, not hers: that the two signatures replace the signature trait (#316);
that the six breeds her document works with (Mustang, Friesian, Quarter
Horse, Gypsy Vanner, Shetland, Thoroughbred, with walk, trot, canter and
gallop) are the answer to "which breeds" (#267); and every release these
were filed under. Every number in the document is a proposal.

## Builds are the player's (owner, testing 0.2.2, 2026-10-12)

- **"The main choices people get to make in this game is their builds."**
  Anything that makes a build choice for the player takes the game away
  from them.
- **Nothing equips by itself, and the game does not suggest a horse for a
  piece.** "Items shouldn't auto-equip to a certain horse, or even suggest
  it, because that's taking too much choice away from the user." (#351.)
  This overrules auto-equip as a fallback, wherever an older file allows
  it. Asked the same day, she chose: the comparison on the selected horse
  keeps its numbers, and its verdict line ("Upgrade for Sable: +11") goes.
  The game gives the numbers and never says a piece is better.
- **Far more drops, chaff and all.** "We need to massively increase the
  drop rate of all loot. It's okay if there's lots and lots of chaff, I
  want players to have more agency in changing their builds by giving them
  more options." (#352.) How much, the same day: "Every monster should
  drop at least one piece of gear, often multiple. Rare gear is still
  rare, but you get a lot more options among common gear, especially early
  on." Options come before tidiness: the answer to too
  much loot is tools for sorting it, not fewer drops.
- **A hint has to be met.** "The hints are definitely not obvious enough
  for them to ever be encountered, you'll likely need to make them overlays
  or pop-ups." (#353.) The line under the status strip is not enough.
  Asked the same day whether a lesson ever stops the game, she chose
  **always a pop-up**: every lesson pauses the game, on the road as in
  town, until it is answered. That overrules "never holding the party up"
  for a lesson that is showing. The tutorial rule above still holds: a
  pop-up shows or asks for one thing, with at most a line, and is never a
  wall of text.

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

Nothing in this file is still proposed. Most of what stood here was confirmed
on 2026-10-10 and is in the section above. The last four, below, were
**confirmed by the owner later on 2026-10-10** and are decided:

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

*Open again since 2026-10-11:* what her document on signatures and breeds
leaves to flesh out (#316: how complex a signature may be, each breed's
pools; #267: the colour genetics of a breed, the body loci, which gait
boosts what in a pet), and the tactics proposal, which is advice she handed
over and not yet her decision (#338, [tactics.md](tactics.md)). The
opening's questions (#289) are all answered: 3 on 2026-10-16 (the tutorial
trail cannot be lost, by tuning) and 5 on 2026-10-17 (lessons play on every
launch until saving exists).

Issues labelled `proposed` are ruled on **per release** (owner, 2026-10-10):
when a release is next to be built, its proposed issues are put to her then,
not earlier.

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
