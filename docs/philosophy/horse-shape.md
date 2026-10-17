# The shape of the horse

The base horse: what it is, its proportions, and which features carry the
likeness. This is the specification to build the horse model to, received
from the owner as a brief for whoever (or whatever) makes the mesh.

*2026-10-17: the hock is at 0.32 H now, not 57-63 cm (owner, #278;
`vision.md`, The hock).*

**The base is a mature, modern sport-type purebred Friesian:** tall and
relatively long-legged, with a rectangular body, a long, high-set, arched
neck, a refined head, and limbs that are substantial without being
draft-horse heavy.

> An elegant, substantial Friesian sport horse: not a heavy draft horse and
> not a lightweight warmblood with Friesian hair.

## What kind of numbers these are

Breed standards describe shape and balance. They do not give centimetres for
every part. **The dimensions here are a chosen modelling specification, not
measured breed averages**, and the ranges are windows for adjusting the
model, not population statistics. They exist to give one consistent starting
shape.

## Principles

### Everything is a ratio of one height

Let **H** be the height from the ground to the top of the withers. The
reference horse has H = 165 cm (about 16.1 hands), a chosen scale within the
breed's range. Every measurement is given relative to H so the horse can be
resized.

Measured in a relaxed standing pose: all four hooves down; the neck naturally
raised, neither stretched forward nor in an exaggerated frame; the head
neither tucked nor thrown up. **Hair is excluded from anatomical measurements
unless stated.**

### Main dimensions

| Feature | Target at H = 165 cm | Ratio | Range | Measured |
| --- | ---: | ---: | ---: | --- |
| Head length | 59 cm | 0.36 H | 56–62 | Poll to front of muzzle, without ears |
| Head width | 23 cm | 0.14 H | 21–25 | Widest across cheek and jaw |
| Muzzle width | 15 cm | 0.09 H | 14–17 | Across the muzzle, nostrils not flared |
| Ear length | 15 cm | 0.09 H | 13–16 | Base to tip |
| Neck length | 88 cm | 0.53 H | 82–94 | Straight line, poll to the top of the neck's root just ahead of the withers |
| Neck carriage | 50° above horizontal | | 45–55° | Root-to-poll direction in the standing pose |
| Body length | 178 cm | 1.08 H | 173–183 | Point of shoulder to point of buttock |
| Barrel width | 60 cm | 0.36 H | 56–64 | Widest across the ribcage |
| Front chest width | 43 cm | 0.26 H | 39–47 | Across the front of the chest only |
| Hindquarter width | 56 cm | 0.34 H | 52–60 | Across the hindquarters |
| Body depth at girth | 76 cm | 0.46 H | 72–79 | Withers level down to the underside of the chest behind the forelegs |
| Clearance under chest | 89 cm | 0.54 H | 86–93 | Ground to that same underside |
| Tail dock | 42 cm | 0.25 H | 38–46 | Tail base to the end of the dock |
| Tail with hair | about 140 cm | 0.85 H | 130–150 | Root to the longest tips |

**Depth and clearance are linked:** 76 + 89 = 165. Never change one without
the other while H stays the same.

### Landmarks, in place of "leg length"

"Leg length" is ambiguous: the limbs do not start at the belly line, and the
hind legs are not straight columns. Use heights above the ground.

| Landmark | Height at H = 165 cm |
| --- | ---: |
| Top of withers | 165 cm |
| Top of croup | about 162–165 |
| Elbow | about 96–103 |
| Underside of chest behind the forelegs | 89 |
| Centre of the front knee | about 48–53 |
| Centre of the hock | about 57–63 (*replaced 2026-10-17: 0.32 H, 53 cm; `vision.md`, The hock*) |
| Centre of the fetlock | about 18–22 |
| Top of the hoof | about 10–12 |

Block these in, then check joint placement from the front and the side.

### What carries the likeness, in order

1. A high-set, long, arched neck.
2. A rectangular torso with substantial shoulder and hindquarter masses.
3. Long-looking, sturdy limbs.
4. A refined head with short ears.
5. A long mane and tail.
6. Distinct feathering on the lower legs.

**For a low-poly horse, spend detail in that order.**

### Head: refined, not tiny

- Moderately long, and looking small against the neck and body.
- A clear taper from cheek to muzzle (23 cm narrowing to about 15).
- A restrained jaw: readable in silhouette, not a square block.
- An almost straight profile, with at most a very shallow dish.
- A distinct muzzle, not a face that narrows to a point.
- **Do not** dish the face dramatically or shorten the muzzle.

### Ears: short and tapered

About a quarter of the head's length; 5 to 6 cm wide at the base; a slim
leaf shape; tips inclined slightly toward each other. **Separate geometry**,
so they can turn without changing the skull.

### Neck: the most important identifier

- Long, lightly arched, muscled along its upper half, emerging **high** from
  the chest and flowing into the withers.
- A tapered, curved volume: broad at the base, full through the middle,
  narrower under the head, convex along the crest, lighter through the
  throat.
- Thickness, seen from the side: 20–24 cm near the head, 30–36 cm in the
  middle, 44–50 cm at the base.
- **Two separate ideas:** *where* the neck attaches (high, blending into the
  withers; a breed trait) and the *angle* it is held at in this pose (about
  50°; a chosen setting).
- The root blends broadly into the shoulder. Not a thin tube stuck on the
  front of the torso, and not emerging nearly level from low on the chest.
- Not one uniformly thick banana-shaped cylinder.
- Do not confuse neck carriage with the shoulder, which is long and slopes
  at about 45 to 50°.
- **Keep neck bulk restrained on the base.** A heavier crest is a later
  variation, not something every horse has.

### Body: a slightly long rectangle

- Three roughly balanced masses along the side: forehand, middle,
  hindquarters. A guide, not three identical blocks.
- A substantial shoulder; a ribcage long enough not to look compact or
  pony-square; hindquarters with real length, not a small round rump.
- Body length is not back length.
- From above, a softly tapered oval: narrower at the chest, fuller through
  the ribs, narrowing a little at the flank, broad again at the
  hindquarters. **Roomy, not cylindrical.**
- The deepest point of the chest is just behind the forelegs; the belly
  rises moderately toward the flank; the loin is strong, not pinched.
- The torso must not hang so low that the legs look short.

### Legs

Muscular upper limbs; lower limbs noticeably narrower but not fragile; joints
with clear changes of direction and volume; forelegs visually straight when
standing; the angled structure of the hind legs kept. **Do not borrow very
fine legs from a lighter type.**

### Tail, mane and feathering

- The dock is a flexible taper from the root, not a rigid spike.
- **The controlling target for tail hair is clearance, not length:** the tips
  end 5 to 10 cm above the ground in the standing pose.
- For low-poly: a narrow upper tail, a long tapered curtain of hair below the
  dock, a few uneven clumps at the bottom and not a level edge. Not a short,
  thin tail, and not a tail permanently fanned up.
- **Mane, tail hair and feathering are separate geometry from the body**, so
  they never hide whether the anatomy underneath is right.

### A breed is a profile of ratios

*From the owner's document on breed mathematics, 2026-10-11. The figures
are art starting values, not measurements of any breed.*

- **"Build" means conformation**, not a combat loadout. A breed is a
  distribution of the same ratios of H that describe the base horse, with
  its own reference height and its own interval of heights.
- **A small breed is not a big breed scaled down.** A Shetland has its own
  ratios: less clearance, a longer body, a proportionally larger head.
- **The silhouettes differ where it reads**: a Quarter Horse in the width
  of chest and hindquarters, a Gypsy Vanner in depth and bone, a
  Thoroughbred in clearance and narrow volumes, a Friesian in a long raised
  neck. A Mustang varies more widely than the others.
- **Hair is an envelope, not a sphere.** How far the mane falls and how far
  the feathering stands off the lower leg are maximum controls; feathering
  tapers and follows where it attaches.
- **Shape never sets a combat number.** Any lean a breed might have is
  generated and balanced separately.

### Variation is correlated

Do not roll every dimension independently. A few bounded traits (size,
breadth and substance, relative leg length, neck expression, head
expression) each move several dimensions together, with a small residual on
each. Linked values stay linked. A value outside the breed's window is
truncated or the horse is rejected.

### Crossbreeds are bred, not blended

The owner's rule: a crossbred foal's build follows inherited genetics, not
an average of two breeds' profiles weighted by ancestry. Each parent has two
alleles at each body locus and gives one. The genotype maps to the traits
above, and the traits to dimensions, inside the anatomical constraints. A
breed's profile describes its founders; a foal is not rerolled into one
breed's window. The loci and the maps are a game abstraction, not a claim
about real genes.

### Joint centres, masses and a curve

Landmark heights differ by breed and are heights only; the joints are
generated together as one constrained skeleton, and segment lengths come
from it and never change in motion. The torso is overlapping shoulder,
ribcage and hindquarter masses, blended so they do not read as three balls.
The neck follows a curve from root to poll with a bounded arch and a taper
to the throat; its stated length is the straight line, not the length along
the curve.

### Check before accepting

Every generated horse: positive dimensions; depth plus clearance equal to
H; joint heights rising in order; the resting skeleton the same on both
sides; hooves that reach the ground; gear sockets placed from the horse's
own landmarks.

## For Horse Heroes

- **The breed profiles fit the data that exists.**
  `assets/data/horse_shape.ron` already holds each dimension as a base with
  a window, in ratios of H, with depth and clearance linked. The Friesian
  column of the owner's breed table agrees with that file's base values for
  the main dimensions (checked), so the Friesian is the first profile
  unchanged, and the other five are new records (#327).
- **The windows in that file become one breed's windows**, and #63's
  variation is rolled through the correlated traits (#328), not one
  dimension at a time.
- **Body genetics sit beside the coat genetics** (#332, #171), and both end
  in the same saved, resolved appearance.
- The work for breeds is under #266; what is still hers to answer is #267.

- **This is the first thing in the project that describes a specific
  horse.** Pillar 3 says primitive meshes until the loop is fun, and that
  nothing may depend on a specific model. Both still hold: these are
  proportions, and proportions can be met with boxes. The box horse in the
  slice can be rebuilt to these ratios now, long before any real mesh.
- **Ratios of H are generator parameters.** The owner's premise is that horse
  models are procedurally generated. The table above is that generator's
  default values, and the ranges are its first variation limits: a horse
  varies inside the windows and stays recognisably this kind of horse. Linked
  values, like depth and clearance, are constraints the generator must keep.
- **The landmarks are where the sockets go.** Halter at the head, barding
  over the body, boots between fetlock and knee or hock, shoes at the hoof,
  the breastcollar at the chest, the weapon at the muzzle, the braid in the
  mane or tail, the pet saddle on the back, saddlebags at the flanks
  ([gear.md](gear.md)). Heights given as ratios of H let the sockets follow a
  horse of any size.
- **Separate hair is also a gameplay need.** The braid slot hangs in the mane
  or tail; boots and leg wraps sit where the feathering is, so feathering
  must be able to give way to them without changing the leg; and a horse
  with a different mane still needs somewhere for a charm.
- **The likeness order is the readability order.** In a small view the neck,
  the rectangular body and the long legs are what will read
  ([fairyland-content.md](fairyland-content.md)). Gear and effects must not
  cover them ([cosmetics.md](cosmetics.md), silhouette protection).
- **Friesians are black, and this game has coat colour genetics.** The brief
  is about shape. Coat colour comes from the owner's genetics
  ([generations.md](generations.md)), so the base here should be read as a
  Friesian *build*, with colour decided separately. Worth confirming with her
  that this is the intent.
- "Mature" matters for descendants: foals and youngsters will need their own
  proportions, not this table scaled down.
- The brief includes a side-profile image as a general silhouette reference.
  It was not among the files received.

## Source

A base-model brief for the horse, received from the owner 2026-10-09. Its
breed descriptions come from the Friesian studbook's published breeding goal
and related breed-society pages; the centimetre values are the brief's own
modelling choices.

The sections on breeds, correlated variation, crossbreeds, the skeleton and
the acceptance check are from the owner's working design "procedural
abilities and breed mathematics", received 2026-10-11 and kept unedited as
`docs/source/procedural-abilities-and-breed-mathematics.md`, where the
tables of ratios, landmark heights and the deformation formulas are.
