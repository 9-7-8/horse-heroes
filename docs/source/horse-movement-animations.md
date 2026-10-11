For a 3D animator, the most useful description is a **phase-based** system: define when each hoof contacts and leaves the ground, then build joint rotations, body motion, and head–neck motion around those events.

There is an important scientific limitation: horses do not have one universal minimum and maximum joint angle for each gait. Angles depend on speed, breed, conformation, collection, surface, and measurement convention. Published studies give measured extrema for particular horses—not anatomical limits that every animation should reach. Even stride timing differs substantially between breeds. [scielo](https://www.scielo.br/j/cr/a/78nBthf7XrxbTZFn6jjWJVt/?format=pdf&lang=en)

Below, I separate measured research values from proposed animation settings. The timing presets and procedural equations are an implementation framework, not a claim that every horse follows those exact curves.

## 1. Establish the skeleton and angle conventions

### Required articulated regions

Use these chains:

| Region | Suggested articulated chain |
|---|---|
| Forelimb | Scapular control → shoulder → elbow → carpus → fetlock → pastern → coffin/hoof |
| Hindlimb | Pelvis → hip → stifle → hock → fetlock → pastern → coffin/hoof |
| Axial body | Thorax → lumbar region → lumbosacral/pelvic control |
| Head and neck | Neck base → several cervical controls → poll → head |
| Tail | Anatomical dock chain → separate hair chain or simulation |

Do not make the forelimb a pendulum attached to a fixed shoulder point. The scapula moves relative to the trunk, and the trunk is supported between the forelimbs by a muscular sling. Scapular movement contributes to reaching and retracting the limb. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)

Likewise, movement of the hindlimb relative to the ground includes pelvic and lumbosacral motion, not just hip flexion. That distinction becomes particularly important in canter and gallop. [veteriankey](https://veteriankey.com/performance-in-equestrian-sports/)

### Anatomical angles are not rig rotations

For an elbow, carpus, stifle, or hock, a study often reports the angle between adjacent segments:

- A smaller angle generally represents flexion.
- An angle near \(180^\circ\) represents an approximately straight chain.
- Fetlock measurements may use a directed angle exceeding \(180^\circ\) to describe extension under load. [scielo](https://www.scielo.br/j/cr/a/78nBthf7XrxbTZFn6jjWJVt/?format=pdf&lang=en)

For three joint landmarks \(A,B,C\), the unsigned included angle is:

\[
\alpha=
\cos^{-1}
\left[
\frac{(A-B)\cdot(C-B)}
{\|A-B\|\|C-B\|}
\right].
\]

This equation alone cannot distinguish a \(230^\circ\) directed fetlock angle from its \(130^\circ\) complement. For fetlocks, preserve a signed sagittal angle using an `atan2` convention and unwrap it consistently.

Convert a measured anatomical angle to a rig rotation using:

\[
q_j(t)=s_j\left[\alpha_j(t)-\alpha_{j,\mathrm{bind}}\right],
\]

where:

- \(q_j\) is the local animation rotation.
- \(\alpha_{j,\mathrm{bind}}\) is the anatomical angle in the bind pose.
- \(s_j\) is \(+1\) or \(-1\), depending on the local axis.

For example, an elbow changing from a bind-pose angle of \(150^\circ\) to \(105^\circ\) requires approximately \(45^\circ\) of flexion—not a local rotation of \(105^\circ\).

## 2. Cadence: describe the gait with a normalized stride

A stride is one complete cycle of a particular limb, from one contact to its next contact. Stance is its grounded interval; swing is its airborne interval. Whole-horse suspension means all four hooves are airborne simultaneously. These are different concepts. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)

Define:

\[
T=\text{stride duration},\qquad
f=\frac1T,\qquad
L=vT,
\]

\[
\phi(t)=\operatorname{frac}\left(\frac{t}{T}\right).
\]

For limb \(i\):

\[
p_i=\operatorname{frac}(\phi-\delta_i),
\]

where \(\delta_i\) is that limb’s contact phase.

With duty factor \(D_i\):

\[
\text{stance if }0\leq p_i<D_i;
\qquad
\text{swing if }D_i\leq p_i<1.
\]

Duty factor is the fraction of the stride spent grounded. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

### Practical timing presets

The following are proposed starting presets for a straight-line animation. They reproduce the established footfall relationships, but their exact phase offsets and duty factors are artist-selected.

Abbreviations: LH = left hind, RH = right hind, LF = left fore, RF = right fore.

| Gait | Suggested \(T\) | Strides/second | Contact phases | Suggested duty factor |
|---|---:|---:|---|---|
| Walk | 1.10 s | 0.91 | LH 0.00; LF 0.25; RH 0.50; RF 0.75 | All limbs 0.62 |
| Trot | 0.70 s | 1.43 | LH + RF 0.00; RH + LF 0.50 | All limbs 0.44 |
| Right-lead canter | 0.60 s | 1.67 | LH 0.00; RH + LF 0.30; RF 0.60 | All limbs 0.32 |
| Right-lead gallop | 0.50 s | 2.00 | LH 0.00; RH 0.15; LF 0.40; RF 0.55 | All limbs 0.25 |

The underlying sequences are:

- Walk: four-beat lateral sequence.
- Trot: alternating diagonal pairs.
- Right-lead canter: LH → RH + LF → RF.
- Right-lead transverse gallop: LH → RH → LF → RF. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)

Mirror left and right for left-lead canter and gallop.

The preset produces:

- Walk: no suspension.
- Trot: suspension from phase 0.44–0.50 and 0.94–1.00.
- Canter: suspension from 0.92–1.00.
- Gallop: suspension from 0.80–1.00.

These suspension intervals follow mathematically from the proposed contacts and duty factors; they are not measured averages.

### How these presets compare with observations

One study of in-hand horses reported median walk stride durations of 1.0–1.3 seconds and trot durations of 0.63–0.78 seconds, depending on breed. Forelimb duty factors were approximately 0.63–0.65 at walk and 0.46–0.49 at trot; hindlimb duty factors were generally lower. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

A separate study of ridden Criollo horses recorded mean stride durations of approximately 0.90 seconds at walk, 0.60 seconds at trot, and 0.51 seconds at canter. This illustrates why one fixed cadence is not a universal horse standard. [scielo](https://www.scielo.br/j/cr/a/78nBthf7XrxbTZFn6jjWJVt/?format=pdf&lang=en)

For production, keep \(T\), \(D_i\), contact offsets, and stride length editable independently.

## 3. Measured joint-angle minima and maxima

### Walk and trot: a useful measured baseline

The following values are rounded left-side mean extrema from a study of 35 ridden Criollo horses. They are sagittal, marker-based measurements. They are neither full 3D joint limits nor Friesian-specific values. Individual standard deviations were often several degrees and sometimes more than \(10^\circ\). [scielo](https://www.scielo.br/j/cr/a/78nBthf7XrxbTZFn6jjWJVt/?format=pdf&lang=en)

| Joint | Walk: minimum → maximum | Trot: minimum → maximum |
|---|---:|---:|
| Shoulder | 114° → 120° | 115° → 121° |
| Elbow | 106° → 152° | 104° → 150° |
| Carpus | 120° → 185° | 110° → 183° |
| Fore fetlock | 167° → 232° | 163° → 231° |
| Hip | 89° → 105° | 90° → 106° |
| Stifle | 135° → 160° | 125° → 151° |
| Hock | 129° → 165° | 116° → 161° |
| Hind fetlock | 150° → 224° | 148° → 230° |

All values in this table come from the study’s joint-extrema tables. [scielo](https://www.scielo.br/j/cr/a/78nBthf7XrxbTZFn6jjWJVt/?format=pdf&lang=en)

Two cautions matter:

1. Small measured shoulder-angle changes do not mean the entire forelimb barely swings. Scapular movement and segment orientation account for additional reach. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)
2. The Criollo paper contains internal inconsistencies: some separately reported midstance fetlock angles exceed the maxima in its joint-extrema tables, particularly at canter. Therefore, I would not use its canter extrema as unquestioned production targets. [scielo](https://www.scielo.br/j/cr/a/78nBthf7XrxbTZFn6jjWJVt/?format=pdf&lang=en)

Independent fetlock research also found different ranges: approximately \(58^\circ\) forelimb and \(60^\circ\) hindlimb at walk, increasing to approximately \(83^\circ\) and \(89^\circ\) at trot. Different horses and measurement methods can produce materially different results. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9502055/)

### Canter: measured lope data, with a significant limitation

A small study of four western-pleasure horses reported the following leading-limb extrema during lope:

| Joint | Reported maximum flexion | Reported maximum extension |
|---|---:|---:|
| Elbow | 105° | 153° |
| Carpus | 120° | 180° |
| Hip | 84° | Not supplied in the accessible abstract |
| Stifle | 130° | Not supplied in the accessible abstract |
| Hock | 128° | 157° |
| Fore fetlock | Not supplied in the accessible abstract | Approximately 228°–230° |
| Hind fetlock | Not supplied in the accessible abstract | 232° |

These are useful comparison values, but the authors classified the studied lope as a four-beat stepping gait. It is not a complete dataset for a normal three-beat working canter. [cambridge](https://www.cambridge.org/core/journals/equine-and-comparative-exercise-physiology/article/twodimensional-kinematics-of-the-jog-and-lope-of-the-stock-breed-western-pleasure-horse/974077D9536B4B86D1EA23B0B9B31739)

### Gallop: avoid inventing a complete angle table

The retrieved gallop research establishes limb coordination and the ordering of joint events, but it does not supply a verified, complete minimum–maximum table for every joint.

In particular, galloping forelimb recovery was reported in this sequence:

\[
\text{hoof-off}
\rightarrow
\text{maximum carpal flexion}
\rightarrow
\text{maximum shoulder flexion}
\rightarrow
\text{maximum elbow flexion}
\rightarrow
\text{shoulder extension and an initial carpal extension peak}
\rightarrow
\text{contact}.
\]

That sequence is valuable animation information: the joints do not all flex and extend simultaneously. [beva.onlinelibrary.wiley](https://beva.onlinelibrary.wiley.com/doi/abs/10.1111/j.2042-3306.1994.tb04869.x)

For a scientifically defensible gallop, use the correct contacts and this recovery sequence, then fit the actual extrema to calibrated side-view footage of the intended horse and speed. A table claiming exact universal gallop limits would be misleading.

### Pastern, coffin, cervical, and tail joints

The sources above do not provide a reliable full set of gait-specific extrema for:

- Proximal interphalangeal/pastern joints.
- Distal interphalangeal/coffin joints.
- Individual cervical joints.
- Individual tail vertebral joints.
- All three rotational axes of the limb joints.

Do not substitute passive veterinary range-of-motion limits for active gait extrema; passive joint measurements depend on testing conditions and are not equivalent to locomotor motion. [beva.onlinelibrary.wiley](https://beva.onlinelibrary.wiley.com/doi/full/10.1111/j.2042-3306.2010.00254.x)

For a simplified rig, these regions can be controlled procedurally, but their numeric settings should be identified as animation choices.

## 4. When each limb part moves

The most transferable approach is to normalize stance and swing separately:

\[
u=\frac{p_i}{D_i}
\quad\text{during stance},
\]

\[
w=\frac{p_i-D_i}{1-D_i}
\quad\text{during swing}.
\]

Thus, \(u=0\) is contact, \(u=1\) is hoof-off, \(w=0\) is hoof-off, and \(w=1\) is the next contact.

### Forelimb coordination

The following event relationships are particularly well described for trot. Walk uses the same broad support/recovery functions, but should not simply reuse the identical joint curves. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)

| Event | Scapula and shoulder | Elbow and carpus | Fetlock and hoof |
|---|---|---|---|
| Contact | Limb is protracted; shoulder relatively extended | Elbow relatively extended; carpus prepares to support load | Hoof meets ground; fetlock begins loading |
| Early stance | Shoulder flexes under loading; scapular retraction begins | Elbow yields; carpus straightens | Fetlock extends rapidly |
| Midstance | Trunk passes over grounded hoof | Support chain carries load | Fetlock approaches maximum loaded extension |
| Late stance | Limb retracts behind trunk | Elbow extends; carpus prepares for recovery | Fetlock unloads; heel rises into breakover |
| Early swing | Limb begins forward recovery | Carpus folds rapidly; elbow flexes | Hoof clears ground; fetlock flexes |
| Mid/late swing | Scapula advances limb | Folded limb unfolds | Hoof advances and orients for landing |
| Terminal swing | Limb reaches contact position | Carpus and elbow extend | Fetlock extends toward landing configuration |

These relationships, including early loading, midstance fetlock extension, and folding during swing, are supported by the gait-phase review. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)

For gallop, preserve the more specific carpus → shoulder → elbow flexion ordering described above rather than forcing all recovery peaks into one frame. [beva.onlinelibrary.wiley](https://beva.onlinelibrary.wiley.com/doi/abs/10.1111/j.2042-3306.1994.tb04869.x)

### Hindlimb coordination

| Event | Pelvis and hip | Stifle and hock | Fetlock and hoof |
|---|---|---|---|
| Contact | Hindlimb is advanced relative to pelvis | Relatively extended, then yield under loading | Hoof contacts; fetlock begins extension |
| Early stance | Hip begins extending | Stifle and hock flex under loading | Fetlock extends as load increases |
| Midstance | Trunk advances over limb | Support and propulsion overlap | Fetlock approaches maximum loaded extension |
| Late stance | Hip approaches maximum extension | Hock extends toward push-off | Heel rises; hoof completes breakover |
| Early swing | Hip begins forward recovery | Stifle and hock fold | Hoof clears ground; fetlock flexes |
| Mid swing | Thigh advances | Stifle and hock near strong swing flexion | Hoof passes beneath body |
| Terminal swing | Hip approaches maximum flexion | Stifle and hock unfold | Hoof lowers and prepares for contact |

Hip extension near late stance, hip flexion near terminal swing, and coordinated stifle–hock folding are documented in the gait-phase review. Fetlock swing curves can contain two flexion peaks rather than one. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)

### Suggested keyframe windows

These are proposed blocking windows—not measured universal event times:

| Control | Starting keyframe window |
|---|---|
| Loaded fetlock extension peak | \(u=0.40\)–0.60 |
| Hip maximum extension | \(u=0.85\)–1.00 |
| Forelimb folding | First half of swing |
| Stifle/hock swing flexion peak | \(w=0.35\)–0.55 |
| Hip maximum flexion | \(w=0.85\)–1.00 |
| Final limb unfolding | Last third of swing |
| Hoof breakover | Final portion of stance |

Use these to block the animation, then shift peaks against footage. They are intentionally broad enough to avoid false precision.

## 5. What makes each gait mechanically different?

### Walk: vaulting over support

Walk is predominantly an inverted-pendulum gait: the withers and pelvis rise as their supporting limbs pass through midstance, then lower toward the support transitions. It is not a slow-motion trot. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

Animate:

- Four distinct contacts.
- Overlapping support.
- Modest vertical thoracic motion.
- Alternating front and rear elevation.
- Separate recovery of each limb.

### Trot: compression and rebound

Trot behaves more like a spring–mass system. The body descends during diagonal support as the limbs yield, then rises toward push-off and suspension. Head, withers, and pelvis generally rise and fall together twice per stride. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

Animate:

- Opposite diagonal pairs.
- Two compression/rebound events per stride.
- Visible loaded fetlock extension.
- Limb folding during recovery rather than rigid straight-leg swinging.

Real diagonal contacts may be slightly dissociated rather than perfectly simultaneous. Such dissociation varies with horse and presentation. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

### Canter: asymmetric support and rocking

Canter has three principal beats and lead-dependent limb roles. The trailing hind contacts first, followed by the diagonal pair, then the leading forelimb. [odr.chalmers](https://odr.chalmers.se/bitstreams/9b21f471-f971-41c6-96a7-e5f67c17fddd/download)

Animate separate curves for:

- Leading forelimb.
- Trailing forelimb.
- Leading hindlimb.
- Trailing hindlimb.

Do not merely shift one generic leg animation in time. Research finds differences in lead/trail limb reach and marker trajectories. [scielo](https://www.scielo.br/j/cr/a/78nBthf7XrxbTZFn6jjWJVt/?format=pdf&lang=en)

### Gallop: four sequential contacts

The usual horse gallop is transverse: the canter’s middle diagonal contact separates into hind-first and fore-second contacts. At increasing speed, stance overlaps decrease and the four contacts become more distinct. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC2696142/)

Animate:

- Hind pair first, fore pair second.
- Distinct lead/trail recovery.
- Shorter support intervals.
- Hindlimb gathering for the next stride.
- A gathered suspension rather than a stereotypical “all four legs stretched outward” flying pose. Horses normally have one gathered flight phase, unlike the double-flight pattern associated with some other galloping mammals. [originalwisdom](https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/10/Biancardi-and-Minetti_2012_Biomechanical-determinants-of-transverse-and-rotary-gallop-in-cursorial-mammals.pdf)

Do not assume that faster galloping necessarily requires a longer aerial interval. Field research found aerial-phase duration independent of speed in the studied racehorses; reduced support overlap was an important adaptation. [journals.biologists](https://journals.biologists.com/jeb/article/209/21/4389/16347/Effect-of-speed-on-stride-parameters-in-racehorses)

## 6. Neck and head movement

### Separate translation from rotation

A horse’s poll can move substantially vertically while the head’s orientation changes comparatively little.

Research on walk, trot, and canter found that the head, neck, and trunk maintained relatively small rotational excursions, with the neck helping isolate the head from trunk movement. Therefore, a large vertical head bob should not automatically become a large head-pitch rotation. [madbarn](https://madbarn.com/research/stabilization-and-mobility-of-the-head-neck-and-trunk-in-horses-during-overground-locomotion-comparisons-with-humans-and-other-primates/)

Use separate controls for:

1. Neck-base rotation.
2. Distributed neck curvature.
3. Poll position.
4. Head orientation relative to the upper neck.

### Measured vertical excursions

These are peak-to-peak vertical displacements, not angular excursions:

| Gait | Poll/head | Withers | Pelvis |
|---|---:|---:|---:|
| Walk, in hand | Approximately 79–117 mm | 27–36 mm | 66–79 mm |
| Trot, in hand | Approximately 61–78 mm | 47–92 mm | 56–89 mm |

The intervals span breed-specific means for Icelandic, Iberian, and Warmblood horses; they are not the minimum and maximum individual horses could show. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

### Walk timing

At walk:

- Head and pelvis motion are approximately in phase.
- Withers motion is approximately opposite in phase.
- There are two vertical oscillations per stride.
- The head reaches its low point near the middle of forelimb stance. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

A proposed first-pass approximation is:

\[
z_W=z_{W0}+A_W\cos(4\pi\phi+\psi),
\]

\[
z_P=z_{P0}-A_P\cos(4\pi\phi+\psi),
\]

\[
z_H=z_{H0}-A_H\cos(4\pi\phi+\psi).
\]

Here \(A\) is half the peak-to-peak displacement. Adjust \(\psi\) to align the curves with your chosen contact reference.

### Trot timing

At trot:

- Head, withers, and pelvis descend during diagonal support.
- Their minima occur around midstance.
- They rise toward hoof-off and suspension.
- There are two oscillations per stride. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/)

A proposed approximation is:

\[
z_k=z_{k0}-A_k
\cos\left[4\pi\left(\phi-\phi_{\mathrm{midstance}}\right)\right],
\]

where \(k\) identifies head, withers, or pelvis.

Add small phase differences only after the primary motion works.

### Canter and gallop

For canter, build a stride-period rocking curve rather than automatically copying trot’s double-frequency waveform. Keep the head comparatively stabilized while the neck accommodates trunk movement. [madbarn](https://madbarn.com/research/stabilization-and-mobility-of-the-head-neck-and-trunk-in-horses-during-overground-locomotion-comparisons-with-humans-and-other-primates/)

For gallop, the retrieved evidence does not establish universal head/neck angular extrema or an exact phase curve. Use reference footage to fit poll translation and neck extension; do not label arbitrary degree values as measured biomechanics.

As an initial animation-only choice, you can start with a small head-orientation oscillation—approximately ±2°–4° about the selected carriage—and let neck articulation create most of the positional change. This is a tuning suggestion, not a research-derived limit.

For a sport Friesian, treat the high-set neck and characteristic carriage as the base pose; overlay locomotor motion rather than replacing the carriage with a generic low-neck silhouette.

## 7. Tail carriage and motion

Tail carriage is not determined by gait alone. Breed and individual differences can be substantial. A biomechanics review specifically notes that Friesians and related Iberian types may carry a relatively quiet, downward-hanging tail even when the topline is moving freely. [sciencedirect](https://www.sciencedirect.com/science/article/pii/S0737080610002364)

Therefore, separate:

- Dock carriage: an anatomical pose control.
- Passive dock response: small movement driven by the pelvis.
- Hair motion: lag, gravity, airflow, and inertia.
- Active swishing: a separate behavioral animation.

### Proposed carriage setup

These are art-direction starting settings, not measured gait-specific limits:

| Gait | Suggested dock behavior |
|---|---|
| Walk | Relaxed baseline carriage; subtle pelvis-driven response |
| Trot | Similar baseline, with slightly more passive response |
| Canter | Optional modest lift from baseline; no obligatory high carriage |
| Gallop | Optional additional dock lift; stronger trailing hair response |

For a Friesian, a restrained dock is a defensible visual choice. Let the long hair provide much of the secondary motion.

A simple passive dock model is:

\[
I\ddot{\theta}
+c\dot{\theta}
+k(\theta-\theta_0)
=
\tau_{\mathrm{pelvis}}+\tau_{\mathrm{air}}+\tau_{\mathrm{active}}.
\]

This is a proposed mechanical animation model:

- \(I\): rotational inertia.
- \(c\): damping.
- \(k\): restoring stiffness.
- \(\theta_0\): selected resting carriage.

Do not make a perfectly regular left–right tail swish part of every gait cycle. Keep active swishing independent of the locomotion loop.

## 8. Turn this into a working animation system

### Grounded hooves first

During planted stance, maintain the hoof’s world-space contact position:

\[
\mathbf{x}_{\mathrm{hoof,world}}=\text{constant}
\]

until breakover and release.

In an in-place clip representing forward speed \(v\), the planted hoof moves backward relative to the root:

\[
x_{\mathrm{hoof,root}}(t)
=
x_{\mathrm{contact}}-v(t-t_{\mathrm{contact}}).
\]

This is a proposed kinematic constraint. It prevents foot sliding and links stride length to root speed.

### Swing trajectories second

A simple proposed swing trajectory is:

\[
x(w)=x_{\mathrm{off}}
+(x_{\mathrm{contact}}-x_{\mathrm{off}})
(3w^2-2w^3),
\]

\[
z(w)=z_{\mathrm{ground}}+h\sin^2(\pi w).
\]

Use this only for blocking. Real hoof flight is not necessarily a symmetric arch, and animation should include coordinated joint folding rather than relying on the hoof path alone.

### Joint curves third

Use event-based cubic curves:

\[
q_j=C_j(u)
\quad\text{during stance},
\qquad
q_j=S_j(w)
\quad\text{during swing}.
\]

For each curve, store:

- Angle at contact.
- Loading response.
- Midstance extreme.
- Angle at hoof-off.
- Swing flexion peak or peaks.
- Terminal-swing angle.

Do not animate every joint with a single sine wave. Fetlock patterns alone demonstrate why this fails: they can have multiple flexion peaks and gait-dependent extension patterns. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC9502055/)

### Body, head, and secondary motion last

A practical production order is:

1. Establish contact and hoof-off timing.
2. Lock stance hooves to the ground.
3. Fit thorax and pelvis motion.
4. Solve limb chains with anatomical bend directions.
5. Tune joint extrema against measured values and footage.
6. Add scapular movement and pelvic articulation.
7. Fit neck curvature and poll translation.
8. Stabilize head orientation.
9. Add dock response and hair motion.
10. Check the loop seam for both position and velocity continuity.

The key deliverable for the animator is not just “joint X rotates between A and B.” It is a set of synchronized contact events and joint curves, with measured angle ranges used as checks. That gives you recognizable gait mechanics without implying that one breed’s measurements are universal limits.