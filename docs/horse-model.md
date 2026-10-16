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

A hit moves both sides, as placeholders until bodies move as their own
(#365, #15); every number is in `assets/data/attacks.ron`. An ordinary
hit, horse's or enemy's, is wound up ahead of it: the body leans back from
`wind_up` seconds before the hit lands, swings in from `release`, is
furthest forward (`lunge`) on the step the hit lands, and settles over
`recovery`. The sim keeps deciding when; it only tells the presentation
ahead of time (`Expedition::swing_in`, which counts steps as `tick` does,
less the clock's `accumulator`), and a test holds that every hit lands on
the step it said. Each attacker has its own swing: "horse" for every
horse, each enemy by its id. An ability's hit is not wound up yet (#18): it
tips in and out over `recovery`. A wind-up whose hit never comes (the
fight ended) goes back over a recovery rather than snapping. Whoever takes
a hit tips back and steps back (`flinch`), times its kind's `recoil`, and
flashes toward white ([hit-reactions.md](hit-reactions.md)). The sim's `Hit` says who dealt
it (`by`) as well as who took it; the `Motion` resource keeps each horse's
and each enemy place's pose, and an enemy leans on its base.

`--bare` starts with the hair hidden and F5 toggles it, to see the build
underneath.

## Worn gear

A piece a horse wears is seen on it, at its simplest (#359), painted with
its look (#361, item-card.md "The picture"): its rarity's colour, its
pattern and its decoration, laid over the slot's pieces as seen from the
side (`side_uvs`), the decoration at `worn_decoration`. `Frame::worn(slot)` gives its pieces, of
`Kind::Gear`, built from the horse's own sections so it fits any horse:

- **Head**: a band round the head behind the eyes, on the neck's joint.
- **Body**: a blanket over the back from behind the withers to over the
  hip, hanging 60% of the way down the trunk, through each of the trunk's
  sections between so it never cuts into it.
- **Saddle**: a block on the back, raised above it with flaps down the
  sides, wider than the blanket so it lies over it.

Each stands off what it covers (`swell`). `scene.rs` makes every horse each
slot's pieces at launch, hidden, and `show_worn` shows them each frame the
horse wears something there, in a material of that piece's look, made
once per look and tier, so Equip,
a swap, Undo and a drag are seen at once. A thin black rim round each (the
outline's trick, `WORN_RIM`) keeps a white Common piece readable on a pale
horse. It moves with its joint, so it walks, fights and lies down with the
horse. `--worn` starts every horse wearing a piece in each slot, rolled
from the seed. Real gear models (#66) replace all of it.

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
- the hair is separate from the body;
- the tail that comes out is inside its window, at the corners of the
  croup's, the dock's and the hair's clearance's windows and anywhere
  between;
- the pieces are the ones counted above, each on its joint, and a hoof is
  the last piece of a leg and of nothing else;
- the far side is the near side turned over: the legs of a pair and the two
  ears differ only in their side, and the trunk is on the middle;
- a leg is level sections at its landmarks from the top down, a foreleg
  straight as far as the fetlock, a hind leg back from the stifle to the
  hock and down from there;
- worn gear is never one of the horse's pieces, moves with what it covers,
  is wider and higher than it everywhere, and the saddle lies over the
  blanket.

## Not built

- Variation between horses inside the windows (#63).
- Sockets for gear at the landmarks (#62); worn gear today is placed from
  the trunk's and head's sections instead.
- Coat colour; the shade is a placeholder.
- Bending joints: a leg swings from one joint as a whole, and the neck
  turns about its base.
- Ears that turn, a mane or tail that moves.
