//! Overlay state tracking and UI rendering
//!
//! Defines resource for tracking which overlay elements should be displayed.
//! Overlay UI rendering system (update_overlay_ui) is implemented with egui integration.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Resource tracking overlay visibility
///
/// Stores visibility flags for various HUD elements.
/// Overlay UI rendering is implemented via update_overlay_ui system with egui.
///
/// # Missing Field: show_epoch_info
///
/// The [`DisplayConfig`] struct in genesis-core (defined at genesis-core/src/config.rs:112-120)
/// has a `show_epoch_info` field, and genesis.toml also includes this field.
///
/// However, this OverlayState struct is **missing** the `show_epoch_info` field.
/// This means epoch information cannot be displayed in the overlay even when enabled in configuration.
///
/// main.rs attempts to set OverlayState.show_epoch_info (line 63) which causes a compilation error
/// because this field doesn't exist in the struct.
///
/// **TODO**: Add `pub show_epoch_info: bool` field to this struct and update
/// [`update_overlay_ui()`] to display epoch information when enabled.
#[derive(Resource, Default)]
pub struct OverlayState {
    pub show_fps: bool,
    pub show_particle_count: bool,
}

/// System to update and display the overlay UI
///
/// Renders a semi-transparent overlay in the top-left corner of the screen
/// showing FPS counter and particle count based on visibility flags.
pub fn update_overlay_ui(
    mut contexts: EguiContexts,
    overlay_state: Res<OverlayState>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    particles: Query<&genesis_render::particle::Particle>,
) {
    let ctx = contexts.ctx_mut();

    // Don't show overlay if all visibility flags are false
    if !overlay_state.show_fps && !overlay_state.show_particle_count {
        return;
    }

    egui::Window::new("Overlay")
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .default_width(200.0)
        .show(ctx, |ui| {
            // Display FPS if enabled
            if overlay_state.show_fps {
                if let Some(fps) =
                    diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
                {
                    if let Some(fps_value) = fps.smoothed() {
                        ui.label(format!("FPS: {:.1}", fps_value));
                    }
                }
            }

            // Display particle count if enabled
            if overlay_state.show_particle_count {
                let particle_count = particles.iter().count();
                ui.label(format!("Particles: {}", particle_count));
            }
        });
}
