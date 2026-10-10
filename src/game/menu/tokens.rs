//! The menu tokens in `assets/data/menu.ron`, compiled into the binary.

use bevy::prelude::*;
use serde::{Deserialize, Deserializer};

#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Tokens {
    pub space: Space,
    pub text: TextSizes,
    pub size: Sizes,
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
    pub side_panel_width: f32,
    pub confirm_width: f32,
    pub counter_width: f32,
    pub strip_horse_width: f32,
    pub horses_width: f32,
    pub cargo_width: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Times {
    pub hover: f32,
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
    pub scrim: Hex,
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
