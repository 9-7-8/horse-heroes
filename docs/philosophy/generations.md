# Retirement and generations

What happens to a horse who stops adventuring, how towns live while the party
is away, and how the world ends. The idea is the owner's proposal; the details
below are recommendations developed from it unless marked otherwise, and the
open ones are `decision` issues. See [vision.md](vision.md) for what is
confirmed.

> The horses who once fought beside you do not disappear when their adventure
> ends. They become part of the restored world, and the next generation may
> carry the journey onward.

## Principles

### Retirement turns party overflow into world continuity

Unwanted party members are never discarded. A horse retires into a town and
becomes part of the world the player is restoring. The party is not a
replaceable roster; it is one generation in a longer effort.

    recruit and travel -> fight corruption -> restore towns
      -> retire adventurers into those towns -> relationships and families
      -> recruit a new generation -> carry the struggle into new regions

**Retirement is a transition, not deletion and not a storage slot.** The
retired horse keeps its name, appearance, history and accomplishments, is
visible in its town, counts in its population, and can be met on a visit.

- It should read as **the horse choosing a different life**, not the player
  firing a unit.
- It should be **reversible without being meaningless**: the horse can be
  asked to rejoin, if willing and if the rules allow.
- It must **never be mandatory maintenance**. It is a valuable choice, not a
  pressure to keep removing horses.
- It must **not dominate**. If town benefits are too strong, players retire
  horses early because they feel forced to. Active play and town life are
  valuable in different ways.

### Towns

- A town has several qualities (safety, health, prosperity, hope or
  reputation, population, corruption). **They must not become six meters with
  six unrelated chores.** Present them as a few clear states:

      Corrupted -> Struggling -> Recovering -> Flourishing

- A town improves through meaningful actions: defeating a nearby threat,
  clearing a route, helping residents, bringing back resources, protecting a
  gathering, making it a safe place to settle.
- **Residents are not magical factories.** What a retired horse gives a town
  is community: companionship, patrols, training, knowledge, trade
  connections, magical help, a population recovering.
- **Preserve world logic.** No human-style work for horses by default. If a
  town has food or crafted goods, the world says how: other inhabitants,
  adapted tools, magic, trade, restored ecosystems.
- Growth comes in a few stages, each of which unlocks something: more
  residents, a little more capacity, services or visitors, more varied horse
  and pet looks, recruits, new events, a landmark or celebration.

### Families

**Owner's rule:** healthier towns have more children and grow more.

- The next generation grows from a healthy, safe community, not from a
  character generator disconnected from the world.
- **Not a demographic simulator.** A few coarse life stages; growth bounded by
  health, capacity and pacing.
- **Children are characters, never production units.** Population must not
  become a resource exploit.
- A child is **not a copy and not an optimal recruit**. It may inherit a few
  recognisable things (coat or marking tendencies, temperament, a family
  name or origin, a resemblance, a small chance of a parent's signature
  trait). It does not inherit exact class, equipment, combat role or build.
- **No genetics system needed.** A few inherited tags; the rest is generated
  from the town and the world.
- **Recruiting a descendant is the player's choice**, and never compulsory.
  A grown child may join, stay as a resident, become available later, or
  help the town another way. No family produces a stream of recruits.

### Relationships and consent

Partnership and family are relationships between characters, not a
population button.

- Horses in a healthy town form friendships and partnerships over time.
- **The player never forces two characters together.** The player shapes
  conditions (safe towns, time, community, opportunity) and can see the
  relationships that result.
- A horse's decision to settle or to partner is part of that horse's story.
- Decide the tone and how much is shown before building any family scene.

### Simulate on demand

> A town advances when the player visits it or opens the world map.

Each town keeps a checkpoint. When activated, the game works out the elapsed
time and applies a **bounded summary** of what would have happened:

1. elapsed world time since the checkpoint;
2. changes from completed quests and restored routes;
3. resident and family milestones;
4. partnership or recruitment events, if eligible;
5. population growth, within capacity;
6. resources and reputation, within caps;
7. threat and corruption, from the town's safety;
8. noteworthy events, recorded for the journal;
9. the new checkpoint.

- Do not simulate each day where a summary or a schedule will do. Coarse
  steps (weeks, seasons) are easier to explain.
- **Cap the catch-up.** Nobody returns after a year to hundreds of children,
  overflowing stores, or a town in a state they could not have predicted.
- Updating a town is updating data. Its scene is built only when visited.
- If real time is used, handle clock changes and manipulation deliberately.
  The alternative is that world time advances only through play.

### A world that can be completed

    Cleanse every region of the dragon's greed and corruption.

- **Completion is defined by the world, not a boss's health bar.** A region
  is cleared when its major threat is defeated, its corruption source removed
  or contained, its routes and settlements safe, its restoration done.
- Progress is visible on a world map.
- The ending shows the world recovering and the retired horses and their
  families reacting, records the current generation's journey, and offers a
  clear way to continue.
- **The ending never requires collecting everything.** Optional collection
  stays optional.

### Endless play is optional

- It must not make finishing the campaign feel pointless, and it must not be
  an excuse to withhold an ending.
- The dragon's threat is resolved in the campaign. Endless play is about new
  local threats and restoration, not the same villain returning unexplained.
- "Endless" must not mean each region is a bigger number. Rotate a
  manageable set of enemy, biome and encounter rules and let the combinations
  supply the variety.

### Stories come from state

A story such as "the horse who once helped cleanse the spring now watches
their child set out" is generated from persistent state. It is never a
hand-written quest, and the journal never claims a horse was somewhere it was
not.

## For Horse Heroes

- This answers an open question from the vision: what makes a new recruit
  valuable without replacing a favourite. The favourite is not replaced; it
  settles, and stays in the world.
- It also gives the long game a shape that is not a reset. Where the earlier
  notes on incremental games ask what a "prestige" would mean
  ([incremental-and-idle.md](incremental-and-idle.md)), a new generation
  setting out is the answer this game already has.
- **It sits against a guardrail.** The vision says to avoid breeding until
  the main loop works. This proposal is not a breeding system (no genetics, no
  player-arranged pairs, children are characters), and its own advice is to
  prototype it very small and test whether players care about the connection.
  Keep both: nothing here is built ahead of the loop, and when it is built it
  is the bounded prototype first.
- The confirmed line that horses left in towns "generate resources" and the
  recommendation here that residents are not factories have to be reconciled
  when residency is designed. The common ground is that whatever a resident
  yields is explained, bounded, and told as something that horse did.
- The sim is deterministic and has no rendering, so a town update is a pure
  function of the town's saved state and the elapsed time. It can be tested
  by running it across long gaps and checking the caps.
- Idle-first (pillar 2) applies to towns too: a town never waits on the
  player, and nothing in it decays as a punishment for being away.
- It all depends on things that do not exist yet: saving, towns, a roster,
  persistent horse identity.

## Source

Owner's proposal on retirement, town life and a multigenerational world, as
developed in a design document received 2026-10-09 (all sections).
