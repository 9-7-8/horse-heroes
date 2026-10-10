# Town and the caravan

The half of the game that is not combat. Selling in towns, buying, residents,
relationships that move prices, recruiting and retiring are all confirmed or
already proposed ([vision.md](vision.md), [generations.md](generations.md)).
The caravan was first a proposal; **the owner has since confirmed it as the
party's travelling home and shop**, with the six points below. Arrival rates,
price limits, reputation formulas and upgrades are still to be prototyped.

## Confirmed by the owner

1. The caravan is the party's **travelling home and shop**.
2. The player **lists gear for sale; local horses and townspeople buy it over
   time**.
3. The player **sets prices as a percentage above or below a baseline**.
4. **Lower prices improve relationships** with townsfolk.
5. **Unsold stock comes with the party** when the caravan leaves.
6. The shop is **idle-friendly and simpler than a business simulator**.

> The caravan shop is an idle social marketplace, not a customer-clicking
> simulator.

> Combat says: "look what our gaggle can overcome together."
> Town says: "look what that journey lets us build, share and become."

The player should leave a town thinking: *our gaggle is better prepared, our
caravan feels more like ours, and we have people here worth coming back to.*

## Principles

### Town is not a punishment between expeditions

It turns loot into choices, relationships, expression and a renewed wish to
travel.

    arrive -> unpack -> choose what to keep and sell -> open the stall
      -> meet locals -> buy or exchange -> recruit or settle companions
      -> decorate -> prepare to go

### What makes it cosy

- **Safety.** Not that nothing can go wrong, but that ordinary experimenting
  never threatens a cherished companion or erases hard-earned progress.
- **Abundance.** Not unlimited money, but enough that arranging, helping,
  collecting and expressing can be enjoyed without fear of ruin.
- **Softness.** Not pastel colour, but timing, sound, predictability,
  manageable scope, and being able to leave an interaction comfortably.

A shop satisfies through a modest transformation with visible results:

    messy return -> understandable stock -> welcoming display
      -> pleased customer -> useful earnings -> improved outfit or home
      -> ready for another journey

Trade makes connections, not only a larger ledger.

### Design backward from the feeling

Relief (a safe, familiar place), expression (this is ours), belonging
(residents know us), discovery (a local look, pet or recruit suggests a
journey), stewardship (our surplus helps this town), gentle mastery (we
understand trade), continuity (retired companions and their families are still
here).

| Rule | Behaviour it produces | Feeling |
| --- | --- | --- |
| Favourites and equipped gear cannot be sold | Preparing a sale freely | Safety, confidence |
| Extra loot is listed in a few readable groups | Curating, not inspecting duplicates | Order, authorship, completion |
| Customers have understandable needs and remembered purchases | Sometimes saving a piece for a familiar resident | Connection, generosity |
| Decoration is reversible and untimed | Experimenting | Expression, relaxed play |
| Towns remember retired horses and milestones | Returning to check on people | Belonging, continuity |

### One shared place

The caravan is one place in three states, not three management systems:
**travelling** (packed, moving with the horses), **town market** (awning and
displays open), **camp or home** (a relaxed view for inspecting and
decorating). The same appearance carries between them.

- **Say what moves it.** Enchanted wheels, wind or sun magic, an arrangement
  for hauling: one stated rule.
- **No unseen humans.** No human chairs, no reins, no unexplained hands.
  Horses cannot stock shelves by hand; listing an item directs an enchanted
  display, and horses use mechanisms, voices or established magic.
- Start with the outside and a few decoration sockets. An interior is
  optional and later.

### The town session

1. **Arrive.** The palette, music and pace change. Someone familiar may greet
   the party. The town's name and state are shown without a report covering
   the welcome.
2. **Review.** Notable drops, coins, new looks, encounters, journal events.
   Spotlight the unusual; aggregate the ordinary.
3. **Sort.** Keep, equip or sell. Suggested sales are explainable and never
   include equipped, favourite or saved-loadout pieces. The sale is previewed.
4. **Open the stall.** Listed stock appears on a few displays; town horses
   visit and buy according to the chosen pricing policy.
5. **Explore.** Buy, meet a recruit, adopt a pet, visit a retired horse,
   choose a home for someone leaving.
6. **Personalise.** Looks, colours, decorations, pet assignments. All
   skippable without penalty.
7. **Prepare.** Party, builds, saddlebags, the next journey. What the party is
   ready for is shown without opening every menu.

**The player may leave at any point.** "Finished" never means waiting on a
hidden checklist of social interactions.

There is a **short essential path** (review, keep or equip, sell, prepare,
depart) and **optional paths** (decorate, visit, meet descendants, browse
outfits, read the journal). Voluntary engagement does not mean many unrelated
activities: offer several ways to enjoy the few systems that exist.

### Selling is curation, not labour

The player makes a few meaningful choices; the game handles the repetitive
transactions.

    return with gear -> keep what the party needs -> select the surplus
      -> set a price policy -> open the stall -> townsfolk browse and buy over time
      -> collect earnings and goodwill -> shop, recruit, retire, decorate, prepare

**The player decides:** what stays with the party, what is listed, which
price policy, whether to stay longer or go, whether to buy something local.
**The game automates:** who visits, whether their wants match the stock, the
transaction at the shown price, the records, and what happens while away.

- **Two modes.** *Open stall:* stopped in a town, stock listed, customers may
  buy over time. *Packed caravan:* the party has left, unsold stock travels
  with it, and sales in that town stop.
- That gives one travel decision: leave now with unsold stock, or stay longer
  for more chances to sell. **It is always optional and never a forced wait:
  the player can always leave at once.**
- **Listing is a batch, not shelf-stocking.** The inventory suggests
  candidates, the player selects one or many, they become listed stock, and
  the stall shows a representative group. Each item stays identifiable in the
  list and can be reclaimed before leaving.
- Safeguards on listing: equipped items cannot be listed; favourite and
  locked items are left out of a batch by default; a piece in a saved build
  warns; Rare and unusual pieces are confirmed one by one unless the player
  changes that; count and potential earnings are shown first.
- Customers arrive at a steady, unhurried rate. A visit may buy, browse and
  leave, find the price too high, or ask for something not stocked. A first
  version can use abstract visits, each producing a short line or scene.
- **No haggling as the normal loop**, no restocking clicks, no production
  chain, no rapid customer timer, no need to watch, no staff schedule.
- **Nothing is lost by leaving.** On departure the stall closes, sales and
  earnings stand, unsold stock returns to the shared cargo, and the next town
  starts with a fresh stall; old stock is carried, not automatically listed.
  No decay, spoilage or deletion.
- **Unsold stock takes cargo space.** The caravan is not a warehouse that
  bypasses saddlebags ([cargo.md](cargo.md)). A sale frees a slot.

### Prices, set by the player

Confirmed: the player sets a price as a percentage above or below an item's
baseline, and lower prices improve relationships.

- A narrow, legible range to start with, bounded above zero. Set per item,
  per category, or as a default for newly listed stock.
- Policies: fair (baseline), friendly (a modest discount), premium (a modest
  markup), custom.
- The price shows on the stall and in the list, with its effect in plain
  words: a higher price brings more coins per sale and usually fewer buyers;
  a lower one brings fewer coins and local goodwill.
- **Two separate things:** how much a customer pays if they buy (baseline
  times the player's modifier), and how likely they are to buy (interest,
  demand, how well the piece fits the local market, trust, less resistance to
  the price). The formula may stay hidden; **the factors must be shown.**
- **The dearest price is not always best, and neither is the cheapest.** The
  choice is among profit, time in town, goodwill and cargo space. Test that
  it really is a choice.

### Goodwill

- A sale may affect the buyer (if named), the caravan's reputation as fair
  or generous, and the town's wider trust.
- **Small and capped**, with diminishing returns. A cheap common item never
  buys a large relationship gain.
- In the fiction a low price is generosity: fair access to useful gear, a
  town that trusts the caravan, buyers who recommend it. **Friendship is
  never an opportunity to charge a horse more.** High trust improves
  interest; a low price is the player's own explicit generosity.
- Discounts are one path to goodwill, not the only one: helping with
  restoration, visiting retired companions and local requests are others.
  **No daily gifts, no decay, no repeated dialogue clicks.**
- Show why a sale came out as it did: baseline, the player's adjustment,
  local demand, trust, the final amount.

### Demand

A few plain demand tags to begin with: protective gear, elemental gear,
carrying gear, pet accessories, cosmetic or local pieces. A town favours one
or two. Its identity shapes interest, but **most stock must not be unsellable
outside one market**. Demand holds steady during a visit and changes only at
a town update, never when a screen is reopened.

### While away

- The stall sells while the player is away, up to a clear cap on sales or
  stock, so nobody returns to a pile of messages or unbounded coins. The cap
  limits the economy; it does not punish a long absence.
- The report is short: how long the stall was open, items sold, earnings,
  goodwill, unsold stock. Highlights only: a rare sale, a named buyer, a new
  regular, an item that did not sell at its price, a change in demand.

### Safeguards

Prevent: buying and reselling for more; repeating a cheap sale to farm
goodwill; unbounded earnings while away; one town always paying best; prices
beyond reasonable bounds; selling a favourite or a loadout piece
automatically; retiring and recalling a horse for a repeatable bonus.

Use: price bounds, values that rule out arbitrage, goodwill caps, limits on
sales while away, a price history, and **a visible explanation when an item
is not selling**.

### The shop tells stories too

Retired companions become regulars; a descendant buys gear for a first
journey; a town outfits its defenders; a generous sale helps a struggling
family; a customer comes back wearing what they bought. All of it from
recorded transactions and town state, never invented.

### Buying is discovery and preparation

- Local stock expresses the place, as a theme and not a prohibition.
- Show slot and purpose, compatibility, the change against a suitable horse's
  current gear, whether the look is new, and the price with the reason for
  any modifier.
- **Nothing vanishes while it is being read.** Stock changes only at an
  explicit town update. No timer that rewards checking every hour.
- The road stays the main source of exciting loot. Shops fill gaps and
  reveal possibilities.

### Recruiting and retiring are social moments

- **Kept apart from selling.** The shop trades equipment. It never sells a
  party member, and recruiting is an invitation or an agreement, never the
  purchase of a horse as property.
- A recruit's preview shows name, origin, class, a modest trait, appearance
  and family links. It answers "what could this companion add?" without
  reducing them to a score.
- Retirement shows a welcome and where the horse will live, and states what
  happens to its gear and pet **before** it is confirmed.
- Family news arrives as selected highlights, never a popup per event.
- Safeguards: no irreversible retirement without clear confirmation; no
  surprise loss of an attached pet or an heirloom; no penalty for not
  visiting, unless that risk was explicitly chosen.

### Decoration is expression, not another build tax

- A few sockets to begin with: awning, trim, sign or emblem, one or two
  displays, hanging ornaments, a pet nook, a small memory shelf.
- Decorations come from what the game already has: restoration gifts,
  exploration, purchases, family keepsakes, pets, memorable victories.
- **Mostly mechanically neutral.** If style affects customers at all, it is
  through gentle, visible preferences with several equally good styles, never
  a hidden "correct decor" score.
- **Function is separate and explicit:** more display room, more convenience.
  Never selling efficiency tied to a rug's colour.
- Layouts and looks are saved and survive generator updates.

### Juice for town

Feedback acknowledges the action, explains the result, and says how
important it was ([juice.md](juice.md)).

- **Arrival:** a brief slowing, the awning unfurling, warm light, soft
  ambience. Never a long animation on every visit.
- **Listing:** the piece travels to the display; stock and cargo totals
  update together. **A batch gets one summary, not thirty flashes.**
- **Sale:** the customer looks, is pleased, the piece leaves; a restrained
  chime and a receipt.
- **Purchase:** the piece arrives where it belongs, marked if its look is
  new; coins update at once; the comparison stays open.
- **Equipping:** the gear visibly changes and **the right thing is
  highlighted.** No celebrating attack when the change was cargo or
  targeting.
- **Decorating:** a ghost preview snaps to a valid socket; undo is instant;
  an invalid placement is explained gently.
- **Recruiting:** the newcomer walks up, is marked as party, greets.
- **Retiring:** a welcome, the horse moving into the town scene, a journal
  note. **No coin sound: it must not feel like a sale.**
- **Assigning a pet:** it goes to its horse and settles.

Routine coins are quiet. A recruit, a restoration gift, a family reunion gets
attention. Settings for reduced motion, quieter repeated sounds and shorter
transaction animations.

### Interface

- A small set of places: Party, Trade, Wardrobe, Town. Trade has two clear
  tabs, selling and buying.
- **Sale, retirement and recruitment are never in one undifferentiated
  list.** Their consequences are too different.
- One pattern everywhere: select, preview, explicit action, visible result.
- Protection from mistakes: locked favourites cannot be bulk-sold; a sale
  preview shows count and proceeds; recently sold gear can be bought back;
  significant purchases confirm their cost; retirement explains destination,
  recall, pets and gear.
- "New", "equipped", "favourite", "look owned" and "listed for sale" each
  have their own indicator, in text or icon as well as colour.

### State

- The caravan's state (inventory, listed stock, money, layout, decorations)
  is separate from a town's (population, relationships, demand, stock,
  retired horses, families).
- Towns update on a visit or from the map, from a checkpoint, with a bounded
  catch-up. **Opening the map again never rerolls stock, adds children or
  creates free trades**: change depends on elapsed world time and recorded
  events, not on how often a screen is opened.
- When the caravan leaves, selling stops. Residents do not secretly run the
  player's stock.

### Pacing

Town is the safe contrast to the road. Threat lives on the road; town is
where consequences are understood and relationships deepen.

**Not in a first version:** rent, compulsory repairs, customer anger, daily
attendance, haggling, supply chains, staff, an elaborate interior. Challenge
comes from choosing stock, budgeting a purchase and preparing a build, never
from fear of bankruptcy.

### Questions to test with

Can players tell their inventory from the shop's? Sell ordinary gear without
losing a favourite? Understand a quoted price? See what a purchase changes?
Does decorating feel optional? Does retirement feel like settling, not
disposal? Do customers feel like residents, not cash machines? Can a short
visit finish without waiting on animations? Is there a clear reason to travel
again? Do players return for people and discoveries, not to avoid a penalty?

Measure actions per ordinary sale batch, time to find a useful purchase,
accidental sales, abandoned comparisons, and whether optional features are
revisited by choice.

## For Horse Heroes

- The slice's camp is a 12-second pause with a line of text. Everything here
  replaces it. The short essential path is what keeps idle-first true: with
  no input the party reviews, equips, sells by its rules, and leaves.
- The vision warns against supporting features becoming independent
  management games. The caravan is the main risk of that. Its own guards are
  the right ones: one shared place, automatic displays, no timers, a
  smallest slice first, and nothing built that delays a decision.
- Idle-first and "the player may leave at any point" are the same rule seen
  from two sides. A town that needs attention to function breaks both.
- The review step is the loot report already filed; the spotlight there is
  the fountain's last beat ([loot-fountain.md](loot-fountain.md)). Town
  should not open with a second, separate summary.
- "Selling stops when the caravan leaves" sits beside the confirmed line that
  horses left in towns generate resources. They are compatible only if a
  resident's yield is something other than the player's stock being sold.
- **Selling over time pulls against idle-first.** Pillar 2 says the game
  progresses with no input; a stall that sells while parked rewards staying
  put, and the road is where loot comes from. The game needs a stated default
  for when the party leaves by itself (when the stock is sold, after a set
  time, when a cap is reached), and it must not make sitting in town the
  best way to play. This is unresolved and is filed as a question for the
  owner.
- The stall's report measures time ("open 3 hours 20 minutes"), which
  assumes real time away counts. That is the same open question as offline
  trails and town updates, and should have one answer.
- An earlier version of this file recommended a fair price with no
  item-by-item pricing. Her confirmed direction is that the player sets the
  percentage; the fair price survives as the default policy.
- The document treats a screen layout as established: the top third a fixed
  3D view, the lower two-thirds menus. Four of the owner's documents refer to
  it; the one that decided it has not been received. It is filed to be
  confirmed, not assumed.

## Source

A design proposal for the non-combat half of this game, received from the
owner 2026-10-09 (all sections), and her later document on the caravan shop,
same date, which confirms the caravan and replaces the earlier sections on
selling and prices. Both drew on other shopkeeping games for tone; only the
principles are kept here.
