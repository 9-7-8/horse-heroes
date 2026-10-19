# The opening

The design of a new game's first trail and first town visit (#293), written
for the owner to correct. It is a sheet to check every lesson against, and
the plan #294 builds the authored trail from and #295 rewrites the road's
lessons from.

**Status: a draft, not approved.** Nothing here is her decision unless it
says so. Her rulings that bind it are in [vision.md](philosophy/vision.md),
Tutorials: the game starts on the road and combat is learned by watching;
the first trail is authored, one fixed plan as data, under the ordinary
rules; it cannot be lost, by tuning alone, and a test holds that; no town
leaves by itself; lessons play again on every launch until saving (#44); a
lesson is a pop-up by its thing and the game waits on it (#353); a tutorial
is "never, ever, ever just a wall of text". The advice is
[onboarding.md](philosophy/onboarding.md); the worksheet is section 14 of
[the source](source/first-level-of-an-idle-rpg.md).

Measured on 2026-10-17 against the game as built (0.2.4 and #353), with the
sim's own fight run over 1000 seeds. The measuring was a throwaway test and
is not in the code; #294's tests are what will hold these numbers.

## The worksheet

1. **Player role.** The organiser of a gaggle of three horses. No avatar
   and nothing to do in a fight. The player decides in town: who wears
   which piece, which ability each horse carries, and when the party sets
   out.
2. **Emotional promise.** Confidence and delight: "my horses can handle
   the road, what they bring back is a joy to watch land, and I can help
   them do better."
3. **Starting state.** On the road, the party walking out of the first
   town's gate (#292). Juniper carries Ember Shot (Fire), Morrow Frost
   Burst (Ice), Sable the Ward. No gear, no coins, a level 0 trail. A
   working party: it wins the first trail with no help.
4. **First objective.** The end of the trail: its boss. The footer's trail
   tracker shows the way there, four marks, the last one bigger.
5. **First ability.** Sable's Ward, two seconds into the first fight, then
   Ember Shot and Frost Burst in the second. The rule shown: each horse's
   ability fires by itself whenever it is ready, and each looks, sounds and
   hits its own way (#18, #358).
6. **First reward.** The trail brings back about a dozen pieces, one of
   them the trail's own, the same piece in every new game. It makes the
   first decision possible: who wears what.
7. **First improvement.** Seen as numbers, never as a verdict (#351): the
   card's before and after on the selected horse, and once a piece is on,
   "attack 6 to 10 and 2 more with it on" (the Starlit Halter on
   Juniper, below) on that horse (#298). On the
   second trail its hits are bigger. That trail is level 1 and its enemies
   15% stronger, so the improvement is said apart from the level, never as
   "the next fight is easier".
8. **First variation.** Inside the first trail, one fight: the Mire Toad
   is Ice, and Juniper's Ember Shot is resisted there, its hit small and
   pale beside the Strong ones before it (her ruling, 2026-10-17). In the
   first town the resisted-ability lesson points at Juniper. Then the
   second trail, rolled by the generator: another order of fights, other
   enemies, rolled elements, the first thing the opening did not show.
   Transfer is shown by the player changing a piece or an ability for a
   reason they can say (#301).
9. **Automation contract.** By itself: walking, fighting, abilities,
   collecting the loot, walking home. Waits for the player: setting out
   from town, every time, unless Keep going is on (it then leaves after
   12 s, until a trail is lost; it stands in for the itinerary, #201). Stops
   the game: a lesson's pop-up, until it is answered, and nothing else.
   Never needed: anything on the road.
10. **Exit condition.** The player sets out from the second town visit
    with no lesson showing, having either put a piece on or chosen to leave
    them carried, and can say the six things at the end of
    [onboarding.md](philosophy/onboarding.md), "For Horse Heroes". #301
    checks it with somebody who has not been told.

## The first trail

One fixed plan, kept as data (#294): the same fights, enemy elements,
no spring, and the same trail's piece in every new game, whatever the seed. The seed
still moves what the ordinary rules roll: who an enemy strikes, each swing's
timing, coins, and the pieces the kills throw.

| # | Fight | Element | Why |
| --- | --- | --- | --- |
| 1 | One Thornling | Plant | One small enemy: the fight is read whole. Long enough (about 4 s) for the Ward, not for the shots |
| 2 | Three Gloom Sprites | Fire | A group, for Frost Burst: Morrow's first use hits all three at once, each Strong. Ember Shot fires twice here |
| 3 | One Mire Toad | Ice | The hardest hitter of the three: the Ward's halved hits are seen, and each ability fires once. Ice resists Fire, so Ember Shot is resisted here (her ruling, 2026-10-17: one fight resists one ability); Frost Burst meets its own element and is plain |
| | ~~A fairy spring~~ | | ~~Heals the party by 30% of its health~~. *No spring (her ruling, 2026-10-17): the Yearling is tuned down instead, so no horse falls* |
| 4 | A Corrupted Yearling, the boss | Fire | The full fountain and the trail's piece. Long enough (12 to 14 s) for every ability to fire two or three times |

**The Yearling** is a new enemy in `enemies.ron`, not a change to the
Stag: the Stag's body and death, health 200, attack 6, defense 1, a blow
every 2 s, coins 10 to 15, 2 to 4 pieces, the full fountain. The two
finals as they are cannot be the first boss: with the Stag, the starting
party lost one first trail in four; with the Wolf and a spring before it,
some sets of abilities still lost. Name and numbers are proposals.

**Elements.** One fight resists one starting ability (her ruling,
2026-10-17), so elements are met on the first trail and taught in the first
town. Strong comes first: Ember Shot meets Plant and Fire in fights 1 and
2, Frost Burst Fire in fight 2, and those hits are seen as Strong (coloured
numbers, "Strong!" in the log). Then fight 3's Toad is Ice: Ice beats
Fire, so Ember Shot's hit there is resisted, the smallest and palest number
of the trail and "Resisted" in the log, right after the biggest. Frost
Burst meets Ice there and is plain. The boss is Fire again, so the trail
ends on Strong hits.

Why this fight and this ability. Fight 3 is the last before the boss, so
the contrast comes after both abilities have been seen to hit hard, and
the boss's fight is not spent on it. Ember Shot fires every 4 s and fight
3 lasts more than that, so it is resisted at least once in every run;
Frost Burst against a Plant enemy would also do, but Plant is fight 1,
which ends before Frost Burst's first use, and a Plant group in fight 2
would spoil Frost Burst's first, Strong hit on three. Juniper is resisted,
so the town's lesson points at Juniper, and the change it asks, Abilities
[2], has two others to choose between. The element is data in #294's plan,
as is the fight.

The Toad at Ice lives a little longer than at Fire (Ember Shot does three
quarters, Frost Burst no bonus), which #294's no-fall test measures with
the Yearling's tuning.

**As built** (#294, 2026-10-18): the Yearling is health 200, attack 3, defense 1, a
blow every 2.8 s. Over 1000 seeds the starting party wins every run, loses
no horse (6 health is the least any horse is left with), uses all three
abilities and has Ember Shot resisted; the boss fight lasts 11.7 to 13.8 s.
Every one of the 64 sets of abilities wins every one of 100 seeds, but a
horse falls in about 20 of those 6400 runs, all in the boss fight or the
Toad's: with some sets (Frost Burst on Juniper, few damage abilities)
Morrow reaches the boss with almost nothing left, so no tuning of the boss
alone keeps every set standing. She ruled (2026-10-19, #371): no fall is
held for the starting party; every set must still win.

*The numbers below are from the draft that had a spring and a Fire Toad.*

**What was measured**, on today's rules, with the Yearling, the spring and
a Fire Toad:

- The starting party wins every run, and no horse falls. Least health left
  at the end over 1000 seeds: Juniper 19 of 40, Morrow 1 of 30, Sable 14 of
  35. Without the spring Morrow fell in 13 runs of 1000: the spring is
  there for that.
- Every set of abilities the party may carry, all 64 of them, wins every
  one of 200 seeds, measured without the spring; three Wards and no damage
  ability is the closest, with 12 health left.
- Fight 1 lasts about 4.2 s, fights 2 and 3 five to six, the boss 12 to 14;
  the trail, walks included, 43 to 45 s of game time, before pauses. The
  first kill comes about 8 s after the window opens.
- The Ward is raised in every fight; about seven enemy hits a trail are
  Warded. Ember Shot first fires in fight 2 (in fight 1 in 6 runs of 100),
  Frost Burst first in fight 2, every run.

**Drops.** The kills throw what they throw at the ordinary kill odds,
Normal and Magic, 8 to 15 pieces in all (11.6 on average), so the choice
among several (#352) is there every time. The trail's own piece is fixed: a
**Starlit Halter**, Rare (yellow, the ringed stone), attack +4, defense +2,
health +10, eight points, with a look rolled once from a fixed seed. Rare
because it is the one tier above anything a kill can throw, so it stands out
by colour and shape without a word calling it an upgrade; every horse starts
bare, so it improves whoever wears it, and so does every other piece.
Comparing one worn piece against another, the first trade-off, comes after
(onboarding, principle 8).

**After it.** From the second trail the generator and the ordinary balance
apply. `first_trail_balance` then measures a level 1 trail, the one a new
player meets second.

## The beat sheet

A beat is a lesson of `lessons.ron`, or an entry of its list of things to
try; every lesson and entry is one beat here. The words are the file's, as
they stand. "Seen" is how the player sees the connection.

### On the road

| Beat | Event, when | Rule | Behaviour | Feeling | Seen | Words |
| --- | --- | --- | --- | --- | --- | --- |
| the horses go by themselves | `SetOut`: the window opens on the party leaving the gate | The player organises; the road runs itself | Watch, press nothing | Welcome, calm | The horses walk off with no input | "You organise the gaggle. The horses walk, fight and collect loot by themselves." |
| the party fights by itself | `Fight`: the Thornling, about 4 s in | Fights are automatic | Do not hunt for an attack button | Reassured | The horses strike with no input | "The horses fight by themselves. Nothing in a fight needs a press." |
| abilities fire by themselves | `Ability`: Sable's Ward, 2 s into fight 1 | Each horse's ability fires when ready | Watch for each one | Curiosity | The Ward's bubble over the party, its sound | "Each horse has one ability, and uses it by itself when it is ready." |
| the numbers are hits | `Kill`: the Thornling, about 8 s in, once its loot has landed | Hits are sized by damage, coloured by element | Read the numbers | Understanding | The numbers just seen | "Each number is a hit, sized by damage, coloured by element; a minus is a horse hit." |
| speed | `Kill`: the first of fight 2 (her ruling, 2026-10-17) | Faster horses hit more often | Notice Juniper's pace | Interest | Juniper's hits come most often | "The faster a horse, the more often it hits. The party walks at their average speed." |
| the boss | `Boss`: the Yearling's approach | The last fight has the best loot | Watch it | Anticipation | Its banner and bar, its approach | "The trail's boss: its last fight, and the one with the best loot." |

Between them, with no line: the first loot landing and drawn in (the
opening's line has said it is collected), Ember Shot and Frost Burst first
seen in fight 2, the boss's fountain and the Starlit Halter's
fall, the walk home.

### In the first town

The party waits at the gate until Set out (#305). All of these are done by
doing the thing, and none has to be.

| Beat | Event | Rule | Behaviour | Feeling | Seen | Words |
| --- | --- | --- | --- | --- | --- | --- |
| press the piece | `Gear`: back in town | The item is the handle | Press it | Reward | Its card opens | "The trail brought this back. Press it and its card opens." |
| the card is about the selected horse | `Card` | Before and after are the selected horse's | Press another horse | Control | The numbers change with the horse | "The selected horse: the card's before and after are its. Press another horse." |
| choose among them | `Several`: always, on this trail | The choice is the player's | Open another piece | Plenty | Another card | "The trail brought back more than one piece. Open another's card: you choose." |
| equip or leave it | `Card` | Nothing equips by itself | Put one on, or leave it | Ownership | The piece worn on the horse (#359) | "You choose who: Equip's, or another from this list. Left alone, it stays carried." |
| it is on | `Equipped` | What a piece changed, and Undo | Look at the horse | Achievement | The horse's numbers, as they were and are | "{change} with it on. Undo takes it back until you set out." |
| a resisted ability | `Resisted`: the Toad, fight 3; shown in town, on Juniper | An ability's element can be resisted, and the player can change it | Select Juniper, Abilities [2], pick another | Insight | The small pale number seen on the road; the ability list | "Its ability's element was resisted. Select it, then Abilities [2] to change it." |
| a swap loses nothing | `Swapped`, only after one | Swapping is safe | Swap freely | Safety | Both horses' pieces | "The two horses exchanged pieces. Nothing is lost." |
| a win raises the level | `Win` | The next trail is stronger, and so are its pieces | Expect it | Honest stakes | "Level 1 trail." on the footer | "A win raises the trail level, now {level}: enemies gain health and attack, pieces stats." |
| it can be left | `Win` | The party can go on by itself | Turn on Keep going, or not | Freedom | Keep going on the Map | "Keep going here sends the party out by itself till a trail is lost: leave it to it." |

The things to try, over the town scene, each paid once: select another
horse (2), press the new piece (2), with its card open select another horse
(3), put the piece on a horse (3), open a piece and press Details (2, once a
card is seen), drag a piece onto a horse (2, once one is on), open every
place in the bar (5), set out (2). Their behaviour is the doing; the
feeling, a small coin's worth of reward for trying each control.

The town shows one lesson at a time, the first in `lessons.ron`'s order
whose thing is there, so the resisted ability comes after the gear lessons
that fit and before the two that follow a win. Changing Juniper's ability is
a choice, not the answer: Ember Shot is Strong on the boss, and the second
trail's elements are rolled.

### Later than the opening

Nothing: every lesson of `lessons.ron` today is met in the opening. The
second trail is the first variation (worksheet, 8), with no lesson of its
own.

## What is not taught, and when it joins

| Not in the opening | Joins | Where |
| --- | --- | --- |
| Elements: the whole of what beats what | Seen, not said: the opening shows Strong and one resist; the triangle is learned on later trails' rolled elements | the resisted-ability lesson is the only line |
| Losing a trail; nobody dies | The first fall or loss: never on the first trail | #176, #178 (0.3.2) |
| Saving, and lessons kept | — | #44 (0.3.2), #47 |
| Primers, Finishers, reactions | The first real reaction | #94, #95 (0.3.3) |
| Cargo and listing at the stall | The first spare piece to sell | #34 (0.4.0), #147 (0.4.1) |
| Ability points and levels | The first level up | #220 (0.6.1) |
| The itinerary; offline progress | The second town; the first time away | #201 (0.7.0), #31 (0.7.3) |
| Coins | Named, not taught: nothing to spend them on yet | |
| The fairy spring | A later trail; not on the first (her ruling, 2026-10-17) | |

## For her to rule on

My calls in this draft, each easy to change:

- **The boss.** A new, weaker final for the first trail only (the
  Yearling), rather than tuning the Stag or the Wolf, which every later
  trail uses. *Confirmed by her 2026-10-17 (vision.md).*
- **The spring** on the first trail, so that no horse falls in it. Without
  it the trail is still never lost, but Morrow sometimes falls.
  *Overruled by her 2026-10-17 (vision.md): no spring; the Yearling is
  tuned down until no horse falls.*
- **The trail's piece**: a Rare Halter, the same in every game, above any
  kill's tier. *Confirmed by her 2026-10-17 (vision.md).*
- **No resisted hits** on the first trail, so elements wait for the second.
  *Overruled by her 2026-10-17 (vision.md): one fight resists one ability,
  and the resisted-ability lesson plays in the first town. Redrawn for it:
  the Mire Toad is Ice and Ember Shot is resisted (Elements, above; which
  fight and element were left to the build, and are mine).*
- **Two pop-ups after the first kill**: the numbers, then speed, back to
  back. I would move speed to the second kill or the second trail; it is
  the only place the opening stacks two. *Confirmed by her 2026-10-17
  (vision.md): speed moves to the first kill of fight 2.*
- **Against #295 as written**, which this sheet would change: it names
  Focused Shot and Elemental Burst, which are now Ember Shot and Frost
  Burst (#357); it wants a line for each ability naming its horse, where one
  line on the first and the abilities' own looks and sounds for the rest
  keeps the road to six pauses; it wants a line for the trail tracker,
  which needs an event of its own (a fight cleared) and would be a seventh;
  and its loot line is said already by the opening's. #294's "equipped by
  itself if left alone" was overruled by #351 and goes. *Confirmed by her
  2026-10-17 (vision.md): six road pauses, as this sheet has them.*
