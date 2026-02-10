//! Overlay state tracking and UI rendering
//!
//! Defines resource for tracking which overlay elements should be displayed.
//! Overlay UI rendering system (update_overlay_ui) is implemented with egui integration.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Resource tracking overlay visibility
///
/// Stores visibility flags for various HUD elements including FPS counter
/// and particle count display. This resource is initialized from DisplayConfig
/// and allows runtime toggling of overlay elements.
///
/// # Fields
///
/// - `show_fps`: When true, displays the current frame rate counter
/// - `show_particle_count`: When true, displays the total number of active particles
///
/// # Note
///
/// The `show_epoch_info` field is not implemented yet (not present in OverlayState
/// or DisplayConfig).
#[derive(Resource)]
pub struct OverlayState {
    /// Display FPS counter in HUD
    pub show_fps: bool,
    /// Display particle count
    pub show_particle_count: bool,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            show_fps: true,
            show_particle_count: true,
        }
    }
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
