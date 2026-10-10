# Cosmetics and the wardrobe

How dress-up and loot live together. Fashion is the confirmed endgame
([vision.md](vision.md)); what follows is a proposal for how to do it, and its
open points are a `decision` issue.

The player should be able to say both:

> "This gear makes my party work better."
>
> "This is exactly how I want my horses to look."

## Principles

### The ten

1. Gameplay gear decides what the horse can do.
2. The chosen appearance decides how that gear looks.
3. Cosmetics can carry emotional and narrative meaning without mandatory
   power.
4. Appearances are unlocked through play, and the player is never made to keep
   an obsolete item to keep its look.
5. If dress-up is a core activity, changing looks is easy.
6. The player makes a distinctive horse and party, not the mathematically
   optimal appearance.
7. Horse equipment suits a world without riders.
8. Saddlebags visibly show carrying capacity and never become another attack
   slot.
9. A cosmetic's origin is tied to places, people, pets and family history.
10. Procedural variety works inside strong visual rules, so pieces stay
    readable and memorable.

### Two pleasures

- **Expression:** making a look the player enjoys, whether elegant, cosy,
  strange, heroic, coordinated or deliberately mismatched.
- **Collection and discovery:** finding a piece in the world, seeing that it
  suits a horse, adding it to the wardrobe.

Serve both. **Do not assume the player wants the party dressed in the highest
rarity.** Expression must be free to ignore matching and power signals.

### Three things tracked per horse

| | Decides |
| --- | --- |
| Equipped gear | Stats, abilities, elemental behaviour, capacity |
| Displayed appearance | The visible piece |
| Colour and material | Dye or palette, where the appearance supports it |

Changing one must never unexpectedly damage another. The interface makes the
distinction plain.

Default policy:

- a slot set to "use the equipment's appearance" shows new gear normally;
- a chosen look stays until the player changes it, and equipping new gear
  does not overwrite it;
- reverting to the real appearance is always free;
- changing colours is free, or costs something trivial and renewable.

### Why separate them

- Build choices are protected from looking wrong: a stronger piece need not
  be worn as it looks.
- Loot gains a second reward: a drop may improve the build, unlock a look, or
  both.
- Looks can be assembled freely, not limited to what is equipped.
- Collection is easy to understand: find it, own its look.
- Customisation is tied to play: encounters and places yield looks.

### Do not tax dress-up

A fee on every change makes experimenting feel like a tax, and here it would
compete with buying and selling in town. **Put the economic tension in gear,
trade and cargo, not in expression.** Dyes are reusable once found, or are
themselves a collection, not consumables.

### Unlocking

> When the player obtains an eligible piece, its appearance joins the
> collection for good.

Selling or scrapping the piece never removes the look. Other sources, each of
which should **mean something**: a guardian, a restored region or town
milestone, a shop, a gift from a resident, a pet's adoption or hometown, an
event, a distinctive place, a retired horse or a family history, a rare
generated variant.

Announce an unlock clearly and let it be previewed before use.

### A wardrobe for horses

No bridles, bits or reins; halters; saddles seat pets; saddlebags carry.
Beyond that, avoid any visual language that suggests an unseen rider: trailing
reins, a bit, stirrups, a human-sized saddle.

Families of visible equipment:

- **Head and neck:** halters, mane ornaments, flower crowns, ear decorations,
  charms that do not restrain, horn decorations.
- **Body:** blankets, barding and light armour, capes and mantles made for
  horses, ornamental harness that is clearly not riding tack, markings and
  glow patterns.
- **Legs and hooves:** hoof boots, leg wraps, fetlock ornaments, hoof effects
  and trails.
- **Cargo:** saddlebags, pack baskets, trail bundles.
- **Pet seating:** saddles and seats, pet blankets and nests, pet
  accessories.

**Appearance options are broader than gameplay categories.** One gameplay
identity may take several forms: a focus as a pendant, a woven charm or a
floating rune; protection as a blanket, a mantle or light barding. The
gameplay categories stay distinct; the wardrobe stays varied.

### Meaning without power

A cosmetic can matter without changing a number. It can carry a memory, a
relationship, a place, a milestone, a chosen identity, a family history.

If a cosmetic ever has an effect:

- primary power stays on real equipment;
- appearance-only pieces are mechanically neutral;
- any bonus is small, optional and legible;
- **never a large exclusive bonus on a look a player may dislike**;
- no set bonus that requires matching every slot, unless appearance
  overrides fully cover it.

The compromise worth having: a **collection bonus** for obtaining a set, with
no requirement to wear it.

**Never require a story reward to be worn for the story to count.**

### What makes dress-up good

- **Preview** at the real gameplay angle, with a closer inspection view: how
  it fits, whether it clips, how the pet sits.
- **Outfit presets**, named, and **independent of gameplay loadouts**: one
  outfit may go with several builds.
- **Colour control** by part (base, trim, metal or crystal, pattern, glow),
  from curated palettes first, to avoid muddy results.
- **Layers** in a fixed order with compatibility rules: coat and markings,
  legs and hooves, blanket or barding, saddlebag, halter and head ornament,
  pet saddle and pet, optional aura or trail. Never unlimited attachments in
  one place.
- **Silhouette protection:** the horse still reads as a horse; head, legs and
  identity stay visible; class and element cues survive where they matter;
  the pet is recognisable; combat signals are never covered. Large effects
  can be reduced or hidden.
- **Party coordination as an option, never a uniform:** shared town colours,
  a family emblem, complementary palettes, or deliberate mismatch.

### Cosmetics as memory

Appearances can be linked to events: a restored town's woven blanket, a
family ornament passed to a descendant, a sold piece later seen on a
resident, a token gained after a major journey, a town's palette changing as
it prospers. A short journal line gives the provenance. The piece then
records the player's own history.

### Generated looks

    family -> silhouette recipe -> material palette -> trim -> pattern
      -> ornament -> validation

- tested attachment points; no straps or ornaments through the body;
- silhouette and material count inside a budget;
- readable at the small gameplay camera;
- **stats independent of visual complexity**;
- a stable appearance ID, recipe version and resolved parameters, so saved
  looks survive updates.

**No "infinite cosmetics" that differ by tiny noise.** Aim for a library of
shapes and combinations a player can recognise and remember.

### The collection

Organised by family, slot, palette, source place, element theme, pet
accessories, new, favourites, owned or not. Each entry shows whether it is
unlocked, where it came from, which slot it uses, what it conflicts with,
whether it is purely cosmetic, a preview on the current horse, and a
comparison with the current look. Search and filters arrive before the
collection is large.

### Order of work (as suggested)

1. Separate stats from the model.
2. Unlock appearances and a simple wardrobe; looks survive selling.
3. Dyes and presets.
4. Pets and town cosmetics; provenance in the journal.
5. Generated variants, with fit and readability checks.

## For Horse Heroes

- A piece today has no look at all; gear is not yet drawn on the horse. The
  first step in the order above therefore starts from showing gear, which is
  already filed. The separation should be in the data model from the start,
  so that a look is never bolted onto a stat item afterwards.
- This is where the fountain pays off twice. A drop that is no upgrade can
  still be a new look, which answers the relevance problem in
  [loot-fountain.md](loot-fountain.md) from a second direction: fewer drops
  are worthless.
- The spotlight in the loot report should treat a first-time appearance as
  worth showing, alongside upgrades.
- The rule that a rarity signal must be readable
  ([juice.md](juice.md)) applies to the **drop**. On the horse, the player's
  chosen look wins: a horse need not look as rare as its gear.
- Readability rules still bind the wardrobe
  ([fairyland-content.md](fairyland-content.md)): element and class cues, and
  the horse's own silhouette, come before any outfit.
- Generated looks and stable identity are covered in
  [procedural-generation.md](procedural-generation.md). The same rule
  applies: a favourite look never changes under the player after an update.
- Provenance needs the journal and persistent state, so it comes late; the
  record of where a look came from should be stored from the first unlock,
  even before anything displays it.

## Source

A design proposal on cosmetics, appearance collection and equipment identity
for this game, received from the owner 2026-10-09 (all sections). Its study of
how a reference action RPG separates a piece's look from its function is
summarised under "Why separate them" and "Do not tax dress-up".
