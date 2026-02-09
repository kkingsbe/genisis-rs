//! Overlay state tracking and UI rendering
//!
//! Defines resource for tracking which overlay elements should be displayed.
//! Overlay UI rendering system (update_overlay_ui) is implemented with egui integration.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use genesis_core::epoch::EpochManager;

/// Resource tracking overlay visibility
///
/// Stores visibility flags for various HUD elements.
/// Overlay UI rendering is implemented via update_overlay_ui system with egui.
#[derive(Resource, Default)]
pub struct OverlayState {
    pub show_fps: bool,
    pub show_particle_count: bool,
    pub show_epoch_info: bool,
}

/// System to update and display the overlay UI
///
/// Renders a semi-transparent overlay in the top-left corner of the screen
/// showing FPS counter, particle count, and epoch info based on visibility flags.
pub fn update_overlay_ui(
    mut contexts: EguiContexts,
    overlay_state: Res<OverlayState>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    particles: Query<&genesis_render::particle::Particle>,
    epoch_manager: Res<EpochManager>,
) {
    let ctx = contexts.ctx_mut();

    // Don't show overlay if all visibility flags are false
    if !overlay_state.show_fps
        && !overlay_state.show_particle_count
        && !overlay_state.show_epoch_info
    {
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

            // Display epoch info if enabled
            if overlay_state.show_epoch_info {
                let epoch_name = epoch_manager
                    .get_current_epoch()
                    .map(|epoch| epoch.name())
                    .unwrap_or("N/A");
                ui.label(format!("Epoch: {}", epoch_name));
            }
        });
}
