# Memory

What the game holds on the GPU and in RAM, who holds it, and why (#375).
It is an idle game of flat colours and should hold little; 0.2.5 ran out of
GPU memory on the owner's laptop (#374). How to measure is at the end.

## The GPU

**wgpu's memory blocks were most of it.** wgpu takes the GPU's memory from the
driver in blocks and hands out pieces of them. Its default
(`MemoryHints::Performance`) makes each block 128 to 256 MB on the card and 64
to 128 MB in shared memory, and a block is held whole however little is in
it, so a game that needs tens of megabytes held six hundred. `main.rs` sets
`MemoryHints::MemoryUsage` on the `RenderPlugin`: blocks of 8 to 64 MB on the
card, 4 to 32 MB shared. Measured below.

What is in those blocks, made once and shared unless said otherwise:

- **Meshes, 171.** `game/scene.rs` setup: one cube shared by the stones, the
  gate and the enemies; the trail and backdrop planes, one per layer; one
  sphere and one post shared by every backdrop shape; each horse piece and
  its outline once, shared by all three horses (they are one shape); each
  worn piece and its rim once. `fx/drops.rs`: each rarity's parts, the coin
  and the beam. `fx/tells.rs`, `fx/death.rs`, `fx/impact.rs`: a ball and a
  cube each. None is made after startup.
- **Materials, 59.** Made at setup, except three caches: the sparks' one per
  colour thrown (`SparkLooks`, a handful of colours, which hit kinds and
  elements fix), worn gear's one per look worn (`WornLooks`), and none
  elsewhere. Each horse has its own body, hoof and hair materials (they
  flash and tint apart). `WornLooks` keeps only the looks some horse wears
  now, and a hidden worn piece goes back to the shared plain material, so a
  look taken off is let go.
- **Images.** The fonts' glyph atlases, one per font size (Bevy never
  frees one, so text is only ever drawn at a few whole-pixel sizes; the damage
  numbers' pop is a scale, #374). A few of Bevy's own. And the looks (`game/menu/picture.rs`, `LookImages`):
  a piece's look painted for a menu picture at each size it is shown at, and
  at 64 by 64 for a worn piece. `LookImages` lets go of every image only it
  still holds, every 5 s by the wall clock, so a look nobody shows is freed
  and painted again if it comes back.
- **The window's targets.** The swap chain, depth, and 4x multisampling,
  which keeps the flat-coloured edges smooth. From the smallest window
  (1120x640) to a large one (2400x1300) is about 20 MB; not worth losing the
  smoothing for.
- **Shadows are off** (`DirectionalLight` default), so there is no shadow map.
  There is no HDR target and no bloom.

Images keep their pixels in RAM as well as on the card (Bevy's default
`RenderAssetUsages`): after an out-of-memory error every image is sent again
(`render_errors.rs`), which needs them.

## RAM

Mostly Bevy, wgpu and the driver; the debug build is about twice the release.
What grows with play is the cargo: every dropped piece is kept until it is
sold, and the Inventory has a tile for every place in the cargo in each of
its groups (`grow_cargo`, #352), made once and never taken away. That is a
few entities per piece per group; the RAM it costs is small beside the rest
(below), but it is the one thing that keeps growing (#TODO).

## Measured

GTX 1070 Ti, Vulkan, debug build, seed 6, 2026-10-19:

| | GPU dedicated | GPU shared | RAM (private) |
|---|---|---|---|
| 0.2.5, 30-min soak (154b7cb) | 622 MB, flat | 127 MB | 987 to 1027 MB |
| small blocks, 16 s run | 429 MB | 67 MB | 708 MB |
| empty Bevy window + 3D camera, big blocks | 603 MB | 123 MB | 878 MB |
| the same, small blocks | 425 MB | 63 MB | 640 MB |
| empty window, no camera, small blocks | 198 MB | | 370 MB |

The game's own content is about 4 MB of GPU memory over an empty Bevy
window: what is left is Bevy's 3D view (about 230 MB) and the driver. The
30-minute soak's GPU memory was flat; its image count climbed 124 to 281
with the cargo (fixed by `forget_unheld`, not yet re-measured) and its
entities 3593 to 9314 (the Inventory's tiles, above). A 30-minute soak
after the change is still to run.

## How to measure

- **A long run:** `pwsh tools/soak.ps1 -Minutes 30 -Seed 6`. `memory.tsv`
  has, each minute, the working set, private bytes, and the GPU's dedicated
  and shared memory as Windows counts them for the process (the
  `GPU Process Memory` counter, the same as Task Manager's). The report's
  `memory` and `GPU memory` lines read it, and say when either grows; its
  beats count images, meshes and materials (#374).
- **A quick look:** Task Manager, Details, with the "Dedicated GPU memory"
  column added.
- The soak runs the debug build; measure a release with the installed game
  and Task Manager.
