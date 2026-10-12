# Horse Heroes: procedural abilities and breed mathematics

Working design, October 11, 2026. No GitHub issues are to be opened yet. This expands the earlier breed proposal into a combinable ability vocabulary and explicit mathematical models for Mustang, Friesian, Quarter Horse, Gypsy Vanner, Shetland and Thoroughbred.

All balance bands, breed ratios and animation settings below are proposed game data, not measured breed averages or confirmed implementation. Examples illustrate possible generated results; they are not a catalogue of finished abilities to choose from.

## Two signatures and component-level inheritance

Updated owner decision: every horse has exactly two permanent, procedurally generated signatures, one for combat and one for town. Both use the grammar below, with separate eligible event/effect pools, power budgets, seed domains and saved resolved parameters.

For a horse, store the pair \(S=(A_{\rm combat},A_{\rm town})\). A town benefit never replaces the combat signature, and neither signature consumes a learned-ability loadout slot.

A foal constructs each signature from the corresponding parental signatures, choosing each inheritable component independently. It does not copy a whole signature or the whole combat/town pair.

\[
c_{{\rm child},k,j}=
\begin{cases}
\operatorname{mutate}_{k,j}()&\text{with probability }0.01,\\
c_{{\rm parent\,1},k,j}&\text{with probability }0.495,\\
c_{{\rm parent\,2},k,j}&\text{with probability }0.495,
\end{cases}
\quad k\in\{\mathrm{combat},\mathrm{town}\}.
\]

Here j identifies a semantic component: event, filter, condition, target, effect, repetition pattern, frequency limit or an attached numerical parameter. The 49.5% probabilities describe an unconstrained component draw: first roll its independent 1% mutation chance; otherwise choose a parent 50/50. Validity conditioning can change the distribution of accepted results.

Numerical parameters are chosen from exact parental values, not interpolated or freshly rolled during ordinary inheritance. A number is attached to a named semantic field and predicate schema, not copied into the same array position regardless of meaning.

For example, a health threshold of 40% belongs to a health-fraction condition, while “every fourth action” requires a positive integer count. A parent can supply a compatible value for the same field, but a percentage cannot be placed into an action-count field. Even matching units must satisfy the chosen predicate's allowed domain and the full signature's constraints.

The foal rerolls until both expressions are valid, including all attached numbers. If a chosen predicate has no compatible parameter from the other parent, reroll that parameter/predicate combination using supported parental candidates; do not invent an intermediate value or silently add an outside component without mutation.

An exact parental clone is valid and allowed. Do not reject a signature because it matches a parent, force a mutation for identical parents, or add breeding-specific novelty requirements.

For n independently rolled inheritable components, the probability of at least one mutation before validity filtering is \(1-0.99^n\). It is not a flat 1% per ability or foal; n depends on the resolved grammar. A mutation changes that component, not automatically the entire signature.

Recommended retry implementation: identify semantic fields explicitly, retain each component's mutation flag during validity repair, and reroll incompatible values from its allowed candidate domain. This avoids repeatedly rolling mutation just because another field failed validation. The exact repair algorithm is implementation guidance; only valid completed expressions may be accepted.

Both resolved signatures remain stable on save/load and across generator updates. Breeding is optional flavor, not a requirement for main-loop viability or a guarantee of novelty.

### Town scope

The owner defines the town signature broadly as everything that is not combat. Travel, forage, recovery, construction, trading and other future noncombat systems are in scope; the name does not mean effects must be limited to healing and building.

Town-context abilities are active for every horse physically in the town, regardless of why it is there: resident, party visitor or another town visitor. Permanent residency or a separate activation assignment is not an eligibility requirement. A predicate may still need a specific activity to resolve its effect.

Town abilities stack. Resident optimization can be optional endgame content, but must not become necessary for the main adventure loop. The exact additive/multiplicative treatment of each effect family is tuning/implementation work; do not replace stacking with a strongest-only rule or an unapproved global diminishing-return rule.

Travel components activate in the relevant travel context; this does not automatically authorize a distant town's horses to boost a travelling party. Spatial and activity eligibility must be explicit in each component.

Town signatures can use the same typed grammar with town-specific components. Proposed vocabulary additions:

| Component type | Combinable town components |
|---|---|
| Events / modifier contexts | Town recovery resolves; an eligible stall sale resolves; a town project receives a contribution; construction work resolves; completed forage award; travel progress resolves |
| Filters | Horse is present in the relevant location; selected goods category; selected project type; town is liberated; current hex is roadless |
| Conditions | Recipient has missing health; project is active; eligible stock exists; town has at least N residents |
| Targets | Self; one lowest-health eligible horse; active project; eligible stall transaction; eligible forage award; relevant party travel operation |
| Effects | Modify recovery rate; increase effective construction contribution; modify stall-sale rate; add eligible forage units; modify travel speed |
| Limits | Credit each horse once per contribution; explicit per-signature activation/reset rules; effect-specific physical/economic validity constraints |

These town operations are proposals, not confirmed sale or construction formulas. They must operate on actual simulated recovery, work and transactions, not award benefits per menu opening or catch-up update.

Generation remains compositional: a town horse does not select a finished “healer” or “builder” skill. The generator combines a valid town context, condition, target, effect and numbers just as it does for combat.

## Procedural generation is the system

Author the small vocabulary, compatibility rules and budgets; generate the horse’s actual ability from those components. This follows the repository’s stated approach of a small authored vocabulary producing a large space of constrained combinations, with separate seeds, resolved saved parameters and bounded triggers. ([Procedural-generation design](https://github.com/9-7-8/horse-heroes/blob/main/docs/philosophy/procedural-generation.md))

Use a typed expression, not arbitrary prose:

\[
A=(M,E,F,C,S,X,R,\Theta,L).
\]

- **\(M\), mode:** Triggered effect or modifier of an existing action/payoff.
- **\(E\), event:** The simulation event being observed.
- **\(F\), event filter:** Role, element, origin and participant restrictions.
- **\(C\), state predicate:** An optional condition evaluated at a defined time.
- **\(S\), selector:** Which horse, enemy, setup or award is affected.
- **\(X\), effect:** A primitive operation with a specified destination.
- **\(R\), repetition pattern:** Every event, every Nth qualifying event, first event, or a bounded chance.
- **\(\Theta\), parameters:** Magnitude, threshold, duration, count and probability.
- **\(L\), limits:** Cooldown, charges, caps and reset scope.

The grammar should generate one understandable rule per signature, two signatures per horse. Initially allow one event, up to two compatible event filters, at most one state condition, one selector, one effect and one repetition pattern, plus safety limits.

Each signature means one generated expression, not one primitive operation. A rule such as “every fourth direct Plant Primer cast, shield the lowest-health ally while you are below 40% health” is one combat signature, assembled from multiple reusable components alongside a separate town signature.

### Strict predicates versus actions

An event is an opportunity to evaluate a rule; a predicate returns true or false; a selector returns an entity or bounded set; an effect changes state. Keeping those types distinct makes the generator able to reject nonsensical combinations before they reach combat.

```text
On event E:
  require origin/role/element filters F
  require state condition C
  update qualifying-event counter
  require repetition gate R and frequency limit L
  select target S
  apply effect X using resolved parameters
```

For modifier mode, run this inside the relevant action’s resolution before its result is finalized. Do not generate a second attack simply because a predicate references a Finisher.

## Combinable vocabulary

### Events

These are proposed hooks for the planned simulation, not claims that every hook exists today. Enable a component only when its underlying mechanic exists.

| ID | Event | Data it exposes | Important restriction |
|---|---|---|---|
| E01 | Encounter begins | Encounter ID, enemy roster, final/gate flag | Once per encounter |
| E02 | Direct basic attack resolves | Actor, target, hit result, element, damage | Once per action, not per victim |
| E03 | Direct learned ability resolves | Actor, role, element, cast ID, targets | Once per cast |
| E04 | Direct Primer cast applies setup | Actor, target, element, charges, setup power | Modifier runs before setup is finalized |
| E05 | Direct Finisher consumes setup | Actor, target, selected Primer, contributors | Once for the consumed setup |
| E06 | An ally finishes your contributed setup | Finisher, contributors, selected setup, payoff | Same payoff may grant each eligible contributor credit |
| E07 | Direct hit is critical | Actor, primary target, cast ID | First eligible direct hit per cast |
| E08 | Direct hit is glancing | Actor, target, ordinary and glancing damage | Modifier must run before damage is finalized |
| E09 | Self takes direct enemy damage | Source, actual health loss, cast ID | Aggregate multi-hit action before counting |
| E10 | Standing ally crosses health threshold | Ally, health before/after, encounter ID | Crossing, not “still below” every tick |
| E11 | Self barrier absorbs damage | Barrier owner, amount absorbed, source | Once per incoming action |
| E12 | Self barrier breaks from damage | Barrier owner and remaining amount before hit | Expiration is not a break |
| E13 | Enemy defeated by your direct action | Enemy, credited actor, cast ID | Signature damage cannot earn this trigger |
| E14 | Encounter ends successfully | Encounter ID, survivors, contributions | No trigger on failed/retreated encounter |
| E15 | Combat-active interval elapses | Actor, encounter ID, elapsed combat time | Never runs while travelling or exhausted |
| E16 | Completed forage award | Hex, goods category, amount, award ID | Once per award, not per frame |
| E17 | A roadless hex is completed | Route, hex, progress ID | Not when entering/re-entering a menu |
| E18 | Town healing is resolved | Town, resting horse, elapsed simulation time | Apply as a rate modifier, not bonus per update call |

E01–E15 form the combat library; E16–E18 belong to the broadly defined town/noncombat slot. Enable each hook only when its mechanic exists. The planned world ties forage and travel to biomes and restores health while waiting in town. ([World and town decisions](https://github.com/9-7-8/horse-heroes/blob/main/docs/philosophy/vision.md))

### Event filters

| ID | Boolean filter | Rolled values |
|---|---|---|
| F01 | Actor is self | Fixed |
| F02 | Actor is a different ally | Fixed |
| F03 | Self is a setup contributor | Recorded contributor membership |
| F04 | Action is a basic attack | Fixed |
| F05 | Ability role is selected role | Primer, Finisher, Support |
| F06 | Ability element is selected element | Fire, Ice, Plant |
| F07 | Hit has favorable matchup | Compare action and enemy elements |
| F08 | Hit has unfavorable matchup | Same comparison |
| F09 | Hit is critical | Resolved direct-hit flag |
| F10 | Hit is glancing | Resolved direct-hit flag |
| F11 | Target has selected enemy kind | Armoured, Evasive, Swarm, Warded |
| F12 | Consumed setup matches Finisher element | Same-element Crescendo |
| F13 | Consumed setup differs from Finisher element | Pair reaction |
| F14 | Existing payoff includes a barrier | Payoff output flags |
| F15 | Existing payoff includes recovery | Payoff output flags |
| F16 | Consumed setup has at least Q charges | Q = 2–3, subject to actual charge cap |
| F17 | Award contains selected goods category | Only categories the biome can produce |
| F18 | Action origin is eligible direct origin | Excludes signature, item, pet and damage-over-time procs |

Some filters are implicit type constraints, not extra tooltip clauses: E04 already implies a direct Primer cast. Do not roll “Primer role” as decorative redundancy on that event.

### State predicates

Define health fraction as \(h_i=HP_i/HP_{i,\max}\). Read ordinary state predicates from the action-start or pre-impact snapshot; explicit threshold-crossing events read before and after health.

| ID | Predicate | Starting parameter domain |
|---|---|---|
| C01 | Self health below threshold | \(h_{\rm self}<0.30,0.35,0.40,0.45\) |
| C02 | Self health above threshold | \(h_{\rm self}>0.60,0.70,0.80\) |
| C03 | Primary enemy health below threshold | \(h_{\rm enemy}<0.25,0.35,0.45\) |
| C04 | Primary enemy health above threshold | \(h_{\rm enemy}>0.65,0.75,0.85\) |
| C05 | Another standing ally below threshold | Threshold 0.30–0.50 |
| C06 | Self currently has a barrier | True/false |
| C07 | Selected ally has no barrier | True/false |
| C08 | Primary enemy has selected elemental Primer | Fire, Ice, Plant |
| C09 | At least K enemies remain | K = 2–3 |
| C10 | Only one enemy remains | Fixed |
| C11 | Encounter is final/gate fight | Existing encounter flag |
| C12 | Elapsed combat time exceeds threshold | 6, 10 or 14 seconds |
| C13 | All standing allies exceed health threshold | 0.50 or 0.70 |
| C14 | Self has taken no health damage this encounter | Explicit encounter-local state |
| C15 | Selected ally is not self | Selector restriction |
| C16 | Current hex has no road | Travel only |
| C17 | Horse is resting in town | Town only |

Avoid incompatible or tautological combinations. “Enemy health above 85%” plus “only after 14 seconds” is not impossible, but should be scored as unusually restrictive, not compensated with unlimited power.

### Selectors

| ID | Selector | Compatible destinations |
|---|---|---|
| S01 | Self | Recovery, barrier, temporary ward, one-charge benefit |
| S02 | Lowest-health standing ally, including self | Recovery/barrier |
| S03 | Lowest-health standing ally other than self | Recovery/barrier |
| S04 | Threshold-crossing ally | Rescue effects |
| S05 | Ally with lowest barrier as fraction of max health | Barrier |
| S06 | Eligible ally with greatest actual missing health | Recovery |
| S07 | Actor of the triggering action | Next-action benefit |
| S08 | Primary enemy of the triggering direct action | Direct-action modifier |
| S09 | The selected existing Primer | Setup power/lifetime modifier |
| S10 | The existing payoff currently resolving | Payoff component modifier |
| S11 | Up to K lowest-health standing allies | Split-budget support |
| S12 | Completed eligible forage award | Goods modifier |
| S13 | Resting horse’s healing rate | Town modifier |

Stable horse/enemy IDs break ties. If no eligible target exists, the event contributes no effect and is reported as unproductive; specify that its qualifying-event counter still advances but no cooldown or stored charge is spent until an effect is actually applicable.

For selectors that affect K allies, split a fixed total budget; do not grant K full-strength effects by default. An increasing party size should increase coverage, not multiply one signature’s power without a cap.

### Effects

| ID | Effect primitive | Initial generation band |
|---|---|---|
| X01 | Recover recipient health | 2–4% recipient maximum health |
| X02 | Create/refresh recipient barrier | 4–7% source maximum health |
| X03 | Temporary damage-reduction ward | 5–10% for 2–3 seconds |
| X04 | Modify current direct damage | +8–18% |
| X05 | Improve current glancing damage | Recover 5–15% of its shortfall from ordinary-hit damage |
| X06 | Modify current Primer setup power | +8–16%; no extra charges |
| X07 | Extend current Primer lifetime | +0.5–1.0 seconds; shared duration cap |
| X08 | Modify existing payoff damage | +6–14%; no new payoff |
| X09 | Modify existing payoff barrier | +10–18%; existing barrier cap |
| X10 | Modify existing payoff recovery | +10–18%; existing recovery cap |
| X11 | Store one next-basic-attack damage benefit | +10–18%, expires in 4–8 seconds |
| X12 | Store one next-Primer setup benefit | +10–18%, expires in 4–8 seconds |
| X13 | Add eligible forage units | +1 unit, maximum 2–4 per journey |
| X14 | Modify own town recovery rate | +8–16% |

For X05, if ordinary damage is \(D_o\), normal glancing damage is \(D_g\), and rolled recovery fraction is \(r\), use \(D'_g=D_g+r(D_o-D_g)\). This improves a glance without exceeding an ordinary hit or multiplying reduced damage ambiguously.

For X11/X12, refresh or replace a single stored charge; never stack it. Its later consumption modifies one ordinary direct action and does not emit another qualifying signature event.

These primitives deliberately omit new reactions, free revival loops, cooldown resets, additional full-strength casts, innate elemental resistance and increased Primer charges on crit. The game’s settled rules keep elements on abilities, resistance on barding, one selected Primer consumed by a Finisher, and no recursive secondary chains. ([Combat decisions](https://github.com/9-7-8/horse-heroes/blob/main/docs/philosophy/vision.md))

### Repetition patterns and limits

| ID | Gate | Parameter domain |
|---|---|---|
| R01 | Each qualifying event, subject to cooldown | 8–16 seconds |
| R02 | Every Nth qualifying event | N = 3–6; optional 8–12 second minimum interval |
| R03 | First qualifying event per encounter | One |
| R04 | First qualifying event per enemy | 2–4 enemies maximum per encounter |
| R05 | Bounded chance on qualifying event | 15–30%; mandatory cooldown |
| R06 | At most Q qualifying activations per encounter | Q = 1–3 |
| R07 | Once per combat interval | 8, 10, 12 or 16 seconds |
| R08 | Bounded chance per forage award | 5–10%; mandatory journey cap |

Counts reset per encounter unless the tooltip explicitly says journey. A repeated low-health crossing can retrigger only if its cooldown and encounter cap allow it; oscillating around a threshold cannot bypass those limits.

Prefer counter-based combat abilities to random chance in the first release because their rhythm is visible. Their generation is still procedural: random chance during activation is not required for an ability to be procedurally generated.

## Grammar, compatibility and balance

### Generate structures, not named finished abilities

For each signature slot, choose a broad mechanical family, then assemble components from that breed's predicate pool inside typed constraints. Combat families might be sustained recovery, rescue, direct pressure, setup investment and payoff cooperation; town families can cover every noncombat system. These are constraints on assembly, not prewritten skills.

Each breed has an explicit pool of predicates it can generate. Pools may overlap, but breed is a way to deliberately search for particular predicates, not merely a cosmetic hint over one universally available pool. The precise membership of each starting breed's combat and noncombat pools still needs to be designed.

\[
P(A\mid b)=
\frac{\mathbf{1}_{\rm valid}(A)\,\mathbf{1}_{A\in\mathcal P_b}\,
w_b(E,F,C,S,X,R)\,q(A)}
{\sum_{A'}\mathbf{1}_{\rm valid}(A')\,\mathbf{1}_{A'\in\mathcal P_b}\,w_b(A')\,q(A')}.
\]

Here \(\mathcal P_b\) is the breed's allowed component pool, \(w_b\) is an optional within-pool weight, and \(q\) is a quality weight. Invalid or out-of-pool generated candidates have probability zero. Parental component inheritance is not the same operation as fresh breed generation.

Examples of compatibility:

- **Primer application:** E04 permits X06/X07 on S09, or a separate bounded self/ally support effect; it cannot modify a nonexistent payoff.
- **Finisher consumption:** E05 permits X08–X10 on S10; support-component modifiers require F14/F15.
- **Health crossing:** E10 permits support on S04, not damage to an enemy that the event never identified.
- **Direct damage modifier:** X04 needs a pre-finalization direct damage context; “after the enemy dies, increase that hit” is too late.
- **Forage:** E16 permits X13/S12, never a combat ward.
- **Town:** E18 permits a continuous rate modifier, never healing once per UI update.

A generated combat ability must have a witness build: at least one legal learned loadout and reachable encounter state where it activates usefully. Do not require the horse’s current loadout to match it, since the shared pool lets the player rebuild. A town signature instead needs a reachable eligible assignment or town activity where it contributes usefully.

### Power budget

Estimate trigger rate before rolling magnitude. If qualifying actions occur at rate \(r\), every-N gating has nominal rate \(r/N\); probability gating has rate \(pr\).

A first conservative approximation with cooldown \(c\) is:

\[
\lambda\leq\min\left(\frac rN,\frac1c\right)
\quad\text{or}\quad
\lambda\leq\min(pr,1/c).
\]

These are upper bounds, not exact renewal-process expectations. Actual rate depends on action scheduling, predicates and which events occur while cooldown is active.

For an encounter lasting \(t\), useful output is roughly:

\[
U\approx
\min(\lambda t,Q)\,m\,P(C)\,\eta.
\]

\(m\) is effect magnitude, \(Q\) is an activation cap, and \(\eta\) is utilization, such as the fraction of healing not wasted. Normalize health, damage and barrier contributions against reference encounters before comparing families; raw units are not interchangeable.

Rare conditions can receive modestly larger magnitudes, but must never create an unbounded coefficient. Enforce both sustained-output and maximum-single-event budgets, then test the resolved ability across short fights, long bosses and three- and five-horse parties.

### Diversity that changes a build

Mechanically canonicalize the rule before hashing it. Sort equivalent filters, remove implied predicates, distinguish selector inclusion of self, and include all resolved gameplay parameters and limits.

For unrelated founder generation, the earlier proposed diversity policy can use:

- **Exact mechanical fingerprint:** Prevent unrelated founders from receiving identical rules.
- **Structural distance:** Favor changes in event, selector, effect or condition, not only “12% versus 14%.”

For example, define:

\[
d(A,B)=3[E_A\ne E_B]+3[X_A\ne X_B]+2[S_A\ne S_B]
+[F_A\ne F_B]+[C_A\ne C_B]+[R_A\ne R_B].
\]

The weights are proposed. Use this as a local novelty preference rather than a hard promise that every horse differs radically from all previous horses.

An illustrative unconstrained slice with 8 events, 6 filters, 8 conditions, 5 selectors, 6 effects, 4 repetition patterns and 5 magnitude steps has 230,400 combinations. Compatibility filtering reduces that number substantially; it is not a count of approved abilities or a guarantee of endless unique outputs.

Foals are explicitly exempt from exact-match rejection: component recombination may yield either a new expression or a parental clone. Component mutation may introduce new vocabulary, but must not be forced merely to avoid a duplicate.

For founders only, bounded collision retries may search other valid combinations inside the breed's actual pool; never broaden outside that pool silently. Literal global uniqueness is not a requirement for breeding.

### Illustrative generated outputs

These are examples of assembling components, not authored entries in an ability list:

```text
E03 + F01/F05(Primer) + C01(40%) + S02 + X02(5%)
    + R02(4) + cooldown(10s)

"Every fourth direct Primer cast while below 40% health shields the
lowest-health standing ally for 5% of your maximum health.
At most once every 10 seconds."
```

```text
E05 + F12 + F16(2) + S10 + X08(10%)
    + R01 + cooldown(12s)

"When your Finisher consumes a same-element Primer with at least two
charges, that payoff deals 10% more damage. Once every 12 seconds."
```

```text
E08 + F05(Finisher) + S08 + X05(12%) + R01 + cooldown(8s)

"Your glancing direct Finishers recover 12% of the damage lost to
glancing. Once every 8 seconds."
```

Generate names and tooltip sentences from the resolved expression. Names never determine mechanics, and optional adjectives never count toward mechanical uniqueness.

## Mathematical breed builds

“Build” here means physical conformation, not combat loadout. Appearance is generated from a breed-conditioned parameter distribution and does not secretly set combat damage or defense.

### Scale and normalized proportions

Let \(H\) be ground-to-withers height in metres. Store every anatomical length as a ratio of \(H\), and multiply by \(H\) when constructing a horse:

\[
{\bf p}=H{\bf r}.
\]

The current Friesian shape already uses withers-relative dimensions and variation windows, including the constraint that body depth plus under-chest clearance equals height. Keep that approach while adding breed-specific profiles. ([Existing shape data](https://github.com/9-7-8/horse-heroes/blob/main/assets/data/horse_shape.ron))

The following are chosen art starting values. Height intervals are proposed generator limits, not claims about the full real-world range of any breed.

| Parameter | Mustang | Friesian | Quarter Horse | Gypsy Vanner | Shetland | Thoroughbred |
|---|---:|---:|---:|---:|---:|---:|
| Reference \(H\), m | 1.47 | 1.65 | 1.52 | 1.45 | 1.00 | 1.65 |
| Generated \(H\) interval, m | 1.35–1.58 | 1.55–1.75 | 1.42–1.65 | 1.32–1.60 | 0.85–1.07 | 1.55–1.75 |
| Shoulder-to-buttock length / H | 1.06 | 1.08 | 1.04 | 1.10 | 1.14 | 1.06 |
| Barrel width / H | 0.35 | 0.36 | 0.38 | 0.43 | 0.46 | 0.32 |
| Chest width / H | 0.25 | 0.26 | 0.29 | 0.32 | 0.33 | 0.23 |
| Hindquarter width / H | 0.33 | 0.34 | 0.38 | 0.40 | 0.40 | 0.32 |
| Girth depth / H | 0.47 | 0.46 | 0.48 | 0.51 | 0.53 | 0.45 |
| Under-chest clearance / H | 0.53 | 0.54 | 0.52 | 0.49 | 0.47 | 0.55 |
| Head length / H | 0.35 | 0.36 | 0.34 | 0.35 | 0.38 | 0.35 |
| Head width / H | 0.14 | 0.14 | 0.15 | 0.15 | 0.17 | 0.13 |
| Neck root-to-poll length / H | 0.46 | 0.53 | 0.45 | 0.47 | 0.40 | 0.52 |
| Neck carriage above horizontal | 40° | 50° | 35° | 45° | 35° | 40° |
| Neck side-depth at root / H | 0.25 | 0.285 | 0.27 | 0.31 | 0.30 | 0.23 |
| Cannon side-thickness / H | 0.052 | 0.058 | 0.058 | 0.067 | 0.066 | 0.045 |
| Mane fall / H | 0.16 | 0.24 | 0.12 | 0.30 | 0.28 | 0.10 |
| Feather envelope beyond lower limb / H | 0.010 | 0.045 | 0.006 | 0.065 | 0.025 | 0.004 |

Hair values are maximum envelope controls, not uniform spheres around the legs. Feather should taper and follow its anatomical attachment zone.

These profiles produce distinct silhouettes: Quarter Horse has greater hindquarter/chest width, Vanner greater depth and bone, Shetland less clearance and proportionally larger head, Thoroughbred greater clearance with narrower volumes, and Friesian a longer raised neck. Mustang should have a wider permitted shape distribution than the other profiles.

### Correlated individual variation

Do not roll every dimension independently. Sample bounded latent traits \(z_s,z_b,z_l,z_n,z_h\in[-1,1]\) for size, breadth/substance, relative leg length, neck expression and head expression, plus small per-parameter residuals.

One proposed deformation is:

\[
\begin{aligned}
H&=H_b(1+\sigma_Hz_s),\\
w_{\rm barrel}&=w_b+0.025z_b+\epsilon_w,\\
w_{\rm chest}&=c_b+0.020z_b+\epsilon_c,\\
w_{\rm quarters}&=q_b+0.022z_b+\epsilon_q,\\
d_{\rm girth}&=d_b+0.015z_b-0.018z_l,\\
c_{\rm under}&=1-d_{\rm girth},\\
\ell_{\rm body}&=\ell_b+0.020z_s+0.012z_b+\epsilon_\ell,\\
\ell_{\rm neck}&=n_b+0.020z_n+\epsilon_n,\\
\theta_{\rm neck}&=\theta_b+4^\circ z_n+\epsilon_\theta,\\
\ell_{\rm head}&=a_b+0.010z_h+\epsilon_a.
\end{aligned}
\]

The \(w,d,\ell\) terms are normalized ratios. Truncate or reject values outside each breed’s windows; choose \(\sigma_H\) to fit the height interval, and let \(|\epsilon|\leq0.005\) for ratio residuals and \(|\epsilon_\theta|\leq1^\circ\).

The numerical offsets are modeling settings. A bounded triangular distribution centered at zero is an adequate first latent distribution; Mustang can multiply shape-variation amplitudes by 1.3 before constraint checks.

Crossbreeds use inherited genetics, not ancestry-weighted averaging of breed body profiles. Keep the real coat-genetics system, and introduce abstracted heritable loci for body type, limb proportions, height and related dimensions.

One proposed abstraction gives each parent two alleles at every body locus l; the foal receives one randomly chosen allele from each parent. Map the resulting genotype into the existing latent shape controls:

\[
G_{{\rm child},l}=(\operatorname{pick}(G_{{\rm parent\,1},l}),
\operatorname{pick}(G_{{\rm parent\,2},l})),\qquad
{\bf z}=f(G_{\rm body}),\qquad {\bf r}=g({\bf z}).
\]

The phenotype maps f/g and their loci are an abstraction to design, not assertions about identified real equine body-shape genes. Breed profiles describe founder phenotype/genotype distributions; a crossbred foal's dimensions follow inherited alleles and anatomical constraints, not a cosmetic reroll into one breed's window. The exact abstract loci and phenotype mappings remain implementation/design work.

### Skeleton and volume construction

Joint centers are separate from surface dimensions. As first limb-height landmarks, use the following proposed ratios:

| Landmark / H | Mustang | Friesian | Quarter Horse | Gypsy Vanner | Shetland | Thoroughbred |
|---|---:|---:|---:|---:|---:|---:|
| Elbow | 0.59 | 0.60 | 0.59 | 0.56 | 0.54 | 0.61 |
| Carpus | 0.30 | 0.306 | 0.29 | 0.28 | 0.25 | 0.32 |
| Hock | 0.35 | 0.364 | 0.35 | 0.33 | 0.31 | 0.37 |
| Fetlock | 0.12 | 0.121 | 0.12 | 0.12 | 0.11 | 0.12 |
| Coronet | 0.065 | 0.067 | 0.065 | 0.070 | 0.065 | 0.060 |

These height landmarks do not specify complete joint positions or anatomical angles. Generate the scapular, shoulder, elbow, hip, stifle and hock centers as one constrained skeleton, calculate segment lengths from that skeleton, and keep those lengths fixed during animation.

For a segment between joint centers \(J_i,J_{i+1}\):

\[
\ell_i=\|J_{i+1}-J_i\|,\qquad
\alpha_j=\cos^{-1}
\frac{(J_{j-1}-J_j)\cdot(J_{j+1}-J_j)}
{\|J_{j-1}-J_j\|\,\|J_{j+1}-J_j\|}.
\]

Use signed/unwrap conventions for directed fetlock angles and local rotations. This equation computes an included angle, not a rig rotation or a verified anatomical limit.

Represent the torso using overlapping ellipsoidal shoulder, ribcage and hindquarter volumes. Their centers follow body landmarks and their radii follow chest/barrel/hindquarter widths and girth depth; smoothly blend them rather than constructing three visibly separate balls.

A neck can follow a cubic Bézier centerline:

\[
B(t)=(1-t)^3P_0+3(1-t)^2tP_1+3(1-t)t^2P_2+t^3P_3.
\]

Set \(P_0\) at the neck root and \(P_3=P_0+H\ell_{\rm neck}(\cos\theta,\sin\theta,0)\). Place \(P_1,P_2\) to create a bounded crest arch while tapering cross-section from root to throat; Friesian and Vanner can have stronger arch priors than Quarter Horse or Thoroughbred.

Here root-to-poll length is straight-line distance, not Bézier arc length. If a later implementation instead defines anatomical centerline length, it must solve that length explicitly rather than confusing the two.

### Geometry constraints

Always enforce positive dimensions, body depth plus clearance equal to H, increasing joint heights from coronet to elbow where appropriate, left/right symmetry of the resting skeleton, and gear sockets derived from actual landmarks. Check stance reach and hoof-ground contact before accepting a generated horse.

A Shetland must use its own ratio profile, not just a smaller H. Shape changes should not alter combat contribution; any future breed stat tendencies must be separately generated and balanced.

## Mathematical breed gaits

### Timing family, breed presentation, individual

The six starting breeds use walk, trot, canter and gallop as their initial timing repertoire. This is a chosen starter scope; individual optional gait variants can be added later rather than assigning a fantasy amble merely to differentiate a breed.

Separate footfall timing from breed style. A published study found breed differences in walk/trot timing in Warmblood, Iberian and Icelandic horses, but it does not provide breed-specific values for these six starting breeds and did not model speed effects. The numbers below are consequently animation proposals, not extrapolated measurements. ([Gait-timing study](https://pmc.ncbi.nlm.nih.gov/articles/PMC9657284/))

Let \(v\) be displayed ground speed, \(T\) stride duration and \(\phi\) normalized phase:

\[
L=vT,\qquad f=1/T,\qquad
\phi(t)=\operatorname{frac}\left(\int_0^t f(\tau)\,d\tau+\phi_0\right).
\]

The integral keeps phase continuous when cadence changes. It is preferable to recomputing \(t/T\) when T varies.

For limb \(i\), with contact offset \(\delta_i\) and duty factor \(D_i\):

\[
p_i=\operatorname{frac}(\phi-\delta_i),\qquad
\text{grounded}_i\iff p_i<D_i.
\]

### Shared initial footfall presets

Use limb order LH, LF, RH, RF. Mirror left/right for a left-lead canter or gallop.

| Gait | Contact offsets \((\delta_{LH},\delta_{LF},\delta_{RH},\delta_{RF})\) | Duty factors | Derived support |
|---|---|---|---|
| Walk | (0, 0.25, 0.50, 0.75) | Fore 0.64, hind 0.60 | Continuous ground contact |
| Trot | (0, 0.50, 0.50, 0) | Fore 0.46, hind 0.43 | Two diagonal supports with intervening suspension |
| Right-lead canter | (0, 0.30, 0.30, 0.60) | All 0.32 initially | Three beats; suspension 0.92–1.00 |
| Right-lead transverse gallop | (0, 0.40, 0.15, 0.55) | All 0.25 initially | Four beats; suspension 0.80–1.00 |

These schedules are simplified blocking presets. Canter/gallop ultimately need per-limb duty factors and trajectories fitted to reference motion, not four shifted copies of one limb animation.

Support count is \(N(\phi)=\sum_i\mathbf{1}[p_i<D_i]\). Suspension is exactly the set of phases where \(N=0\), making support and beat structure testable without rendering.

### Breed stride-length priors

Let \(s=L/H\). Choose \(s\) from a breed/gait prior, then compute \(T=sH/v\); cadence is a consequence rather than an unrelated random number.

| Preferred \(L/H\) | Mustang | Friesian | Quarter Horse | Gypsy Vanner | Shetland | Thoroughbred |
|---|---:|---:|---:|---:|---:|---:|
| Walk | 1.00 | 0.98 | 0.95 | 0.90 | 0.90 | 1.05 |
| Trot | 1.50 | 1.55 | 1.40 | 1.35 | 1.30 | 1.65 |
| Canter | 1.95 | 2.00 | 1.85 | 1.75 | 1.65 | 2.15 |
| Gallop | 2.55 | 2.60 | 2.45 | 2.25 | 2.15 | 2.85 |

Generate individual stride preference within ±6% of its breed center. Larger ranges and genuinely speed-dependent curves should come only after visual/reference validation.

For initial cadence envelopes in strides/second, use walk 0.65–1.40, trot 1.10–2.20, canter 1.30–2.60 and gallop 1.60–3.20. Those are controller settings, not breed speed limits.

Choose a gait and small permissible stride adjustment so that both cadence and stride are inside their envelopes. Add transition hysteresis, such as 10%, to avoid flipping gait at every minor speed change.

At the same displayed speed \(v=1.2\) m/s and the reference heights above:

| Breed | Walk stride L, m | T, seconds | f, strides/second |
|---|---:|---:|---:|
| Mustang | 1.470 | 1.225 | 0.816 |
| Friesian | 1.617 | 1.348 | 0.742 |
| Quarter Horse | 1.444 | 1.203 | 0.831 |
| Gypsy Vanner | 1.305 | 1.088 | 0.920 |
| Shetland | 0.900 | 0.750 | 1.333 |
| Thoroughbred | 1.733 | 1.444 | 0.693 |

Thus the Shetland takes more steps without skating or gaining additional combat actions. At higher party speeds it may transition to trot earlier while a taller horse still walks; each horse shares the same displayed ground speed, not necessarily the same gait.

### Breed presentation parameters

The following are proposed controls for a reference trot. They are not maximum joint angles, and they should be used with solved leg geometry, not copied as joint rotations.

| Parameter | Mustang | Friesian | Quarter Horse | Gypsy Vanner | Shetland | Thoroughbred |
|---|---:|---:|---:|---:|---:|---:|
| Maximum swing hoof clearance / H | 0.055 | 0.090 | 0.045 | 0.075 | 0.065 | 0.060 |
| Trunk vertical amplitude / H | 0.018 | 0.024 | 0.014 | 0.020 | 0.020 | 0.022 |
| Pelvic pitch amplitude | 2.0° | 2.5° | 1.5° | 2.0° | 2.0° | 2.5° |
| Poll vertical amplitude / H | 0.012 | 0.010 | 0.010 | 0.012 | 0.012 | 0.014 |
| Standing neck carriage prior | 40° | 50° | 35° | 45° | 35° | 40° |

Higher hoof clearance is a visible style control, not a complete description of knee action. The IK solution should distribute lift through scapula, shoulder, elbow and carpus rather than rotating one joint by an invented breed angle.

A prototype trot trunk target can use:

\[
y_{\rm trunk}=\bar y-A_y\cos(4\pi\phi+\psi_y).
\]

Fit \(\psi_y\) to support so the trunk loads and rebounds at appropriate phases. Walk, canter and gallop require their own event curves; do not reuse this trot cosine as the complete gait model.

Generate bounded individual presentation variation of about ±10% around amplitudes, with near-zero left/right asymmetry by default. Do not generate apparent lameness as ordinary personality.

### Planted hooves and swing paths

At constant v and T within a stride, in stance:

\[
x_i(p_i)=x_{c,i}-vTp_i,\qquad y_i=0.
\]

That is the hoof’s movement relative to the stationary displayed horse. The world-space contact point stays fixed while the ground scrolls.

For swing, let \(u=(p_i-D_i)/(1-D_i)\), \(Q(u)=6u^5-15u^4+10u^3\), and \(x_{\rm off}=x_c-vTD_i\). One continuous path is:

\[
\begin{aligned}
x_i(u)&=x_{\rm off}-vT(1-D_i)u+vTQ(u),\\
y_i(u)&=16h_i u^2(1-u)^2.
\end{aligned}
\]

This reaches \(x_c\) at the next contact, has zero vertical velocity at contact and hoof-off, and matches the stance’s horizontal velocity at both ends. It is a trajectory proposal, not a complete anatomical animation.

Solve the articulated limb to this hoof path while preserving segment lengths and joint bend directions. If it cannot reach, adjust body pose, stride, cadence or selected gait; never stretch the bones or permit the hoof to slide.

When displayed speed changes during stance, maintain the saved world contact point, or integrate \(-v(t)\), rather than applying the constant-speed expression unchanged. Transition gaits at compatible contact phases without teleporting a planted foot.

### Gaits have mechanical travel and pet effects

Confirmed owner decision: breed gaits affect travel speed and give a boost to pets. There is no travel-efficiency stat. This supersedes the earlier recommendation that breed gait be presentation-only.

Keep a gait's gameplay values separate from its animation settings: its supported travel-speed values and pet-boost definition are data, while contact phases, duty factors and limb curves describe how that motion is shown. Gameplay does not derive power from the number of rendered hoofbeats.

For horse i, use a gait-dependent travel speed \(v_i=V(G_i,g_i)\), where \(G_i\) is its inherited body/gait data and \(g_i\) is the active gait. Confirmed owner decision: party travel speed uses the arithmetic average, \(v_{\rm party}=\frac{1}{N}\sum_{i=1}^N v_i\), not the slowest horse or a selected leader. Apply route/biome rules consistently with this aggregation, then advance distance through \(d(t)=\int v_{\rm party}(t)\,dt\).

Confirmed owner decision: different gaits boost different aspects of pets. Each boost must be an explicit gameplay definition, not an accidental consequence of faster animation. The exact gait-to-pet-benefit mapping, which pets receive it and whether it applies only while travelling remain open.

### Keep simulation and animation responsibilities distinct

Simulation travel progress now depends on gait travel-speed data. Displayed cadence must remain consistent with that motion, but it must not independently accelerate the horse's combat cooldowns or directly generate extra forage awards per hoofbeat.

A mechanical change of travel gait can change journey duration because it changes travel speed, although route distance stays the same. Cosmetic approach/arrival animation may still be staged separately; it cannot override the simulation's resolved progress. During combat, align attack animations with simulation events rather than counting hoofbeats, and apply pet boosts only through their defined gameplay rules.

## Validation and future issue organization

The owner has settled component-level 50/50 inheritance, exact parental numerical choices with schema-aware validity, 1% mutation per inheritable component, rerolling until valid, permitted parental clones, broad noncombat signatures, all-present-horse town activation, stackable town benefits, breed-specific predicate pools and genetics-based crossbreeds with abstract body loci.

The owner has also settled that breed gaits affect travel speed and boost pets; there is no efficiency stat.

The owner has settled arithmetic-average party travel speed and different pet benefits for different gaits. Still to flesh out: the exact predicate membership of each breed's two pools; the body-genetics locus/phenotype abstraction; the complexity limit per signature; and the exact mapping, recipients and activation scope of gait-based pet boosts. Numerical effect bands and stacking coefficients are tuning proposals, not a reason to reopen the decisions above.

Later issue groups should cover the grammar, event dispatch/origin rules, compatibility and budget validation, uniqueness and inheritance, breed parameter generation, shared gait timing, breed presentation, planted-hoof solving, UI descriptions and reports. Keep them linked to existing breed/gait/ability work rather than creating duplicate epics.

The repository has thematic labels including `generation`, `sim`, `combat`, `build`, `models`, `gaggle`, `town`, `decision`, `proposed` and `epic`, as well as release labels such as `v0.12.0` for the next-generation work. Use the established labels when filing later, not new parallel terminology. ([Repository labels](https://github.com/9-7-8/horse-heroes/labels))

Breed data currently carries `generation`, `models`, `v0.10.0` and milestone “0.10.0 Horses you can see.” ([Existing breed-data issue](https://github.com/9-7-8/horse-heroes/issues/268))

The basic gait epic is assigned to milestone “0.5.0 How a horse moves.” This is routing context, not permission to file anything now, and the release placement of the new signature work still needs to be chosen when filing. ([Existing gait epic](https://github.com/9-7-8/horse-heroes/issues/255))

Validation targets:

- **Abilities:** Valid typed combinations; witness builds; deterministic outputs; no recursive activation; exact counter/reset behavior; caps across party sizes; comparable useful power; genuine structural variety.
- **Shapes:** Finite positive dimensions; breed profiles recognizable with hair hidden; linked dimensions consistent; articulated reach feasible; gear sockets follow landmarks.
- **Gaits:** Correct beats and support count; planted feet; invariant segment lengths; correct lead mirroring; smooth transitions; travel-speed effects change simulation progress; pet boosts use explicit gameplay hooks, never rendered frame/hoofbeat counts.
- **Persistence:** Save resolved expression/shape/gait parameters and versions; adding scenery or changing breed weights must not reroll an existing horse.

Nothing in this document creates or edits a GitHub issue. It is the specification to finish reviewing before that separate step.
