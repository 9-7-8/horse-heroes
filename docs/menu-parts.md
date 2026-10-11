# Menu parts

The one set of parts every menu screen is made from (#237). The guidance
behind it is [philosophy/menus.md](philosophy/menus.md), "Every control has
all its states" and "Things stay where they are".

## Where things are

| File | What it holds |
| --- | --- |
| `assets/data/menu.ron` | The tokens: spacing, text sizes, control sizes, the hover time, and colours by role |
| `src/game/menu/tokens.rs` | Loads the tokens into the `Tokens` resource |
| `src/game/menu/parts.rs` | The parts, the rule for how each state looks (`paint`), and the system that applies it (`style_parts`) |
| `src/game/menu/gallery.rs` | The dev screen |

## The parts

Each is a function that takes the tokens and returns a bundle to spawn.

- Interactive, with all six states: `button`, `tab`, `row` (a list row),
  `card` (the item card frame; its lines are added as children), `cell`
  (one square of a grid, such as a slot of the gear overview: as wide as
  its column, at least `size.cell_height` tall, drawn as a card).
- Without states: `panel`, `side_panel` (title and Close), `counter`,
  `reason` (the line that says why a control is unavailable),
  `trail_mark` (one fight in the footer's trail tracker: behind, here or
  ahead, and whether it is the boss),
  `confirmation` on a `scrim` (the one modal).
- Text: `text`, `dim`, `title`.

A screen never sets a colour or a size on a control. It spawns a part and
marks its state with Bevy's own components: `Selected` for chosen,
`InteractionDisabled` for unavailable. Hover, press and keyboard focus are
tracked by Bevy (`Hovered`, `Pressed`, `InputFocus`), and every interactive
part is a headless `bevy::ui_widgets::Button`, so it sends `Activate` on a
click, or on Enter or Space when focused.

## How a state looks

`paint` is the only place that decides, from the colour tokens:

| State | Drawn as |
| --- | --- |
| default | control fill, thin border |
| hover | lighter fill and border |
| focus | a ring outside the control, on top of any other state |
| pressed | inverted: light fill, dark text |
| selected | sunk fill, accent border, and the mark bar (under a button or tab, down the left of a row or card) |
| unavailable | no fill, faint border, faint text |

Selected wins over hover, so the two never look alike. The mark and the ring
sit outside the layout, and the border keeps its width, so no state change
moves anything. Only the hover fill eases (`time.hover`); press, selection
and focus show on the frame they happen.

Three tests in `parts.rs` hold the tokens to this: every pair of states
differs in lightness and not only hue, hover and selected differ in both fill
and mark, and text stands out from its fill.

## The dev screen

    cargo run -- --parts

or F2 in the running game. The grid shows each interactive part in each
state, forced with `ShowLook`. Below it are the parts without states and a
live set to hover, press and tab through, including a real confirmation.

To tune: change a value in `menu.ron`, run again, look at this screen. The
tokens are compiled in, so a change needs a rebuild but no code change.

## Not here yet

- An unavailable control that answers a press with its reason: #233.
- Keys that work the controls they are printed on: #229.
- The shell is built: [menu-shell.md](menu-shell.md). The skeleton of every
  button: #238.
