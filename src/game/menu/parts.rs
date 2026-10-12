//! The parts every menu screen is made from. A screen composes these and
//! never draws a control of its own, so a control looks and behaves the same
//! wherever it appears (docs/philosophy/menus.md, "Every control has all its
//! states"). Every number and colour comes from `Tokens`.
//!
//! The four interactive parts (button, tab, list row, item card) share one
//! styling system, `style_parts`, and one rule for what each state looks
//! like, `paint`.

use bevy::{
    input_focus::{InputFocus, InputFocusVisible, tab_navigation::TabIndex},
    picking::hover::Hovered,
    prelude::*,
    text::FontSize,
    ui::{InteractionDisabled, Pressed, Selected},
    ui_widgets::Button,
};

use super::tokens::{Colours, Tokens};

/// An interactive part. `Selected` marks it chosen and `InteractionDisabled`
/// marks it unavailable; hover, press and focus are tracked by Bevy.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Button,
    Tab,
    Row,
    Card,
}

/// Text inside a part, recoloured with the part's state.
#[derive(Component, Clone, Copy)]
pub enum PartText {
    Main,
    Dim,
}

/// The bar that shows a part is selected.
#[derive(Component)]
pub struct Mark;

/// On a tab that has something the player has not seen (#232). It shows the
/// tab's `Attention` mark.
#[derive(Component)]
pub struct Unseen;

/// The dot that shows a tab has something not yet seen.
#[derive(Component)]
pub struct Attention;

/// On a part a hint is about (#291). It shows the part's `Pointer` mark.
#[derive(Component)]
pub struct Pointed;

/// The mark that shows a hint is about a part: a square stood on a corner.
#[derive(Component)]
pub struct Pointer;

/// The texts of a `hint`: the control it is about, that control's key, and
/// the line said beside them.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HintText {
    About,
    Key,
    Words,
}

/// The control of a hint that puts it away.
#[derive(Component)]
pub struct DismissHint;

/// The control of a hint that puts it away and every hint after it.
#[derive(Component)]
pub struct SkipHints;

/// The number inside a `counter`.
#[derive(Component)]
pub struct CounterValue;

/// The value on the right of a `row`.
#[derive(Component)]
pub struct RowValue;

#[derive(Component)]
pub struct ClosePanel;

/// On a control whose system is not built. It is drawn as unavailable and
/// can still be pressed: the press is what says so.
#[derive(Component)]
pub struct NotBuilt;

#[derive(Component)]
pub struct ConfirmAction;

#[derive(Component)]
pub struct ConfirmCancel;

/// The state a part is drawn in.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Look {
    pub hovered: bool,
    pub pressed: bool,
    pub selected: bool,
    pub unavailable: bool,
    pub focused: bool,
}

/// Draw a part in a fixed state whatever the pointer does. For the dev screen
/// that shows every state side by side.
#[derive(Component)]
pub struct ShowLook(pub Look);

/// What a part is drawn with.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Paint {
    pub fill: Color,
    pub border: Color,
    pub text: Color,
    pub dim: Color,
    pub mark: Color,
    pub ring: Color,
}

/// The one rule for how a state looks. Hover changes the fill; selected adds
/// the mark and an accent border and never uses the hover fill; pressed
/// inverts; unavailable empties the control; focus adds a ring to any of them.
pub fn paint(c: &Colours, look: Look) -> Paint {
    let ring = if look.focused { c.focus.0 } else { Color::NONE };
    if look.unavailable {
        return Paint {
            fill: Color::NONE,
            border: c.unavailable_border.0,
            text: c.unavailable_text.0,
            dim: c.unavailable_text.0,
            mark: if look.selected {
                c.unavailable_text.0
            } else {
                Color::NONE
            },
            ring,
        };
    }
    Paint {
        fill: if look.pressed {
            c.pressed.0
        } else if look.selected {
            c.selected.0
        } else if look.hovered {
            c.hover.0
        } else {
            c.control.0
        },
        border: if look.selected {
            c.accent.0
        } else if look.hovered || look.pressed {
            c.hover_border.0
        } else {
            c.control_border.0
        },
        text: if look.pressed {
            c.pressed_text.0
        } else {
            c.text.0
        },
        dim: if look.pressed {
            c.pressed_text.0
        } else {
            c.text_dim.0
        },
        mark: if look.selected {
            c.accent.0
        } else {
            Color::NONE
        },
        ring,
    }
}

fn sized(size: f32) -> TextFont {
    TextFont {
        font_size: FontSize::Px(size),
        ..default()
    }
}

pub fn text(t: &Tokens, words: &str) -> impl Bundle + use<> {
    (
        Text::new(words),
        sized(t.text.body),
        TextColor(t.colour.text.0),
        PartText::Main,
    )
}

/// Secondary text: a key, a unit, a column heading.
pub fn dim(t: &Tokens, words: &str) -> impl Bundle + use<> {
    (
        Text::new(words),
        sized(t.text.small),
        TextColor(t.colour.text_dim.0),
        PartText::Dim,
    )
}

pub fn title(t: &Tokens, words: &str) -> impl Bundle + use<> {
    (
        Text::new(words),
        sized(t.text.title),
        TextColor(t.colour.text.0),
    )
}

fn interactive(t: &Tokens, part: Part) -> impl Bundle + use<> {
    (
        part,
        Button,
        Hovered::default(),
        TabIndex(0),
        BackgroundColor(t.colour.control.0),
        BorderColor::all(t.colour.control_border.0),
        Outline::new(
            Val::Px(t.size.focus_ring),
            Val::Px(t.size.focus_gap),
            Color::NONE,
        ),
    )
}

/// The selected mark: under a button or tab, down the left of a row or card.
/// Out of the layout, so selecting never moves anything.
fn mark(t: &Tokens, part: Part) -> impl Bundle + use<> {
    let across = matches!(part, Part::Button | Part::Tab);
    (
        Mark,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            right: if across { Val::Px(0.0) } else { Val::Auto },
            top: if across { Val::Auto } else { Val::Px(0.0) },
            width: if across {
                Val::Auto
            } else {
                Val::Px(t.size.mark)
            },
            height: if across {
                Val::Px(t.size.mark)
            } else {
                Val::Auto
            },
            ..default()
        },
        BackgroundColor(Color::NONE),
    )
}

/// The attention mark: a round dot in the top corner of a tab, there while
/// the tab carries `Unseen`. Round where the selected mark is a bar, so the
/// two are told apart by shape, and out of the layout, so it moves nothing.
fn attention(t: &Tokens) -> impl Bundle + use<> {
    (
        Attention,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(t.space.xs),
            right: Val::Px(t.space.xs),
            width: Val::Px(t.size.attention),
            height: Val::Px(t.size.attention),
            border: UiRect::all(Val::Px(t.size.border)),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BackgroundColor(t.colour.attention.0),
        // Edged in the screen's colour, so it stands off any fill.
        BorderColor::all(t.colour.screen.0),
        Visibility::Hidden,
    )
}

/// The square on a corner that a hint and its control share (#291). It has
/// the attention mark's colour and neither its shape nor the selected
/// mark's, so the three are told apart with the colour taken away.
fn diamond(t: &Tokens) -> impl Bundle + use<> {
    (
        BackgroundColor(pointer_colour(&t.colour)),
        UiTransform::from_rotation(Rot2::degrees(45.0)),
    )
}

pub fn pointer_colour(c: &Colours) -> Color {
    c.attention.0
}

/// The hint's mark on a part: in its top left corner, clear of its label,
/// there while the part carries `Pointed`. Out of the layout, so it moves
/// nothing.
fn pointer(t: &Tokens) -> impl Bundle + use<> {
    (
        Pointer,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(t.space.s),
            left: Val::Px(t.space.s),
            width: Val::Px(t.size.pointer),
            height: Val::Px(t.size.pointer),
            ..default()
        },
        diamond(t),
        Visibility::Hidden,
    )
}

/// The key that works a control, printed on it. Takes no room when absent.
fn key_hint(t: &Tokens, key: Option<&str>) -> impl Bundle + use<> {
    (
        dim(t, &key.map(|k| format!("[{k}]")).unwrap_or_default()),
        Node {
            display: if key.is_some() {
                Display::Flex
            } else {
                Display::None
            },
            ..default()
        },
    )
}

/// A button. The label says what it will do with what.
pub fn button(t: &Tokens, label: &str, key: Option<&str>) -> impl Bundle + use<> {
    (
        interactive(t, Part::Button),
        Node {
            height: Val::Px(t.size.control_height),
            padding: UiRect::horizontal(Val::Px(t.space.l)),
            column_gap: Val::Px(t.space.m),
            border: UiRect::all(Val::Px(t.size.border)),
            border_radius: BorderRadius::all(Val::Px(t.size.radius)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_shrink: 0.0,
            ..default()
        },
        children![
            text(t, label),
            key_hint(t, key),
            mark(t, Part::Button),
            pointer(t)
        ],
    )
}

/// A button on a line that has less than a control's height: a hint's.
pub fn small_button(t: &Tokens, label: &str, key: Option<&str>) -> impl Bundle + use<> {
    (
        interactive(t, Part::Button),
        Node {
            height: Val::Px(t.size.hint_control),
            padding: UiRect::horizontal(Val::Px(t.space.m)),
            column_gap: Val::Px(t.space.s),
            border: UiRect::all(Val::Px(t.size.border)),
            border_radius: BorderRadius::all(Val::Px(t.size.radius)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_shrink: 0.0,
            ..default()
        },
        children![
            (
                Text::new(label),
                sized(t.text.small),
                TextColor(t.colour.text.0),
                PartText::Main,
            ),
            key_hint(t, key),
            mark(t, Part::Button)
        ],
    )
}

/// A place in the bar or a tab in a place: a label on a line, underlined
/// when it is the current one, with a dot in its corner while it carries
/// `Unseen`.
pub fn tab(t: &Tokens, label: &str, key: Option<&str>) -> impl Bundle + use<> {
    (
        interactive(t, Part::Tab),
        Node {
            height: Val::Px(t.size.control_height),
            padding: UiRect::horizontal(Val::Px(t.space.l)),
            column_gap: Val::Px(t.space.m),
            border: UiRect::bottom(Val::Px(t.size.border)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_shrink: 0.0,
            ..default()
        },
        children![
            text(t, label),
            key_hint(t, key),
            mark(t, Part::Tab),
            attention(t),
            pointer(t)
        ],
    )
}

/// One line of a list: a name on the left, with the key that chooses it if
/// it has one, and a value on the right.
pub fn row(t: &Tokens, name: &str, key: Option<&str>, value: &str) -> impl Bundle + use<> {
    (
        interactive(t, Part::Row),
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(t.size.row_height),
            padding: UiRect::horizontal(Val::Px(t.space.m + t.size.mark)),
            column_gap: Val::Px(t.space.m),
            border: UiRect::bottom(Val::Px(t.size.border)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            flex_shrink: 0.0,
            ..default()
        },
        children![
            (
                Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(t.space.m),
                    ..default()
                },
                children![text(t, name), key_hint(t, key)],
            ),
            (dim(t, value), RowValue),
            mark(t, Part::Row),
            pointer(t)
        ],
    )
}

/// The frame of an item card: what it is and what kind. The lines of the card
/// are added as children by whoever shows the item.
pub fn card(t: &Tokens, name: &str, kind: &str) -> impl Bundle + use<> {
    (
        interactive(t, Part::Card),
        Node {
            width: Val::Px(t.size.card_width),
            flex_direction: FlexDirection::Column,
            padding: UiRect::axes(Val::Px(t.space.m + t.size.mark), Val::Px(t.space.m)),
            row_gap: Val::Px(t.space.xs),
            border: UiRect::all(Val::Px(t.size.border)),
            border_radius: BorderRadius::all(Val::Px(t.size.radius)),
            flex_shrink: 0.0,
            ..default()
        },
        children![text(t, name), dim(t, kind), mark(t, Part::Card), pointer(t)],
    )
}

/// One square of a grid of things, such as a gear slot in the overview. It
/// takes the width of the column it is in and keeps its height full or
/// empty; its lines are added as children by whoever fills it.
pub fn cell(t: &Tokens) -> impl Bundle + use<> {
    (
        interactive(t, Part::Card),
        Node {
            width: Val::Percent(100.0),
            min_height: Val::Px(t.size.cell_height),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            // The mark is on the left only, so only that side makes room.
            padding: UiRect {
                left: Val::Px(t.space.m + t.size.mark),
                right: Val::Px(t.space.s),
                top: Val::Px(t.space.s),
                bottom: Val::Px(t.space.s),
            },
            row_gap: Val::Px(t.space.xs),
            border: UiRect::all(Val::Px(t.size.border)),
            border_radius: BorderRadius::all(Val::Px(t.size.radius)),
            overflow: Overflow::clip(),
            flex_shrink: 0.0,
            ..default()
        },
        children![mark(t, Part::Card), pointer(t)],
    )
}

/// A flat panel. Its contents are added as children.
pub fn panel(t: &Tokens) -> impl Bundle + use<> {
    (
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(t.space.m)),
            row_gap: Val::Px(t.space.s),
            border: UiRect::all(Val::Px(t.size.border)),
            ..default()
        },
        BackgroundColor(t.colour.panel.0),
        BorderColor::all(t.colour.panel_border.0),
    )
}

/// Detail about one thing, beside the list it came from. Opens with its
/// title and the control that closes it; the detail is added as children.
pub fn side_panel(t: &Tokens, heading: &str, width: f32) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Px(width),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(t.space.m)),
            row_gap: Val::Px(t.space.s),
            border: UiRect::left(Val::Px(t.size.mark)),
            flex_shrink: 0.0,
            ..default()
        },
        BackgroundColor(t.colour.panel.0),
        BorderColor::all(t.colour.panel_border.0),
        children![(
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(t.space.m),
                ..default()
            },
            children![
                title(t, heading),
                (button(t, "Close", Some("Esc")), ClosePanel)
            ],
        )],
    )
}

/// A labelled number in a box of fixed width, so the number can grow without
/// moving anything. The number starts beside its label: set against the far
/// edge it sat next to the following counter and read as that one (#252).
pub fn counter(t: &Tokens, label: &str, value: &str) -> impl Bundle + use<> {
    (
        Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(t.space.s),
            ..default()
        },
        children![
            dim(t, label),
            (
                Node {
                    width: Val::Px(t.size.counter_width),
                    justify_content: JustifyContent::FlexStart,
                    flex_shrink: 0.0,
                    ..default()
                },
                children![(text(t, value), CounterValue)],
            ),
        ],
    )
}

/// Where a fight of the trail stands against the party.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    Behind,
    Here,
    Ahead,
}

/// One fight in the trail tracker. Told apart by shape as well as colour: a
/// fight behind the party is solid, the one it is at is a thick ring with a
/// dot in it, one ahead is a thin outline, and the boss stands on a corner.
pub fn trail_mark(t: &Tokens, stop: Stop, boss: bool) -> impl Bundle + use<> {
    let size = if boss {
        t.size.trail_boss
    } else {
        t.size.trail_mark
    };
    let (fill, edge, width) = match stop {
        Stop::Behind => (t.colour.text.0, t.colour.text.0, t.size.border),
        Stop::Here => (Color::NONE, t.colour.accent.0, t.size.mark),
        Stop::Ahead => (Color::NONE, t.colour.text_dim.0, t.size.border),
    };
    let dot = size - 2.0 * width - 2.0 * t.space.xs;
    (
        Node {
            width: Val::Px(size),
            height: Val::Px(size),
            border: UiRect::all(Val::Px(width)),
            // On its corner the boss is wider than its box by this much.
            margin: UiRect::horizontal(Val::Px(if boss { size * 0.21 } else { 0.0 })),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_shrink: 0.0,
            ..default()
        },
        BackgroundColor(fill),
        BorderColor::all(edge),
        UiTransform::from_rotation(if boss {
            Rot2::degrees(45.0)
        } else {
            Rot2::IDENTITY
        }),
        children![(
            Node {
                width: Val::Px(dot),
                height: Val::Px(dot),
                ..default()
            },
            BackgroundColor(if stop == Stop::Here {
                t.colour.accent.0
            } else {
                Color::NONE
            }),
        )],
    )
}

/// Why a control cannot be used, said in place. The hollow square marks it
/// as a reason by shape as well as colour.
pub fn reason(t: &Tokens, why: &str) -> impl Bundle + use<> {
    (
        Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(t.space.s),
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(t.text.small * 0.7),
                    height: Val::Px(t.text.small * 0.7),
                    border: UiRect::all(Val::Px(t.size.border)),
                    flex_shrink: 0.0,
                    ..default()
                },
                BorderColor::all(t.colour.reason.0),
            ),
            (
                Text::new(why),
                sized(t.text.small),
                TextColor(t.colour.reason.0)
            ),
        ],
    )
}

/// A lesson's one line (#291): the mark its control carries, that control's
/// name and key, what is said about it, and the two controls that put it
/// away. It has one line's height whatever it says, so a line that is too
/// long is cut and never pushes anything down.
pub fn hint(t: &Tokens, about: &str, key: Option<&str>, words: &str) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(t.size.hint_height),
            align_items: AlignItems::Center,
            column_gap: Val::Px(t.space.m),
            padding: UiRect::horizontal(Val::Px(t.space.l)),
            flex_shrink: 0.0,
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(t.size.pointer),
                    height: Val::Px(t.size.pointer),
                    flex_shrink: 0.0,
                    ..default()
                },
                diamond(t),
            ),
            (text(t, about), HintText::About, TextLayout::no_wrap()),
            (
                dim(t, &key.map(|k| format!("[{k}]")).unwrap_or_default()),
                HintText::Key,
                TextLayout::no_wrap(),
            ),
            (
                Node {
                    flex_grow: 1.0,
                    min_width: Val::Px(0.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
                children![(text(t, words), HintText::Words, TextLayout::no_wrap())],
            ),
            (small_button(t, "Got it", Some("Esc")), DismissHint),
            (small_button(t, "Skip all hints", None), SkipHints),
        ],
    )
}

/// The one kind of modal: confirming something that cannot be taken back.
/// The action button names what it does; Cancel abandons it.
pub fn confirmation(t: &Tokens, heading: &str, body: &str, action: &str) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Px(t.size.confirm_width),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(t.space.l)),
            row_gap: Val::Px(t.space.m),
            border: UiRect::all(Val::Px(t.size.border)),
            flex_shrink: 0.0,
            ..default()
        },
        BackgroundColor(t.colour.panel.0),
        BorderColor::all(t.colour.hover_border.0),
        children![
            title(t, heading),
            text(t, body),
            (
                Node {
                    justify_content: JustifyContent::FlexEnd,
                    column_gap: Val::Px(t.space.m),
                    ..default()
                },
                children![
                    (button(t, "Cancel", Some("Esc")), ConfirmCancel),
                    (button(t, action, None), ConfirmAction),
                ],
            ),
        ],
    )
}

/// The layer a confirmation sits on: dims the screen behind it.
pub fn scrim(t: &Tokens) -> impl Bundle + use<> {
    (
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(t.colour.scrim.0),
    )
}

/// Draw every part in its current state. A press, a selection and a focus
/// ring show on the frame they happen; only the fill eases, and only for
/// hover, so input is never waiting on a flourish.
pub fn style_parts(
    tokens: Res<Tokens>,
    time: Res<Time>,
    focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    mut parts: Query<
        (
            Entity,
            &Hovered,
            Has<Pressed>,
            Has<Selected>,
            Has<InteractionDisabled>,
            Has<NotBuilt>,
            Has<Unseen>,
            Has<Pointed>,
            Option<&ShowLook>,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Outline,
        ),
        With<Part>,
    >,
    is_part: Query<(), With<Part>>,
    children: Query<&Children>,
    mut texts: Query<(&PartText, &mut TextColor)>,
    mut marks: Query<&mut BackgroundColor, (With<Mark>, Without<Part>)>,
    mut attentions: Query<&mut Visibility, (With<Attention>, Without<Pointer>)>,
    mut pointers: Query<&mut Visibility, (With<Pointer>, Without<Attention>)>,
) {
    let ease = if tokens.time.hover > 0.0 {
        (time.delta_secs() / tokens.time.hover).min(1.0)
    } else {
        1.0
    };
    let mut stack = Vec::new();

    for (
        entity,
        hovered,
        pressed,
        selected,
        disabled,
        not_built,
        unseen,
        pointed,
        shown,
        mut fill,
        mut border,
        mut ring,
    ) in &mut parts
    {
        let unavailable = disabled || not_built;
        let look = match shown {
            Some(shown) => shown.0,
            None => Look {
                hovered: hovered.get() && !unavailable,
                pressed,
                selected,
                unavailable,
                focused: focus_visible.0 && focus.get() == Some(entity),
            },
        };
        let paint = paint(&tokens.colour, look);

        if fill.0 != paint.fill {
            let (from, to) = (fill.0.to_srgba(), paint.fill.to_srgba());
            let settled = (from.red - to.red).abs() < 0.01
                && (from.green - to.green).abs() < 0.01
                && (from.blue - to.blue).abs() < 0.01;
            let snap = shown.is_some() || look.pressed || look.selected || look.unavailable;
            fill.0 = if snap || settled || from.alpha != to.alpha {
                paint.fill
            } else {
                from.mix(&to, ease).into()
            };
        }
        let edge = BorderColor::all(paint.border);
        if *border != edge {
            *border = edge;
        }
        if ring.color != paint.ring {
            ring.color = paint.ring;
        }

        // The part's own text and mark, stopping at any part nested inside it.
        stack.clear();
        stack.extend(children.get(entity).into_iter().flatten());
        while let Some(child) = stack.pop() {
            if is_part.contains(child) {
                continue;
            }
            if let Ok((kind, mut colour)) = texts.get_mut(child) {
                let wanted = match kind {
                    PartText::Main => paint.text,
                    PartText::Dim => paint.dim,
                };
                if colour.0 != wanted {
                    colour.0 = wanted;
                }
            }
            if let Ok(mut mark) = marks.get_mut(child) {
                if mark.0 != paint.mark {
                    mark.0 = paint.mark;
                }
            }
            if let Ok(mut attention) = attentions.get_mut(child) {
                let wanted = if unseen {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                if *attention != wanted {
                    *attention = wanted;
                }
            }
            if let Ok(mut pointer) = pointers.get_mut(child) {
                let wanted = if pointed {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                if *pointer != wanted {
                    *pointer = wanted;
                }
            }
            stack.extend(children.get(child).into_iter().flatten());
        }
    }
}

/// The six states the menus guide names, one at a time.
pub fn six() -> [(&'static str, Look); 6] {
    let base = Look::default();
    [
        ("default", base),
        (
            "hover",
            Look {
                hovered: true,
                ..base
            },
        ),
        (
            "focus",
            Look {
                focused: true,
                ..base
            },
        ),
        (
            "pressed",
            Look {
                hovered: true,
                pressed: true,
                ..base
            },
        ),
        (
            "selected",
            Look {
                selected: true,
                ..base
            },
        ),
        (
            "unavailable",
            Look {
                unavailable: true,
                ..base
            },
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lightness with the colour taken out; `None` for "not drawn".
    fn grey(colour: Color) -> Option<f32> {
        let c = colour.to_srgba();
        (c.alpha > 0.0).then(|| 0.299 * c.red + 0.587 * c.green + 0.114 * c.blue)
    }

    fn differ(a: Color, b: Color) -> bool {
        match (grey(a), grey(b)) {
            (Some(a), Some(b)) => (a - b).abs() >= 0.1,
            (None, None) => false,
            _ => true,
        }
    }

    /// Whatever the tokens are set to, no two states may differ by hue alone.
    #[test]
    fn every_state_can_be_told_apart_in_greyscale() {
        let colours = Tokens::load().colour;
        let states = six();
        for (i, (name_a, a)) in states.iter().enumerate() {
            for (name_b, b) in &states[i + 1..] {
                let (a, b) = (paint(&colours, *a), paint(&colours, *b));
                let apart = differ(a.fill, b.fill)
                    || differ(a.border, b.border)
                    || differ(a.text, b.text)
                    || differ(a.mark, b.mark)
                    || differ(a.ring, b.ring);
                assert!(apart, "{name_a} and {name_b} look alike without colour");
            }
        }
    }

    /// "Hover means this can be done; selected means this is chosen; they
    /// never look alike": apart in the fill and in the mark, not only one.
    #[test]
    fn hover_and_selected_never_look_alike() {
        let colours = Tokens::load().colour;
        let states = six();
        let hover = paint(&colours, states[1].1);
        let selected = paint(&colours, states[4].1);
        assert!(differ(hover.fill, selected.fill));
        assert!(differ(hover.mark, selected.mark));

        let both = paint(
            &colours,
            Look {
                hovered: true,
                selected: true,
                ..default()
            },
        );
        assert_eq!(both.fill, selected.fill);
        assert_eq!(both.mark, selected.mark);
    }

    /// The attention mark (#232) is found with the colour taken away: it
    /// stands off every fill a tab can have.
    #[test]
    fn the_attention_mark_stands_off_every_fill() {
        let colours = Tokens::load().colour;
        for (name, look) in six() {
            let fill = paint(&colours, look).fill;
            let fill = if grey(fill).is_some() { fill } else { colours.panel.0 };
            assert!(differ(colours.attention.0, fill), "{name}");
        }
    }

    /// The control a hint is about is found with the colour taken away
    /// (#291): its mark stands off every fill a part can have.
    #[test]
    fn the_hints_mark_stands_off_every_fill() {
        let colours = Tokens::load().colour;
        let mark = pointer_colour(&colours);
        for (name, look) in six() {
            let fill = paint(&colours, look).fill;
            let fill = if grey(fill).is_some() { fill } else { colours.panel.0 };
            assert!(differ(mark, fill), "{name}");
        }
        assert!(differ(mark, colours.screen.0));
    }

    /// Text stays readable against its fill in every state that can be used.
    #[test]
    fn text_stands_out_from_its_fill() {
        let colours = Tokens::load().colour;
        for (name, look) in six() {
            let paint = paint(&colours, look);
            let fill = grey(paint.fill).unwrap_or(grey(colours.panel.0).unwrap());
            let text = grey(paint.text).unwrap();
            let needed = if look.unavailable { 0.2 } else { 0.5 };
            assert!((text - fill).abs() >= needed, "{name}: {text} on {fill}");
        }
    }
}
