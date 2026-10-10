# Menus and navigation

How the player gets around. The owner's judgment (2026-10-10): "This game is
going to live and die on how easy and intuitive the menus are to navigate."
The player never controls a fight; every decision they make is made in a
menu, so the menus are the controls of this game.

Her four decisions are in [vision.md](vision.md), "Menus and navigation":
five places with the horse first, mouse first with a key for everything,
look and plan on the road, and this file. The rest of this file is advice
built on those and has not been tried in the running game. The work is epic
#225.

## The map of the menus

    +--------------------------------------------------+
    |              3D scene (top third)                |
    +--------------------------------------------------+
    | [Horses] Trade   Map   Town   Journal    coins.. |
    +--------+--------------------------+--------------+
    | party  |  Gear  Abilities Pet Look| shared cargo |
    |  (o) A |                          |  [] [] [] [] |
    |  ( ) B |   slots of horse A       |  [] [] [] [] |
    |  ( ) C |   item card / compare    |  filter sort |
    +--------+--------------------------+--------------+
    |  Ready for: Mossford, 3 days          [ Set out ]|
    +--------------------------------------------------+

| Place | Tabs | What is decided there |
| --- | --- | --- |
| **Horses** | Gear, Abilities, Pet, Look | What the selected horse wears, which abilities it carries, which pet rides with it, how it looks. The party strip and the shared cargo stay in view on every tab |
| **Trade** | Sell, Buy, Storage, Rules | What is listed and at what price; local gear and goods; the town's stash; keep rules and auto-buy rules |
| **Map** | none | Destination, route options, the itinerary, and what is restored |
| **Town** | Projects, Residents | Deliveries to projects; recruits, retired horses, invitations back, retiring. Goodwill is in its header |
| **Journal** | Report, Journal, Codex | The latest loot, trail and stall reports; what happened; what has been found |

Three things never move: the **bar** of places, the **status strip** (coins,
bags, hold, party health, world day), and the **footer**, which in town says
what the party is ready for and holds the one departure action, and on the
road says where the party is and how many changes are planned. Settings are
an icon at the end of the bar, not a place.

## Principles

### Town opens on the horse

Town is mostly about assigning gear, so that is where the player lands: the
selected horse, its slots, and the cargo it can draw on. The most frequent
job takes no navigation at all. Everything rarer is one click further away,
and nothing is more than two.

### Two levels and no further

A place, then a tab. Detail about one thing (an item, a resident, a town
project) opens in a side panel beside the list it came from, never as a new
screen and never as a popup on a popup. The player can always say where they
are in two words: "Horses, Abilities".

There is one kind of modal: the confirmation of something that cannot be
undone.

### The item is the handle

A piece of gear appears in cargo, on a horse, in the report, in the stall, in
storage and in the codex. Wherever it appears it is the same card, and from
it every action that applies is offered: equip on the selected horse, list,
store, favourite, wear as a look. The player never carries an item to the
screen that owns a verb. If moving between two places is how a common job
gets done, the layout is wrong.

Hover shows the card with the comparison against what the selected horse
wears. A click pins it with its actions. Dragging to a slot is a shortcut and
never the only way.

### One selected horse, everywhere

Selecting a horse selects it in every place and tab, and every comparison,
preview and default action refers to it. Each place remembers its tab, its
scroll position and its filters, so going to look something up and coming
back costs nothing.

### Things stay where they are

A place keeps its position in the bar from the first minute to the last.
Systems that have not opened yet leave their space reserved, and appear there
when the event that makes them relevant arrives. The primary action of a
screen is always in the same corner. The same list, with the same filters and
sorting, is used for every collection of items.

### Undo, not "are you sure"

Equipping, listing, pricing, spending ability points, changing a look and
assigning a pet are all free and reversible by the owner's rules. They apply
at once and can be undone. A confirmation is kept for what cannot be taken
back: retiring a horse, selling a Rare piece, a significant purchase
([town.md](town.md), Interface). Asking about everything teaches the player
to click through the one question that matters.

### Nothing is silently unavailable

A control that cannot be used says why, in place: "in town only", "opens at
the second town", "bags are full". An empty list says what belongs in it and
where that comes from. A newcomer should be able to learn the game by reading
what is greyed out.

### A place says when it wants attention

A mark on a place means there is something in it the player has not seen or
can now decide: a new look, an unspent point, an upgrade in cargo. It clears
when seen. Nothing that needs no decision earns a mark, and a mark is never a
count of routine events ([juice.md](juice.md), the three levels). Indicators
are told apart by shape or text as well as colour.

### No menu ever holds the game up

The road runs and the stall sells with a menu open. A report never has to be
dismissed before the party moves on; it waits in the Journal. Nothing in the
menus is on a timer, and nothing vanishes while it is being read. This is
pillar 2 seen from the interface.

### On the road: look, and plan

Every place opens on the road. Opening one brings the scene back to the top
third; closing it lets the scene expand again, and no menu covers a drop in
flight. Gear, abilities and pets change in town only, so on the road those
changes are **planned**: shown on the horse as pending, listed in the footer,
editable until arrival, and applied together when the party reaches a town.
A player who checks in between towns is never left with nothing to do, and
the rule that builds change in town still holds.

### The mouse first, and a key for everything

Designed for a mouse. Every place, tab and common action also has a key, and
the key is printed on the control it works, so shortcuts are learned by
looking. Escape goes back exactly one level and never leaves town. Nothing
is reachable by hover alone.

### Count the clicks

Ease is measurable. Keep a short list of the jobs done most often and a
budget for each, and walk them in the running game whenever a screen changes:

- equip an upgrade the report just showed;
- list everything not wanted;
- give a horse a different ability;
- set out for the next town;
- find out why something did not sell, or why a piece was left behind.

Also the questions [town.md](town.md) already asks: accidental sales,
abandoned comparisons, time to find a useful purchase.

## For Horse Heroes

- The slice has one text overlay and key presses. The shell (bar, status
  strip, footer, selected horse, cargo) is the first thing to build, before
  any single screen is made good, because every later screen hangs on it.
- Primitive UI is in keeping with pillar 3. Flat panels and plain text are
  enough to learn whether the map of the menus is right; nothing may depend
  on finished art.
- The fountain is the product (pillar 1). The menus serve it in two ways:
  they get out of its way on the road, and they make the appraisal, the
  fountain's last beat, one glance and one action.
- An itinerary means the party leaves town by itself. Without planning on the
  road, an idle player could go a whole session unable to act on a drop they
  watched fall. That is why the plan exists.
- The wardrobe is the Look tab of the selected horse, and its collection view
  is reached from there. If the fashion endgame outgrows a tab, that is a
  finding to bring to the owner, not a reason to add a sixth place quietly.
- Sale, retirement and recruitment stay apart, as town.md requires: selling
  is in Trade; recruiting and retiring are in Town.
- The danger is the usual one: five places each growing tabs until the menus
  are the breadth the vision warns against. A new tab should pass the same
  test as a new system.

## Source

The owner's four decisions of 2026-10-10, made from options put to her in
chat and recorded in vision.md. The principles are recommendations written
for this game the same day, drawing on the interface sections of town.md,
cargo.md and juice.md; they do not come from one of her documents and should
be corrected by what the running game shows.
