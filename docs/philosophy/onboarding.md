# Onboarding: the opening and the tutorial

How a new player learns the game. The opening is a small, real version of
the game, not a lecture before it. The word to hold on to is **confidence**:
"my horses can handle the road; I can understand what happened and help them
do better next time."

**The owner's rule (2026-10-11), which outranks everything below:** tutorials
are interactive and taught through gameplay, "never, ever, ever just a wall
of text a player is expected to read". A lesson is something that happens or
something the player does, with at most a line beside it. And once 0.2.2 has
shipped, every new feature has a tutorial somewhere, as it has a test.

Also hers, the same day: the game starts on the road and combat is learned
by watching; the first town does not leave by itself (#302), so where this
file speaks below of the 12 seconds in the first town, that now applies to
later towns only; and a first-town list pays a few coins for trying things
(#303; built 2026-10-13, `docs/lessons.md`, "The things to try"). The statement is [vision.md](vision.md), Tutorials.

The rest is advice from the two source documents and is not her decision;
what she has decided is in [vision.md](vision.md), and the open questions
are a `decision` issue (#289).

## Principles

1. **Teach the smallest whole loop, not every system.**

       observe an encounter -> earn a reward -> make a build decision
         -> observe the changed result -> continue

   By the end of it the player can say what the goal is, what happens by
   itself, what is theirs to decide, how to tell a useful item, how to change
   something, how to tell that it helped, and what can safely be left
   running.

2. **Design backward from the feeling.** For each step ask: what rule is
   introduced, what behaviour should it cause, what should that feel like,
   and can the player see the connection? A step that is only "follow the
   arrow" teaches navigation and not the game.

3. **The opening promises the real game.** Start with a working party and a
   readable, forgiving encounter. Never ask for manual play that the game
   will not ask for later. If the opening softens a rule, say plainly when
   the ordinary rule begins.

4. **Observe first, then decide.** The first required outcome is watching.
   The first offered decision comes after a reward makes it relevant, and it
   is small, optional and reversible.

5. **Explain at the event, not before it.** A short line when the thing
   happens, at the place it happens. Never announce an effect the player has
   not seen; if it did not happen, the lesson waits.

6. **Teach the rule, not the click path.** "Open, select, equip" is
   operation. "This piece changes the stat this horse leans on; watch the
   next fight" is a reason. Remove the scaffolding in steps: guided, lightly
   prompted, unprompted.

7. **Causality, honestly.** Show what changed, and keep it apart from
   everything else that changed. Do not credit a decision with a result the
   game cannot substantiate.

8. **The first important reward is not left to chance.** Randomness may stay
   around it; the lesson may not stall for want of a drop. Start with an
   easy comparison and bring in trade-offs once comparing is understood.
   Never teach that a higher tier or a green number always means better.

9. **Disclose progressively, by emphasis.** Three things are different and
   must look different: detail available on demand, a function locked until
   later, and a function that is there while something else is emphasised.
   A lock is not a way of organising information.

10. **A lesson never holds the game up.** No "OK" advances the simulation.
    Guidance can be dismissed, skipped and found again. A skipped lesson
    never leaves the player with a broken build.

11. **Availability, eligibility and progress are three states.** Whether the
    mechanic exists; whether something happened that makes explaining it
    useful; whether the player saw, used or skipped the explanation. A
    feature is never locked because its explanation was not read.

12. **One focus at a time.** Lessons that become due together queue.
    Instruction and milestone reveals share one attention budget.

13. **Pace by understanding, not by the clock.** A prompt goes when its task
    is done, not when a timer runs out, and it survives an interruption.

14. **Teach the idle contract out loud.** What repeats by itself, what
    continues while away, what stops it, and what needs a decision. Never
    imply progress the game does not make.

15. **Early challenge is safe and still informative.** A mistake gives
    understandable feedback and a manageable recovery. Teach recovery before
    the player meets an unexplained stop. One new variable per beat.

16. **Juice confirms meaning.** Feedback says what happened and who caused
    it, in proportion to its importance, and never covers the thing being
    taught. An essential lesson does not depend on colour, sound, motion or
    a timed cue alone.

17. **Measure understanding, not completion.** Watch someone play without
    helping. Success is that they can explain the game and predict it, not
    that every hint was clicked or the opening was fast.

## For Horse Heroes

Checked against the game as built on 2026-10-11 (`docs/expedition.md`,
`docs/menu-shell.md`, `docs/item-card.md`, `src/game/mod.rs`,
`assets/data/rules.ron`).

- **The first trail needs no decision, and takes none.** A new game
  begins on the road (#292): the party is walking out of the gate when the
  window opens, with Fire chosen and one line, "You organise the gaggle.
  The horses walk, fight and collect loot by themselves.", on no control.
  The first town visit is the one after the first trail, and it waits for
  Set out (#305). There is no title screen and no "begin" button, and none
  should be added that must be pressed.
- **The first trail was not forgiving; now it cannot be lost.** On
  2026-10-11 a new player lost it about one time in three. Since #294 it is
  one authored plan that every set of abilities wins ([opening.md](../opening.md)).
  The second trail, the first rolled one, is won nearly always once the
  Starlit Halter is worn and seldom bare (owner, 2026-10-19: that stands),
  so equipping the first piece is what carries the party on.
- **The reward is nearly reliable already.** One piece drops per run if any
  encounter was cleared. What is not reliable is that it is an upgrade a
  new player can read at a glance.
- **Each lesson is tied to its real effect.** Focused Shot is first ready 4 s
  into fighting, Elemental Burst 5 s, Ward 2 s. A fight that ends sooner
  shows none of them, so a line about an ability waits for the ability.
- **The loot shower teaches collection by itself.** Nothing is drawn over
  it. Pets are not how loot is collected today and are not mentioned.
- **The item is the handle, and the selected horse is the context.** The
  first drop is where the card, the selected horse, the comparison and Equip
  are taught together, because that is where selection first has a purpose.
- **Nothing equips by itself** (owner, 2026-10-12, [vision.md](vision.md),
  "Builds are the player's"; #351). A drop left alone stays carried, and no
  lesson names a horse for it. Leaving it is still valid play; do not call
  it a mistake. (Built 2026-10-12; this file used to call auto-equip at
  setting out valid play.)
- **A hint has to be met** (owner, 2026-10-12, #353): the line under the
  status strip was not noticed in 0.2.2. A lesson is a pop-up by the thing
  it is about, and the game pauses until it is answered, on the road too.
  Where this file says a lesson never holds the party up, that is
  overruled for a lesson that is showing.
- **Every town hint asks for something to be done** (owner, 2026-10-19,
  after 0.2.5 put about ten pop-ups in a row in front of her with nothing
  to do but read them: "ALL in-town hints MUST require the user to do
  something: a hint should never just be a pop up with words that require
  no action, because a user will ignore it"). A town lesson names one
  thing to do on a real control and goes when it is done; it has no Got
  it. A line with nothing to do is cut, or folded into one that has.
- **The card gives numbers, not a verdict** (owner, 2026-10-12, #351): no
  lesson or card says a piece is an upgrade.
- **The 12 seconds are not a deadline to learn.** Setting out mid-read loses
  nothing: the piece stays carried, Equip
  says "in town only", and the lesson is offered again at the next town.
- **Do not oversell an upgrade.** A win raises enemy health and attack by
  15%, and enemy elements are rolled per trail. The next trail is not a fair
  comparison; say what the piece changed, and say separately that winning
  made the trail harder and the drops better.
- **Tell rarity by what was found.** Colour and shape of the piece in hand.
  Legendary and Set are more stat points and a beam until #33; do not
  promise powers or set bonuses.
- **Coins are named, not taught.** There is nothing to spend them on.
- **The party element is temporary.** It goes in 0.3.3 when elements move to
  abilities. One short line at the first resisted fight; no branch of the
  tutorial is built on it.
- **The permanent frame is the map.** Every place and planned button is
  already visible, so disclosure here is emphasis and explanation, not
  hiding. Order: scene and footer, status strip, Horses and Gear, Journal,
  then Trade and Map when they matter. Name a control with its printed key;
  teach Escape as "back one level" where it is first useful. Never send a
  new player to press an unbuilt control.
- **Nobody dies; say it early.** The costs of failure are taught when they
  exist (0.3.2), in the tone of [failure.md](failure.md).
- **Later systems join at their own first event:** route choice, a real
  Primer-to-Finisher reaction (it needs a fight long enough to happen),
  potions and failure, listing at the stall, cargo, prices, ability points,
  slot identities, the itinerary, offline progress, trade goods. The table
  is in the source document, section 8, and on #47.
- **Lessons are data.** They are a list in `assets/data/lessons.ron`,
  queued and shown one at a time, told what happens and never asked
  anything (#290; how it works is `docs/lessons.md`). The first item's
  are written (#296); the road's two are a starting set from this page,
  to be replaced by #295 and #299.
- **Progress is not kept** until saving exists (#44): the opening plays
  again on each launch, and nothing may claim otherwise.

What a player should be able to say after the opening:

- "I organise the party; I do not control a fight."
- "The horses collect their rewards by themselves."
- "I can inspect an item and choose who wears it."
- "The comparison is about the selected horse."
- "I can let the first trail happen without doing anything."
- "I know where to look for the next decision or an explanation."

## Source

Two documents received from the owner on 2026-10-11, kept whole in
`docs/source/`:

- "Horse Heroes: Tutorial and Opening-Pacing Design"
  ([tutorial-and-opening-pacing.md](../source/tutorial-and-opening-pacing.md)),
  written against `docs/mechanics-for-tutorial.md`. It calls its own
  recommendations proposals.
- "Designing the First Level of an Idle RPG"
  ([first-level-of-an-idle-rpg.md](../source/first-level-of-an-idle-rpg.md)),
  the general guide the first refines. Where it speaks of classes and of
  starting abilities chosen by the player, the first document corrects it:
  this game has neither at the start.

Both carry citation markers (`[web:15]` and the like) whose reference list
did not arrive. The work is under epic #288.
