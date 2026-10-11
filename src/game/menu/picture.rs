//! The stand-in picture of a piece of gear (#286), wherever a piece is shown:
//! the flat shape of its slot, in its rarity's colour, with the rarity's
//! shape from the loot shower stamped on it as seen from above. An empty slot
//! is the outline of what goes in it. All of it is drawn from `gear.ron`, so
//! nothing depends on it and art replaces it later. How it works:
//! docs/item-card.md, "The picture".

use bevy::prelude::*;

use super::tokens::Tokens;
use crate::fx::drops::rarity_colour;
use crate::game::{Game, Piece};
use crate::sim::data::{PictureDef, RarityDef, RarityLook, Slot, Solid};

/// What a solid of a rarity's shape is, seen from above.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flat {
    Bar,
    Disc,
    /// A ring, with the width of its band.
    Hoop(f32),
}

/// One solid of a rarity's shape seen from above. Lengths are shares of the
/// shape's reach, so the whole stamp lies within 1 of its middle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stamp {
    pub flat: Flat,
    /// Its middle from the stamp's middle: along and across.
    pub at: (f32, f32),
    /// Its width and height before it is turned.
    pub size: (f32, f32),
    /// Degrees it is turned.
    pub turn: f32,
}

/// A rarity's shape as it lies flat, seen from above: what the menus stamp
/// on a piece of that tier. A prism is drawn as a disc.
pub fn stamp(look: &RarityLook) -> Vec<Stamp> {
    let reach = look.reach() / look.size;
    look.shape
        .iter()
        .map(|part| {
            let (flat, size) = match part.solid {
                Solid::Box(long, _, deep) => (Flat::Bar, (long, deep)),
                Solid::Ball(radius) | Solid::Prism(_, radius, _) => {
                    (Flat::Disc, (2.0 * radius, 2.0 * radius))
                }
                Solid::Ring(radius, tube) => {
                    let across = 2.0 * (radius + tube);
                    (Flat::Hoop(2.0 * tube / reach), (across, across))
                }
            };
            Stamp {
                flat,
                at: (part.at.0 / reach, part.at.2 / reach),
                size: (size.0 / reach, size.1 / reach),
                turn: part.turn,
            }
        })
        .collect()
}

/// The square a picture is drawn in, and the piece it is of where that is:
/// a slot shows its outline while it is empty. `shown` is what is drawn in
/// it now.
#[derive(Component)]
pub struct Picture {
    of: Of,
    shown: Option<Option<(usize, Option<usize>)>>,
}

/// What a picture is of: the piece at one place, or the piece of the report
/// the Journal shows, wherever that piece is now (#231).
#[derive(Clone, Copy)]
enum Of {
    At(Piece),
    Reported,
}

/// The square of a piece's picture. It keeps its size full or empty, and is
/// away only while there is no piece or slot for it to be about.
pub fn picture(t: &Tokens, of: Piece) -> impl Bundle + use<> {
    (Picture { of: Of::At(of), shown: None }, square(t))
}

/// The square of the picture of the piece the showing report dropped. It is
/// away while the party does not have that piece.
pub fn reported_picture(t: &Tokens) -> impl Bundle + use<> {
    (Picture { of: Of::Reported, shown: None }, square(t))
}

/// The square by itself, for `draw` to fill.
pub fn square(t: &Tokens) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Px(t.size.picture),
            height: Val::Px(t.size.picture),
            border_radius: BorderRadius::all(Val::Px(t.size.radius)),
            flex_shrink: 0.0,
            ..default()
        },
        // Its own ground, so the picture is the same in every state of the
        // control it is on.
        BackgroundColor(t.colour.screen.0),
    )
}

/// Draw the picture of a slot's gear into its square: of a piece of this
/// tier, or with none the outline of what the slot takes.
pub fn draw(
    square: &mut ChildSpawnerCommands,
    t: &Tokens,
    shape: PictureDef,
    tier: Option<&RarityDef>,
) {
    let side = t.size.picture;
    let room = side - 2.0 * t.size.picture_inset;
    let (wide, tall) = (shape.wide * room, shape.tall * room);
    let (a, b, c, d) = shape.round;
    let placed = |wide: f32, tall: f32, at: (f32, f32)| Node {
        position_type: PositionType::Absolute,
        left: Val::Px(side / 2.0 + at.0 - wide / 2.0),
        top: Val::Px(side / 2.0 + at.1 - tall / 2.0),
        width: Val::Px(wide),
        height: Val::Px(tall),
        ..default()
    };
    let outline = Node {
        border: UiRect::all(Val::Px(t.size.border)),
        border_radius: BorderRadius::px(a * room, b * room, c * room, d * room),
        ..placed(wide, tall, (0.0, 0.0))
    };
    let Some(tier) = tier else {
        square.spawn((outline, BorderColor::all(t.colour.text_dim.0)));
        return;
    };
    let colour = Color::from(rarity_colour(tier));
    square.spawn((outline, BackgroundColor(colour), BorderColor::all(colour)));
    // The tier's mark, cut out of the shape: told with the colour taken away.
    let half = t.size.picture_stamp * wide.min(tall) / 2.0;
    let cut = t.colour.screen.0;
    for stamp in stamp(&tier.look) {
        let (wide, tall) = (stamp.size.0 * half, stamp.size.1 * half);
        let mut node = placed(wide, tall, (stamp.at.0 * half, stamp.at.1 * half));
        let turned = UiTransform::from_rotation(Rot2::degrees(stamp.turn));
        match stamp.flat {
            Flat::Bar => {
                square.spawn((node, BackgroundColor(cut), turned));
            }
            Flat::Disc => {
                node.border_radius = BorderRadius::all(Val::Px(wide / 2.0));
                square.spawn((node, BackgroundColor(cut), turned));
            }
            Flat::Hoop(band) => {
                node.border_radius = BorderRadius::all(Val::Px(wide / 2.0));
                node.border = UiRect::all(Val::Px(band * half));
                square.spawn((node, BorderColor::all(cut), turned));
            }
        }
    }
}

/// The slot and the tier a square is to show, if it is about anything.
fn wanted(game: &Game, of: Of) -> Option<(usize, Option<usize>)> {
    let piece = match of {
        Of::At(piece) => piece,
        Of::Reported => game.reported_piece()?,
    };
    let tier = game.piece(piece).map(|gear| gear.rarity);
    match piece {
        Piece::Worn { horse, slot } => {
            (horse < game.party.len() && slot < Slot::ALL.len()).then_some((slot, tier))
        }
        Piece::Carried => game.pending_drop().map(|gear| (gear.slot.index(), tier)),
    }
}

/// Draw each picture again when what it is of has changed.
pub fn show_pictures(
    mut commands: Commands,
    tokens: Res<Tokens>,
    game: Res<Game>,
    mut squares: Query<(Entity, &mut Picture, &mut Node)>,
) {
    for (entity, mut picture, mut node) in &mut squares {
        let wanted = wanted(&game, picture.of);
        if picture.shown == Some(wanted) {
            continue;
        }
        picture.shown = Some(wanted);
        node.display = if wanted.is_some() {
            Display::Flex
        } else {
            Display::None
        };
        let mut square = commands.entity(entity);
        square.despawn_children();
        if let Some((slot, tier)) = wanted {
            let gear = &game.data.gear;
            let shape = gear.picture(Slot::ALL[slot]);
            square.with_children(|square| {
                draw(square, &tokens, shape, tier.map(|tier| &gear.rarities[tier]));
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::data::Data;

    /// No two tiers leave the same mark, so a piece's rarity is told from
    /// its picture with the colour taken away, and a mark fits on the piece.
    #[test]
    fn every_tier_stamps_a_mark_of_its_own() {
        let rarities = Data::load().gear.rarities;
        for (i, a) in rarities.iter().enumerate() {
            let mark = stamp(&a.look);
            assert_eq!(mark.len(), a.look.shape.len(), "{}", a.name);
            for part in &mark {
                let reach = part.at.0.hypot(part.at.1) + part.size.0.max(part.size.1) / 2.0;
                assert!(reach <= 1.0 + 1e-4, "{}: {part:?}", a.name);
                assert!(part.size.0 > 0.0 && part.size.1 > 0.0, "{}", a.name);
            }
            for b in &rarities[i + 1..] {
                assert_ne!(mark, stamp(&b.look), "{} and {}", a.name, b.name);
            }
        }
    }

    /// A square shows the slot's outline while the slot is empty, the piece
    /// once one is worn, and nothing for a cargo that carries nothing.
    #[test]
    fn a_picture_is_of_the_piece_or_of_the_empty_slot() {
        let home = || {
            (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some())
                .expect("no seed under 50 brings a drop home")
        };
        let game = home();
        let gear = game.pending_drop().unwrap().clone();
        let slot = gear.slot.index();
        let worn = Piece::Worn { horse: 0, slot };
        assert_eq!(wanted(&game, Of::At(worn)), Some((slot, None)));
        assert_eq!(wanted(&game, Of::At(Piece::Carried)), Some((slot, Some(gear.rarity))));

        let mut on = home();
        on.party[0].gear[slot] = Some(gear.clone());
        assert_eq!(wanted(&on, Of::At(worn)), Some((slot, Some(gear.rarity))));

        let mut away = home();
        away.act(crate::game::Action::Equip, crate::game::Hand::horse(0));
        assert_eq!(wanted(&away, Of::At(Piece::Carried)), None);
        // The report's piece is pictured where it is: carried, then worn.
        assert_eq!(wanted(&game, Of::Reported), Some((slot, Some(gear.rarity))));
        assert_eq!(wanted(&away, Of::Reported), Some((slot, Some(gear.rarity))));
        assert_eq!(wanted(&Game::new(6), Of::Reported), None);
    }
}
