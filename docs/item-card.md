# The item card

One way of showing a piece of gear, wherever it is (#228, part built). The
guidance is [philosophy/menus.md](philosophy/menus.md), "The item is the
handle": the player never carries a piece to the screen that owns a verb.
It hangs on the shell's side panel: [menu-shell.md](menu-shell.md).

## Where things are

| File | What it holds |
| --- | --- |
| `src/game/menu/card.rs` | What the card says: `kind`, `consequence`, `comparison`, `pointed_comparison`. Plain text from the game, tested without Bevy |
| `assets/data/places.ron`, `card` | The card's actions, in order: Equip, List, Store, Lock, Favourite, Look |
| `src/game/mod.rs` | `Piece`, where a piece is; `Action::Equip`; `Game::piece` and `Game::equipped_at` |
| `src/game/menu/picture.rs` | The stand-in picture: `stamp`, `picture`, `draw`, `show_pictures` |
| `assets/data/gear.ron` | A slot's flat shape (`bases`, `picture`) and a rarity's colour and `shape` |
| `src/game/menu/shell.rs` | `Item::Carried` and `Item::Worn`, the details that are pieces; `CarriedCell`; `Page::Card`, the actions under the card |

## The three layers

From the owner's guide on menu design (#228): a summary, the detail on
selection, and formulas behind a Details control. All three are built.

**The summary** is what a list shows of a piece: its name, then its rarity,
slot and what it adds (`kind`). It says nothing of whether the piece is
better for a horse: the game gives the numbers and never a verdict (owner,
2026-10-12, #351). The carried piece is shown this way in the cargo column
and under the report, where a third line says where it is ("Carried in
the cargo.", or the horse that wears it). A slot of
the Gear tab shows the shorter form: the name, and the slot with what the
piece adds. Its picture says which slot and which rarity.

**The pinned card** is what a press opens in the side panel of the place the
piece was pressed in: the comparison below, the same on the Gear tab,
the cargo, a report and the Inventory (#312). Until 2026-10-12 every place
but the Inventory pinned an older card of lines ("health 35 to 50 (+15)"
under "On Sable, who wears nothing there:"); those lines are now only in
the appraisal in camp (`compare`).

- An empty slot has a card too: whose slot, "Nothing worn.", where gear
  comes from, and what Equip would put there. An empty cargo says what
  belongs in it.

**The Details layer** is the formulas behind the Details switch (see
below).

## The comparison

What the pointer shows on a piece, and what a press pins, in every place a
piece is shown (#308, #312; `docs/philosophy/inventory.md`, "The comparison
answers three questions at once"). Under the pointer
(`card::pointed_comparison`) it ends with "A press pins it, with its
actions."; pinned (`card::comparison`) it ends with what Equip would do,
beside Equip.

    For Sable, Head
    Equipped: Plain Halter (Normal)
    Selected: Sunforged Halter (Legendary)
    Carried in the cargo.

    AFTER EQUIPPING, Sable:
    health 40 to 50 (+10)
    attack 3 to 5 (+2)
    defense 5 to 1 (-4)

    Piece      Equipped  Selected  Change
    health           +5       +15  +10 better
    attack            -        +2  +2 new
    defense          +4         -  -4 lost
    elemental        +2        +2  same

    Equip puts it on Sable. Plain Halter goes to the cargo.

- **Context first:** who would receive it and in which slot, then the
  piece equipped there ("nothing" for an empty slot) and the piece
  selected, in that order and labelled in words, and where the selected
  piece is.
- **The horse apart from the pieces:** "after equipping" is the horse's
  own stats as they are and would be (the card's `changes`); the table is
  the two pieces, a stat on one line for both, amounts right-aligned (the
  menus' font is monospaced), and the change. A stat only the selected
  piece has is "new", one only the equipped piece has is "lost", a higher
  one is "better", a lower one "worse", and an equal one is "same"; a stat
  neither has gets no row.
- **The recipient is the selected horse.** Another horse's key compares
  the same piece there, and nothing else on the panel moves.
- **A horse's own piece** is not compared with itself: it says who wears
  it, and what it adds.
- The last line (`consequence`) is what Equip would do, and it is there
  exactly when Equip can be done. It names the displaced piece: "Old Halter
  goes to the cargo." (a carried piece put over a worn one: the worn one
  takes its place in the cargo, #352) or "Equip puts A on
  Juniper, and B on Sable." (two horses changing places). The card's
  actions are under it.
- **Its width is its own** (`size.compare_width`, 420), wherever it is
  pinned and under the pointer, so the table's four columns fit. In the
  Horses place it opens in the cargo's room and takes the rest from the
  Gear tab, whose rows are narrower while a piece is pinned; a
  horse's view there keeps the cargo's width and moves nothing. That is a
  call made for the owner: the other way was a second, narrower table of
  25 letters with shortened headings.
- **Changed only (O)** narrows the table to the stats that differ (#349):
  a "same" row has none, the others keep their order, and a line under
  the table says how many are left out ("Changed only: 1 same row
  hidden.", or "no row is the same."), so the table itself says it is
  narrowed. The lines above the table do not change. It is a control of
  the card, beside Equip (`switch: ChangedOnly` in `places.ron`,
  `card::Rows`, `Shell::press_switch`), marked as chosen while it is on.
  It is one setting for every piece and place, the card under the pointer
  follows it, and it changes nothing in the game, so Undo has nothing of
  it. Its key does nothing while no piece's card is pinned in view. A
  horse's own piece has no change column, so nothing of it is left out.
- The stat's word takes its colour (#251) and the amounts are plain. The
  change is coloured by whether it is good (#313): see Colour.
- At the smallest window the table is a press of PageDown, or a scroll of
  the wheel, away (see Fitting).

Tests in `card.rs`: the table's columns line up and say new, lost and
same; narrowed, it leaves out only the rows that are the same and the lines
above it are the same (`shell.rs` holds the switch from place to place and
its key in the Keys list); the comparison names the horse and both pieces, ends with what Equip
would do, follows the selected horse, and does not compare a horse's piece
with itself; a piece has the same comparison carried and worn; under the
pointer it is the pinned one as far as it goes.

Pictures: `docs/gallery/2026-10-12-one-comparison`, at the smallest window;
`docs/gallery/2026-10-12-changed-only` for the switch.

## The Details layer

The card's third layer (#228; menus.md, "The item is the handle"): the
formulas and assumptions behind a piece, behind **Details** (V), a switch
of the card beside Changed only (`switch: Details` in `places.ron`,
`card::details`, `card::comparison_with`).

    DETAILS: Sunforged Halter
    HOW IT WAS ROLLED
    A Legendary piece has 10 to 11 points, and 1 more for each level of
    its trail. This one has 11.
    Each point went to one stat at random: +1 attack, +1 defense,
    +1 elemental or +5 health.
    Odds of Legendary: 3% of a lost trail's own piece, 8% of a won
    trail's, none of a kill's pieces.

    IN A FIGHT
    Each horse hits at its own pace: every 1.4 to 1.6 s, a little sooner or later each time.
    attack: a hit's physical part, less the enemy's defense, never below 0.
    elemental: a hit's elemental part, x1.5 where the party's element is
    strong, x0.75 where it is resisted.
    defense: taken off each enemy hit, which still does at least 1.
    health: what a horse can lose before it falls.
    Focused Shot hits with attack x2.5; Elemental Burst hits every enemy
    with elemental x2. A Ward takes 50% off enemy hits.

- **Every number is the data's**: the points, the health a point is worth
  and the weights from `gear.ron`, the rest from `rules.ron`. A change
  there is said here with no code. A percentage says what it is a
  percentage of (menus.md), and a share that rounds to 0 is "under 1%".
- **Where:** under the comparison's table and before what Equip would do,
  so that line stays beside Equip. A horse's own piece has it under its
  list of what it adds; an empty slot or cargo has none.
- **One setting**, as Changed only is: it stays from piece to piece and
  place to place, and does nothing while no card is pinned in view. It
  changes nothing in the game, so Undo has nothing of it.
- **Not under the pointer.** The card the pointer shows keeps to the
  comparison: the advanced layer is asked for.
- A piece does not keep the level of its trail, so the card says the
  rule and the piece's total, not the level.
- Its tutorial is an entry on the list of things to try in town, "Turn on
  a card's Details." (`Detailed`, docs/lessons.md).

Tests: `details_says_how_a_piece_was_rolled_and_what_its_stats_do`
(`card.rs`) and `details_is_one_switch_for_every_piece_and_place`
(`shell.rs`). Pictures: `docs/gallery/2026-10-14-card-details`.

## Under the pointer

The pointer resting on a piece shows its comparison beside it, on the Gear
tab, the cargo, the report and the Inventory
(`card::pointed_comparison`; `point`, `show_hover_card` and
`Shell::points_at` in `shell.rs`).

- It is the pinned comparison's own lines (`weighed`), so the two cannot
  differ. What Equip would do is not on it: a consequence is said beside
  the action, never only under the pointer (menus.md).
- It shows nothing that a press does not: nothing is reachable by hover
  alone. A press pins the card, as before.
- The first card waits for the pointer to rest (`time.hover_card` in
  `menu.ron`, 0.25 s), so a pointer on its way elsewhere raises nothing.
  With a card up, the next piece's shows at once.
- No card for an empty slot, nor for the piece whose card is pinned beside
  it already.
- It is one node over the shell, out of the layout, so it moves nothing,
  and the pointer passes through it (`Pickable::IGNORE`). It stands beside
  its cell on the side of the window with more room, from the cell's top
  or bottom edge towards the middle, so it is never over its own piece
  (`beside`; a test holds it at the smallest window). Its width is
  the comparison's, `size.compare_width`.
- It can stand over the scene while the pointer rests, on the road too.

Pictures: `docs/gallery/2026-10-11-hover-card`. The menu tour moves Bevy's
pointer onto a piece (`Step::Point`), so the hover is the real one; a hand
on a mouse has not tried it.

## Colour

What an item says is coloured by kind (#251), the same on every screen:

| Kind | Token in `menu.ron` | Now |
| --- | --- | --- |
| An element's name | `colour.element` | Fire red, Ice blue, Plant green: the colour of an enemy of that element in a fight, which the scene and the deaths take from the same token |
| A stat, with its number | `colour.stat` | health pink, attack violet, defense teal, elemental magenta |
| A piece's name | its rarity's `look.colour` in `gear.ron` | Normal white, Magic blue, Rare yellow, Legendary orange, Set green: the colour of its drop in the loot shower (#309) |
| A change for the better or the worse | `colour.change` | gain mint, loss terracotta (#313) |

No screen chooses a colour. `show_facts` (`shell.rs`) passes every live text
through `ink::runs` (`src/game/menu/ink.rs`), which cuts it into runs: an
element is its whole word ("Fire"), and a stat is its word with the number
that belongs to it ("+15 health", "health 35"). The
text itself is what comes before the first colour and each run after it is a
`TextSpan` under it; an uncoloured run keeps following the state of the part
it is in. So a new screen that says "+2 attack" has it in violet with no
code.

- The word is always in the run, so a line reads the same with the colour
  taken away.
- The stat colours keep clear of the element colours, of plain text and of
  the rarity colours in `gear.ron`; a test holds the distances.
- A piece's name is one run in its rarity's colour, all of it, whatever
  words are in it (#309). `ink::Names` is every name `gear.ron` can make, a
  prefix and a noun, with its tier; `runs` finds those first and cuts the
  rest as before. The owner asked for the name or the card's background;
  the name was picked, because a card filled with a rarity's yellow, orange
  or green would take the contrast from the stat colours on it, and a name
  carries the rarity into rows and reports that have no card.
- No other name is coloured: a test runs every horse, enemy and rarity name
  in the data through `runs`, and another every piece name.
- **Gain and loss (#313).** Colour says whether a change is good, on the
  change alone: in "health 35 to 50 (+15)" the stat is its word, the two
  amounts are plain and "(+15)" is a gain; in the table the change column
  is ("+10 better", "+2 new" a gain; "-2 worse", "-4 lost" a loss; "same"
  plain). No line is a verdict on the piece, and none is coloured as one
  (#351): a stat's change is judged, the piece never is. `ink` reads the
  sign and the word the card wrote, so a change is never told by colour
  alone; every stat today is better for being higher.
- Tests hold the two colours 4.5 to 1 off the screen, a panel, a control
  and a selected control, clear of every other ink, the text and the
  rarities, and apart from each other in grey. Under the pointer a control
  is lighter and a loss falls to 2.8 to 1 there, as the element and stat
  colours do.
- A new kind of line (#69's affixes) is a variant of `Ink`, its words in
  `ink`, and a token.

Pictures: `docs/gallery/2026-10-12-gain-and-loss`, in colour and in grey.
The parts screen (`--parts`) has a sample with a gain, a loss and "same".

Not coloured: the labels of controls (the Fire, Ice and Plant buttons) and
the reason lines, which have their own colour by state. Items carry no
element yet; elements belong to abilities.

## The picture

A stand-in for every piece until there is art (#286; the owner: "gear badly
needs some kind of stand-in image"). It is drawn from `gear.ron` alone, of
flat UI nodes, so nothing depends on it.

- **The slot is the outline.** Each base in `gear.ron` has a `picture`: one
  flat shape, as shares of the picture's square, with each corner rounded
  by its own amount. A tall shield for the head, a wide blanket for the
  body, a low dome for the saddle.
- **The rarity is the colour and the mark.** The shape is filled with the
  rarity's colour, and the rarity's `shape` from the loot shower, seen from
  above, is cut out of it (`stamp`): a slab, a cross, a ringed stone, a sun
  with rays, three stones. So the piece on the ground and the piece in the
  menu carry the same sign, and it is told with the colour taken away. A
  prism is drawn as a disc.
- **An empty slot** is the outline alone, unfilled.

It is in every cell that shows a piece: the slots of the Gear tab, the
cargo and the drop under the report. It sits under the name, beside the
small lines, because the longest name needs a slot's whole width at the
smallest window (menu-shell.md). Its size is `size.picture` in `menu.ron`
(24), with `picture_inset` and `picture_stamp`. `show_pictures` draws a
square again only when its slot or rarity has changed. The pinned card in
the side panel has no picture: the cell that opened it is marked beside it,
and the panel has no height to spare.

A new slot or rarity needs a line of `gear.ron` and no code. Tests hold
that no two slots share an outline (`data.rs`) and no two rarities a mark
(`picture.rs`). To see all of them: `--parts`.

## The actions

Under the card, in every place's side panel, are the `card` controls of
`places.ron`. They are ordinary controls of the map (menu-shell.md, "Every
planned control"): the same reason lines, the same keys, the same "not
built" detail on a press. They show while the detail is a piece of gear.

**Changed only** (O) and **Details** (V) are switches on what the card
shows (see The comparison, The Details layer); **Another horse** (Q) is
Equip's list (below). **Equip** (E) is the one action that works. It is one action, `Action::Equip`,
done to the piece whose card is open (`Hand::open`):

| The card is on | Equip |
| --- | --- |
| the carried piece, or no card is open | puts the carried piece on the selected horse |
| a piece another horse wears | the two horses change what they wear in that slot (`loot::swap_worn`), nothing lost |
| another horse's empty slot | the same: the selected horse's piece goes over |
| what the selected horse wears | refused: "The selected horse wears it already." |
| the selected horse's empty slot | refused: "Nothing is worn there." |

So E with no card open still equips the drop in one key, as before. Equip
and Swap were two controls in two places (the cargo column and the Gear
tab, keys E and W) until this was built; W is no longer a key.

**Equip names who** (#356). The button reads "Equip on Sable", the
selected horse (`show_equip` in `shell.rs`). Beside it is **Another horse**
(Q), a switch (`OtherHorses`) that opens a list under itself of every
other horse, in the order they stand across the scene, left to right
(`scene::line_order`: the last of the party first, as the party walks
right). It is marked as chosen while open.

- **Choosing one** (`Shell::equip_on`) is Equip done for that horse, as a
  drag onto it is (`Game::drop_hand`): one Undo, the card stays on the
  piece. The horse is then selected, so the card compares there ("Juniper
  wears it."), and the list closes.
- **The pointer on one**, or the focus, shows beside it what choosing it
  would do (`card::offered`): the comparison on that horse, what Equip
  would do, and "Choose it to do it; the horse is selected after."
- **The horse that wears the piece** is in the list, unavailable, reading
  "Morrow wears it". The selected horse is not in it.
- Opening it puts the focus on the first horse that may take the piece, so
  Q then Enter equips there by keys alone.
- A new card closes it. It does nothing while no piece's card is pinned.

Test: `another_horse_equips_on_the_horse_chosen` (`shell.rs`). The tour
opens it by Q, chooses by Enter and undoes it. Chosen for the owner: the
line read left to right; choosing selects the horse; the wearer listed but
unavailable rather than left out; a list that opens in the panel and
pushes the controls under it down, rather than over them.

**The card stays on its piece.** After Equip the card that was open is on
the piece where it is now (`Game::equipped_at`, `Shell::piece_moved`), in
every place it was open, so its slot or tile is the one marked and
a second Equip is refused with its reason. A card on an empty slot stays on
that slot, which now holds the piece. A card on the carried piece is put
away when nothing is carried any more (`follow_the_party`).

List, Store, Lock, Favourite and Look are not built; the line under them
says so ("All but Equip, Another horse, Changed only, Details: not built
yet."), and a press on one says what it will do and names its issue. That
press puts the card away, as a press on any unbuilt control replaces the
detail.

## Dragging

A piece can be dragged onto a horse or a slot (#228;
docs/philosophy/inventory.md, "Direct manipulation, with an equal"). The
drag is Equip by another hand: E and the button do the same.

- **From** a slot that holds a piece (Gear tab, Inventory), a carried piece
  (the cargo column, the Inventory) or the report's drop. In town only
  (`on_drag_start` in `shell.rs`).
- **Onto** a slot of the piece's type, or a horse in the scene (#356; the
  same ray as a press, its tag marked as the target and the preview beside
  the tag), which takes
  it in that type's slot (the Inventory has no tile for a horse's empty
  slot). Never onto the horse that wears it. `Game::drop_hand` says where it
  may go, by `can(Equip)` for that horse.
- **Marked**: while the drag lasts every horse and slot it may go to is a
  target (`parts::Target`): the hover fill, and the accent for border and
  ring (`paint`; a test holds it apart from every state in grey).
- **Preview**: over a target the pointer's card is the comparison on that
  horse, whoever is selected, then what Equip would do and "Let go here to
  do it; anywhere else, and nothing changes." (`card::dragged`). Over
  anything else no card shows, so none covers the targets.
- **Letting go** on a target is `Game::drop_piece`: Equip for that horse,
  one Undo, a card open on the piece goes with it. Anywhere else nothing
  changes.
- **In the hand** (#363; `held.rs`, `fx/dangle.rs`). When the drag begins
  the piece comes out of its cell: every cell that shows it, the report's
  too, draws its slot empty (`picture::wanted`, given `Held::lifted`), and
  its picture hangs from the pointer on a short string. It swings as a
  damped pendulum, kicked by each change in the pointer's speed: a quick
  move swings it back against the move, a stop lets it settle within a
  second or so, and a slow move tilts it a degree or two. Let go, it flies
  the short way (`swing.fly`, 0.14 s) into the cell it went to, a slot or
  a horse's tag, or back into its own (`Drag::from`); the cell shows it
  again once it is there. The game has changed already by then: nothing
  waits on the flight.
  - The numbers are `swing` in `menu.ron`. The swing is worked out in
    fixed steps of 2 ms, so it is the same at any frame rate.
  - The piece is under the pointer's card (`GlobalZIndex` 3 to the card's
    5), so the preview is never behind it, and the pointer passes through
    it (`Pickable::IGNORE` on every part). Hanging under the pointer, it
    covers part of the target the pointer is on, never that target's edge
    all round.
- Its tutorial is an entry on the list of things to try, "Drag a piece onto
  a horse." (`Dragged`), once something has been equipped.

A card too wide to sit beside its cell (a slot of the Gear tab) is kept
inside the window, over the cell's far end (`beside`).

Tests: `a_dragged_piece_goes_where_equip_would_put_it` (`mod.rs`),
`a_dragged_piece_previews_the_drop` (`card.rs`),
`a_target_of_a_drag_stands_out_without_colour` (`parts.rs`), and for the
piece in the hand `it_settles_from_a_push`,
`a_slow_move_barely_tilts_it`, `the_swing_is_the_same_at_any_frame_rate`
(`dangle.rs`) and `a_picture_is_of_the_piece_or_of_the_empty_slot`
(`picture.rs`). The menu tour drags with Bevy's own pointer (`Step::Grab`,
`DragTo`, `LetGo`); pictures: `docs/gallery/2026-10-14-dragging`, and
`docs/gallery/2026-10-15-in-the-hand` for the piece in the hand.

## The appraisal in camp

What the report says of the drop by itself, with no card opened and no
input (#11; `card::appraisal`, shown by `report_text`). It says what was
brought back and that it is carried, and nothing about where it should go:
the game picks no horse for a piece (owner, 2026-10-12, #351).

    NEW GEAR: Sunforged Halter
    Carried until you put it on a horse.

- No horse is named, whoever is selected and whatever the horses wear.
- The drop's cell is under it, with what it is and "Carried in the cargo.".
  A press opens its card, where the comparison is.
- It is said of the last trail's piece while that is the newest carried.
  Once the piece is on a horse the cargo's newest is an older piece, and
  nothing is appraised.
- Nothing goes on at the gate. Setting out with the piece left alone
  changes no horse and no cargo, and the road's log says nothing of gear
  (`setting_out_equips_nothing`).
- Once the piece is on, by E, what happened is kept (`card::equipped`):
  "Equipped A on Juniper, in place of B:", a line per stat that changed,
  and "B went to the cargo.". It is the report's note.

Held by `camp_says_what_was_brought_back_and_names_no_horse` and
`the_card_gives_numbers_and_no_verdict`.

## Fitting

What a detail says scrolls, with its bar, where it does not fit; the actions
under it do not, so Equip is always in view. At the smallest window
(1120 x 640) a side panel shows whose the piece would be, both pieces,
where the selected one is and the first of what changes; the rest and the
table are a scroll of the wheel away.

PageUp and PageDown scroll the card (#304; `scroll` in the shell, the keys
in `places.ron`, `scroll_keys`). A press moves by `size.scroll_page`, four
fifths, of what is in view, so the last line read is still there, and it
stops at either end. While the words do not fit, a line under them says
"PageUp, PageDown: read the rest"; it goes when they fit. In the Gear tab
at the smallest window the room is six lines. A held key does not repeat, and a card
opened after another keeps the place the last was scrolled to.

Tests in `card.rs` hold: a piece has the same comparison carried and
worn; the appraisal names each stat that changes and no other; the
consequence is said exactly when Equip can be done and names the piece lost
or moved; an empty slot and an empty cargo say what belongs there.

## Not here yet (the rest of #228)

- The card in the stall, storage and the codex: none of those lists exists.
- List and Favourite from the card: #99 and the favourite mark (#150).
- The indicators of #150: new, equipped, favourite, look owned, listed.
- In the Details layer: modifier sources, set membership and sale value
  (docs/philosophy/inventory.md), once affixes (#69), sets and the stall
  (#99) exist.
