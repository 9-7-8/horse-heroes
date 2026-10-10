# The loot fountain

The moment a significant enemy dies and releases a visible burst of rewards.
It is not a payout from a random table. It is a **reward ceremony**: it turns
abstract probabilities into an event with a beginning, a climax and an
aftermath. This is the product (pillar 1).

How to tune it, parameter by parameter: `procedures/tune-loot-feel.txt`.

## Principles

### The seven beats

A fountain needs all of them, in order.

1. **Anticipation.** The player can see a payoff coming: a notable target, a
   health bar, an objective nearly done.
2. **Escalation.** The fight intensifies; cooldowns are spent, health swings.
3. **Resolution.** The target dies with a loud, large, readable death.
4. **Eruption.** Rewards of several kinds spray outward. Rarity colour says at
   once that something special may have happened.
5. **Collection.** Drops are gathered, counters climb, pickups tick, the
   ground clears.
6. **Appraisal.** The drop is compared, equipped, or scrapped.
7. **Recommitment.** The new item suggests the next thing to do: try it, push
   higher, chase the missing piece.

### What makes it land

- **Spread.** Items scatter over the ground. Physical abundance reads as more
  than one line in a menu.
- **Rarity signal.** Colour gives a first judgment before anything is opened.
  The top tier must create an instant "look at that".
- **Sound hierarchy.** Each drop and pickup is a small confirmation. A special
  drop never sounds like scrap.
- **Persistence.** Loot stays in the world long enough to be looked at. It is
  owned, not instantly turned into a counter.
- **Contrast.** The burst follows danger and noise. The sudden calm after the
  kill is what makes it legible, so leave that calm in.
- **Variable reward.** Most items are ordinary; occasionally one changes
  everything. Uncertain timing sustains anticipation.

### Scarcity and restraint

- **Reserve the big fountain** for real completion points: named enemies, the
  end of a stage, rare events, milestones. If every kill showers, the player
  goes numb.
- **Ordinary drops are quieter and more compact.**
- **Three-part presentation for anything important:** spawn motion, rarity
  signal, pickup confirmation.
- **Delay the reward just enough to be a release.** An instant invisible grant
  is efficient and emotionally flat; too long a delay reads as lag.
- **Keep a clear hierarchy:** common materials, then uncommon gear, then rare
  gear, then the build-changing reward.
- **Loot never hides what matters.** Enemies, hazards and exits stay readable
  through the pile.

### Aggregate the ordinary, spotlight the meaningful

Do not present eighty useless drops one by one. That is satisfying once and
tedious by the hundredth run. This matters most for a report on a run nobody
watched:

- currency and materials resolve first;
- ordinary items collapse into a summary;
- high-rarity items appear one at a time;
- the top tier gets a stronger reveal of its own;
- each revealed item can be acted on there and then.

The report is a controlled fountain, with the same beats as the one in the
world.

### Relevance

The ceremony carries emotional value even when most items are junk, but
ceremony alone does not last.

> Random rewards need enough relevance to preserve hope.

- A reward system needs both **surprise** (the next item cannot be predicted)
  and **plausibility** (the next item could really matter).
- Too many obviously useless outcomes turn the fountain into a deceptive
  slot-machine show. Players stop reading the colours because they expect
  disappointment.
- Cut clearly useless stat combinations. Bias drops toward what the player can
  use, without making them predictable.
- **Some share of rewards must change play, not just statistics**: range,
  targeting, projectile count, summon behaviour, damage type, defensive
  rhythm. Those are the drops players remember.

## For Horse Heroes

- Today a trail yields one gear drop, reported as text in camp. Beats 3 to 5
  do not exist yet and beats 6 and 7 are thin. That is the gap to close first.
- The ceremony has to run unattended (pillar 2): pickup is a vacuum toward the
  party, not a click. The pause before the vacuum is the "persistence" beat,
  so it must be long enough to read the colours.
- Reserve the full fountain for the trail's final enemy. Ordinary encounters
  get a small, quick drop. A fountain needs several kinds of reward to have
  spread, so the sim has to drop more than one gear piece per run.
- Every gear piece fits one named horse and the party has all three, so a drop
  is never for a class the player does not have. Relevance here is about the
  slot and the stats: watch the share of drops that are a plausible upgrade.
- Appraisal happens in camp and auto-equip exists. Auto-equip must not swallow
  the moment: show what changed.
- All of it is data (pillar 4): rarity tiers, weights, colours, fountain sizes.
- Huge drops are a confirmed goal ([vision.md](vision.md)), and so is low
  management. The owner's rule settles how: the party keeps the best cargo
  its rules allow and carries on, so a fountain never forces an inventory
  interruption ([cargo.md](cargo.md)). What flies to the party is what was
  kept; what stays on the ground was left. Do not hide the rejected loot so
  well that a victory looks empty.
- Rarity is shown several ways, never by colour alone, and its colours must
  not be confusable with the element colours.

## Source

Owner's notes on loot fountains and combat feedback in Diablo III,
received 2026-10-09 (section 2). The part on aggregating and spotlighting is
from her notes on carrying those systems over to an idle game, same date
(section 2.1). What makes an item worth dropping is in
[items-and-builds.md](items-and-builds.md).
