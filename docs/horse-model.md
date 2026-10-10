# The horse model

How a horse is built. What it should look like, and why, is
[philosophy/horse-shape.md](philosophy/horse-shape.md); this file is how the
code gets there. Pictures: [gallery](gallery/2026-10-10-horse-build/README.md).

| Part | File |
| --- | --- |
| The proportions | `assets/data/horse_shape.ron` |
| Proportions to lengths to pieces | `src/sim/horse_shape.rs` |
| Pieces to meshes, joints, hiding the hair | `src/game/scene.rs` |

## The data

`horse_shape.ron` holds every length as a ratio of H, the height at the
withers, and `height` is H in world units. Change `height` and the whole
horse changes with it; nothing else in the file is a length.

A dimension is `(base, least, most)`. Every horse is built to the base
today. Least and most are the brief's adjustment window, there for
variation (#63) and used so far only by the tests.

Two values are not chosen but follow from others:

- **Clearance under the chest** is H less the depth at the girth. The file
  carries the clearance's window only to check the result against.
- **Tail length** follows from where the hair's tips end
  (`tail_clearance`), which is the brief's controlling target.

The last block of the file is numbers the brief does not give: the angle of
the face, the head's depth, the neck's width seen from the front, `bone`
(the cannon's thickness; every other thickness of a leg is a multiple of
it, so it is the one number for how sturdy the legs are) and how far the
mane falls.

## From numbers to pieces

`HorseShape::base(height)` gives a `Frame`: the same dimensions as lengths
in world units, with the clearance worked out. `varied(height, roll)` picks
each dimension from its window instead.

`Frame::parts()` gives the pieces. A piece is drawn in the side view (x
toward the nose, y up from the ground under the middle of the body): two
`Section`s, each an edge from a top point to a bottom point with a width
across, and the piece is the solid between them. So a taper is one piece
whose ends differ, and no piece needs turning into place.

- **Trunk**: six sections, five pieces. Narrow at the chest, deepest just
  behind the forelegs, fullest through the ribs, a little in at the flank,
  broad at the hindquarters. The back dips between withers and croup and
  the belly rises toward the flank.
- **Neck**: four pieces along an arched crest from the root, just ahead of
  the withers, to the poll, which is the neck's length away at its
  carriage. Thick and broad at the base, light at the throat.
- **Head**: three pieces from the poll along a straight face, deepest at
  the jaw, tapering to a blunt muzzle. Two ears, each two pieces.
- **Legs**: level sections at the landmark heights. A foreleg is straight;
  a hind leg runs back from the stifle to the hock and down from there. The
  last piece of each is the hoof.
- **Tail**: a tapering dock.
- **Hair** (`Kind::Hair`): a ridge and a curtain of mane, a forelock, the
  tail's hair ending in three uneven clumps, and a skirt of feathering on
  each leg. Never part of a body piece.

Each piece names the joint it moves with: the trunk, the neck (neck, head,
ears, mane) or one of four legs. `Frame` says where each joint is.

## In the scene

`scene.rs` makes one flat-faced mesh per piece (`piece_mesh`), placed
relative to its joint, and shares the set between the three horses, which
differ only in shade. Hooves and hair are darker shares of the body's shade
(`HOOF_SHADE`, `HAIR_SHADE`). A horse is a `HorseBody` with the trunk's
pieces, a `Neck` joint and four `Leg` joints as children, which is what the
walk and the exhausted pose ([death-events.md](death-events.md)) move.

`--bare` starts with the hair hidden and F5 toggles it, to see the build
underneath.

## Tests

`cargo test`, in `src/sim/horse_shape.rs`:

- the file holds together: each base is inside its window, and the
  clearance's window is the depth's seen from the ground;
- the base horse at 165 cm is the brief's, to the centimetre it rounds to;
- for any height and any choice inside the windows, depth and clearance
  make the height, the landmarks keep their order, the trunk runs from the
  clearance to the withers, every leg stands on a hoof of the stated
  height, the tail's hair ends at its clearance and the poll is where the
  neck's length and carriage put it;
- a horse at another height is the same horse, every point scaled;
- the hair is separate from the body.

## Not built

- Variation between horses inside the windows (#63).
- Sockets for gear at the landmarks (#62).
- Coat colour; the shade is a placeholder.
- Bending joints: a leg swings from one joint as a whole, and the neck
  turns about its base.
- Ears that turn, a mane or tail that moves.
