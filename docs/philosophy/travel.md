# Travel and road encounters

> **Revised by the owner, 2026-10-10.** Her world design is in
> [vision.md](vision.md), "The generated world": a RimWorld-like hex map on
> one landmass, towns joined by shortest roads, off-road travel that is
> slower and more dangerous, and danger from comparing a hex's purity score
> with its corruption score. Corruption is cleared only by liberating a town
> through an idle task, not by a boss victory. Where this file says a single
> purity value sets danger, or that defeating a boss raises purity, the
> vision is the newer statement.

How the party gets from one town to the next. Eight rules here are the
owner's, established in discussion and listed first. Everything after them is
a proposal built on those rules; numbers are examples to test, and the open
points are a `decision` issue.

    choose a city -> inspect distance, purity and danger -> prepare the gaggle
      -> travel a bounded number of days -> varied road events
      -> fight the destination boss -> earn loot and restore the region
      -> arrive in town and decide again

## Established by the owner

1. The world is procedurally generated **and finite**.
2. The player **chooses a destination city**.
3. Travel **distance determines the number of travel days**.
4. Horses can travel only **a limited distance per day**.
5. **Every journey ends with a boss encounter.** (Her later endgame rule
   refines this: routes in a fully cleared region have no encounters and no
   boss. See [endgame.md](endgame.md).)
6. More dangerous areas produce **more encounters, stronger encounters and
   stronger bosses**.
7. Local danger depends on the **purity of nearby world tiles**.
8. Towns and regions **update when visited or when the world map is opened**,
   not continuously.

## Principles

### A journey is an expedition, not a progress bar

It has a clear beginning, a changing middle and a guaranteed climax. The
player should understand where the party is going and how long it may take,
see why a route is more or less dangerous, prepare for likely threats, get a
few memorable incidents, reach a boss, receive a reward that makes arrival
mean something, and then have a safe place to rest and decide.

Choices come from the build and the route. Nobody simulates each step by hand
or fights every minor enemy.

### What to take from elsewhere

- **From world-map travel games:** a finite set of connected tiles; visible
  destinations and regions; duration derived from distance and conditions;
  persistent position; a preview of duration and risk before leaving. Not the
  survival pressure: no caravan needs.
- **From tabletop journey guidance:** summarise quiet stretches; make a
  limited number of checks; include discoveries, travellers, landmarks and
  hazards, not only fights. **Do not make every pause an ambush, or players
  come to dread the road.** Use relative danger as an idea, with this game's
  own tested power model, not imported tables.
- **From bounded action-RPG runs:** a clear objective, building pressure, a
  guaranteed guardian and a payout; a reason to repeat that is not identical
  each time.
- **From mission-based games:** state the purpose and the reward focus before
  starting; vary within a recognisable structure.
- **From readable action combat:** danger is communicated before it causes a
  failure; bosses have distinguishable behaviours; a stronger encounter is
  more than a larger health bar.
- **From long-running idle games:** show what the party is doing and what it
  can gain; let a goal be chosen and left running; make the return report
  explain progress. A journey is never a black box that emits loot.

### The map

- A finite tile map. A tile holds only what routing, danger, generation and
  restoration need: a stable ID, position and neighbours, terrain, **purity**
  (a bounded value), threat tags, travel cost, known or visited, what stands
  on it, its seed and generator version, and a checkpoint where needed.
- **Generate once and save.** The world is never regenerated on load.
- Generation is constrained so a campaign cannot be impossible: a connected
  map, cities and landmarks placed, terrain and costs, initial purity and
  corruption, regional danger, routes, then a check that every city and the
  main campaign can be reached.
- Finite means the player can eventually restore all of it.

### Distance and days

- The route preview shows distance, estimated days, average danger, the
  expected number of ordinary encounters, the destination boss's danger, and
  any known landmarks.
- Start with a fixed daily distance and a cost per tile. **No stamina, food,
  fatigue or injury until travel itself is fun.**
- A one-day journey is still a journey. A long one is not watched day by day:
  quiet days pass quickly and are summarised.
- One party pace by a simple group rule. A weak recruit must not cripple the
  party unless travel specialisation is deliberate and tested.

### Routes

- Offer a small number of real options: safest (longer, through purer tiles),
  fastest, balanced, perhaps scouting first, perhaps a path chosen by hand.
- **No cosmetic route choices.** A route changes time, danger, known rewards
  or terrain.

### Purity and danger

Purity measures a place's recovery from the dragon's greed and corruption.
Danger comes from the purity of the tile and its neighbours, weighted so that
nearer tiles matter more, plus any threat site, less any protection or
clearance, within fixed bounds.

- A city on one pure tile surrounded by corruption is not safe.
- A cleared path through a dangerous region lowers the danger of that path.
- **Show the cause.** The player never has to reverse-engineer a hidden
  formula to understand why a journey is hard.

### How many encounters

- The count comes from danger and distance together, **within a known upper
  bound**, so the duration can be estimated. Danger adds excitement, never an
  unpredictable grind.
- A table is enough to begin with: a short safe route has none or one and an
  easy guardian; a long or dangerous one has several and a strong boss.
- **The boss is always last, even on a short safe trip**, where it is a
  modest local guardian. It belongs to the destination's identity. It must
  never feel like a monster inserted to satisfy a rule.

### The road, day by day

1. Prepare: route, days, danger, encounter budget, shown before leaving.
2. Advance a day along the route.
3. Make a seeded daily check, weighted by danger. It may give quiet travel, a
   discovery, travellers or a request, a helpful find, a hazard, or a fight.
   **Not every check that comes up is a fight.**
4. Resolve or summarise. Low-stakes events are reported briefly; significant
   ones are shown.
5. If not there yet, the next day. If there, the boss.
6. Arrive: a summary of time, notable events, loot, harm, and the boss.

The player sees an event and a consequence, never the dice.

A small vocabulary of events: **combat**, **hazard** (answered by a trait,
gear, a pet's ability or a simple choice), **discovery**, **social**, and
**rest or camp** (a quiet scene with no mini-game). Which is eligible depends
on route and biome, purity, the party, campaign state, and recent history, so
the same scene does not repeat. Authored campaign beats override the random
choice when needed.

### Bosses

- Boss power comes from several inputs, not one hidden number: regional
  corruption, route danger, the destination's threat rank, campaign progress,
  the boss's archetype, an optional elemental or environmental modifier.
- The result picks a **tested boss recipe**. Never a boss made by multiplying
  health and damage without changing behaviour.
- A more dangerous boss gains **one** of: a new attack pattern, another
  phase, a stronger elemental setup, a defensive feature, an elite modifier,
  better loot, a distinct corrupted look. Never all at once; it must stay
  readable, because nobody is steering.
- **A telegraph must not require a dodge that no input can perform.** In
  automatic combat a telegraph becomes something the party's preparation
  answers: a visible charge-up that a ward or defensive piece responds to, a
  phase that changes resistance, a support creature that shifts target
  priority, a spreading hazard that rewards a defensive build.

### Purity closes the loop

    enter a dangerous region -> survive the road -> defeat its boss
      -> raise regional purity -> later travel is safer -> further destinations open

*(Revised.)* This document expected a restored region to stay useful for
patrols and loot. The owner's endgame rule says otherwise: a fully cleared
region has no fights and no combat loot, and is worth visiting for its town
and its people ([endgame.md](endgame.md)). A region only partly restored
still has danger, and loot, on its remaining routes.

### Preparation

The preview makes the party's choices matter before leaving: expected
elements, likely statuses and hazards, whether the party has a counter,
whether capacity is enough, whether a pet or a particular piece of gear would
help, whether there is a way to recover.

**Never demand the perfect counter.** Several answers should work: a
counter-element, more defence, a safer route, a suitable pet, or accepting
the risk for the reward.

### What a fight must make clear

Who is attacking; with which element; whether it is strong or resisted; which
setup is active; when a combination fires; why a horse is taking damage; what
the boss's big attack does; why the party won or lost.

### Failure and safety

The owner has since decided this, and it is in [failure.md](failure.md):
nobody dies; a beaten party returns to the city it left, loses a configurable
amount of loot and pays a modest cost; and it waits to be sent out again,
never retrying by itself. From this document, still:

- Automatic play must never trap the player in repeated failure with no
  explanation.
- The likely time and the cost of a failed boss attempt are shown before
  committing.

### Generation and state

- A journey is generated from a structured recipe, not independent rolls:
  origin, destination, path, days, danger, encounter budget, a sequence seed,
  the boss recipe and tier, a reward-family bias, optional landmarks.
- Validate: the destination is reachable; exactly one final boss; the count
  within its bound; the boss tier fits the region and stage; reward matches
  risk; essential campaign routes stay possible; random events never
  overwrite required ones.
- Repeat trips keep the geometry and vary encounters, order, composition and
  rewards from their own seeds.
- Separate seeds for world, route events, encounters, bosses and loot. A
  changed decoration never alters a drop or the schedule.
- Journey progress is saved as it goes, so **reloading never duplicates a
  reward**.
- Opening the map updates light records from checkpoints. It never
  regenerates the world or rerolls a route, boss or loot table.

### Start small

A map of 8 to 12 tiles and 3 cities; purity per tile; a route preview; a
fixed daily distance; one short, one medium and one dangerous route; a seeded
daily check; the five kinds of event; a bounded count; one guaranteed boss
whose tier follows danger and progress; one purity gain after a victory; a
short travel report on arrival.

**Not yet:** thousands of tiles, survival supplies, dynamic weather,
injuries, road building, dozens of encounter tables. Prove the
route-danger-boss loop first.

## For Horse Heroes

- The slice's trail already is a bounded run ending in a final enemy, so
  rules 5 and the bounded count are met in miniature. What it lacks is
  everything that makes the run a choice: a map, a destination, a visible
  reason for its difficulty.
- **Trail level is replaced, not extended.** Difficulty today is a number
  that rises on a win. Under these rules difficulty is a property of where
  the party goes, read from the map. "Farm low or push high" survives as
  choosing a pure route or a corrupted one.
- Purity and the corruption level already described for scenery and enemies
  ([fairyland-content.md](fairyland-content.md)) are one quantity seen from
  two ends. There should be one number per tile, not two systems.
- The boss at the end of every journey is where the loot fountain erupts.
  Rule 5 guarantees the game its payoff on every trip, including the short
  safe one, which is exactly the "reserve the big fountain for real
  completion points" rule in [loot-fountain.md](loot-fountain.md). The scale
  of the fountain should follow the boss's tier.
- "Do not make every pause an ambush" is also a pacing rule for the eye:
  quiet days are the calm that lets the fights and the fountain read.
- Hazards answered by a trait, gear or a pet give signature traits, halters
  and pet jobs something to do outside combat, which each of those needed.
- Non-combat events are where recruits, pets and remembered consequences
  enter ([generations.md](generations.md), [pets.md](pets.md)). The event
  list is shared with those systems, not a second one.
- The sim is seeded, deterministic and free of rendering, so a journey can be
  resolved day by day with no scene, and resumed exactly. The seed domains
  this needs are already filed.
- Idle-first: with no input the party picks a destination by a stated rule
  and goes. A journey never waits at a fork or an event for an answer.

## Source

A design proposal on procedural travel and road encounters for this game,
received from the owner 2026-10-09 (all sections). It studied how several
other games and a tabletop ruleset handle travel and encounter pacing; only
the principles are kept here.
