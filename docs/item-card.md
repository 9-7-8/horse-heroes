# The item card

One way of showing a piece of gear, wherever it is (#228, part built). The
guidance is [philosophy/menus.md](philosophy/menus.md), "The item is the
handle": the player never carries a piece to the screen that owns a verb.
It hangs on the shell's side panel: [menu-shell.md](menu-shell.md).

## Where things are

| File | What it holds |
| --- | --- |
| `src/game/menu/card.rs` | What the card says: `kind`, `verdict`, `compare`, `consequence`, `pinned`. Plain text from the game, tested without Bevy |
| `assets/data/places.ron`, `card` | The card's actions, in order: Equip, List, Store, Lock, Favourite, Look |
| `src/game/mod.rs` | `Piece`, where a piece is; `Action::Equip`; `Game::piece` and `Game::equipped_at` |
| `src/game/menu/picture.rs` | The stand-in picture: `stamp`, `picture`, `draw`, `show_pictures` |
| `assets/data/gear.ron` | A slot's flat shape (`bases`, `picture`) and a rarity's colour and `shape` |
| `src/game/menu/shell.rs` | `Item::Carried` and `Item::Worn`, the details that are pieces; `CarriedCell`; `Page::Card`, the actions under the card |

## The three layers

From the owner's guide on menu design (#228): a summary, the detail on
selection, and formulas behind a Details control. Two are built.

**The summary** is what a list shows of a piece: its name, then its rarity,
slot and what it adds (`kind`), then whether it matters to the selected
horse now (`verdict`: "Upgrade for Sable: +11", "No better for Sable",
"Worse for Sable: -3"; a word and a sign, never colour alone). The carried
piece is shown this way in the cargo column and under the report. A slot of
the gear overview shows the shorter form its width allows: the name and
what the piece adds. Its picture says which slot and which rarity.

**The pinned card** is what a press opens in the side panel of the place the
piece was pressed in:

    Sunforged Halter
    Legendary, Head  +15 health  +2 attack  +4 defense  +2 elemental
    Carried in the cargo.

    On Sable, who wears nothing there:
    health 35 to 50 (+15)
    attack 3 to 5 (+2)
    defense 1 to 5 (+4)
    elemental 2 to 4 (+2)
    An upgrade: +11 points.
    Equip puts it on Sable.

- The third line is where it is: carried, or worn and by whom.
- The comparison (`compare`) is on the selected horse, the one Equip puts it
  on: each stat of that horse that would change, as it is and as it would
  be, with the signed difference, then the points (`loot::gain`). This is
  the "difference per stat" of #11. A horse is not compared with what it
  wears already. Selecting another horse changes the comparison.
- The last line (`consequence`) is what Equip would do, and it is there
  exactly when Equip can be done. It names the displaced piece: "Old Halter
  is thrown away: the cargo cannot hold it yet." (a carried piece put over a
  worn one; the cargo holds one thing until #227) or "Equip puts A on
  Juniper, and B on Sable." (two horses changing places).
- An empty slot has a card too: whose slot, "Nothing worn.", and where gear
  comes from.

**Formulas behind a Details control** are not built.

## Under the pointer

The pointer resting on a piece shows its card beside it, in the gear
overview, the cargo and the report (`card::hovered`; `point`,
`show_hover_card` and `Shell::points_at` in `shell.rs`):

    Sunforged Halter
    Legendary, Head  +15 health  +2 attack  +4 defense  +2 elemental
    Carried in the cargo.

    On Sable, who wears nothing there:
    health 35 to 50 (+15)
    ...
    An upgrade: +11 points.

    A press pins it, with its actions.

- It is the pinned card's own lines (`face`), so the two cannot differ. What
  Equip would do is not on it: a consequence is said beside the action,
  never only under the pointer (menus.md).
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
  `size.hover_card_width`.
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

No screen chooses a colour. `show_facts` (`shell.rs`) passes every live text
through `ink::runs` (`src/game/menu/ink.rs`), which cuts it into runs: an
element is its whole word ("Fire"), and a stat is its word with the number
that belongs to it ("+15 health", "health 35", "health 35 to 50 (+15)"). The
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
- A new kind of line (#69's affixes) is a variant of `Ink`, its words in
  `ink`, and a token.

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

It is in every cell that shows a piece: the slots of the overview, the
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

**Equip** (E) is the one that works. It is one action, `Action::Equip`,
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

**The card stays on its piece.** After Equip the card that was open is on
the piece where it is now (`Game::equipped_at`, `Shell::piece_moved`), in
every place it was open, so its slot in the overview is the one marked and
a second Equip is refused with its reason. A card on an empty slot stays on
that slot, which now holds the piece. A card on the carried piece is put
away when nothing is carried any more (`follow_the_party`).

List, Store, Lock, Favourite and Look are not built; the line under them
says so, and a press on one says what it will do and names its issue. That
press puts the card away, as a press on any unbuilt control replaces the
detail.

## The appraisal in camp

What the report says of the drop by itself, with no card opened and no
input (#11; `card::appraisal`, shown by `report_text`). It is about the
horse the piece goes on when the party sets out (`loot::best_wearer`),
whoever is selected:

    NEW GEAR: Sunforged Halter, on setting out:
    On Juniper, who wears nothing there:
    health 40 to 55 (+15)
    attack 6 to 8 (+2)
    defense 1 to 5 (+4)
    elemental 1 to 3 (+2)
    An upgrade: +11 points.
    E puts it on the selected horse instead.

- The comparison is the card's own (`compare`). Over a worn piece its
  heading names that piece, and a line says it is thrown away.
- When no horse is better off: "left behind on setting out", then why, a
  verdict per horse ("Worse for Juniper: -3"), and that E puts it on the
  selected horse all the same.
- The drop's cell is under it, with its verdict for the selected horse.
- Once the piece is on, by E or at the gate, what happened is kept
  (`card::equipped`): "Equipped A on Juniper, in place of B:", the same
  lines per stat, and "B was thrown away". By E it is the report's note;
  at the gate it is the first lines of the road's log, after "The party
  sets out", so the Journal still says it on the road.

Held by `camp_says_what_the_drop_replaces_or_why_it_is_not_used`.

## Fitting

What a detail says scrolls, with its bar, where it does not fit; the actions
under it do not, so Equip is always in view. At the smallest window
(1120 x 640) the Horses place shows the name, what the piece is and where,
and the comparison is a scroll of the wheel away; the summary in the list
has the verdict meanwhile. The Journal's panel is wider and shows the start
of the comparison. There is no key that scrolls the card.

Tests in `card.rs` hold: a piece has the same opening lines carried and
worn; the comparison names each stat that changes and no other; the
consequence is said exactly when Equip can be done and names the piece lost
or moved; an empty slot and an empty cargo say what belongs there.

## Not here yet (the rest of #228)

- The card in the stall, storage and the codex: none of those lists exists.
- List and Favourite from the card: #99 and the favourite mark (#150).
- Dragging a piece to a slot.
- The indicators of #150: new, equipped, favourite, look owned, listed.
- Formulas behind a Details control.
