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
| `assets/data/places.ron` | The map of the menus: the places in order, the tabs of each, and the line saying what is decided there |
| `assets/data/menu.ron` | Sizes of the shell's columns, and `layout`: the scene's share of the window in each layout and the minimum window size; `time.layout`: how long the move takes |
| `src/game/menu/shell.rs` | The shell: its state (`Shell`), its layout, and the systems that keep it showing the game |
| `src/game/mod.rs` | `SceneView`, which layout the window is in and how far between them; the report text shown in the Journal |
| `src/game/scene.rs` | `frame`, which sizes the camera to the layout every frame |

## The layout

    +--------------------------------------------------+
    |        3D scene (top third, SCENE_SHARE)         |
    +--------------------------------------------------+
    | Horses  Trade  Map  Town  Journal       Settings |  bar
    | coins  bags  hold    A  B  C                 day |  status strip
    +--------------------------------------------------+
    | tabs of the open place                           |
    | [horses] | the open tab | [cargo] | [side panel] |
    +--------------------------------------------------+
    | what the party is doing              [ Set out ] |  footer
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
  left.
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
  Settings at the far end (unavailable until there are settings).
- **The status strip.** Coins, bags, hold, the three horses with their
  health, and the day. Bags, hold and the day show "-" and keep their
  positions until the systems behind them exist. A horse in the strip is a
  control: pressing it selects that horse.
- **The footer.** In town: what the party is ready for, the countdown, and
  Set out. On the road: where the party is, and Set out unavailable.

## How a place keeps its state

All five places, and every tab of each, are spawned once at startup and only
shown or hidden (`Display::None`). Nothing is rebuilt on a visit, so a place
is found as it was left, whatever it comes to hold.

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
`TabButton`, `HorseButton`, `SlotRow`, `ClosePanel`, `SetOut`).

## What a tab shows

`content` in `places.ron` says what a tab holds:

- `Gear`: the selected horse's stats and its three slots. Pressing a slot
  opens the piece in the side panel.
- `Report`: the latest report, in two halves of fixed width. In town, the
  last trail and its drop, and the next trail; on the road, the fight and
  its log. This is the text the old overlay showed.
- none: the tab says what will be decided there and that it is not built.

A place marked `horse_columns` keeps the horses down its left and the cargo
down its right on every tab. Cargo is an empty panel until 0.4.0.

## Keys

The slice's keys still work in town: 1, 2, 3 choose the element, E equips
the drop, Space sets out. Space is the one conflict with the controls, since
a focused button also answers Space: while a control shows keyboard focus
(after Tab), Space presses that control and does not send the party off.
Keys for places and tabs are #229.

## Looking at it

    $env:IDLE_TOUR = "screenshots/tour"; cargo run -- --seed 6 *> run.log

presses through every place and tab, saves a numbered PNG of each into the
folder (which must exist), sends the party off, photographs the road view
and a place being opened and closed there (two of them part of the way
through the move), waits for the party to come home, and exits. The party
does not leave by itself during the tour. The
presses are the `Activate` event a click sends, so the tour goes through the
same code as the pointer, but it is not the pointer: hover and real clicks
are checked by hand.

## Not here yet

- A fountain to check the road view's room against: #4, #8 (the last part of
  #205).
- What is planned on the road, listed in the footer: #234.
- Every planned button in its place: #238.
- Controls that say why they are unavailable: #233.
- Keys printed on the places and tabs: #229.
- The Horses place proper, with cargo: #227.
