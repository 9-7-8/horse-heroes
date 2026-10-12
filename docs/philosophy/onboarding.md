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
(#303). The statement is [vision.md](vision.md), Tutorials.

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

- **The first trail needs no decision, and already takes none.** A new game
  begins in town with Fire chosen and waits there for Set out (#305; it
  left by itself after 12 seconds until then). So the party does not start
  by itself today: a new game that starts on the road is #292. There is no title screen and no
  "begin" button, and none should be added that must be pressed.
- **The first trail is not forgiving.** The win rate at trail level 0 with
  no gear is 68%, and 44% with the worst element for the trail
  (`first_trail_balance`). A new player has about one chance in three of
  losing the trail that is meant to build confidence. A loss costs nothing
  yet, and nothing says so.
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
- **Auto-equip is valid play.** A drop left alone goes on the horse it helps
  most at setting out, or is left behind. Say so; do not call it a mistake.
  It also hides the decision, which is why the first-item lesson exists.
- **The 12 seconds are not a deadline to learn.** Setting out mid-read loses
  nothing: the appraisal and the road's log say where the piece went, Equip
  says "in town only", and the lesson is offered again at the next town.
- **Do not oversell an upgrade.** A win raises enemy health and attack by
  15%, and enemy elements are rolled per trail. The next trail is not a fair
  comparison; say what the piece changed, and say separately that winning
  made the trail harder and the drops better.
- **Tell rarity by what was found.** Colour and shape of the piece in hand.
  Legendary and Set are more stat points and a beam until #33; do not
  promise powers or set bonuses.
- **Coins are named, not taught.** There is nothing to spend them on.
- **The party element is temporary.** It goes in 0.3.1 when elements move to
  abilities. One short line at the first resisted fight; no branch of the
  tutorial is built on it.
- **The permanent frame is the map.** Every place and planned button is
  already visible, so disclosure here is emphasis and explanation, not
  hiding. Order: scene and footer, status strip, Horses and Gear, Journal,
  then Trade and Map when they matter. Name a control with its printed key;
  teach Escape as "back one level" where it is first useful. Never send a
  new player to press an unbuilt control.
- **Nobody dies; say it early.** The costs of failure are taught when they
  exist (0.3.1), in the tone of [failure.md](failure.md).
- **Later systems join at their own first event:** route choice, a real
  Primer-to-Finisher reaction (it needs a fight long enough to happen),
  potions and failure, listing at the stall, cargo, prices, ability points,
  slot identities, the itinerary, offline progress, trade goods. The table
  is in the source document, section 8, and on #47.
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
