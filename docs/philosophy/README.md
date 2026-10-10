# Philosophy

Advice and guidelines to follow while building Horse Heroes. `CLAUDE.md` holds
the rules and the four pillars; these files hold the reasoning behind feel and
design calls, so that a new system can be checked against something more
specific than "make it satisfying".

Read the relevant file before designing or tuning the system it covers.

| File | Covers |
| --- | --- |
| [vision.md](vision.md) | **Read first.** What the owner wants built: the world, the loop, what the player controls, equipment rules, what is confirmed, proposed and open. Outranks the other files |
| [elemental-combat.md](elemental-combat.md) | How horses cooperate: Primers, Finishers, three pair reactions, three same-element Crescendos, anti-chain rules, automation, how it is shown (a proposal, see the file) |
| [generations.md](generations.md) | Retirement into towns, town states, families and descendants, updating towns on demand, a completable world and optional endless play (her proposal; details are recommendations) |
| [cosmetics.md](cosmetics.md) | The wardrobe: gear, displayed look and dye kept separate, unlocking looks through play, rider-free equipment, meaning without power, presets, layers, provenance (a proposal) |
| [pets.md](pets.md) | Pets as companions with one small job: a collar, forgiving assignment, cuteness that is not a stat (a recommendation toward an open decision) |
| [gear.md](gear.md) | The starter gear system: each category owns a question, what each may carry, a small item, saddlebags, the town shop, how to grow it and what not to add early (categories are a proposal) |
| [town.md](town.md) | The non-combat half: what makes it cosy, the town session, selling as curation, prices and trust, buying, recruiting and retiring as social moments, decoration, town juice and interface (the caravan is a proposal) |
| [travel.md](travel.md) | Journeys: the owner's eight travel rules, the tile map, purity and danger, routes, travel days, road events, the guaranteed boss, failure, and starting small |
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
- The game is original. These files take principles from studying other games,
  never names, content, or specific designs.
