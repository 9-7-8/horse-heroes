# Retirement and generations

What happens to a horse who stops adventuring, how towns live while the party
is away, and how the world ends. The idea is the owner's proposal; the details
below are recommendations developed from it unless marked otherwise, and the
open ones are `decision` issues. See [vision.md](vision.md) for what is
confirmed.

> The horses who once fought beside you do not disappear when their adventure
> ends. They become part of the restored world, and the next generation may
> carry the journey onward.

## Confirmed by the owner

From her second document on retirement and from what she said when handing it
over (2026-10-09):

1. **A retired horse can be invited back** to the party.
2. **Its gear and its assigned pets go with it and come back with it.**
   Nothing is lost, sold or silently reassigned.
3. **Resident horses help build towns with telekinetic horse magic.** That is
   how horses lift, carry, place and assemble without hands.
4. Towns update when visited or when the world map is opened.
5. **Breeding has genetics, and they are superficial: colour only, for now.**
   The system is borrowed from the horse breeding mod she has already made.
6. **Breeding is background flavour, not an important mechanic.** Everything
   it affects is aesthetic.

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
- It is **reversible without being meaningless**. Visiting its town, the
  player invites the horse back, and it rejoins with everything it had. An
  invitation is generally accepted; a refusal belongs to an authored story
  event, never to a hidden roll that locks away a favourite. A horse who
  rejoins keeps its connection to the town, and the town keeps its history
  and anything permanent it gained.
- **The horse retires as a complete package**: itself, its gear, its pet.
  Before confirming, the game says which town, what happens to the gear and
  the pet, and how to invite the horse back.
- **No bonus for leaving and returning.** Permanent milestones belong to the
  town's history and are never awarded twice.
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
- **Residents build with telekinesis** (confirmed). It is a visible
  expression of horse magic and community, not manual labour in the human
  sense: one horse lifts a beam while another aligns it; several move stone
  together; a group holds a structure steady while it is mended. Bridges,
  shelters, market stalls, solar gardens, water systems, protective barriers.
- **Residents are not factories.** Do not have each resident produce a
  resource every hour. Residents contribute to a small number of legible town
  **projects**, and in a first version to one thing: construction progress.
  "Fern and three residents are rebuilding the western footbridge: 6 of 10"
  is more tangible than "+2.3 town production".
- **Other inhabitants do what horses do not**: farming, weaving,
  metalworking, cooking, trade, teaching. Horse magic helps the community
  build; it does not replace every occupation.
- **Keep each benefit distinct.** Residents build and defend. Other
  inhabitants provide services. Reputation is trust, and unlocks invitations,
  welcome, recruits and local looks; it is not also construction speed,
  population, shop prices and combat power. Town health is overall condition
  and lets families grow. Companionship is greetings, visits and stories, and
  need not produce a number at all.
- Growth comes in a few stages, each of which unlocks something: more
  residents, a little more capacity, services or visitors, more varied horse
  and pet looks, recruits, new events, a landmark or celebration, a safer
  route to another city.
- **A small state model**, to begin with: the town's condition (the later
  document uses three steps, Struggling, Recovering, Flourishing, once a
  place is no longer corrupted); its capacity; its roster of retired
  adventurers, local horses and notable inhabitants; its community progress;
  and a few meaningful relationships and family records, not a population
  database.
- **Population is capped and summarised.** Capacity comes from condition and
  built housing. Only a few children are individually modelled; other
  residents are counts or background groups. When a town is full, growth
  pauses gently: nobody is evicted or harmed, and the game says what would
  let the town grow.

### Families

**Owner's rule:** healthier towns have more children and grow more.

- The next generation grows from a healthy, safe community, not from a
  character generator disconnected from the world.
- **Not a demographic simulator.** A few coarse life stages; growth bounded by
  health, capacity and pacing.
- **Children are characters, never production units.** Population must not
  become a resource exploit.
- **Inheritance is genetic and aesthetic only** (owner, 2026-10-09). A foal's
  coat colour comes from its parents through a real, if superficial, genetics
  system, taken from the horse breeding mod the owner has already built.
  Colour only, for now. **Nothing a horse inherits affects how it fights.**
- That overrides the two documents' suggestions that a descendant might also
  inherit a temperament that nudges behaviour, a signature trait, or a
  magical affinity. Those would be gameplay inheritance, and she has ruled
  that out. A family name, an origin and a keepsake are fine: they are
  flavour.
- A child is **not a copy and not an optimal recruit**. Class, attunement,
  trait and gear are its own.
- **Breeding is background flavour.** The player never needs to arrange a
  pairing or raise a foal to have a viable party, and no system should push
  them to optimise it.
- A few coarse life stages (young foal, growing youngster, adult), advanced
  by play: a stage after a small number of major expeditions, and the whole
  path from birth to adulthood within a campaign arc. **No real-time timer**,
  and nothing that punishes being away.
- The roster shows family: "Pip, child of retired adventurers Bramble and
  Sol."
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
- **Recommended time source: game-world time tied to play**, such as one
  completed expedition or a set of travel days, not the real clock. Then no
  one returns to find that years of family life passed while they were away,
  and the simulation is the same on every machine. If real time is ever used:
  cap the catch-up, make clock changes harmless, keep ageing bounded, and
  explain the rule.

### A world that can be completed

    Cleanse every region of the dragon's greed and corruption.

- **Completion is defined by the world, not a boss's health bar.** A region
  is cleared when its corruption source is removed, its town restored and
  its routes secured. The rule and the ending are in
  [endgame.md](endgame.md).
- Progress is visible on a world map.
- The ending shows the world recovering and the retired horses and their
  families reacting, and records the current generation's journey.
- **The ending never requires collecting everything.** Optional collection
  stays optional.

### After the ending (revised)

Both retirement documents proposed an optional endless frontier of new
threats after the campaign. **The owner's endgame rule replaces that:**
success makes combat unnecessary, and there is no combat after the dragon.
A completed save stays open for peaceful play, and an open-ended peaceful
mode is a possible later expansion that nothing depends on. What survives
from the earlier advice is its first point: nothing may make finishing the
campaign feel pointless or withhold the ending.

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
- **The guardrail on breeding has moved.** The design summary said to avoid
  breeding until the main loop works. The owner has since said breeding is in:
  as background flavour, with colour genetics, aesthetic only. What survives
  of the guardrail is the part about depth: it is still not built ahead of
  the loop, it still starts as the bounded prototype, and it never becomes a
  system the player must work.
- **The genetics are borrowed, not designed here.** The source is the owner's
  own mod. Its folder is one this project's rules say not to touch, so
  reading it to port the system needs her say-so first. Until then the horse
  generator should keep coat colour as its own small piece of data, so that a
  genotype can replace the rolled palette without disturbing anything else.
- Colour genetics give the horse generator
  ([fairyland-content.md](fairyland-content.md)) a second way to produce a
  coat: rolled for a horse with no recorded parents, inherited for one with.
  Both must end in the same saved, resolved appearance.
- Telekinesis answers more than building. It is how a horse can work a market
  stall, hang a decoration or dress another horse, which the town and
  wardrobe documents each had to step around.
- The confirmed line that horses left in towns "generate resources" is now
  best read through the recommended model: what a resident yields is progress
  on a town project, explained, bounded, and told as something that horse
  did.
- The two time sources in her documents disagree. This one recommends world
  time tied to play; the caravan shop reports hours the stall was open. One
  rule has to cover both.
- The sim is deterministic and has no rendering, so a town update is a pure
  function of the town's saved state and the elapsed time. It can be tested
  by running it across long gaps and checking the caps.
- Idle-first (pillar 2) applies to towns too: a town never waits on the
  player, and nothing in it decays as a punishment for being away.
- It all depends on things that do not exist yet: saving, towns, a roster,
  persistent horse identity.

## Source

Owner's proposal on retirement, town life and a multigenerational world, as
developed in a design document received 2026-10-09 (all sections); a second
document on retirement, residents and generations, same date, which carries
her confirmed points; and her own statement on breeding and genetics when
handing the second one over.
