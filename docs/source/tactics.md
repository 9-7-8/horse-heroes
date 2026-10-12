Yes—Siralim Ultimate has exactly that kind of system, called **macros**. It includes an in-game rule builder that lets you tell a creature what to do based on conditions such as health, buffs or debuffs, dead allies, and turn counts. No external scripting program is required. My earlier Nevergrind answers were about a different kind of automation. [youdonthaveto.substack](https://youdonthaveto.substack.com/p/siralim-ultimate)

## How the rules work

A macro normally behaves like a prioritized list of conditional instructions:

```text
IF condition A is true → perform action A
ELSE IF condition B is true → perform action B
ELSE IF condition C is true → perform action C
```

The game checks the lines in order. When a line qualifies and selects an action, that ends the normal decision sequence for that invocation of the macro. You therefore get “else-if” behavior through the order of the lines, without writing a literal `else` statement. [reddit](https://www.reddit.com/r/Siralim/comments/zsm0ip/what_is_a_good_else_replacement_in_macros/)

For example, a community macro uses this priority:

| Priority | Condition | Action |
|---|---|---|
| 1 | An ally lacks a barrier | Cast Holy Bulwark |
| 2 | An ally has less than 30% health | Cast Breath of Life on that creature |
| 3 | At least one enemy is alive | Defend |

With that ordering, supplying a barrier takes priority over healing. Moving the healing rule above the barrier rule changes the creature’s priorities. The final line serves as the fallback when the earlier conditions are false. [reddit](https://www.reddit.com/r/Siralim/comments/zsm0ip/what_is_a_good_else_replacement_in_macros/)

## How you build and use one in-game

You assemble rules through the macro builder’s predefined choices rather than typing a general-purpose programming language. The available conditions cover things such as:

- A creature’s health.
- Whether creatures have particular buffs or debuffs.
- How many allies are dead.
- Turn counts.

Rules can choose different actions, spell casts, and targets. The developer’s 2.0 patch notes also explicitly document adding a `Health < 100%` condition. [youdonthaveto.substack](https://youdonthaveto.substack.com/p/siralim-ultimate)

You assign the macro to a creature. On that creature’s turn, choosing the Macro command makes it use those rules to decide its action. It does not inherently remove your ability to choose actions manually. You can also set Macro as the default action and use the game’s turbo/input controls to speed up repeated execution. [steamcommunity](https://steamcommunity.com/app/1289810/discussions/0/4702413966613159824/?l=latam)

## How close is it to a programming language?

It is closer to a small, visual combat-decision language than to Python or Lua: you configure conditions and actions within the game’s supported vocabulary. The important programming concept is priority—specific or urgent cases go above broad fallback cases. [youdonthaveto.substack](https://youdonthaveto.substack.com/p/siralim-ultimate)

There are also more advanced options, including a “test next line” instruction. One caveat is that successive checks involving “any ally” do not necessarily refer to the same ally: checking that an ally lacks a harmful debuff, then checking that an ally has low health, can match two different creatures. That distinction matters when designing safe healing rules. [steamcommunity](https://steamcommunity.com/app/1289810/discussions/0/4702413966613159824/?l=latam)

So yes: Siralim Ultimate’s macros are the in-game “if this situation, try this action; otherwise try the next rule” system you were describing. [youdonthaveto.substack](https://youdonthaveto.substack.com/p/siralim-ultimate)
---
Generally, this is a **priority-rule** system: the player defines an ordered list of “when this is true, do this” instructions, and the game repeatedly uses that list to choose a character’s next action.

In an idle game, the same system runs automatically instead of waiting for the player to press a Macro button. The player’s gameplay becomes configuring a strategy, watching how it performs, and adjusting it—not manually selecting every attack.

The underlying idea resembles a priority selector in game AI: try the highest-priority option, then move down the list until one works.  The design below is a general proposed implementation, not a claim about Siralim Ultimate’s exact internal code. [gamedeveloper](https://www.gamedeveloper.com/programming/behavior-trees-for-ai-how-they-work)

## 1. A rule has four parts

A useful rule structure is:

> When [condition], use [action] on [target], at [priority].

| Part | Purpose | Example |
|---|---|---|
| Condition | Determines when the rule applies | An ally has less than 35% health |
| Action | Specifies what to attempt | Use Healing Light |
| Target selection | Specifies who receives the action | The lowest-health eligible ally |
| Priority | Determines which rule gets considered first | Above damage-dealing rules |

The action’s ordinary restrictions still apply. A rule does not grant extra mana, bypass cooldowns, or make an enemy a valid healing target.

For a proposed implementation, I would make a rule eligible only when both its condition and its action requirements are satisfied.

## 2. The game chooses one action at a time

Suppose a healer has these rules:

| Priority | Condition | Action and target |
|---|---|---|
| 1 | An ally is dead | Revive a dead ally |
| 2 | An ally has less than 35% health | Heal the lowest-health ally |
| 3 | An ally has a removable harmful status | Cleanse that ally |
| 4 | An enemy is vulnerable to light damage | Use a light attack on that enemy |
| 5 | Otherwise | Basic attack |

Each time the healer is allowed to act, the proposed evaluator would:

1. Read the current combat state.
2. Check rule 1.
3. Find an appropriate target.
4. Check whether the action is usable.
5. If it is usable, select that action and stop checking.
6. Otherwise, try rule 2, then rule 3, and so on.

That “try options in order until one succeeds” structure is the defining behavior of a priority selector. [gamedeveloper](https://www.gamedeveloper.com/programming/behavior-trees-for-ai-how-they-work)

For example, if an ally is dead but Revive is on cooldown, the healer moves on to the healing rule. It does not freeze merely because the first rule’s condition is true.

After the selected action resolves, the next decision starts from the top again. The list is a policy for choosing actions, not a script that must execute every line sequentially.

### Priority creates the equivalent of “else”

The list effectively means:

```text
If I can revive someone, revive them.
Otherwise, if I can heal someone who needs it, heal them.
Otherwise, if I can cleanse someone, cleanse them.
Otherwise, try an appropriate attack.
Otherwise, use the fallback.
```

Changing the order changes the strategy. A healer that cleanses before healing behaves differently from one that prioritizes keeping critically injured allies alive.

## 3. Conditions and targets must stay connected

One important design detail is ensuring that the action targets the creature that satisfied the condition.

Consider:

> If an enemy has a Plant primer, use a Fire finisher.

A poorly specified implementation could notice that enemy A has the primer but attack enemy B. The condition would be true, but the intended combination would fail.

A clearer rule is:

> Find enemies with a Plant primer → choose one of those enemies → use a Fire finisher on it.

I would give the rule builder explicit target choices:

- The creature that matched the condition.
- Lowest-health matching ally.
- Highest-health matching enemy.
- Matching enemy with the most primer charges.
- Self.
- All valid targets, for an area ability.

Similarly, multiple conditions should make their scope clear:

> Heal an ally who has less than 35% health AND is not affected by a healing-blocking status.

Both checks should apply to the same ally—not one injured ally and a different ally who can receive healing.

## 4. How it becomes an idle-game system

The main change is who triggers evaluation.

| Aspect | Player-triggered macro | Idle-game policy |
|---|---|---|
| Decision trigger | Player chooses Macro | Character becomes ready to act |
| Player involvement | Can choose actions turn by turn | Configures rules beforehand |
| Evaluation frequency | On each requested macro execution | On every appropriate decision opportunity |
| Main feedback | Result of the chosen action | Performance across battles and expeditions |

For a cooldown-based idle combat system, I would use this loop:

```text
Character becomes ready
    ↓
Evaluate rules from highest to lowest priority
    ↓
Choose the first usable action and valid target
    ↓
Execute the action
    ↓
Apply effects, costs, and cooldowns
    ↓
Wait until the character can act again
```

For a turn-based system, “becomes ready” simply means “its turn begins.”

I would not reevaluate every render frame or constantly replace a spell halfway through casting. Start with decisions at action boundaries; add interruption rules only if interruption is an intentional mechanic.

## 5. The player designs behavior rather than individual moves

The idle gameplay loop would be:

> Configure party → configure behavior → run battles → inspect results → revise the build or rules.

The interesting choice is not just “which ability is strongest?” It becomes:

- Should healing happen early or only during emergencies?
- Should a finisher wait for a primer or deal damage immediately?
- Should a powerful skill be saved for a boss?
- Should the party focus one enemy or spread status effects?
- Should scarce consumables be used automatically?

These are proposed strategic choices. You do not need to expose all of them in the first version.

### Example: elemental cooperation

For a hypothetical party using primers and finishers:

| Character | Priority | Rule |
|---|---|---|
| Plant specialist | 1 | If an enemy lacks a Plant primer, apply one to that enemy |
| Plant specialist | 2 | Otherwise, basic attack |
| Fire specialist | 1 | If an enemy has a Plant primer, use a Fire finisher on that enemy |
| Fire specialist | 2 | Otherwise, use a normal attack |

The two characters cooperate without a single hardcoded “perform combo now” command. Each follows its own policy, and the shared combat state connects their decisions.

However, I would make waiting an explicit option rather than an accidental consequence. “Wait up to two seconds for a primer” is a different strategy from “attack normally until a primer appears.”

## 6. Defaults and explanations matter more than elaborate syntax

For an idle game, I would start with a rule-card interface rather than a text programming language:

```text
WHEN: An ally’s health is below [35%]
DO:   [Healing Light]
TO:   [Lowest-health matching ally]
```

Players could drag cards to change priority.

A practical initial design would include:

- Working default behavior for every character.
- A small editable rule list.
- An automatic fallback if no custom rule works.
- Presets such as Support, Aggressive, or Combo-focused.
- A test battle.
- Explanations for selected and skipped rules.

For example:

```text
Revive skipped: no dead allies.
Heal skipped: ability on cooldown.
Cleanse selected: ally has Poison.
```

This feedback tells the player whether the problem is their strategy, ability availability, or target selection.

I would make custom rules optional. A player who enjoys party composition and equipment should still have a functional team without programming every character.

## 7. Offline progression needs to respect the policy

An idle game must decide how it handles these rules while closed.

Some games calculate elapsed time when the player returns and simulate the activity that would have occurred. Melvor Idle documents this approach, including offline combat when enabled.  Other designs use a simplified model: Clicker Heroes describes offline progress based on idle damage per second rather than replaying player-driven abilities. [wiki.melvoridle](https://wiki.melvoridle.com/w/Offline_Progression)

For a game where conditional combat behavior is a major feature, I would prefer:

> Run the same combat rules offline, without rendering animations, and process elapsed simulation time faster than real time.

Otherwise, a carefully configured healer or elemental-combination strategy might matter while the window is open but disappear from the offline calculation.

That recommendation has a cost: complex rules and long offline periods require more simulation work. You can cap simulated time, optimize the evaluator, or introduce approximations—but any approximation should preserve the strategies players are being asked to design.

## 8. Keep the first version deliberately small

My suggested starting scope would be:

- One ordered rule list per character.
- Conditions for health, statuses, enemy count, and combo setup.
- Actions drawn only from equipped abilities.
- Explicit target matching.
- First usable rule wins.
- One default fallback.
- No arbitrary loops, variables, or nested scripts.
- A readable decision log.

That gives you a system with meaningful tactical depth while keeping the player-facing idea simple: describe the situations that matter, put them in priority order, and let the party follow those instructions automatically.
---
Implement this as an optional **tactics** layer: characters fight competently by default, while players can customize a few priorities to make their party behave the way they intend. The reward should be “my team understood my plan,” not “I finally fixed the AI.”

For your idle RPG, I would connect three design lenses:

- MDA determines which rules produce the experience you want.
- UX makes those rules easy to configure, understand, and trust.
- Juice makes successful decisions and cooperation visibly satisfying.

Below is a proposed design, not a description of features already implemented in your game.

## 1. Use MDA to start with the player’s experience

MDA distinguishes mechanics—the rules and algorithms—from dynamics—the behavior that emerges during play—and aesthetics—the emotional experience. “Aesthetics” here means more than visual style. [users.cs.northwestern](https://users.cs.northwestern.edu/~hunicke/MDA.pdf)

For this system, start with these intended experiences:

> “I understand what my party is doing.”  
> “My preparation mattered.”  
> “These characters work well together.”  
> “I can leave them alone without worrying.”

Then work backward.

| Desired experience | Dynamic you want | Mechanics that support it |
|---|---|---|
| Confidence | The party behaves predictably | Consistent evaluation order, valid targets, reliable fallback |
| Mastery | A small adjustment produces an understandable improvement | Editable priorities, repeatable test encounters, decision explanations |
| Expression | Different parties support different strategies | Role presets, ability-specific conditions, target preferences |
| Delight in cooperation | One character creates an opportunity another exploits | Shared status information and explicit combo rules |
| Relaxed engagement | The party operates without constant intervention | Good defaults, persistent policies, offline consistency |

The critical design constraint is that mastery and relaxation must coexist. Players should be able to optimize behavior, but they should not need to supervise it.

### Avoid turning automation into maintenance

If every new enemy requires rewriting the party’s rules, the system stops feeling idle.

Prefer broadly useful conditions:

- Ally needs healing.
- Enemy has an exploitable status.
- Several enemies are present.
- Ability is available.
- A high-priority target is present.

Use encounter-specific rules sparingly. Make them optional tools for specialization rather than prerequisites for ordinary progression.

## 2. Keep your existing player role intact

For your horse-party game, present tactics as preferences for how the gaggle cooperates—not as manual combat commands.

The player still makes decisions about party composition, equipment, pets, and expeditions. Tactics should refine those decisions rather than replace them with a separate programming game.

A useful interface hierarchy would be:

```text
Horse
 ├─ Equipment
 ├─ Abilities
 ├─ Pet
 └─ Tactics
```

Equipping an ability should immediately give it sensible default behavior. Players can then customize that behavior if they want.

For example:

> Equip a healing ability → the horse automatically uses it when appropriate.  
> Open Tactics → change how urgently healing takes priority.

That lets a new ability deliver value immediately, instead of arriving with an unpaid configuration task.

## 3. Build three levels of control

Progressive disclosure means showing common, important options first and placing advanced options in a secondary interface. It helps make a system easier to learn without removing depth. [nngroup](https://www.nngroup.com/articles/progressive-disclosure/)

I would use three levels.

### Level 1: Choose a preset

Examples:

- Balanced
- Support
- Aggressive
- Combo-focused

Each preset should explain its behavior:

> Support: prioritizes emergency healing and cleansing before damage.

Do not require players to infer the difference from a name or icon.

### Level 2: Adjust a few preferences

Expose understandable controls such as:

- Healing threshold.
- Preferred target.
- Whether finishers prioritize prepared enemies.
- Whether consumables are permitted.

These can modify the preset without exposing every underlying rule.

### Level 3: Edit rule cards

Advanced players can open the ordered list:

```text
1. WHEN an ally is below 35% health
   USE Healing Light
   ON the lowest-health matching ally

2. WHEN an enemy has a Plant primer
   USE Fire Burst
   ON a matching enemy

3. OTHERWISE
   USE default behavior
```

The levels should represent the same behavior system, not three competing systems. If a player customizes the rules, label the preset “Custom” rather than continuing to imply that it is unchanged.

## 4. Make the evaluator simple enough to explain in one sentence

The player-facing rule should be:

> At each action opportunity, check from top to bottom and use the first action that has a valid target and can be performed.

That sentence should accurately describe the implementation.

I recommend these semantics:

| Situation | Behavior |
|---|---|
| Condition is false | Try the next rule |
| No matching target exists | Try the next rule |
| Ability is on cooldown | Try the next rule |
| Resources are insufficient | Try the next rule |
| Action is valid | Execute it and end this decision |
| No custom rule works | Use default behavior |

Keep deliberate waiting separate:

> Wait for a combo opportunity for up to two seconds.

Without an explicit wait rule, a blocked high-priority ability should not freeze the character.

For the initial version, choose actions at action boundaries. Do not let ordinary rule changes repeatedly cancel casts or animations.

### Bind conditions to their targets

A rule that detects a primed enemy must attack that matching enemy, not an unrelated enemy.

Represent this internally as:

```text
Find matching candidates
    → Filter for action legality
    → Select a target
    → Execute
```

This avoids a particularly frustrating failure: the interface appears to express the correct strategy, but the game interprets it differently.

## 5. Give players explanations, not just animations

Visibility of system status supports understanding and trust: users need to know what the system is doing and how their input was interpreted. [nngroup](https://www.nngroup.com/articles/visibility-system-status/)

For automation, that means showing why a character acted.

I would provide three levels of feedback:

| Feedback | Purpose | Proposed presentation |
|---|---|---|
| Immediate | Explain the current action | Small “Emergency heal” or “Combo opportunity” label |
| Inspectable | Diagnose a surprising choice | Expandable decision trace |
| Summary | Identify recurring issues | Post-expedition tactics report |

An inspectable trace could show:

```text
Revive — skipped: no defeated allies.
Heal — skipped: ability on cooldown.
Cleanse — selected: ally has Poison.
Attack — not evaluated.
```

“Not evaluated” matters. A lower-priority rule was not necessarily wrong; an earlier rule already selected the action.

### Surface actionable problems

Useful report:

> Fire Burst was skipped 12 times because there was insufficient mana.

Less useful report:

> Efficiency: 71%.

The first gives the player a concrete hypothesis to test. The second creates a score without explaining what to do.

Warn about definite problems—such as a rule referencing an unequipped ability—but avoid claiming that a rare or unused rule is automatically bad. An emergency revive rule may be valuable precisely because it seldom triggers.

## 6. Use juice to communicate the plan succeeding

Juice is the feedback and exaggeration that makes an action satisfying. It should remain readable and respect reduced-motion preferences. [egmatic](https://egmatic.com/blog/how-to-make-your-game-feel-good)

For this system, direct most of the polish toward meaningful events rather than every routine action.

### Give cooperation a readable sequence

For a proposed Plant → Fire interaction:

1. A Plant ability places a recognizable primer marker on an enemy.
2. The Fire specialist selects that enemy.
3. The finisher consumes the marker.
4. A distinctive reaction effect and sound confirm the combination.
5. An optional small label identifies both contributors.

That sequence communicates cause and effect:

> This horse prepared the opportunity; that horse completed it.

If the reaction only appears as a larger damage number, the player may enjoy the hit without understanding that their configuration caused it.

### Use an emphasis budget

| Event | Suggested intensity |
|---|---|
| Routine basic attack | Minimal |
| Routine rule selection | Subtle |
| Successful elemental reaction | Distinct, brief |
| Emergency save | Stronger emphasis |
| First demonstration of a new strategy | Optional explanatory highlight |
| Repeated farming | Reduced effects |

Provide controls for screen shake, flashes, floating text, and effect density. Keep battle effects out of configuration panels.

Do not silently change simulation timing to make an effect feel stronger. If you add hit-stop or slow motion, decide explicitly whether it affects simulation time or only presentation, and keep online/offline outcomes consistent with that choice.

## 7. Teach one meaningful change after defaults have earned trust

Do not open with a blank macro editor.

A proposed onboarding sequence:

1. Let the party complete an encounter using defaults.
2. Show one readable cooperative event.
3. Introduce a useful customization opportunity.
4. Offer one change, such as prioritizing a finisher against primed enemies.
5. Demonstrate the resulting decision in a suitable encounter.
6. Leave the advanced editor available without making it mandatory.

The demonstration must contain the conditions needed for the rule to matter. A fight that ends before an ability becomes ready cannot teach cooldown-based tactics.

Allow players to skip the lesson and retain a working preset.

## 8. Provide a safe place to experiment

A test arena would make the system much easier to learn.

I would start with:

- A repeatable encounter.
- Pause and single-action stepping.
- The selected rule highlighted.
- A before/after comparison.
- Undo and reset-to-default.
- A copy of the party state that does not consume live resources.

Undo, clear exits, and recovery from mistakes align with UX principles of user control and freedom. [nngroup](https://www.nngroup.com/articles/ten-usability-heuristics/)

Compare more than damage:

- Completion time.
- Survival.
- Resource consumption.
- Reaction frequency.
- Unused or frequently blocked abilities.

For fair comparisons, keep the encounter, equipment, and starting resources fixed. Where randomness matters, use the same seed for a diagnostic comparison or several trials; do not present one lucky battle as proof of an improvement.

## 9. Treat unattended and offline behavior as part of the promise

The system should not work one way while watched and another way when the player leaves.

I would use one tactics evaluator for:

- Normal combat.
- Accelerated testing.
- Offline simulation.

Presentation can differ; decision semantics should not.

Save the policy with the party state. When the player returns, settle the elapsed offline period using the saved policy before applying newly edited rules to future actions.

Also make potentially costly automation explicit:

- Consumable use is opt-in.
- Scarce items can have reserve limits.
- Retreat behavior is visible.
- Invalid custom rules fall back safely.
- The return report explains material outcomes.

This is where trust becomes more important than spectacle.

## 10. Build a small vertical slice and test the experience

My recommended first prototype:

| Area | Initial scope |
|---|---|
| Party | A few characters with distinct roles |
| Rules | A short list per character |
| Conditions | Health, statuses, enemy count, primer presence |
| Actions | Equipped abilities only |
| Interface | Presets plus editable cards |
| Feedback | Selected-rule highlight and decision trace |
| Encounter | One ordinary fight and one cooperation-focused test |
| Safety | Default fallback, undo, no accidental consumable use |

Avoid variables, nested branches, arbitrary code, and cross-character scripting until simple priorities prove insufficient.

### Evaluate it through all three lenses

| Lens | Playtest question | Evidence to observe |
|---|---|---|
| MDA | Does customization create mastery without demanding supervision? | Players make deliberate changes, then comfortably leave the party running |
| UX | Can players predict and explain the next action? | They understand rule order, targets, and skipped actions |
| Juice | Can players notice when their strategy succeeds? | They recognize the setup and payoff without consulting a log |
| Idle suitability | Does the system remain useful unattended? | Defaults function and offline results match the stated behavior |
| Build depth | Are several approaches viable? | Players achieve different survival, speed, and resource tradeoffs |

The strongest implementation makes configuration brief, behavior legible, and improvement satisfying. Players should spend their attention deciding what kind of team they want—not repeatedly repairing the team’s ability to function.
---
Manage complexity by giving players a small number of understandable **decisions**, while letting those decisions interact with party composition, abilities, equipment, and enemy behavior. Do not make depth depend on writing longer scripts.

For your idle RPG, my strongest recommendation is:

> Characters have competent default behavior. Players customize a few exceptions and priorities—not the entire process of fighting.

That preserves the relaxed idle experience while giving enthusiasts room to optimize. The following is a proposed design direction, not a set of finalized mechanics.

## 1. Separate strategic depth from configuration work

Use this practical distinction when evaluating features:

- Complexity: what the player must learn, remember, configure, or troubleshoot.
- Depth: how much worthwhile experimentation and adaptation the system supports.

More options do not automatically create more depth. A large system can require substantial effort merely to understand what a decision will do, rather than offering rewarding strategic learning. [decisionspacepodcast](https://www.decisionspacepodcast.com/articles/going-deep-on-depth-in-games)

For example:

| Mostly configuration burden | More useful strategic depth |
|---|---|
| Specify whether an attack needs a living target | Choose which enemy to focus |
| Add a cooldown check to every rule | Decide whether to save an ability for a combo |
| Write a fallback for every character | Decide whether healing or damage takes priority |
| Configure basic behavior again after changing equipment | Explore how equipment changes the same strategy |
| Memorize ten status-specific exceptions | Choose whether cleansing takes priority over healing |

Automate action legality and routine competence. Expose decisions about intent.

A useful feature test is:

> Does this option let the player express a different strategy, or merely repair behavior that should already work?

If it mostly repairs behavior, move it into the defaults.

## 2. Make customization an exception layer over defaults

Rather than starting each horse with a blank rule list, give it default behavior based on its equipped abilities.

Players add a few instructions above that behavior:

```text
Custom priorities:
1. Heal allies below 35% health.
2. Prefer primed enemies when using a finisher.

Then:
Use normal behavior.
```

This is easier to engage with than asking the player to specify healing, targeting, cooldown checks, resource checks, damage selection, and fallback attacks separately.

### Define the boundary clearly

The interface should explain:

> Custom priorities are checked first. If none can be performed, this horse uses its normal behavior.

Also distinguish between two kinds of customization:

- Priority: “Try this before other actions.”
- Restriction: “Do not use this ability unless this condition is met.”

Those are not equivalent. “Prefer finishers on primed enemies” still allows ordinary use; “only use finishers on primed enemies” reserves the ability.

For the first version, I would expose priorities and only a few clearly labeled restrictions. Avoid a general-purpose language for everything.

## 3. Give depth to the party, not every individual rule list

Your game already has party composition, gear, abilities, and pets. Tactics should connect those systems—not become another equally large system competing for attention.

For example, one rule can create several build questions:

> Prioritize Fire finishers against Plant-primed enemies.

The player can then experiment with:

- Which horse applies the primer.
- How often that horse can apply it.
- Whether the finisher is single-target or area-based.
- Whether equipment favors frequent small reactions or rarer powerful ones.
- Whether a pet supports setup, survival, or resource recovery.

The rule remains understandable while the build changes around it.

That is the design direction I would favor: stable instructions whose consequences become richer as the party develops.

### Avoid making tactics invalidate other choices

If clever scripting is so powerful that equipment and party composition barely matter, tactics has swallowed the rest of the game.

Conversely, if gear determines everything and tactics never changes outcomes, the editor is busywork.

Aim for tactics to help a build express its strengths, not manufacture strengths the build does not possess.

## 4. Start with intent controls, then offer rules

Progressive disclosure can defer advanced options to a secondary interface. But it should not hide information players need to understand their current configuration. [nngroup](https://www.nngroup.com/articles/usability-heuristics-complex-applications/)

I would use this structure:

| Layer | Player task | Example |
|---|---|---|
| Default | Do nothing | The horse fights competently |
| Simple preferences | Express a broad strategy | Support-first; prefer prepared targets |
| Custom priorities | Specify a few exceptions | Heal below a threshold before attacking |
| Diagnostics | Understand an unexpected result | Why was this rule skipped? |

The advanced layer should be optional, not a mandatory destination every player eventually reaches.

### Keep one source of truth

Do not have a preset, several toggles, and a separate script all silently competing.

Instead:

- A preset supplies the behavior.
- Preferences visibly modify it.
- Custom priorities are shown above it.
- Restrictions are explicitly listed.
- Reset restores the preset.

A player should be able to read one concise summary and know what the horse is configured to do.

## 5. Constrain the rule vocabulary before constraining the player

A short rule list with enormous nested expressions can still be overwhelming. Restrict the grammar first.

For an initial version, I would allow:

- One action per rule.
- One explicit target selector.
- One main condition.
- An optional second condition joined with AND.
- Top-to-bottom priority.
- A built-in fallback.

I would postpone:

- Arbitrary nested AND/OR groups.
- Variables and counters.
- Commands that modify another character’s rules.
- Multi-action sequences.
- Exact simulation-time conditions.
- General-purpose code.

A prototype could start with three custom priorities per horse. Treat that number as a hypothesis to test, not a universal ideal.

If players repeatedly need five rules to express one simple idea, first ask whether the rule vocabulary is missing a useful abstraction. Adding more slots may only make the workaround longer.

## 6. Show relevant choices instead of a giant condition catalog

A healer does not need to browse every possible offensive condition when configuring healing.

For a selected healing ability, offer:

- Ally below a health threshold.
- Lowest-health ally.
- Self first.
- Ally who can receive healing.

For a finisher, offer:

- Matching primer present.
- Multiple valid targets.
- Boss or elite target.
- Prefer versus require a setup.

This supports recognition rather than recall: players choose from visible, relevant options instead of remembering terminology or syntax. [nngroup](https://www.nngroup.com/articles/recognition-and-recall/)

However, keep these as filtered views of a shared rule system. Do not give every ability an unrelated editor with different conventions.

### Make rules read like sentences

Prefer:

> When an ally is below 35% health, heal the lowest-health eligible ally.

Over:

```text
ALLY_ANY.HP_RATIO < 0.35
TARGET = MIN_VALID_HP
```

The sentence should include the target, because “when someone is injured” does not explain who receives the heal.

## 7. Create tradeoffs—not one universally correct configuration

Depth requires room for different priorities to be useful. For your prototype, I would design a few explicit tradeoffs:

| Choice | Potential benefit | Potential cost |
|---|---|---|
| Heal early | More safety | More resources spent, less damage |
| Heal late | More offensive uptime | Greater risk of defeat |
| Reserve a finisher | More reliable combinations | Lost damage opportunities while waiting |
| Use the finisher freely | Faster immediate damage | Fewer prepared reactions |
| Focus one enemy | Removes a threat sooner | Less setup spread across the group |
| Spread primers | Enables broader reactions | Delays eliminating individual enemies |

These consequences must actually exist in combat. If healing is free and never reduces offensive output, a healing threshold may be a cosmetic choice rather than a meaningful one.

Do not solve this by adding obscure penalties. Use existing constraints—action opportunities, resource costs, cooldowns, and target availability.

Also avoid requiring constant encounter-by-encounter changes. Broad profiles such as Safe, Fast, and Combo-focused should remain useful across many fights.

## 8. Make experimentation cheap and interpretation easy

Players should not have to change six settings at once and guess which one helped.

Provide a small experiment loop:

> Change one thing → test → see what changed → keep or undo.

A useful comparison might show:

```text
Earlier healing:
Fewer defeats
Slower clear time
Greater mana use
```

This presents a tradeoff rather than reducing everything to one “best build” score.

Show summaries first, details on request:

- What the horse prioritized.
- Which abilities were frequently blocked.
- Whether combinations occurred.
- Whether resources became a bottleneck.

Detailed traces belong in an expandable view. Keeping advanced information available without displaying all of it at once is consistent with staged disclosure for complex applications. [nngroup](https://www.nngroup.com/articles/usability-heuristics-complex-applications/)

### Avoid creating a notification job

Do not constantly prompt players to “improve” their rules.

Notify them about genuine configuration failures, such as an unavailable ability. Let optional reports identify optimization opportunities when the player chooses to inspect them.

An idle game should not make leaving the window feel irresponsible.

## 9. Use juice to reduce explanation burden

Feedback should help players recognize the consequences of their decisions.

For a cooperative attack, show a clear sequence:

> Primer appears → partner uses finisher → primer is consumed → reaction resolves.

For an emergency heal:

> Ally becomes endangered → healer responds → ally visibly recovers.

Reserve stronger audiovisual emphasis for those strategically meaningful events. Routine actions can remain understated.

The goal is not to animate every rule evaluation. It is to make the player’s plan recognizable in the fight.

Then provide a quiet explanation when needed:

> “Used Healing Light: ally below your healing threshold.”

This combines emotional payoff with diagnostic clarity without turning combat into a stream of labels.

## 10. Protect the player’s attention across the whole party

Three rules per horse can still become eighteen rules across six horses. Evaluate the whole-party workload, not just one editor.

I would provide:

- Role presets.
- Copyable profiles.
- A party summary of each horse’s intent.
- Context-aware validation when importing a profile.
- Automatic adaptation to routine equipment changes where possible.
- A clear indication of what needs review.

For example:

```text
Plant specialist — spreads primers
Fire specialist — prioritizes prepared targets
Support — heals early
Defender — prioritizes protective abilities
```

That overview is more useful for normal play than showing every condition simultaneously.

Do not blindly copy a rule that refers to an ability another horse lacks. Preserve the intent where a compatible ability exists; otherwise explain what could not be transferred.

## 11. Test whether you have depth or merely friction

For the first playtest, give players a functional preset and one optional adjustment.

Observe:

| Question | Healthy signal | Warning sign |
|---|---|---|
| Can they leave defaults alone? | The party functions without editing | Editing is necessary to stop obvious mistakes |
| Can they explain a rule? | They predict its action and target | They rely on trial and error to understand syntax |
| Does customization matter? | A change creates an understandable tradeoff | Changes have no visible effect |
| Is experimentation appealing? | They voluntarily try another approach | They avoid the editor because setup is exhausting |
| Can they leave the game running? | They trust the party unattended | They feel compelled to monitor it |
| Do different strategies work? | Several builds suit different goals | Everyone converges on one mandatory rule list |

My recommended starting point is competent defaults, a few intent controls, and roughly three optional custom priorities per horse. Expand only when testing reveals a valuable strategy players cannot express—not simply because the underlying engine can support more complexity.

That keeps the mental model small while leaving the party interactions deep.
---
I would build a **sentence-based** tactics editor: an ordered stack of rule cards whose editable phrases open constrained selection menus. Players choose abilities, conditions, and targets; the game generates the readable instruction. No code, typed ability names, or manually entered rule names are necessary.

For this particular system, I would prefer that over a node graph or a Scratch-style programming canvas. Your underlying logic is an ordered decision list, so the interface should look like an ordered decision list—not a general-purpose programming tool.

Recent UX guidance supports showing only fields relevant to the current task, grouping related inputs, and progressively revealing advanced options. That is a stronger foundation than adopting a particular fashionable visual style. [nngroup](https://www.nngroup.com/articles/4-principles-reduce-cognitive-load/)

## 1. The main menu: character, priorities, explanation

On desktop, I would use three regions:

```text
┌──────────────────────────────────────────────────────────────┐
│ Tactics                                  Test   Apply changes│
├──────────────┬────────────────────────────┬───────────────────┤
│ PARTY        │ SELECTED HORSE             │ DETAILS           │
│              │                            │                   │
│ Juniper      │ Behavior: [Balanced ▾]     │ Selected rule     │
│ Morrow       │                            │ explanation       │
│ Sable        │ Custom priorities          │                   │
│              │                            │ Target preview    │
│              │ 1. Emergency healing       │                   │
│              │ 2. Prepared-target attack  │ Test results      │
│              │                            │                   │
│              │ [+ Add priority]           │                   │
│              │                            │                   │
│              │ Otherwise: normal behavior │                   │
└──────────────┴────────────────────────────┴───────────────────┘
```

My proposed responsibilities:

- Left: switch characters and see a short behavior summary.
- Center: read and reorder the selected character’s rules.
- Right: edit one rule, inspect its meaning, or see test results.

On a narrow screen, switch to a single column with a dedicated rule-editing view. Do not squeeze three panels into an unreadable layout.

Keep equipment visible as context, but do not turn this screen into another full equipment-management screen.

## 2. Make each rule a readable card

A collapsed rule might read:

```text
1  Emergency healing                         Enabled

WHEN an ally’s health is below 35%
USE  Healing Light
ON   the lowest-health matching ally

Edit     Move up     Move down     More…
```

The editable values are selectable controls:

```text
WHEN [An ally’s health ▾] [is below ▾] [35%]
USE  [Healing Light ▾]
ON   [Lowest-health matching ally ▾]
```

Players never type “Healing Light” or “lowest-health ally.”

This resembles established no-code rule builders, which construct conditions from selected criteria, operators, and values rather than requiring a written expression. [docs.oracle](https://docs.oracle.com/en/cloud/paas/visual-builder/visualbuilder-building-applications/conditions-rule.html)

### Generate the names automatically

Use labels such as:

- Emergency healing
- Cleanse poisoned allies
- Finish primed enemies
- Attack groups

Generate these from the rule’s contents. A separate editable name is optional—and unnecessary for the initial version.

Likewise, automatically name saved profiles:

> Support · Juniper · 2

Players can choose an icon or predefined label if you want personalization without typing.

## 3. Start creation with an ability or an intent—not a blank expression

The display order can be WHEN → USE → ON, but the creation flow should start with something concrete.

When players click “Add priority,” show:

```text
What should this priority do?

[Heal an ally]       [Cleanse an ally]
[Prepare a combo]    [Finish a combo]
[Deal damage]        [Protect an ally]
```

Only show intents supported by the character’s equipped abilities. Alternatively, let players select an ability directly from a small ability grid.

Then guide them through:

1. Choose the ability.
2. Choose when it should be prioritized.
3. Choose the target.
4. Read the generated sentence.
5. Add the rule.

Selecting the ability first lets you filter everything that follows. A healing ability gets ally-health conditions; a finisher gets primer conditions. Conditional, relevant fields are consistent with recent guidance on reducing cognitive load in forms. [nngroup](https://www.nngroup.com/articles/4-principles-reduce-cognitive-load/)

### Offer a completed starting point

For Healing Light, prefill:

> When an ally is below 35% health, use Healing Light on the lowest-health matching ally.

The player can accept it immediately or modify it. The threshold is a proposed default for testing, not a universally correct value.

A useful editor begins with a valid instruction—not three empty dropdowns.

## 4. Choose controls according to the decision

Do not use dropdowns for everything.

| Selection | Proposed control | Reason |
|---|---|---|
| Equipped ability | Small icon-and-name grid | Lets players recognize abilities visually |
| A few behavior presets | Radio cards or segmented buttons | Makes alternatives visible |
| Condition category | Grouped picker | Keeps related choices together |
| Comparison | Plain-language selector | “Below,” “above,” “has,” “does not have” |
| Health threshold | Preset buttons plus numeric stepper | Supports quick choices and precision |
| Target preference | Small set of labeled options | Makes targeting explicit |
| Enabled/disabled | Switch with text label | Clearly communicates state |
| Rule order | Move buttons plus optional dragging | Works without precision dragging |
| Extra condition | “Add condition” button | Keeps advanced controls out of the default view |

For numbers:

```text
Health below:

[25%] [35%] [50%] [75%]

Custom: [−] [35] [+] %
```

Typing can be available, but never required. Validate the range, display units, and make the step size sensible.

A slider can be an additional convenience, but I would not make it the only way to set an exact threshold.

## 5. Make the mental model visible at all times

Above the stack, display:

> Checked from top to bottom. The first usable priority wins.

Below the stack, display:

> If none can be used, normal behavior takes over.

That explains the system without requiring players to learn `if`, `else`, or execution flow.

### Do not expose “else” as a separate editable object

For this design:

- Another rule means “otherwise, try this.”
- The fixed fallback means “otherwise, behave normally.”
- AND means another requirement for the same rule.

Start with one condition and optionally one additional AND condition. Avoid nested boolean groups in the first version.

When multiple conditions refer to a target, phrase the scope clearly:

> An ally whose health is below 35% AND who can receive healing.

Do not use two ambiguous “any ally” checks that might match different creatures.

## 6. Separate preferences from prohibitions

This is one of the most important interface distinctions.

These instructions are different:

> Prefer using Fire Burst on primed enemies.

> Only use Fire Burst on primed enemies.

The first affects priority; the second restricts the ability even in fallback behavior.

I would label the main editor “Custom priorities” and put restrictions in a separate, advanced section:

```text
Ability restrictions

Fire Burst:
[✓] Only use when a matching primer is present
```

If restrictions are not part of the initial implementation, do not offer wording that implies them.

Also show a contextual warning:

> This horse may leave Fire Burst unused when no suitable primer is available.

Avoid a generic “Are you sure?” prompt when a specific explanation would be more useful.

## 7. Make reordering and recovery easy

Provide:

- Move up/down.
- Optional drag handle.
- Duplicate.
- Enable/disable.
- Delete with undo.
- Reset to preset.

Dragging should be a shortcut, not a requirement. WCAG 2.2 calls for a non-dragging pointer alternative where dragging is not essential; move buttons satisfy the intent well for rule ordering. [w3](https://www.w3.org/TR/WCAG22/)

For a touch-capable interface, aim for comfortably large controls. WCAG 2.2’s minimum target-size criterion uses 24 by 24 CSS pixels with exceptions; its enhanced criterion uses 44 by 44. These are useful reference points for web UI, not literal engine-pixel specifications for every game. [w3](https://www.w3.org/TR/WCAG22/)

Include keyboard/controller navigation, visible focus, text labels, and non-color status indicators from the start.

## 8. Let players preview meaning before testing performance

The detail panel should translate the rule into an explanation:

```text
This priority:

• Finds living allies below 35% health.
• Excludes allies this ability cannot heal.
• Chooses the lowest-health eligible ally.
• Uses Healing Light if it is available.
• Otherwise, checks the next priority.
```

That is a semantic preview: what the rule means.

A test encounter provides a different preview: how the strategy performs.

Keep the distinction clear. A perfectly valid rule can still be strategically poor for a particular encounter.

### Show concise validation

Useful messages:

- “This ability is no longer equipped.”
- “No current party member can provide this primer.”
- “Threshold must be between 1% and 100%.”

Avoid overwhelming players with speculative warnings such as “This rule might be inefficient.” Put observed performance issues in the test report instead.

## 9. Use draft-and-apply behavior for live combat

For an idle game, I would let players edit a draft while the party continues running.

```text
Unsaved changes

[Discard] [Test draft] [Apply changes]
```

Explain when changes take effect:

> Applies at the next action decision. Does not cancel actions already in progress.

This prevents half-edited rules from unexpectedly affecting live combat.

Autosave the draft if desired, but distinguish:

- Draft saved.
- Active tactics updated.

If you instead choose immediate application, make it equally explicit and provide undo. Do not leave players guessing whether their edits are active.

## 10. Under the interface, store structured rules—not generated code

Architecturally, I would treat the menu as an editor for structured data.

```text
Rule editor
    ↓
Structured rule data
    ↓
Validation
    ↓
Shared tactics evaluator
    ↓
Selected action + explanation
```

Each rule stores stable identifiers for:

- Ability.
- Condition type.
- Comparison.
- Numeric value, if applicable.
- Target filter.
- Target selection.
- Enabled state.
- Order.

The readable sentence is generated from that data. It is not parsed back into logic.

This lets you:

- Translate the interface without changing behavior.
- Rename abilities without breaking profiles.
- Validate imported or copied rules.
- Generate consistent explanations.
- Run the same evaluator in combat, testing, and offline simulation.

Keep condition definitions in a shared registry that specifies their compatible subjects, value types, controls, and explanation text. That prevents the UI from offering combinations the evaluator cannot interpret.

## 11. The initial version I would actually build

Start with:

1. One Tactics tab per character.
2. A working preset.
3. A short ordered list of custom priorities.
4. Ability-first creation.
5. Selectable sentence fields.
6. Preset numeric values and steppers.
7. Move buttons.
8. A permanent fallback.
9. Draft testing and explicit application.
10. An expandable “why this happened” trace.

Do not start with a node canvas, a large condition encyclopedia, or arbitrary nested logic.

The target experience is closer to configuring a helpful teammate than programming a machine: choose what matters, read a clear sentence, and see the party carry it out.