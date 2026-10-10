# The menu shell, 2026-10-10

Pictures taken while building the menu shell (#226): what was there
before, what the first tries looked like, and a walk through every place
and tab of the shell as committed. All from `cargo run -- --seed 6`; the
walk is `IDLE_TOUR` (procedures/build-and-run.txt). How the shell works:
[menu-shell.md](../../menu-shell.md).

## Before

**1.** Before: one text overlay across the whole window, in camp.

![Before: one text overlay across the whole window, in camp.](01-before-camp.jpg)

**2.** Before: the same overlay on the trail.

![Before: the same overlay on the trail.](02-before-trail.jpg)

## Along the way

**3.** First run of the shell. The scene is the top third; the bar, the status strip and the footer are in place.

![First run of the shell. The scene is the top third; the bar, the status strip and the footer are in place.](03-first-run.jpg)

**4.** First try at the report in the Journal: the old overlay text, moved as it was. It repeats the party, which the status strip already shows.

![First try at the report in the Journal: the old overlay text, moved as it was. It repeats the party, which the status strip already shows.](04-journal-report.jpg)

**5.** The same report on the road: the fight log runs off the bottom of the place.

![The same report on the road: the fight log runs off the bottom of the place.](05-on-the-road.jpg)

**6.** Second try: two columns, the log on the right. It fits, but the right column starts wherever the left one ends, so it jumps as the text changes.

![Second try: two columns, the log on the right. It fits, but the right column starts wherever the left one ends, so it jumps as the text changes.](06-on-the-road.jpg)

## The shell as committed: a walk through every place and tab

**7.** Town opens on Horses, Gear, with the first horse selected.

![Town opens on Horses, Gear, with the first horse selected.](07-town-opens-on-horses.jpg)

**8.** Morrow pressed in the list: selected there, in the status strip, and in the text of the tab.

![Morrow pressed in the list: selected there, in the status strip, and in the text of the tab.](08-second-horse-selected.jpg)

**9.** A gear slot pressed: its detail opens in the side panel, beside the list it came from.

![A gear slot pressed: its detail opens in the side panel, beside the list it came from.](09-side-panel-open.jpg)

**10.** Horses, Gear.

![Horses, Gear.](10-horses-gear.jpg)

**11.** Horses, Look: in its position, saying what will be decided there and that it is not built yet.

![Horses, Look: in its position, saying what will be decided there and that it is not built yet.](11-horses-look.jpg)

**12.** Horses, Pet: in its position, saying what will be decided there and that it is not built yet.

![Horses, Pet: in its position, saying what will be decided there and that it is not built yet.](12-horses-pet.jpg)

**13.** Horses, Abilities: in its position, saying what will be decided there and that it is not built yet.

![Horses, Abilities: in its position, saying what will be decided there and that it is not built yet.](13-horses-abilities.jpg)

**14.** Trade, Sell: in its position, saying what will be decided there and that it is not built yet.

![Trade, Sell: in its position, saying what will be decided there and that it is not built yet.](14-trade-sell.jpg)

**15.** Trade, Buy: in its position, saying what will be decided there and that it is not built yet.

![Trade, Buy: in its position, saying what will be decided there and that it is not built yet.](15-trade-buy.jpg)

**16.** Trade, Storage: in its position, saying what will be decided there and that it is not built yet.

![Trade, Storage: in its position, saying what will be decided there and that it is not built yet.](16-trade-storage.jpg)

**17.** Trade, Rules: in its position, saying what will be decided there and that it is not built yet.

![Trade, Rules: in its position, saying what will be decided there and that it is not built yet.](17-trade-rules.jpg)

**18.** Map: a place with no tabs. The bar, strip and footer are where they were.

![Map: a place with no tabs. The bar, strip and footer are where they were.](18-map.jpg)

**19.** Town, Projects: in its position, saying what will be decided there and that it is not built yet.

![Town, Projects: in its position, saying what will be decided there and that it is not built yet.](19-town-projects.jpg)

**20.** Town, Residents: in its position, saying what will be decided there and that it is not built yet.

![Town, Residents: in its position, saying what will be decided there and that it is not built yet.](20-town-residents.jpg)

**21.** Journal, Report.

![Journal, Report.](21-journal-report.jpg)

**22.** Journal, Journal: in its position, saying what will be decided there and that it is not built yet.

![Journal, Journal: in its position, saying what will be decided there and that it is not built yet.](22-journal-journal.jpg)

**23.** Journal, Codex: in its position, saying what will be decided there and that it is not built yet.

![Journal, Codex: in its position, saying what will be decided there and that it is not built yet.](23-journal-codex.jpg)

**24.** Back on Horses after visiting every other place: still on Abilities, where it was left. The party has set out meanwhile; the footer says so and Set out is unavailable.

![Back on Horses after visiting every other place: still on Abilities, where it was left. The party has set out meanwhile; the footer says so and Set out is unavailable.](24-back-on-horses-as-it-was-left.jpg)

**25.** Back on the Gear tab: the side panel is still open on the Head slot.

![Back on the Gear tab: the side panel is still open on the Head slot.](25-side-panel-still-open-on-its-tab.jpg)

**26.** Journal, Report on the road: the fight on the left, its log on the right, each in a half that does not move.

![Journal, Report on the road: the fight on the left, its log on the right, each in a half that does not move.](26-on-the-road.jpg)

**27.** Sable selected from the status strip on the road; Horses shows the same horse.

![Sable selected from the status strip on the road; Horses shows the same horse.](27-on-the-road-horses.jpg)

**28.** Back in town: the result and the drop on the left, the next trail on the right. The footer has the departure again.

![Back in town: the result and the drop on the left, the next trail on the right. The footer has the departure again.](28-back-in-town-with-the-report.jpg)
