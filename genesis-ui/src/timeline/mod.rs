//! Timeline playback state
//!
//! Defines resources for tracking playback state, cosmic time, and speed.
//! Actual timeline UI widgets and controls are not yet implemented.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

/// Resource tracking playback state
///
/// Stores playing state and speed factor for time control.
/// Timeline UI systems need to be implemented separately.
#[derive(Resource, Default)]
pub struct PlaybackState {
    pub playing: bool,
    pub speed: f32, // 0.0 to 1.0 for logarithmic mapping (when implemented)
}

/// Resource for cosmic timeline state management
///
/// Tracks the current cosmic time in years from 0 (Big Bang) to 13.8 billion years.
/// Provides methods for logarithmic slider mapping for timeline scrubbing UI.
#[derive(Resource)]
pub struct CosmicTime {
    /// Current cosmic time in years (0.0 to 13.8 billion)
    pub cosmic_time: f64,
    /// Minimum time (0.0 = Big Bang)
    pub min_time: f64,
    /// Maximum time (13.8 billion years = 13.8e9)
    pub max_time: f64,
}

impl CosmicTime {
    /// Creates a new CosmicTime resource with default values.
    ///
    /// Initializes with cosmic_time at 0.0 (Big Bang), min_time at 0.0,
    /// and max_time at 13.8 billion years.
    pub fn new() -> Self {
        Self {
            cosmic_time: 0.0,
            min_time: 0.0,
            max_time: 13.8e9,
        }
    }

    /// Maps a logarithmic slider value (0.0 to 1.0) to cosmic time in years.
    ///
    /// Uses logarithmic scaling: t = min_time * 10^(slider_value * log10(max_time/min_time))
    ///
    /// # Arguments
    /// * `slider_value` - Slider position from 0.0 to 1.0
    ///
    /// # Returns
    /// Cosmic time in years corresponding to the slider position
    ///
    /// # Note
    /// When min_time is 0.0, this uses a small epsilon value to avoid log10(0).
    pub fn from_slider(slider_value: f64) -> f64 {
        let min_time: f64 = 1.0; // Use 1 year as minimum for log scale to avoid log10(0)
        let max_time: f64 = 13.8e9;
        let log_ratio: f64 = f64::log10(max_time / min_time);
        min_time * 10_f64.powf(slider_value * log_ratio)
    }

    /// Maps cosmic time in years to a logarithmic slider value (0.0 to 1.0).
    ///
    /// Inverse of `from_slider`: slider = log10(cosmic_time/min_time) / log10(max_time/min_time)
    ///
    /// # Arguments
    /// * `cosmic_time` - Cosmic time in years to convert to slider position
    ///
    /// # Returns
    /// Slider position from 0.0 to 1.0 corresponding to the cosmic time
    ///
    /// # Note
    /// When cosmic_time is 0.0 or less than 1 year, returns 0.0 as slider position.
    pub fn to_slider(cosmic_time: f64) -> f64 {
        let min_time: f64 = 1.0; // Use 1 year as minimum for log scale to avoid log10(0)
        let max_time: f64 = 13.8e9;
        if cosmic_time < min_time {
            return 0.0;
        }
        let log_ratio: f64 = f64::log10(max_time / min_time);
        f64::log10(cosmic_time / min_time) / log_ratio
    }

    /// Sets the current cosmic time.
    ///
    /// Clamps the value between min_time and max_time.
    ///
    /// # Arguments
    /// * `time` - New cosmic time in years
    pub fn set_time(&mut self, time: f64) {
        self.cosmic_time = time.clamp(self.min_time, self.max_time);
    }

    /// Gets the current cosmic time.
    ///
    /// # Returns
    /// Current cosmic time in years
    pub fn get_time(&self) -> f64 {
        self.cosmic_time
    }

    /// Resets cosmic time to the Big Bang (0.0 years).
    pub fn reset(&mut self) {
        self.cosmic_time = self.min_time;
    }
}

impl Default for CosmicTime {
    fn default() -> Self {
        Self::new()
    }
}

/// System that renders the timeline UI panel using egui.
///
/// Creates a fixed panel at the bottom of the screen with playback controls:
/// - Play/Pause button
/// - Timeline slider with logarithmic scale
/// - Speed control slider
/// - Time display in billions of years
pub fn timeline_panel_ui(
    mut contexts: EguiContexts,
    mut cosmic_time: ResMut<CosmicTime>,
    mut playback_state: ResMut<PlaybackState>,
) {
    egui::Window::new("Timeline")
        .resizable(false)
        .collapsible(false)
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                // Play/Pause button
                if playback_state.playing {
                    if ui.button("⏸ Pause").clicked() {
                        playback_state.playing = false;
                    }
                } else {
                    if ui.button("▶ Play").clicked() {
                        playback_state.playing = true;
                    }
                }

                ui.separator();

                // Timeline slider
                ui.label("Time:");
                let current_slider_value = CosmicTime::to_slider(cosmic_time.cosmic_time);
                let mut slider_value = current_slider_value;
                if ui.add(egui::Slider::new(&mut slider_value, 0.0..=1.0).show_value(false)).changed() {
                    cosmic_time.cosmic_time = CosmicTime::from_slider(slider_value);
                }

                ui.separator();

                // Speed control slider
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut playback_state.speed, 0.1..=10.0).logarithmic(true)
                    .step_by(0.1)
                    .prefix("")
                    .suffix("x"));

                ui.separator();

                // Time display in billions of years
                let time_in_billions = cosmic_time.cosmic_time / 1e9;
                ui.label(format!("Time: {:.2} billion years", time_in_billions));
            });
        });
}

/// Plugin that sets up the timeline UI system and resources.
pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CosmicTime::new())
            .insert_resource(PlaybackState::default())
            .add_systems(bevy::app::PostUpdate, timeline_panel_ui);
    }
}
