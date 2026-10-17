//! Screen response: when a fountain erupts the scene shakes and flashes once,
//! as strongly as the best piece in the burst says and no more for there
//! being many. The piece's rarity has a level, and the level's budget in
//! `assets/data/importance.ron` (#39) says how strong and how long; the two
//! scales that turn every level down or off are in
//! `assets/data/loot_shower.ron`.
//!
//! How far the view is thrown and how solid the flash is are plain arithmetic
//! (`shake_offset`, `flash_alpha`), so they are unit-tested; the system below
//! them only applies them.

use std::f32::consts::TAU;

use bevy::prelude::*;
use serde::Deserialize;

use super::drops::{Shower, rarity_colour};
use crate::game::menu::tokens::Tokens;
use crate::game::scene::camera_home;
use crate::game::{Game, SceneView};
use super::importance::{Budget, Importance};
use crate::sim::data::GearTable;

/// The `response` numbers in `loot_shower.ron`.
#[derive(Clone, Debug, Deserialize)]
pub struct ResponseFeel {
    /// Multiply every shake and every flash. 0 turns that one off.
    pub shake_scale: f32,
    pub flash_scale: f32,
    pub shake_hz: f32,
}

/// A burst has been thrown: its first piece leaves `in_secs` from now, and
/// the best of its gear is of rarity tier `best`, if it has any gear.
#[derive(Message)]
pub struct Erupted {
    pub in_secs: f32,
    pub best: Option<usize>,
}

/// How strongly, and for how long, the screen answers a burst whose best
/// piece is `best`: its rarity's level's budget, turned down by the scales.
pub fn strength(feel: &ResponseFeel, importance: &Importance, gear: &GearTable, best: Option<usize>) -> Budget {
    let level = best.map_or(importance.coins, |tier| importance.rarity(&gear.rarities[tier]));
    let budget = *importance.budget(level);
    Budget {
        shake: budget.shake * feel.shake_scale,
        flash: budget.flash * feel.flash_scale,
        ..budget
    }
}

/// What is left of a pulse `age` seconds after it started, 1 falling to 0
/// over `secs`: fast at first, then gently.
fn left(age: f32, secs: f32) -> f32 {
    if secs <= 0.0 || age < 0.0 || age >= secs {
        return 0.0;
    }
    let left = 1.0 - age / secs;
    left * left
}

/// Where a shake of `strength` lasting `secs` has thrown the view `age`
/// seconds in, across the screen and up it, in world units. Never further
/// than `strength` either way, and back where it was by `secs`.
pub fn shake_offset(feel: &ResponseFeel, strength: f32, secs: f32, age: f32) -> Vec2 {
    let turn = TAU * feel.shake_hz * age;
    // Two rates that do not line up, so it does not trace the same loop.
    strength * left(age, secs) * Vec2::new(turn.sin(), (1.3 * turn + 1.0).cos())
}

/// How solid a flash of `strength` lasting `secs` is `age` seconds in. It
/// only fades: one pulse, never a flicker.
pub fn flash_alpha(strength: f32, secs: f32, age: f32) -> f32 {
    strength * left(age, secs)
}

pub struct ResponsePlugin;

impl Plugin for ResponsePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pulses>()
            .add_message::<Erupted>()
            .add_systems(Startup, setup)
            .add_systems(Update, respond);
    }
}

/// The sheet of colour over the scene that a flash shows on.
#[derive(Component)]
struct Flash;

#[derive(Clone, Copy)]
struct Pulse {
    strength: f32,
    secs: f32,
    age: f32,
}

impl Pulse {
    /// How much of its strength it still has.
    fn left(&self) -> f32 {
        self.strength * left(self.age, self.secs)
    }

    fn over(&self) -> bool {
        self.age >= self.secs
    }
}

#[derive(Resource, Default)]
struct Pulses {
    /// Bursts thrown whose first piece has not left yet.
    waiting: Vec<Erupted>,
    shake: Option<Pulse>,
    flash: Option<(Pulse, Color)>,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Flash,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            height: Val::Percent(0.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
        Pickable::IGNORE,
    ));
}

fn respond(
    time: Res<Time>,
    game: Res<Game>,
    shower: Res<Shower>,
    view: Res<SceneView>,
    tokens: Res<Tokens>,
    importance: Res<Importance>,
    mut erupted: MessageReader<Erupted>,
    mut pulses: ResMut<Pulses>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    flash: Single<(&mut Node, &mut BackgroundColor), With<Flash>>,
) {
    let dt = time.delta_secs();
    let feel = &shower.response;
    let pulses = &mut *pulses;

    if let Some(pulse) = &mut pulses.shake {
        pulse.age += dt;
    }
    if let Some((pulse, _)) = &mut pulses.flash {
        pulse.age += dt;
    }

    pulses.waiting.extend(erupted.read().map(|burst| Erupted {
        in_secs: burst.in_secs,
        best: burst.best,
    }));
    let mut due = Vec::new();
    pulses.waiting.retain_mut(|burst| {
        burst.in_secs -= dt;
        if burst.in_secs <= 0.0 {
            due.push(burst.best);
        }
        burst.in_secs > 0.0
    });
    for best in due {
        // A weaker response does not cut short what is left of a stronger one.
        let wanted = strength(feel, &importance, &game.data.gear, best);
        let shaking = pulses.shake.map_or(0.0, |pulse| pulse.left());
        if wanted.shake > shaking {
            pulses.shake = Some(Pulse {
                strength: wanted.shake,
                secs: wanted.shake_secs,
                age: 0.0,
            });
        }
        let flashing = pulses.flash.map_or(0.0, |(pulse, _)| pulse.left());
        if wanted.flash > flashing {
            let colour = match best {
                Some(tier) => rarity_colour(&game.data.gear.rarities[tier]).into(),
                None => shower.coin_colour.0,
            };
            let pulse = Pulse {
                strength: wanted.flash,
                secs: wanted.flash_secs,
                age: 0.0,
            };
            pulses.flash = Some((pulse, colour));
        }
    }

    if pulses.shake.is_some_and(|pulse| pulse.over()) {
        pulses.shake = None;
    }
    if pulses.flash.is_some_and(|(pulse, _)| pulse.over()) {
        pulses.flash = None;
    }

    let home = camera_home();
    let offset = pulses.shake.map_or(Vec2::ZERO, |pulse| {
        shake_offset(feel, pulse.strength, pulse.secs, pulse.age)
    });
    let wanted = home.translation + home.rotation * offset.extend(0.0);
    if camera.translation != wanted {
        camera.translation = wanted;
    }

    // The flash covers the scene's share of the window and none of the menus.
    let (mut node, mut background) = flash.into_inner();
    let colour = pulses.flash.map_or(Color::NONE, |(pulse, colour)| {
        colour.with_alpha(flash_alpha(pulse.strength, pulse.secs, pulse.age))
    });
    if background.0 != colour {
        background.0 = colour;
    }
    let height = Val::Percent(tokens.layout.share(view.expanded()) * 100.0);
    if pulses.flash.is_some() && node.height != height {
        node.height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::drops::{Kind, best_tier};
    use crate::fx::importance::Level;
    use crate::sim::data::Data;

    fn feel() -> ResponseFeel {
        Shower::load().response
    }

    /// The response numbers can be used as they are: scales that are not
    /// negative, and a flash too short and too faint to hide what it is
    /// over.
    #[test]
    fn the_response_numbers_are_usable() {
        let (feel, importance, data) = (feel(), Importance::load(), Data::load());
        assert!(feel.shake_scale >= 0.0 && feel.flash_scale >= 0.0 && feel.shake_hz > 0.0);
        for tier in (0..data.gear.rarities.len()).map(Some).chain([None]) {
            let wanted = strength(&feel, &importance, &data.gear, tier);
            assert!(wanted.flash <= 0.5 && wanted.shake <= 0.5, "{tier:?}");
            assert!(wanted.shake_secs <= 1.0 && wanted.flash_secs <= 1.0, "{tier:?}");
        }
    }

    /// The screen answers a burst with its best piece's level and nothing
    /// of its own: changing a level's budget changes every burst at it.
    #[test]
    fn a_burst_is_answered_at_its_level() {
        let (feel, data) = (feel(), Data::load());
        let mut importance = Importance::load();
        let rare = data.gear.rarities.iter().position(|r| r.name == "Rare").unwrap();
        let level = importance.rarity(&data.gear.rarities[rare]);
        assert_eq!(strength(&feel, &importance, &data.gear, Some(rare)), *importance.budget(level));
        let mut budget = *importance.budget(level);
        budget.shake += 0.1;
        budget.flash_secs += 0.1;
        importance.set(level, budget);
        for tier in 0..data.gear.rarities.len() {
            let at = importance.rarity(&data.gear.rarities[tier]) == level;
            let wanted = strength(&feel, &importance, &data.gear, Some(tier));
            assert_eq!(wanted == budget, at, "{}", data.gear.rarities[tier].name);
        }
    }

    /// The response is sized to the best piece in the burst, not to how many
    /// pieces there are.
    #[test]
    fn the_best_piece_decides_not_the_count() {
        let (feel, importance, data) = (feel(), Importance::load(), Data::load());
        let rare = data.gear.rarities.iter().position(|r| r.name == "Rare").unwrap();
        assert_eq!(best_tier(&[Kind::Coin; 40]), None);
        assert_eq!(best_tier(&[]), None);
        let mut few = vec![Kind::Coin, Kind::Gear(rare), Kind::Gear(0)];
        assert_eq!(best_tier(&few), Some(rare));
        let small = strength(&feel, &importance, &data.gear, best_tier(&few));
        few.extend([Kind::Coin; 40]);
        few.extend([Kind::Gear(0); 5]);
        assert_eq!(strength(&feel, &importance, &data.gear, best_tier(&few)), small);
    }

    /// A better piece is never answered more weakly, and a fountain with a
    /// Rare in it is answered more strongly, both ways, than the same
    /// fountain without: with coins alone, or with any lower tier.
    #[test]
    fn a_rare_is_felt() {
        let (feel, importance, data) = (feel(), Importance::load(), Data::load());
        let tiers = data.gear.rarities.len();
        let of = |best| strength(&feel, &importance, &data.gear, best);
        for tier in 1..tiers {
            let (lower, higher) = (of(Some(tier - 1)), of(Some(tier)));
            assert!(lower.shake <= higher.shake && lower.flash <= higher.flash);
        }
        assert!(of(None).shake <= of(Some(0)).shake && of(None).flash <= of(Some(0)).flash);
        let rare = data.gear.rarities.iter().position(|r| r.name == "Rare").unwrap();
        for lesser in (0..rare).map(Some).chain([None]) {
            assert!(of(lesser).shake < of(Some(rare)).shake, "{lesser:?}");
            assert!(of(lesser).flash < of(Some(rare)).flash, "{lesser:?}");
        }
    }

    /// Either scale at zero turns its half off for every burst, and leaves
    /// the other half alone.
    #[test]
    fn each_scale_turns_its_response_off() {
        let (importance, data) = (Importance::load(), Data::load());
        let top = Some(data.gear.rarities.len() - 1);
        let full = strength(&feel(), &importance, &data.gear, top);
        assert!(full.shake > 0.0 && full.flash > 0.0);

        let mut still = feel();
        still.shake_scale = 0.0;
        let wanted = strength(&still, &importance, &data.gear, top);
        assert_eq!((wanted.shake, wanted.flash), (0.0, full.flash));
        for step in 0..60 {
            let age = step as f32 / 120.0;
            assert_eq!(shake_offset(&still, wanted.shake, wanted.shake_secs, age), Vec2::ZERO);
        }

        let mut dark = feel();
        dark.flash_scale = 0.0;
        let wanted = strength(&dark, &importance, &data.gear, top);
        assert_eq!((wanted.shake, wanted.flash), (full.shake, 0.0));
        for step in 0..60 {
            assert_eq!(flash_alpha(wanted.flash, wanted.flash_secs, step as f32 / 120.0), 0.0);
        }
    }

    /// A shake moves the view, never further than its strength, and puts it
    /// back; a flash starts at its strength and only fades.
    #[test]
    fn a_response_stays_within_its_strength_and_ends() {
        let feel = feel();
        let high = *Importance::load().budget(Level::High);
        let (shake_secs, flash_secs) = (high.shake_secs, high.flash_secs);
        let (mut furthest, mut last_alpha) = (0.0_f32, f32::MAX);
        for step in 0..240 {
            let age = step as f32 / 240.0 * shake_secs.max(flash_secs);
            let offset = shake_offset(&feel, 0.3, shake_secs, age);
            assert!(offset.x.abs() <= 0.3 && offset.y.abs() <= 0.3, "{offset} at {age}");
            furthest = furthest.max(offset.length());
            let alpha = flash_alpha(0.25, flash_secs, age);
            assert!(alpha <= last_alpha, "the flash grew at {age}");
            last_alpha = alpha;
        }
        assert!(furthest > 0.15, "the shake hardly moved: {furthest}");
        assert_eq!(flash_alpha(0.25, flash_secs, 0.0), 0.25);
        assert_eq!(shake_offset(&feel, 0.3, shake_secs, shake_secs), Vec2::ZERO);
        assert_eq!(shake_offset(&feel, 0.3, shake_secs, shake_secs + 1.0), Vec2::ZERO);
        assert_eq!(flash_alpha(0.25, flash_secs, flash_secs), 0.0);
    }
}
