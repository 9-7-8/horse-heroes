# The loot shower

What is built of it so far: drops as physical objects (#4) and the rarity
signal (#6). The eruption beat of
[the loot fountain](philosophy/loot-fountain.md). Pickup, fountain sizes,
screen response and sound are their own issues (#7 to #10) and are not built.

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

The sim decides what drops; `fx` only shows it. Every kill drops coins. The
enemy whose fall wins the trail also drops the run's one gear piece, the same
piece town reports afterwards. A lost trail's piece has no body to come out
of and is only reported in town. More gear per run is #8.

## Rarity

A gear piece is a box (`gear_size` in `loot_shower.ron`); a coin is a disc.
Its rarity is shown four ways, all in the rarity's `look` in
`assets/data/gear.ron`, beside its weights:

- `colour`: Common a dull grey-brown, Uncommon violet, Rare pink.
- `glow`: how strongly the piece gives off that colour. Common has none.
- `size`: a multiple of `gear_size`. Higher tiers are larger.
- `beam`: a column of light standing on the ground where the piece is. It
  is hidden with the piece and rises over `rise_secs` from the moment the
  piece is thrown, so it is up while the piece is still in the air. Only
  Rare has one today; any tier given a `beam` gets one.

A piece names its rarity by its place in the list in `gear.ron`, lowest
first, and there is no rarity enum, so a new tier is one more line there and
no code. The colours get lighter up the tiers, so they differ in greyscale
too, and each is at least 40 degrees of hue from the three element colours
(`scene::element_colour`) and from the coins' gold; a test holds both.

## The numbers

All of them are named fields in `assets/data/loot_shower.ron`, loaded into
the `Shower` resource. No drop code holds a number of its own, except three
that are not feel: when `--burst` fires, how many directions are tried, and
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
legs come to rest (`Game::party_moving`). Drops lie where they landed for as
long as the fight lasts. When the party walks on, the ground takes the drops
with it to the left, also the ones still in the air, and each is removed
once it is well off screen. Nothing picks them up yet (#7).

The last kill of a trail sends the party back to town in the same moment, so
its burst is seen while the scene shrinks to the town layout and drifts off
with the ground.

## The test burst

`F3`, or `--burst` one second after launch, throws `burst_pieces` coins and
the gear named in `burst_gear` (three Common, two Uncommon, one Rare) from
`burst_seed` from where the first enemy stands. It is the same burst every
time, it stays where it lands, and pressing F3 again replaces it. When its
pieces are at rest the log has one line:

    test burst: 46 pieces at rest after 2.47 s, mean frame 33.4 ms, worst
    39.3 ms (with none in the air: mean 33.3 ms, worst 37.2 ms)

Compare the two means to see what a burst costs. To tune:
`procedures/tune-loot-feel.txt`.

    $env:IDLE_SHOT = "screenshots/burst.png"; $env:IDLE_SHOT_AT = "2.2"
    cargo run -- --seed 6 --burst *> run.log

A picture at 2.2 s is the burst in the air; at 2.8 s the Rare is in the air
under its beam; at 4.5 s it is at rest.

## Tests

`src/fx/drops.rs`: the data is usable; the same seed gives the same burst;
launches are staggered and within their speeds; a burst is spread over the
fan; every piece bounces as often as the data says and rests on the trail;
no piece rests on a standing enemy, whichever of the three died; the test
burst has gear of every rarity; the seed puts the gear among the coins; the
rarity colours are lighter up the tiers and clear of the element colours.
`src/sim/expedition.rs`: every kill is reported once, and the coins of the
kills add up to the run's coins; the winning kill carries the run's gear and
no other kill carries any.

Pictures: [drops](gallery/2026-10-10-drops/README.md),
[the rarity signal](gallery/2026-10-10-rarity-signal/README.md).
