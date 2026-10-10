# The loot shower

What is built of it so far: drops as physical objects (#4). The eruption
beat of [the loot fountain](philosophy/loot-fountain.md). Rarity signal,
pickup, fountain sizes, screen response and sound are their own issues (#6 to
#10) and are not built.

## What happens

1. The sim brings an enemy down and records a `Kill` (which enemy, its size,
   the coins it dropped) in `Expedition::kills`.
2. `game::advance` takes the kills after each sim tick and writes a `Killed`
   message: where the body was, the coins, and where the enemies still
   standing are.
3. `fx::drops` plans a burst (`plan_burst`) and spawns one coin piece per
   coin, up to `most_pieces`.
4. Each piece waits its turn (`stagger_secs` apart, in a shuffled order),
   leaves the body on its own arc, tumbles, bounces `bounces` times with a
   squash at each landing, stops, and lies down flat.

The sim decides what drops; `fx` only shows it. Today the only thing a kill
drops is coins. The run's gear piece is still rolled at the end of the trail
and reported in town; it joins the shower with #8.

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

`F3`, or `--burst` one second after launch, throws `burst_pieces` pieces from
`burst_seed` from where the first enemy stands. It is the same burst every
time, it stays where it lands, and pressing F3 again replaces it. When its
pieces are at rest the log has one line:

    test burst: 40 pieces at rest after 2.34 s, mean frame 33.4 ms, worst
    52.0 ms (with none in the air: mean 33.4 ms, worst 33.8 ms)

Compare the two means to see what a burst costs. To tune:
`procedures/tune-loot-feel.txt`.

    $env:IDLE_SHOT = "screenshots/burst.png"; $env:IDLE_SHOT_AT = "2.2"
    cargo run -- --seed 6 --burst *> run.log

A picture at 2.2 s is the burst in the air; at 4.5 s it is at rest.

## Tests

`src/fx/drops.rs`: the data is usable; the same seed gives the same burst;
launches are staggered and within their speeds; a burst is spread over the
fan; every piece bounces as often as the data says and rests on the trail;
no piece rests on a standing enemy, whichever of the three died.
`src/sim/expedition.rs`: every kill is reported once, and the coins of the
kills add up to the run's coins.

Pictures: [gallery](gallery/2026-10-10-drops/README.md).
