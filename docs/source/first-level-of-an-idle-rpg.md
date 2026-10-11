# Designing the First Level of an Idle RPG

## Core recommendation

Build the opening as a small, satisfying version of the real game—not as a lecture before the game begins. Give the player a functioning starting character or party, a readable encounter, one meaningful improvement, and an immediate opportunity to see that improvement matter.

The opening should establish this relationship:

> I understand what happens automatically, I understand what I control, and I can see how my decisions improve the outcome.

This document stays deliberately broad. It does not assume a particular setting, party size, class structure, combat formula, or progression economy. The suggested sequence and starting loadout are prototype recommendations, not universal rules or finalized mechanics.

## 1. Define what the first level must teach

Treat the first level as an onboarding sequence rather than necessarily a physical map or a particular character level. It might span a starting screen, a short encounter, a reward screen, and the next expedition.

By the end, the player should be able to answer:

- What is my immediate goal?
- What does the game do automatically?
- What decisions belong to me?
- How do I obtain loot and recognize a useful item?
- How do I change an ability, class option, or piece of equipment?
- How can I tell whether that change helped?
- What do I do next, and what can I safely leave running?

Do not make the tutorial responsible for explaining every system. Teach the minimum complete loop that makes later systems understandable.

A general loop is:

> Observe an encounter → earn a reward → make a build decision → observe the changed result → continue.

## 2. Use MDA to design backward from the experience

MDA means Mechanics, Dynamics, and Aesthetics. Mechanics are the implemented rules and components; dynamics are how those rules behave during play; aesthetics are the desired emotional responses. Here, aesthetics does not simply mean visual style. [web:15]

Start with what you want the player to feel, then choose behaviors and rules that can produce it. MDA is an analysis and iteration framework, not a formula that guarantees a successful tutorial. [web:8][web:15]

| Desired opening experience | Behavior to encourage | Supporting opening mechanic |
|---|---|---|
| Competence: “I understand this.” | Player predicts a simple outcome and sees it happen. | A readable encounter and an ability with a visible role. |
| Agency: “My choices matter.” | Player changes a build and notices the difference. | A reversible equipment or ability decision. |
| Discovery: “There is more to explore.” | Player inspects a nearby unlock. | A limited preview of a later class or ability. |
| Satisfaction: “That was worth doing.” | Player connects effort, reward, and improvement. | A reliable early reward followed by a useful application. |
| Confidence in automation: “I can let this run.” | Player understands what continues without input. | Explicit automation status and a clear stopping condition. |

These are proposed applications of MDA to your idle RPG, not a list of required MDA aesthetic categories.

### The useful diagnostic

For each tutorial step, ask:

1. What rule am I introducing?
2. What behavior should that rule cause?
3. What feeling should that behavior produce?
4. Can the player actually observe the connection?

If the intended feeling is mastery but the player only follows a flashing arrow, the step may teach navigation without teaching the game.

## 3. Decide exactly where the player starts

Choose a starting situation that minimizes prerequisite knowledge while still representing normal play.

A useful default is a safe staging area or a low-risk encounter, with a working starter build already assembled. Avoid beginning with an empty character sheet and several irreversible decisions unless character creation itself is a central pleasure of the game.

| Starting element | Broad recommendation | Reason |
|---|---|---|
| Character or party | Start with the smallest viable playable unit. | Reduces simultaneous relationships to explain. |
| Class identity | Provide a clear starter identity, or a small reversible selection. | Establishes a role without requiring mastery of the full class list. |
| Basic equipment | Supply enough to function immediately. | Prevents the first lesson from becoming repair of a broken setup. |
| First encounter | Use a legible, forgiving threat. | Gives the player room to observe cause and effect. |
| First objective | State one concrete goal. | Keeps attention on the current loop. |
| Resources | Supply enough for the intended first interaction. | Avoids a tutorial that stalls because of an earlier choice. |
| Interface | Expose the current loop and contextual help. | Keeps advanced management from competing with the first task. |
| Future systems | Preview selectively, with understandable unlock conditions. | Shows direction without presenting a wall of unavailable options. |

Choose location, enemy, and narrative context to support the same lesson. The setting can be quiet and low-risk without feeling fake: a short local expedition, practice encounter, or introductory contract can all work.

The starting experience should promise the actual game. Do not require intensive manual combat in the opening if later play mainly involves automated combat and build management.

## 4. Choose starting abilities by teaching role, not by count

There is no universal correct number of starting abilities. The important limit is how many distinct ideas the player must understand simultaneously.

A conservative prototype could begin with:

- A baseline action that makes the character functional.
- One signature ability that communicates the character’s role.
- One additional ability or modifier introduced after the baseline is understood.

The baseline and signature ability may be the same ability. Do not add a second button merely to satisfy a template.

### What makes a good starting ability?

Prefer an ability that:

- Has an understandable purpose: damage, protection, recovery, setup, or another clear role.
- Produces an effect the player can notice without a combat spreadsheet.
- Uses the same activation rules that will apply later.
- Does not require several hidden prerequisites.
- Creates a foundation for later combinations rather than becoming immediately irrelevant.
- Has wording that connects the description to what appears in combat.

If abilities fire automatically, teach their trigger, target, and cooldown behavior instead of implying that the player should press them during combat.

### Introduce class complexity in layers

Because your class-list structure is not yet specified, separate these concepts explicitly:

1. Class identity: what role or style this class offers.
2. Access: what abilities the class unlocks.
3. Selection: which accessible abilities the player has chosen.
4. Execution: how those selected abilities actually operate.
5. Progression: how more options become available.

Only introduce distinctions your system actually uses. If “unlocked” does not mean “equipped” or “active,” make that distinction visible at the moment it matters.

Do not begin with the entire class catalog and expect players to infer a good build. Introduce the class list when the player has a reason to use it, and highlight a small relevant subset without pretending the other options do not exist.

The first ability unlock should let the player do something newly understandable—not simply add another unexplained icon.

## 5. Build the tutorial around a complete reward loop

The following is a suggested sequence, not a mandatory timeline.

| Beat | Player experience | Lesson | Evidence that the lesson landed |
|---|---|---|---|
| 1. Orient | Sees a character, objective, and current activity. | “This is my role and immediate goal.” | Can identify what to do next. |
| 2. Observe | Watches or starts a short encounter. | “This is what the game handles.” | Can distinguish automatic actions from player decisions. |
| 3. Receive | Gets a useful reward. | “Combat produces something I can use.” | Finds the reward and understands its category. |
| 4. Decide | Equips an item or selects an ability. | “I control the build.” | Makes the change without guessing at unrelated menus. |
| 5. Apply | Faces a comparable encounter. | “My change affected the outcome.” | Can explain at least one relevant difference. |
| 6. Vary | Meets a small new threat or choice. | “The rule works beyond the original example.” | Applies the lesson without identical instructions. |
| 7. Continue | Sees the next goal and idle behavior. | “I know how to keep progressing.” | Continues independently or leaves safely. |

Give a brief explanation before the relevant decision, not a large explanation long before the player can use it.

Remove scaffolding gradually: guided interaction first, a lightly prompted repetition next, then an unprompted application.

### Teach a rule, not just a click path

“Open equipment → select item → equip” teaches operation.

“This item changes the stat your current ability uses; compare the next encounter” teaches a reason to operate the system.

Both are useful, but the second is what helps players make future decisions.

## 6. Make early loot teach item evaluation

The first important drop should have a clear instructional purpose. A reliable suitable reward is usually a better prototype than leaving the essential lesson entirely to random drops.

This does not require all early loot to be deterministic. You can preserve randomness around a guaranteed teaching reward while ensuring the tutorial cannot stall.

For the initial reward:

- Make its arrival noticeable.
- Make its destination clear: inventory, reward panel, or automatic equipment.
- Show who can use it and whether prerequisites apply.
- Compare only the properties relevant to the current decision by default.
- Let the player inspect more detail when needed.
- Make equipping or selecting it visibly change the build.
- Provide a chance to observe its effect soon afterward.

Start with an easy-to-read improvement, then introduce tradeoffs once the player understands the comparison interface. An endless stream of obvious upgrades teaches replacement, not build evaluation.

Avoid suggesting that higher rarity or a green number always means “better” if your eventual item system depends on synergy, role, or tradeoffs.

## 7. Apply intuitive UX through progressive disclosure

Progressive disclosure presents the important options first and moves advanced or less-used options to additional layers. Its purpose is to reduce initial complexity while preserving access to depth. [web:11]

For this opening, distinguish three things:

- Hidden detail: information is available on demand.
- Locked functionality: the player cannot use it yet.
- Guided emphasis: the function exists, but the interface emphasizes the relevant next action.

Do not use progression locks as a substitute for organizing information.

### Practical interface rules

- Keep the immediate goal visible without occupying the entire screen.
- Use consistent names for the same class, ability, item category, and action.
- Make the next relevant action discoverable through placement, wording, and state—not only an animated arrow.
- Put comparisons beside the decision they inform.
- Show selected, equipped, locked, unavailable, and active states distinctly.
- Explain why an option is unavailable and what would make it available.
- Keep back, close, confirm, and cancel behavior predictable.
- Preserve context when opening inventory or a class panel.
- Make exploratory changes reversible where possible.
- Reserve confirmation prompts for consequential actions rather than every ordinary interaction.
- Keep help available after the tutorial ends.

Use prompts that describe a goal and a reason. For example: “Choose an ability for this slot. Its effect will be used automatically in the next encounter.” Adapt the wording to your actual rules.

### Accessibility is part of initial usability

Microsoft’s Xbox Accessibility Guidelines cover text, contrast, multiple cue channels, input, navigation, focus handling, destructive actions, time limits, visual distractions, and photosensitivity. These are relevant to the opening experience, not merely to late-stage polish. [web:9]

Apply that broadly by providing readable text, visible focus, non-color state indicators, appropriate input support, and control over distracting effects. Do not require sound alone, color alone, or a brief animation alone to understand an essential tutorial event.

## 8. Use juice to reinforce meaning

Juice is the layer of satisfying feedback that makes an action feel impactful. It should echo the core gameplay rather than decorate unrelated activity. [web:10]

In a menu-driven idle RPG, important feedback may happen around decisions as much as combat:

| Event | Possible feedback | Information it should communicate |
|---|---|---|
| Equip an item | Slot highlight, short sound, changed values. | The item is now assigned and the build changed. |
| Unlock an ability | Reveal animation and a readable summary. | A new option exists; clarify whether it is active yet. |
| Change a class option | Updated identity label and affected ability display. | Which build elements changed. |
| Trigger a signature ability | Distinct visual effect and optional label. | Which character or ability caused the event. |
| Earn meaningful loot | Brief emphasis and persistent reward entry. | What arrived and where to inspect it. |
| Complete an encounter | Clear result and reward transition. | The encounter ended and the next decision is available. |

### Keep feedback truthful and proportionate

- Acknowledge input promptly, even if the resulting action takes time.
- Make consequential changes more noticeable than routine events.
- Keep persistent confirmation for facts that cannot safely depend on a fleeting animation.
- Avoid delaying the next action just to finish a celebration.
- Do not make every ordinary item look like a legendary discovery.
- Do not use effects so dense that they conceal the causal chain being taught.
- Offer reduced-motion, audio, and other relevant presentation controls.

Satisfaction and understanding should support each other. A rewarding flourish is most useful when it also confirms what happened.

## 9. Teach idle behavior explicitly

An idle RPG has two learning problems: understanding what to do while present, and understanding what happens while absent.

If the relevant systems exist, clarify:

- Whether combat repeats automatically.
- Whether progress continues in the background or offline.
- What determines the current destination or activity.
- What happens at defeat, completion, or a progression barrier.
- Whether inventory or resource limits can stop collection.
- How rewards are collected or summarized on return.
- Which conditions require a player decision.

Do not imply offline progress if the game only advances while running. Likewise, do not imply indefinite farming if capacity or another stopping condition applies.

Introduce these details as they become relevant rather than in a single startup wall of text.

Give players enough visible activity to establish confidence before asking them to wait. Avoid making an essential teaching step depend on a long timer with no useful decision in between.

The opening should show the intended attention rhythm: make a decision, let the system operate, then return to evaluate results.

## 10. Make early challenge safe but informative

Forgiving does not have to mean consequence-free or unrepresentative.

Use a situation where a mistake produces understandable feedback and a manageable recovery. Avoid permanent character damage, irreversible spending, or loss of an essential teaching item while players are still learning what controls mean.

A useful first challenge asks the player to apply an established rule with one variable changed. Examples include a different enemy role, a new equipment comparison, or a reason to select an alternative ability.

Do not introduce several new enemy mechanics, a new resource, a class branch, and a complicated loot decision in the same beat.

If failure is part of normal play, teach recovery before letting players meet an unexplained hard stop. If the opening suppresses or softens failure, transition clearly to the normal rules.

## 11. Pace by understanding, not just elapsed time

Use player actions and demonstrated understanding to trigger tutorial progression where practical.

Examples:

- Introduce equipment comparison after the first suitable item arrives.
- Introduce ability selection when a real slot or choice becomes available.
- Explain a stopping condition when the player first encounters it.
- Remove a prompt after the player completes its task, not because a timer expired.

Elapsed-time targets are useful for prototype tuning, but they are not proof of comprehension. Keep prompts readable at the player’s pace and account for interruptions.

Provide a way to dismiss or skip guidance without leaving the player with an unusable build. Returning players may need less explanation; new players may need to reopen it later.

## 12. Playtest the opening without explaining it yourself

The tutorial is ready for testing when the smallest complete loop works. Do not wait until every class and loot tier exists.

Ask testers to play while you observe. Avoid rescuing them immediately; note where they hesitate, guess, misinterpret a state, or wait for something that will never happen.

After the opening, ask:

- What is your main job in this game?
- Which actions happen automatically?
- Why did you choose that item or ability?
- What changed after you made the choice?
- What would you try against the next challenge?
- What happens if you leave the game in its current state?

### Useful observations and measures

| Measure | What it can reveal | Important limitation |
|---|---|---|
| Time to first meaningful decision | Whether agency arrives promptly. | Faster is not automatically better. |
| Time to first observable improvement | Whether reward and consequence connect. | A flashy event may not demonstrate understanding. |
| Unassisted completion of the core loop | Whether navigation and sequencing work. | Following prompts can conceal weak comprehension. |
| Correct explanation of automation | Whether the player trusts the right rules. | Verify against actual behavior, not expectations. |
| Application in a slightly different encounter | Whether learning transfers. | Keep the new task relevant to the taught rule. |
| Recovery from a mistake or interruption | Whether the tutorial is robust. | Include different inputs and accessibility needs. |
| Independent continuation | Whether the next goal is clear. | Continuation alone does not explain satisfaction. |

Set success thresholds after observing your prototype and intended audience. There is no universal opening duration, ability count, or completion percentage that proves your design is good.

## 13. Common opening-level mistakes

- Explaining the entire class list before the player understands one class.
- Starting with too many equally prominent menus.
- Giving several unlocks at once with no chance to use them separately.
- Relying on random loot for an essential tutorial step.
- Teaching manual micromanagement that normal idle play does not require.
- Allowing a required tutorial resource to be spent elsewhere and blocking progress.
- Mistaking button-click compliance for understanding.
- Making changes visually exciting but mechanically unreadable.
- Requiring repetitive waiting before the player understands the reward loop.
- Overprotecting the player so much that the opening never resembles the actual game.
- Hiding future goals completely, or displaying every distant feature as an overwhelming locked menu.
- Leaving the player unsure whether a new ability is available, equipped, or active.

## 14. Compact planning worksheet

Before implementing the opening, fill in:

1. Player role: What does the player control?
2. Emotional promise: What should the first loop feel like?
3. Starting state: Where are they, and what functional build do they have?
4. First objective: What one thing are they trying to accomplish?
5. First ability: Which core rule does it demonstrate?
6. First reward: What decision does it make possible?
7. First improvement: How will its effect be observable?
8. First variation: How will you check transfer of learning?
9. Automation contract: What continues, what stops, and what needs attention?
10. Exit condition: What shows the player can continue independently?

## 15. First-level acceptance checklist

- [ ] The player begins with a viable build and a clear immediate goal.
- [ ] The first encounter represents the real gameplay model.
- [ ] The distinction between automated activity and player decisions is clear.
- [ ] Starting abilities have readable purposes and activation rules.
- [ ] The class interface separates access, selection, and execution wherever those differ.
- [ ] One complete combat–reward–build–combat loop occurs.
- [ ] The first important reward cannot leave the tutorial stuck.
- [ ] The player can observe the effect of a meaningful change.
- [ ] A small variation checks understanding beyond an exact click sequence.
- [ ] Advanced detail is available without dominating the first decisions.
- [ ] Feedback confirms state changes without obscuring them.
- [ ] Guidance can be dismissed, resumed, or reopened appropriately.
- [ ] Essential lessons do not depend solely on color, sound, motion, or a timed cue.
- [ ] Failure and interruption do not create unexplained dead ends.
- [ ] The next goal and relevant idle behavior are understandable.
- [ ] New players can continue without the designer narrating beside them.

The central design principle is **causality**: let players make a small decision, see a trustworthy result, and use that understanding to make the next decision. MDA helps define why the experience should matter; intuitive UX makes the decision understandable; juice makes its result legible and satisfying. [web:15][web:11][web:10]
