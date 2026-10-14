//! Damage numbers (#249): each hit that lands shows its damage, rising from
//! what it struck and fading. A horse's hit is its element's colour, an
//! enemy's hit on a horse another colour altogether; a bigger hit is a bigger
//! number, and hits on one thing close together are set side by side so a
//! flurry stays readable. Every feel number is in
//! `assets/data/damage_numbers.ron`; Damage numbers in Settings turns them
//! off.
//!
//! How big, how high and how solid a number is are plain arithmetic, so they
//! are unit-tested; the system below them only applies them. How it works:
//! docs/damage-numbers.md.

use bevy::color::Mix;
use bevy::prelude::*;
use serde::Deserialize;

use crate::game::Landed;
use crate::game::menu::shell::{Shell, Switch};
use crate::game::menu::tokens::{Hex, Tokens};
use crate::sim::expedition::{Hit, HitKind, Struck};

/// The numbers in `damage_numbers.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Feel {
    pub small: f32,
    pub large: f32,
    pub big_damage: i32,
    pub strong_scale: f32,
    pub weak_scale: f32,
    pub weak_alpha: f32,
    pub start_above: f32,
    pub rise: f32,
    pub life_secs: f32,
    pub fade_secs: f32,
    pub pop: f32,
    pub pop_secs: f32,
    pub spread: f32,
    pub row: usize,
    pub row_height: f32,
    pub most: usize,
    pub lighten: f32,
    pub physical: Hex,
    pub taken: Hex,
    pub shadow: Hex,
}

impl Feel {
    pub fn load() -> Feel {
        ron::from_str(include_str!("../../assets/data/damage_numbers.ron"))
            .unwrap_or_else(|e| panic!("assets/data/damage_numbers.ron: {e}"))
    }
}

/// How big a hit's number is, in pixels of text: `small` for a hit of 1,
/// growing with the square root of its damage to `large`, then bigger again
/// for a Strong hit and smaller for a weak one.
pub fn size(feel: &Feel, damage: i32, kind: HitKind) -> f32 {
    let share = ((damage.max(1) - 1) as f32 / (feel.big_damage - 1).max(1) as f32).min(1.0);
    let size = feel.small + (feel.large - feel.small) * share.sqrt();
    size * match kind {
        HitKind::Strong => feel.strong_scale,
        HitKind::Resisted | HitKind::Warded => feel.weak_scale,
        HitKind::Plain => 1.0,
    }
}

/// What a number says: a hit taken starts with a minus, a Strong one ends
/// with a "!".
pub fn words(hit: &Hit) -> String {
    match (hit.struck, hit.kind) {
        (Struck::Horse(_), _) => format!("-{}", hit.damage),
        (_, HitKind::Strong) => format!("{}!", hit.damage),
        _ => hit.damage.to_string(),
    }
}

/// How solid a number is `age` seconds in: whole, then fading over its last
/// `fade_secs`, and gone at the end of its life.
pub fn alpha(feel: &Feel, age: f32) -> f32 {
    if age >= feel.life_secs {
        return 0.0;
    }
    let left = feel.life_secs - age;
    if feel.fade_secs <= 0.0 { 1.0 } else { (left / feel.fade_secs).min(1.0) }
}

/// How far a number has risen `age` seconds in, in world units: quickly at
/// first, then slowing, to `rise` at the end of its life.
pub fn lift(feel: &Feel, age: f32) -> f32 {
    let t = (age / feel.life_secs).clamp(0.0, 1.0);
    feel.rise * (1.0 - (1.0 - t) * (1.0 - t))
}

/// How much bigger than its size a number is `age` seconds in: `pop` as it
/// appears, shrinking to 1.
pub fn pop(feel: &Feel, age: f32) -> f32 {
    if feel.pop_secs <= 0.0 || age >= feel.pop_secs {
        return 1.0;
    }
    feel.pop + (1.0 - feel.pop) * age / feel.pop_secs
}

/// Where the `n`th number on one thing, of those close together, is set from
/// the first, in pixels across and up the screen: the middle, then right,
/// then left, and on up a row.
pub fn place(feel: &Feel, n: usize) -> Vec2 {
    let row = feel.row.max(1);
    let (col, up) = (n % row, (n / row) % row);
    let side = if col == 0 {
        0.0
    } else if col % 2 == 1 {
        col.div_ceil(2) as f32
    } else {
        -((col / 2) as f32)
    };
    Vec2::new(side * feel.spread, up as f32 * feel.row_height)
}

/// A number's colour: an enemy's hit on a horse is `taken`; a horse's hit
/// is its element's colour, lightened, when it had an elemental part,
/// `physical` when not.
pub fn colour(feel: &Feel, tokens: &Tokens, hit: &Hit) -> Color {
    match (hit.struck, hit.element) {
        (Struck::Horse(_), _) => feel.taken.0,
        (Struck::Foe { .. }, Some(element)) => tokens.colour.of(element).mix(&Color::WHITE, feel.lighten),
        (Struck::Foe { .. }, None) => feel.physical.0,
    }
}

pub struct NumbersPlugin;

impl Plugin for NumbersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Feel::load())
            .init_resource::<Recent>()
            .add_systems(Update, show.after(crate::game::advance));
    }
}

/// A box wide and tall enough for any number, centred on where it is.
const BOX: Vec2 = Vec2::new(160.0, 80.0);

/// One number in the air, on its box; `text` is the text in it.
#[derive(Component)]
struct Number {
    text: Entity,
    at: Vec3,
    offset: Vec2,
    age: f32,
    size: f32,
    solid: f32,
    colour: Color,
}

/// The things struck lately: how many numbers each has had in its run, and
/// how long since the last.
#[derive(Resource, Default)]
struct Recent(Vec<(Struck, usize, f32)>);

#[allow(clippy::too_many_arguments)]
fn show(
    mut commands: Commands,
    time: Res<Time>,
    feel: Res<Feel>,
    tokens: Res<Tokens>,
    shell: Res<Shell>,
    mut recent: ResMut<Recent>,
    mut landed: MessageReader<Landed>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut numbers: Query<(Entity, &mut Number, &mut Node)>,
    mut texts: Query<(&mut TextFont, &mut TextColor, &mut TextShadow)>,
) {
    let dt = time.delta_secs();
    let on = shell.is_on(Switch::DamageNumbers);
    for (_, _, since) in &mut recent.0 {
        *since += dt;
    }
    // A run on one thing ends once its last number is half gone.
    let run_secs = feel.life_secs * 0.5;
    recent.0.retain(|(_, _, since)| *since < run_secs);

    let mut alive = 0;
    for (entity, mut number, _) in &mut numbers {
        number.age += dt;
        if !on || number.age >= feel.life_secs {
            commands.entity(entity).despawn();
        } else {
            alive += 1;
        }
    }

    for Landed { hit, at } in landed.read() {
        if !on {
            continue;
        }
        let n = match recent.0.iter_mut().find(|(struck, _, _)| *struck == hit.struck) {
            Some((_, count, since)) => {
                *since = 0.0;
                *count += 1;
                *count - 1
            }
            None => {
                recent.0.push((hit.struck, 1, 0.0));
                0
            }
        };
        let size = size(&feel, hit.damage, hit.kind);
        let solid = match hit.kind {
            HitKind::Resisted | HitKind::Warded => feel.weak_alpha,
            _ => 1.0,
        };
        let colour = colour(&feel, &tokens, hit);
        let text = commands
            .spawn((
                Text::new(words(hit)),
                TextFont {
                    font_size: FontSize::Px(size * pop(&feel, 0.0)),
                    ..default()
                },
                TextColor(colour.with_alpha(solid)),
                TextShadow {
                    offset: Vec2::splat(2.0),
                    color: feel.shadow.0,
                },
                TextLayout::no_wrap(),
                Pickable::IGNORE,
            ))
            .id();
        commands
            .spawn((
                Number {
                    text,
                    at: *at,
                    offset: place(&feel, n),
                    age: 0.0,
                    size,
                    solid,
                    colour,
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(BOX.x),
                    height: Val::Px(BOX.y),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    left: Val::Px(-BOX.x),
                    top: Val::Px(-BOX.y),
                    ..default()
                },
                // Under the menus, so a number never covers a control.
                GlobalZIndex(-1),
                Pickable::IGNORE,
            ))
            .add_child(text);
        alive += 1;
    }

    // Past `most`, the oldest go first.
    if alive > feel.most {
        let mut ages: Vec<(f32, Entity)> = numbers
            .iter()
            .filter(|(_, number, _)| number.age < feel.life_secs)
            .map(|(entity, number, _)| (number.age, entity))
            .collect();
        ages.sort_by(|a, b| b.0.total_cmp(&a.0));
        for (_, entity) in ages.into_iter().take(alive - feel.most) {
            commands.entity(entity).despawn();
        }
    }

    let (camera, view) = *camera;
    for (_, number, mut node) in &mut numbers {
        if number.age >= feel.life_secs {
            continue;
        }
        let rising = number.at + Vec3::Y * (feel.start_above + lift(&feel, number.age));
        let Ok(spot) = camera.world_to_viewport(view, rising) else {
            continue;
        };
        let spot = spot + Vec2::new(number.offset.x, -number.offset.y) - BOX / 2.0;
        node.left = Val::Px(spot.x);
        node.top = Val::Px(spot.y);
        if let Ok((mut font, mut colour, mut shadow)) = texts.get_mut(number.text) {
            let wanted = FontSize::Px(number.size * pop(&feel, number.age));
            if font.font_size != wanted {
                font.font_size = wanted;
            }
            let fade = alpha(&feel, number.age);
            colour.0 = number.colour.with_alpha(number.solid * fade);
            shadow.color = feel.shadow.0.with_alpha(feel.shadow.0.alpha() * fade);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::data::Element;

    /// The numbers can be used as they are: sizes that grow, a life with
    /// room to fade, a row and a cap.
    #[test]
    fn the_feel_numbers_are_usable() {
        let feel = Feel::load();
        assert!(0.0 < feel.small && feel.small < feel.large && feel.big_damage > 1);
        assert!(feel.strong_scale > 1.0 && feel.weak_scale <= 1.0);
        assert!((0.0..=1.0).contains(&feel.weak_alpha) && feel.weak_alpha > 0.3);
        assert!(feel.life_secs > 0.0 && feel.fade_secs <= feel.life_secs);
        assert!(feel.pop >= 1.0 && feel.pop_secs < feel.life_secs);
        assert!(feel.row >= 1 && feel.most >= 10);
        // Every number fits its box, popped and Strong: four glyphs of about
        // six tenths of the size each, "999!".
        let biggest = feel.large * feel.strong_scale * feel.pop;
        assert!(biggest * 0.6 * 4.0 < BOX.x && biggest * 1.4 < BOX.y, "{biggest}");
    }

    /// A bigger hit is never a smaller number, and stops growing at
    /// `big_damage`; Strong is bigger than plain, weak smaller.
    #[test]
    fn a_bigger_hit_is_a_bigger_number() {
        let feel = Feel::load();
        assert_eq!(size(&feel, 1, HitKind::Plain), feel.small);
        assert_eq!(size(&feel, 0, HitKind::Plain), feel.small);
        let mut last = 0.0;
        for damage in 1..=feel.big_damage * 3 {
            let now = size(&feel, damage, HitKind::Plain);
            assert!(now >= last, "{damage}");
            assert!(size(&feel, damage, HitKind::Strong) > now);
            assert!(size(&feel, damage, HitKind::Resisted) <= now);
            last = now;
        }
        assert_eq!(size(&feel, feel.big_damage, HitKind::Plain), feel.large);
        assert_eq!(size(&feel, feel.big_damage * 3, HitKind::Plain), feel.large);
        assert!(size(&feel, 10, HitKind::Plain) > size(&feel, 2, HitKind::Plain) + 2.0);
    }

    /// A number pops in, rises and slows, is whole until its fade, and is
    /// gone at the end of its life.
    #[test]
    fn a_number_rises_fades_and_ends() {
        let feel = Feel::load();
        assert_eq!(pop(&feel, 0.0), feel.pop);
        assert_eq!(pop(&feel, feel.pop_secs), 1.0);
        assert_eq!(alpha(&feel, 0.0), 1.0);
        assert!(alpha(&feel, feel.life_secs - feel.fade_secs) > 0.999);
        assert_eq!(alpha(&feel, feel.life_secs), 0.0);
        assert_eq!(lift(&feel, 0.0), 0.0);
        assert_eq!(lift(&feel, feel.life_secs), feel.rise);
        let (mut last_lift, mut last_alpha, mut last_step) = (0.0, 1.0, f32::MAX);
        for step in 1..=100 {
            let age = step as f32 / 100.0 * feel.life_secs;
            let (up, solid) = (lift(&feel, age), alpha(&feel, age));
            assert!(up >= last_lift && up - last_lift <= last_step + 1e-5, "at {age}");
            assert!(solid <= last_alpha, "at {age}");
            (last_step, last_lift, last_alpha) = (up - last_lift, up, solid);
        }
    }

    /// Hits on one thing close together are each set somewhere of their own,
    /// a row at a time, the first in the middle.
    #[test]
    fn a_flurry_is_set_side_by_side() {
        let feel = Feel::load();
        assert_eq!(place(&feel, 0), Vec2::ZERO);
        let spots: Vec<Vec2> = (0..feel.row * feel.row).map(|n| place(&feel, n)).collect();
        for (i, a) in spots.iter().enumerate() {
            for b in &spots[i + 1..] {
                assert!(a.distance(*b) >= feel.spread.min(feel.row_height) - 1e-3, "{a} {b}");
            }
        }
        // Across a row, it is the middle and either side of it.
        let row: Vec<f32> = (0..feel.row).map(|n| place(&feel, n).x).collect();
        assert!(row.iter().sum::<f32>().abs() <= feel.spread);
    }

    /// An enemy's hit on a horse is never mistaken for a horse's: it has a
    /// colour of its own, unlike every element's and the physical one.
    #[test]
    fn a_hit_taken_is_told_from_a_hit_dealt() {
        let (feel, tokens) = (Feel::load(), Tokens::load());
        let hit = |struck, element| Hit {
            struck,
            damage: 5,
            element,
            kind: HitKind::Plain,
        };
        let foe = Struck::Foe { slot: 0, max_health: 60 };
        let taken = colour(&feel, &tokens, &hit(Struck::Horse(1), None));
        let mut dealt = vec![colour(&feel, &tokens, &hit(foe, None))];
        for element in Element::ALL {
            let shown = colour(&feel, &tokens, &hit(foe, Some(element)));
            dealt.push(shown);
        }
        // Apart by hue as well, not only by a shade (Fire is red).
        let hue = |c: Color| Hsla::from(c).hue;
        for other in dealt {
            let apart = (hue(taken) - hue(other)).abs();
            assert!(apart.min(360.0 - apart) >= 40.0, "{other:?}");
        }
        let strong = Hit { kind: HitKind::Strong, ..hit(foe, Some(Element::Fire)) };
        assert_eq!(words(&strong), "5!");
        assert_eq!(words(&hit(foe, None)), "5");
        assert_eq!(words(&hit(Struck::Horse(0), None)), "-5");
    }
}
