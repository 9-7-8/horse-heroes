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

### Joints do not all move together

- Stance and swing are each normalised on their own, and a joint's curve is
  written against them: its angle at contact, as load comes on, at
  mid-stance, at hoof-off, at its fold in swing, and reaching for the
  ground.
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
  built). That is none of the above. The first work is legs that bend and a
  gait that is data.
- **The party is drawn in place and the trail scrolls** at one fixed speed
  (`SCROLL_SPEED`). So the planted-hoof rule is the form in which it applies
  here: the trail's speed, a gait's stride duration and its stride length
  have to agree, and a change of gait is a change of the trail's speed
  (#248).
- **This is presentation.** The sim decides how long a walk between two
  encounters takes; a gait must not change it. Gait arithmetic, though, is
  plain numbers with no rendering in it, and is unit-tested like the sim:
  which feet are down at a phase, when there is suspension, whether a gait
  keeps a foot on the ground.
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
  genetics (#171), so how a breed and a genotype meet is an open question,
  as is whether a gait is only looks.
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

Two documents received from the owner on 2026-10-11, written as answers to
an animator: one on walk, trot, canter and gallop and the limb, head, neck
and tail mechanics behind them, and one on the ambling gaits of the gaited
breeds. Both cite the studies they draw on, and both end complete. They are
kept unedited in `docs/source/`.
