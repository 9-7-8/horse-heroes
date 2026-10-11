# The inventory and the comparison

What the Inventory is for, and how a piece is compared with the one it would
replace. The owner's ruling comes first; the rest is advice from a document
she handed over on 2026-10-11 and has not been tried on a player. Navigation
in general is [menus.md](menus.md); what a piece must be is
[items-and-builds.md](items-and-builds.md) and [gear.md](gear.md); what is
carried and kept is [cargo.md](cargo.md).

## Confirmed by the owner

- **The Inventory is all the items, not divided by horse** (2026-10-11:
  "The inventory screen should just be all items, NOT DIVIDED BY HORSE").
  A piece says who wears it; the screen is not arranged by who does.
- It is a grid split by item type, of every piece the horses and the
  caravan hold (#308).

## Principles

### Agency, not administration

Arranging and choosing items is part of playing, not work between the
interesting parts. Make the tradeoff meaningful, its consequence
understandable, and the action's feedback clear.

### Keep the friction that is a decision; remove the rest

"Which piece deserves this slot?" is friction worth having. "Why does it
take six presses to move this?" is not. A limit is not satisfying by
itself: it has to make a decision that serves the experience. Too tight and
it is a rearranging chore; with no pressure it is a storage database.

### An idle game makes abundance manageable

- Ordinary, repeated loot is dealt with in batches.
- A piece that might be interesting stays easy to inspect.
- A piece can be protected before anything automatic may dispose of it.
- A comparison shows the tradeoff, and does not call every higher number an
  upgrade.
- Capacity, where there is any, has a purpose and does not force
  housekeeping.
- Repeated clicks are not engagement.

### Start from what the player should feel

Work back from the experience to the rules. Here: satisfaction in what has
piled up, mastery of a growing collection, and looking forward to the next
upgrade. The behaviour that goes with it: look over the loot, compare
builds, set filters and rules, turn the unwanted into progress.

### The loop of one piece

Acquire it, understand what it is for, compare it with what is needed,
decide, be told it happened, and see the next trail differ because of it.
The satisfaction is in the decision and its consequence, so everything
routine around it is made short.

### What the screen must do

- **Hierarchy.** What the piece is, whether it is worn, and the action that
  matters now come first.
- **Recognition over recall.** What is worn and what a piece does are shown
  at the point of deciding, not remembered from another screen.
- **Progressive disclosure.** The main view is short; modifiers and
  formulas are behind a Details control. A build-changing effect is never
  hidden only because it is complicated.
- **Direct manipulation, with an equal.** Dragging where it suits, and a
  button and a key that do the same.
- **Prevent and recover.** Protect favourites, preview a bulk disposal,
  undo where the rules allow; do not confirm what is harmless.
- **Consistency.** Selecting, moving about and the actions' keys are the
  same in the inventory, storage and a shop.

### The comparison answers three questions at once

What am I replacing? What changes if I equip this? Does that suit this
horse's build?

1. **Context first.** Who receives the piece, which slot, the piece
   equipped and the piece selected, and, when it cannot be used, exactly
   why. In a party the same piece helps one member and weakens another, so
   "an upgrade" always names its recipient, and choosing the recipient
   stays in view.
2. **Stable positions.** Equipped first and selected second, labelled in
   words, and not told apart by panel colour. Choosing another piece
   changes the selected side only; the rows keep their order.
3. **One table, not two lists.** A stat is on one line for both pieces,
   with the same units and precision, numbers right-aligned, and the change
   beside them. A modifier that appears is "new" and one that goes is
   "lost". A percentage-point change is not written like a relative one. A
   decrease that is good (a shorter cooldown) is labelled as good.
4. **The pieces apart from the build.** The table says how the two objects
   differ. A short "after equipping" says what happens to the horse. A
   figure that is an estimate names what it assumes.
5. **No single power number that settles it.** A score may sort a list; it
   must not hide lost utility or a changed synergy. The comparison does the
   arithmetic, not the thinking.
6. **Unlike effects are not forced into a number.** Two different passives
   are set side by side in words, with a line on what is lost and what is
   gained.
7. **It can be pinned,** and everything shown under the pointer can be
   reached by keys.
8. **A swap that touches more than one slot shows the whole result,** and
   the comparison is never quietly made against whichever worn piece
   flatters the candidate.
9. **Preview against the loadout,** not against a passing combat bonus,
   and say what the preview includes.

### Each colour has one job, in one place

- Most of the panel is neutral: plain surfaces, strong primary text,
  quieter but readable secondary text.
- Rarity colours the header, an element its own tag, and gain or loss the
  change column. The two amounts stay neutral: colour the consequence, not
  every number.
- A change is coloured by whether it is good, not by its sign. A change
  that depends on the build ("Fire bonus lost, Plant bonus new") stays
  neutral unless the screen says what build it is judging by.
- Colour repeats what a sign, a word or a shape already says. Check the
  coloured text against the surface it is really on (4.5 to 1 for text that
  matters), in grey and under colour-vision simulation.
- A rarer piece is not shown as more important than the properties that
  decide whether it is useful.

### Actions at the foot, and feedback that does not wait

- A few distinct actions under the comparison: equip on the selected
  horse, lock or favourite, upgrade options, sell or salvage. The primary
  one has its own accent. A destructive one is never as prominent, and
  never where Equip is on another screen.
- After equipping, the panel is at once the new baseline, with a slight
  highlight on the slot, a sound and "Equipped to Sable".
- The state changes immediately and the presentation finishes afterwards;
  no animation holds the next press. No upgrade marker glows for ever.
- Routine loot gets light feedback so that a real find stands out. A batch
  gets one summary and one sound, not one per piece. A refused action
  points at the limit and says why, quietly.

### Judge it by tasks

- Can a new player find what is worn, what is carried and what can be done?
- How many presses does a routine equip, comparison or sale take?
- Can the player say why they kept one piece over another?
- How often is something equipped or lost by mistake?
- Can the player predict what a keep rule will keep?
- Does the feedback make the action clear, or get in the way of the next?

## For Horse Heroes

Checked against the game as built on 2026-10-11 (`docs/item-card.md`,
`docs/menu-shell.md`, `assets/data/gear.ron`).

- **Built.** The Inventory tab of Horses (#308): every piece worn or
  carried, in a grid by type, with the place's whole width; each cell names
  its piece in its rarity's colour, pictures it, says what it adds and who
  wears it or that it is carried. Beside it the comparison: the recipient
  and slot, equipped and selected, what changes for the horse, one aligned
  table with new, lost and same in words, and what Equip would do. The
  recipient is the selected horse in the strip, which stays in view, and a
  horse's key compares the same piece there.
- **Already in line.** One selected horse everywhere, undo over
  confirmation, a reason for every refusal, a key for every action, and
  a verdict that names its horse ("Upgrade for Sable: +11")
  ([menus.md](menus.md)).
- **Differs today.**
  - The comparison in the Gear tab and the Journal is still the older card
    of lines ("health 35 to 50 (+15)"), not the table: #312.
  - Stats are coloured by which stat they are (#251), on the amounts as
    well as the label, and no colour says gain or loss: #313.
  - The verdict is one number of points (`loot::gain`). Today a piece only
    adds to four stats, so the number hides nothing; it must stop being the
    last word when gear effects (#13), affixes (#69) and sets (#36) arrive.
  - Equip is acknowledged by a line in the report and nothing on the
    screen where it happened: #40, #43.
  - At the smallest window the comparison's table is a scroll away, and no
    key scrolls it: #304.
- **Not yet relevant.** Batches, protection and salvage wait for a cargo
  that holds more than one piece (#34, #99, #52, #167); filters, sorting
  and search are #235; a Details layer is the rest of #228.
- **Capacity.** The owner has ruled that a full bag never stops a journey
  ([cargo.md](cargo.md)); that is the "capacity with a purpose" above.

## Source

`docs/source/inventory-and-comparison.md`, handed over by the owner on
2026-10-11: a design description of a satisfying inventory, for survival
horror and for idle RPGs, and a specification for an idle RPG's comparison
window. The survival-horror half is left out as not this game. Its numbers
and item names are illustrations and were not taken.
