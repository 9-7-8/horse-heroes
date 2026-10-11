This is the right model-specific foundation. It changes the animation task substantially: your present horse cannot reproduce anatomical gait curves by changing its existing leg-swing angles alone. The first requirement is an **articulated** limb rig; the second is matching hoof travel to the scrolling ground.

I would use your supplied geometry as the authoritative model specification—not treat it as measured equine anatomy. My earlier gait-angle tables were anatomical reference material, not directly usable rotations for this mesh.

## 1. What can be specified against this model today?

| Region | What your geometry supports | What is missing |
|---|---|---|
| Fore carpus | Rest angle of 180.0°; separate forearm/cannon geometry | An actual bending joint |
| Fore fetlock | Included 159.7° / directed 200.3° | An actual bending joint |
| Stifle | Stand-in rest angle of 121.2° | Anatomical hip and femur |
| Hock | Rest angle of 136.5° | An actual bending joint |
| Hind fetlock | Included 162.5° / directed 197.5° | An actual bending joint |
| Shoulder/elbow | Whole-leg attachment and forearm position | Humerus, shoulder point, scapular system |
| Hip | Hindleg attachment and stand-in upper segment | Anatomical pelvis/hip/femur |
| Head/neck | Neck-base pivot and fixed head | Independent poll/head articulation |
| Tail | Root and dock geometry | Moving dock and hair controls |

Those conclusions follow directly from the model description you supplied.

For a first implementation, I would retain the existing upper attachments, articulate the joints that already have geometric definitions, and specify the upper chains using segment leans. That avoids presenting invented shoulder and hip anatomy as though it already exists.

## 2. Your current speed and rigid swing are incompatible with planted feet

Using your supplied pivot locations, hoof-ground locations, and ±0.45-radian swing, I calculated the following:

| Quantity | Result |
|---|---:|
| Current cycle duration, \(2\pi/7\) | 0.898 s |
| Ground travel during that cycle | 1.402 H |
| Maximum forehoof horizontal sweep across the full angular range | 0.565 H |
| Maximum hindhoof horizontal sweep across the full angular range | 0.565 H |
| Ground travel during a 1.10 s walk with duty factor 0.62 | 1.066 H |

These are calculations from your geometry, not horse measurements.

The full available rigid-leg sweep is only about half the travel needed during that proposed walk stance. Even before accounting for its curved path, the limb cannot cover the required planted-foot interval.

The maximum time the ground can traverse that entire horizontal sweep is approximately:

\[
t=\frac{0.565}{1.5625}\approx0.362\ \mathrm{s}.
\]

That is an upper bound on horizontal travel, not a feasible planted stance: a rigid limb rotating around a fixed-height pivot also raises and lowers the hoof.

### The rigid swing lifts the hoof

For a hoof point with initial pivot-relative coordinates \((d_x,d_y)\):

\[
x(\theta)=x_p+d_x\cos\theta-d_y\sin\theta,
\]

\[
y(\theta)=y_p+d_x\sin\theta+d_y\cos\theta.
\]

At the two angular extremes, your forehoof ground-reference point rises approximately:

- 0.056 H at one extreme.
- 0.073 H at the other.

That is about 9–12 cm at the reference size.

The hindhoof reference point rises approximately:

- 0.124 H at one extreme.
- 0.005 H at the other.

That is approximately 20 cm versus 1 cm.

Again, these are geometric calculations, not observed locomotion. They explain why a sinusoidal whole-leg swing cannot provide a convincing support phase without additional joint motion or root compensation.

## 3. Correctly converting included angles to local rotations

Your rest angles are useful, but angle subtraction needs a signed convention.

Let a segment’s lean from downward vertical be:

\[
\beta=\operatorname{atan2}(\Delta x,-\Delta y).
\]

For two successive segments:

\[
r=\operatorname{wrap}(\beta_{\mathrm{distal}}-\beta_{\mathrm{proximal}}).
\]

The animation change at their connecting joint is:

\[
\Delta r=r_{\mathrm{target}}-r_{\mathrm{rest}}.
\]

This is more robust than blindly applying “target included angle minus rest included angle,” because an unsigned included angle does not identify which side of the straight configuration the chain occupies.

### Rest values expressed as signed segment differences

Using the rounded leans you supplied:

| Joint | Signed distal-minus-proximal lean at rest |
|---|---:|
| Fore carpus | \(0.0^\circ-0.0^\circ=0.0^\circ\) |
| Fore fetlock | \(20.3^\circ-0.0^\circ=+20.3^\circ\) |
| Stand-in stifle | \(-41.6^\circ-17.1^\circ=-58.7^\circ\) |
| Hock | \(1.9^\circ-(-41.6^\circ)=+43.5^\circ\) |
| Hind fetlock | \(19.4^\circ-1.9^\circ=+17.5^\circ\) |

Small discrepancies from your included-angle table reflect rounding.

Your animator-facing specification can still use included angles. The implementation must preserve the intended bend direction when reconstructing the signed local rotations.

For fetlocks:

\[
\alpha_{\mathrm{directed}}
=
360^\circ-\alpha_{\mathrm{included}}.
\]

Consequently, increasing the directed fetlock angle means decreasing its included angle. Do not mix these two conventions inside a curve.

## 4. The gait data should now be an event table

For each gait, I would deliver this structure:

| Event | Timing | Upper-chain lean | Carpus/stifle | Hock | Fetlock | Hoof |
|---|---|---|---|---|---|---|
| Contact | Global stride phase | Specified | Included angle | Included angle | Explicit included or directed angle | Position and orientation |
| Loading | Fraction of stance | Specified | Included angle | Included angle | Loaded extension | Ground contact |
| Midstance | Fraction of stance | Specified | Included angle | Included angle | Peak or near-peak extension | Ground contact |
| Hoof-off | End of stance | Specified | Included angle | Included angle | Unloading | Breakover/release |
| Deepest fold | Fraction of swing | Specified | Included angle | Included angle | Recovery angle | Clearance |
| Reaching | Late swing | Specified | Included angle | Included angle | Landing preparation | Next contact target |

Use separate fore and hind tables. When joints reach their deepest fold at different times, give them separate events rather than forcing one “deepest fold” frame for the entire limb.

Each numeric entry should carry one of your requested labels:

- Measured: study, population, gait/speed, and angle convention.
- Fitted to footage: reference clip and fitting method.
- Animator’s setting: proposed value for this model.

An event-specific value should not be labeled measured merely because it falls between a study’s measured minimum and maximum.

## 5. Do not require both exact hoof planting and arbitrary joint curves

A production specification needs to distinguish constraints from preferences.

### Hard constraints

- Grounded hoof position relative to the scrolling terrain.
- Segment lengths.
- Anatomical bend direction.
- Contact and release timing.
- No ground penetration.

### Adjustable targets

- Study-derived joint extrema.
- Intermediate joint angles.
- Scapular/upper-leg lean.
- Body height and pitch.
- Hoof clearance.

If every joint angle is prescribed independently, the resulting hoof will generally miss the ground-contact target.

A practical solver should minimize deviation from the target angles while satisfying contact:

\[
\min_{\mathbf q}
\sum_j w_j
\left(q_j-q_{j,\mathrm{target}}\right)^2
\]

subject to:

\[
\mathbf x_{\mathrm{hoof}}(\mathbf q)
=
\mathbf x_{\mathrm{contact}}(t).
\]

That is a proposed animation method, not a biological model.

During swing, give the animator more direct control of the folding pose and hoof trajectory. During stance, prioritize support and non-sliding contact.

## 6. For the breed-specific gaits, keep speed and cadence separate

At your single scrolling speed:

\[
L=1.5625T
\]

in units of H.

Therefore, changing gait cadence automatically changes the stride length required for non-sliding movement.

For example, the previously proposed 0.37-second fino cycle would require:

\[
L\approx0.578H\approx0.954\ \mathrm{m}
\]

at your current reference speed.

That is a consequence of your game settings—not a measured classic-fino stride.

Classic fino is characterized by rapid, very short steps and slow progression; corto and largo increase extension and forward travel while retaining the Paso Fino breed’s intended even four-beat rhythm. [pfha](https://pfha.org/the-breed/the-gaits/)

Accordingly, there are three possible production choices:

1. Give each gait an appropriate scrolling speed.
2. Keep scrolling speed fixed and fit stride length/cadence to that constraint.
3. Accept deliberate foot sliding as a stylized effect.

For a science-forward movement system, I would choose the first. A classic-fino clip should not be forced to cover ground at the same speed as every other gait.

## 7. Neck, head, and tail need additional controls

Your present neck and fixed head cannot independently reproduce:

- Poll translation.
- Neck curvature changes.
- Head-orientation stabilization.
- Gait-dependent nodding.

That matters particularly when distinguishing a quiet-headed rack/tölt from a running walk with conspicuous head movement. Those differences are documented in the gait literature. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC10451235/)

A minimal upgrade would be:

- Neck-base control.
- One intermediate neck control.
- Independent poll/head control.
- Tail-dock root rotation.
- At least one additional dock segment.
- Separate tail-hair secondary motion.

For each gait, specify carriage as a base pose, then motion relative to it. A fixed “neck rotates ±X degrees” instruction is not enough to define both head position and head orientation.

## 8. What I would carry forward from this specification

I would keep your current geometry unchanged as the reference rest pose, then build the movement description around:

- Included joint angles with explicit fetlock conventions.
- Signed segment leans above the elbow and stifle until anatomical upper chains exist.
- Per-limb contact and hoof-off events.
- Stance-constrained hoof motion.
- Separately identified measured, fitted, and animator-selected values.
- Gait-specific root/ground speed.
- Independent head and tail controls.

That makes the next gait tables genuinely usable for this model, rather than anatomically plausible numbers that its current rigid limbs cannot execute.