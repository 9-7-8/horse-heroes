# Horse Heroes: Tutorial and Opening-Pacing Design

## 1. The recommendation in one sentence

Start by letting Juniper, Morrow, and Sable complete a real trail without requiring any player decision; then teach the player to understand and influence the next journey through a small, optional, reversible town decision.

The organising principle is **confidence**: “My horses can handle the road; I can understand what happened and help them do better next time.”

This document refines the earlier general guide against the supplied “Mechanics, for designing a tutorial” snapshot dated 2026-10-11. That snapshot is the source for game-specific mechanics and release status. The recommendations below are tutorial proposals, not newly confirmed game rules. The underlying repository documents and issues have not been independently inspected.

## 2. Corrections to the earlier general advice

The supplied mechanics change several important assumptions:

- There are no classes. Later progression uses a shared ability pool, shared party level, freely reassigned points, and a few equipped ability slots per horse.
- The player is an organiser, not a rider or combat controller. There is no avatar.
- The first trail needs no decisions at all. Do not make character creation, ability selection, equipment assignment, element selection, or destination choice a prerequisite for that first trail.
- The three starters already function: Juniper has Focused Shot, Morrow has Elemental Burst, and Sable has Ward in the current build.
- Automatic collection and current auto-equipping make inaction valid. The tutorial should reveal agency without making automation seem like failure to participate.
- Reports and offers never hold the party up. Tutorial prompts must not become mandatory acknowledgements.
- Full bags never halt a journey. Teaching cargo as an emergency would contradict the game.
- Horses and pets never die. Failure concerns exhaustion, returning, and bounded economic loss, not permanent companion loss.
- Selling, offline progression, ability points, richer equipment, and itinerary automation arrive in different releases. Do not teach them as if they exist now.

In particular, replace the earlier idea of a “first class unlock” with a later “first ability-pool or loadout decision,” when that system actually exists.

## 3. Use MDA to define the opening’s promise

MDA links implemented mechanics, the behaviour they produce during play, and the player’s emotional experience. “Aesthetics” means the desired experience, not simply the art style. [web:15]

For Horse Heroes, use the following proposed chain:

| Intended experience | Behaviour to encourage | Actual mechanic that supports it |
|---|---|---|
| Trust and companionship | Watch the gaggle act competently without controlling it. | Automatic walking, combat, ability use, and collection; companions cannot die. |
| Understanding | Connect an observable event to a readable explanation. | Trail tracker, item comparisons, camp appraisal, later trail reports. |
| Agency without pressure | Make a useful decision when interested, or continue without it. | Town equipment decisions; current fallback auto-equipping; nonblocking reports. |
| Satisfaction | Recognise a reward and see its contribution. | Loot shower, useful equipment, later camp-choice acknowledgement. |
| Curiosity | Notice that horses can cooperate in different ways. | Later Primer/Finisher reactions and eventually freely reassigned ability points. |
| Long-term stewardship | Improve a travelling group and the places it visits. | Later liberation, goodwill, recruitment, and town projects. |

Do not treat every visible system as an opening lesson. The opening’s job is to establish trust, observation, and one understandable town decision. Restoration and stewardship can initially be a promise rather than a task list.

## 4. Exact proposed starting state

### For testing against the built game

Use the existing three-horse party and starter abilities unchanged. Start with the normal loop already able to run.

| Element | Recommended tutorial treatment |
|---|---|
| Juniper, Morrow, Sable | Introduce their names and readable roles; do not require a horse picker. |
| Starter abilities | Explain their visible purpose when they occur, not in a startup lecture. |
| Town departure | Let the existing 12-second departure happen. Space / Set out remains an optional shortcut. |
| Party element | Keep the normal default; do not ask for a selection before the first trail. |
| Gear | Do not require equipping before departure. Work with the existing functional starting state. |
| Menus | Preserve the permanent frame. Highlight only the currently relevant control. |
| Coins | Identify as collected money if needed; do not create a spending lesson because nothing can spend them yet. |
| Objective | “Watch the party complete its first trail.” No mandatory confirmation. |

If the game technically begins in town, the first trail still starts automatically. Do not add a “Begin tutorial” button that must be pressed to make the world move.

### For the eventual 0.5.0 tutorial release

The opening must use the mechanics shipped by then, not a preserved simulation of 0.2.0:

- Ability elements and cooperation replace the temporary whole-party element system.
- Route choice, exhaustion, potions, failure, cargo, and the stall may exist.
- Seven combat gear slots and shared ability points do not arrive until 0.6.0.
- Itinerary automation and offline progression do not arrive until 0.7.0.

The preselected first route or first-departure setup is an implementation dependency. The snapshot requires a decision-free first trail but does not specify how the later destination-selection system supplies that initial journey. Resolve this in the pacing implementation; do not silently assume a confirmed starting town, map size, or opening boss.

## 5. Proposed first-trail sequence: observe, do not operate

### Beat A — Establish the player’s role

Use one short, nonblocking line:

> “You organise the gaggle. The horses walk, fight, and collect loot by themselves.”

Keep this in the existing interface rather than a separate tutorial screen. Do not ask the player to aim, click an enemy, activate a skill, or approve a drop.

Success criterion: a new player does not search for combat controls or believe clicking makes attacks happen.

### Beat B — Let the first fight demonstrate the starter roles

Associate each explanation with its actual effect:

- Focused Shot: a large single hit.
- Elemental Burst: a hit across enemies.
- Ward: damage reduction.

These are observational cues, not three lessons to acknowledge. If a fight ends before an ability fires, defer its explanation. Never announce an effect the player has not seen.

For later readable-combat releases, use ability tells and hit feedback to carry this explanation. Current-build tests must account for the absence of that later presentation work.

### Beat C — Show the first loot shower

Let coins arc, bounce, settle, and reach the party automatically. The motion itself demonstrates collection.

Suggested line:

> “Loot comes to the party. You do not need to pick it up.”

Do not introduce bag management, filters, every rarity, or pets here. Pets are not the current collection mechanism.

### Beat D — Make progress through the trail legible

At a relevant encounter transition, briefly explain the tracker: completed markers behind, current fight ring, upcoming outlines, and the larger boss diamond.

This can be an inspectable explanation rather than a demanded task. The tracker helps the player understand the first journey, so it is useful before the second trail even though mastery of it is not required.

### Beat E — Let the boss fountain be the first crescendo

The existing fuller boss loot fountain can distinguish completion from an ordinary fight. Avoid adding an acknowledgement screen that holds the party up.

Keep rarity teaching local: explain the colour and shape of the item actually obtained. Do not describe orange or green items as having unique powers or set bonuses in the current build, where they only provide more stat points and a beam.

### Beat F — Return and offer the first decision

The first required learning outcome is observation. The first offered decision is gear inspection and assignment after return.

No failure to open the card should invalidate the tutorial or stop travel.

## 6. First town decision: teach the item as the handle

Use the first real drop to teach the established interaction model:

> “Press a piece to inspect it. Its card compares it with gear on the selected horse.”

Recommended order:

1. Show where the item is.
2. Show which horse is selected in the status strip.
3. Open the item card when the player presses the item.
4. Read the before/after stats and verdict together.
5. Offer Equip, with E shown as the keyboard alternative.
6. Acknowledge the assignment and keep its resulting state visible.

The selected horse is shared across every place. This is not a minor implementation detail: it is essential to understanding why comparisons change. Introduce horse selection here, where it has a concrete purpose, rather than making the player click all three horses at startup.

### Handle auto-equip honestly

Current fallback behaviour is helpful but can obscure agency. Suggested copy:

> “You can choose who wears it. If you leave it in cargo, it will go on the horse it helps most when the party sets out, or be left behind.”

Do not call the fallback an error. A player who watches another trail without touching equipment is playing validly.

Do not promise that this fallback persists into later releases unless it remains part of the shipped rules. In the release-specific tutorial, gate this explanation on the actual implemented behaviour.

### The 12-second town window is not a deadline to learn

Automatic departure may happen while someone is reading. Keep guidance recoverable, and resolve it against current state:

- If the item has already auto-equipped, explain where it went through the appraisal or report.
- If equipping is currently town-only, show that reason rather than implying the button is broken.
- Preserve the learning opportunity at a later eligible town visit.
- Once 0.6.0 road planning ships, explain queued changes and their application on arrival instead of saying road changes are simply unavailable.

Do not pause the whole loop until the player presses Equip. If optional staying-in-town controls are needed, they are a separate design decision—not an assumed existing feature.

### Keep swapping as contextual help

Teach the swap when the player first tries to equip an item another horse already wears:

> “The horses exchange pieces. Nothing is lost.”

Do not add it to the first equipment lecture unless that is the interaction actually happening.

## 7. Show the effect without inventing a comparison result

The next encounter is a chance to connect assignment and performance. However, the current build simultaneously increases enemy health and attack by 15% after a win. A stronger item does not guarantee a faster or easier next trail.

Use evidence appropriate to the change:

- Immediately show the changed stat and assignment.
- Identify the item’s relevant benefit.
- Explain separately that a victory increases trail difficulty and improves drop stats.
- Use later reports to explain relevant outcomes when those reports actually support the claim.

Do not say “Your new item made that fight faster” merely because the player equipped it. Enemy composition, elements, and difficulty can all change.

This is the game-specific version of teaching causality: show what changed, and distinguish it from everything else that changed.

## 8. Release-specific learning plan

| Release / system | First relevant teaching event | Teach | Defer |
|---|---|---|---|
| Built opening loop | First departure and fight | Organiser role; automatic action and collection. | Menus as a complete catalog. |
| Built gear | First drop, preferably inspected in town | Item card, selected horse, comparison, Equip, fallback. | All rarity tiers and every gear interaction. |
| Built difficulty | First completed trail / next trail | Winning raises the next trail’s difficulty and drop stats. | General combat mathematics. |
| Temporary party elements | First resisted fight, if testing 0.2.x | One short explanation of the current matchup and town-only change. | A major tutorial branch for a system being replaced. |
| 0.3.0 routes | First player-chosen journey after the introductory route | Destination, route preview, distance, danger, departure commitment. | Advanced routes and biome optimisation. |
| 0.3.0 cooperation | First clearly visible Primer-to-Finisher event | One horse marks, another consumes the mark for a reaction. | All three reactions and all damage rules at once. |
| 0.3.0 recovery | First visible potion use, retreat opportunity, or failure | Automatic potions, exhaustion, bounded costs, free chosen retreat, failure waits. | Treating companions as mortal or needing manual healing in combat. |
| 0.4.0 selling | First spare piece eligible for listing | Listing is required; townsfolk buy over time while in town. | Goodwill strategy before a sale exists. |
| 0.4.0 cargo | First relevant capacity event or cargo inspection | Shared space, equipped items excluded, full bags do not stop travel. | Custom retention optimisation. |
| 0.4.0 prices | First sale / price inspection | Price affects earnings and goodwill. | Requiring a discount or claiming an exact sale wait. |
| 0.6.0 abilities | First level-up or ability allocation opportunity | Shared pool and level, a few slots, free town reassignment, no classes. | Full combinatorial build optimisation. |
| 0.6.0 gear roles | First relevant item in each slot | That slot’s identity and item effects. | Seven-slot lecture before an actual comparison. |
| 0.7.0 itinerary | First meaningful multi-destination planning opportunity | Success continues according to itinerary; empty itinerary and failure wait. | Offline automation before the itinerary is understood. |
| 0.7.0 offline | First opportunity to leave an ongoing activity | The same activity continues away, up to eight hours. | Promising progress when activity is stopped. |
| 0.9.0 trade | Second town, when trading opens | Goods use the caravan hold, separate from gear bags. | Price trends and auto-buy rules in the same step. |
| Later companions and expression | Each actual first recruit, pet, look, or family event | The immediately relevant interaction. | A roadmap lecture in the opening. |

Events that coincide should produce a queue of learning opportunities, not several simultaneous panels.

## 9. Teaching cooperation without adding combat controls

The tutorial must teach preparation and interpretation, not skill activation.

A suitable cooperation demonstration needs enough encounter duration for the actual equipped abilities to complete the sequence. Do not fake a reaction in a harmless encounter that ends before the relevant abilities fire.

Proposed sequence:

1. Highlight the actual Primer mark when applied.
2. Identify the horse or ability responsible.
3. Highlight the actual Finisher and consumed Primer.
4. Name the resulting reaction and its effect.
5. Preserve an explanation in the report or Codex where available.

Teach the one reaction that occurred:

- Thermal Shock: Fire + Ice, a burst on a dangerous target.
- Wildbloom: Fire + Plant, damage erupting into nearby enemies.
- Wintergarden: Ice + Plant, a party barrier with a damage burst.

Do not assert which starter ability has which tag or element: the supplied snapshot does not specify the later starter loadouts.

At 0.5.0, observing cooperation is distinct from freely allocating shared ability points, which is planned for 0.6.0. The early tutorial must use whatever cooperation-capable loadout the shipped game actually supplies; that configuration remains a dependency to resolve.

Avoid teaching mixed elements as universally optimal. Same-element crescendos arrive in 0.8.0 and must fit into the explanation later.

## 10. Failure: reassurance with accurate stakes

Explain “horses and pets cannot die” early, because that belief changes how safely players watch fights.

Teach detailed failure costs at the first relevant risk preview or failure, not as a dramatic opening warning.

For 0.3.0 onward, proposed wording:

> “The party returned to the previous town. No horse or pet is lost. A failed journey can cost a small capped amount of coins and some newly found gear. The party waits for your next decision.”

Offer precise details on demand: at most one tenth of gear found on that trip, lowest rarity first, and at most three pieces. Retreat by choice is free.

For the current build, say only that losing costs nothing yet. Do not present planned costs as active.

Health restoration while waiting is gradual, not an instant town reset. Later loss of trade goods belongs to the trade tutorial, once that system exists.

## 11. Selling is another lesson, not part of the first-trail contract

Once 0.4.0 is available, the first spare gear creates a natural selling lesson:

1. Identify a piece the player is willing to sell.
2. Show protection / lock status before batch listing.
3. Explain that listing places it at the stall; it is not an instant vendor transaction.
4. Show the sale preview and price.
5. Explain that customers buy over time while the party is in town.
6. Explain that departing stops sales and unsold stock travels with the party.

Do not require staying until a sale completes to finish the tutorial. That would turn a nonblocking offer into a hidden obligation.

Goodwill follows naturally from the first sale or price interaction. Explain its connection to welcome, recruits, and looks without making the player optimise prices immediately.

The later itinerary can govern selling until sold out or a chosen time passes. Do not imply that this automatic schedule exists in 0.4.0.

## 12. Make the permanent menu frame the tutorial’s map

Progressive disclosure reduces initial complexity by prioritising immediate options and putting less-needed detail in additional layers. [web:11]

Horse Heroes already has a permanent frame and visible unavailable controls. Apply progressive disclosure to explanations and emphasis, not by pretending that all the existing places vanish until unlocked.

Teach the frame in this order:

1. Scene and footer: where the party is and what it is doing.
2. Status strip: selected horse, relevant health or resources.
3. Horses → Gear: the first useful assignment.
4. Journal: where explanations and reports can be revisited, when its consolidated home ships.
5. Trade: when listing matters.
6. Map: when choosing a destination matters.
7. Remaining tabs: when their actual content becomes relevant.

Mouse-first instructions should name the control and include its printed shortcut. Do not require keyboard memorisation.

Introduce Escape as “back one level” in context. Avoid teaching all place keys, tab numbers, and horse-selection keys simultaneously.

Unavailable controls should retain their explanation. The tutorial should not direct a new player to press an unbuilt action to make progress.

When attention indicators ship, use them to signal the relevant place, then clear them once seen. Avoid permanent tutorial badges that compete with future notices.

## 13. Juice should explain the gaggle’s work

Juice adds satisfying feedback, and it works best when it reflects the core gameplay. [web:10]

For Horse Heroes, the key feedback hierarchy is:

- Ordinary hit: readable, restrained feedback.
- Signature ability: recognisable tell and result.
- Primer: persistent, identifiable mark.
- Finisher reaction: distinct payoff with an understandable source.
- Ordinary loot: satisfying collection without a major interruption.
- Boss fountain / meaningful first: larger emphasis.
- Camp choice: immediate acknowledgement of the change, not a modal celebration.

Existing loot motion already teaches automatic collection. Do not cover it with a tutorial panel at the exact moment it should be visible.

Later loot reports should summarise ordinary rewards and reveal the best individually. Do not use first-time reveals to turn every drop into a ceremony.

Offer feel settings before effects become an accessibility problem. Essential lessons must survive reduced shake, flash, or effects, and must not depend on sound or colour alone. Keep rarity shapes and readable labels as meaningful alternatives.

## 14. Turn #47 into a data-driven learning sequence

The existing pacing requirement is an unlock order in one data file, one new thing at a time, tied to relevant events. The tutorial should become a declarative sequence with runtime checks, not a chain of compulsory click gates.

Separate three states:

- Feature availability: whether this release and save can use the mechanic.
- Lesson eligibility: whether something happened that makes its explanation useful.
- Lesson progress: whether the player saw, used, skipped, or revisited the explanation.

A tutorial must not accidentally lock a built feature just because its explanation has not been read. Some genuine progression unlocks may still exist; their conditions should be separate from reading a prompt.

### Proposed record shape

The following is illustrative configuration, not an existing repository schema or confirmed event API:

```yaml
- id: inspect_first_gear
  order: 30
  requires_feature: item_cards
  eligible_after: first_gear_received
  preferred_context: town
  availability_check: inspectable_owned_item
  presentation: contextual_hint
  target: owned_item
  blocks_simulation: false
  objective: understand_item_comparison
  observed_by:
    - item_card_opened
    - comparison_viewed
  followup_after: equip_completed
  fallback_if_state_changed: show_latest_appraisal
  revisit_location: journal_if_available
```

Other useful fields include prerequisites, localisation keys, release/profile constraints, priority, dismissal behaviour, and replacement lessons after a mechanic migrates.

### Runtime requirements

- Queue eligible lessons and present one focus at a time.
- Recheck the actual context before showing or acting on a hint.
- Resolve targets dynamically; an item may already be equipped, swapped, left behind, or listed.
- Defer a town-only task when the party departs.
- Never wait for “OK” to advance the simulation.
- Distinguish viewing an explanation from demonstrating understanding.
- Let a skipped lesson remain retrievable.
- Make triggers idempotent so returning to town does not replay the opening.
- Persist progress only once save/load exists; current-build tests should not imply that progress survives closing the game.
- Replace obsolete element guidance when the ability-element system ships.

Milestone reveals and tutorial instruction should share an attention budget. A first boss, first rare item, and first gear hint may happen together; do not stack three overlays.

## 15. Specific risks to test

| Risk | Test situation | Desired result |
|---|---|---|
| Players look for attack buttons | Watch the first fight without guidance from the designer. | They identify automatic combat. |
| Players think loot needs clicking | Let the first fountain play normally. | They recognise automatic collection. |
| Auto-equip hides agency | Let several trails run with no equipment interaction. | The player can still discover manual assignment without punishment. |
| Selection context is missed | Switch horses while viewing the same item. | The player understands whose stats are compared. |
| Departure interrupts reading | Open a card near the end of town time. | No lost progress, misleading action, or blocked journey. |
| Upgrade causality is oversold | Equip an item before a harder trail. | The explanation distinguishes item benefit from enemy scaling. |
| Reaction lesson cannot occur | Try the intended demonstration encounter with the actual loadout. | A visible real reaction occurs, or the lesson safely defers. |
| Selling is mistaken for instant cash | List a spare piece and depart before it sells. | The player understands why stock remains unsold. |
| Failure implies death | Observe a loss. | The player knows companions remain and the party waits. |
| Offline claims exceed shipped scope | Close or suspend each supported release profile. | Tutorial wording matches actual progression and saving behaviour. |

## 16. Opening acceptance criteria

A successful opening is not one where every hint was clicked. It is one where a player can explain:

- “I organise the party; I do not control a fight.”
- “The horses collect their rewards by themselves.”
- “I can inspect an item and choose who wears it.”
- “The comparison is about the selected horse.”
- “I can let the first trail happen without doing anything.”
- “I know where to look for the next decision or an explanation.”

For later shipped systems, add separate criteria for reaction interpretation, listing, failure recovery, itinerary behaviour, and offline progress. Do not bundle all of those into the first-level test.

Measure unassisted understanding, recoverability after interruption, and correct predictions—not merely tutorial completion speed.

## 17. Decisions still needed before implementation

The supplied snapshot leaves several opening dependencies unresolved:

1. What establishes the initial route once destination selection exists?
2. What exact starter ability configuration demonstrates cooperation in 0.3.0–0.5.0?
3. Does fallback auto-equip remain in later releases, and how does it interact with the developing cargo and loadout systems?
4. How does a learner revisit a missed town-only action before queued road planning exists?
5. Which tutorial information can the shipped report actually substantiate?
6. How will unavailable release features differ visually from progression-locked or context-restricted features?

None of these requires deciding classic world size, breed combat effects, first pet abilities, or proposed build-laboratory systems just to prototype the opening.

The immediate implementation target is therefore narrow: a decision-free first trail, a nonblocking first-item lesson, clear selection context, and one trustworthy explanation of the next journey. Everything else should join the tutorial at the event and release where it genuinely becomes relevant.
