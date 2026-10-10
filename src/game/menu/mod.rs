//! The menus: the tokens they draw with, the parts they are made from, the
//! shell every screen hangs on, and the dev screen that shows every part in
//! every state. How it works: docs/menu-parts.md and docs/menu-shell.md.

use bevy::{input_focus::tab_navigation::TabNavigationPlugin, prelude::*};

mod gallery;
pub mod parts;
pub mod shell;
pub mod tokens;

pub struct MenuPlugin {
    /// Open the parts screen at startup (`--parts`). F2 toggles it either way.
    pub show_parts: bool,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(tokens::Tokens::load())
            .add_plugins(TabNavigationPlugin)
            .add_plugins(shell::ShellPlugin)
            .add_plugins(gallery::GalleryPlugin {
                open: self.show_parts,
            })
            .add_systems(Update, parts::style_parts);
    }
}
