//! Information overlays
//!
//! FPS counter, particle count display, epoch info panels,
//! and other HUD elements.

use bevy::prelude::*;

/// Resource tracking overlay visibility
#[derive(Resource, Default)]
pub struct OverlayState {
    pub show_fps: bool,
    pub show_particle_count: bool,
    pub show_epoch_info: bool,
}
