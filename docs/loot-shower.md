# The loot shower

What is built of it so far: drops as physical objects (#4), the rarity
signal (#6), the pickup (#7), fountain sizes (#8), the screen's response
with the calm after the final kill (#9), and sound (#10). The eruption and
collection beats of [the loot fountain](philosophy/loot-fountain.md).

## What happens

1. The sim brings an enemy down and records a `Kill` (which enemy, its size,
   the coins it dropped, the name of its fountain) in `Expedition::kills`.
2. `game::advance` takes the kills after each sim tick and writes a `Killed`
   message: where the body was, the coins, the fountain, and where the
   enemies still standing are. The same message starts the body's death
   ([death-events.md](death-events.md)), which the drops come out of.
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
| `sound`, as the first piece leaves | `launch_small` | `launch_full` |

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

The final fountain plays in the road view, with the party standing and
nothing else going on (The calm after the kill, below).

## Rarity

A coin is a disc. A gear piece's rarity is shown five ways, all in the
rarity's `look` in `assets/data/gear.ron`, beside its weights:

- `shape`: the solids the piece is built of, as it lies flat. Each is a
  `Box(long, thick, deep)`, `Ball(radius)`, `Ring(radius, tube radius)` or
  `Prism(sides, radius, thick)`, with `at` (along, up, across from the
  piece's middle) and `turn` in degrees about the upright axis. Normal is a
  plain slab, Magic a cross, Rare a ring with a stone in it, Legendary a
  sun with eight rays, Set three stones for the herd. This is what tells a
  tier apart with the colour taken out (`docs/philosophy/cargo.md`, "Not
  colour alone"). The five shapes are a first choice made without the
  owner. All the solids of a piece share its one material.
  `RarityLook::reach` and `thickness` (`src/sim/data.rs`) measure the shape,
  for how high the piece tumbles and what it lies on; `part_mesh`
  (`src/fx/drops.rs`) makes each solid's mesh.

- `colour`: Normal white, Magic blue, Rare yellow, Legendary orange, Set
  green. The five tiers and their colours are the owner's choice
  (2026-10-10, [vision.md](philosophy/vision.md)).
- `glow`: how strongly the piece gives off that colour. Normal has none.
- `size`: what the shape is multiplied by. Higher tiers are larger.
- `beam`: a column of light standing on the ground where the piece is. It
  is hidden with the piece and rises over `rise_secs` from the moment the
  piece is thrown, so it is up while the piece is still in the air.
  Legendary and Set have one; any tier given a `beam` gets one.

A piece names its rarity by its place in the list in `gear.ron`, lowest
first, and there is no rarity enum, so a new tier is one more line there and
no code. Tests hold that no two tiers share a colour or a shape. Rarity
colours do not have to differ from the element colours: rarity shows on
drops and in menus only (owner, 2026-10-10). Blue is close to Ice, orange
to Fire and green to Plant, and a yellow Rare is hard to pick out among
gold coins; the shape is what separates those.

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

## Screen response

When a burst erupts, which is when its first piece leaves (the fountain's
`delay_secs` after the death), the scene shakes and flashes once. How
strongly depends on the best piece in the burst and not on how many pieces
there are ([juice.md](philosophy/juice.md): never spend a higher level on a
lower event). `best_tier` finds the highest rarity among the burst's gear;
that rarity's `response` in `gear.ron` gives the two strengths, and a burst
with no gear in it uses `response.coins` in `loot_shower.ron`:

| best piece | `shake`, world units | `flash`, 0 to 1 |
|---|---|---|
| coins alone | 0 | 0 |
| Normal | 0.03 | 0 |
| Magic | 0.06 | 0.015 |
| Rare | 0.14 | 0.03 |
| Legendary | 0.26 | 0.06 |
| Set | 0.3 | 0.07 |

So an ordinary kill, which drops only coins today, does nothing to the
screen, and a final kill is answered by what it dropped.

- The shake moves the camera across and up the screen (`shake_offset`),
  never further than its strength, `shake_hz` times a second, dying away
  over `shake_secs`.
- The flash is a sheet of the best piece's rarity colour over the scene's
  share of the window, never over the menus. It starts at its strength and
  only fades, over `flash_secs` (`flash_alpha`): one pulse, never a
  flicker. It is mixed as light, so a small number shows a good deal: 0.07
  of pure green over the black background is a clear dark green.
- `shake_scale` and `flash_scale` in `loot_shower.ron` multiply every shake
  and every flash. Zero turns that one off; they are where a settings
  control will reach.
- A burst that erupts while another's response is still playing only takes
  over if it is stronger than what is left of it.

The test burst answers too (its best piece is the Set), so F3 is how to
look at a response. All of these numbers are first guesses and nobody has
tuned them. There is no single low, medium and high scale shared with
combat and town, which [juice.md](philosophy/juice.md) asks for once those
have effects of their own.

## Sound

There are no sound files. Every sound is a recipe in
`assets/data/sounds.ron`: a list of voices, each one tone with a wave
(`Sine`, `Triangle`, `Square`, or `Noise` for a hiss), a pitch, a slide, a
start, a length, an attack and a level. `fx::sound::render` makes each into
samples when the game starts, and Bevy plays them as its own kind of audio
source (`Clip`). So every sound is ours to give away under the GPL, and
nothing is read from disk at run time.

What is heard, and where it is named:

| when | sound | named in |
|---|---|---|
| an enemy's body breaks | `death_small`, a crack; `death_big`, the crack and a low boom | the death's `sound`, `deaths.ron` ([death-events.md](death-events.md)) |
| a burst's first piece leaves | `launch_small`, a pop; `launch_full`, a rush of air over a rising note | the fountain's `sound`, `loot_shower.ron` |
| a piece of gear is thrown | one for each rarity, below | the rarity's `look.sound`, `gear.ron` |
| a piece touches the ground | `land_coin`, a clink; `land_gear`, a thud | `sound.land_coin`, `sound.land_gear` |
| a piece reaches the party | `pickup_coin`, a tick; `pickup_gear`, a soft note | `sound.pickup_coin`, `sound.pickup_gear` |
| a horse uses its ability (#358) | one for each: Ember Shot a flare rushing up, Frost Burst a shimmer of falling glassy notes, Thorn Shot a low twang, Ward a soft swell | the ability's `sound`, `abilities.ron` |
| a horse's hit lands on an enemy | by the hit's element: `hit_plain` a thump, `hit_fire` a crackle, `hit_ice` a tink, `hit_plant` a woody knock, at `sound.hit_level` | `sound.hits`, `loot_shower.ron` |

An enemy's hit on a horse makes no sound. The sim tells each use of an
ability in `Expedition::casts`, which `advance` sends on as `Cast`; the
hits come as `Landed`, the message the damage numbers read.

The rarity sounds grow with the tier: Normal is one dull note, Magic two
notes going up, Rare three bright notes up an octave with a glint on the
last, Legendary a low boom with a major chord climbing out of it, and Set a
bell of stacked fifths struck at once with a high ring after. A piece's
sound plays as it leaves the body, so it is heard before the piece lands.

Sound is never one noise per event
([juice.md](philosophy/juice.md): aggregate repeated events, or vary them):

- **Landings.** Those that come within `land_gap_secs` of another are one
  sound (`Gate`). Each later bounce of a piece is `land_fade` as loud as the
  one before, and each landing heard is moved up to `land_vary` semitones
  either way. The test burst's 144 touches are 36 sounds.
- **Pickups.** Each coin that arrives is one step further up `pickup_scale`
  than the last (`Run`, `run_semitones`): the scale's notes, then the same
  an octave up. The run stops climbing after `pickup_most_steps` and stays
  on its top note; a pickup more than `pickup_run_secs` after the last
  starts a new run at the bottom. With today's numbers that is two octaves
  over ten coins, so an ordinary kill is a few rising notes and a final
  kill a whole run. Gear arriving has its own sound and is not part of the
  run.
- **The fight.** An ability's sound or a hit's heard again within
  `hit_gap_secs` of the last time is not played (`Gates`, one gate for
  each sound), so a Frost Burst on three enemies is one tink, and two
  horses on one ability firing together are one sound.

`sound.volume` in `loot_shower.ron` multiplies every sound; zero turns
sound off, and it is where a settings control will reach. `--mute` plays
nothing. To see what was played, in the order it was:

    $env:RUST_LOG = "info,horse_heroes::fx::sound=debug"

Every recipe and every number is a first guess, written without hearing
it: nobody has listened yet. F3 and F4 are how to. A sound is played
faster to raise its pitch, so a high tick is also a shorter one.

## The calm after the kill

The burst is legible because of the quiet that follows the kill
([loot-fountain.md](philosophy/loot-fountain.md), Contrast). When a trail
is won the party does not go to town in the moment its last enemy falls.
It stands on the road, the road view stays, and nothing else happens while
the full fountain goes up, lands, lies still for the pause and is
collected; the party sets off for town when the last piece has reached it
(`game::road_is_held`, from `Uncollected::pieces`, which `fx` counts in as
it throws a kill's pieces and out as each arrives). With today's drops
that is about four and a half seconds. The footer says the trail is
cleared meanwhile. A lost trail has no fountain and goes home at once.

A place left open on the road keeps the scene in the top third, and a beam
still runs off the top of it there; the hold does not close the place.

## The test burst

`F3`, or `--burst` one second after launch, throws `burst_pieces` coins and
the gear named in `burst_gear` (three Normal, two Magic, and one each of
Rare, Legendary and Set) as the fountain `burst_fountain` (the full one),
from `burst_seed`, from where the first enemy stands. `--burst-at SECONDS`
moves it: with `--keep-going` the party sets out at 12 s, so `--keep-going --burst
--burst-at 14` throws it in the road view. It is the same burst every
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
has a beam; no two rarities share a shape, and every piece lies flat; a
kill's coin pieces add up to its coins; the
burst waits out the pause and then leaves one piece at a time, nearest
first; a collected piece only gains speed and arrives, at 20, 60 and 240
frames a second.
`src/fx/response.rs`: the response numbers are usable, and no flash is
strong enough to hide the scene; the best piece decides and the count does
not; a better piece is never answered more weakly, and a Rare is answered
more strongly, both ways, than coins alone or any lower tier; either scale
at zero turns its half off and leaves the other; a shake stays within its
strength and ends, and a flash only fades.
`src/fx/sound.rs`: every recipe makes a sound that can be played, within
full scale, short, and silent at both ends; a voice is the pitch it says;
the same recipe gives the same samples; every sound the data names is
there, and every fountain and rarity names one; no two rarities share a
sound or its notes, none is used for anything else, a better tier's is
never shorter and a Rare has more notes than any tier below; a final
death's sound is told from an ordinary one's, and a death and its launch
fit inside full scale together; a stream of
pickups only rises until it tops out; a run starts again after a silence
and not between two pieces of one burst; landings together are one sound
and a later bounce is quieter.
`src/game/mod.rs`: the road is held for a won trail's last drops and for
nothing else.
`src/sim/expedition.rs`: every kill is reported once, and the coins of the
kills add up to the run's coins; every kill throws a piece of gear or more
(#352), and the winning kill throws the trail's own piece last. A kill's
burst is its coins and every piece it threw.

Pictures: [drops](gallery/2026-10-10-drops/README.md),
[the rarity signal](gallery/2026-10-10-rarity-signal/README.md),
[rarity by shape](gallery/2026-10-10-rarity-shapes/README.md),
[the pickup](gallery/2026-10-10-pickup/README.md),
[fountain sizes](gallery/2026-10-10-fountain-sizes/README.md),
[screen response](gallery/2026-10-10-screen-response/README.md).
