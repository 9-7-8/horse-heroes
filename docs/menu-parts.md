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
  (one square of a grid, such as a slot of the Gear tab: as wide as
  its column, at least `size.cell_height` tall, drawn as a card).
- Without states: `panel`, `side_panel` (title and Close), `counter`,
  `reason` (the line that says why a control is unavailable),
  `trail_mark` (one fight in the footer's trail tracker: behind, here or
  ahead, and whether it is the boss),
  `confirmation` on a `scrim` (the one modal).
- `popup`, a lesson's pop-up, with `small_button` for its two controls
  (see "The hint").
- Text: `text`, `dim`, `title`.
- The stand-in picture of a piece of gear is not in `parts.rs` but in
  `picture.rs` (item-card.md, "The picture"). The parts screen shows every
  one: a line per slot, the empty outline and then each rarity.

A screen never sets a colour or a size on a control. It spawns a part and
marks its state with Bevy's own components: `Selected` for chosen,
`InteractionDisabled` for unavailable. Hover, press and keyboard focus are
tracked by Bevy (`Hovered`, `Pressed`, `InputFocus`), and every interactive
part is a headless `bevy::ui_widgets::Button`, so it sends `Activate` on a
click, or on Enter or Space when focused.

A `tab` has one more state, beside the six: `Unseen`, for a place or tab
with something not yet seen (#232, menu-shell.md, Marks). It shows the
attention mark, a round dot in the tab's top corner (`size.attention`,
`colour.attention`), edged in the screen's colour. It is round where the
selected mark is a bar, and out of the layout. A test holds its colour
apart from every fill a tab can have with the colour taken away.

## The hint

A lesson as a pop-up (#353, after the one line of #291;
`docs/philosophy/onboarding.md`, principles 9, 10 and 16):
`popup(tokens, about, key, words)`.

         <>
    +----------------------------------+
    | <> Equip [E]                     |
    | You choose who wears it. Left    |
    | alone it stays carried.          |
    |       [Got it [Esc]] [Skip all]  |
    +----------------------------------+

- **It names its control and prints its key**, as the control does, then
  says its one line in the title's size, wrapping within
  `size.popup_width` (340).
- **It stands out without colour**: a panel with an edge of
  `size.popup_border` (3), thicker than any control's, in
  `colour.attention`, and a shadow under it.
- **Its point** (`PopupArrow`) is a square stood on a corner,
  `size.popup_arrow` (14), half out of the edge nearest its control. The
  shell moves it along that edge and to the other edge.
- **Its controls** are `small_button`s (`size.hint_control`, 20 high, small
  text). They carry `DismissHint` and `SkipHints`.
- **The mark.** Any interactive part that carries `Pointed` shows a square
  stood on a corner in its top left corner (`size.pointer`, 6), and is
  ringed in `colour.attention` where the focus ring goes, unless it has
  focus or is a drag's target (#353). The pop-up's first line begins with
  the same mark. Both are out of the layout, so they move nothing. The
  mark is told from the attention mark and the selected mark by shape: a
  square on a corner, a round dot, a bar. A test holds that colour apart
  from every fill a part can have with the colour taken away.
- Where the pop-up stands, how it comes up, what raises a hint and what
  puts it away are the shell's: [menu-shell.md](menu-shell.md), "The
  hint's pop-up".

Chosen without the owner: the square on a corner, that it shares the
attention mark's blue, the words "Got it" and "Skip all hints"; and for
the pop-up its width, edge, shadow, the title's size for its words, and
the ring.

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

Under the side panel is the hint's pop-up, with its point, about a control
with a key; its controls in each state; and its mark and ring on a button,
a tab that also has the attention mark, a row, a selected tab and a
pressed button.

## Not here yet

- An unavailable control that answers a press with its reason: #233.
- Keys that work the controls they are printed on: #229.
- The shell is built: [menu-shell.md](menu-shell.md). The skeleton of every
  button: #238.
