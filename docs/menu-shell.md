# The menu shell

The frame every menu screen hangs on (#226): the bar of places (the
owner's five, and the Inventory she gave a place of its own), the
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
| `src/game/menu/card.rs` | What an item card says ([item-card.md](item-card.md)) |
| `src/game/menu/shell.rs` | The shell: its state (`Shell`), its layout, and the systems that keep it showing the game |
| `src/game/mod.rs` | `SceneView`, which layout the window is in and how far between them; `Action`, `Game::can` and `Game::act`, what a working control does; the report text shown in the Journal |
| `src/game/scene.rs` | `frame`, which sizes the camera to the layout every frame |

## The layout

    +--------------------------------------------------+
    |        3D scene (top third, SCENE_SHARE)         |
    +--------------------------------------------------+
    | Horses Inventory Trade Map Town Journal Settings |  bar
    | coins  bags  hold    A  B  C                 day |  status strip
    +--------------------------------------------------+
    | tabs of the open place                           |
    | the open tab            | [cargo or side panel]  |
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
  that is open, a press on the scene, or Escape with no side panel open,
  closes it.
- **A press on the scene** (#307) closes the open place in one press, side
  panel and all; the place is found as it was left. In town, and with every
  place closed, it does nothing. It lands on `SceneCatcher`, a node the size
  of the window that lies behind every menu (`GlobalZIndex(-1)`), so a press
  a menu takes never reaches it; `Shell::press_scene` is what it does. The
  fight log and the pointer's card let a press through, and neither is ever
  under the pointer with a place open: the log is away, and the card is up
  only while the pointer is on its piece.
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
  control: pressing it selects that horse and opens its view (see "A
  horse's view"). This is the one list of horses (#311).
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

## The fight log

The log of the fight lies over the road scene, at its top left (#306; the
owner, testing 0.2.1: "overlayed on the horses walking screen, probably on
the left", and "I see we already have a combat log under journal. Move
that."). It is not in the Journal's report any more.

    FIGHT LOG  [L] hides it
    Encounter 2/4: Mire Toad (Fire)          faintest
    Juniper Focused Shot on Mire Toad for 16
    ...
    Sable hits Mire Toad for 5               newest, whole

- **When.** On the road with every place closed (`Shell::log_in_view`):
  the road layout, where the fight is watched. Not in town, and not under
  an open place, where the scene has a third of the window.
- **Where.** `space.l` from the top and the left of the window,
  `size.fight_log_width` (440) wide. The party stands below it and an
  enemy falls to the right of it, so at the smallest window it covers
  neither, nor the fountain of the final kill (pictures: `--seed 6
  --window 1120x640` at 30 and 65.9).
- **What.** The latest `size.fight_log_lines` (6) lines of
  `Expedition::log` (`fight_log` in `src/game/mod.rs`), the newest lowest.
  Each older line is fainter, down to `size.fight_log_fade` (0.3) for the
  oldest of a full log (`log_alpha`). Elements and stats are in their
  colours (#251), by the same `ink::runs` every live text uses.
- **The pointer passes through it** (`Pickable::IGNORE`); it has no
  backing, so it hides nothing but its own letters.
- **L hides and shows its lines** (`log_key` in `places.ron`). The heading
  stays, since the key is printed on it: "FIGHT LOG hidden  [L] shows it".
  The key does nothing while the log is not in view. It is in the Keys
  list. Hidden or shown is kept for the run; saving it is #44.
- `show_log` in the shell keeps it; a test holds when it is in view, the
  key, the lines and that the report no longer carries them.

Chosen without the owner: the key L, six lines, no backing behind the
letters, and that it is away while a place is open on the road.

## How a place keeps its state

All the places, and every tab of each, are spawned once at startup and only
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
words. Two more, `see` and `mark_unseen`, keep the marks (see Marks).

One observer, `on_activate`, handles every control of the shell: what a
press does depends on which marker the control carries (`PlaceButton`,
`TabButton`, `HorseButton`, `SlotRow`, `ClosePanel`, `ControlButton`).

## Every planned control

The owner, 2026-10-10: "All buttons should be there, even if they're not
functional, as early as possible." So `places.ron` lists every control
planned for every tab, for the cargo column (`cargo`), for a place without
tabs and for Settings (`controls`), for the footer (`footer`), and for the
item card (`card`, in every place's side panel: [item-card.md](item-card.md)). The shell
makes a button for each, in the order of the file, in a row that wraps.
Renaming, reordering or moving a control is a change to that file only.

A control in the file is:

| Field | What it is |
| --- | --- |
| `name` | What the button says |
| `does` | What it does, or will do, in one sentence |
| `issue` | The issue that will make it work |
| `proposed` | That issue is a proposal the owner has not decided |
| `action` | What it does today: `Element(Fire)`, `Equip`, `SetOut`, `Undo`, `Earlier`, `Later` |
| `shows` | What a press opens in the side panel of its page: `Keys` |
| `key` | The key that works it, printed on it: a letter, a digit, `[`, `]` or `Space` |

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

`can` and `act` take a `Hand`: the selected horse and, when a piece's card
is open in the side panel, where that piece is (`Piece`, from
`Shell::hand`). That is all an action knows of the menus.

**Equip** (E, on the item card) is the one way a piece is put on the
selected horse, from the cargo or from another horse: see
[item-card.md](item-card.md). From another horse the two change places
(`loot::swap_worn`), an empty slot like a piece, so nothing is lost. The
displaced piece goes to the other horse and not to the cargo because the
cargo holds one thing until 0.4.0 (#227).

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
  A slot is a `cell`: the piece's name and, under it, its picture (#286,
  item-card.md) beside what the piece adds; or "nothing worn" over the
  slot's outline and its name. The selected horse's column is outlined and its
  heading says "selected". Pressing a slot opens its card in the side panel
  and does not change the selected horse. Another horse's piece is compared
  on the selected horse, as the cargo's piece is, with what Equip would do.
- `Element`: the party's one element and what beats what, above the three
  element buttons. The slice's stand-in for abilities.
- `Report`: the report the Journal is open at, in two halves of fixed
  width. The latest: in town, the last trail and the next trail; on the
  road, the fight, and beside it a line saying where its log is (see
  "The fight log"). Under the last trail is what becomes of its
  drop at setting out (item-card.md, "The appraisal in camp"), then the
  drop as the cargo shows it, and a press opens its card in the Journal's
  side panel, where Equip works without going to the Horses place. An
  earlier one: see "The Journal's reports".
- `Inventory`: every piece the party has, worn or carried, in a grid by
  type (#308): see "The Inventory" below. It is the `content` of a place
  without tabs, which a place may have as a tab does.
- none: the tab says what will be decided there and that it is not built.

Under whatever a tab shows come its controls.

A place marked `horse_columns` keeps the cargo down its right on every
tab. It had the horses down its left too until #311: they were listed
twice, and the strip is the list that stayed. Until 0.4.0 the cargo holds one thing: the
piece the last trail dropped, until it is equipped or the party leaves.
It is shown as the summary of its card: its name, what it is, and whether
it is an upgrade for the selected horse, no better or worse, in points
(`loot::gain`). Selecting another horse changes that line. A press opens
its card, with Equip, which puts the piece on the selected horse (#250).
The cargo's own controls are those of a list: Filter, Sort, Search.

## The Inventory

A place of its own in the bar, after Horses (key B): all the items, not
divided by horse (the owner, 2026-10-11; `docs/philosophy/inventory.md`).
It was first built as the fifth tab of Horses, with the comparison on a
press; she asked the same day for "its own tab with a grid view with
comparisons when you hover an item".

    | Head                                    | compare            |
    | [tile] [tile] [tile] [tile]             | Point at a piece   |
    | Body                                    | to compare it ...  |
    | No body piece is worn or carried yet.   |                    |
    | Saddle                                  |                    |
    | [tile]                                  |                    |

- **A headed group for each type**, in the order of `Slot::ALL`, so a new
  slot gets its group with no menu code. In a group: what each horse wears
  in that slot, in the party's order, then the piece in the cargo when it
  is of that type (`held` in the shell; a test holds that every piece is
  there once, under its type). An empty type says so in a line
  (`card::none_held`) and keeps its heading.
- **A tile** is the piece's picture, large (`size.inventory_picture`, 56),
  and under it whose it is: the horse's name, or "Carried". Its name and
  what it adds are under the pointer. It is the same control as a slot of
  the overview (`SlotRow`) or the cargo's piece (`CarriedCell`), so the
  arrow keys and the selected mark come with it.
- **The pointer on a tile compares it** (`card::pointed_comparison`, in
  the hover card of [item-card.md](item-card.md), at `size.compare_width`):
  for the selected horse, the piece equipped and the piece under the
  pointer, what changes, and the table. A press pins the same comparison
  at the right, with Equip and the card's other actions; Enter does that
  for the keyboard.
- **Every cell a group can hold is spawned once** and shows while its piece
  is held (`When::Held`, `When::NoneHeld`, in `show_carried`), as the rest
  of the shell is shown and hidden and never rebuilt. When the cargo holds
  more than one piece (#227) this is the part that changes.
- **The grid** is a CSS grid of as many columns as fit, each
  `size.inventory_cell` wide (104): six at the smallest window.
- **It has the place's width.** The place has no horses column and no
  cargo column. The horse a piece is compared on, and equipped on, is the
  one selected in the status strip, which is always in view.
- **The pinned comparison keeps its room** at the right,
  `size.compare_width` (420), whether or not a piece is pinned:
  `Page::Compare` says how to use it while none is, and the detail opens at
  that same width, so the grid never moves. What it says is in [item-card.md](item-card.md), "The
  comparison".
- A carried piece in view here has been seen, as in the cargo column
  (Marks).
- The place has no controls of its own: its tiles are its controls. Filter,
  Sort and Search are #235.

## A horse's view

A press on a horse in the strip, or its key (Z, X, C), selects it and opens
everything about it in the side panel of the place the player is in (#311;
the owner: "Clicking a horse should open up an inspection screen that lets
you view all the information about that horse"). `Shell::inspect`,
`Item::Horse`, `spawn_horse_parts`, `horse_part` and `Shell::go_to` in the
shell, `card::horse_sheet` and `card::horse_wears`.

    Sable
    Health 32/35

              horse gear  now
    health       35  +15   50
    attack        3   +2    5
    ...

    [ Gear [H, 1] ]
    Head: Sunforged Halter
      +15 health  +2 attack ...
    Body: nothing worn
    ...
    [ Abilities [H, 2] ]
    [ Pet [H, 3] ]
    [ Look [H, 4] ]
    LAST TRAIL

- **Every total is accounted for:** what the horse has by itself, what its
  gear adds, and the two together, a stat a line; under Gear each piece is
  named with what it adds. The table is 25 letters wide, which the
  narrowest side panel (the cargo's room, 248) holds.
- **A part for each tab of the horses' place**, from `places.ron`, headed
  by a button with the tab's name and the keys that lead to it, since that
  is where the part is changed. A tab that is not built says so with its
  job.
- **A press on a part goes to its tab**, from any place and on the road:
  the Horses place opens on that tab, with the horse still selected. The
  view is put away there, as it stands over the cargo the tab works with,
  and it stays open in the place the player came from.
- **On the road** its health is the fight's, and with every place closed
  the press opens the place that was last open, with the view in it.
- **A pinned piece stays.** While a piece's card is pinned in that side
  panel, a press on a horse leaves it there and compares it on that horse:
  the piece is the handle, and that is how a piece is tried on each horse.
- Close or Escape puts it away. In the Horses place it stands over the
  cargo, as every detail there does.
- At the smallest window it is longer than the panel; PageDown reads on.

Not here yet: what the horse did on the last trail (nothing is kept for
each horse).

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

The horses and cargo columns do not scroll; they fit at the smallest window
today. What a side panel says scrolls in the same way where it does not
fit, and the item card's actions under it stay in view.

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
the horses column that is gone (#311),
`size.cargo_width` (248) and the padding of a `cell`. That is why a piece's
picture is under its name and not beside it: beside it, 18 letters fit. A
piece's stats may still take two lines, which makes its slot a few pixels
taller than an empty one. The three horses' slots are in view without
scrolling; the tab's controls are a scroll away.

## Keys

Every key is in `places.ron` and is printed on what it works (#229). `keys`
in the shell presses the thing when its key goes down, by the same
`Activate` a click sends.

| Key | What it works | Where it is in the file |
| --- | --- | --- |
| H, B, T, M, N, J | Horses, Inventory, Trade, Map, Town, Journal | `key` of the place |
| S | Settings | `key` of `settings` |
| 1 to 4 | The tabs of the place that is showing | `key` of the tab |
| Z, X, C | The first, second and third horse | `horse_keys` |
| F, I, P | Fire, Ice, Plant, in town only | `key` of the control |
| E | Equip the piece whose card is open, or the drop when none is, in town only | `key` of the control |
| U | Undo the last change of this town visit, in town only | `key` of the control |
| [, ] | Earlier and Later: the report before, and after, the one the Journal shows | `key` of the control |
| Space | Set out, in town only | `key` of the control |
| K | Keys: the list of every key, in Settings | `key` of the control |
| Escape | Back one level | not in the file |
| Arrows | Move the keyboard focus to the nearest control that way | not in the file |
| PageUp, PageDown | Scroll what the side panel says, or the tab's page when no panel is open | `scroll_keys` |
| L | Hide or show the fight log, on the road with every place closed | `log_key` |

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
what it works, then Esc, the arrows, the scroll keys, Tab with Shift+Tab,
and Enter. `keys_list` builds it from `places.ron`, so a key changed there
is changed in the list; only the words of the last five lines are written
by hand. With the scroll keys' line the list is two lines longer than the
side panel at the smallest window, and is read to its end by those keys.

**The scroll keys** (#304, `scroll`) move what the side panel says while
one is open, and otherwise the page of the tab that is showing, by
`size.scroll_page` of what is in view. They do nothing with every place
closed. A side panel names them under its words while the words do not
fit (`ScrollHint`, shown by `show_scrollbars`); a tab's page does not, as
the focus already brings its controls into view. `docs/item-card.md`,
Fitting.

Still to come under #229: a key for each action as its button is built
(List, with #99, is the one a town visit lacks).

## Undo

Undo (U; on Horses, Gear, and its key works in every place) takes back the
last change made during this town visit, one step a press (#230,
`docs/philosophy/menus.md`, Undo, not "are you sure"). Nothing reversible
asks for confirmation.

- **What it takes back today:** an equip of the drop, a swap between two
  horses, and a change of element. Those are all the changes the slice
  has. Listing, prices, ability points, looks and pets join the list as
  each is built: an action is undoable when `Game::act` pushes a `Change`
  before doing it.
- **It restores the real state.** A `Change` holds the whole party, the
  cargo and the element as they were before it (`Game::before`), and
  `Game::undo` puts them back, so a piece that was equipped over is on its
  horse again and the drop is in the cargo again.
- **What changes nothing is not a step:** choosing the element the party
  already has, and an Equip that is refused.
- **It never reaches back past a departure.** `Game::depart` empties the
  list, whether the player set out or the countdown did. With nothing to
  take back, Undo is unavailable and says "Undo: nothing changed this
  visit."; on the road, "in town only".
- **The card goes with the piece.** A change that moved a piece remembers
  from where to where; `on_activate` moves any open card back the other
  way (`Game::undone_move`), so a card opened on the drop is on the drop
  again.
- **It is acknowledged** as every choice in camp is today, by a line in
  the report: "Taken back: Sunforged Halter on Sable." More than that is
  #40.

Chosen without the owner: the key U, and that Undo stays on the Gear tab
where the map had it. In the footer it would be in sight from every place,
but at the smallest window it made the footer's sentence wrap to a second
line.

## The Journal's reports

No report holds the party up, and none is lost by not being read (#231,
`docs/philosophy/menus.md`, No menu ever holds the game up). The countdown
sends the party off whether the report was looked at or not, and
`Game::return_to_camp` keeps the report of every trail (`Kept`: the trail's
number, its level, and how it ended, with its drop as it was). The newest
`reports_kept` of them are kept (`rules.ron`, 20).

- **Earlier ([) and Later (])**, on Journal, Report, step through them, in
  town and on the road. The latest is the last trail in town and the fight
  on the road; `Game::earlier` is what lies behind it. Each control says
  why it cannot be used: "No earlier report.", "This is the earliest
  report kept.", "This is the latest report."
- **The key opens what it turned.** `[` or `]` from another place, or from
  the road with every place closed, opens the Journal on Report.
- **The report being read stays put.** It is held by its trail's number
  (`Game::reading`), not by how far back it is, so it does not change when
  the party leaves or arrives. One that is no longer kept gives way to the
  latest.
- **An earlier report** says which it is of how many, the trail and its
  level, how it ended and what it dropped. Beside it is the list of the
  reports kept, newest first, with `>` on the one that is open.
- **Its piece is found where it is now.** While the party has the piece a
  report's trail dropped, the report shows it as a cell (`ReportedCell`):
  what it is and whether it is in the cargo or which horse wears it. A
  press opens its card there, so Equip works on it as from anywhere
  (`Game::reported_piece`). When the party no longer has it the report
  says so. The latest report does the same, so its piece stays in the
  report once it is worn.
- A card open on a report's piece is put away when the page is turned.
- Turning the pages is not a change, and Undo does not count it.

A piece has no identity of its own yet: one exactly like the piece a
report dropped (same name, slot, rarity and stats) counts as it.

Still to come: the loot, trail and stall reports as separate reports (#32,
#51, #170), the Journal tab (#107) and the Codex (#30).

## Marks

A place says when it has something the player has not seen, and stops when
it has been seen (#232, `docs/philosophy/menus.md`, A place says when it
wants attention). The mark is a round dot in the corner of the place in the
bar, and of the tab the thing is on. It is a state of the `tab` part
(`Unseen`, [menu-parts.md](menu-parts.md)).

What raises one is `marks` in `places.ron`: a `raise`, the `place` it
marks, and the `tab` when the thing is on one.

| `raise` | There is a mark while | It is seen when |
| --- | --- | --- |
| `Upgrade` | the piece in the cargo betters what some horse wears | the cargo column shows, or the Inventory, or the piece's card, or the report that shows the piece |
| `Report(lost, rarity)` | the newest report kept is of a trail that was lost (`lost`), or that dropped a piece of at least `rarity` | that report shows, on its tab |

Today: an upgrade marks Horses, and a lost trail or a Legendary piece or
better marks Journal, Report. A trail simply won, a common drop and a
piece that is no upgrade raise nothing.

- **Seen is in view.** `in_view` in the shell says which of the two is
  showing, `see` tells the game each frame, and `Game::wants` is what
  `mark_unseen` asks for every place and tab. A slot's card standing over
  the cargo hides the cargo, so the upgrade is not seen through it.
- **It does not come back for the same thing.** The game keeps the trail
  whose drop was seen and the newest report read. A piece equipped and
  taken back with Undo is the same piece.
- **Only the newest report raises one.** So a long absence leaves at most
  one report marked, not twenty, and a lost trail that another has
  followed raises nothing.
- **Two actions at most.** In town the Journal opens on the report, or it
  is one tab away. On the road the Journal opens on the fight and the
  mark stays: the report is behind it, and Earlier (`[`) is on it in one
  press from anywhere.
- A new kind of thing is a new `Raise` in `src/game/mod.rs`, with what
  seeing it is, and a line in `places.ron`.

Tests: the rules of the file point at a place that shows their thing and
name a rarity `gear.ron` has; a mark leads to its thing in two actions,
goes when it is seen and stays gone; three hundred trails unwatched raise
a report's mark only for the newest.

Chosen without the owner: Legendary as the rarity, the dot and its blue,
and that a report showing the piece counts as seeing the upgrade.

Still to come, each with its system: a mark for an unspent ability point
(#220), a new look (#92), a recruit to meet (#116) and a project that can
take a delivery (#172).

## Looking at it

    $env:IDLE_TOUR = "screenshots/tour"; cargo run -- --seed 6 *> run.log

presses through every place and tab and Settings, presses a control that
is not built on two of them, chooses an element, saves a numbered PNG of
each into the folder (which must exist), sends the party off, photographs
the road view (with the fight log over it, and again with the log hidden by L)
and a place being opened and closed there (two of them part of the way
through the move, and one with the keyboard focus put on Sets to show the
page scrolling to it), presses the scene (in town, on the road with every
place closed, and with a place and its side panel open, which that press
closes; the press is Bevy's own pointer, at `ON_THE_SCENE`), waits for the party to come home, opens the report, and then makes a town visit by keys
alone (after opening the drop's card from the report, and again from the cargo, there against another horse): H, 1, Down twice onto the second slot, Enter to open it, Escape, Left (which now finds nothing to go to), Right four times past the overview to the cargo, E to equip the drop, Z and a press on what the third horse wears on its head, E to equip it, so the two change places, U three times (the swap taken back, the equip taken back, and a press too many), Escape, C, 2, I and F on the abilities, T, 3, S,
K for the list of keys, Escape twice, Space to set out, and on the road J, 1, [ for the report of the trail before, a press on its piece, Escape, ] back to the latest, and Escape. Both times the party comes home, Trade is open, so that the marks are in the picture before a press clears them (the second time: T on the road, then H in town). Then it exits, in town, after about two and a half minutes. Those
keys are put into Bevy's keyboard state for one frame each (`tour_keys`), so
they go through `keys`, `arrows` and `escape` as a real press does (Enter is the one exception: the tour sends the `Activate` Bevy's button sends for it); they are written
in `plan_tour`, so a key changed in `places.ron` is changed there too. The
party does not leave by itself during the tour.
The tour finds the controls it presses by name (Compare, Shake, Ice, Fire,
Equip), so one of those renamed in `places.ron` is renamed in
`plan_tour` too, or its press is skipped. At its first picture the tour also goes round every stop Tab makes and logs
the count ("tour: Tab makes 36 stops, 0 of them out of sight"); a stop out of
sight is #244 come back. The
presses are the `Activate` event a click sends, so the tour goes through the
same code as the pointer, but it is not the pointer: real clicks
are checked by hand. For the card under the pointer (item-card.md) the tour
does move Bevy's own pointer onto a piece (`Step::Point`, then
`Step::PointAway`), as a moved mouse would.

`IDLE_WALK` is the same machinery counting the presses of the common jobs:
[click-budget.md](click-budget.md).

## Not here yet

- A fountain to check the road view's room against: #4, #8 (the last part of
  #205).
- What is planned on the road, listed in the footer: #234.
- The rest of #229: keys for actions not built yet.
- The rest of #230: undo for listing, prices, points, looks and pets, and
  the stop at a completed sale, each with its system.
- The Horses place proper, with cargo: #227.
- The rest of the item card: [item-card.md](item-card.md), "Not here yet".
