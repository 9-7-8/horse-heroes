Yes—the Paso Fino’s smooth gait belongs to a broader group of **ambling** gaits. For animation, their defining differences are not simply “smaller joint rotations”: they are changes in footfall spacing, stance overlap, limb recovery, and how motion is transmitted through the back to the rider. Most maintain ground contact continuously, unlike a suspended trot or flying pace. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

I’ll cover:

- Paso Fino: classic fino, corto, and largo.
- Icelandic tölt.
- Rack and single-foot.
- Tennessee Walking Horse flat walk and running walk.
- Missouri Fox Trotter fox trot.
- Peruvian Paso paso llano and sobreandando.
- Mangalarga Marchador marcha batida and marcha picada.
- Stepping pace/amble.
- Flying pace and harness-racing pace.
- Colombian trocha and collected Colombian trot.

As before, measured values and proposed animation settings are separated. A complete, scientifically verified table of every joint’s minimum and maximum angle does not exist in the retrieved evidence for all these gaits. I will not fill that gap with invented degree values.

## 1. The mathematical distinction: sequence versus coupling

Several apparently different gaits share this contact order:

\[
LH\rightarrow LF\rightarrow RH\rightarrow RF.
\]

What changes is how close those contacts are in time.

“Lateral sequence” means a hind foot is followed by the forefoot on the same side. “Lateral coupling” means same-side contacts are close together; “diagonal coupling” means opposite-side fore/hind contacts are close together. A gait can therefore have a lateral sequence but diagonal coupling—as the fox trot and Colombian trocha do. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

### A useful parameterized gait family

For a symmetrical cycle, define:

\[
\delta_{LH}=0,\qquad
\delta_{RH}=0.5,
\]

\[
\delta_{LF}=\lambda,\qquad
\delta_{RF}=0.5+\lambda.
\]

Here \(\lambda\) is the delay from hind contact to same-side fore contact, expressed as a fraction of the stride.

| \(\lambda\) | Timing relationship | Approximate gait family |
|---:|---|---|
| 0.00 | Same-side contacts simultaneous | Idealized pace |
| 0.10–0.18 | Same-side contacts close together | Stepping pace/broken pace |
| 0.25 | Four evenly spaced contacts | Even rack, tölt, Paso Fino target |
| 0.35–0.45 | Fore contact close to next opposite hind contact | Fox-trot/trocha-like diagonal coupling |
| 0.50 | Diagonal contacts simultaneous | Idealized trot |

These are useful modeling categories, not rigid breed boundaries. Actual contact timing forms a continuum and changes with speed and the individual horse. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

For example, at \(\lambda=0.40\):

\[
LH(0.00)\rightarrow LF(0.40)
\rightarrow RH(0.50)\rightarrow RF(0.90).
\]

The close pairs are:

- LF, then RH.
- RF, then LH of the next cycle.

That produces a fore-first broken diagonal rhythm without changing the overall lateral sequence.

### Ground contact depends on duty factor, too

Use the same limb phase as before:

\[
p_i=\operatorname{frac}(\phi-\delta_i).
\]

The limb is grounded when:

\[
p_i<D_i.
\]

Changing \(\lambda\) alone does not guarantee smoothness or remove suspension. Duty factors must also provide the appropriate support overlap.

For equal duty factors in this simplified model, continuous contact requires:

\[
D\geq \max(\lambda,0.5-\lambda),
\]

for \(0\leq\lambda\leq0.5\).

That equation is a geometric consequence of the proposed contact schedule—not an experimentally measured rule for horses.

## 2. Timing presets for an animator

The following table gives suggested blocking settings. They are not published breed averages. Use them to establish recognizable support patterns, then fit footage of the particular breed and presentation.

All duty factors in this table are initially equal across limbs for simplicity; real horses need separate fore/hind values.

| Animation preset | Stride duration \(T\) | \(\lambda\) | Duty factor \(D\) | Intended support character |
|---|---:|---:|---:|---|
| Classic fino | 0.37 s | 0.25 | 0.65 | Rapid contacts; mostly two/three grounded hooves |
| Paso corto | 0.48 s | 0.25 | 0.55 | Even four-beat; overlapping support |
| Paso largo | 0.43 s | 0.25 | 0.38 | Longer reach; one/two grounded hooves |
| Moderate tölt | 0.55 s | 0.25 | 0.45 | Alternating single and double support |
| Moderate rack | 0.55 s | 0.25 | 0.45 | Similar timing family; different limb style |
| Flat walk | 1.00 s | 0.25 | 0.65 | Walk-like double/triple support |
| Running walk | 0.75 s | 0.23 | 0.55 | Extended walking mechanics; continuous contact |
| Fox trot | 0.65 s | 0.40 | 0.60 | Broken diagonal rhythm; no suspension |
| Paso llano | 0.60 s | 0.25 | 0.55 | Even sequence plus outward forelimb sweep |
| Sobreandando | 0.55 s | 0.15 | 0.55 | Uneven, laterally coupled |
| Marcha picada | 0.80 s | 0.15 | 0.65 | Lateral coupling; substantial triple support |
| Marcha batida, initial approximation | 0.70 s | 0.38 | 0.65 | Diagonal-biased support; continuous contact |
| Stepping pace | 0.55 s | 0.15 | 0.50 | Uneven lateral couplets |
| Flying pace | 0.40 s | 0.00 | 0.38 | Same-side pairs plus suspension |
| Colombian trocha | 0.35 s | 0.40 | 0.55 | Very rapid broken diagonal rhythm |
| Collected Colombian trot | 0.50 s | 0.50 | 0.55 | Diagonal pairs with overlapping support |

For marcha batida, treat the row only as an initial diagonal-coupled approximation. Its specific contact and release timing must be matched to reference footage; it should not be treated as an interchangeable fox-trot clip.

### Cadence versus audible hoofbeats

For a four-beat gait:

\[
\text{contacts per second}=\frac4T.
\]

For a two-beat paired gait:

\[
\text{paired beats per second}=\frac2T.
\]

Thus, the proposed classic-fino cycle of 0.37 seconds produces approximately 10.8 individual hoof contacts per second, while each individual leg completes about 2.7 strides per second.

That distinction explains the rapid “sewing-machine” impression without requiring each leg to complete ten cycles per second.

## 3. Paso Fino: classic fino, corto, and largo

### Scientific identity

The Paso Fino Horse Association defines the breed’s gait as an evenly spaced four-beat lateral gait, with each hoof contacting independently. It describes three presentations:

- Classic fino: full collection, extremely short steps, very slow forward progression.
- Corto: moderate forward speed and medium extension.
- Largo: fastest presentation, longer extension, less collection. [pfha](https://pfha.org/the-breed/the-gaits/)

These are not three unrelated contact sequences. The registry target is the same even rhythm expressed at different stride lengths and degrees of collection. [pfha](https://pfha.org/the-breed/the-gaits/)

A research study of Colombian Paso horses recorded Paso Fino frequencies of approximately 2.60–2.85 strides per second. Those correspond to stride durations of approximately 0.35–0.38 seconds. The study also emphasizes continuous ground contact and frequent three-limb support. [ncbi.nlm.nih](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6097835/)

### Classic fino animation

The defining combination is:

\[
\text{high stride frequency}
+
\text{short stride length}
+
\text{low trunk excursion}.
\]

For a proposed clip:

\[
T=0.37\ \mathrm{s},\qquad L=0.65\ \mathrm{m},
\]

\[
v=\frac LT\approx1.76\ \mathrm{m/s}.
\]

The stride length here is an animation choice within the scale of short strides described for collected Paso presentations—not a universal breed specification. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

Animate:

- Small forward and backward hoof excursions.
- Fast, compact folding and unfolding.
- Independent hoof placement.
- Frequent support overlap.
- A collected outline without freezing the spine.
- Very restrained saddle-region displacement.

Do not simply shrink every joint’s rotation. The Colombian study found that joint flexion/extension measurements distinguish Paso Fino from trocha and Colombian trot; short ground-covering steps do not necessarily imply negligible folding at the elbow, carpus, or fetlock. [ncbi.nlm.nih](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6097835/)

### Corto and largo

For your rig, vary these controls separately:

| Control | Classic fino → corto → largo |
|---|---|
| Stride length | Increase |
| Collection | Generally decrease |
| Limb protraction/retraction | Increase |
| Stance overlap | Tune downward as appropriate |
| Cadence | Fit the horse; do not force a monotonic increase |
| Hoofbeat regularity | Preserve the even registry target |

A review notes that some observed corto/largo performances shift toward broken-trot timing, whereas PFHA describes the desired gait at all speeds as even. For a breed-standard animation, use PFHA’s even target; for individual gait variation, allow measured departures from it. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

### Head, neck, and tail

Suggested animation treatment:

- Use a collected neck base pose for fino.
- Keep the head relatively quiet.
- Permit small cervical adjustments rather than making the head rigid.
- Reduce collection and increase forward reach for largo.
- Keep tail-dock carriage separate from speed; do not automatically raise it with cadence.

PFHA’s “virtually motionless rider” and “no perceptible croup movement” describe the desired visual result, not literal zero movement of every anatomical segment. [pfha](https://pfha.org/the-breed/the-gaits/)

## 4. Tölt, rack, and single-foot

### Shared timing, different style

The Icelandic tölt is an even four-beat lateral gait with the walk-like contact order:

\[
LH\rightarrow LF\rightarrow RH\rightarrow RF.
\]

Flying pace is a separate two-beat gait and must not be used as a fast-tölt substitute. [ihsgb.co](https://ihsgb.co.uk/icelandic-horses/gaits-of-the-icelandic-horse/)

The rack belongs to the same broad four-beat family. Breed-associated differences include forelimb elevation, stride length, speed, carriage, and support timing; a Saddlebred rack and an Icelandic tölt should not be visually identical. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

### Mechanics

Tölt/rack uses spring-like limb mechanics despite retaining ground contact. At higher speeds, single-limb support becomes important. Absence of suspension therefore does not make it mechanically equivalent to walk. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

Animate:

- Four separate load/release events.
- Fetlock yielding during support.
- Limb folding during swing.
- Relatively quiet head carriage.
- Less saddle-region vertical excursion than a comparable trot.
- Small lateral motion where the reference shows it.

The review reports rack/tölt croup vertical excursions around 3.0–6.5 cm in the summarized literature. These are displacement measurements, not head or spinal rotation angles. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

### Joint styling

| Presentation | Proposed rig emphasis |
|---|---|
| Icelandic tölt | Free forward limb movement; reference-dependent forelimb lift |
| Saddlebred rack | More animated forelimb folding and elevation |
| Rocky Mountain-style rack | Typically lower, less animated forelimb action |
| Speed rack/single-foot | Longer reach and more single support; retain independent contacts |

These breed-style distinctions are described in the review; the actual degree values must come from the chosen footage. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

Head/neck: favor comparatively little nod rather than a running-walk-style deep nod. Tail: allow passive vibration and hair lag, but do not impose a synchronized active swish. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

## 5. Tennessee Walking Horse: flat walk and running walk

### What distinguishes it from tölt

The running walk retains predominantly walking, inverted-pendulum mechanics and emphasizes hindlimb overstep. It also has conspicuous head nodding. Rack/tölt instead has more spring-like mechanics and usually less head nod. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

Published values summarized in the review include:

| Parameter | Flat walk | Running walk |
|---|---:|---:|
| Stride duration | Approximately 0.95–1.10 s | Approximately 0.65–0.85 s |
| Stride length | Approximately 1.8–2.4 m | Approximately 2.1–2.9 m |
| Vertical head displacement | Approximately 11–19 cm | Approximately 10–24 cm |
| Vertical croup displacement | Approximately 8 cm | Approximately 3–6 cm |

These are literature ranges summarized by the review, not anatomical limits or mandatory animation targets. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

### Limb animation

Emphasize:

- Hindlimb reaching substantially beneath the trunk.
- Hind hoof landing ahead of the previous same-side forehoof footprint.
- Relatively low, extended hindlimb flight.
- Reference-dependent forelimb elevation and carpal flexion.
- Continuous support rather than trot-like suspension. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

Represent overstep with:

\[
O=x_{\mathrm{hind,next}}-x_{\mathrm{fore,previous}},
\]

measured in world-space travel coordinates.

- \(O>0\): overstep.
- \(O=0\): tracking/capping.
- \(O<0\): understep.

This is an implementation definition, not a prescribed overstep distance.

### Neck/head animation

Allow substantially more poll translation than in the quiet-headed rack or fino.

A proposed starting waveform is:

\[
z_H=z_{H0}-A_H\cos(4\pi\phi+\psi_H),
\]

but fit \(\psi_H\) to footage. Do not claim a universal nod phase from the displacement ranges alone.

Use distributed neck motion, with some head-orientation stabilization, instead of rotating the head as one rigid block around the shoulders.

For a natural-gait game animation, use flat-shod reference rather than treating exaggerated “big lick” show action as the breed’s normal locomotion. The review distinguishes that extreme presentation and discusses its association with modified footwear and soring. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

## 6. Missouri Fox Trotter: fox trot

### Contact rhythm

The fox trot is a broken diagonal gait: the forehoof of a diagonal pair contacts before the opposite hindhoof, with continuous ground contact rather than ordinary trot suspension. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

For an illustrative cycle:

\[
LH(0.00),\quad LF(0.40),\quad RH(0.50),\quad RF(0.90).
\]

The rhythm is uneven:

\[
0.40,\ 0.10,\ 0.40,\ 0.10.
\]

Do not animate “walking front legs plus trotting rear legs” as two unrelated loops. All four limbs must share a coherent stride and support schedule.

### Joint and body treatment

Proposed blocking:

- Forelimbs: relatively flowing placement and recovery.
- Hindlimbs: visible folding and forward recovery.
- Diagonal contacts: separated rather than locked together.
- Trunk: reduced trot-like bounce; fit the mild rocking motion to footage.
- Head/neck: fit the nod to the individual performance rather than copying a Tennessee Walker’s larger nod.

Tail carriage remains a separate pose control; add passive pelvis-driven response and hair lag.

## 7. Peruvian Paso: paso llano and sobreandando

### Paso llano

Paso llano is a smooth lateral-sequence gait distinguished visually by término: an outward sweeping motion of the forelimbs during recovery. The review also describes less head nod and less hindlimb extension than a typical Tennessee running walk. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

The summarized literature gives:

- Stride duration approximately 0.57–0.63 seconds.
- Stride length approximately 1.7–1.9 m.
- Sometimes slightly diagonal-biased contact spacing rather than perfectly quarter-stride contacts. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

### Término requires a 3D trajectory

A sagittal-only rig cannot reproduce it.

For swing phase \(w\), add a lateral component:

\[
y_{\mathrm{hoof}}(w)
=
y_0+sA_{\mathrm{term}}\sin^2(\pi w),
\]

where:

- \(s=+1\) or \(-1\) directs each forelimb outward.
- \(A_{\mathrm{term}}\) is fitted to frontal/three-quarter footage.
- The hoof returns toward its contact line before landing.

This is a proposed trajectory, not a measured término curve.

Distribute the motion through scapular and shoulder controls. Do not create the entire sweep by twisting the carpus sideways. A hoof trajectory does not uniquely specify the anatomical joint rotations.

### Sobreandando

Sobreandando is a more laterally coupled, uneven four-beat presentation, closer to a broken pace than the even paso llano. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

A proposed starting schedule is:

\[
LH(0),\quad LF(0.15),\quad RH(0.50),\quad RF(0.65).
\]

Retain continuous contact and fit both término and overstep to the horse. Do not automatically turn it into a suspended pace.

Head/neck: use relatively restrained nod for paso llano. Tail: keep dock carriage independent of the outward forelimb sweep.

## 8. Mangalarga Marchador: marcha batida and marcha picada

### The central difference is support bias

- Marcha batida emphasizes diagonal coupling/support.
- Marcha picada emphasizes lateral coupling and generally more lateral and triple support. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

They are not simply “trot” and “pace” with the same rotations slowed down.

### Marcha picada

A useful starting point is a laterally coupled contact schedule:

\[
LH(0),\quad LF(0.15),\quad RH(0.50),\quad RF(0.65).
\]

Use longer stance overlap for a slower, triple-support-rich presentation.

The review describes considerable variation: some slower marcha picada observations had extensive triple support, whereas faster examples included appreciable single support. Therefore, one fixed duty factor should not represent every speed. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

### Marcha batida

Start with separated diagonal contacts, but fit the actual contact and hoof-off events to footage before finalizing the clip.

Important controls:

- Fore/hind contact dissociation.
- Fore/hind duty factors.
- Diagonal support duration.
- Triple-support duration.
- Limb folding and extension.
- Thoracolumbar motion.

The gait’s spinal motion has been studied separately, reinforcing that footfall order alone is not a complete marcha animation. [madbarn](https://madbarn.com/research-topics/locomotion/page/6/)

For both marchas, keep head/neck carriage and tail behavior as separately fitted layers. The retrieved sources do not establish universal cervical or tail-joint angular extrema.

## 9. Stepping pace versus flying pace

### Stepping pace/amble

Stepping pace is four-beat and laterally coupled:

\[
LH\rightarrow LF\quad\text{then}\quad RH\rightarrow RF,
\]

with a short within-pair interval and a longer interval between pairs. It generally retains continuous support and may show lateral sway. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

Animation emphasis:

- Same-side limbs recover in broadly related timing.
- Hind contact precedes same-side fore contact.
- Preserve four distinct contacts.
- Allow reference-dependent lateral trunk/head motion.
- Avoid introducing suspension just because the gait is pace-related.

### Flying pace and Standardbred pace

Flying pace is two-beat with near-simultaneous same-side contacts and a suspension phase. Icelandic flying pace is explicitly distinguished from tölt by this pairing and suspension. [ihsgb.co](https://ihsgb.co.uk/icelandic-horses/gaits-of-the-icelandic-horse/)

Idealized contacts:

\[
LH+LF(0),\qquad RH+RF(0.5).
\]

At the proposed \(D=0.38\), suspension occurs over:

\[
0.38\text{–}0.50,\qquad0.88\text{–}1.00.
\]

Those intervals follow from the preset; they are not measured averages.

For a harness pacer, fit actual same-side dissociation rather than forcing perfect simultaneity. Real pacing limbs may land a small interval apart. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

Head/neck and tail should respond to the faster body motion, but do not use canter’s asymmetric rocking pattern.

## 10. Colombian trocha and collected Colombian trot

### Trocha

Trocha is lateral-sequence, diagonally coupled, and uneven. The forelimb contacts before the contralateral hindlimb. It shares a timing family with the fox trot but has a distinct collected, high-frequency presentation. [ncbi.nlm.nih](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6097835/)

The Colombian study measured approximately:

\[
f=2.70\text{–}2.96\ \mathrm{strides/s},
\]

or:

\[
T\approx0.34\text{–}0.37\ \mathrm{s}.
\]

That is considerably faster limb cycling than a typical relaxed trail-gait clip. [ncbi.nlm.nih](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6097835/)

Use rapid, compact recovery and a quiet rider region, but preserve the uneven diagonal rhythm. Do not relabel an even fino clip as trocha.

### Collected Colombian trot

The same study describes a highly collected two-beat diagonal trot with continuous support: either two or four limbs are grounded, rather than an ordinary suspended trot. [ncbi.nlm.nih](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6097835/)

For an initial rig:

\[
\delta_{LH}=\delta_{RF}=0,
\qquad
\delta_{RH}=\delta_{LF}=0.5,
\]

with:

\[
D>0.5.
\]

That creates overlap between diagonal supports. The precise duty factor remains reference-dependent.

## 11. Joint-angle extrema: what can and cannot be specified

There is real angular research on these gaits. The Colombian study measured:

- Fore and hind fetlock maximum swing flexion.
- Fore and hind fetlock maximum stance extension.
- Carpal maximum swing flexion.
- Elbow maximum swing flexion.
- Hock maximum swing flexion.
- Whole-limb protraction/retraction. [ncbi.nlm.nih](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6097835/)

However, the accessible main text does not provide a complete numerical minimum–maximum table for every joint. It places detailed group values in supplementary material and does not furnish full curves for shoulder, hip, stifle, cervical joints, and tail joints.

Consequently, the honest specification is:

| Joint/region | What the animator can reliably use now | What remains to be fitted |
|---|---|---|
| Scapula/shoulder | Reach/retraction and breed-specific limb path | Actual local extrema |
| Elbow/carpus | Swing folding and terminal unfolding | Gait-specific degree values |
| Fetlocks | Loaded stance extension; swing recovery | Extrema and loading curve |
| Hip | Stance retraction and swing protraction | Local extrema versus pelvic contribution |
| Stifle/hock | Coordinated swing folding | Gait-specific extrema |
| Pastern/coffin | Hoof orientation and breakover constraints | Separate joint curves |
| Neck/poll | Carriage, nod, and head stabilization | Segment-by-segment rotations |
| Tail | Independent carriage plus passive response | Vertebral extrema |

Do not transfer the Criollo walk/trot angles from the previous answer into every gaited breed and call them measured Paso Fino or tölt values.

### How to obtain defensible extrema

For each intended breed/gait:

1. Use calibrated lateral footage and frontal footage for lateral motion.
2. Identify contact, hoof-off, maximum folding, and loaded extension.
3. Track anatomical landmarks.
4. Calculate angles with the same convention used by the reference study.
5. Separate pelvic/scapular motion from local joint motion.
6. Normalize several strides before choosing the representative curve.

Then convert anatomical angles to rig rotations:

\[
q_j=s_j(\alpha_j-\alpha_{j,\mathrm{bind}}).
\]

A single angle minimum and maximum still does not specify the animation: two gaits can share similar extrema but reach them at different times.

## 12. Shared limb-cycle implementation

For all these gaits, reuse the stance/swing framework—not necessarily identical joint curves.

| Cycle event | Forelimb control | Hindlimb control |
|---|---|---|
| Contact | Place and orient hoof; begin loading | Place hoof; begin loading |
| Early stance | Allow support-chain yielding | Coordinate hip, stifle, hock yielding |
| Midstance | Peak loaded fetlock extension, as reference dictates | Peak loaded fetlock extension |
| Late stance | Retract limb; perform breakover | Extend/retract into release |
| Early swing | Fold elbow/carpus; clear hoof | Fold stifle/hock; clear hoof |
| Mid swing | Advance folded limb | Advance folded limb beneath body |
| Terminal swing | Unfold and prepare contact | Unfold and prepare contact |

This is a proposed rig organization. Exact peak phases should be fitted, particularly in gaited horses whose fore and hind release times differ substantially.

### A practical data structure

For your simulator, I would store each gait as:

```text
gait:
    stride_duration
    contact_phase [ihsgb.co](https://ihsgb.co.uk/icelandic-horses/gaits-of-the-icelandic-horse/)
    duty_factor [ihsgb.co](https://ihsgb.co.uk/icelandic-horses/gaits-of-the-icelandic-horse/)
    stride_length
    hoof_path [ihsgb.co](https://ihsgb.co.uk/icelandic-horses/gaits-of-the-icelandic-horse/)
    joint_curves [ihsgb.co](https://ihsgb.co.uk/icelandic-horses/gaits-of-the-icelandic-horse/)
    scapula_curve [ncbi.nlm.nih](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6097835/)
    pelvis_and_trunk_curves
    neck_carriage
    neck_motion_curves
    head_stabilization
    tail_dock_pose
    secondary_motion
```

Keep three layers independent:

1. Timing family: even four-beat, lateral couplets, diagonal couplets, or paired pace.
2. Breed presentation: collection, limb elevation, término, overstep, and carriage.
3. Individual variation: actual cadence, asymmetry, speed-dependent timing, and motion amplitude.

That lets a Paso Fino, an Icelandic horse, and a Saddlebred share underlying four-beat timing mathematics without looking like the same horse animation with a different mesh.