//! GENESIS UI - User interface state and components
//!
//! This crate contains UI state resources and data structures.
//! Provides bevy_egui integration and timeline controls.
//!
//! # Modules
//!
//! - [`overlay`] - Overlay state tracking and UI rendering (FPS counter, particle count)
//! - [`timeline`] - Timeline playback state, cosmic time management, and timeline UI controls
//!
//! # Public Exports
//!
//! - `GenesisUiPlugin` - Main plugin that initializes bevy_egui and all UI systems
//! - `UIPlugin` - Type alias for GenesisUiPlugin (backwards compatibility)
//!
//! # UI Systems
//!
//! - `update_overlay_ui` (Update): Renders FPS and particle count overlay
//! - `timeline_panel_ui` (PostUpdate): Renders timeline with play/pause, slider, speed control
//! - `sync_time_resources` (Update): Synchronizes TimeAccumulator with PlaybackState

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod overlay;
pub mod timeline;

use timeline::TimelinePlugin;

/// Version of the UI library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin that initializes the UI system with bevy_egui and timeline controls
///
/// This plugin registers the following:
/// - `EguiPlugin`: Provides egui integration for UI rendering
/// - `TimelinePlugin`: Timeline UI with play/pause, logarithmic slider, speed control
/// - `update_overlay_ui` system: FPS counter and particle count display
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
