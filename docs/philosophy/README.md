# Philosophy

Advice and guidelines to follow while building Horse Heroes. `CLAUDE.md` holds
the rules and the four pillars; these files hold the reasoning behind feel and
design calls, so that a new system can be checked against something more
specific than "make it satisfying".

Read the relevant file before designing or tuning the system it covers.

| File | Covers |
| --- | --- |
| [five-link-loop.md](five-link-loop.md) | **Build this first.** The smallest whole game: five links (route, party, expedition, loot, town), one rule and one test each, and the minimum integrated test |
| [inspirations.md](inspirations.md) | The games this one learns from, by name, and the one problem each is for; how to study one without growing the scope |
| [vision.md](vision.md) | **Read first.** What the owner wants built: the world, the loop, what the player controls, equipment rules, what is confirmed, proposed and open. Outranks the other files |
| [elemental-combat.md](elemental-combat.md) | How horses cooperate: Primers, Finishers, three pair reactions, three same-element Crescendos, anti-chain rules, automation, how it is shown (a proposal, see the file) |
| [failure.md](failure.md) | What happens when a journey fails: nobody dies, a safe return, a bounded loss and cost, no automatic retry, a report that teaches, and the tone of it |
| [endgame.md](endgame.md) | How it ends: success makes combat unnecessary; what "cleared" means; a finite loot supply; the dragon's defeat; what stays playable in peace |
| [generations.md](generations.md) | Retirement into towns and back, residents building by magic, town states, families and colour genetics, descendants, updating towns on demand |
| [cosmetics.md](cosmetics.md) | The wardrobe: gear, displayed look and dye kept separate, unlocking looks through play, rider-free equipment, meaning without power, presets, layers, provenance (a proposal) |
| [pets.md](pets.md) | Pets: what is confirmed (cannot die, assigned to horses, collar and optional armour), one signature ability, supporting reactions without replacing horses, gentle exhaustion, favourites kept viable |
| [gear.md](gear.md) | The owner's slot mapping (halter, barding, boots, martingale, horseshoes, mouth weapon, braid, plus saddlebags and pet saddle), what each slot owns, accuracy and crit rules, exceptional pieces, how a piece is generated and compared |
| [town.md](town.md) | The non-combat half: the owner's six caravan-shop decisions, what makes it cosy, the town session, listing and selling over time, player-set prices and goodwill, buying, recruiting and retiring as social moments, decoration, town juice and interface |
| [travel.md](travel.md) | Journeys: the owner's eight travel rules, the tile map, purity and danger, routes, travel days, road events, the guaranteed boss, failure, and starting small |
| [cargo.md](cargo.md) | Shared party cargo: the owner's five decisions, what happens when a drop arrives, protection, keep rules, explanations, rarity shown several ways, selling on arrival, capacity changes |
| [horse-shape.md](horse-shape.md) | The base horse: a sport-type Friesian build, every dimension as a ratio of withers height, landmark heights, and what carries the likeness in a low-poly model |
| [distribution.md](distribution.md) | Shipping on Windows: a per-user installer, a separate launcher that checks releases, verifying before executing, where saves live, publishing order and clean-machine tests (untested guide) |
| [the-loop.md](the-loop.md) | The four layers, what the player should be able to say, the loop from intention to hypothesis, and the checklist to run before adding content |
| [incremental-and-idle.md](incremental-and-idle.md) | Accumulation and reinvestment, the tension every resource needs, respecting absence, bounded offline progress, prestige |
| [juice.md](juice.md) | Feedback in general: the four questions, three levels of importance, state changes, sound, motion, and not hiding information |
| [feedback-chain.md](feedback-chain.md) | Why every action must visibly change the world, and the time scales a reward loop runs on |
| [loot-fountain.md](loot-fountain.md) | The reward ceremony: its seven beats, what makes it land, and why drops must stay relevant |
| [combat-rhythm.md](combat-rhythm.md) | Attack phases, keeping timings separate, and response before polish (incomplete, see the file) |
| [build-laboratory.md](build-laboratory.md) | What an idle game keeps from an action RPG: the five promises, policies in place of execution, bounded runs, difficulty as a choice, targeted farming (incomplete, see the file) |
| [items-and-builds.md](items-and-builds.md) | What an item must be: a rarity ladder with jobs, rule-changing powers, relevance as a mix, sets without uniforms |
| [procedural-generation.md](procedural-generation.md) | How anything is generated: authored vocabulary, four layers that agree, one pipeline, seeds and stable identity, constraints and fallbacks, testing, order of work |
| [fairyland-content.md](fairyland-content.md) | The rules for each generated thing: horses, enemies, gear, trails, corruption, and readability above detail |

## How these files are kept

- One topic per file. Each has **Principles** (general, stated as things to do),
  **For Horse Heroes** (what that means in an idle game where the party fights
  by itself), and **Source**.
- They are guidance, not a backlog. Work to be done is a GitHub issue; an issue
  cites the philosophy file that justifies it.
- Keep the owner's decisions apart from advice. What she has confirmed goes in
  `vision.md` as confirmed; a proposal stays marked as proposed until she
  decides, and an open question is an issue labelled `decision`.
- When new source material arrives, fold it into the file that already covers
  the topic. Add a file only for a new topic, and add it to the table above.
- When a principle is tried in the running game and turns out wrong for this
  game, change the file and say why. The owner's verdict on feel outranks
  anything written here.
- The game is original. These files take principles from studying other
  games, never content, numbers or specific designs. The games themselves may
  be named as inspirations (owner, 2026-10-09) and are listed in
  `inspirations.md`; files written before that say "a reference action RPG"
  and the like.
