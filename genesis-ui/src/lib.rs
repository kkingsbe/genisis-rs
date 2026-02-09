//! GENESIS UI - User interface state and components
//!
//! This crate contains UI state resources and data structures.
//! Provides bevy_egui integration and timeline controls.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod overlay;
pub mod timeline;

use timeline::TimelinePlugin;

/// Version of the UI library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin that initializes the UI system with bevy_egui and timeline controls
pub struct GenesisUiPlugin;

impl Plugin for GenesisUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_plugins(TimelinePlugin)
            .add_systems(Update, overlay::update_overlay_ui.after(bevy_egui::EguiSet::InitContexts));
    }
}

/// Type alias for GenesisUiPlugin for backwards compatibility
pub use GenesisUiPlugin as UIPlugin;
