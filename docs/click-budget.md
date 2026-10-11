# The click budget

The jobs done most often in the menus, the most actions each may take, and
what each takes today (#236). The guidance is
[philosophy/menus.md](philosophy/menus.md), "Count the clicks": ease is
measurable, and without a number "intuitive" is whatever the last person to
touch a screen thought.

An action is one press: a click, or a key going down. **A budget is a
ceiling, not a target.** Never cut an action at the cost of the player
understanding or being safe; time to understand matters more than time to
click.

## The table

Walked 2026-10-11 at `--seed 6`, in a window of 1120x640, by
[the walk](#the-walk).

| Job | Counted from | Budget, mouse | Took | Budget, keys | Took |
| --- | --- | --- | --- | --- | --- |
| Equip an upgrade the report just showed | in town, the Journal open on the report | 2 | 2 | 2 | 1 |
| List everything not wanted | in town, Horses open on Gear | 3 | cannot be done yet, #99 | 3 | cannot be done yet, #99 |
| Give a horse a different ability | in town, Horses open on Gear | 3 | 2 | 3 | 1 |
| Set out for the next town | in town, any place open | 1 | 1 | 1 | 1 |
| Find out why a piece was left behind | in town, Horses open on Gear | 2 | 1 | 2 | 1 |
| Find out why something did not sell | in town, Horses open on Gear | 2 | cannot be done yet, #170 | 2 | cannot be done yet, #170 |

No job that can be done is over its budget.

**The budgets are the owner's**, set on 2026-10-11 after this first walk.
They are in `assets/data/jobs.ron`, with where each count starts from, and
nowhere else: the walk reads them there.

## What the presses are

| Job | Mouse | Keys |
| --- | --- | --- |
| Equip an upgrade | the piece in the report, which opens its card; Equip on the card | E |
| A different ability | the Abilities tab; Ice | I |
| Set out | Set out, in the footer | Space |
| Why a piece was left behind | Journal, which opens on the report | J |

- **Equip** puts the piece on the selected horse. Another horse is one more
  press, by either way (its row, or Z, X, C). From Horses, with the cargo
  in view, the count is the same: the piece in the cargo, then Equip; or E.
- **A different ability** is the slice's stand-in: the party has one
  element and no abilities yet (#220, #222). The count will be taken again
  when a horse has abilities of its own, and the budget of 3 is sized for
  that: the horse, the tab, the ability.
- **Why a piece was left behind** is answered before it happens: the report
  says what becomes of the drop at setting out, on which horse, and by how
  many points ([item-card.md](item-card.md), "The appraisal in camp").
  After the party has left, the report is one more press behind the latest
  (Earlier, or `[`).

## A press on the scene

On the road, a press on the scene closes the open place (#307). None of the
six jobs is done on the road, so it changes no count in the table. Where it
saves a press: a place open with its side panel open is closed by one press
on the scene, where Escape takes two (the panel, then the place). With no
side panel open it is one press either way: the scene, the place's own tab,
or Escape.

## The walk

    $env:IDLE_WALK = "screenshots/walk"; cargo run -- --seed 6 --mute *> run.log

The game does each job that can be done, once with the presses a pointer
makes and once by keys, and writes `counts.txt` into the folder, with a
picture of where some of the jobs end. The routine is
`procedures/walk-the-click-budget.txt`. It is `walk_steps` in
`src/dev_screenshot.rs`, on the machinery of the menu tour
([menu-shell.md](menu-shell.md), "Looking at it").

- **Only the job is counted.** The presses between a job's start and its end
  are counted; those before put the menus where the count starts from, and
  those after take back what the job changed (Undo).
- **A job is checked in the game, not on the screen.** Equipped means the
  piece that was in the cargo is worn; a different ability means the
  party's element changed; set out means the party has left; the report
  means the Journal is open on it at the latest report. A job that did not
  come out so is written "NOT DONE", whatever it counted.
- **A pointer presses only what is in sight.** A press by pointer on
  something that is not laid out makes its job "NOT DONE". Where a control
  is in more than one place (the card's), the one in sight is pressed.
- **A job by keys is keys alone**, and a job by pointer has no key in it.
  A test holds the walk to that, and to walking every job of `jobs.ron`
  that does not wait on an issue, both ways.
- A job over its budget is written "OVER".

What the walk is not: a hand on a mouse. Its presses are the `Activate` a
click sends, so hover, aim and the size of a target are not measured. And it
is not a newcomer: which place they try first, the wrong turns, the
backtracking and whether they can predict a batch action are watched, not
counted (menus.md).

## When to walk again

Whenever a place or a tab is added, a control is moved in `places.ron`, or
a job's system is built. A job whose system lands loses its `waits_on` in
`jobs.ron`, gets its steps in `walk_steps`, and the table above gets its
count; the test fails until the walk has it.

## Not here yet

- List everything not wanted: #99, and the keep rules of #52.
- Why something did not sell: the stall report, #170.
- The newcomer's tasks from the owner's guide (say what the party is doing
  and whether anything blocks it; compare two pieces and equip one; protect
  a piece and list routine loot safely; change a keep rule and predict its
  result; come back from a side panel without losing their place; read a
  report and choose a next action): these need a person watched.
