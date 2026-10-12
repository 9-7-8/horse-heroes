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
| `target` | the control its mark goes on, named as `places.ron` names it: `Place`, `Tab`, `Control`, `Horse(n)`, or `Drop` |
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
| `Equipped` | `Game::act`, when Equip did something. The gate's own equipping at setting out is not the player's and does not count |
| `Card` | `teach::follow`, when a piece's card is pinned |

An event makes its lessons due the first time it happens and never again,
so a return to town does not replay the opening. A test holds that the game
raises every one of them.

## The queue

`Lessons` keeps four facts for each lesson and keeps them apart
(`Progress`): **due**, **seen** (it has been shown), **done** (watched or
did), **skipped** (put away). A lesson can be done without having been seen:
a player who equips a piece unprompted is not then told how.

`Lessons::show(in_town, found)` is asked every frame which lesson to show.
`found` is where each lesson's target is at that moment, so the game is
looked at again every time:

- A lesson fits when it is due, not done, not skipped, this is where it is
  shown, and its target is there.
- One that is showing and no longer fits goes back to waiting, and is not
  counted as put away: a town lesson when the party sets out, or one whose
  target has gone.
- With none showing, the first in the file that fits is shown. One showing
  is never pushed aside by another.

A `Drop` is the newest piece a kept report dropped that the party still
has (`Game::newest_piece`): in the cargo it is `Here`; on a horse it is
`Moved`, and the lesson says its `instead`; left behind it is `Gone`, and
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

## Put away, skipped, found again

Got it (or Escape) and Skip all hints are the shell's controls;
`teach::follow` tells the queue what was pressed (`dismiss`, `skip_all`).
With all skipped, lessons still become due and done behind it.
`Lessons::restore` shows them again, those put away too; the control for
that, and finding a lesson in the Journal, is #300. Progress is kept in
memory only, so the lessons play again on each launch until saving (#44).

## The four in the file

A starting set, written by the session from `philosophy/onboarding.md`
("For Horse Heroes") and not approved line by line: the horses fight by
themselves (first fight, on the road, until a kill is watched); each has
one ability (first ability, until another is watched); press the piece
(first piece home, in town, until a card is opened); Equip, or leave it
(first card, until a piece is equipped). The first trail's lessons proper
are #295, the first item's #296, the element's #299. `Kill`, `Resisted`
and `Win` are raised and no lesson waits for them yet.

## Looking at it

`--seed 6 --keep-going` at `IDLE_SHOT_AT=17` is the first lesson on the
road, at 24 the second, at 80 the piece's in town
([gallery](gallery/2026-10-12-lessons/README.md)). F6 puts a sample hint
over whatever lesson is showing and takes it away again; the lesson comes
back after it. The dev key also brings back every lesson put away.
