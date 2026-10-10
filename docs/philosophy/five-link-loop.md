# The five-link core loop

The smallest version of the whole game: five links, one rule each, and a test
for each. **Build this first.** It consolidates the owner's established
decisions into something that can be played from end to end, and needs no
further content systems.

One thing in it is not yet confirmed by her: the elemental Primer and
Finisher rule is adopted here as a **recommended prototype rule**.

    choose route -> configure party -> automatic expedition
      -> resolve loot and capacity -> town trade and roster decisions

## The three rules to implement first

| | Rule |
| --- | --- |
| **Trip end** | Arrival after the final boss of an uncleared route, or a safe retreat on defeat. Restored routes have no combat. A failure never relaunches by itself. |
| **Full bags** | Keep travelling. Reject the new drop or swap out a carried one, by shared rules the player sets, rarity first by default. |
| **Reaction trigger** | A designated Finisher consumes a designated Primer once. Different elements give a pair reaction; matching elements have their own payoff. Secondary effects never recurse. |

## The five links

### 1. Choose route

**Rule.** The player selects a destination city on the finite generated world
map. The path's distance determines travel days, and the purity of the
surrounding tiles determines road danger, encounter frequency and the final
boss's strength.

**First test build.** Show one proposed path, its duration, its danger and a
boss warning. Horses advance a limited distance each game-world day. Quiet
travel is summarised. No manual navigation and no competing route policies
yet.

**Exception.** An uncleared journey ends with a boss. A fully restored route
has no monsters and no boss. **Permanent peace supersedes "every trip has a
boss".**

**Test.** Pick two cities at different distances and through different
purity. The preview matches the actual duration and danger. A restored route
stays safe and does not spawn a boss merely because a journey happened.

### 2. Configure party

**Rule.** Before departure the player chooses the travelling horses, each
horse's gear, its assigned pet, and the loot-retention rules. Combat then
runs with no manual commands.

**First test build.** Three active horses, the existing class roles,
individual elemental contributions, and one saved party configuration.
**Three is a prototype size, not a confirmed cap.** Equipment and capacity
are settled before leaving. The slot mapping is the owner's
([gear.md](gear.md)); not every slot's advanced effects are needed for this
test.

**Test.** Change one horse's elemental skill or weapon and run the same
encounter seed: its cooperation opportunities visibly change. Reassign a pet:
it accompanies exactly its assigned horse, with no duplicate.

### 3. Automatic expedition

**Rule.** The party advances and resolves scheduled road events by itself
until it wins the destination encounter and arrives, or exhausts its usable
health and potions and retreats to the city it left.

**What ends a trip.**

- **Success:** the final boss of an uncleared journey is defeated; the party
  arrives.
- **Safe travel:** the destination is reached without combat on restored
  routes.
- **Failure:** no horse can continue and no usable recovery can restore one;
  the party returns safely to the departure city.

No horse or pet dies. On failure the configured loot loss and the modest
charge are applied once. A failed route is never retried automatically. One
exhausted horse need not end a fight the others can win. **For the smallest
slice a successful trip also ends in town, with no automatic next
departure.**

**Elemental trigger (recommended prototype).** Only designated Primer skills
apply a temporary Fire, Ice or Plant setup. Only designated Finisher skills
consume it. A different-element Finisher gives a pair reaction; a
same-element one gives a same-element payoff. Basic attacks, damage ticks and
the reaction itself never trigger a reaction. The setup is consumed
atomically before the payoff. The triangle stays, and an enemy's defensive
element and a temporary setup are separate values.

For the first test, only **Plant Primer and Fire Finisher, giving
Wildbloom.** The other pairs and the same-element payoffs follow once the
trigger rule works. Names, exact effects, durations and scaling are tuning.

**Test.**

- A Primer followed by an eligible Finisher gives exactly one payoff.
- An ordinary hit of another element does not consume it.
- Two simultaneous Finishers cannot consume it twice.
- A reaction's splash cannot start a chain.
- A defeat with no potions returns the party once, charges once, and leaves
  it waiting for a manual departure, even after a long time away.

### 4. Resolve loot and capacity

**Rule.** All loot enters one shared party cargo pool. When it is full, the
retention rules keep the higher-priority item, by rejecting the new drop or
replacing the lowest-priority eligible carried item.

**When bags fill.** Travel continues. Capacity never triggers a return,
skips the boss or sells loot. Default priority is highest rarity; on a tie
the carried item stays. Favourites and protected pieces are never
candidates. Equipped gear is separate from cargo. Saddlebags add to the
shared total, and nothing is shuffled between horses. Rare drops get their
own colour, look and stronger presentation. Routine rejections are
summarised and never interrupt a fight.

**Test.** Fill cargo with Common and Uncommon pieces, then award a Rare: it
replaces the lowest-priority unprotected item. A Common drop is rejected.
Protected items remain. The party keeps moving. Watched and unwatched runs
keep the same cargo under the same rules and seed.

### 5. Town trade and roster decisions

**Rule.** In town the player lists surplus cargo at a chosen percentage of
its baseline price and lets residents buy over time, while buying gear,
changing builds, recruiting, retiring, assigning pets, and choosing when to
depart.

Transactions run by themselves while the stall is open. Discounts improve
local relationships. Unsold stock travels with the party and still takes
shared capacity; no hidden shop storage bypasses the saddlebags.

Retirement returns all gear and pets to the party at once. The horse stays a
town resident and can be invited back, when it needs gear and a pet again;
nothing is duplicated. Residents use telekinetic horse magic to help build
and restore their community.

**For this slice departure is manual.** Town generation, families,
decoration and relationship stories come after trade and roster changes
work.

**Test.** List an item below baseline: a simulated customer can buy it over
time, the proceeds match the displayed price, and the intended goodwill is
recorded. Leave before another item sells: it is still in cargo. Retire a
horse: its gear and pet are available to the party. Invite it back without
duplicating either.

## The minimum integrated test

- Two cities and a short seeded route.
- Three horses with basic automatic skills.
- A Plant Primer, a Fire Finisher and one support skill.
- One ordinary fight and a final boss.
- Shared capacity small enough to test replacement.
- Common, Uncommon and Rare equipment.
- A potion supply and a safe return on defeat.
- A town stall with a price percentage and one automatic customer event.
- One retirement and recall.

**Run the same slice to victory and to defeat.** Both must return a clear
result, keep every companion, respect the cargo rules, and end with
understandable choices in town.

## World completion

When every route and region is safe and the dragon's threat is ended,
monsters and bosses stop appearing. Saving the finite world ends the combat
loop and, in effect, the game. Infinite trading or frontier modes are out of
scope. **Do not bring danger back to keep this loop running after success.**

## For Horse Heroes

- This replaces the several overlapping "next slice" lists that earlier
  documents each suggested (two towns, gear, shop, travel, generations).
  They remain as later widenings; this is the one to build.
- **Against the slice as it stands:** there is no map, no choice of
  destination, no shared cargo, no potions, no retreat, no town. There are
  three horses with automatic skills, a seeded run ending in a final enemy,
  three rarities, and a deterministic sim with tests. The five links can be
  grown from that without throwing it away.
- **Two places where this slice sets idle-first aside**: departure from town
  is manual after a success as well as after a failure. After a failure that
  is the owner's rule. After a success it is a simplification "for the
  smallest slice", and the automatic departure that pillar 2 expects still
  has to be designed, together with the open question of when a party with a
  stall open should leave by itself.
- **Where the loot shower fits.** Pillar 1 says the shower is the product.
  This loop is the skeleton that gives a shower its reasons: a boss to burst
  from, a bag to fill, a town to carry it to. Link 4's "stronger
  presentation for Rare drops" is where the shower lives, and it should not
  be left as a line of text while the other four links are built.
- Every test here is a sim test: seeded, with no rendering. That is the
  existing practice in `src/sim/`, and the watched-and-unwatched checks need
  nothing more.
- The loop's shape is the one in [the-loop.md](the-loop.md) made concrete:
  intention (route), build (party), automated test (expedition), reward
  (loot), and the next hypothesis (town).

## Source

Owner's minimal rules-and-tests specification for the core loop, received
2026-10-09 (all sections). It consolidates her earlier decisions on travel,
gear, failure, cargo, the caravan shop, retirement and the ending.
