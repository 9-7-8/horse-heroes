# The menu shell

The frame every menu screen hangs on (#226): the bar of five places, the
status strip and the footer, which never move, and between them the place
that is open. The guidance behind it is
[philosophy/menus.md](philosophy/menus.md), "The map of the menus" and
"Things stay where they are". It is built from the parts in
[menu-parts.md](menu-parts.md). Pictures: [gallery](gallery/README.md).

## Where things are

| File | What it holds |
| --- | --- |
| `assets/data/places.ron` | The map of the menus: the places in order, the tabs of each, the line saying what is decided there, and every control planned for each (#238) |
| `assets/data/menu.ron` | Sizes of the shell's columns, and `layout`: the scene's share of the window in each layout and the minimum window size; `time.layout`: how long the move takes |
| `src/game/menu/shell.rs` | The shell: its state (`Shell`), its layout, and the systems that keep it showing the game |
| `src/game/mod.rs` | `SceneView`, which layout the window is in and how far between them; `Action`, `Game::can` and `Game::act`, what a working control does; the report text shown in the Journal |
| `src/game/scene.rs` | `frame`, which sizes the camera to the layout every frame |

## The layout

    +--------------------------------------------------+
    |        3D scene (top third, SCENE_SHARE)         |
    +--------------------------------------------------+
    | Horses  Trade  Map  Town  Journal       Settings |  bar
    | coins  bags  hold    A  B  C                 day |  status strip
    +--------------------------------------------------+
    | tabs of the open place                           |
    | [horses] | the open tab | [cargo or side panel]  |
    +--------------------------------------------------+
    | trail  what the party is doing       [ Set out ] |  footer
    +--------------------------------------------------+

The scene is not a separate viewport. The camera's projection covers the
whole window, sized so the scene's share is as many world units tall as the
layout asks, and the shell is drawn over the rest. The split is a proportion
of the window's height, so it holds at any window size.

## The two layouts

| | Town | Road |
| --- | --- | --- |
| When | In town; on the road with a place open | On the road with every place closed |
| Scene's share of the height | `layout.town_share`, a third | `layout.road_share`, four fifths |
| World units of height in view | `SCENE_HEIGHT`, 7 | `ROAD_HEIGHT`, 12 |
| Where the aim point sits in the scene | the middle | about a third of the way up |

    +--------------------------------------------------+
    |                                                  |
    |          3D scene (road_share), with the         |
    |          room above the party for drops          |
    |                                                  |
    +--------------------------------------------------+
    | Horses  Trade  Map  Town  Journal       Settings |  bar
    | coins  bags  hold    A  B  C                 day |  status strip
    | one line saying how to open a place              |
    | what the party is doing              [ Set out ] |  footer
    +--------------------------------------------------+

On the road the scene is nearer (about 1.4 times) and, more to the point,
taller: with the aim point low, about ten world units stand clear above
the trail where a kill happens. That is the room a fountain (#4, #8) is
sized against.

Which layout is wanted follows one thing, `Shell::is_open`:

- Setting out closes the open place; arriving opens it again, as it was
  left. **Nothing else moves the layout by itself.** Arriving is something
  the player sees: when a trail is over the party walks back, for as long
  as it walks between two encounters, the town gate comes up the trail, and
  the place opens as the party stops at it (#246). Until then the footer
  says "On the way back to town." A drop does not move the layout: gear
  cannot be used before town.
- On the road, a press on a place in the bar opens it. A press on the place
  that is open, or Escape with no side panel open, closes it.
- In town a place is always open.

`SceneView` (in `src/game/mod.rs`) holds how far the window is from the town
layout (0) to the road's (1). `move_scene` in the shell steps it towards what
is wanted over `time.layout` seconds and sets the shell's top edge; `frame`
in the scene sets the camera. Both read the same eased value, so the scene
and the menus move as one and there is no cut. The place's area is clipped:
a place is covered and uncovered as the scene moves, not squeezed. With every
place closed the area holds one line saying how to open one.

The window cannot be made smaller than `layout.min_width` x
`layout.min_height` (1120 x 640): the width the status strip needs, and the
height at which the bar, the strip, that one line and the footer fit under
the road's scene. In a window too narrow for its height the scene shows more
height, so that `SCENE_WIDTH` world units, the party and its enemies, stay
in view.

## What stays put

- **The bar.** One `tab` part per place, in the order of `places.ron`, and
  Settings at the far end. Settings is a page like a place and not one of
  the five: it opens between the strip and the footer as they do.
- **The status strip.** Coins, bags, hold, the three horses with their
  health, and the day. Bags, hold and the day show "-" and keep their
  positions until the systems behind them exist. A horse in the strip is a
  control: pressing it selects that horse.
- **The footer.** First the trail tracker, then, in town: what the party
  is ready for, the countdown, and the footer's controls from the map, Set
  out last. On the road: where the party is, and Set out unavailable.

## The trail tracker

How many fights are left before the boss, seen without reading (#253). It
is the first thing in the footer, so nothing beside it can move it, and it
is there in both layouts.

    [#] [#] [o] < >  boss     two behind, the third being fought, the boss ahead

One mark per fight of the trail's plan, in order (`trail_mark` in
`parts.rs`, sized by `size.trail_mark` and `size.trail_boss`):

| The fight is | Its mark |
| --- | --- |
| behind the party | solid |
| the one the party is at, walking to it or fighting | a thick ring in the accent colour with a dot in it |
| ahead | a thin outline |
| the last, the boss | the same, larger and stood on a corner, with the word "boss" after it |

So the states differ by shape and not by colour alone. In town the marks
are the next trail's, all ahead. A won trail is all solid on the walk home;
a lost one keeps its ring on the fight that stopped the party.

`trail_stops` in the shell turns the game into the marks, from
`plan.encounters`, so a longer trail has more of them; `show_trail` redraws
them only when that answer changes. A test holds the count and the order
for trails of one to eight fights.

The footer's sentence in town no longer counts the encounters: the tracker
does, and with both the sentence wrapped at the smallest window.

## How a place keeps its state

All five places, and every tab of each, are spawned once at startup and only
shown or hidden. Nothing is rebuilt on a visit, so a place is found as it was
left, whatever it comes to hold. A page that is not showing is both out of
the layout (`Display::None`) and hidden (`Visibility::Hidden`): the first
takes its room away, and the second is what Bevy's Tab navigation looks at,
so Tab passes over its controls (#244).

What is showing is decided by one resource, `Shell`:

- `place`: the open place.
- `horse`: the selected horse. There is one, for every place.
- a tab per place.
- a detail per place: the thing open in its side panel, the tab it was
  opened from, and the control that opened it. It shows only on that tab,
  and is still there on return. Close or Escape closes it and puts the
  keyboard focus back on the control that opened it.

Three systems follow it each frame: `show_place` shows and hides,
`mark_chosen` puts `Selected` on the current place, tab, horse and slot (and
makes Set out unavailable on the road), and `show_facts` fills in every text
that follows the game. A text, counter or row that should follow the game
carries `Live(Fact)`; `fact` is the one place that turns game state into
words.

One observer, `on_activate`, handles every control of the shell: what a
press does depends on which marker the control carries (`PlaceButton`,
`TabButton`, `HorseButton`, `SlotRow`, `ClosePanel`, `ControlButton`).

## Every planned control

The owner, 2026-10-10: "All buttons should be there, even if they're not
functional, as early as possible." So `places.ron` lists every control
planned for every tab, for the cargo column (`cargo`), for a place without
tabs and for Settings (`controls`), and for the footer (`footer`). The shell
makes a button for each, in the order of the file, in a row that wraps.
Renaming, reordering or moving a control is a change to that file only.

A control in the file is:

| Field | What it is |
| --- | --- |
| `name` | What the button says |
| `does` | What it does, or will do, in one sentence |
| `issue` | The issue that will make it work |
| `proposed` | That issue is a proposal the owner has not decided |
| `action` | What it does today: `Element(Fire)`, `EquipDrop`, `Swap`, `SetOut` |
| `shows` | What a press opens in the side panel of its page: `Keys` |
| `key` | The key that works it, printed on it: a letter, a digit or `Space` |

A place, Settings and a tab have a `key` too, and `horse_keys` at the end of
the file are the keys of the party's horses, in order (see Keys).

- **A control with no `action` and no `shows` is not built.** It carries `NotBuilt`, which
  draws it as unavailable, and it can still be pressed and reached with Tab.
  A press opens the side panel of the page it is on, with its name, "Not
  built yet.", what it will do and its issue. While that panel is open the
  control carries the selected mark. A footer control opens the panel of
  the open place, and opens the place first if none is showing.
- **A control with an `action` works.** A press calls `Game::act`. It is
  unavailable (`InteractionDisabled`) while `Game::can` refuses the action,
  and `can` returns the reason in plain words ("In town only.", "Nothing to
  equip."), which is shown in place (see below). An element's button is
  marked selected while it is the party's element.
- **A control with a `shows` works too.** A press opens the side panel of
  its page with what it shows, and it carries the selected mark while that
  panel is open. It can always be used, so no reason line names it. Keys,
  in Settings, is the one there is (see Keys).
- **When a system lands**, its control gets an `action` or a `shows` and
  stays where it is.

## Why a control cannot be used

Nothing is silently unavailable (#233). Under each group of controls (a
tab's, a place's, the cargo column's) and beside the footer's, `spawn_reasons`
puts up to two `reason` lines:

- **For the controls that work**, a line that follows the game
  (`Blocked`, filled by `show_reasons`). It names those that cannot be used
  now, gathered by reason, with the words `Game::can` refuses them with:
  "Fire, Ice, Plant: in town only.", "Equip: nothing to equip." `Game::act`
  makes the same check, so the reason shown is why the press would do
  nothing. The line is hidden while every one of them can be used.
- **For the controls that are not built**, a line that never changes
  (`unbuilt_line`). It names them, or the working ones where that is the
  shorter list ("All but Equip: not built yet."), or says "None of these is
  built yet." Outside the footer it adds that a press says more.

A new reason is a new `Err` in `Game::can`; nothing in the shell changes.

`can` and `act` take a `Hand`: the selected horse and, when a worn piece is
open in the side panel, whose it is and which slot (`Shell::hand`). That is
all an action knows of the menus.

**Swap** (W, on Gear) is how a worn piece is equipped on the selected horse
from the overview: the piece that is open and what the selected horse wears
in that slot change places (`loot::swap_worn`), an empty slot like a piece,
so nothing is lost and a second press puts it back. It is refused, with
its reason, on the road, with no slot of another horse open, and when
neither horse wears anything there. The displaced piece goes to the other
horse and not to the cargo because the cargo holds one thing until 0.4.0
(#227).

An empty list says what belongs in it: the cargo, an empty slot's side
panel, and the report before any trail. A tab with no content says "This
page is not built yet."

Tests in `shell.rs` hold the file to this: in town, in town with a drop and
on the road, every control that cannot be used has its line, and a working
one's reason is `can`'s; no control is silent (each has a
`does`, and an action, a `shows` or an issue); a key is a real key, on a
working control, and on one only; the Keys control lists every key in the
file; each thing the slice can do has exactly one
control; no two controls of a page share a name; every page has controls;
every place, tab and horse has a key, and no key means two things at once.

## What a tab shows

`content` in `places.ron` says what a tab holds:

- `Gear`: the overview (#285). The selected horse's stats, then a column
  for each horse with its three slots under its name, so everything worn
  is seen at once beside the cargo and no horse has to be selected to look.
  A slot is a `cell`: the piece and, under it, the slot and what the piece
  adds, or "nothing worn". The selected horse's column is outlined and its
  heading says "selected". Pressing a slot opens it in the side panel and
  does not change the selected horse. Another horse's slot is shown against
  the selected horse, as the cargo's piece is, with what Swap would do.
- `Element`: the party's one element and what beats what, above the three
  element buttons. The slice's stand-in for abilities.
- `Report`: the latest report, in two halves of fixed width. In town, the
  last trail and its drop, and the next trail; on the road, the fight and
  its log. This is the text the old overlay showed.
- none: the tab says what will be decided there and that it is not built.

Under whatever a tab shows come its controls.

A place marked `horse_columns` keeps the horses down its left and the cargo
down its right on every tab. Until 0.4.0 the cargo holds one thing: the
piece the last trail dropped, until it is equipped or the party leaves.
It is compared with the selected horse: what that horse wears in the slot,
and whether the piece is an upgrade for it, no better or worse, in points
(`loot::gain`). Selecting another horse changes the comparison. Equip is
there and puts the piece on the selected horse (#250); before, a piece was
rolled for one horse and went to it whoever was selected.

## A tab that does not fit

A tab's page scrolls when it is taller than its room (#243), which today
happens at the smallest window with the side panel open. It is Bevy's
`ScrollArea` with a `Scrollbar` beside it, both from `bevy::ui_widgets`,
whose plugins come with `DefaultPlugins`:

- The wheel scrolls the page the pointer is over.
- The bar shows only while the page does not fit (`show_scrollbars`). It
  keeps its width, `size.scrollbar_width`, while hidden, so a page does not
  move when it starts to scroll. It can be dragged.
- A control that takes the keyboard focus is brought into view
  (`follow_focus`, which sends Bevy's `ScrollIntoView`), so Tab never lands
  on something out of sight.

The horses and cargo columns and the side panel do not scroll; they fit at
the smallest window today.

## Where the side panel opens

In a place with `horse_columns` the side panel opens **in the cargo
column's room**, at the cargo's width (`size.cargo_width`): the cargo is
put away while a detail shows and comes back when it is closed (#287). So
opening or closing a detail moves nothing on the tab beside it, and the
overview keeps its width. Before, the panel took its 280 pixels from the
tab, and at the smallest window a column of the overview was about 95
pixels wide. In the other places and in Settings the panel opens at the
right at `size.side_panel_width`, as it did.

Two things follow from the cargo being covered:

- A detail left open in such a place is put away when the party arrives in
  town (`follow_the_party`), on every tab: the party has come back with
  something in the cargo, and that must not be hidden behind what was being
  looked at before. This is the one case in which a place is not found
  exactly as it was left.
- A cargo control that is not built opens its "Not built yet." detail over
  the cargo, itself included. Close or Escape brings the cargo back with
  the focus on that control.

Equip (E) still works with the cargo covered, as every key does.

At the smallest window a slot of the overview holds 22 letters on one line,
with one to spare: the longest name the data can make today ("Fae-touched
Saddle Pad"). A test in `sim/loot.rs` holds `gear.ron` to that, so a longer
prefix or noun fails there and not on screen. The room comes from
`size.horses_width` (164, as narrow as a horse's row reads),
`size.cargo_width` (248) and the padding of a `cell`. A piece's stats may
still take two lines. The three horses' slots are in view without
scrolling; the tab's controls are partly a scroll away.

## Keys

Every key is in `places.ron` and is printed on what it works (#229). `keys`
in the shell presses the thing when its key goes down, by the same
`Activate` a click sends.

| Key | What it works | Where it is in the file |
| --- | --- | --- |
| H, T, M, N, J | Horses, Trade, Map, Town, Journal | `key` of the place |
| S | Settings | `key` of `settings` |
| 1 to 4 | The tabs of the place that is showing | `key` of the tab |
| Z, X, C | The first, second and third horse | `horse_keys` |
| F, I, P | Fire, Ice, Plant, in town only | `key` of the control |
| E | Equip the drop, in town only | `key` of the control |
| W | Swap the open piece onto the selected horse, in town only | `key` of the control |
| Space | Set out, in town only | `key` of the control |
| K | Keys: the list of every key, in Settings | `key` of the control |
| Escape | Back one level | not in the file |
| Arrows | Move the keyboard focus to the nearest control that way | not in the file |

The keys of places, horses and controls work wherever the player is, so no
two of them are the same. A tab's key works only while its place is showing,
so every place numbers its tabs from 1, and no tab may take a key of the
other kind. A place's key is a press on the bar: on the road it opens the
place, or closes the one that is showing.

Escape closes the side panel; on the road, with no side panel open, it
closes the place. In town it does nothing else: it never sets out, never
closes the game and never undoes anything.

Space is the one conflict, since a focused button also answers Space: while
a control shows keyboard focus (after Tab or an arrow), Space presses that
control and does not send the party off. Every control, built or not, is
reached with Tab or the arrows and pressed with Enter; that is how a gear
slot is opened by keyboard.

**The arrow keys** move the focus by direction (`arrows`): down a list,
along a row of buttons, across to the next column. The next control is the
nearest one that way among the parts of the shell that are laid out, scored
by Bevy's own rule (`find_best_candidate`, with the default
`AutoNavigationConfig`), so nothing is wired by hand and a control moved in
`places.ron` is still reached. Where nothing lies that way the focus stays.
With no focus in sight, the first press puts it on the first control of the
page that is showing (the first slot, on Gear), or on the open place in the
bar when every place is closed. A held arrow does not repeat.

A control that goes out of sight gives up the focus (`lose_hidden_focus`):
after a tab is changed by its key, Enter and Space cannot press something
on the page that was left.

**The Keys control** in Settings (K) lists every key in the side panel:
places, the tabs of a place, horses and actions, each beside the name of
what it works, then Esc, the arrows, Tab with Shift+Tab, and Enter.
`keys_list` builds it from `places.ron`, so a key changed there is changed
in the list; only the last four lines are written by hand. The lines are kept short enough
that the list fits the side panel at the smallest window, which does not
scroll.

Still to come under #229: a key for each action as its button is built
(List, with #99, is the one a town visit lacks).

## Looking at it

    $env:IDLE_TOUR = "screenshots/tour"; cargo run -- --seed 6 *> run.log

presses through every place and tab and Settings, presses a control that
is not built on two of them, chooses an element, saves a numbered PNG of
each into the folder (which must exist), sends the party off, photographs
the road view
and a place being opened and closed there (two of them part of the way
through the move, and one with the keyboard focus put on Sets to show the
page scrolling to it), waits for the party to come home, and then makes a town visit by keys
alone: H, 1, Down twice onto the second slot, Enter to open it, Escape, Left to the horses, Right four times past the overview to the cargo, E to equip the drop, Z and a press on what the third horse wears on its head, W to swap it over, Escape, C, 2, I and F on the abilities, T, 3, S,
K for the list of keys, Escape twice, Space to set out, and J, 1, Escape on the road. Then it exits. Those
keys are put into Bevy's keyboard state for one frame each (`tour_keys`), so
they go through `keys`, `arrows` and `escape` as a real press does (Enter is the one exception: the tour sends the `Activate` Bevy's button sends for it); they are written
in `plan_tour`, so a key changed in `places.ron` is changed there too. The
party does not leave by itself during the tour.
The tour finds the controls it presses by name (Compare, Shake, Ice, Fire,
Equip), so one of those renamed in `places.ron` is renamed in
`plan_tour` too, or its press is skipped. At its first picture the tour also goes round every stop Tab makes and logs
the count ("tour: Tab makes 35 stops, 0 of them out of sight"); a stop out of
sight is #244 come back. The
presses are the `Activate` event a click sends, so the tour goes through the
same code as the pointer, but it is not the pointer: hover and real clicks
are checked by hand.

## Not here yet

- A fountain to check the road view's room against: #4, #8 (the last part of
  #205).
- What is planned on the road, listed in the footer: #234.
- The rest of #229: keys for actions not built yet.
- The Horses place proper, with cargo: #227.
