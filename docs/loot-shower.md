# The loot shower

What is built of it so far: drops as physical objects (#4), the rarity
signal (#6) and the pickup (#7). The eruption and collection beats of
[the loot fountain](philosophy/loot-fountain.md). Fountain sizes, screen
response and sound are their own issues (#8 to #10) and are not built.

## What happens

1. The sim brings an enemy down and records a `Kill` (which enemy, its size,
   the coins it dropped) in `Expedition::kills`.
2. `game::advance` takes the kills after each sim tick and writes a `Killed`
   message: where the body was, the coins, and where the enemies still
   standing are.
3. `fx::drops` plans a burst (`plan_burst`) and spawns one coin piece per
   coin, up to `most_pieces`, and a gear piece if the kill dropped gear.
   The seed puts the gear somewhere among the coins (`burst_kinds`).
4. Each piece waits its turn (`stagger_secs` apart, in a shuffled order),
   leaves the body on its own arc, tumbles, bounces `bounces` times with a
   squash at each landing, stops, and lies down flat.
5. The burst lies still, then its pieces go to the party one after another
   (Pickup, below).

The sim decides what drops; `fx` only shows it. Every kill drops coins. The
enemy whose fall wins the trail also drops the run's one gear piece, the same
piece town reports afterwards. A lost trail's piece has no body to come out
of and is only reported in town. More gear per run is #8.

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

The fan is `fan_degrees` wide and opens towards the party. Each piece gets
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
Rare, Legendary and Set) from
`burst_seed` from where the first enemy stands. It is the same burst every
time, it stays where it lands, and pressing F3 again replaces it. `F4`, or
`--pickup`, throws the same burst and lets the party collect it; its pieces
are worth no coins. When its
pieces are at rest the log has one line:

    test burst: 48 pieces at rest after 2.67 s, mean frame 33.4 ms, worst
    39.1 ms (with none in the air: mean 33.4 ms, worst 34.3 ms)

Compare the two means to see what a burst costs. To tune:
`procedures/tune-loot-feel.txt`.

    $env:IDLE_SHOT = "screenshots/burst.png"; $env:IDLE_SHOT_AT = "2.2"
    cargo run -- --seed 6 --burst *> run.log

A picture at 2.2 s is the burst in the air; at 2.9 s the last pieces are in
the air and both beams are up; at 4.8 s it is at rest.

## Tests

`src/fx/drops.rs`: the data is usable; the same seed gives the same burst;
launches are staggered and within their speeds; a burst is spread over the
fan; every piece bounces as often as the data says and rests on the trail;
no piece rests on a standing enemy, whichever of the three died; the test
burst has gear of every rarity; the seed puts the gear among the coins; the
no two rarities share a colour, and the top tier has a beam; a kill's coin pieces add up to its coins; the
burst waits out the pause and then leaves one piece at a time, nearest
first; a collected piece only gains speed and arrives, at 20, 60 and 240
frames a second.
`src/sim/expedition.rs`: every kill is reported once, and the coins of the
kills add up to the run's coins; the winning kill carries the run's gear and
no other kill carries any.

Pictures: [drops](gallery/2026-10-10-drops/README.md),
[the rarity signal](gallery/2026-10-10-rarity-signal/README.md),
[the pickup](gallery/2026-10-10-pickup/README.md).
