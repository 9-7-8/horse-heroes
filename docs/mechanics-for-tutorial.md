# Mechanics, for designing a tutorial

Every mechanic of Horse Heroes, built and planned, as of 2026-10-11, arranged
by what a new player has to be taught. Sources: `philosophy/vision.md` (which
outranks the rest), `expedition.md`, `menu-shell.md`, `item-card.md`, and the
issues on `9-7-8/horse-heroes`. Nothing here is a new decision. Where a line
is my reading and not the owner's words, it says "mine".

For each mechanic: **what the player decides**, **what happens by itself**,
and **status** (Built, or the release it is planned for).

The existing issue for tutorial pacing is **#47, Opening pacing** (0.3.3): an
unlock order in one data file, one new thing at a time, each tied to the
event that makes it relevant, and "the first trail needs no decisions at
all". A tutorial design should become that file.

**Since this was written (2026-10-11):** the tutorial documents written from
it are taken in. The guidance is `philosophy/onboarding.md`; the opening is
releases 0.2.2 to 0.2.4, epic #288; #47 is now the lessons of the later systems.

## 1. The one thing to teach first

The player is the **organiser**, not a rider. There is no avatar. Horses
walk, fight and pick up loot by themselves. The player decides in town; the
road is watched.

    horses walk -> fight -> loot showers out -> equip what is useful
      -> sell the rest in town -> fight something stronger

Everything below is either a link of that loop or a reason to care about it.

## 2. What the player never has to do

Worth saying outright in a tutorial, because players will look for these:

- Control a fight, aim, or trigger an ability.
- Pick up loot (it is collected; later, pets fetch it).
- Answer a prompt to keep the game moving. No offer needs an answer and no
  report holds the party up.
- Fear losing a horse or pet. **Neither can die.**
- Manage a full bag on the road. A full bag never stops a journey.
- Come back on a schedule. No timed content, apart from daily trade prices.

## 3. Built today (0.2.0, and the built part of 0.2.1)

This is what a tutorial could be tested against now.

| Mechanic | The player decides | By itself |
| --- | --- | --- |
| **The loop** | Nothing required | Town, trail, walk back to town. No town leaves by itself (#305); with Keep going on (G, on the Map) it leaves after 12 s, until a trail is lost |
| **Setting out** | Space, or the Set out button; Keep going, to have it leave by itself | Waits in town |
| **The party** | Nothing: Juniper, Morrow, Sable are fixed | One ability each, fired on a cooldown: Focused Shot (big single hit), Elemental Burst (hits all), Ward (halves damage) |
| **A trail** | Nothing | 3 encounters, then a boss. A quarter of trails have a fairy spring that heals |
| **Trail tracker** | Nothing | Footer marks: solid behind, ring for the current fight, outline ahead, a larger diamond for the boss |
| **Party element** | Fire, Ice or Plant (F, I, P), in town only | Fire beats Plant beats Ice beats Fire: x1.5 strong, x0.75 resisted. Enemy elements are rolled per trail |
| **Winning and losing** | Nothing | A win raises the trail level: enemies +15% health and attack, drops +1 stat point. A loss costs nothing yet |
| **The loot shower** | Nothing | A kill bursts coins that arc, bounce, settle and are vacuumed to the party; the boss gives the full fountain |
| **Gear drop** | Whether to equip it, and on whom | One piece per run if any encounter was cleared. Nothing equips by itself (#351): left in cargo it stays carried, and the cargo keeps every piece (#352) |
| **Slots** | Which horse wears what | Three: Head, Body, Saddle. Any horse wears any piece |
| **Rarity** | Nothing | Normal (white), Magic (blue), Rare (yellow), Legendary (orange), Set (green). Each also has its own shape, on the ground and in the menu. Today the top tiers are only more stat points and a beam |
| **Item card** | Press a piece to pin its card; Equip (E) | Shows the piece, where it is, and each stat before and after on the selected horse. The numbers only: no verdict, and no horse suggested (#351) |
| **Swap** | Equip on a piece another horse wears | The two horses change pieces; nothing is lost |
| **Camp appraisal** | Nothing | The report says where the drop will go and what it replaces, unprompted |
| **Coins** | Nothing: there is nothing to spend them on | Counted in the status strip |

Not built and likely to be assumed: saving (#44), spending coins, a horse
picker, buttons for most actions.

### The menus, as built

"This game is all menus." The frame is permanent and is the tutorial's map.

- **Scene on top, menus below.** In town the scene is the top third. On the
  road it grows to four fifths when no place is open.
- **A press on a horse in the scene, or its key, opens everything about
  it** in the side panel: its stats and where each point comes from, what
  it wears, and a part for each tab of Horses (#311).
- **A bar of six places, always visible:** Horses (H), Inventory (B),
  Trade (T), Map (M), Town (N), Journal (J), and Settings (S) at the end.
- **Tabs per place** (keys 1 to 4): Horses has Gear, Abilities, Pet, Look.
  Trade has Sell, Buy, Storage, Rules. Town has Projects, Residents. Journal
  has Report, Journal, Codex. Map has none.
- **Status strip:** coins, bags, hold, the day. Each horse has a tag under
  it in the scene with its health; press the horse, or Z, X, C, to select it.
- **Footer:** trail tracker, what the party is doing, Set out.
- **One selected horse**, shared by every place. Comparisons and Equip are
  always about it.
- **The item is the handle.** Press a piece anywhere and its card opens in
  the side panel with every action that applies.
- **Every planned button is already there.** One that is not built is drawn
  unavailable, and a press says what it will do. Nothing is silently
  unavailable: a line under each group says why ("in town only").
- **Mouse first, a key for everything.** Keys are printed on their controls;
  K in Settings lists them. Escape goes back one level and never does
  anything else. Arrows and Tab move focus.

Tutorial note (mine): because unbuilt buttons are visible and explain
themselves, the tutorial can point at where a system will live before it
exists, but should not send a new player to press one.

## 4. Planned, in release order

### 0.2.1 Menus first (in progress)

- **Undo** for everything reversible; confirmation only for what cannot be
  taken back (#230).
- **A place says when it wants attention** and stops once seen (#232,
  built: a dot on Horses for new gear in the cargo, on Journal for a
  lost trail or a Legendary drop).
- **Journal** as the one home for reports, journal and codex (#231).
- **Coloured text** per element and trait on items (#251).
- **Click budget** for the common jobs (#236).

### 0.3.2 to 0.3.4 The road (failure and saving, fighting together, the road)

Combat stops being one element for the party and becomes cooperation.

- **Abilities carry the element, not horses.** A horse fires whichever
  equipped ability best suits the enemy in front of it (#93).
- **Ability tags:** Primer, Finisher, Support (#97).
- **Primers:** a mark one horse leaves on an enemy. An enemy holds one
  Primer of each element (#94).
- **Finishers and pair reactions** (#95). A Finisher consumes one Primer:

  | Pair | Reaction | Does |
  | --- | --- | --- |
  | Fire + Ice | Thermal Shock | A burst on one dangerous target |
  | Fire + Plant | Wildbloom | Erupts into nearby enemies |
  | Ice + Plant | Wintergarden | A barrier for the party, which also bursts for damage |

- **Damage rules** (#224): defence is a percentage with diminishing returns;
  nothing misses (a failed accuracy check is a glancing hit); every hit can
  crit; bonuses of one kind add, different kinds multiply. An enemy's
  element is fixed by its kind. Basic attacks are neutral.
- **Exhaustion, not death** (#176). **Health potions**, used automatically
  (#177). Health carries between fights; quiet days restore part.
- **Failure** (#178 to #180): the party returns to the previous town, loses
  at most a tenth of the gear picked up on that trip (lowest rarity first,
  at most three pieces) and a small capped coin cost. It **waits** and never
  retries by itself. A retreat by choice is free.
- **The world:** a finite hex map, one landmass, the party in one corner and
  the dragon in the opposite one (#154). Each hex has a **purity** and a
  **corruption** score; danger comes from comparing them (#155).
- **Travel:** choose a destination town and see the route first (#156);
  distance sets travel days (#158); encounters follow danger and distance
  (#159). **A journey to a corrupt town ends with a boss at its gate**
  (#161); journeys between liberated towns have ordinary fights only.
- **Fixed on departure:** the route, gear and pets change in town only.
- **Towns** as the place decisions are made (#98). **Save and load** (#44).
- Seen, not taught: gaits and a bending horse (#255), parallax, sounds.

### 0.4.0 to 0.4.2 Cargo and town (cargo, the stall, pets and a roster)

The second half of the loop: selling.

- **Shared party cargo** (#34): loot belongs to the party; one piece, one
  slot. Equipped gear, looks and pets take none. Lock a piece to protect it.
- **Saddlebags** set capacity (#100). Full bags never stop a journey: the
  party leaves the new piece or drops a worse one.
- **Keep rules** (#52): the player tunes what is kept. Default: highest
  rarity first.
- **The caravan** (#146): the party's travelling home and shop.
- **The stall** (#147): listing is the only way to sell. Townsfolk buy over
  time. It opens on any return to town and stops at once on departure;
  unsold stock travels on.
- **Prices** (#101): set from -50% to +50% of baseline. Lower prices earn
  **goodwill**, the one standing number per town; it gently improves prices
  and unlocks welcome, recruits and looks.
- **Batch listing with protection** (#99); **sale preview and buyback**
  (#151); **saved loadouts** (#50).
- **Roster and travelling party** (#102). **Retire a horse** into a town and
  invite it back (#117): rare, free, gear and pets return to the party.
- **Adopt a pet and assign it to a horse** (#106).
- **Health returns over time while waiting in town** (#218), not instantly.

### 0.3.0 and 0.3.1 Every hit and choice felt (ahead of the road since 2026-10-11)

Mostly feedback, and the release the tutorial itself belongs to.

- **Opening pacing** (#47), **milestone reveals** for firsts (#42), every
  camp choice acknowledged (#40).
- **Readable combat:** attack phases, hit reactions, ability tells, boss
  announced, primer markers and finisher payoffs shown (#15 to #19, #166),
  floating damage numbers (#249).
- **Loot report** (#32): the ordinary summarised, the best revealed one at a
  time. **Trail report** (#51): what happened and why.
- **Feel settings** (#45): turn down shake, flash and effects.

### 0.6.0 to 0.6.2 Gear that matters

Where builds begin. "Town is mostly about assigning gear."

- **Seven slots, each with one identity** (#91):

  | Slot | Is for |
  | --- | --- |
  | Halter | Accuracy |
  | Barding | Defence and damage reduction; the only place for elemental resistance, capped at 75% |
  | Boots | Defence |
  | Breastcollar | Varied utility |
  | Horseshoes | Critical-hit chance |
  | Mouth-held weapon | Offence; can give basic attacks an element; adds elemental damage |
  | Braid (mane or tail) | Charms and build interactions; adds elemental damage |

  Plus saddlebags (capacity only) and a pet saddle (seating and looks only).
  No bridles, bits, reins or stirrups.
- **Affixes** with tags and a power budget (#69); **gear effects that change
  how a horse fights** (#13); **party sets** with thresholds (#36); each
  rarity tier given a job (#33). Gear modifies abilities, never replaces
  them, and never needs repair.
- **Party level and ability points** (#220): one shared level; a late
  recruit has the same points. **One shared pool of abilities** (#221).
  **A loadout of a few ability slots per horse**, set in town (#222). Points
  are reassigned freely in town. There are no classes.
- **Stat lean** per horse: hardy, quick or potent (#223).
- **Plan on the road** (#234): gear, ability, pet and look changes made on
  the road are queued and applied on arrival.
- **Appearance library** (#92): collecting a piece unlocks its look for the
  whole save. Gear, look and dye tracked separately (#129).
- **Codex** (#30); **town shop** to buy and use a service (#141); **pet
  collars**, one slot that specialises the pet (#139); coins and materials
  alongside gear in a drop (#5); one item list with the same filter, sort
  and search everywhere (#235).

### 0.7.0 to 0.7.3 A wider world

- **Itinerary** (#201): queue destinations. After a success the party sells
  until sold out or a set time passes, then leaves by itself. With an empty
  itinerary, or after a failure, it waits. This is the idle game proper.
- **Offline progress** (#31): time away continues what the party was doing,
  up to 8 hours. One world clock, in days.
- **Liberating a town** (#163): beat the boss at a corrupt town's gate and
  it is liberated at once; corruption on nearby hexes weakens. Fighting on
  the road clears nothing.
- **Route options** that really differ (#157), **off-road travel**, slower
  and more dangerous (#215), **biomes** setting speed, forage and predators
  (#213).
- **Bandits and predators** on every road, even restored ones (#207): easy,
  and they drop coins and trade goods, rarely gear. Rare gear and new looks
  come from corrupted creatures and bosses.
- **Road events** that are not fights (#160); **journey journal** (#107);
  **world map** of what is restored (#123).
- **Enemy roles** (#70): Armoured, Evasive, Swarm, Warded; one modifier per
  enemy, shown in shape and text (#71).
- **Run setup** (#216): world size and corruption reach. "Classic" (all but
  the starting town corrupted) is the balanced one.

### 0.8.0 to 0.8.4 The gaggle

- **Crescendos** (#96), same-element payoffs so that mixed is not always
  best: Flare (Fire, a burn), Deep Frost (Ice, a burst and a brief slow),
  Bloom (Plant, a hit and healing for the most injured horse).
- **Impairments** (#204): Slowed, Staggered, Rooted.
- **Accuracy and crits** as visible systems (#173, #174); **mouth-weapon
  families** (#175).
- **Recruiting** on the road (#104); **party grows from three to five** by
  restoration (#203). The three starters are expected to reach the end.
- **Horse identity** (#103): name, look, stat lean, one signature trait, a
  record of events.
- **Pet signature abilities**, one each, automatic (#138). Pets fetch the
  loot after the shower. Reassigning is free in town.
- **Build laboratory** (proposed, ruled on when the release is next): ability
  policies, modifiers, party stance, recovery routing, a party profile
  (#22 to #25, #49).
- **Events that remember** (#108).

### 0.9.0 to 0.9.2 Town life

- **Trade goods** (#209): wood, iron, apples and the like, bought in one
  town and listed at the stall in another. They ride in the **caravan hold**
  (#208), counted in units, apart from saddlebags. Trading opens at the
  second town.
- **Daily prices within a trend** (#210), on the real calendar.
- **Auto-buy rules and trade routes** (#211): "only buy below X", and a
  price floor for selling.
- **Foraging** (#217): goods gathered from hexes crossed, by biome.
- **A failed journey loses a bounded share of goods aboard** (#212).
- **Town projects** (#172): the party delivers goods and coins, residents
  build, and the town's healing rate, prices or goods improve.
- **Town states** (#115): Corrupted, Struggling, Recovering, Flourishing.
  **Town storage** (#202), a stash per town. Larger towns sell faster.
- **Residents as customers** (#148), **stall report** of what sold while
  away (#170), **caravan decoration** (#149).

### 0.10.0 to 0.12.0 Looks and families

Expression only: none of this changes combat.

- **Generated horses and visible gear** (#55); the player defines the
  **genotype of the starting horses**, and every later horse has a rolled
  look. **Breeds** decide colours and gaits (#266; what else is open, #267).
- **The wardrobe** (#127): looks apart from gear, free to change, dyes
  reusable, presets apart from loadouts, gear can be hidden, no collection
  bonuses.
- **Generations** (#111): partnerships form by themselves, children grow up
  in town through three life stages, descendants can be recruited and look
  like their family. Inheritance is colour only.

### 0.13.0 The dragon falls

- **The dragon can be faced only when every town is liberated** (#124).
- **The campaign is finite.** Defeated creatures are freed, not killed.
  Afterwards the save stays open and trading along never-quite-safe roads is
  the game (#183).

## 5. A teaching order (mine, for the owner to rearrange)

Each step is one new idea, tied to the event that makes it matter, as #47
asks. Steps 1 to 4 work in the game as built.

| # | Teach | When | Built |
| --- | --- | --- | --- |
| 1 | The party goes by itself; watch | First trail, no decisions | Yes |
| 2 | Loot comes out of kills, and rarity has a colour and a shape | First kill, first boss fountain | Yes |
| 3 | Press a piece, read its card, Equip on the selected horse | First gear drop | Yes |
| 4 | The trail tracker, and that a win makes the next trail harder | Second trail | Yes |
| 5 | Elements: what beats what | First loss, or first resisted fight | Yes, as a party element |
| 6 | Failure is cheap and the party waits for you | First loss | 0.3.2 |
| 7 | Choosing a destination; corrupt towns end in a boss | First town reached | 0.3.4 |
| 8 | Primer then Finisher makes a reaction | First hard fight | 0.3.3 |
| 9 | Cargo, and listing at the stall | First bag with spare gear | 0.4.0, 0.4.1 |
| 10 | Prices and goodwill | First sale | 0.4.1 |
| 11 | Slots have jobs; ability points and loadouts | First level up | 0.6.0, 0.6.1 |
| 12 | The itinerary: queue towns and leave it running | Second liberated town | 0.7.0 |
| 13 | Trade goods and the hold | Second town | 0.9.1 |
| 14 | Pets, recruits, looks, families | As each first appears | 0.4.2 to 0.12.0 |

## 6. Things a tutorial design will run into

- **The built element system will be replaced.** One element for the whole
  party (step 5) goes in 0.3.3, when elements move to abilities. Anything
  taught about it now is taught twice.
- **Nothing equips by itself** (owner, 2026-10-12, #351). A drop left alone
  stays carried, so a new player who never opens a card walks every trail
  bare. That is a reason step 3 needs its prompt (mine).
- **Legendary and Set promise more than they give** until #33.
- **Reactions are for hard fights**: an easy fight may end before one fires,
  so step 8 cannot be taught on an easy trail.
- **Not decided yet:** the size of the classic world; which pet abilities
  come first; what a breed affects beyond looks (#267); the build laboratory
  issues, which are proposals.
