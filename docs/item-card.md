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
| `src/game/menu/shell.rs` | `Item::Carried` and `Item::Worn`, the details that are pieces; `CarriedCell`; `Page::Card`, the actions under the card |

## The three layers

From the owner's guide on menu design (#228): a summary, the detail on
selection, and formulas behind a Details control. Two are built.

**The summary** is what a list shows of a piece: its name, then its rarity,
slot and what it adds (`kind`), then whether it matters to the selected
horse now (`verdict`: "Upgrade for Sable: +11", "No better for Sable",
"Worse for Sable: -3"; a word and a sign, never colour alone). The carried
piece is shown this way in the cargo column and under the report. A slot of
the gear overview shows the shorter form its width allows: name, slot and
what the piece adds.

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
- Hover showing the card. Today a press pins it.
- Dragging a piece to a slot.
- The indicators of #150: new, equipped, favourite, look owned, listed.
- Formulas behind a Details control.
