//! The loot shower and every other piece of juice. Presentation only: the sim
//! decides what drops, and this shows it. How it works: docs/loot-shower.md.

use bevy::prelude::*;

pub mod drops;

pub struct FxPlugin {
    /// Throw the test burst shortly after launch (`--burst`). F3 throws it
    /// either way.
    pub burst: bool,
}

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(drops::DropsPlugin { burst: self.burst });
    }
}
