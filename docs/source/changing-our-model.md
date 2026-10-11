I would change it in two stages: first build an **anatomical** joint hierarchy, then reshape and bind the visible mesh to that hierarchy. Cutting the present rigid legs into pieces will add bending, but it will not fix the missing upper limbs, the false hind “thigh,” or the hoof pivots.

For your game, you do not need a veterinary-grade skeleton. You need the correct chains, credible proportions, correctly placed joint centers, and enough articulation to reproduce support, folding, and hoof placement.

## 1. Replace the whole-leg pivots with these chains

### Forelimb

```text
thorax
└── scapular_motion_control
    └── shoulder_joint
        └── elbow_joint
            └── carpus_joint
                └── fetlock_joint
                    └── pastern_joint
                        └── coffin_joint
                            └── hoof_mesh
```

Anatomically:

| Between joints | Anatomical segment |
|---|---|
| Scapular control → shoulder | Scapular representation |
| Shoulder → elbow | Humerus |
| Elbow → carpus | Radius/ulna, represented as one forearm segment |
| Carpus → fetlock | Metacarpal/cannon segment |
| Fetlock → pastern joint | Long pastern, P1 |
| Pastern joint → coffin joint | Short pastern, P2 |
| Coffin joint → hoof attachment | Coffin/pedal bone, P3, inside the hoof |

That is the actual sequence of the forelimb. The present “fore top of leg” should not be treated as a shoulder joint: the shoulder is where the scapula meets the humerus, and the elbow is where the humerus meets the forearm. [en.wikipedia](https://en.wikipedia.org/wiki/Equine_forelimb_anatomy)

The scapular control is an animation abstraction, not an extra anatomical ball joint. The horse’s forelimb is attached to the trunk through a muscular sling rather than a clavicle-and-socket connection to the chest. Allowing scapular movement is therefore more appropriate than bolting the shoulder to one fixed trunk point. [veteriankey](https://veteriankey.com/the-horse/)

### Hindlimb

```text
lumbar_body_control
└── pelvis_control
    └── hip_joint
        └── stifle_joint
            └── hock_joint
                └── fetlock_joint
                    └── pastern_joint
                        └── coffin_joint
                            └── hoof_mesh
```

| Between joints | Anatomical segment |
|---|---|
| Hip → stifle | Femur |
| Stifle → hock | Tibia/gaskin |
| Hock → fetlock | Metatarsal/cannon segment |
| Fetlock → pastern joint | Long pastern, P1 |
| Pastern joint → coffin joint | Short pastern, P2 |
| Coffin joint → hoof attachment | Coffin/pedal bone, P3 |

The hip joint is internal to the hindquarter: the femoral head articulates with the acetabulum in the pelvis. It is not the visible “point of hip,” the croup, or the current hindleg swing pivot. [veteriankey](https://veteriankey.com/the-horse/)

## 2. Make these specific changes to your existing geometry

### Forelimb: add the missing folded upper chain

Your current forearm and cannon are approximately vertical. You can retain that basic standing arrangement.

What must change is the part above the elbow:

1. Add a scapular representation descending forward from the upper thorax.
2. Add a shoulder joint toward the front of the chest.
3. Add a humerus descending backward from shoulder to elbow.
4. Keep the elbow behind the shoulder.
5. Connect the elbow to the existing lower forelimb.

This creates a folded upper chain rather than a vertical pole penetrating the chest. The humerus lies between shoulder and elbow and slopes downward and backward in the standing horse. [veteriankey](https://veteriankey.com/the-horse/)

### Hindlimb: replace the short stand-in thigh

Your current “thigh” is only 0.136 H because its upper endpoint is an arbitrary leg attachment near the stifle. That length should not carry over as the femur.

Instead:

1. Add a pelvis control inside the hindquarters.
2. Place an internal hip center higher and farther back than the stifle.
3. Connect hip to stifle with a forward-descending femur.
4. Connect stifle to hock with a backward-descending tibia.
5. Connect hock to fetlock with the cannon.

Keep the zigzag:

```text
hip
   \
    stifle
   /
hock
  |
fetlock
   \
    hoof
```

The exact slope depends on conformation, but the femur and tibia must be separate articulated segments. [open.lib.umn](https://open.lib.umn.edu/largeanimalanatomy/chapter/pelvic-limb/)

### Lower limbs: reconsider the unusually long hind cannon

In your current model:

- Fore cannon: 0.185 H.
- Hind cannon: 0.243 H.
- Hind hock height: 0.364 H.
- Fore carpus height: 0.306 H.

Those are your model’s values, not proof of an anatomical error. However, I would revisit the elevated hock and long hind cannon when fitting a reference skeleton. Splitting the existing hindleg without changing those landmarks will preserve its present silhouette and proportions.

## 3. A concrete first-pass skeleton in your coordinate system

Here is a proposed layout that preserves many existing visible landmarks while adding real upper chains and bringing the carpus and hock to the same initial height.

Every new or altered coordinate below is an animator’s setting. It is not measured equine anatomy and should be fitted to a side-view skeleton/reference horse before being called anatomically calibrated.

Coordinates are \((x/H,y/H)\), with your existing convention: positive \(x\) toward the head.

### Proposed forelimb

| Point | \(x/H\) | \(y/H\) | Change |
|---|---:|---:|---|
| Scapular upper reference/control | +0.2500 | 0.9800 | New internal reference |
| Shoulder joint center | +0.5200 | 0.8000 | New |
| Elbow joint center | +0.3780 | 0.6000 | Retain your current landmark |
| Carpus joint center | +0.3780 | 0.3200 | Raise from 0.3060 |
| Fore fetlock | +0.3780 | 0.1210 | Retain |
| Coronet reference | +0.3980 | 0.0670 | Retain as a surface landmark |
| Hoof ground reference | +0.3980 | 0.0000 | Retain as a contact reference |

The scapular upper reference is not a literal joint at the top of the scapula. Its purpose is to define scapular orientation and movement.

### Proposed hindlimb

| Point | \(x/H\) | \(y/H\) | Change |
|---|---:|---:|---|
| Hip joint center | −0.4700 | 0.8200 | New internal joint |
| Stifle joint center | −0.3024 | 0.5700 | Retain |
| Hock joint center | −0.4856 | 0.3200 | Lower from 0.3640 |
| Hind fetlock | −0.4776 | 0.1210 | Retain |
| Coronet reference | −0.4586 | 0.0670 | Retain as a surface landmark |
| Hoof ground reference | −0.4586 | 0.0000 | Retain as a contact reference |

This is a candidate engineering layout, not a breed-average skeleton. Its advantage is that you can implement it without moving every existing landmark at once.

### Resulting segment lengths

Calculated from those proposed coordinates:

| Segment | Length/H | At H = 165 cm |
|---|---:|---:|
| Scapular reference → shoulder | 0.3245 | 53.5 cm |
| Humerus | 0.2453 | 40.5 cm |
| Forearm | 0.2800 | 46.2 cm |
| Fore cannon | 0.1990 | 32.8 cm |
| Existing fore fetlock → coronet line | 0.0576 | 9.5 cm |
| Femur | 0.3010 | 49.7 cm |
| Tibia/gaskin | 0.3099 | 51.1 cm |
| Hind cannon | 0.1992 | 32.9 cm |
| Existing hind fetlock → coronet line | 0.0572 | 9.4 cm |

These are joint-center/reference-line distances calculated for the proposed rig, not measurements of physical bones.

### Resulting rest angles

| Joint | Proposed included rest angle |
|---|---:|
| Shoulder, using scapular reference | 88.3° |
| Elbow | 144.6° |
| Carpus | 180.0° |
| Fore fetlock, using the existing coronet line | 159.7° |
| Stifle | 109.9° |
| Hock | 141.5° |
| Hind fetlock, using the existing coronet line | 162.9° |

These values follow mathematically from the proposed coordinates.

The shoulder angle in particular is a rig-reference angle. Do not compare it with a study’s “shoulder angle” until you verify that the study uses equivalent scapular and humeral axes. Likewise, a fetlock angle measured to the coronet is only an approximation to one measured along the first phalanx.

## 4. Separate surface landmarks from joint centers

This is essential for your data model.

A visible landmark and a rotation pivot are not always the same thing:

| Surface/reference point | Why it should not automatically become the pivot |
|---|---|
| Point of shoulder | External landmark near, but not identical to, the shoulder rotation center |
| Point of elbow | Olecranon prominence, not necessarily the elbow’s rotational center |
| Point of hock | Calcaneal prominence, not the principal flexion center |
| Point of hip | Pelvic prominence, not the hip socket |
| Coronet | Hoof surface boundary, not the coffin-joint center |
| Ground point | Contact reference, not an anatomical joint |

The distal limb contains separate fetlock, pastern, and coffin articulations. The coffin joint is between P2 and P3; it should not simply be placed at the center of the coronet because that is a convenient mesh boundary. [en.wikipedia](https://en.wikipedia.org/wiki/Equine_forelimb_anatomy)

I would introduce separate data categories:

```text
joint_centers
surface_landmarks
segment_axes
hoof_contact_markers
mesh_attachment_points
```

That prevents a later mesh change from silently changing your skeletal anatomy.

## 5. Upgrade the hoof and pastern assembly

### Minimum workable version

For the first implementation:

```text
cannon
└── fetlock pivot
    └── combined pastern segment
        └── hoof-orientation pivot
            └── rigid hoof
```

This is a useful simplification, but label the hoof-orientation pivot as a rig approximation—not necessarily a correctly positioned anatomical coffin joint.

It provides:

- Fetlock yielding under load.
- Hoof clearance during swing.
- Independent hoof orientation.
- Toe/heel contact behavior.

### More anatomical version

Use:

```text
cannon
└── fetlock
    └── P1
        └── pastern joint
            └── P2
                └── coffin joint
                    └── P3 / hoof
```

The lower limb actually contains P1, P2, and P3, with the coffin bone enclosed within the hoof. [en.wikipedia](https://en.wikipedia.org/wiki/Equine_forelimb_anatomy)

Do not divide your existing 9.5 cm fetlock-to-coronet line into arbitrary equal thirds and call those bone lengths. Fit the internal joints to a reference diagram or radiograph; your surface line does not uniquely determine them.

### Add contact markers to the hoof

At minimum:

- Toe contact.
- Heel contact.
- Central sole reference.

Breakover should be implemented as a contact behavior: the heel rises and the hoof rolls toward the toe before release. A toe-roll control can temporarily act as a rig pivot without pretending the toe is an anatomical joint.

Keep the hoof mesh rigid. Deform the soft tissue above it, not the hoof wall as though it were a rubber foot.

## 6. Use different motion rules for different joints

A game rig should not give every joint unrestricted three-axis motion.

| Region | Suggested game-rig behavior |
|---|---|
| Scapular control | Sagittal translation/rotation; limited additional movement |
| Shoulder | Primarily flexion/extension, with controlled lateral freedom |
| Elbow | Primarily hinge-like |
| Carpus | Primarily hinge-like; small secondary motion only if needed |
| Fetlock | Primary flexion/extension with load-dependent yielding |
| Pastern/coffin | Smaller, controlled articulation |
| Pelvis | Translation and pitch; limited roll/yaw |
| Hip | Flexion/extension plus controlled abduction/adduction |
| Stifle | Primarily hinge-like for the game |
| Hock | Primarily hinge-like, coordinated with stifle |

These are proposed game abstractions, not exact descriptions of all anatomical degrees of freedom. The hip permits both sagittal and lateral movement, while the stifle and hock are mechanically coordinated by the reciprocal apparatus. [physio-pedia](https://www.physio-pedia.com/Anatomy_of_the_Equine_Hind_Limb)

### Couple stifle and hock—but not with identical rotations

The horse’s reciprocal apparatus coordinates their flexion and extension. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC1571089/)

For animation, use a fitted relationship such as:

\[
q_{\mathrm{hock}}
=
g(q_{\mathrm{stifle}},\text{gait phase}),
\]

rather than:

\[
q_{\mathrm{hock}}=q_{\mathrm{stifle}}.
\]

Anatomical coupling does not mean equal local angles. The bones have different orientations and lever geometry.

This is especially useful in inverse kinematics: it prevents the solver from finding technically reachable but implausible folded-hindlimb poses.

## 7. Reshape the mesh around the skeleton

### Upper limbs should blend into the body

The new humerus, scapular region, femur, and pelvis should mostly remain inside the body silhouette. Do not draw them as thin exposed rods with obvious mechanical hinges.

For a low-poly mesh:

- Add a shoulder mass that blends into the chest.
- Let the upper arm connect smoothly to the elbow region.
- Add a thigh mass blending into the hindquarter.
- Keep the stifle recognizable without making it a protruding human-style knee.
- Taper the lower limb toward the cannon.
- Preserve distinct fetlock and hoof silhouettes.

The internal skeleton and the visible outer shape are related, but they are not interchangeable.

### Add enough geometry around bending regions

For a skinned low-poly model, a practical starting choice is:

- One loop centered near the joint.
- One loop above it.
- One loop below it.

That is an animator’s topology recommendation, not an anatomical requirement.

Use additional geometry where silhouette changes matter—particularly elbow, carpus, stifle, hock, and fetlock. Avoid spending most of your polygons on straight cannon sections.

If your renderer uses rigid pieces rather than skinning, use overlapping or carefully covered joint transitions. Otherwise, deep folding will reveal gaps.

### Keep equipment attached to the appropriate controls

- Saddle and girth: thorax/body controls.
- Reins and bridle: head/neck system.
- Hoof mesh: distal limb.
- Feathering: lower-limb attachments plus optional secondary motion.
- Tail: pelvis/tail root, not a leg chain.

For Friesian-style feathering, validate the bare limb first. Hair can hide an incorrectly positioned fetlock or sliding hoof.

## 8. Change the shape data from boxes to a skeleton-plus-mesh definition

I would keep your RON-based workflow, but make joint positions explicit rather than deriving every articulation from the shape of a standing rigid leg.

Conceptually:

```text
horse_shape
├── reference_scale
├── skeleton_rest_pose
│   ├── thorax
│   ├── pelvis
│   ├── fore_left_chain
│   ├── fore_right_chain
│   ├── hind_left_chain
│   └── hind_right_chain
├── surface_landmarks
├── mesh_dimensions
├── skinning_or_part_bindings
└── hoof_contact_markers
```

Store either:

- Joint positions, with lengths and rest angles derived from them; or
- Segment lengths and rest orientations, with positions derived through forward kinematics.

Do not store both as independently editable authoritative values. They will eventually disagree.

For your existing system, explicit rest-pose joint positions are probably the easiest migration path. Your reporting script can then calculate:

- Segment lengths.
- Signed segment leans.
- Included rest angles.
- Directed fetlock angles.
- Left/right symmetry.
- Ground clearance.

Give the anatomy/rest-pose data a version or hash so gait clips can identify the geometry against which they were fitted.

## 9. Add a solver, not just more sine waves

Articulating the mesh is only half the work.

During stance:

\[
\mathbf{x}_{\mathrm{hoof}}(t)
=
\mathbf{x}_{\mathrm{contact}}
-
\mathbf{v}_{\mathrm{ground}}
(t-t_{\mathrm{contact}})
\]

in your in-place horse coordinate frame.

Then solve the limb to that target while preserving segment lengths and preferred bend directions.

During swing:

- Animate a hoof trajectory.
- Provide a preferred folding pose.
- Unfold toward the next contact.
- Orient the hoof independently.

A horse limb has more useful articulations than a simple two-bone human arm solver. Use either:

1. A multi-joint solver with preferred poses and coupling constraints; or
2. A main upper/lower-chain solve combined with separately controlled fetlock and hoof orientation.

Make reach failure explicit. If the contact target exceeds available reach, adjust body position, speed, stance duration, or contact placement—not bone lengths.

## 10. Recommended implementation order

For your present horse, I would prioritize:

1. Add real humerus and femur chains.
2. Add elbow, carpus, stifle, hock, and fetlock articulation.
3. Add independently orientable hooves.
4. Add a pelvis control and scapular movement controls.
5. Rebind or split the mesh to match.
6. Implement planted-hoof constraints.
7. Validate a slow walk before fitting faster gaits.
8. Add separate pastern and coffin articulation if close-up quality requires it.
9. Add neck/head articulation and tail movement.
10. Fit breed-specific carriage and gait styles.

The first usable milestone is a horse that can shift its body over a planted hoof, fold a limb without opening mesh gaps, and return that hoof to the next contact without changing segment lengths. Once that works, your anatomical angle tables become useful controls rather than numbers the model cannot physically express.