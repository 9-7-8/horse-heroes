//! The loot shower and every other piece of juice. Presentation only: the sim
//! decides what drops, and this shows it. How it works: docs/loot-shower.md.

use bevy::prelude::*;

pub mod death;
pub mod drops;
pub mod response;
pub mod sound;

pub struct FxPlugin {
    /// Throw the test burst shortly after launch (`--burst`). F3 throws it
    /// either way.
    pub burst: bool,
    /// Throw it and let the party collect it (`--pickup`). F4 does the same.
    pub pickup: bool,
    /// Seconds after launch at which either of them throws it
    /// (`--burst-at`), for a look at it on the road.
    pub burst_at: Option<f32>,
    /// Play no sound (`--mute`).
    pub mute: bool,
}

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(drops::DropsPlugin {
            burst: self.burst,
            pickup: self.pickup,
            burst_at: self.burst_at.unwrap_or(drops::BURST_AT),
        })
        .add_plugins(death::DeathPlugin)
        .add_plugins(response::ResponsePlugin)
        .add_plugins(sound::SoundPlugin { mute: self.mute });
    }
}
