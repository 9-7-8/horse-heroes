//! The stand-in picture of a piece of gear (#286), wherever a piece is shown:
//! the flat shape of its slot, in its rarity's colour, with the rarity's
//! shape from the loot shower stamped on it as seen from above. An empty slot
//! is the outline of what goes in it. All of it is drawn from `gear.ron`, so
//! nothing depends on it and art replaces it later. How it works:
//! docs/item-card.md, "The picture".

use std::collections::HashMap;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::held::Held;
use super::shell::Shell;
use super::tokens::Tokens;
use crate::fx::drops::{rarity_colour, rarity_fill};
use crate::game::{Game, Piece};
use crate::sim::data::{Data, PictureDef, RarityDef, RarityLook, Slot, Solid};
use crate::sim::look::{Layer, Look, paint};

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
    /// The side of its square.
    side: f32,
    shown: Option<Option<(usize, Option<(usize, Look)>)>>,
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
    picture_of(t, of, t.size.picture)
}

/// The same at another size: a tile of the Inventory is mostly its picture.
pub fn picture_of(t: &Tokens, of: Piece, side: f32) -> impl Bundle + use<> {
    (Picture { of: Of::At(of), side, shown: None }, square(t, side))
}

impl Picture {
    /// Make the square the picture of another piece: a tile of the Inventory
    /// is of whichever carried piece of its type is at its place (#377). It
    /// is drawn again when what it shows changes.
    pub fn aim(&mut self, of: Piece) {
        if !matches!(self.of, Of::At(now) if now == of) {
            self.of = Of::At(of);
        }
    }
}

/// The square of the picture of the piece the showing report dropped. It is
/// away while the party does not have that piece.
pub fn reported_picture(t: &Tokens) -> impl Bundle + use<> {
    (Picture { of: Of::Reported, side: t.size.picture, shown: None }, square(t, t.size.picture))
}

/// The square by itself, for `draw` to fill.
pub fn square(t: &Tokens, side: f32) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Px(side),
            height: Val::Px(side),
            border_radius: BorderRadius::all(Val::Px(t.size.radius)),
            flex_shrink: 0.0,
            ..default()
        },
        // Its own ground, so the picture is the same in every state of the
        // control it is on.
        BackgroundColor(t.colour.screen.0),
    )
}

/// The pixels a look's image has for each of the window's, so it stays
/// sharp on a screen that scales the window up.
const LOOK_DENSITY: f32 = 2.0;
/// The side of the image worn gear is painted from, in pixels.
const WORN_SIDE: u32 = 64;

/// Each look's image, once painted, by what it was painted for: the look,
/// the tier, and the slot and size of the picture (none for worn gear).
#[derive(Resource, Default)]
pub struct LookImages(HashMap<(Look, usize, Option<(usize, u32, u32)>), Handle<Image>>);

impl LookImages {
    /// The image of a piece's look for its menu picture at `side` (#361):
    /// its slot's shape in its rarity's colour, its pattern and its
    /// decoration.
    pub fn picture(
        &mut self,
        images: &mut Assets<Image>,
        t: &Tokens,
        data: &Data,
        (slot, tier, look): (usize, usize, Look),
        side: f32,
    ) -> Handle<Image> {
        let (wide, tall, round) = shape_box(t, data.gear.picture(Slot::ALL[slot]), side);
        let size = ((wide * LOOK_DENSITY).round() as u32, (tall * LOOK_DENSITY).round() as u32);
        let key = (look, tier, Some((slot, size.0, size.1)));
        self.0
            .entry(key)
            .or_insert_with(|| {
                let round = round.map(|r| r * LOOK_DENSITY);
                let ground = rarity_fill(&data.gear.rarities[tier]);
                images.add(look_image(data, ground, look, size, round, data.looks.picture_decoration))
            })
            .clone()
    }

    /// The image of a piece's look as the horse wears it (#359): square, with
    /// nothing cut away, stretched over the worn piece.
    pub fn worn(&mut self, images: &mut Assets<Image>, data: &Data, tier: usize, look: Look) -> Handle<Image> {
        self.0
            .entry((look, tier, None))
            .or_insert_with(|| {
                let size = (WORN_SIDE, WORN_SIDE);
                let ground = rarity_colour(&data.gear.rarities[tier]);
                images.add(look_image(data, ground, look, size, [0.0; 4], data.looks.worn_decoration))
            })
            .clone()
    }

    /// Let go of every image only the cache still holds (#375): the look of
    /// a piece sold, worn out of sight or drawn at a size no longer shown.
    /// Nothing else holding it, the image is freed, and is painted again if
    /// it is wanted again.
    pub fn forget_unheld(&mut self) {
        self.0.retain(|_, image| held_elsewhere(image));
    }
}

/// Whether anything but the one handle given holds this image.
fn held_elsewhere(image: &Handle<Image>) -> bool {
    match image {
        Handle::Strong(strong) => std::sync::Arc::strong_count(strong) > 1,
        Handle::Uuid(..) => true,
    }
}

/// Every few seconds, by the wall clock so a pop-up's pause does not stop
/// it, the cache lets go of what nothing shows.
pub fn forget_unheld_looks(mut looks: ResMut<LookImages>) {
    looks.forget_unheld();
}

/// A look painted into pixels: the `ground` (the rarity's fill in a menu, its
/// colour on the horse), the pattern's ink and the decoration's, and clear
/// outside the shape.
fn look_image(
    data: &Data,
    ground: Srgba,
    look: Look,
    (w, h): (u32, u32),
    round: [f32; 4],
    place: (f32, f32, f32),
) -> Image {
    let (w, h) = (w.max(1), h.max(1));
    let ink = |i: usize| {
        let text = &data.looks.inks[i % data.looks.inks.len()];
        Srgba::hex(text).unwrap_or_else(|e| panic!("looks.ron: ink `{text}`: {e}")).to_u8_array()
    };
    let colours = [
        [0, 0, 0, 0],
        ground.to_u8_array(),
        ink(look.pattern_ink),
        ink(look.decoration_ink),
    ];
    let pixels = paint(&data.looks, look, (w as usize, h as usize), round, place)
        .into_iter()
        .flat_map(|layer| match layer {
            Layer::Outside => colours[0],
            Layer::Ground => colours[1],
            Layer::Pattern => colours[2],
            Layer::Decoration => colours[3],
        })
        .collect();
    Image::new(
        Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

/// The shape of a slot's picture in a square of `side`: its width, its
/// height, and the rounding of its corners, in the window's pixels.
fn shape_box(t: &Tokens, shape: PictureDef, side: f32) -> (f32, f32, [f32; 4]) {
    // The room around the shape grows with the square.
    let room = side - 2.0 * t.size.picture_inset * side / t.size.picture;
    let (a, b, c, d) = shape.round;
    (shape.wide * room, shape.tall * room, [a, b, c, d].map(|r| r * room))
}

/// Draw the picture of a slot's gear into its square: of a piece of this
/// tier, painted with its look where it has one (#361), or with none the
/// outline of what the slot takes.
pub fn draw(
    square: &mut ChildSpawnerCommands,
    t: &Tokens,
    shape: PictureDef,
    tier: Option<&RarityDef>,
    look: Option<Handle<Image>>,
    side: f32,
) {
    let (wide, tall, [a, b, c, d]) = shape_box(t, shape, side);
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
        border_radius: BorderRadius::px(a, b, c, d),
        ..placed(wide, tall, (0.0, 0.0))
    };
    let Some(tier) = tier else {
        square.spawn((outline, BorderColor::all(t.colour.text_dim.0)));
        return;
    };
    let colour = Color::from(rarity_colour(tier));
    match look {
        // The look fills the whole shape; the border is the rarity's colour,
        // which may differ from its fill (#373).
        Some(image) => {
            let filled = Node { border: UiRect::ZERO, ..outline.clone() };
            square.spawn((filled, ImageNode::new(image)));
            square.spawn((outline, BorderColor::all(colour)));
        }
        None => {
            square.spawn((outline, BackgroundColor(rarity_fill(tier).into()), BorderColor::all(colour)));
        }
    }
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

/// The slot a square is to show, and the tier and look of the piece in it,
/// if it is about anything. The piece in the hand (#363) has gone from its
/// cell, which shows the slot empty until the piece is back in a cell.
fn wanted(game: &Game, of: Of, lifted: Option<Piece>) -> Option<(usize, Option<(usize, Look)>)> {
    let piece = match of {
        Of::At(piece) => piece,
        Of::Reported => game.reported_piece()?,
    };
    let tier = game
        .piece(piece)
        .map(|gear| (gear.rarity, gear.look))
        .filter(|_| lifted != Some(piece));
    match piece {
        Piece::Worn { horse, slot } => {
            (horse < game.party.len() && slot < Slot::ALL.len()).then_some((slot, tier))
        }
        Piece::Carried(_) => game.piece(piece).map(|gear| (gear.slot.index(), tier)),
    }
}

/// Draw each picture again when what it is of has changed.
pub fn show_pictures(
    mut commands: Commands,
    tokens: Res<Tokens>,
    game: Res<Game>,
    shell: Res<Shell>,
    held: Res<Held>,
    mut images: ResMut<Assets<Image>>,
    mut looks: ResMut<LookImages>,
    mut squares: Query<(Entity, &mut Picture, &mut Node)>,
) {
    let lifted = held.lifted(&shell);
    for (entity, mut picture, mut node) in &mut squares {
        let wanted = wanted(&game, picture.of, lifted);
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
        if let Some((slot, piece)) = wanted {
            let gear = &game.data.gear;
            let shape = gear.picture(Slot::ALL[slot]);
            let look = piece.map(|(tier, look)| {
                looks.picture(&mut images, &tokens, &game.data, (slot, tier, look), picture.side)
            });
            square.with_children(|square| {
                let tier = piece.map(|(tier, _)| &gear.rarities[tier]);
                draw(square, &tokens, shape, tier, look, picture.side);
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

    /// No tier fills its picture with white or near it, which reads as a
    /// blank ground rather than a piece (#373).
    #[test]
    fn no_picture_is_filled_white() {
        for rarity in Data::load().gear.rarities {
            let fill = rarity_fill(&rarity);
            let palest = fill.red.min(fill.green).min(fill.blue);
            assert!(palest < 0.8, "{}: fill {fill:?} is near white", rarity.name);
        }
    }

    /// A square shows the slot's outline while the slot is empty, the piece
    /// once one is worn, and nothing for a cargo that carries nothing.
    #[test]
    fn a_picture_is_of_the_piece_or_of_the_empty_slot() {
        let home = Game::back_with_one_piece;
        let game = home();
        let gear = game.pending_drop().unwrap().clone();
        let slot = gear.slot.index();
        let worn = Piece::Worn { horse: 0, slot };
        assert_eq!(wanted(&game, Of::At(worn), None), Some((slot, None)));
        assert_eq!(wanted(&game, Of::At(Piece::Carried(0)), None), Some((slot, Some((gear.rarity, gear.look)))));

        let mut on = home();
        on.party[0].gear[slot] = Some(gear.clone());
        assert_eq!(wanted(&on, Of::At(worn), None), Some((slot, Some((gear.rarity, gear.look)))));

        let mut away = home();
        away.act(crate::game::Action::Equip, crate::game::Hand::horse(0));
        assert_eq!(wanted(&away, Of::At(Piece::Carried(0)), None), None);
        // The report's piece is pictured where it is: carried, then worn.
        assert_eq!(wanted(&game, Of::Reported, None), Some((slot, Some((gear.rarity, gear.look)))));
        assert_eq!(wanted(&away, Of::Reported, None), Some((slot, Some((gear.rarity, gear.look)))));
        assert_eq!(wanted(&Game::new(6), Of::Reported, None), None);

        // The piece in the hand (#363) is gone from every cell that shows
        // it, the report's too, and from no other.
        let held = Some(Piece::Carried(0));
        assert_eq!(wanted(&game, Of::At(Piece::Carried(0)), held), Some((slot, None)));
        assert_eq!(wanted(&game, Of::Reported, held), Some((slot, None)));
        assert_eq!(wanted(&on, Of::At(worn), held), Some((slot, Some((gear.rarity, gear.look)))));
    }
}
