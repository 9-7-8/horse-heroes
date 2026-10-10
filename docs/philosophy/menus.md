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

### Built first, with every button in place

The owner, 2026-10-10: "The menus should be one of the first things we
build, so we can tune it throughout. All buttons should be there, even if
they're not functional, as early as possible."

So the whole map of the menus exists before most of the game behind it does:
every place, every tab and every planned control, in its final position. A
control whose system is not built responds when pressed and says so, as any
unavailable control does. When the system arrives, the button starts working
and nothing moves. The map is one data file, so a place, tab or control can
be renamed or rearranged while tuning without touching code.

This is what makes navigation tunable from the start: the cost of a layout
shows up while it is still cheap to change, and a new system is fitted to a
place that has already been walked many times.

### One job to a screen

Each tab has one main job and one primary action. Before a screen is laid
out, write five lines for it: what the player should feel, the decision that
produces it, what they must know to decide, the shortest understandable way
to act, and how they will see what changed. A screen that cannot fill in the
decision line is a report, and belongs in the Journal.

### One name for one thing

Horses is never also Party, Team or Roster; coins are never also gold. The
names in the bar, the tabs, the buttons and the messages come from the same
list. A button says what it will do with what: "List 18 pieces for 240
coins", never "OK".

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

Back and Cancel are different things: Back returns to where the player was;
Cancel abandons a change not yet made. Closing a panel puts the focus back on
the control that opened it. Related things link to each other, so a pet slot
opens the pets that could go in it and an ingredient of a town project opens
where it comes from, and the bar remains the plain route to everything.

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

The card has three layers. The **summary** is what it is, its slot, what it
does and whether it matters now. The **detail**, on selection, is every
effect, the comparison, and what happens to the piece it displaces. The
**advanced** layer, behind a Details control, is formulas and assumptions.
A cost or a consequence is never only in a tooltip. Differences are marked
by sign or word as well as colour, and a percentage says what it is a
percentage of.

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

A list holds still while it is being read. New loot arriving does not move a
row out from under the pointer; it waits in an incoming group or for a
refresh the player asks for. The filters in force are shown, with one action
to clear them, and "nothing matches these filters" is a different message
from "nothing here yet". Numbers that grow do not push the layout about.

### Every control has all its states

Default, hover, focus, pressed, selected and unavailable are each drawn
differently, on every control, from one shared set of parts. Hover means
"this can be done"; selected means "this is chosen"; they never look alike.
Selecting a piece to compare it does not favourite it or add it to a sale.
A press is acknowledged at once, and the next press is accepted while the
flourish for the last one is still playing. The place the player is in is
marked by shape or underline as well as colour.

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

The footer says in one line what the party is doing and whether anything is
in its way, and says so when something that sounds like a blocker is not
one: "Bags full. The journey continues; keep rules choose what is carried."

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

A count is a floor, not the goal. Never cut an action at the cost of the
player understanding or being safe; time to understand matters more than
time to click. So watch as well as count: which place a newcomer tries
first, the wrong turns, the backtracking, whether they can predict what a
batch action will do before committing, and whether they can say afterwards
what their change accomplished.

## For Horse Heroes

- The slice has one text overlay and key presses. The shell (bar, status
  strip, footer, selected horse, cargo) and the full skeleton of buttons are
  built first, in 0.2.0 beside the loot shower, before any single screen is
  made good, because every later screen hangs on them.
- The research guide proposes six destinations of its own and a scene the
  player can collapse. Neither is taken: the owner's five places and her
  fixed proportion (vision.md) stand. Its sections on colour, sound, timing
  and upgrade controls are left for when those are worked on.
- Menu presentation is kept apart from game state: equipping, listing and
  assigning are decided by `src/sim`, by stable identifiers and never list
  positions, and an animation is never the transaction.
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
chat and recorded in vision.md, and her instruction the same day that the
menus are built first with every button in place. The principles are
recommendations written for this game the same day, drawing on the interface
sections of town.md, cargo.md and juice.md, and on a research guide she
commissioned and handed over, "Rules for good menu UX and UI in an
incremental idle RPG" (sections 3 to 6, 9 to 12, 16, 18 and 19, the menu
design; she asked that little else be taken from it). Its layouts and
thresholds are proposals to prototype. All of it should be corrected by what
the running game shows.
