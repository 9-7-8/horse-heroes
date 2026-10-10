# Inspirations

The games this one learns from, and exactly what each is for. The owner said
on 2026-10-09 that these may be named: inspiration is not infringement.

Use them as a **reference stack**, not one overall model.

> Each reference should solve a particular design problem, not bring its
> entire feature set into this game.

Horse Heroes is an original game. Nothing here is a claim that a reference
already implements one of its mechanics, and nothing is to be copied: no
names, content, numbers or specific designs. What is taken is the lesson.

## The lenses

- **Mechanics:** rules and systems. Equipment slots, prices, abilities,
  travel costs.
- **Dynamics:** the behaviour those rules produce. Saving a piece for a
  future recruit, changing the party to beat a boss, staying in town longer.
- **Aesthetics:** what the player experiences. Attachment, discovery,
  expression, comfort, mastery. Not visual style.
- **Juice:** audiovisual and interface feedback. Hit reactions, reveals,
  placement sounds, readable transitions.

The first three are the MDA layers ([the-loop.md](the-loop.md)). Juice is a
complementary lens, not a fourth layer ([juice.md](juice.md)).

## Who owns which part

| Part of this game | Reference |
| --- | --- |
| Automatic party and build loop | Lootun |
| Combat impact and loot payoff | Diablo III |
| Affix boundaries and loot filtering | Path of Exile 2 |
| Elemental cooperation | Genshin Impact, simplified |
| World-map travel | RimWorld |
| Road-event pacing | Pathfinder First Edition |
| Idle safety and reporting | Melvor Idle |
| Caravan shop and locals | Tiny Bookshop |
| Fantasy shop presentation | Teddy's Haven, Loot Shop Simulator |
| Generations and remembered lives | Wildermyth |
| Pets and fashion | Warframe |
| Cute creature expression | Slime Rancher |
| A travelling home, and departures | Spiritfarer |
| Nonlethal setbacks | Pokémon, Stardew Valley |
| Restoration and final closure | Terra Nil |

**If only five are studied, these:** Lootun, Diablo III, Tiny Bookshop,
Wildermyth, Terra Nil. Together they cover structure, moment-to-moment
satisfaction, the town half, emotional continuity and the finite ending. The
rest are consulted when their particular system is being built.

## Primary references

| Game | Study | Lenses | Use here | Do not import |
| --- | --- | --- | --- | --- |
| **Lootun** | Automated adventures, party builds, skill targeting, appraising equipment | Mechanics, dynamics | The closest structural reference for "horses fight by themselves; the player manages builds and loot". How quickly can an interesting item be recognised and acted on? | Its volume of modifiers, upgrade systems and equipment complexity |
| **Diablo III** | Skills, equipment synergies, dramatic combat payoffs, guardian rewards | Juice, mechanics, dynamics | The standard for attacks that feel consequential, a group defeat that feels rewarding, and a boss that ends the expedition with a satisfying payout | Endless combat progression after the world is saved; anything needing manual execution |
| **Path of Exile 2** | Restricted item-modifier pools; loot-filter presentation | Mechanics, dynamics, juice | Category-specific modifiers for horse gear and a readable priority system for loot. Keep what an item does apart from how strongly it is highlighted | Its full modifier complexity. The automatic keep-and-replace cargo rule is this game's own, not something a visual filter does |
| **Tiny Bookshop** | A portable shop, stocking, scenic locations, decoration, getting to know locals | Aesthetics, dynamics, mechanics | The strongest reference for a caravan that feels like a personal place in different towns: the relationship between stock, setting, presentation and familiar customers | Manual customer interaction that would become repetitive when idle |
| **RimWorld** | Caravans crossing a persistent tiled world with goods | Mechanics, dynamics | Route previews, travel duration, meaningful destinations, persistent world records; journeys that connect places | Survival micromanagement, permanent loss of characters, simulating everything continuously |
| **Wildermyth** | Procedural heroes, personalities, history, retirement, descendants, legacy characters | Dynamics, aesthetics, mechanics | The strongest reference for a horse becoming memorable across adventures, settling down, and having descendants who carry the story on | Forced ageing-out, injuries, death. Here retirement is reversible and companions are safe |
| **Warframe** | Companion-specific abilities, equipment that specialises them, fashion, recoverable incapacitation | Mechanics, dynamics, aesthetics | Pets with distinct abilities; pet gear that specialises them; fashion that stays personal while builds change | Large modification grids, separate pet weapon builds, mandatory upkeep |
| **Melvor Idle** | Automatic combat recovery, preparation, safe idle thresholds and supplies | Mechanics, dynamics | Clear potion rules, useful reports on unattended play, predictable unattended combat: how does a player know an activity is safe to leave running? | Permanent equipment loss and harsher death rules. Here the first failure ends the journey and waits |

## Supporting references

| Game | Study | Lenses | Use here |
| --- | --- | --- | --- |
| **Dragon Cliff** | Semi-idle party collection and equipment interactions | Mechanics, dynamics | A second view, beside Lootun, of whether party composition creates real differences and not three independent damage dealers. Not a reason to add classes |
| **Genshin Impact** | Applying one element and triggering a reaction with another; setup character against triggering character | Mechanics, dynamics, juice | How players learn elemental cooperation and recognise a payoff. The Primer and Finisher proposal stays smaller and suited to automation |
| **Teddy's Haven** | Cosy shop presentation, merchandise, decoration, a fantasy clientele | Aesthetics, juice, mechanics | A welcoming magical stall and attractive displays, with ordinary shop tasks translated into horse telekinesis and idle transactions |
| **Loot Shop Simulator** | Fantasy merchandise, unusual customers, a shop tied to their problems | Aesthetics, dynamics | Making surplus gear feel like merchandise with a place in the world, not objects deleted for coins |
| **Spiritfarer** | A travelling home, distinct companions, customisation, meaningful departures | Aesthetics, dynamics, juice | How a mobile home gathers personality and how a companion's departure is made to matter. The emotional framing for settling down, not its premise of death and an afterlife |
| **Slime Rancher** | Cute creatures with recognisable traits, expressive behaviour, collectible combinations | Aesthetics, juice, mechanics | Pet silhouettes, small animations, sounds and understandable identities: why a creature is lovable before any number. No hunger chores |
| **Stardew Valley** | Returning from danger with rewards, town relationships, recoverable collapse | Dynamics, aesthetics, mechanics | The rhythm of adventure and then familiar community. Its setbacks are a comparison; this game's are gentler, with no loss of equipped or protected gear |
| **Pokémon** | Team identity and nonlethal failure: fainting leads to recovery | Mechanics, aesthetics | The emotional contract that favourite horses and pets are safe even when an expedition fails |
| **Terra Nil** | Restoring damaged environments, wildlife returning, and leaving a recovered ecosystem | Aesthetics, dynamics, mechanics | The ending: success visibly removes the need for further intervention. The world becomes peaceful and does not grow another tier of corruption to keep farming alive |

## One that is not a video game

**Pathfinder First Edition**, for its guidance on journeys: summarise quiet
stretches, use a bounded number of encounter opportunities, and include
discoveries and social events instead of making every interruption an
ambush.

- Mechanics: encounter selection and threat budgets.
- Dynamics: preparing for a journey with a recognisable risk.
- Aesthetics: anticipation, exploration, breathing room.

The destination boss, danger from purity, automatic combat and the finite
ending are this game's adaptations, not that ruleset's rules.

## Also drawn on

- **The MDA framework** (Hunicke, LeBlanc and Zubek), for designing backward
  from the experience ([the-loop.md](the-loop.md)).
- **The owner's own horse breeding mod for Minecraft**, as the source of the
  coat colour genetics ([generations.md](generations.md)). That one is to be
  borrowed from directly, since it is hers.

## How to study one without growing the scope

Take **one short loop** from the game and write down four things:

1. **Rule:** what does the player actually do?
2. **Consequence:** what changes because of it?
3. **Feeling:** why does the player care?
4. **Feedback:** how does the game make the change understandable and
   satisfying?

Questions worth asking:

- Lootun: how many clicks separate finding a useful piece from testing it?
- Diablo III: what happens immediately before, during and after a rewarding
  kill?
- Tiny Bookshop: what makes an interaction with a customer feel personal and
  not transactional?
- Wildermyth: which remembered details turn a generated unit into a
  character?
- Warframe: when does a companion create a new build possibility and not
  just extra damage?
- Terra Nil: how does the environment say that a problem is truly resolved?

## Where each shows up in these files

| Reference | Files |
| --- | --- |
| Diablo III | [feedback-chain.md](feedback-chain.md), [loot-fountain.md](loot-fountain.md), [combat-rhythm.md](combat-rhythm.md), [build-laboratory.md](build-laboratory.md), [items-and-builds.md](items-and-builds.md), [cosmetics.md](cosmetics.md), [elemental-combat.md](elemental-combat.md), [gear.md](gear.md) |
| Path of Exile 2 | [gear.md](gear.md), [cargo.md](cargo.md), [travel.md](travel.md), [pets.md](pets.md) |
| Warframe | [pets.md](pets.md), [travel.md](travel.md) |
| RimWorld, Pathfinder First Edition | [travel.md](travel.md) |
| Melvor Idle | [failure.md](failure.md), [travel.md](travel.md) |
| Pokémon, Stardew Valley | [failure.md](failure.md) |
| Tiny Bookshop, Teddy's Haven, Loot Shop Simulator | [town.md](town.md) |
| Wildermyth | [generations.md](generations.md), [vision.md](vision.md) |
| Terra Nil | [endgame.md](endgame.md) |

Those files were written before the owner said the references may be named,
and say "a reference action RPG" or "other shopkeeping games". This table is
the key.

## For Horse Heroes

- The stack is long because the game has many halves. That is the risk it
  describes: each reference is here for one problem, and the vision's rule
  is depth, not breadth ([vision.md](vision.md)).
- The two references that matter most to the pillars are Diablo III, for the
  loot shower that is the product, and Terra Nil, for the ending that makes
  the shower finite.
- Naming a game as an inspiration is fine. Naming this game after one is
  not: it is called Horse Heroes, and `CLAUDE.md` says not to use the name of
  any existing game for it.

## Source

A note on reference games for this project, received from the owner
2026-10-09, with her statement that the inspirations may be recorded by name.
