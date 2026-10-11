# The standing horse: joint positions, segment lengths and rest angles

The figures of the horse the game draws today, for anyone working out joint
angles for it. An angle from a gait study is an angle between two bones; the
rotation to give a joint in this model is the difference from the rest angle
here (`docs/philosophy/horse-movement.md`). **Every figure is worked out from
`assets/data/horse_shape.ron` by the formulas of `src/sim/horse_shape.rs`;
none is measured from a horse.** Made by `target/hh_rest_pose.py` on
2026-10-11; if the shape data changes, these are stale.

## Conventions

- Side view. **x** is along the horse, positive toward the head, zero at the
  middle of the body's length. **y** is up, zero at the ground. **z** is
  across, positive toward the camera (the near side).
- Lengths are ratios of **H**, the height at the withers. Centimetres are at
  the reference H of 165 cm. In the game H is 1.6 world units.
- A segment's **lean** is its angle from straight down, positive when its
  lower end is toward the head.
- An **included angle** is the angle at a joint between the two segments
  meeting there, 180 degrees being straight. For a fetlock the **directed**
  angle of the gait studies (over 180 when the fetlock is sunk) is 360 minus
  the included angle here.

## What the model has, and has not

- **A leg is one rigid piece today.** It has the shape of a standing leg and
  swings whole from one pivot. No joint in it bends yet (#257).
- **The foreleg is a straight vertical column** from above the elbow to the
  fetlock: elbow, knee and fetlock are in one line.
- **There is no shoulder blade, no upper arm (humerus), no point of
  shoulder, no pelvis, no hip joint and no thigh bone.** The leg's top
  section is sunk into the trunk. So there is **no rest angle for the
  shoulder, the elbow or the hip**, and the "thigh" below is only the line
  from the leg's top section to the stifle.
- **Pastern and coffin are one segment**, fetlock to coronet. The hoof is a
  block from the coronet to the ground.
- **The neck is one rigid piece** turning about one pivot at its base, with
  the head fixed to it. The tail does not move.
- Left and right legs are the same shape, mirrored in z.

## Joint positions, standing

| Point | x / H | y / H | x cm | y cm |
| --- | ---: | ---: | ---: | ---: |
| fore top of leg | +0.3780 | 0.6800 | +62.4 | 112.2 |
| elbow | +0.3780 | 0.6000 | +62.4 | 99.0 |
| knee (carpus) | +0.3780 | 0.3060 | +62.4 | 50.5 |
| fore fetlock | +0.3780 | 0.1210 | +62.4 | 20.0 |
| fore hoof top (coronet) | +0.3980 | 0.0670 | +65.7 | 11.1 |
| fore hoof on the ground | +0.3980 | 0.0000 | +65.7 | 0.0 |
| hind top of leg | -0.3424 | 0.7000 | -56.5 | 115.5 |
| stifle | -0.3024 | 0.5700 | -49.9 | 94.1 |
| hock | -0.4856 | 0.3640 | -80.1 | 60.1 |
| hind fetlock | -0.4776 | 0.1210 | -78.8 | 20.0 |
| hind hoof top (coronet) | -0.4586 | 0.0670 | -75.7 | 11.1 |
| hind hoof on the ground | -0.4586 | 0.0000 | -75.7 | 0.0 |
| fore swing pivot (today) | +0.3780 | 0.6500 | +62.4 | 107.2 |
| hind swing pivot (today) | -0.3224 | 0.6500 | -53.2 | 107.2 |
| neck root (top, ahead of withers) | +0.3402 | 0.9850 | +56.1 | 162.5 |
| poll | +0.6809 | 1.3910 | +112.3 | 229.5 |
| neck pivot (today) | +0.4494 | 0.8934 | +74.1 | 147.4 |
| tail root | -0.5300 | 0.9000 | -87.5 | 148.5 |
| end of dock | -0.5800 | 0.6550 | -95.7 | 108.1 |
| croup (highest point) | -0.3618 | 0.9900 | -59.7 | 163.3 |

Across the horse: a foreleg's centre line is 0.0778 H (12.8 cm) from
the middle, a hind leg's 0.1062 H (17.5 cm). Chest 43 cm wide,
barrel 59 cm, hindquarters 56 cm. Body length 178 cm,
depth at the girth 76 cm, clearance under the chest 89 cm.

## Segment lengths and leans, standing

| Segment | From | To | Length / H | cm | Lean, degrees |
| --- | --- | --- | ---: | ---: | ---: |
| Forearm | elbow | knee (carpus) | 0.2940 | 48.5 | +0.0 |
| Fore cannon | knee (carpus) | fore fetlock | 0.1850 | 30.5 | +0.0 |
| Fore pastern | fore fetlock | fore hoof top (coronet) | 0.0576 | 9.5 | +20.3 |
| Thigh (stand-in, see above) | hind top of leg | stifle | 0.1360 | 22.4 | +17.1 |
| Gaskin | stifle | hock | 0.2757 | 45.5 | -41.6 |
| Hind cannon | hock | hind fetlock | 0.2431 | 40.1 | +1.9 |
| Hind pastern | hind fetlock | hind hoof top (coronet) | 0.0572 | 9.4 | +19.4 |

Hoof height, coronet to ground: 0.067 H (11.1 cm), fore and hind.
Foreleg along its segments, elbow to ground: 0.604 H (100 cm).
Hind leg along its segments, stifle to ground: 0.643 H (106 cm).
Neck, root to poll: 0.53 H (87 cm), carried 50 degrees above level.
Withers: 1.0000 H (165.0 cm) high, by definition.
Head, poll to muzzle: 0.36 H (59 cm), the face 47 degrees below level.
Dock: 0.25 H (41 cm), 11.5 degrees behind straight down.

## Rest angles, standing

| Joint | Between | Included angle | Directed (fetlocks) |
| --- | --- | ---: | ---: |
| Shoulder | not in the model | none | |
| Elbow | not in the model | none | |
| Knee (carpus) | elbow, fore fetlock | 180.0 |  |
| Fore fetlock | knee (carpus), fore hoof top (coronet) | 159.7 | 200.3 |
| Hip | not in the model | none | |
| Stifle (thigh is a stand-in) | hind top of leg, hock | 121.2 |  |
| Hock | stifle, hind fetlock | 136.5 |  |
| Hind fetlock | hock, hind hoof top (coronet) | 162.5 | 197.5 |

## How it moves today

- The horse is drawn in place and the ground scrolls under it at 2.5 world
  units a second: **1.5625 H a second, 2.58 m/s at the reference height**. That
  is the one speed there is; the walk between encounters and the walk back
  to town both use it. A hoof that is planted must move back at this speed,
  so **stride length = speed x stride duration**:
  a stride of 1.10 s covers 1.719 H (2.84 m).
  a stride of 0.70 s covers 1.094 H (1.80 m).
  a stride of 0.60 s covers 0.938 H (1.55 m).
  a stride of 0.50 s covers 0.781 H (1.29 m).
- Each leg swings whole about its pivot through plus and minus 0.45 radians
  (25.8 degrees), as a sine wave of 0.898 s (7 radians a second),
  diagonal pairs together. An exhausted horse swings half as far.
- The camera looks across the trail from about 35 degrees above level, with
  no perspective. Sideways movement of a hoof shows as a small rise or fall
  on screen.

## What to ask for, given the above

- Angles as **included angles between segments** at named events of stance
  and swing (contact, loading, mid-stance, hoof-off, deepest fold, reaching),
  with fetlocks stated as included or directed, not as rotations: the
  rotation is taken from the rest angles here.
- For the shoulder, elbow and hip there is nothing here to take a difference
  from. Either a rest angle is proposed with them, with the position of the
  point of shoulder and the hip joint as ratios of H, or the forelimb above
  the elbow and the hindlimb above the stifle are given as the lean of the
  segment.
- Each figure marked as measured (with its study), fitted to footage, or an
  animator's setting.
