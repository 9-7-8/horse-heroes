//! The abilities' tells (#18): what each throws as it is wound up and used,
//! so that someone watching can say which it was with the HUD hidden. The
//! horse's own lean for each is its swing in `attacks.ron`, `casts`, drawn
//! by `game::scene`; this draws what goes with it, timed by the same swing.
//!
//! - A shot (Ember Shot, Thorn Shot) gathers an orb of its element in front
//!   of the horse and throws it at the first enemy standing, to land with
//!   the hit.
//! - A burst (Frost Burst) closes a ring of shards in round the horse and
//!   sends it rushing out along the ground through every enemy.
//! - The Ward swells a glow round the horse raising it, then closes a
//!   bubble over every horse standing for as long as it lasts, flickers
//!   as it runs out and pops when it ends.
//!
//! Where each is at a moment is plain arithmetic (`orb_at`, `ring_closing`,
//! `ring_after`, `bubble_at`, `popped`), so it is unit-tested; the systems
//! only draw it. Every number is in `assets/data/tells.ron`. How it works:
//! docs/ability-tells.md.

use bevy::color::Mix;
use bevy::prelude::*;
use serde::Deserialize;

use crate::game::menu::tokens::{Hex, Tokens};
use crate::game::scene::{Attacks, HORSE_PLACES, Swing, enemy_centre, horse_centre};
use crate::game::{Cast, Game, Screen, TICK};
use crate::sim::data::AbilityKind;
use crate::sim::expedition::{Expedition, Phase};

/// The numbers in `tells.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Tells {
    pub orb: Orb,
    pub ring: Ring,
    pub ward: WardLook,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Orb {
    pub size: f32,
    pub ahead: (f32, f32),
    pub arc: f32,
    pub pulse: (f32, f32),
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Ring {
    pub count: usize,
    pub size: f32,
    pub from: f32,
    pub to: f32,
    pub beyond: f32,
    pub fade_secs: f32,
    pub across: f32,
}

impl Ring {
    /// Where one of its `count` shards is on the ground, from the middle,
    /// when the ring is `radius` wide: no further across the trail than
    /// `across`.
    pub fn shard(&self, shard: usize, radius: f32) -> Vec2 {
        let angle = shard as f32 / self.count as f32 * std::f32::consts::TAU;
        Vec2::new(radius * angle.cos(), radius.min(self.across) * angle.sin())
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct WardLook {
    pub colour: Hex,
    pub radius: (f32, f32, f32),
    pub alpha: f32,
    pub glow: f32,
    pub grow_secs: f32,
    pub warn_secs: f32,
    pub flicker: f32,
    pub pop: f32,
    pub pop_secs: f32,
}

impl Tells {
    pub fn load() -> Tells {
        ron::from_str(include_str!("../../assets/data/tells.ron"))
            .unwrap_or_else(|e| panic!("assets/data/tells.ron: {e}"))
    }
}

/// How far through the gathering part of a swing's wind-up it is, 0 to 1,
/// `due` seconds before it lands; None outside it.
fn gathering(swing: Swing, due: f32) -> Option<f32> {
    (swing.release..swing.wind_up)
        .contains(&due)
        .then(|| (swing.wind_up - due) / (swing.wind_up - swing.release))
}

/// How far through its release it is, 0 to 1; None outside it.
fn releasing(swing: Swing, due: f32) -> Option<f32> {
    (0.0..swing.release).contains(&due).then(|| 1.0 - due / swing.release)
}

/// Where a shot's orb is and its diameter, `due` seconds before its hit
/// lands: growing at `from` through the wind-up, then flying in an arc to
/// `to`, there as it lands. None before the wind-up and once it has landed.
pub fn orb_at(orb: &Orb, swing: Swing, due: f32, from: Vec3, to: Vec3) -> Option<(Vec3, f32)> {
    if let Some(t) = gathering(swing, due) {
        Some((from, orb.size * eased(t)))
    } else {
        releasing(swing, due).map(|t| (from.lerp(to, t) + Vec3::Y * orb.arc * 4.0 * t * (1.0 - t), orb.size))
    }
}

/// How wide a burst's ring is, `due` seconds before its hits land: closing
/// in from `from` to `to` through the wind-up, then rushing out to `reach`
/// by the moment they land. None before the wind-up and once they have.
pub fn ring_closing(ring: &Ring, swing: Swing, due: f32, reach: f32) -> Option<f32> {
    if let Some(t) = gathering(swing, due) {
        Some(ring.from + (ring.to - ring.from) * eased(t))
    } else {
        releasing(swing, due).map(|t| ring.to + (reach - ring.to) * t)
    }
}

/// A burst's ring `since` seconds after its hits landed at `reach`: how wide
/// it is and how much of its shards' size is left. None once it is gone.
pub fn ring_after(ring: &Ring, since: f32, reach: f32) -> Option<(f32, f32)> {
    let t = since / ring.fade_secs;
    (0.0..1.0).contains(&t).then(|| (reach + ring.beyond * t, 1.0 - t))
}

/// How much of a Ward's bubble shows, 0 to 1, `since` seconds after it was
/// raised with `left` seconds of it to go: growing in, then whole, then in
/// its last `warn_secs` flickering between whole and faint.
pub fn bubble_at(ward: &WardLook, since: f32, left: f32) -> f32 {
    if left <= 0.0 {
        return 0.0;
    }
    let grown = (since / ward.grow_secs).clamp(0.0, 1.0);
    if left < ward.warn_secs && ((ward.warn_secs - left) * ward.flicker).fract() >= 0.5 {
        grown * 0.3
    } else {
        grown
    }
}

/// A bubble `since` seconds after its Ward ended: how far it has swelled,
/// as a share of its size, and how much of it still shows. None once gone.
pub fn popped(ward: &WardLook, since: f32) -> Option<(f32, f32)> {
    let t = since / ward.pop_secs;
    (0.0..1.0).contains(&t).then(|| (1.0 + (ward.pop - 1.0) * t, 1.0 - t))
}

fn eased(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub struct TellsPlugin;

impl Plugin for TellsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tells::load())
            .insert_resource(CastSwings(Attacks::load()))
            .init_resource::<After>()
            .add_systems(Startup, spawn)
            .add_systems(Update, show.after(crate::game::advance));
    }
}

/// The abilities' swings, which time their tells as they time the lean.
#[derive(Resource)]
struct CastSwings(Attacks);

/// What a horse's tells are drawn with: its orb, the shards of its ring, the
/// glow round it and the bubble over it, each with its own material.
#[derive(Component)]
struct TellOf {
    horse: usize,
    part: TellPart,
}

#[derive(Clone, Copy, PartialEq)]
enum TellPart {
    Orb,
    Shard(usize),
    Glow,
    Bubble,
}

/// What goes on after a use, per horse: seconds since its burst's hits
/// landed and how far they reached; seconds since the Ward was raised or
/// ended; and how much was left of it a frame ago.
#[derive(Resource)]
struct After {
    burst: [Option<(f32, f32)>; HORSE_PLACES],
    raised: f32,
    ended: f32,
    left: f32,
}

impl Default for After {
    fn default() -> Self {
        After { burst: [None; HORSE_PLACES], raised: f32::INFINITY, ended: f32::INFINITY, left: 0.0 }
    }
}

fn spawn(
    mut commands: Commands,
    tells: Res<Tells>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball = meshes.add(Sphere::new(0.5));
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let mut solid = || materials.add(StandardMaterial { unlit: true, ..default() });
    let part = |commands: &mut Commands, horse, part, mesh: &Handle<Mesh>, material| {
        commands.spawn((
            TellOf { horse, part },
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            Transform::default(),
            Visibility::Hidden,
        ));
    };
    for horse in 0..HORSE_PLACES {
        part(&mut commands, horse, TellPart::Orb, &ball, solid());
        let shards = solid();
        for shard in 0..tells.ring.count {
            part(&mut commands, horse, TellPart::Shard(shard), &cube, shards.clone());
        }
    }
    let mut clear = || {
        materials.add(StandardMaterial {
            base_color: tells.ward.colour.0.with_alpha(tells.ward.alpha),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })
    };
    let glows: Vec<_> = (0..HORSE_PLACES).map(|_| clear()).collect();
    let bubbles: Vec<_> = (0..HORSE_PLACES).map(|_| clear()).collect();
    for horse in 0..HORSE_PLACES {
        part(&mut commands, horse, TellPart::Glow, &ball, glows[horse].clone());
        part(&mut commands, horse, TellPart::Bubble, &ball, bubbles[horse].clone());
    }
}

/// Where the first enemy standing is, and how far past the party's horse
/// at `from` the furthest one is along the trail.
fn foes(expedition: &Expedition, from: Vec3) -> (Option<Vec3>, f32) {
    let standing: Vec<Vec3> = expedition
        .foes
        .iter()
        .enumerate()
        .filter(|(_, foe)| foe.health > 0)
        .map(|(slot, foe)| enemy_centre(slot, foe.max_health))
        .collect();
    let reach = standing.iter().map(|at| (at.x - from.x).abs() + 0.5).fold(0.0, f32::max);
    (standing.first().copied(), reach)
}

#[allow(clippy::too_many_arguments)]
fn show(
    time: Res<Time>,
    game: Res<Game>,
    tells: Res<Tells>,
    swings: Res<CastSwings>,
    tokens: Res<Tokens>,
    mut after: ResMut<After>,
    mut casts: MessageReader<Cast>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut parts: Query<(&TellOf, &MeshMaterial3d<StandardMaterial>, &mut Transform, &mut Visibility)>,
) {
    let dt = time.delta_secs();
    let expedition = match &game.screen {
        Screen::Trail(expedition) => Some(expedition),
        Screen::Camp { .. } => None,
    };
    let ability = |horse: usize| {
        let fighter = expedition?.party.get(horse)?;
        let def = &game.data.abilities[fighter.ability];
        Some((def, swings.0.cast(&def.id)))
    };

    // A burst's ring runs on from where its hits landed.
    for slot in after.burst.iter_mut() {
        if let Some((since, _)) = slot {
            *since += dt;
        }
    }
    for Cast { horse, ability: used } in casts.read() {
        debug!("tell: {} by horse {horse} at {:.2}s", game.data.abilities[*used].id, time.elapsed_secs());
        if let (Some(expedition), AbilityKind::Burst(_)) = (expedition, game.data.abilities[*used].kind)
            && *horse < HORSE_PLACES
        {
            let (_, reach) = foes(expedition, horse_centre(*horse));
            after.burst[*horse] = Some((0.0, reach.max(tells.ring.to)));
        }
    }

    // The Ward's bubble: raised when it goes from none to some, ended when
    // it goes back to none, or the trail is over.
    let left = expedition.filter(|e| e.phase != Phase::Finished).map_or(0.0, |e| e.ward);
    if left > 0.0 && after.left <= 0.0 {
        after.raised = 0.0;
    } else if left <= 0.0 && after.left > 0.0 {
        after.ended = 0.0;
    }
    after.raised += dt;
    after.ended += dt;
    after.left = left;

    for (tell, material, mut transform, mut visibility) in &mut parts {
        let horse = tell.horse;
        let centre = horse_centre(horse);
        let standing = expedition.and_then(|e| e.party.get(horse)).is_some_and(|f| f.health > 0);
        let due = expedition
            .and_then(|e| e.cast_in(horse, TICK))
            .map(|due| due - game.behind());
        let element = ability(horse).and_then(|(def, _)| def.element);
        let mut colour = None;

        // Each part says where it is, how big, and in what colour; or is hidden.
        let shown: Option<(Vec3, Vec3)> = match tell.part {
            TellPart::Orb => match (ability(horse), due, expedition) {
                (Some((def, swing)), Some(due), Some(expedition)) if matches!(def.kind, AbilityKind::Shot(_)) => {
                    let from = centre + Vec3::new(tells.orb.ahead.0, tells.orb.ahead.1, 0.0);
                    let (target, _) = foes(expedition, centre);
                    target.and_then(|to| orb_at(&tells.orb, swing, due, from, to)).map(|(at, size)| {
                        let (share, rate) = tells.orb.pulse;
                        let pulse = 1.0 + share * (time.elapsed_secs() * rate * std::f32::consts::TAU).sin();
                        colour = element.map(|e| tokens.colour.of(e).mix(&Color::WHITE, 0.25));
                        (at, Vec3::splat(size * pulse))
                    })
                }
                _ => None,
            },
            TellPart::Shard(shard) => {
                let wound = match (ability(horse), due, expedition) {
                    (Some((def, swing)), Some(due), Some(expedition)) if matches!(def.kind, AbilityKind::Burst(_)) => {
                        let (_, reach) = foes(expedition, centre);
                        ring_closing(&tells.ring, swing, due, reach.max(tells.ring.to)).map(|r| (r, 1.0))
                    }
                    _ => None,
                };
                let ring = wound.or_else(|| after.burst[horse].and_then(|(since, reach)| ring_after(&tells.ring, since, reach)));
                ring.map(|(radius, scale)| {
                    colour = element.map(|e| tokens.colour.of(e).mix(&Color::WHITE, 0.4));
                    let on = tells.ring.shard(shard, radius);
                    let at = Vec3::new(centre.x + on.x, tells.ring.size / 2.0, centre.z + on.y);
                    (at, Vec3::splat(tells.ring.size * scale))
                })
            }
            TellPart::Glow => match (ability(horse), due) {
                (Some((def, swing)), Some(due)) if matches!(def.kind, AbilityKind::Ward(..)) && due < swing.wind_up => {
                    let t = eased((swing.wind_up - due) / swing.wind_up);
                    let (x, y, z) = tells.ward.radius;
                    alpha(&mut materials, material, &tells.ward, t);
                    Some((centre, Vec3::new(x, y, z) * 2.0 * tells.ward.glow * t))
                }
                _ => None,
            },
            TellPart::Bubble => {
                let (x, y, z) = tells.ward.radius;
                let whole = Vec3::new(x, y, z) * 2.0;
                if left > 0.0 && standing {
                    // It closes in from half its size as it grows in.
                    let grown = (after.raised / tells.ward.grow_secs).clamp(0.0, 1.0);
                    alpha(&mut materials, material, &tells.ward, bubble_at(&tells.ward, after.raised, left));
                    Some((centre, whole * (0.5 + 0.5 * grown)))
                } else if let Some((swell, showing)) = popped(&tells.ward, after.ended) {
                    alpha(&mut materials, material, &tells.ward, showing);
                    Some((centre, whole * swell))
                } else {
                    None
                }
            }
        };

        let wanted = if shown.is_some() { Visibility::Inherited } else { Visibility::Hidden };
        if *visibility != wanted {
            *visibility = wanted;
        }
        if let Some((at, scale)) = shown {
            transform.translation = at;
            transform.scale = scale;
        }
        if let Some(colour) = colour
            && materials.get(&material.0).is_some_and(|m| m.base_color != colour)
            && let Some(mut m) = materials.get_mut(&material.0)
        {
            m.base_color = colour;
        }
    }
}

/// Set a Ward material to `share` of its full cover.
fn alpha(materials: &mut Assets<StandardMaterial>, material: &MeshMaterial3d<StandardMaterial>, ward: &WardLook, share: f32) {
    let wanted = ward.colour.0.with_alpha(ward.alpha * share.clamp(0.0, 1.0));
    if materials.get(&material.0).is_some_and(|m| m.base_color != wanted)
        && let Some(mut m) = materials.get_mut(&material.0)
    {
        m.base_color = wanted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shot() -> Swing {
        Attacks::load().cast("ember_shot")
    }

    fn burst() -> Swing {
        Attacks::load().cast("frost_burst")
    }

    /// The numbers can be used as they are.
    #[test]
    fn the_tells_numbers_are_sound() {
        let tells = Tells::load();
        assert!(tells.orb.size > 0.0 && tells.orb.arc >= 0.0 && tells.orb.pulse.0 < 1.0);
        let ring = tells.ring;
        assert!(ring.count > 0 && ring.size > 0.0 && ring.from > ring.to && ring.to > 0.0);
        assert!(ring.beyond >= 0.0 && ring.fade_secs > 0.0);
        // However wide, the ring stays on the trail (#18: it once left it).
        assert!(ring.across > 0.0 && ring.across + ring.size < crate::game::scene::TRAIL_DEPTH / 2.0);
        for shard in 0..ring.count {
            let on = ring.shard(shard, 20.0);
            assert!(on.y.abs() <= ring.across + 1e-4 && on.x.abs() <= 20.0 + 1e-4);
        }
        let ward = tells.ward;
        assert!((0.0..=1.0).contains(&ward.alpha) && ward.glow > 0.0 && ward.pop > 1.0);
        assert!(ward.grow_secs > 0.0 && ward.pop_secs > 0.0 && ward.flicker > 0.0);
        // The warning is shorter than the Ward itself.
        let data = crate::sim::data::Data::load();
        for ability in &data.abilities {
            if let AbilityKind::Ward(secs, _) = ability.kind {
                assert!(ward.warn_secs > 0.0 && ward.warn_secs < secs / 2.0);
            }
        }
    }

    /// A shot's orb is not there before its wind-up, grows where it
    /// gathers, leaves as the release starts and is at its target as the
    /// hit lands, above the straight line between.
    #[test]
    fn the_orb_gathers_then_lands_with_the_hit() {
        let (orb, swing) = (Tells::load().orb, shot());
        let (from, to) = (Vec3::new(-1.0, 1.5, 0.0), Vec3::new(4.0, 0.5, 0.0));
        assert_eq!(orb_at(&orb, swing, swing.wind_up + 0.01, from, to), None);
        let (at, early) = orb_at(&orb, swing, swing.wind_up - 0.05, from, to).unwrap();
        let (_, late) = orb_at(&orb, swing, swing.release + 0.01, from, to).unwrap();
        assert_eq!(at, from);
        assert!(early < late && late <= orb.size + 1e-5, "{early} {late}");
        let (middle, _) = orb_at(&orb, swing, swing.release / 2.0, from, to).unwrap();
        assert!(middle.y > (from.y + to.y) / 2.0 && middle.x > from.x && middle.x < to.x);
        let (last, _) = orb_at(&orb, swing, 1e-4, from, to).unwrap();
        assert!(last.distance(to) < 0.05, "{last}");
        assert_eq!(orb_at(&orb, swing, -0.01, from, to), None);
    }

    /// A burst's ring closes in through the wind-up, rushes out to its reach
    /// by the hits, and runs on shrinking until it is gone.
    #[test]
    fn the_ring_closes_in_then_rushes_out_with_the_hits() {
        let (ring, swing) = (Tells::load().ring, burst());
        let reach = 12.0;
        assert_eq!(ring_closing(&ring, swing, swing.wind_up + 0.01, reach), None);
        let start = ring_closing(&ring, swing, swing.wind_up - 1e-3, reach).unwrap();
        let closed = ring_closing(&ring, swing, swing.release + 1e-3, reach).unwrap();
        assert!((start - ring.from).abs() < 0.05 && (closed - ring.to).abs() < 0.05, "{start} {closed}");
        let out = ring_closing(&ring, swing, 1e-4, reach).unwrap();
        assert!((out - reach).abs() < 0.05, "{out}");
        let (just, whole) = ring_after(&ring, 0.0, reach).unwrap();
        let (later, less) = ring_after(&ring, ring.fade_secs / 2.0, reach).unwrap();
        assert!(just == reach && later > just && whole == 1.0 && less < whole);
        assert_eq!(ring_after(&ring, ring.fade_secs, reach), None);
    }

    /// The Ward's bubble grows in as it is raised, holds whole, flickers
    /// only in its last `warn_secs`, is gone when the Ward is, and pops:
    /// swells and fades away.
    #[test]
    fn the_bubble_shows_the_ward_and_its_end() {
        let ward = Tells::load().ward;
        assert_eq!(bubble_at(&ward, 0.0, 4.0), 0.0);
        assert!(bubble_at(&ward, ward.grow_secs / 2.0, 4.0) < 1.0);
        let steps = 400;
        let mut faint = 0;
        for i in 0..steps {
            let left = 4.0 * (1.0 - i as f32 / steps as f32);
            let showing = bubble_at(&ward, 1.0 + 4.0 - left, left);
            if left >= ward.warn_secs {
                assert_eq!(showing, 1.0, "{left}");
            } else if showing < 1.0 {
                faint += 1;
            }
        }
        assert!(faint > 0, "it never flickers");
        assert_eq!(bubble_at(&ward, 10.0, 0.0), 0.0);
        let (swell, showing) = popped(&ward, 0.0).unwrap();
        assert!(swell == 1.0 && showing == 1.0);
        let (bigger, less) = popped(&ward, ward.pop_secs / 2.0).unwrap();
        assert!(bigger > 1.0 && less < 1.0);
        assert_eq!(popped(&ward, ward.pop_secs), None);
    }
}
