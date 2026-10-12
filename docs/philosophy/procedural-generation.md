# Procedural generation

How generated content is made here, whatever it is: horses, enemies, gear,
trails, scenery. What each of those needs in particular is in
[fairyland-content.md](fairyland-content.md).

> Generate instances freely; author rules carefully.

## Principles

### Recombination, not unauthored design

Build a **small authored vocabulary** and a **large space of valid
combinations**. Authored rules are a strength of procedural generation, not a
failure of it.

"Functionally infinite" only means players are unlikely to run out. A million
differently tinted cubes are not a million different enemies. **Depth comes
from interactions and choices, not from the number of random parameters.** The
goal is not that every object is unprecedented; it is that familiar families
produce enough valid, readable, mechanically interesting variation to keep
exploration and customisation personal.

### Four layers that must agree

| Layer | Question |
| --- | --- |
| Identity | What is it, fundamentally? |
| Gameplay | What does it do, and how does it affect choices? |
| Appearance | How do its identity and mechanics become visible? |
| Variation | What details make this one recognisable? |

Decide them in that order. Appearance is **derived from** identity and
gameplay, never rolled beside them without context. A frost guardian that
resists certain attacks is covered in ice plates because of what it is.

**Decide whether a trait is cosmetic or mechanical.** Never infer power from a
cosmetic roll such as horn length. If a visible trait means something, it
means it everywhere: in the silhouette and in the tooltip.

### One pipeline for everything

    context -> recipe -> parameters -> constraints -> validation -> realisation

- **Context:** where and when: biome, difficulty, corruption, progression,
  rarity, tags.
- **Recipe:** a named authored family (a kind of guardian, a kind of saddle).
- **Parameters:** one particular member of the family.
- **Constraints:** reject incompatible combinations or narrow their ranges.
- **Validation:** gameplay fairness and geometry safety.
- **Realisation:** build the gameplay data, meshes, materials, names and UI.

Recipes are data wherever practical.

### Seeds and identity

- **No single random stream.** Adding a flower must not change the next
  boss's loot or an existing horse's coat. Derive a separate seed per domain
  (world, region, trail, encounter, enemy, loot, horse, cosmetic) and per
  aspect of one object (body, markings, attachments, stats, name).
- **Derive seeds with a specified stable hash and a pinned generator.** Never
  a language's default hasher, whose output may change between versions.
- **Everything persistent has a stable ID.**
- **A seed alone is not a save format.** Rules change. Save the ID, the
  recipe, the generator version, the resolved parameters, gameplay properties
  and player edits; the seed is optional, for debugging.

> The player should not log in after an update to discover that their
> favourite horse or rare saddle has become something else.

- **Appearance and gameplay use separate seeds.** A cosmetic edit never
  changes stats or loot, and the reverse.

### Three ways to make a model, used together

1. **Modular assembly:** parts joined at defined sockets. First.
2. **Parametric deformation:** change proportions of an existing shape,
   within modest ranges, with named landmarks so gear still fits.
3. **Generated geometry:** new vertices and triangles, for shapes that are
   actually new (horns, vines, crystals, plates).

Not every surface has to be generated for every instance to be customised.

### Constraints, budgets, fallbacks

- **Hard constraints are about validity:** correct slot, allowed tags,
  connected route, finite stats, usable mesh, no invalid effect, guaranteed
  spawn space. A violation is rejected.
- **Soft preferences are about quality:** pleasing colours, silhouette
  variety, encounter contrast, fitting names, uncommon but comprehensible
  combinations. A violation costs score.
- **Budget each thing separately:** enemy power, item power, encounter
  danger, vertex count, material variety, particles, attachments at once.
- **Never retry forever.** A bounded number of attempts, then a known-valid
  fallback, and a log of the failed seed and the reason.
- **Bound triggered effects.** Limit depth, define whether an effect can
  trigger another, cap stacking. Never combine unrestricted triggers and
  effects at random: that yields dead items or endless loops.
- Generated balance still needs authored limits and playtesting.

### Structure

Four layers of code, each usable without the ones after it:

    data (specs, recipes, versions) -> generation (context + seed -> validated spec)
      -> simulation (consumes specs) -> presentation (specs -> meshes, materials, UI)

Generation and combat run without rendering. An unwatched run uses the same
rules and never builds a mesh.

Generate on spawn or on change, never per frame. Share base meshes and
materials. Cache generated meshes by recipe and resolved parameters, bound the
cache, and round away differences nobody can see.

### Testing

Two kinds, and neither replaces the other.

- **Batch tests, no rendering, thousands of specs:** no invalid numbers or
  indices; budgets respected; every item equippable and every reference real;
  trails connected; frequencies match weights; the same seed and version give
  the same result; cosmetic edits leave stats alone; failures reach the
  fallback; effect chains end.
- **A seed gallery, looked at by a person:** many results side by side. No
  test can say whether it looks like a horse or whether a horn hides the face
  at the real camera angle.

Then ask whether the variety **means** anything:

- Do generated enemies ask different build questions?
- Do items enable different loadouts?
- Do trails change preparation or what is worth farming?
- Can players tell generated things apart and remember them?

> If only the names and colours change, the generator is not yet creating
> depth.

### Order of work

Do not build a universal geometry or rules generator before the smallest loop
is fun. **Every new generator must have an identifiable player benefit.**

The source's suggested order, roughly: (1) the seeded trail, simple stats and
basic gear; (2) shared bodies, palettes, validated sockets, one truly
generated mesh, a gallery; (3) enemy roles and one modifier each, item budgets
and tags, targeting, filters, reports; (4) body proportions, markings, more
generators, branching trails, corruption profiles, a few authored
build-changing effects; (5) large-scale balance testing, customisation and
collection, save migration.

## For Horse Heroes

- Pillar 3 and pillar 4 already point here: nothing may depend on a specific
  model, and numbers are data. A recipe is the data-file form of a family.
- Pillar 1 decides the order. A generator earns its place when it makes a
  drop feel better or mean more. Generated gear that can be seen on the horse
  is close to the product; generated scenery is further away.
- The sim already has the right shape: plain Rust, seeded, no rendering,
  tested by rolling many seeds. Generation belongs in `src/sim/`, realisation
  in `src/game/` and `src/fx/`.
- The RNG is a hand-written SplitMix64, so the generator is pinned. Streams
  are separated today by XOR-ing the trail seed with constants, which is the
  idea done ad hoc; it needs named domains before more generators arrive.
- A horse's look is seeded from its position in the party, in the rendering
  code. It has no stable ID and no spec, and nothing is saved. That is fine
  for boxes and wrong for a horse a player has come to like.
- **A horse's two signatures are the first generated thing that is rules
  and not looks** ([signatures.md](signatures.md), #315). The same pipeline
  applies: an authored vocabulary of typed parts, validity before
  acceptance, a seed domain for each slot, resolved numbers saved with a
  version. A breed's build and a foal's body genetics are
  [horse-shape.md](horse-shape.md).
- Read the pinned Bevy version's docs and its custom-mesh example before
  writing any mesh code (`CLAUDE.md`, Stack). The source is an architecture
  proposal and says so; none of it is checked against Bevy 0.20.

## Source

Owner's design document on rule-bound procedural content for this game,
received 2026-10-09 (sections 1 to 4, 11 to 14, 16).
