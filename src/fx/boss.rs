//! The final enemy, announced (#19): while its fight holds for the sim's
//! `final_approach`, it walks up the trail into its place and rears; over
//! the scene its name and element come up, and its own wide health bar fills
//! as it comes, then stays readable through the whole fight, dropping as the
//! small bars do (`impact::trail_after`). After the kill the emptied bar
//! lingers a moment and fades. Every number is in `assets/data/boss.ron`.
//! How it works: docs/boss-announce.md.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use serde::Deserialize;

use super::impact::{Impacts, trail_after};
use crate::game::menu::tokens::{Hex, Tokens};
use crate::game::{Game, Screen};
use crate::sim::expedition::Outcome;

/// The numbers in `boss.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Announce {
    pub walk: f32,
    pub rear_at: f32,
    pub rear: f32,
    pub bar_width: f32,
    pub bar_height: f32,
    pub top: f32,
    pub beside_log: f32,
    pub name_size: f32,
    pub linger: f32,
    pub fill: Hex,
    pub frame: Hex,
}

impl Announce {
    pub fn load() -> Announce {
        ron::from_str(include_str!("../../assets/data/boss.ron"))
            .unwrap_or_else(|e| panic!("assets/data/boss.ron: {e}"))
    }

    /// Where the final enemy is in its approach, with `left` of it to go
    /// (1 as the fight opens, 0 at the first blow): how far back up the
    /// trail it is, along +x, and how far it leans back. It slows into its
    /// place by `rear_at` of the way, then rears and settles by the end.
    pub fn approach(&self, left: f32) -> (f32, f32) {
        let walked = 1.0 - left.clamp(0.0, 1.0);
        let step = (walked / self.rear_at).min(1.0);
        let back = self.walk * (1.0 - step) * (1.0 - step);
        let lean = if walked <= self.rear_at {
            0.0
        } else {
            -self.rear * (std::f32::consts::PI * (walked - self.rear_at) / (1.0 - self.rear_at)).sin()
        };
        (back, lean)
    }
}

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Announce::load())
            .add_systems(Startup, spawn_banner)
            .add_systems(Update, banner.after(crate::game::advance));
    }
}

/// The banner and what it last showed: the health share, where its
/// trailing piece is, seconds since the health last dropped, the full
/// health it is a share of, and seconds since the kill, while it lingers.
#[derive(Component)]
struct Banner {
    fill: f32,
    trail: f32,
    since: f32,
    max: i32,
    gone: Option<f32>,
    name: Entity,
    element: Entity,
    fill_node: Entity,
    trail_node: Entity,
}

fn spawn_banner(mut commands: Commands, announce: Res<Announce>, impacts: Res<Impacts>, tokens: Res<Tokens>) {
    let piece = |colour: Color| {
        (
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(colour),
            Pickable::IGNORE,
        )
    };
    let trail_node = commands.spawn(piece(impacts.trail.0)).id();
    let fill_node = commands.spawn(piece(announce.fill.0)).id();
    let bar = commands
        .spawn((
            Node {
                width: Val::Px(announce.bar_width),
                height: Val::Px(announce.bar_height),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(impacts.back.0),
            BorderColor::all(announce.frame.0),
            Pickable::IGNORE,
        ))
        .add_children(&[trail_node, fill_node])
        .id();
    let words = |size: f32, colour: Color| {
        (
            Text::new(""),
            TextFont { font_size: FontSize::Px(size), ..default() },
            TextColor(colour),
            Pickable::IGNORE,
        )
    };
    let name = commands.spawn(words(announce.name_size, tokens.colour.text.0)).id();
    let element = commands.spawn(words(tokens.text.small, tokens.colour.text_dim.0)).id();
    commands
        .spawn((
            Banner { fill: 0.0, trail: 0.0, since: f32::INFINITY, max: 0, gone: None, name, element, fill_node, trail_node },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(announce.top),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(tokens.space.xs),
                ..default()
            },
            // Under the menus, as the small bars are.
            GlobalZIndex(-1),
            Visibility::Hidden,
            Pickable::IGNORE,
        ))
        .add_children(&[name, element, bar]);
}

/// The banner follows the final enemy from its approach to its kill.
#[allow(clippy::too_many_arguments)]
fn banner(
    time: Res<Time>,
    game: Res<Game>,
    announce: Res<Announce>,
    impacts: Res<Impacts>,
    tokens: Res<Tokens>,
    window: Single<&Window, With<PrimaryWindow>>,
    banner: Single<(&mut Banner, &mut Node, &mut Visibility)>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
    mut pieces: Query<&mut Node, Without<Banner>>,
) {
    let dt = time.delta_secs();
    let (mut banner, mut node, mut visibility) = banner.into_inner();
    let won = matches!(&game.screen, Screen::Trail(run) if matches!(run.outcome, Some(Outcome { won: true, .. })));
    // The fight's own state, or the kill lingering.
    let alpha = match game.boss() {
        Some((_, foe)) => {
            let walked = 1.0 - game.approach_left();
            let fill = (foe.health.max(0) as f32 / foe.max_health.max(1) as f32).clamp(0.0, 1.0) * walked;
            if foe.max_health != banner.max {
                banner.max = foe.max_health;
                banner.trail = fill;
                banner.since = f32::INFINITY;
                if let Ok((mut text, _)) = texts.get_mut(banner.name) {
                    text.0 = foe.name.clone();
                }
                if let Ok((mut text, mut colour)) = texts.get_mut(banner.element) {
                    text.0 = foe.element.name().to_string();
                    colour.0 = tokens.colour.of(foe.element);
                }
            } else if fill < banner.fill {
                banner.since = 0.0;
            } else {
                banner.since += dt;
            }
            banner.fill = fill;
            banner.gone = None;
            walked
        }
        None if won && banner.max != 0 && banner.gone.is_none_or(|secs| secs < announce.linger) => {
            let gone = banner.gone.map_or(0.0, |secs| secs + dt);
            banner.gone = Some(gone);
            if banner.fill > 0.0 {
                banner.since = 0.0;
            } else {
                banner.since += dt;
            }
            banner.fill = 0.0;
            1.0 - gone / announce.linger
        }
        None => {
            banner.max = 0;
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
            return;
        }
    };
    banner.trail = trail_after(&impacts, banner.trail, banner.fill, banner.since, dt);

    // Centred on the window, or as near it as clears the fight log.
    let width = window.width();
    let least = tokens.size.fight_log_width + announce.beside_log;
    let left = Val::Px(((width - announce.bar_width) / 2.0).max(least));
    if node.left != left {
        node.left = left;
    }
    for (piece, share) in [(banner.fill_node, banner.fill), (banner.trail_node, banner.trail)] {
        if let Ok(mut piece) = pieces.get_mut(piece) {
            let width = Val::Percent(share * 100.0);
            if piece.width != width {
                piece.width = width;
            }
        }
    }
    for entity in [banner.name, banner.element] {
        if let Ok((_, mut colour)) = texts.get_mut(entity) {
            let alpha = alpha.clamp(0.0, 1.0);
            if colour.0.alpha() != alpha {
                colour.0.set_alpha(alpha);
            }
        }
    }
    if *visibility != Visibility::Inherited {
        *visibility = Visibility::Inherited;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The numbers can be used as they are, and the approach starts back up
    /// the trail, only ever comes nearer, and ends in its place, upright.
    #[test]
    fn the_final_enemy_walks_into_its_place() {
        let announce = Announce::load();
        assert!(announce.walk > 0.0 && (0.0..1.0).contains(&announce.rear_at) && announce.rear > 0.0);
        assert!(announce.bar_width > announce.bar_height && announce.bar_height > 0.0);
        assert!(announce.name_size > 0.0 && announce.linger > 0.0 && announce.top >= 0.0);
        assert_eq!(announce.approach(1.0), (announce.walk, 0.0));
        let (back, lean) = announce.approach(0.0);
        assert!(back.abs() < 1e-5 && lean.abs() < 1e-5, "{back} {lean}");
        let mut was = f32::INFINITY;
        let mut reared = false;
        for step in 0..=100 {
            let (back, lean) = announce.approach(1.0 - step as f32 / 100.0);
            assert!(back <= was && back >= 0.0, "step {step}");
            assert!(lean <= 1e-5 && lean >= -announce.rear - 1e-5, "step {step}: {lean}");
            reared |= lean < -announce.rear * 0.5;
            was = back;
        }
        assert!(reared, "it never rears");
    }
}
