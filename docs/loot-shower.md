# The loot shower

What is built of it so far: drops as physical objects (#4), the rarity
signal (#6), the pickup (#7) and fountain sizes (#8). The eruption and
collection beats of [the loot fountain](philosophy/loot-fountain.md). Screen
response and sound are their own issues (#9, #10) and are not built.

## What happens

1. The sim brings an enemy down and records a `Kill` (which enemy, its size,
   the coins it dropped, the name of its fountain) in `Expedition::kills`.
2. `game::advance` takes the kills after each sim tick and writes a `Killed`
   message: where the body was, the coins, the fountain, and where the
   enemies still standing are.
3. `fx::drops` plans a burst (`plan_burst`) the way that fountain says and
   spawns one coin piece per coin, up to the fountain's `most_pieces`, and
   a gear piece if the kill dropped gear. The seed puts the gear somewhere
   among the coins (`burst_kinds`).
4. Each piece waits its turn (the fountain's `delay_secs`, then
   `stagger_secs` apart, in a shuffled order),
   leaves the body on its own arc, tumbles, bounces `bounces` times with a
   squash at each landing, stops, and lies down flat.
5. The burst lies still, then its pieces go to the party one after another
   (Pickup, below).

The sim decides what drops; `fx` only shows it. Every kill drops coins. The
enemy whose fall wins the trail also drops the run's one gear piece, the same
piece town reports afterwards. A lost trail's piece has no body to come out
of and is only reported in town. More kinds of reward per kill are #5, and
a final enemy's own drops #29.

## Fountain sizes

The big fountain is kept for real completion points, or the player goes numb
([loot-fountain.md](philosophy/loot-fountain.md), Scarcity and restraint).
So how a burst is thrown depends on the kind of kill. A fountain is an entry
in `fountains` in `loot_shower.ron`:

| | `small` | `full` |
|---|---|---|
| for | an ordinary enemy | a trail's last enemy |
| `delay_secs`, from the death to the first piece | 0 | 0.3 |
| `stagger_secs`, between pieces | 0.02 | 0.045 |
| `fan_degrees` | 140 | 220 |
| `up_speed` | 6 to 7.5 | 8.5 to 11.5 |
| `out_speed` | 0.6 to 1.4 | 0.9 to 2.4 |
| `most_pieces`, coin pieces at most | 6 | 24 |

An enemy names its fountain in `assets/data/enemies.ron` (`fountain:`), the
sim carries the name on the `Kill` without reading it, and `fx` looks it up.
A new kind of kill, a milestone trail level say, is one more entry in
`fountains` and its name on the enemies that get it: no code. A test holds
that every enemy names a fountain there is, and that a final enemy's waits
longer, shows more pieces, opens wider and throws higher than any ordinary
one's.

With today's drops an ordinary kill is one to four coins, up for well under
a second, and a final kill is ten to fifteen coins and the run's gear piece
going up about twice as high over most of a second. All fourteen
numbers are first guesses and nobody has tuned them.

The final fountain is not yet seen at its best: the kill that wins the trail
sends the party to town in the same moment (Standing and walking, below), so
it plays while the scene shrinks. Holding the road for it is #9.

## Rarity

A gear piece is a box (`gear_size` in `loot_shower.ron`); a coin is a disc.
Its rarity is shown four ways, all in the rarity's `look` in
`assets/data/gear.ron`, beside its weights:

- `colour`: Normal white, Magic blue, Rare yellow, Legendary orange, Set
  green. The five tiers and their colours are the owner's choice
  (2026-10-10, [vision.md](philosophy/vision.md)).
- `glow`: how strongly the piece gives off that colour. Normal has none.
- `size`: a multiple of `gear_size`. Higher tiers are larger.
- `beam`: a column of light standing on the ground where the piece is. It
  is hidden with the piece and rises over `rise_secs` from the moment the
  piece is thrown, so it is up while the piece is still in the air.
  Legendary and Set have one; any tier given a `beam` gets one.

A piece names its rarity by its place in the list in `gear.ron`, lowest
first, and there is no rarity enum, so a new tier is one more line there and
no code. A test holds that no two tiers share a colour. Rarity colours do
not have to differ from the element colours: rarity shows on drops and in
menus only (owner, 2026-10-10). Blue is close to Ice, orange to Fire and
green to Plant, and a yellow Rare is hard to pick out among gold coins;
shape per tier (#240) is what will separate those.

## Pickup

`plan_pickup` decides, when the burst is thrown, when each piece leaves the
ground: the whole burst is at rest for `pickup_pause_secs`, so it can be
read, and then the pieces leave `pickup_stagger_secs` apart, the nearest to
the party first. A piece heads for the middle of the nearest horse
(`scene::nearest_horse`), starts at `pickup_speed`, gains `pickup_accel`
every second (`step_home`), shrinks over the last `pickup_shrink` of the
way, and is gone when it gets there. A beam sinks as its piece leaves. The
party does not wait for it: if it has walked on, the pieces catch it up.

The coins counter counts a coin when it arrives, not when it drops. A kill's
coins are shared over its coin pieces (`coin_values`; a kill worth more
than `most_pieces` coins has pieces worth several). `game::advance` adds
them to `Uncollected` as it reports the kill, each piece takes its share
off as it arrives, and the counter shows what the party has earned less
what is uncollected. This is only when the counter shows a coin: what the
party owns is the sim's count and does not depend on the drops. A piece
the ground carried off screen before its turn counts as arrived.

Gear arrives the same way and moves no counter yet: the bags counter waits
for saddlebags (#100). Once there are pets they do the fetching (owner,
2026-10-10; #106, #137), and what is kept is for the keep rules (#52).

## The numbers

All of them are named fields in `assets/data/loot_shower.ron`, loaded into
the `Shower` resource. No drop code holds a number of its own, except three
that are not feel: when `--burst` or `--pickup` fires, how many directions are tried, and
how far off screen a drop is forgotten.

## The fan, and keeping enemies in view

The fan is the fountain's `fan_degrees` wide and opens towards the party. Each piece gets
its own slice of it, so a burst is spread out however few pieces it has.
Before a piece is launched its flight is run to the end (`rest_point`); if it
would come to rest within `keep_clear` of an enemy still standing, another
direction is rolled, and after eight tries it goes straight across the
trail, which is always clear because enemies stand in a line along it.
Pieces are small and rest on the ground, so a resting piece does not cover
an enemy; one in the air can pass in front of one for a moment.

## Standing and walking

The party stands still to fight: the ground stops scrolling and the horses'
legs come to rest (`Game::party_moving`). Drops lie where they landed until their
turn to be collected. When the party walks on, the ground takes the drops
with it to the left, also the ones still in the air; a piece on its way to
the party is not carried. One that got well off screen uncollected is
removed.

The last kill of a trail sends the party back to town in the same moment, so
its burst is seen, and collected, while the scene shrinks to the town
layout.

## The test burst

`F3`, or `--burst` one second after launch, throws `burst_pieces` coins and
the gear named in `burst_gear` (three Normal, two Magic, and one each of
Rare, Legendary and Set) as the fountain `burst_fountain` (the full one),
from `burst_seed`, from where the first enemy stands. `--burst-at SECONDS`
moves it: the party sets out at 12 s, so `--burst --burst-at 14` throws it
in the road view. It is the same burst every
time, it stays where it lands, and pressing F3 again replaces it. `F4`, or
`--pickup`, throws the same burst and lets the party collect it; its pieces
are worth no coins. When its
pieces are at rest the log has one line:

    test burst: 48 pieces at rest after 3.57 s, mean frame 33.4 ms, worst
    39.7 ms (with none in the air: mean 33.3 ms, worst 33.8 ms)

Compare the two means to see what a burst costs. To tune:
`procedures/tune-loot-feel.txt`.

    $env:IDLE_SHOT = "screenshots/burst.png"; $env:IDLE_SHOT_AT = "2.4"
    cargo run -- --seed 6 --burst *> run.log

A picture at 2.4 s is the burst in the air; at 3.6 s the last pieces are in
the air and both beams are up; at 5.2 s it is at rest. With
`--burst-at 14`, 16.6 s is the same late moment on the road.

## Tests

`src/fx/drops.rs`, for every fountain where it is one's numbers: the data
is usable; every enemy names a fountain there is; a final enemy's fountain
is a bigger event than an ordinary one's; the same seed gives the same
burst; launches wait out the delay, are staggered and within their speeds;
a burst is spread over the fan; every piece bounces as often as the data
says and rests on the trail; no piece rests on a standing enemy, whichever
of the three died; the test burst has gear of every rarity; the seed puts
the gear among the coins; no two rarities share a colour, and the top tier
has a beam; a kill's coin pieces add up to its coins; the
burst waits out the pause and then leaves one piece at a time, nearest
first; a collected piece only gains speed and arrives, at 20, 60 and 240
frames a second.
`src/sim/expedition.rs`: every kill is reported once, and the coins of the
kills add up to the run's coins; the winning kill carries the run's gear and
no other kill carries any.

Pictures: [drops](gallery/2026-10-10-drops/README.md),
[the rarity signal](gallery/2026-10-10-rarity-signal/README.md),
[the pickup](gallery/2026-10-10-pickup/README.md),
[fountain sizes](gallery/2026-10-10-fountain-sizes/README.md).
