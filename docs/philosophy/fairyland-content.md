# Fairyland content

The rules for each kind of generated thing in this game. The method they share
is in [procedural-generation.md](procedural-generation.md).

The relationship everything here serves:

    world context -> a comprehensible enemy -> a build decision -> victory
      -> coherent loot -> customisation -> a new reason to set out

## Principles

### Readability first

The scene is small on screen. **Readability is more important than geometric
detail.** Nothing (corruption, decoration, effects) may hide elemental
identity, a danger signal, or a horse's silhouette. Danger is shown by shape
and icon, never by colour alone.

### Horses

- Start from the simple primitive horse; vary proportions, coat, mane, tail
  and tack modestly.
- Constraints: the silhouette reads as a horse at gameplay zoom; head and
  legs stay in readable proportion; hooves meet the ground; saddle and blanket
  sockets follow body landmarks; head space is reserved for ears, bridle and
  ornaments; two attachments never occupy one socket's volume.
- **Customisation is directed by the player as well as rolled:** choose a
  palette or style, lock favourite traits and reroll the rest, preview before
  committing.
- **Appearance is separate from power.** Changing how a horse looks never
  rerolls stats. Coat colour is not combat strength unless that is a
  deliberate, clearly explained fantasy mechanic.
- Classes get distinct tack cues, but the player must still recognise **their
  horse** under them. Class and element remain separate systems.
- Breeding, if ever, is an inheritance model of its own. Visual noise is not
  a substitute for inheritance rules. Not needed now.

### Enemies

- The body is a recipe, not the whole enemy. Variety comes from attachments:
  horns, plates, roots, spikes, eyes, runes, cracks.
- **Mechanics before decoration**, in this order: role (bruiser, swarm,
  ranged, protector, boss), element, tier budget (health, damage, defence,
  attack rate), at most one special modifier to begin with, then appearance
  derived from all of that.
- A visual grammar per element, by shape as well as colour:

  | Element | Cues |
  | --- | --- |
  | Fire | ember cracks, warm highlights, flame-shaped horns |
  | Ice | faceted plates, cool highlights, crystal horns |
  | Plant | bark panels, leaf fins, vine horns |
  | Corruption | shared dark fissures or an accent, never covering the element's cues |

- A modifier is shown twice: in the silhouette and in the tooltip.
- **Families behave consistently.** Variation happens inside a family so the
  player can learn it. An unfamiliar ruleset every encounter teaches nothing.

### Gear

- Generate in this order: slot, base recipe, level, rarity, power budget,
  allowed affixes, appearance, name, validation.
- Rules:
  - every piece fits at least one supported horse;
  - rarity limits both affix count and ornament complexity;
  - a class-tagged bonus refers to an ability that class really has;
  - stats stay inside the tier and rarity budget;
  - two affixes that mean the same thing never stack under two names;
  - decoration never changes which slot a piece occupies;
  - **a name describes real properties**, never a power the piece lacks.
- Build-changing effects are **authored**, with generated supporting stats.
  Trigger, effect, limits, cooldown and eligible abilities all come from
  tested rules.
- A piece can be recoloured without rerolling it.
- **Comparison, locks, filters and salvage come before more drop volume.**

### Trails

- **Two maps.** The logical map (encounters, progression, rewards,
  difficulty) is authoritative. The visual map (ground, props, scenery)
  expresses it.
- **Pace it; do not roll it.** An introductory fight, a variant, a harder
  one, a final. Independent rolls can produce four trivial fights or four
  impossible ones.
- A trail chooses: biome palette, dominant element, corruption level,
  difficulty, encounter seed, scenery seed, and a bias toward a loot family.
- Build order: path or graph, pacing roles, reserved combat areas and lanes,
  decoration outside those, biome detail, then a check on visibility.
- Noise and grammars are tools for scenery. They do not replace gameplay
  constraints.
- **Pick one coordinate convention.** Combat distances live in simulation
  space. Decorative scrolling is never mixed with real movement of a target.
- Keep only nearby ground; reuse what the party has passed.

### Corruption

- Corruption is a **shared input to every generator**, not a random tint. At
  low levels: small fissures, sparse dead plants, one altered enemy. Higher:
  more infected growth, darker accents, more enemy modifiers allowed, more
  elites, a corrupted boss family, more dramatic scenery.
- Visual corruption and mechanical difficulty can be separated. **A healed
  place must still be worth farming**; restoring it must not delete its
  value.

## For Horse Heroes

- Enemies are named creatures (Thornling, Gloom Sprite, Mire Toad and two
  finals) drawn as cubes, each with fixed stats and a rolled element. The
  names already suggest families; they have no roles, no modifiers and no
  appearance of their own. The element is currently shown by colour, which
  the readability rule says is not enough.
- A trail is three ordinary encounters in a shuffled order and one final.
  That is "rolled, not paced": the hardest ordinary fight can come first.
- Gear is a rarity prefix, a base noun and stat points. The name describes
  real properties today because there is so little to describe. That has to
  stay true when effects and looks arrive.
- The trail scrolls left under a fixed camera while the party stays put, and
  the fight is resolved in the sim with no positions at all. So the
  coordinate rule is met for now, by the sim having no space. It becomes a
  real decision the moment range or formation matters.
- The premise is horses bringing back horse gear from corrupted places, so
  corruption is already the fiction. Making it a generator input ties the
  look of a place, its enemies and its loot to one number.
- Horse customisation is worth more here than in most games of this kind:
  the horses are the heroes. A player's attachment to a particular horse is
  an asset, which is why stable identity matters before variety does.

## Source

Owner's design document on rule-bound procedural content for this game,
received 2026-10-09 (sections 6 to 10, 15).
