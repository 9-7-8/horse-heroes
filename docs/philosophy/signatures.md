# Signatures

Every horse has two permanent abilities of its own, generated and never
picked from a list: one for combat, and one for everything that is not
combat. The short version:

> Author the small vocabulary, the compatibility rules and the budgets.
> Generate the horse's actual ability from those parts.

Three kinds of statement are kept apart here, as the source keeps them.
**Settled** is the owner's decision and is also in [vision.md](vision.md).
**Proposed** is every number, band, weight and formula. **Open** is on #316.

## Settled by the owner (2026-10-11)

1. **Two signatures on every horse, both permanent and procedurally
   generated:** one for combat, one for town. Each has its own pool of
   events and effects, its own budget, its own seed domain and its own
   saved, resolved numbers.
2. A town benefit never replaces the combat signature, and **neither takes a
   loadout slot**.
3. **Town means everything that is not combat**: travel, forage, recovery,
   construction, trading and whatever comes later.
4. **A town signature is active for every horse physically in the town**,
   whatever brought it there. No residency and no assignment is needed.
5. **Town signatures stack.** Not strongest-only, and no blanket diminishing
   return. Tuning residents may be an endgame pleasure and is never needed
   for the main journey.
6. **Each breed has its own pool of parts**, for each slot. Pools may
   overlap. Breed is how a player goes looking for particular parts.
7. **A foal builds each signature from its parents' matching signatures,
   part by part:** a 1% chance that the part mutates, otherwise one parent
   or the other, evenly. Numbers are taken exactly from a parent. It rerolls
   until valid. A clone of a parent is allowed.
8. Breeding stays optional flavour: never needed for the main loop, and no
   promise of anything new.

## Principles

### A typed expression

A signature is nine kinds of part: a mode (a triggered effect, or a modifier
of an action already resolving), an event, event filters, a state condition,
a selector, an effect, a repetition pattern, parameters and limits. Each is
a type. The generator composes types, so a rule that makes no sense is
refused before it reaches a fight, and prose is never parsed.

### One understandable rule

One generated expression per signature, not one primitive. "Every fourth
direct Primer cast while below 40% health shields the lowest-health ally" is
one signature made of several parts. To begin with, the proposal is one
event, up to two filters, at most one condition, one selector, one effect
and one repetition pattern. The name and the sentence are generated from the
resolved parts; a name never decides what a signature does.

### Event, predicate, selector, effect

An event is an opportunity to evaluate. A predicate is true or false. A
selector returns a creature or a bounded set. An effect changes state. Kept
apart, and run in one order: the event, its filters, the condition, the
count, the gate and limit, the target, the effect.

- **Each event has a restriction** that says how often it is raised: once
  per action and not per victim; a crossing and not every tick below the
  line; once per award and not per frame.
- **A modifier acts inside the action** it belongs to, before the result is
  final. It never makes a second attack.
- **Conditions read the snapshot** at the start of the action.
- **No target, nothing spent.** The event counts, the cooldown and any
  stored charge are kept, and the report calls it unproductive.
- **Several recipients split one budget.** A bigger party is more coverage,
  not more power.
- **A stored benefit is one charge**: it refreshes, never stacks, and
  spending it raises no event.
- **Only direct actions count.** A signature, an item, a pet or damage over
  time never triggers a signature.
- **What the vocabulary leaves out on purpose:** new reactions, revival
  loops, cooldown resets, extra full-strength casts, innate elemental
  resistance, more Primer charges on a crit.

### Generate structures

Pick a broad family, then assemble parts from the pool inside the types. A
family (sustained recovery, rescue, direct pressure, setup investment,
payoff cooperation) is a constraint on assembly, not a prewritten skill. A
candidate that is invalid or outside the pool has no chance at all.

- **A witness build.** A combat signature is accepted only if some legal
  loadout and reachable fight makes it useful. The horse's present loadout
  need not be that one. A town signature needs a reachable town activity in
  the same way.
- **Enable a part only when its mechanic exists.**
- **Avoid the tautological and the impossible.** A very restrictive
  combination is scored as restrictive, not paid for with unlimited size.
- **Prefer a count to a chance** to begin with: "every fourth" has a rhythm
  that can be seen. A count is still procedural generation.

### Budget the rate first

Work out how often a rule can fire before deciding how large it is: from
the event's rate, its count or chance, and its cooldown. That is an upper
bound, and the real figure comes from running fights. Put health, damage
and barrier on one scale against reference encounters before comparing
families. Enforce two budgets, sustained output and the largest single
event, across short fights, long bosses, and parties of three and five.

### Variety that changes a build

A different event, selector, effect or condition is variety. "12% against
14%" is not. For unrelated founders a rule can be reduced to a canonical
form and fingerprinted, and candidates preferred that differ in kind from
recent ones, always inside the breed's own pool. A local preference, never
a promise that every horse is unlike every other. The count of raw
combinations is not a count of good abilities.

### Breed pools

Fresh generation draws from the breed's pool. Inheritance does not: a
foal's parts come from its parents. They are two operations and must not be
confused.

### Inheritance is by part

- A part is semantic: an event, a filter, a condition, a target, an effect,
  a repetition pattern, a frequency limit, or one attached number.
- **A number belongs to a named field.** A health threshold cannot land in
  an action count, and even matching units must satisfy the field's domain.
- **Exact parental values.** No blending, no fresh roll.
- **Reroll until valid, from what the parents support.** Nothing from
  outside arrives except by mutation.
- **A clone is fine.** No forced mutation, no novelty rule.
- The chance of at least one mutation grows with the number of parts; it is
  not a flat 1% a foal.
- In the retry, keep each part's mutation flag, so a failure elsewhere does
  not roll mutation again.

### The town signature

- It acts on **simulated recovery, work, sales, forage and travel**, as a
  rate or once per real award. Never per menu opened, per catch-up update,
  per frame or per hoofbeat.
- **Where and during what a part applies is written into the part.** A
  travel part applies to the party travelling; a horse in a distant town
  does not boost a journey it is not on.
- The same grammar and the same generator as combat, with its own parts.

### Save what was resolved

Both signatures are stable across save, load and generator updates. Adding
scenery or changing a breed's weights never rerolls a horse that exists.

## For Horse Heroes

- **Nothing of this is built.** Each horse today has one ability picked by
  a hidden `class` field in `assets/data/horses.ron`, fired on cooldown
  (`docs/expedition.md`). There are no Primers, Finishers, crits, glancing
  hits, barriers, towns, forage or breeds yet, so almost
  every event in the vocabulary is switched off. That is why each part
  names the mechanic it needs.
- **It replaces the "one modest signature trait"** the owner chose on
  2026-10-10 ([vision.md](vision.md)); that reading is put to her on #316.
  The stat lean (#223) stands beside it.
- **It reverses "nothing a horse inherits affects how it fights"**
  ([generations.md](generations.md)). What survives is the guardrail:
  breeding is never needed and a descendant is no stronger on average.
- **It fits the generation rules already written**
  ([procedural-generation.md](procedural-generation.md)): an authored
  vocabulary, one generate-validate-fallback pipeline (#58), named seed
  domains (#57), resolved parameters and a version (#59), batch tests (#60).
- **It runs through the effect pipeline** (#165) with an origin of its own,
  as gear effects and pet abilities do. The anti-chain rules of
  [elemental-combat.md](elemental-combat.md) are that pipeline's settings.
- **Pillar 4.** The vocabulary, the compatibility table, every band and
  every budget are data. The source's tables are the first values, each
  marked proposed.
- **Pillar 2.** A signature needs nothing from the player. It is found, not
  configured.
- **Where it stands against pillar 1.** It does not make a drop feel better.
  It makes a recruit and a build worth a second look, and it waits behind
  the fountain.
- **A town signature is not a way round the town rules.** Goodwill still
  never buys combat power, and the party still gains from finished projects
  ([town.md](town.md)); the town effects here are proposals to be fitted to
  those rules when the town systems exist.
- **The work** is the epic #315. The open questions are #316.

## Source

The owner's working design "procedural abilities and breed mathematics",
dated 2026-10-11 and received the same day; its sections on the two
signatures, the vocabulary, the grammar and the validation targets. It ends
complete and is kept unedited as
`docs/source/procedural-abilities-and-breed-mathematics.md`, where the
tables of events, filters, conditions, selectors, effects and limits are.
Its sections on breed builds and gaits are in
[horse-shape.md](horse-shape.md) and [horse-movement.md](horse-movement.md).
The document says no issues were to be opened yet; the owner asked for them
on handing it over.
