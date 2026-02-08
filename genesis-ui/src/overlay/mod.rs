//! Overlay state tracking
//!
//! Defines resource for tracking which overlay elements should be displayed.
//! Actual overlay UI rendering and display systems are not yet implemented.

use bevy::prelude::*;

/// Resource tracking overlay visibility
///
/// Stores visibility flags for various HUD elements.
/// Overlay rendering systems need to be implemented separately.
#[derive(Resource, Default)]
pub struct OverlayState {
    pub show_fps: bool,
    pub show_particle_count: bool,
    pub show_epoch_info: bool,
}
