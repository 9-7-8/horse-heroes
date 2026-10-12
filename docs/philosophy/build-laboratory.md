# The build laboratory

What an idle loot game keeps from an action RPG, and what it must replace.
The short version:

> A build laboratory wrapped around a rewarding automated expedition loop.

Automated combat is not the content. It is the **test bench** for the player's
build decisions.

**Incomplete.** The source file was cut off in its section on timed content.
Extend this file when the rest arrives.

## Principles

### The five promises

Every system should serve at least one. A system that serves none is breadth.

1. My character becomes dramatically stronger.
2. A drop might change my build.
3. I have a clear reason to do another run.
4. My choices still matter even when combat is automated.
5. I can see the consequences of my build.

### Move the value from the active layer to the meta layer

An action RPG spreads skill across two layers. The active layer (moving,
dodging, aiming, timing, reacting) does not survive automation and should not
be faked. The meta layer (skills, gear, powers, sets, activity and difficulty
choice, finding the build's weak point, farming a specific upgrade) is where an
idle game lives.

The player's meaningful decisions happen:

- before the expedition;
- at major decision points during it;
- after it, when evaluating drops;
- when choosing the next target, zone, boss or difficulty;
- when building around a powerful new item.

### Reinterpret, do not automate

A system whose value came from execution loses its purpose if it is simply run
by the computer. Give it a new job on the meta layer.

| In an action game | In an idle game |
| --- | --- |
| Timing a cooldown | A **policy** the player chooses: "save for the boss", "below 50% health", "when 3 or more enemies are alive" |
| Positioning and dodging | **Formation, targeting and stance**: who is in front, what to focus, aggressive or defensive |
| Running to a health pickup | **Recovery routing**: when stored healing is spent, and what else it triggers |
| Reacting to enemy modifiers | **Preparation**: the modifier is shown before the run and the build is checked against it |
| Racing a timer | An **efficiency test** for some content, never a clock on everything |

- **Never reduce every skill to "cast on cooldown".** That is a solved system
  and makes skills blur together.
- **Keep policies readable.** A short list of rules per skill is play. A
  scripting language is work.
- **Stances and policies are build decisions**, and the player must be able to
  recognise them as such. Opaque AI behaviour teaches nothing.
- **Show the policies in the results.** If the player picks "auto" and every
  build behaves alike, automation has removed the strategy. A report that
  names the settings in force lets an outcome be traced to a choice.
- **A modifier is a question put to the build**: is it suitable, should a skill
  change, should the tier drop, is the reward bonus worth it?

### Bounded runs

A good expedition has a known purpose, a limited length, variable enemies and
events, a progress meter, a completion point, a reward report, and a reason to
adjust the build before repeating it.

    choose a zone -> choose a difficulty -> choose build and policies
      -> run (watched or offline) -> encounters, branches, boss
      -> a concise loot report -> adjust -> go again

### Difficulty is a choice, not a ladder

"Can I handle the next tier?" gives an upgrade concrete value in a way a power
score does not. Higher tiers pay more (rarity weight, materials, special
encounters, new item pools).

But the highest unlocked tier must not always be the right answer. There
should be reasons to pick a fast farming tier, a safe offline tier, a
boss-target tier, a risky push tier, or a tier for testing a build.

### Targeted farming

The hunt is stronger when the player can plan around one missing piece. The
thought to produce is "I need that piece for this build, so I will run that
place", not "I will run whatever pays the most generic currency".

Tools: zones that favour a gear family, bosses with small unique drop pools,
contracts for a slot or type, salvage that becomes a currency for a controlled
reward pool, duplicate protection after real progress, and a codex that shows
which item families exist without listing every drop rate.

## For Horse Heroes

- The trail is the test bench. Camp is the laboratory, and today camp offers
  two decisions (element, equip). Most of the missing game is in camp.
- This does not outrank pillar 1. Build systems make a drop **matter**; the
  fountain makes it **felt**. A build feature waits behind fountain work
  unless it makes a drop more worth getting.
- Pillar 2 bounds every policy: each has a sensible default so the game still
  plays itself. Choosing a policy makes the party better, never merely able.
- The three abilities currently fire on cooldown. That is exactly the "solved
  system" above, and with one ability per horse it costs little now. It should
  not survive the arrival of a second ability or a gear effect that keys off
  timing.
- The trail level is the difficulty tier. It only goes up on a win and cannot
  be chosen, so there is no farm-versus-push decision yet.
- Enemy elements are already a small modifier the player answers before
  setting out. Trail modifiers extend the same idea.
- The fairy spring is the nearest thing to a health pickup. It is where
  recovery routing would start.
- A party of three makes formation and focus real choices without inventing a
  movement system.
- **A later note proposes the shape policies should take**: not one rule on
  each ability but a short ordered list of priorities on each horse, over
  default behaviour that already works ([tactics.md](tactics.md), #337).
  "A short list is play, a scripting language is work" is the limit it
  keeps. Proposed, not decided (#338).

## Source

Owner's notes on which loot-and-build systems carry over from a reference
action RPG to an idle game, received 2026-10-09 (purpose, sections 1, 2.5 to
2.8 and 3.1 to 3.5; the file ends mid-sentence in 3.5).
