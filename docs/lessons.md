# Lessons

What the game explains, and when (#290). The lessons are a list in
`assets/data/lessons.ron`; a queue in `src/sim/lessons.rs` shows one at a
time; `src/game/menu/teach.rs` puts the one showing on the hint's line
([menu-shell.md](menu-shell.md), "The hint's line"). Guidance:
[philosophy/onboarding.md](philosophy/onboarding.md), principles 5 and 10
to 13, and the owner's rule that a lesson is watched or done, "never, ever,
ever just a wall of text".

## A lesson

One entry of `lessons.ron`, whose header says the same in full:

| field | what it is |
|---|---|
| `id` | its name, by which a lesson put away is known again |
| `due` | the event that has to happen first |
| `shown` | `Town`, `Road`, or left out for either |
| `target` | the control its mark goes on, named as `places.ron` names it: `Place`, `Tab`, `Control`, `Horse(n)`, `Selected` (the selected horse, whichever it is now), or `Drop` |
| `words` | its one line, 85 letters at most |
| `done` | `Watched(event)`: it happens while the lesson shows. `Did(event)`: it happens once the lesson is due, shown or not |
| `instead` | the line for a `Drop` that is no longer in the cargo but on a horse |

Reordering, rewording, adding or removing a lesson is a change to that file
and nothing else. No code holds a lesson's name or words, and the tests of
the queue and of the menus use lessons of their own.

## Events

Plain facts (`Event`), told to the queue with `Lessons::raise`:

| event | raised by |
|---|---|
| `Fight`, `Ability`, `Kill`, `Resisted`, `Win` | the trail: `Expedition::events`, which `game::step` hands on each tick as it does the kills |
| `Gear` | `Game::return_to_camp`, when the trail brought a piece back |
| `Equipped` | `Game::act`, when Equip did something. Nothing else equips (#351) |
| `Card` | `teach::follow`, when a piece's card is pinned |
| `Selected` | `teach::follow`, when the selected horse changes |
| `Swapped` | `Game::act`, when Equip took a piece from the horse that wore it |
| `SetOut` | `Game::depart`, by Set out or by itself |
| `SentOut` | `Game::act`, when Set out is pressed. Keep going's own departure is not this |
| `Compared` | `teach::follow`, when the selected horse changes with a piece's card open |
| `Toured` | `teach::follow`, when every place of the bar has been opened in town, counted from when an entry of the list asks for it |

An event makes its lessons due the first time it happens and never again,
so a return to town does not replay the opening. A test holds that the game
raises every one of them.

## The queue

`Lessons` keeps four facts for each lesson and keeps them apart
(`Progress`): **due**, **seen** (it has been shown), **done** (watched or
did), **skipped** (put away). It also keeps the order in which lessons came
due (`listed`), for the Journal. A lesson can be done without having been seen:
a player who equips a piece unprompted is not then told how.

`Lessons::show(in_town, found)` is asked every frame which lesson to show.
`found` is where each lesson's target is at that moment, so the game is
looked at again every time:

- A lesson fits when it is due, not done, not skipped, lessons are not
  turned off, this is where it is shown, and its target is there. One the
  player asked for again fits whatever was done, put away or turned off.
- One that is showing and no longer fits goes back to waiting, and is not
  counted as put away: a town lesson when the party sets out, or one whose
  target has gone.
- With none showing, the first in the file that fits is shown. One showing
  is never pushed aside by another.

A `Drop` is the newest piece a kept report dropped that the party still
has (`Game::newest_piece`): in the cargo it is `Here`; on a horse it is
`Moved`, and the lesson says its `instead`; no longer the party's it is `Gone`, and
the lesson waits for the next piece. Its mark goes on the piece wherever it
is listed (`About::Piece`): the cargo, the report, the Gear tab, the
Inventory.

## It never holds anything up

The queue is told things and asked what to show. It is handed nothing of
the game: `raise` takes an event and `show` takes a list of where things
are. `Game::can` does not read it. Two tests hold this from outside: a
first trail and town visit with no input goes tick for tick as it does with
every lesson skipped, and every action can be done or not for the same
reason whatever the lessons' state.

## Put away, turned off, found again

Guidance that can be skipped can be found afterwards (#300;
`philosophy/onboarding.md`, principle 10).

- **Got it** (or Escape) puts the hint showing away. **Skip all hints**
  puts it away and turns hints off. Both are the shell's controls, and what
  is put away and whether hints are off is the shell's to say:
  `teach::follow` tells the queue each frame (`put_away`, `turn_off`).
- **Hints**, in Settings, is the same switch (`Switch::Hints` in
  `places.ron`), marked as chosen while hints are on. Turned off there, the
  hint showing leaves its line without being put away, so it is back when
  they are turned on. Off, lessons still become due and done behind it.
- **Journal, Hints** (J, 4; `content: Hints`) lists every lesson that has
  become due, in the order it came and never before its event: its line
  as it would be said now, and under it what became of it (`State`:
  showing now, done, put away, skipped with hints off, waiting its turn),
  where it applies and what it is on. Until saving (#44) the list starts
  empty on each launch, and says so.
- **A press on a row shows that hint again** on its control
  (`teach::again`, `Lessons::again`), in the place of any other, whether
  it was done or put away and with hints off too. It is then a hint like
  any other: it goes when its thing is done, when it is put away, or when
  its target is no longer there, and one that was done does not come back
  by itself.
- **A row that cannot be shown now says why** and is drawn as unavailable
  (`mark_lessons`): "A press shows it again on the road.", "... in town.",
  or "What it is about is not there now." A press on it changes nothing.
- A row for every lesson in the file is spawned once and shows while the
  list is that long (`When::Listed`), as the rest of the shell is shown
  and hidden and never rebuilt. `teach::listed` is what a row says.

Chosen without the owner: the player's word is "hint" throughout (the tab,
the switch), since that is what Got it and Skip all hints already call it;
the list is a fourth tab of the Journal; Hints in Settings has no key; a
row's second line and its wording.

## The things to try

A short list over the town scene (#303; the owner, 2026-10-11: "a quest
system that rewards the player with small amounts of gold for doing things
like clicking or equipping an item"). It is the `list` of `lessons.ron`, not
a general quest system, and it is how a thing is taught by doing it.

| field | what it is |
|---|---|
| `id` | its name |
| `due` | the event that puts it on the list |
| `words` | the thing to do: one line, 32 letters at most |
| `done` | the event that ticks it |
| `coins` | what it pays, once |

- **Ticked by doing, and by nothing else.** `Lessons::raise` ticks an entry
  when its `done` event happens while it is on the list and the party is
  in town (`arrive`, `leave`). What was done before it was listed does not
  count, what puts it on the list does not tick it in the same breath, and
  nothing done on the road counts. The events are raised by what the
  controls did, so a key and the pointer are the same.
- **Paid once, into the party's count.** A tick adds the entry's coins to
  what the list owes; `Game::bank_rewards`, called by `teach::follow` each
  frame, moves that into `Game::coins`, and `teach::teach` plays a coin's
  arrival for each coin, so it is heard and counted as a coin from a kill
  is. There is nothing to spend coins on yet.
- **Nothing has to be done.** Set out works with every entry open, and an
  entry not done is on the list at the next town. The list is there in town
  while an entry on it is open, stays for the visit in which the last one
  is ticked, and is gone from the next (`Lessons::list_showing`).
- **Its home** is the top right of the town scene, clear of the gate and
  the horses at 1120x640 (`show_list`; `to_try_width` in `menu.ron`). It is
  out of the shell's layout, so it moves no control, and the pointer passes
  through it: an entry is done on the real control, never on the list. The
  road has the fight log instead.
- **Hints off puts it away** (`Shell::list_in_view`), and on brings it
  back. Entries are still ticked and paid behind it.
- A later feature adds its entry here as its tutorial (CLAUDE.md, rule 5).

The entries today, all due when a trail first brings a piece back: select
another horse; press the new piece; with its card open, select another
horse; put the piece on a horse; open every place in the bar; set out.

Chosen without the owner: where it is; the amounts (2, 2, 3, 3, 5, 2);
that it waits for the first piece and is not there at launch; that it goes
with the hints; that entries count in town only; one entry for all six
places and not one each; its heading, "To try in town, if you like. Each
pays once."

## The lessons in the file

Written by the session from `philosophy/onboarding.md` ("For Horse
Heroes") and not approved line by line.

**The road, a starting set** until #295: the horses fight by themselves
(first fight, until a kill is watched); each has one ability (first
ability, until another is watched). The element's line is #299. `Kill`,
`Resisted` and `Win` are raised and no lesson waits for them yet.

**The first item** (#296), in town, each finished by doing the thing and
none of which has to be done:

1. The piece, where it is: press it and its card opens. If the player put
   it on a horse without opening its card, its other line says it is on a
   horse and the mark is on it where it is worn.
2. The selected horse, in the strip: the card's before and after are its.
   Finished by selecting another horse.
3. Equip: you choose who wears it; left alone it stays carried, and
   nothing is lost. Nothing equips by itself and no lesson names a horse
   for the piece (owner, 2026-10-12, #351).
4. Once a piece is on, Undo: it takes it back until the party sets out.
   Finished by setting out.
5. Only after Equip has taken a piece from the horse that wore it: the
   horses exchanged pieces and nothing is lost. Never before a swap.

A player who presses nothing is shown the first and no other, and the
party leaves as it would have. Coins are not mentioned. Whether a
first-time player can then say whose stats the comparison is about is
#301's to find out.

## Looking at it

`--seed 6 --keep-going` at `IDLE_SHOT_AT=17` is the first lesson on the
road, at 24 the second, at 80 the piece's in town
([gallery](gallery/2026-10-12-lessons/README.md)).
`IDLE_LESSON=<folder>` with `--seed 6` walks the first item's lessons by
keys, ticking four of the things to try on the way and a fifth at its end
([gallery](gallery/2026-10-13-things-to-try/README.md)), and saves a picture of each
([gallery](gallery/2026-10-12-first-item/README.md)). F6 puts a sample hint
over whatever lesson is showing and takes it away again; the lesson comes
back after it. The dev key also brings back every lesson put away. The
walk then opens Journal, Hints, shows the piece's hint again with Enter,
turns hints off in Settings and goes back to the list
([gallery](gallery/2026-10-12-hints-found-again/README.md)).
