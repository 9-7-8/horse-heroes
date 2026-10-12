# Tactics

How a player tells a horse what matters in a fight, without fighting it for
them. The short version:

> Horses have competent default behaviour. Players customise a few
> exceptions and priorities, not the entire process of fighting.

The reward is "my team understood my plan", not "I finally fixed the AI".

**All of this is a proposal.** The source is advice the owner handed over,
not her design, and nothing here is confirmed. The questions are on #338.

## Principles

### One sentence the player can hold

> At each action opportunity, check from top to bottom and use the first
> action that has a valid target and can be performed.

That sentence must also be the implementation.

- A rule is four things: a condition, an action, a target and its place in
  the order.
- **A false condition, no target, a cooldown or a missing resource all mean
  "try the next".** A blocked rule never freezes a horse.
- **Order is the "else".** There is no separate else to edit; a fixed last
  line says that normal behaviour takes over.
- **The next decision starts at the top again.** The list is a policy for
  choosing, not a script that runs every line.
- **Decide at action boundaries.** Not every frame, and an edit never
  cancels an action under way.
- **A rule grants nothing.** It cannot skip a cooldown, invent a resource
  or make an illegal target legal.

### Bind the condition to the target

A rule that notices a primed enemy must act on that enemy. Find the
creatures that match, keep those the action may legally target, choose one,
act. Two conditions on "an ally" must be about the same ally. The worst
failure of a rule editor is a sentence that looks right and is read
differently by the game.

### Defaults first

- **Equipping an ability gives it sensible behaviour at once.** A new
  ability is never an unpaid configuration task.
- **Customisation is an exception layer.** A few priorities above normal
  behaviour, never a blank list to fill.
- **Three levels over one system:** do nothing; state an intent with a
  preset and a few preferences; edit the cards. A changed preset says
  Custom. There is one source of truth, and one summary a player can read
  to know what a horse is set to do.
- **Mastery and relaxation must coexist.** If every new enemy needs the
  rules rewritten, it has stopped being idle. Prefer conditions that are
  broadly useful; keep encounter-specific ones as optional tools.

### Depth is not configuration

Complexity is what must be learned, remembered, set up and repaired. Depth
is how much worthwhile experiment the system holds. More options are not
more depth.

- The test for an option: **does it let the player express a different
  strategy, or only repair behaviour that should already work?** If it
  repairs, move it into the defaults.
- **Automate legality and routine competence; expose intent.**
- **Put the depth in the party, not in each list.** One stable rule grows
  richer as the horses, gear and pets round it change.
- **Tactics express a build's strengths; they do not manufacture them.** If
  clever rules make gear irrelevant, tactics has swallowed the game. If
  gear decides everything, the editor is busywork.
- **A choice needs a real cost**, from constraints that already exist:
  action opportunities, cooldowns, resources, targets. No obscure
  penalties. If healing is free, a healing threshold is cosmetic.
- **Constrain the vocabulary before the player.** One action, one target,
  one condition and perhaps a second joined by "and". No nesting,
  variables, loops, sequences or commands to another character. If players
  need five rules to say one simple thing, the vocabulary is missing a
  word; more slots only lengthen the workaround.

### Priority, restriction and waiting are three things

- "Prefer the Finisher on primed enemies" sets an order.
- "Only use the Finisher on primed enemies" holds the ability back, even
  from normal behaviour, and should say what that may cost.
- "Wait up to two seconds for a Primer" is a third strategy. Waiting is a
  stated choice with a bound, never an accident.

Do not offer wording that implies one of these until it is built.

### Explain, do not only animate

A player trusts automation when it says why it acted.

- **Three depths:** a small label for a notable action; a trace that can be
  opened; a summary after the journey.
- A trace says, for each rule: selected, skipped and why, or **not
  evaluated**. A rule that was never reached was not wrong.
- **Report what can be acted on.** "Skipped 12 times: on cooldown" gives
  something to try. "Efficiency: 71%" does not.
- **Warn about definite faults only**, such as an ability no longer
  equipped. A rule that seldom fires may be an emergency rule doing its job.
- **Do not make a notification job.** An idle game must not make leaving
  the window feel irresponsible.

### Juice says the plan worked

Spend the emphasis where a decision paid off: the Primer appears, the
partner finishes, the marker is consumed, the payoff lands; an ally is in
danger, the healer answers, the ally recovers. Routine actions stay quiet,
and farming turns the effects down. If a combination shows only as a bigger
number, the player enjoys the hit and never learns their rule caused it.
Hit-stop or slow motion must not change the simulation's time unless that
is decided on purpose, and watched and unwatched outcomes must agree.

### The editor is a list of sentences

The logic is an ordered list, so the menu is an ordered list. Not a node
graph, not a block-programming canvas.

- **A card reads as a sentence** and includes its target: "when an ally is
  below 35% health, heal the lowest-health eligible ally."
- **Its words are controls** that open constrained choices. Nothing is
  typed: no ability names, no rule names. Names are generated.
- **Start from an ability or an intent**, never an empty expression, and
  arrive with a valid, prefilled sentence.
- **Show only what suits the ability**: recognition, not recall. One shared
  system underneath, never a different editor per ability.
- **Pick the control for the decision**: preset buttons and a stepper for a
  threshold; move buttons for order, with dragging as a shortcut.
- **The mental model is always on screen**: "checked from top to bottom,
  the first usable priority wins" above; "otherwise, normal behaviour"
  below.
- Text labels, visible focus, a key for everything, no meaning by colour
  alone.

### Structured rules, generated sentences

The editor writes structured data: stable identifiers for the ability, the
condition, the comparison, the number, the target filter, the target choice,
enabled, and order. The sentence is generated from the data and never
parsed back. One registry of conditions says what each can be about, its
value, its control and its explanation, so the menu cannot offer what the
evaluator cannot run.

### Draft, test, apply

- Edits are a draft while the party carries on; "draft saved" and "active"
  are told apart; applying says when it takes effect.
- Undo, discard and reset to the preset are always there.

### Cheap experiments, honest comparisons

Change one thing, test, see what changed, keep or undo. A repeatable fight
on a copy of the party, steppable, with the chosen rule lit. Compare more
than damage: time, who was standing, what was used up, reactions, blocked
abilities. Show a trade-off, not a score. Keep the seed fixed, or run
several; one lucky fight proves nothing. A fight that ends before an
ability is ready cannot teach a rule about it.

### Count the whole party

Three rules a horse is fifteen across five horses. Give presets, a one-line
intent for each horse, and copying that keeps the intent where the other
horse has a compatible ability and says plainly what it could not carry.

### One evaluator, watched or not

The same rules run in a fight, in a test and for time away. Time away is
settled with the tactics that were active before a new edit applies.
Anything costly is opt-in and bounded.

### Teach one change, after the defaults have earned trust

Never open on a blank editor. Let the party win on its defaults, show one
cooperation, offer one useful change, and let the next fight show it. It
can be skipped and the preset still works.

## For Horse Heroes

- **Today there is nothing to prioritise.** Each horse has one ability,
  fired on cooldown (`docs/expedition.md`). Tactics needs a horse with more
  than one equipped ability (#222) and the default choice among them (#93,
  #97), which are 0.6.1 and 0.3.3.
- **The default is already specified as cooperative.** #93 and #97 give a
  Finisher on a primed target, then a Primer, then the best match-up, with
  a brief bounded wait. Tactics sits above that and falls back to it; it
  must never be needed to make it work.
- **It revises [build-laboratory.md](build-laboratory.md)'s one rule per
  ability** (#22 as first filed). The principle there stands, "a short list
  of rules is play, a scripting language is work"; the shape changes from a
  rule on each ability to an ordered list on each horse.
- **Stance, focus and recovery routing** (#24, #25) overlap with a preset's
  preferences. Whether they fold in is hers to say (#338).
- **The road rule may disagree with the note.** Gear, ability, pet and look
  changes made on the road queue until the next town
  ([vision.md](vision.md)); the note applies a tactics edit at the next
  decision. Asked on #338.
- **Health potions are used automatically** as filed (#177).
  The note's opt-in for consumables is advice against that rule and does
  not override it.
- **A Tactics tab would be a fifth tab on the horse**, beside Gear,
  Abilities, Pet and Look ([menus.md](menus.md)). Menus first (pillar 5)
  says a menu is built early with its buttons in place; whether an empty
  tab stands before the system exists is on #338.
- **The words exist already.** Primer, Finisher, Support, the three
  elements, the four enemy kinds and the three impairments are the
  condition vocabulary; nothing needs inventing.
- **The sim is deterministic and has no rendering**, so one evaluator for
  watched, test and time-away fights is the natural form, and a same-seed
  comparison is cheap. Time away is capped at 8 hours.
- **The trail report is where the summary goes** (#51); it already means to
  name the settings in force.
- **Pillar 2 is the constraint that matters.** Every horse works with
  nothing set, and a custom priority makes the party better, never merely
  able.
- **Where it stands against pillar 1.** It makes a drop matter, by letting
  a build say what a new ability is for. It does not make a drop feel
  better.
- **The work** is the epic #337, a part of the build laboratory (#20).

## Source

A design note the owner handed over on 2026-10-11: five answers on
priority-rule systems for an idle RPG. It describes the macro system of
Siralim Ultimate, then proposes a tactics layer through three lenses
(experience first, usability, juice), how to manage its complexity, and a
sentence-based editor. It ends complete and is kept unedited as
`docs/source/tactics.md`, with its citations.
