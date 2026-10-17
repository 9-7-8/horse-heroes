# Parallax

Layers of scenery behind the trail, each scrolling at its own share of the
trail's speed, slower the further back it is (#254). They give the trail
depth and make a walk look like going somewhere.

## What is on screen

Three layers, nearest first, all in `assets/data/parallax.ron`:

| Layer   | z     | Speed (share of the trail's) | Shapes                |
|---------|-------|------------------------------|-----------------------|
| Bushes  | -4    | 0.75                         | 22 half-buried balls  |
| A wood  | -7    | 0.5                          | 16 trees              |
| Hills   | -11   | 0.25                         | 9 wide mounds         |

The trail's far edge is at z -2.5. The camera is orthographic, so a layer
further back is not smaller, only higher on the screen and slower; the speed
is the whole of the depth cue, with colour (darker and bluer further back).

Each layer has a strip of ground under it, `ground_depth` deep and centred on
its z, so the strips meet and nothing floats on black.

## Rules it keeps

- **Nothing scrolls while the party stands.** `scroll` in
  `src/game/scene.rs` moves the layers only when `Game::party_moving`, like
  the marker stones.
- **Frame time does not grow with distance.** Each layer has a fixed number
  of shapes over its `span`; a shape that leaves on the left comes round on
  the right (`wrapped`). The span is at least the trail's width (80), wider
  than the widest window, so the wrap is never seen.
- **Nothing reaches into the lane or reads as a target**
  (`docs/philosophy/fairyland-content.md`, Trails). Shapes are round
  (enemies are cubes), flat and unlit (no highlight to catch the eye), and
  every layer's widest shape stands behind the lane's far edge. Drops fly in
  front of the layers, so the depth buffer keeps them on top.
- **Places are seeded.** Each layer rolls its shapes' places and sizes from
  its own `seed`, so the same scenery comes back every run.

`the_far_layers_move_slower_and_stay_out_of_the_lane` holds the data to
these: nearest first, each slower than the last, all between still and the
trail's speed, spans wide enough, and no shape or ground strip in the lane.
`a_layer_comes_round_again` holds the wrap.

## Not built

- The layers do not change with the trail or its biome; every trail has the
  same three.
- The town gate and the marker stones still move at the trail's own speed;
  the scenery beside the lane (#74) is not built.
