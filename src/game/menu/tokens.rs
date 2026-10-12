//! The menu tokens in `assets/data/menu.ron`, compiled into the binary.

use bevy::prelude::*;
use serde::{Deserialize, Deserializer};

use super::ink::{Ink, Stat};
use crate::fx::drops::rarity_colour;
use crate::sim::data::{Element, GearTable};

#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Tokens {
    pub space: Space,
    pub text: TextSizes,
    pub size: Sizes,
    pub layout: Layout,
    pub time: Times,
    pub colour: Colours,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Space {
    pub xs: f32,
    pub s: f32,
    pub m: f32,
    pub l: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TextSizes {
    pub small: f32,
    pub body: f32,
    pub title: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Sizes {
    pub control_height: f32,
    pub row_height: f32,
    pub border: f32,
    pub radius: f32,
    pub mark: f32,
    pub focus_ring: f32,
    pub focus_gap: f32,
    pub card_width: f32,
    pub cell_height: f32,
    pub inventory_cell: f32,
    pub inventory_picture: f32,
    pub compare_width: f32,
    pub picture: f32,
    pub picture_inset: f32,
    pub picture_stamp: f32,
    pub side_panel_width: f32,
    pub confirm_width: f32,
    pub counter_width: f32,
    pub scrollbar_width: f32,
    pub scroll_page: f32,
    pub strip_horse_width: f32,
    pub cargo_width: f32,
    pub trail_mark: f32,
    pub trail_boss: f32,
    pub attention: f32,
    pub fight_log_width: f32,
    pub fight_log_lines: usize,
    pub fight_log_fade: f32,
    pub hint_height: f32,
    pub hint_control: f32,
    pub pointer: f32,
}

/// The split of the window between the scene and the menus.
#[derive(Clone, Debug, Deserialize)]
pub struct Layout {
    pub town_share: f32,
    pub road_share: f32,
    pub min_width: f32,
    pub min_height: f32,
}

impl Layout {
    /// The share of the window's height the scene takes, from the top, when
    /// it is `expanded` of the way from the town layout to the road's.
    pub fn share(&self, expanded: f32) -> f32 {
        self.town_share + (self.road_share - self.town_share) * expanded
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Times {
    pub hover: f32,
    pub hover_card: f32,
    pub layout: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Colours {
    pub screen: Hex,
    pub panel: Hex,
    pub panel_border: Hex,
    pub control: Hex,
    pub control_border: Hex,
    pub hover: Hex,
    pub hover_border: Hex,
    pub pressed: Hex,
    pub pressed_text: Hex,
    pub selected: Hex,
    pub accent: Hex,
    pub unavailable_border: Hex,
    pub unavailable_text: Hex,
    pub focus: Hex,
    pub text: Hex,
    pub text_dim: Hex,
    pub reason: Hex,
    pub attention: Hex,
    pub scrim: Hex,
    pub element: ElementColours,
    pub stat: StatColours,
    pub change: ChangeColours,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChangeColours {
    pub gain: Hex,
    pub loss: Hex,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ElementColours {
    pub fire: Hex,
    pub ice: Hex,
    pub plant: Hex,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StatColours {
    pub health: Hex,
    pub attack: Hex,
    pub defense: Hex,
    pub elemental: Hex,
}

impl Colours {
    /// An element's colour, in a fight and in text.
    pub fn of(&self, element: Element) -> Color {
        match element {
            Element::Fire => self.element.fire.0,
            Element::Ice => self.element.ice.0,
            Element::Plant => self.element.plant.0,
        }
    }

    /// The colour of a piece of text that is about one kind of thing. A
    /// piece's name has its rarity's colour, which is `gear`'s, not a token.
    pub fn ink(&self, ink: Ink, gear: &GearTable) -> Color {
        match ink {
            Ink::Rarity(tier) => rarity_colour(&gear.rarities[tier]).into(),
            Ink::Element(element) => self.of(element),
            Ink::Stat(Stat::Health) => self.stat.health.0,
            Ink::Stat(Stat::Attack) => self.stat.attack.0,
            Ink::Stat(Stat::Defense) => self.stat.defense.0,
            Ink::Stat(Stat::Elemental) => self.stat.elemental.0,
            Ink::Gain => self.change.gain.0,
            Ink::Loss => self.change.loss.0,
        }
    }
}

/// A colour written as sRGB hex in the data file: `"rrggbb"` or `"rrggbbaa"`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hex(pub Color);

impl<'de> Deserialize<'de> for Hex {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Hex, D::Error> {
        let text = String::deserialize(deserializer)?;
        Srgba::hex(&text)
            .map(|colour| Hex(colour.into()))
            .map_err(|e| serde::de::Error::custom(format!("colour `{text}`: {e}")))
    }
}

impl Tokens {
    pub fn load() -> Tokens {
        ron::from_str(include_str!("../../../assets/data/menu.ron"))
            .unwrap_or_else(|e| panic!("assets/data/menu.ron: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The road gives the scene more of the window than the town does, and
    /// both leave room for the menus.
    #[test]
    fn the_road_layout_is_the_larger_scene() {
        let layout = Tokens::load().layout;
        assert!(0.0 < layout.town_share && layout.town_share < layout.road_share);
        assert!(layout.road_share < 1.0);
        assert_eq!(layout.share(0.0), layout.town_share);
        assert_eq!(layout.share(1.0), layout.road_share);
        assert!(layout.min_width > 0.0 && layout.min_height > 0.0);
    }

    /// The hint's line (#291) is there at the smallest window too: the bar,
    /// the status strip, that line and the footer fit under the road's
    /// scene, so a hint pushes nothing out of the window.
    #[test]
    fn the_hints_line_fits_under_the_road_scene_at_the_smallest_window() {
        let Tokens { size, space, layout, .. } = Tokens::load();
        let bar = size.control_height + 2.0 * size.border;
        let strip = size.row_height + 2.0 * space.s + size.border;
        let footer = size.control_height + 2.0 * space.s + size.border;
        let room = layout.min_height * (1.0 - layout.road_share);
        assert!(bar + strip + size.hint_height + footer <= room + 0.01);
        assert!(size.hint_control <= size.hint_height);
    }
}
