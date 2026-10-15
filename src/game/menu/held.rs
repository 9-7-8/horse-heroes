//! The piece in the hand (#363): while a piece is dragged it comes out of
//! its cell, which shows it gone, and dangles from the pointer on a short
//! string, swinging (`fx::dangle`). Let go, it flies the short way into the
//! cell it went to, or back into its own. Presentation only: the drag is the
//! shell's (#228), and nothing here waits to change the game. How it works:
//! docs/item-card.md, "Dragging".

use bevy::{
    picking::pointer::{PointerId, PointerLocation},
    prelude::*,
};

use super::picture::{draw, square};
use super::shell::Shell;
use super::tokens::Tokens;
use crate::fx::dangle::Dangle;
use crate::game::{Game, Piece};

/// The piece in the hand, while there is one, and its flight once let go.
#[derive(Resource, Default)]
pub struct Held {
    /// The node that hangs from the pointer, and the piece it shows.
    node: Option<(Entity, Piece)>,
    dangle: Dangle,
    /// Where the node's middle is now.
    at: Vec2,
    /// Let go, and not yet flying: the cell it goes into, and the piece
    /// that is in that cell now.
    landing: Option<(Entity, Piece)>,
    flight: Option<Flight>,
}

/// A piece let go, on its way into a cell.
struct Flight {
    to: Entity,
    piece: Piece,
    from: Vec2,
    angle: f32,
    /// How far it has come, 0 to 1.
    done: f32,
}

impl Held {
    /// The piece whose cells show it gone: the one in the hand, or the one
    /// on its way into a cell, which shows it once it is there.
    pub fn lifted(&self, shell: &Shell) -> Option<Piece> {
        shell
            .dragging()
            .map(|drag| drag.piece)
            .or(self.landing.map(|(_, piece)| piece))
            .or(self.flight.as_ref().map(|flight| flight.piece))
    }

    /// The piece was let go: it goes into `cell`, where `piece` is now.
    pub fn land(&mut self, cell: Entity, piece: Piece) {
        self.landing = Some((cell, piece));
    }
}

/// The middle of a cell, in the window's pixels.
fn middle_of(node: &ComputedNode, transform: &UiGlobalTransform) -> Vec2 {
    let (_, _, middle) = transform.to_scale_angle_translation();
    middle * node.inverse_scale_factor()
}

/// Lift the dragged piece into the hand, swing it from the pointer, and fly
/// it into its cell once let go.
pub fn hold(
    mut commands: Commands,
    time: Res<Time<Real>>,
    tokens: Res<Tokens>,
    game: Res<Game>,
    shell: Res<Shell>,
    mut held: ResMut<Held>,
    pointers: Query<(&PointerId, &PointerLocation)>,
    cells: Query<(&ComputedNode, &UiGlobalTransform)>,
    mut nodes: Query<(&mut Node, &mut UiTransform)>,
    children: Query<&Children>,
    pickable: Query<(), With<Pickable>>,
) {
    let t = &*tokens;
    let swing = &t.swing;
    let dt = time.delta_secs();
    let held = &mut *held;
    let pointer = pointers
        .iter()
        .find(|(id, _)| **id == PointerId::Mouse)
        .and_then(|(_, location)| location.location())
        .map(|location| location.position);
    // From the point it hangs by to the middle of the node, which is what
    // turns: the string and the piece under it.
    let reach = (swing.side + swing.hang) / 2.0;

    // A drag begins: the piece comes out of its cell into the hand.
    if let Some(drag) = shell.dragging()
        && held.node.is_none_or(|(_, piece)| piece != drag.piece || held.flight.is_some())
        && let Some(gear) = game.piece(drag.piece)
    {
        if let Some((node, _)) = held.node.take() {
            commands.entity(node).despawn();
        }
        held.flight = None;
        held.landing = None;
        held.dangle = Dangle::default();
        let gear_table = &game.data.gear;
        let shape = gear_table.picture(gear.slot);
        let tier = &gear_table.rarities[gear.rarity];
        held.at = pointer.unwrap_or_default() + Vec2::new(0.0, reach);
        let node = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(held.at.x - swing.side / 2.0),
                    top: Val::Px(held.at.y - reach),
                    width: Val::Px(swing.side),
                    height: Val::Px(swing.side + swing.hang),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                UiTransform::default(),
                // Under the pointer's card, so the preview of the drop is
                // never behind the piece.
                GlobalZIndex(3),
                Pickable::IGNORE,
            ))
            .with_children(|root| {
                root.spawn((
                    Node {
                        width: Val::Px(2.0 * t.size.border),
                        height: Val::Px(swing.hang),
                        flex_shrink: 0.0,
                        ..default()
                    },
                    BackgroundColor(t.colour.text_dim.0),
                ));
                root.spawn((square(t, swing.side), BorderColor::all(t.colour.text_dim.0)))
                    .with_children(|square| draw(square, t, shape, Some(tier), swing.side));
            })
            .id();
        held.node = Some((node, drag.piece));
    }

    let Some((node, _)) = held.node else {
        return;
    };
    // The pointer passes through the piece to what is under it.
    for part in children.iter_descendants(node) {
        if !pickable.contains(part) {
            commands.entity(part).insert(Pickable::IGNORE);
        }
    }

    // Let go: it flies from where it hangs into its cell.
    if let Some((to, piece)) = held.landing.take() {
        held.flight = Some(Flight {
            to,
            piece,
            from: held.at,
            angle: held.dangle.angle,
            done: 0.0,
        });
    }

    let (at, angle) = if let Some(flight) = &mut held.flight {
        flight.done = (flight.done + dt / swing.fly).min(1.0);
        // The piece's middle comes to the cell's; the string hangs above it.
        let to = cells
            .get(flight.to)
            .ok()
            .filter(|(cell, _)| !cell.is_empty())
            .map_or(flight.from, |(cell, transform)| middle_of(cell, transform))
            - Vec2::new(0.0, swing.hang / 2.0);
        let eased = 1.0 - (1.0 - flight.done).powi(3);
        (flight.from.lerp(to, eased), flight.angle * (1.0 - eased))
    } else if shell.dragging().is_some() {
        let Some(pointer) = pointer else {
            return;
        };
        held.dangle.update(swing, (pointer.x, pointer.y), dt);
        let angle = held.dangle.angle;
        (pointer + reach * Vec2::new(angle.sin(), angle.cos()), angle)
    } else {
        // Neither held nor let go: the drag ended with nothing told here.
        commands.entity(node).despawn();
        held.node = None;
        return;
    };
    held.at = at;

    if held.flight.as_ref().is_some_and(|flight| flight.done >= 1.0) {
        commands.entity(node).despawn();
        held.node = None;
        held.flight = None;
        return;
    }
    if let Ok((mut layout, mut turn)) = nodes.get_mut(node) {
        layout.left = Val::Px(at.x - swing.side / 2.0);
        layout.top = Val::Px(at.y - reach);
        // A piece out to the right leans back towards the pointer: turned
        // the other way to the clock.
        turn.rotation = Rot2::radians(-angle);
    }
}
