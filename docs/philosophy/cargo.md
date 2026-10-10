# Cargo and what the party keeps

> 2026-10-10: the owner replaced two things below. The tiers are now Normal,
> Magic, Rare, Legendary and Set, not Common, Uncommon and Rare; and rarity
> colours no longer have to be distinguishable from the element colours.
> See [vision.md](vision.md), "Rarity tiers and their colours".

How loot is carried, what survives the journey, and how it is handled on
arrival. Five points here are the owner's confirmed decisions and are listed
first. The rest is a small set of proposed details.

> Give players the spectacle of abundance and the authority to define value,
> without making them shuffle loot among horses or interrupting every fight.

Saddlebags decide how much the gaggle brings home. Retention rules decide
what survives the journey. Town turns the cargo into builds, fashion, trade
and the next reason to travel.

## Confirmed by the owner

1. **Loot belongs to the whole party.** There is no personal inventory per
   horse. Bookkeeping is abstracted wherever it would add tedium and not an
   enjoyable choice.
2. **Saddlebags let the party carry more.**
3. **A full bag does not stop the journey.** The party carries on, and either
   leaves a new item on the ground or abandons one it is carrying to make
   room for a better one. Capacity alone never sends the party home and never
   skips the destination boss. *This replaces the earlier suggestions that a
   full bag ends the trip.*
4. **The player can tune what is kept.** By default the party keeps the
   highest-rarity equipment and drops what ranks lower.
5. **Rare gear is exciting:** a clearly different rarity colour, a more
   distinctive appearance, and a more impressive reveal than common gear.

## Principles

### The loop

    fight -> loot fountain -> drops evaluated automatically
      -> the best cargo the rules allow is kept -> carry on -> reach town
      -> look at the highlights -> equip or keep -> sell the surplus in a batch
      -> travel again

Routine decisions are automated. Interesting discoveries stay visible. The
player may step in, and successful idle play never needs them to.

### Shared capacity

- One piece of equipment takes one slot, to begin with.
- Party capacity is a base allowance plus what each active horse's
  saddlebags contribute. Nothing is distributed among horses or moved between
  bags.
- A better bag buys more retained loot and more to sell per journey. **It
  does not mean more drops or higher rarity**, and it never carries combat
  stats.
- Equipped gear, unlocked looks and pets take no cargo. Spare equipment does.
  Coins are a separate counter. If materials ever take space, say so
  explicitly; never assume every collectible uses the same capacity.

### When a drop arrives

1. Do the rules allow keeping it? If not, leave it, even with room to spare.
2. If allowed and there is room, take it.
3. If allowed and full, find the lowest-priority carried item that may be
   replaced.
4. Replace it only if the new item **strictly** outranks it.
5. Otherwise leave the new item.

- **Protected items are never candidates.**
- **Rarity comes first by default.** Do not let a hidden power estimate
  outweigh it: the player chose highest-rarity retention.
- **On a tie, keep what is already carried.** No pointless churn.
- If everything carried is protected, leave new items and show a short
  warning. Never sell, salvage, destroy or enlarge capacity automatically.
- **A fountain is evaluated as one batch in a stable order**, so the result
  never depends on which drop happened to spawn first.

### Protection and intent

- Equipped items are outside replaceable cargo.
- Locked or favourite items are never abandoned automatically.
- Spare items in a saved loadout are protected, or need explicit permission
  to become candidates.
- "Keep this" locks an item until it is released.
- **Protection does not create room.** A bag full of locked items collects
  nothing new, so show how much cargo is protected.
- Abandoning by hand warns about favourites, loadout pieces and unique story
  items. Anything a quest depends on is handled separately, never by the
  equipment filter.

### Rules: simple before advanced

- The default preset is **Highest Rarity**. Later presets: for the current
  party (slots, classes, element tags), for hunting undiscovered looks, for
  sale value, and custom ordered rules.
- A small first vocabulary: **Keep, Ignore, Protect**, on rarity, gear
  category and an element tag.
- Rules read as sentences: "Always keep Rare or better." "Ignore Common
  boots." "Never abandon locked items."
- Precedence is explicit. In the advanced form, rules run top to bottom and
  the first match decides; unmatched items follow the preset; protection
  overrides replacement.
- **Keeping and selling are different settings.** An item allowed into cargo
  is not thereby approved for sale.
- No scripting language and no huge editor to start with: a reliable rarity
  preset, a few toggles, and a preview.

### Preview and explanation

- Rules can be tested against current cargo without dropping anything, with
  counts: would keep, would ignore, protected.
- Every rejection or replacement has a reason: "Left behind: lower rarity
  than carried gear." "Replaced Common X with Rare Y." "Left behind: all
  cargo is protected."
- **No popup per common rejection.** Routine outcomes go to a loot log; rare
  finds and unusual rule conflicts are spotlighted.

### What is left behind

- Abandoned gear cannot be recovered from elsewhere in the world. It stays on
  the encounter's ground briefly and is gone when the party moves on. Say so.
- Picking something back up before leaving follows the same capacity rules
  and never makes a second copy.
- **No persistent piles on world tiles.** That would weigh down a light world
  and invite tedious backtracking.
- Unwatched, the same keep-and-replace logic runs without drawing anything,
  and saves the resulting cargo with a compact log of what was notable.
- Reward generation, retention and encounter progress are committed together,
  so a crash or reload can neither duplicate loot nor reroll a reward.

### Rarity, shown several ways

Not colour alone: label and border colour, a rarity name or icon, silhouette
and ornament, the drop sound and reveal, and for important drops a brief beam
or ground marker.

- **Common:** modest materials, restrained trim, quiet feedback.
- **Uncommon:** a more deliberate palette or a distinctive motif.
- **Rare:** a stronger silhouette, richer accents, a recognisable ornament or
  effect.
- Any later tier gets a signature treatment, never merely larger numbers.

**Rarity colours must be distinguishable from the Fire, Ice and Plant
colours.** Rarity says how important; element says what it does.

The fountain can look abundant without creating appraisal work: coins and
routine rewards resolve together, ordinary equipment is readable but modest,
a Rare gets a brief moment to itself. **Do not hide rejected loot so
thoroughly that a victory feels empty.** Show the burst and a small summary,
not a screen of labels.

### Cooler is not mandatory fashion

A rare look should be appealing and recognisable, but "more elaborate" is not
the only kind of attractive. A plain Common blanket may be exactly right for
a loved horse. Rarity influences the generated look; it never limits the
player's freedom to use one.

On unlocking looks, **not yet confirmed**: the recommendation is that a look
is unlocked when its item is actually collected, not merely seen and
rejected, and stays unlocked if the item is later abandoned or sold. The
alternative, unlocking on drop, is valid and makes capacity matter less to
fashion. "New look collected" is shown separately from "possible upgrade".

### What deserves a look

Individual attention is for an item that brings a real decision: Rare or
better, a new named effect, a plausible alternative build, a new look,
something for a future recruit or pet, a favourite or a piece with a story.

Batch handling is for known surplus: ordinary duplicates, things already
categorised, gear that suits no build being kept.

**A typical arrival should offer a few meaningful comparisons, not dozens.**
Tune drop rates, rarity weights and capacity to that experience.

"Rare" must remain a meaningful tier. If most of what is kept late in the
game is Rare, excitement has to come from effects, new looks and build
relevance, because colour alone will have stopped working.

### In town

- On arrival: cargo kept, Rare and new items, compatibility with horses and
  loadouts, looks already unlocked, a suggested sale batch with its total,
  and what is locked.
- Selling happens in town, from the caravan. Not on the road.
- Sale suggestions respect protection, and the count and value are previewed
  before the sale.
- Prices that depend on relationships belong to town trade, not to the road
  filter. A sale-value preset uses base value; it does not pretend to know a
  town's future offer.

### When capacity changes

Changing the party, retiring a horse or swapping bags can lower capacity.
**Cargo is never silently dropped as a side effect.** Resolve it in town:
preview the new limit, and have the player sell, store or abandon the excess
before leaving.

Town storage is an optional future feature, not an assumed bank. If a caravan
ever stores things, keep that distinct from cargo, and do not make saddlebags
meaningless with unlimited mobile storage.

### Smallest version

One shared inventory; slot capacity from a base plus active saddlebags; three
rarities; rarity-first retention with stable ties; locking; a small rule
panel; automatic replacement without stopping; rarity-specific look and
sound; a compact loot log; compare, equip and batch-sell in town.

Deferred: weight, compartments, remote retrieval, sorting puzzles,
complicated filter scripts.

## For Horse Heroes

- Decision 3 settles the tension between a huge fountain and low management
  that earlier files left open ([loot-fountain.md](loot-fountain.md),
  [gear.md](gear.md)): the fountain is as generous as it likes, and the bag
  sorts itself. It also keeps the owner's travel rule intact: every journey
  still reaches its boss ([travel.md](travel.md)).
- The slice has no inventory: one drop, equipped or lost. Shared cargo is the
  first inventory the game will have, and it should be built as shared from
  the start.
- Rarity-first is a rule about **carrying**. Which piece a horse wears is a
  separate automatic choice, and with gear categories that own different
  questions it cannot be "highest rarity" either. Keep the two defaults
  apart.
- Evaluating a fountain as one batch means the sim decides what is kept
  before anything is drawn. The pickup animation then shows a decision
  already made: what flies to the party is what was kept, and what stays on
  the ground is what was left. That makes the vacuum honest.
- "Do not hide rejected loot" and "a few comparisons, not dozens" are two
  limits on the same dial. Drops per boss, rarity weights and capacity have
  to be tuned together, by watching, not from a formula.
- Rarity colours distinct from element colours is a constraint on a palette
  not yet chosen. The elements are currently shown by colour on the enemy
  cubes; whatever is picked for rarity must not collide.
- The sim is seeded and deterministic, so "offline and watched give the same
  cargo" is a test that can be written directly.

## Source

Owner's design document on loot and saddlebags, received 2026-10-09 (all
sections). Its retention system adapts the general idea of a loot filter and
says so; it is original to this game.
