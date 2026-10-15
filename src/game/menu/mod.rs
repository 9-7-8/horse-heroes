//! The menus: the tokens they draw with, the parts they are made from, the
//! shell every screen hangs on, and the dev screen that shows every part in
//! every state. How it works: docs/menu-parts.md and docs/menu-shell.md.

use bevy::{input_focus::tab_navigation::TabNavigationPlugin, prelude::*};

pub mod card;
mod gallery;
pub mod held;
pub mod ink;
pub mod parts;
pub mod picture;
pub mod shell;
pub mod teach;
pub mod tokens;

pub struct MenuPlugin {
    /// Open the parts screen at startup (`--parts`). F2 toggles it either way.
    pub show_parts: bool,
    /// Start with the sample hint showing (`--hint`). F6 toggles it.
    pub hint: bool,
    /// A lesson's pop-up stops the game while it is up (#353). Off with
    /// `--no-pause` and in the soak, which nobody is at.
    pub pause: bool,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(tokens::Tokens::load())
            .insert_resource(teach::Pause(self.pause))
            .add_plugins(TabNavigationPlugin)
            .add_plugins(shell::ShellPlugin { hint: self.hint })
            .add_plugins(gallery::GalleryPlugin {
                open: self.show_parts,
            })
            .add_systems(Update, parts::style_parts);
    }
}
