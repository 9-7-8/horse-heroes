# How a horse moves: gaits

How to animate a horse so that a walk is a walk and a tölt is a tölt. The
owner handed over two documents of gait mathematics on 2026-10-11, to help
with the gaits (#248), and said: "We will later have different breeds of
horses in the game, which will dictate their colors and gaits."

The documents are kept whole, with their tables, equations and citations:
[`docs/source/horse-movement-animations.md`](../source/horse-movement-animations.md)
(walk, trot, canter, gallop, the limb cycle, head, neck and tail) and
[`docs/source/breed-specific-gaits.md`](../source/breed-specific-gaits.md)
(the ambling gaits of the gaited breeds). **Read the source before building
any of this**: this file is the rules to judge by, and the numbers are
there.

The same day she took the figures of the horse as built
([`docs/horse-rest-pose.md`](../horse-rest-pose.md)) back to where the
mathematics came from, and returned with two more documents, about this
model: [`docs/source/improving-our-model.md`](../source/improving-our-model.md)
(what the model can and cannot do today, and how gait figures should be
given for it) and
[`docs/source/changing-our-model.md`](../source/changing-our-model.md) (the
skeleton to build, with a first layout of joint positions).

## Principles

### Footfalls first, joints after

A gait is described by **when each hoof meets and leaves the ground**, and
everything else is built round those events. The description is a normalised
stride:

- a stride duration, and a phase that runs from 0 to 1 over it;
- for each leg, the phase at which it lands (its contact phase);
- for each leg, the share of the stride it spends on the ground (its duty
  factor). On the ground is stance; off it is swing.

Suspension, all four feet off the ground, is not set: it follows from the
contacts and the duty factors, and can be worked out and tested.

The order of work is fixed by this. Contacts and hoof-off; hooves locked to
the ground in stance; the trunk and pelvis; the leg chains; joint extremes
checked against measurements and footage; shoulder blade and pelvis; neck
and poll; a steady head; tail and hair; and last the seam of the loop, for
position and for speed.

### A planted hoof does not slide

In stance a hoof stays where it landed. Seen from a horse that is drawn in
place while the ground scrolls, that means the hoof moves back at exactly
the ground's speed. **Stride length is speed times stride duration**: change
one and another must follow, or the feet skate.

### Measured, proposed and unknown are three different things

The source keeps them apart and so must anything built from it.

- **Measured**: a figure from a study of particular horses. It is a check,
  not a limit, and not a breed average unless it says so.
- **Proposed**: a timing preset or a curve offered as a place to start
  blocking. It is an animator's setting.
- **Unknown**: there is no verified table of every joint's extremes for
  every gait. Gallop, the pastern and coffin joints, the neck joints and the
  tail have none. **Do not fill a gap with an invented degree value**, and
  do not carry one breed's walk and trot angles over to another gait and
  call them measured.

An angle in a study is an angle between two bones, not a rotation to give a
joint in a model. The rotation is the difference from that angle in the
model's rest pose. And a minimum and a maximum do not make an animation: two
gaits can reach the same extremes at different moments.

### A skeleton first, then the mesh round it

- **Cutting a rigid leg into pieces gives bending and nothing else.** A leg
  needs its real chain: shoulder blade, shoulder, elbow, knee, fetlock,
  hoof; pelvis, hip, stifle, hock, fetlock, hoof. The shoulder blade is not
  a socket on the trunk but a control that slides on it. The hip is inside
  the hindquarter, not at the point of hip or the croup.
- **A landmark on the surface is not a joint's centre.** The point of
  shoulder, of elbow, of hock and of hip, the coronet and the ground point
  are things to see or to stand on; the pivot is somewhere near and not the
  same. Keep joint centres, surface landmarks, hoof contact points and where
  the mesh attaches as separate data, so that changing the mesh cannot move
  the skeleton.
- **One authority for the rest pose.** Either joint positions, with lengths
  and angles worked out from them, or lengths and angles with positions
  worked out. Never both by hand: they will come to disagree. The rest pose
  carries a version, so a gait knows which skeleton it was fitted to.
- **Joints are not free in every direction.** Elbow, knee, stifle and hock
  are hinges for this purpose; shoulder and hip have a little sideways
  freedom; the fetlock yields under load. Stifle and hock move together,
  by a fitted relation and never by giving both the same angle.
- **The hoof is rigid.** It turns on its own pivot, lands and leaves by
  contact points at heel and toe, and rolls over the toe to leave. What
  gives is above it.
- **The upper limb is inside the body's outline**, as a shoulder mass and a
  thigh mass, not as rods with hinges showing. Where pieces are rigid, they
  overlap at the joints so that a deep fold opens no gap.

### What must hold, and what may give

A planted hoof, a segment's length, the direction a joint bends, the moments
of contact and release, and the ground not being entered **must hold**. A
study's joint extremes, the angles between them, how the upper limb leans,
the body's height and pitch, and how high a hoof clears **may give**. Ask
for every joint angle exactly and the hoof misses the ground; so in stance
the leg is solved to keep the hoof planted while staying as near its target
angles as it can, and in swing the fold and the hoof's path are set
directly. **When a hoof cannot reach, the body, the speed, the stance or the
landing place changes. Never a bone's length.**

### Speed and cadence are separate

Stride length is speed times stride duration, so at one ground speed a
quicker cadence is a shorter stride whether the gait wants one or not. A
gait has its own ground speed. Forcing every gait to cover ground at one
speed, or letting the feet slide for style, are the alternatives, and the
first is wrong for a gait known by its slow progress.

### Stride is the breed's, cadence follows

*From the owner's document on breed mathematics, 2026-10-11; the figures
are animation proposals, not measurements of these breeds.*

Give a breed a preferred stride length for each gait as a multiple of its
height, and work the stride's duration out from the speed. Cadence is then a
consequence, not another random number: at one speed a Shetland takes more
steps than a Friesian, with no skating and no extra action in a fight. A
horse varies a little round its breed's stride. Cadence envelopes are
controller settings, not speed limits. Pick the gait that keeps stride and
cadence inside their envelopes, with some hysteresis, so a small horse may
trot beside a tall one that still walks. Keep the phase continuous when the
cadence changes: accumulate it, do not recompute it from the clock.

### What a gait is worth

The owner's rule (2026-10-11): a breed's gaits affect travel speed and boost
pets, and there is no travel-efficiency stat. So a gait has two records that
must not be confused. Its **gameplay values**, a travel speed and a pet
boost, are data the sim reads. Its **animation settings**, contact phases,
duty factors and limb curves, are how that motion is shown. The party
travels at the arithmetic average of its horses' speeds. Gameplay never
counts hoofbeats or frames: no cooldown, award or boost is paid per
footfall. The drawn cadence must stay consistent with the motion and cannot
overrule the sim's progress.

### Joints do not all move together

- Stance and swing are each normalised on their own, and a joint's curve is
  written against them: its angle at contact, as load comes on, at
  mid-stance, at hoof-off, at its fold in swing, and reaching for the
  ground. That is a table of events, one for fore and one for hind, and a
  joint that folds deepest at its own moment gets its own event.
- **An angle is turned into a rotation by signed leans, not by subtracting
  included angles.** An included angle does not say which side of straight
  the joint is on. A segment's lean is taken from straight down, a joint's
  value is the lower segment's lean less the upper's, and the rotation is
  the change in that from rest. A fetlock's directed angle rises as its
  included angle falls: one convention to a curve.
- A figure is measured only if a study measured that very thing. A value at
  an event is not measured because it lies between a study's least and most.
- **No joint is one sine wave.** A fetlock sinks under load in stance and
  can fold twice in swing.
- The foreleg is not a pendulum on a fixed point: the shoulder blade slides
  on the trunk and gives much of the reach. The hind leg's reach includes
  the pelvis.
- Recovery has an order. In a gallop the knee folds first, then the
  shoulder, then the elbow.

### Each gait is its own mechanics

- **Walk** vaults over its supporting legs, like a pendulum upside down. It
  is not a slow trot.
- **Trot** is a spring: the body sinks in each diagonal support and rebounds
  into suspension, twice a stride.
- **Canter** is lopsided. It has a lead, and the four legs have four
  different jobs; one leg's curve shifted in time is wrong.
- **Gallop** is four separate contacts, hind pair then fore pair, with one
  gathered suspension. Not the legs flung out front and back.

### The ambling gaits are one family with one dial

The smooth gaits of the gaited breeds mostly share the walk's order of
footfalls. What differs is how close in time a hind foot and the fore foot
on the same side land. With that delay as a share of the stride:

- none: a pace, same-side legs together;
- a little: a stepping pace, same-side legs close;
- a quarter: four even beats, as in a rack, a tölt or a Paso Fino's gait;
- more: a fox trot or trocha, the fore foot landing just before the
  opposite hind;
- a half: a trot, diagonal legs together.

The dial alone does not make a gait smooth. The duty factors must also keep
a foot on the ground throughout, and whether they do is a sum that can be
checked. These are families for modelling, not hard lines between breeds:
real horses run along the dial with speed.

### Three layers, kept apart

1. **Timing family**: even four-beat, lateral couplets, diagonal couplets,
   or paired pace.
2. **Breed presentation**: collection, how high the legs lift, an outward
   sweep of the forelegs, overstep, how much the head nods, how the neck and
   tail are carried.
3. **The individual**: its own cadence, asymmetry and amplitude.

That is how two breeds can share the arithmetic of a four-beat gait and not
look like one animation on two meshes. Do not get a collected gait by
shrinking every rotation: short steps still fold the leg.

### The head rides, the neck works

The poll can travel a long way up and down while the head barely turns. So
the neck's base, its curve, the poll's position and the head's own turn are
separate controls, and a big bob is not a big nod of the skull. How much the
head moves is part of a breed's presentation: deep in a running walk, quiet
in a rack.

### The tail is not part of the gait

How the dock is carried is a pose that belongs to the breed and the horse.
On top of it: a small response to the pelvis, hair that lags, and swishing
as a separate behaviour. Never a regular swish locked to the stride.

## For Horse Heroes

- **Today a leg is one piece swinging from one joint**, diagonal pairs
  together, on a sine wave of the clock (`swing_legs` in
  `src/game/scene.rs`; `docs/horse-model.md` lists bending joints as not
  built). That is none of the above. The first work is a skeleton, legs
  that bend on it, and a gait that is data.
- **The swing as built cannot plant a foot, and the sums say so.** In one
  cycle of it (0.898 s) the ground travels 1.40 H; the whole sweep of a hoof
  is 0.565 H; a walk's stance alone needs 1.07 H. And swinging from a pivot
  of fixed height lifts the hoof at each end of the sweep, by up to 0.12 H
  behind. The source worked these out from `docs/horse-rest-pose.md` and
  they were checked here. No tuning of the sine wave fixes it.
- **The horse has no shoulder blade, upper arm, pelvis, hip or thigh bone**,
  and its "thigh" is the line from the top of the leg to the stifle. The
  source proposes a first layout of joint centres in this model's own
  coordinates. It is an animator's setting throughout, and **it moves the
  hock to 0.32 H, below the window the owner's brief gives it** (0.345 to
  0.382; `horse-shape.md`), and the knee to the top of its own. That is hers
  to rule on (#278). *She took the 0.32 H hock on 2026-10-17 (`vision.md`,
  The hock); the knee is still hers to rule on.* *She took the 0.32 H knee
  the same day.*
- **The horse is rigid pieces, not a skinned mesh** (`docs/horse-model.md`).
  So the source's advice for rigid pieces is the one that applies: pieces
  that overlap at the joints. `--bare` already hides the hair, which is how
  a fetlock is checked before feathering covers it.
- **One ground speed is the game's own limit**, not the horse's. At it, the
  quickest gait in the source would have to stride 0.58 H. A gait carries
  its speed (#283).
- The source lists reins and a bridle among things to attach. This game has
  neither: halters, and saddles that seat pets (`vision.md`).
- **The party is drawn in place and the trail scrolls** at one fixed speed
  (`SCROLL_SPEED`). So the planted-hoof rule is the form in which it applies
  here: the trail's speed, a gait's stride duration and its stride length
  have to agree, and a change of gait is a change of the trail's speed
  (#248).
- **The drawing is presentation; the gait is not only that.** *(Revised
  2026-10-11. This read "the sim decides how long a walk between two
  encounters takes; a gait must not change it".)* That still holds for the
  locomotion work of 0.5.1 (#283). Once breeds exist, a gait's travel speed
  is sim data and the party travels at the average (#334), and a gait
  boosts the pet with that horse (#335). Gait arithmetic is plain numbers
  with no rendering in it, and is unit-tested like the sim: which feet are
  down at a phase, when there is suspension, whether a gait keeps a foot on
  the ground.
- **Numbers are data** (pillar 4). A gait, a breed's presentation and a
  horse's own variation are three data records, matching the three layers.
  The source's presets are where their first values come from, each marked
  as proposed, not measured.
- **Nothing may depend on a specific model** (pillar 3). The gait drives
  joints the horse's frame names; a different mesh on the same frame moves
  the same way.
- **The base horse is a sport-type Friesian** (`horse-shape.md`). The source
  speaks to it directly: a high-set neck is the base pose with movement laid
  over it, and a quiet, low-hanging tail is right for the type, with the
  long hair doing most of the moving.
- **Breeds are the owner's, and later.** A breed decides a horse's colours
  and its gaits (`vision.md`). Coat colour already has a home in the
  genetics (#171), so how a breed and a genotype meet is an open question
  (#267). Whether a gait is only looks is settled: it is not.
- **Her document works with six breeds that walk, trot, canter and
  gallop** (Mustang, Friesian, Quarter Horse, Gypsy Vanner, Shetland,
  Thoroughbred), and says an optional gait variant can come later instead
  of giving a breed an amble to tell it apart. So the ambling family above,
  and #269 and #273 to #276, serve no starting breed; whether they move to
  a later release is asked on #267. Stride by breed is #333.
- **The source has footage as its last word**, for every gap. There is no
  footage in this project and no one to track it. Until there is, a gap
  stays an animator's setting, labelled as one, and the owner's eye is the
  check.
- **The work** is two epics: how a horse moves (#255, with #247 and #248
  under it) and breeds (#266). The questions only the owner can answer are
  on #267.
- **Where it stands against the pillars.** None of this makes a drop feel
  better (pillar 1), and menus come first (pillar 5). It earns its place
  because the horses walking are what is on screen most of the time, and
  the owner asked for it after watching them.

## Source

Four documents received from the owner on 2026-10-11, written as answers to
an animator. Two came first: one on walk, trot, canter and gallop and the
limb, head, neck and tail mechanics behind them, and one on the ambling
gaits of the gaited breeds. Two came after she sent back the figures of the
horse as built: one on what that model can express and how gait figures
should be given for it, and one on the skeleton to give it. All four end
complete and are kept unedited in `docs/source/`.

A fifth, her working design "procedural abilities and breed mathematics",
came the same day: stride length by breed, footfall presets for canter and
gallop, breed presentation figures, a swing path, and her rule on what a
gait is worth. It is
`docs/source/procedural-abilities-and-breed-mathematics.md`.
