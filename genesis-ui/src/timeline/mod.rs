//! Timeline playback state and UI controls
//!
//! Defines resources for tracking playback state, cosmic time, and speed.
//! Timeline UI system (timeline_panel_ui) is implemented with egui, featuring play/pause,
//! logarithmic slider, and speed control.
//!
//! # Dual Time System
//!
//! The timeline uses [`CosmicTime`] to track the current cosmic time in years.
//! This resource stores the timeline position used by the slider UI (0.0 to 13.8 billion years).
//!
//! The core simulation uses [`TimeAccumulator::years`] (genesis_core::time) which
//! tracks accumulated cosmic time based on frame delta and acceleration.
//!
//! The timeline slider updates [`CosmicTime::cosmic_time`] directly and synchronizes it
//! to [`TimeAccumulator::years`] when the timeline is scrubbed. This ensures that
//! scrubbing the timeline affects both the UI display and the actual simulation time accumulator.
//!
//! The [`sync_time_resources()`] system synchronizes:
//! - TimeAccumulator's paused state with PlaybackState.playing
//! - PlaybackState.speed to TimeAccumulator.acceleration (direct pass-through)

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiSet};
use genesis_core::events::ScrubbingEvent;
use genesis_core::time::{TimeAccumulator, MIN_YEARS};

/// Resource tracking playback state
///
/// Stores playing state and speed factor for time control.
/// Timeline UI is implemented via timeline_panel_ui system with egui.
#[derive(Resource)]
pub struct PlaybackState {
    pub playing: bool,
    pub speed: f32, // 1.0 to 1e12 for direct pass-through playback speed control (no logarithmic scaling)
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            playing: false,
            speed: 1.0, // Default to 1x real time
        }
    }
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
    /// Uses logarithmic scaling: t = effective_min * 10^(slider_value * log10(max_time/effective_min))
    ///
    /// # Arguments
    /// * `slider_value` - Slider position from 0.0 to 1.0
    ///
    /// # Returns
    /// Cosmic time in years corresponding to the slider position
    ///
    /// # Note
    /// Uses MIN_YEARS (1e-40) as effective_min when min_time is 0.0 to enable sub-year
    /// scaling while avoiding log10(0). This allows the timeline to represent cosmic
    /// time from the earliest epochs after the Big Bang (~10^-43 seconds) through
    /// the present day.
    pub fn from_slider(&self, slider_value: f64) -> f64 {
        let effective_min = if self.min_time == 0.0 { MIN_YEARS } else { self.min_time };
        let log_ratio: f64 = f64::log10(self.max_time / effective_min);
        effective_min * 10_f64.powf(slider_value * log_ratio)
    }

    /// Maps cosmic time in years to a logarithmic slider value.
    ///
    /// Inverse of `from_slider`: slider = log10(cosmic_time/min_time) / log10(max_time/min_time)
    ///
    /// # Arguments
    /// * `cosmic_time` - Cosmic time in years to convert to slider position
    ///
    /// # Returns
    /// Slider position corresponding to the cosmic time:
    /// - Negative values for pre-1-year timescales (representing epochs from ~10^-43 seconds to 1 year)
    /// - 0.0 at exactly 1 year
    /// - 0.0 to 1.0 for 1 year to max_time
    ///
    /// # Note
    /// When min_time is 0.0, negative slider values represent the pre-1-year portion of the timeline,
    /// using MIN_YEARS (1e-40) as the effective minimum. The mapping is mathematically consistent
    /// with from_slider() such that to_slider(from_slider(x)) ≈ x.
    pub fn to_slider(&self, cosmic_time: f64) -> f64 {
        let effective_min = if self.min_time == 0.0 { 1.0 } else { self.min_time };
        if cosmic_time < effective_min {
            // Pre-1-year timescales: return negative slider values
            // Map [MIN_YEARS, 1.0] to [-1.0, 0.0] using logarithmic scaling
            let log_ratio_pre: f64 = f64::log10(1.0 / MIN_YEARS);
            let log_pos: f64 = f64::log10(cosmic_time / MIN_YEARS);
            let pre_1yr_slider = log_pos / log_ratio_pre; // Maps to [-1.0, 0.0]
            
            // Offset to preserve negative values while allowing positive range up to 1.0
            return pre_1yr_slider - 1.0;
        }
        let log_ratio: f64 = f64::log10(self.max_time / effective_min);
        f64::log10(cosmic_time / effective_min) / log_ratio
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
    mut time_accumulator: ResMut<TimeAccumulator>,
    mut scrubbing_events: EventWriter<ScrubbingEvent>,
) {
    egui::Window::new("Timeline")
        .resizable(false)
        .collapsible(false)
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                // Play/Pause button
                if playback_state.playing && ui.button("⏸ Pause").clicked() {
                    playback_state.playing = false;
                } else if !playback_state.playing && ui.button("▶ Play").clicked() {
                    playback_state.playing = true;
                }

                // Reset button
                if ui.button("⏮ Reset").clicked() {
                    cosmic_time.reset();
                    time_accumulator.reset();
                    playback_state.playing = false;
                }

                ui.separator();

                // Timeline slider
                ui.label("Time:");
                let current_slider_value = cosmic_time.to_slider(cosmic_time.cosmic_time);
                let mut slider_value = current_slider_value;
                let slider_response = ui.add(egui::Slider::new(&mut slider_value, 0.0..=1.0).show_value(false));
                
                // Emit ScrubbingEvent when user starts dragging
                if slider_response.drag_started() {
                    scrubbing_events.send(ScrubbingEvent { is_scrubbing: true });
                }
                
                // Update cosmic time when slider changes
                if slider_response.changed() {
                    cosmic_time.cosmic_time = cosmic_time.from_slider(slider_value);
                    // Synchronize TimeAccumulator.years with CosmicTime.cosmic_time
                    time_accumulator.years = cosmic_time.cosmic_time;
                }
                
                // Emit ScrubbingEvent when user stops dragging
                if slider_response.drag_stopped() {
                    scrubbing_events.send(ScrubbingEvent { is_scrubbing: false });
                }

                ui.separator();

                // Speed control slider
                ui.label("Speed:");
                ui.add(
                    egui::Slider::new(&mut playback_state.speed, 1.0..=1e12)
                        .logarithmic(true)
                        .prefix("")
                        .suffix("x"),
                );

                ui.separator();

                // Time display in billions of years
                let time_in_billions = cosmic_time.cosmic_time / 1e9;
                ui.label(format!("Time: {:.2} billion years", time_in_billions));
            });
        });
}

/// System that synchronizes TimeAccumulator with PlaybackState.
///
/// Ensures that TimeAccumulator's paused state is consistent with PlaybackState.playing:
/// - When playing is true and TimeAccumulator is paused, resume TimeAccumulator
/// - When playing is false and TimeAccumulator is not paused, pause TimeAccumulator
///
/// Also maps PlaybackState.speed (1.0-1e12) to TimeAccumulator.acceleration (1.0-1e12)
/// via direct pass-through
pub fn sync_time_resources(
    mut time_accumulator: ResMut<TimeAccumulator>,
    playback_state: Res<PlaybackState>,
) {
    if playback_state.playing && time_accumulator.is_paused() {
        time_accumulator.resume();
    } else if !playback_state.playing && !time_accumulator.is_paused() {
        time_accumulator.pause();
    }

    // Map PlaybackState.speed (1.0-1e12) to TimeAccumulator.acceleration (1.0-1e12)
    let speed = playback_state.speed as f64;
    let acceleration = speed as f64;
    time_accumulator.set_acceleration(acceleration);
}

/// Plugin that sets up the timeline UI system and resources.
pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CosmicTime::new())
            .insert_resource(PlaybackState::default())
            .add_systems(bevy::app::Update, timeline_panel_ui.after(EguiSet::InitContexts))
            .add_systems(bevy::app::Update, sync_time_resources);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genesis_core::time::SECONDS_PER_YEAR;

    /// Tolerance for floating-point comparisons
    const EPSILON: f64 = 1e-10;

    /// Test from_slider produces correct time ranges
    #[test]
    fn test_from_slider_produces_correct_ranges() {
        let cosmic_time = CosmicTime::new();

        // from_slider(0.0) should return MIN_YEARS
        let min_time = cosmic_time.from_slider(0.0);
        assert_eq!(min_time, 1e-40, "from_slider(0.0) should return MIN_YEARS");

        // from_slider(1.0) should return max_time (with tolerance)
        let max_time = cosmic_time.from_slider(1.0);
        let relative_error = (max_time - 13.8e9).abs() / 13.8e9;
        assert!(
            relative_error < 1e-10,
            "from_slider(1.0) should return approximately max_time"
        );

        // from_slider should be monotonically increasing
        let s0 = cosmic_time.from_slider(0.0);
        let s25 = cosmic_time.from_slider(0.25);
        let s5 = cosmic_time.from_slider(0.5);
        let s75 = cosmic_time.from_slider(0.75);
        let s1 = cosmic_time.from_slider(1.0);

        assert!(s0 < s25, "from_slider should be monotonically increasing");
        assert!(s25 < s5, "from_slider should be monotonically increasing");
        assert!(s5 < s75, "from_slider should be monotonically increasing");
        assert!(s75 < s1, "from_slider should be monotonically increasing");
    }

    /// Test to_slider produces correct slider value ranges
    #[test]
    fn test_to_slider_produces_correct_ranges() {
        let cosmic_time = CosmicTime::new();

        // to_slider at MIN_YEARS should return -1.0 (start of pre-1-year range)
        let slider_at_min = cosmic_time.to_slider(1e-40);
        assert!(
            (slider_at_min - (-1.0)).abs() < EPSILON,
            "to_slider(MIN_YEARS) should return approximately -1.0, got={}",
            slider_at_min
        );

        // to_slider at just below 1 year should be negative
        let slider_below_1 = cosmic_time.to_slider(0.999);
        assert!(
            slider_below_1 < 0.0,
            "to_slider should return negative for time < 1 year, got={}",
            slider_below_1
        );

        // to_slider at 1 year should be 0.0
        let slider_at_1 = cosmic_time.to_slider(1.0);
        assert!(
            slider_at_1.abs() < EPSILON,
            "to_slider(1.0) should return approximately 0.0, got={}",
            slider_at_1
        );

        // to_slider at max_time should return 1.0
        let slider_at_max = cosmic_time.to_slider(13.8e9);
        assert!(
            (slider_at_max - 1.0).abs() < EPSILON,
            "to_slider(max_time) should return approximately 1.0, got={}",
            slider_at_max
        );
    }

    /// Test from_slider() at t=10^-30s (~3.17e-38 years)
    #[test]
    fn test_from_slider_at_10_minus_30_seconds() {
        let cosmic_time = CosmicTime::new();

        // t = 10^-30s ≈ 3.17e-38 years
        let target_time_years = 1e-30 / SECONDS_PER_YEAR; // ~3.17e-38 years
        
        // Since MIN_YEARS = 1e-40, 10^-30s is ~317x MIN_YEARS
        // Calculate expected slider value using inverse formula:
        // slider = log10(time / MIN_YEARS) / log10(max_time / MIN_YEARS)
        let log_ratio = f64::log10(13.8e9 / 1e-40);
        let expected_slider = f64::log10(target_time_years / 1e-40) / log_ratio;
        
        let actual_time = cosmic_time.from_slider(expected_slider);
        
        assert!(
            (actual_time - target_time_years).abs() < EPSILON * target_time_years.abs(),
            "from_slider() at t=10^-30s: expected={}, got={}",
            target_time_years,
            actual_time
        );

        // Verify that 10^-30s is representable (well above MIN_YEARS)
        assert!(
            target_time_years > 1e-40,
            "10^-30s should be > MIN_YEARS (1e-40)"
        );
        assert!(
            actual_time > 0.0,
            "from_slider() should return positive value for 10^-30s"
        );
    }

    /// Test from_slider() at t=10^-6s (~3.17e-14 years)
    #[test]
    fn test_from_slider_at_10_minus_6_seconds() {
        let cosmic_time = CosmicTime::new();

        // t = 10^-6s ≈ 3.17e-14 years
        let target_time_years = 1e-6 / SECONDS_PER_YEAR; // ~3.17e-14 years
        
        // Since MIN_YEARS = 1e-40, 10^-6s is ~3.17e26x MIN_YEARS
        let log_ratio = f64::log10(13.8e9 / 1e-40);
        let expected_slider = f64::log10(target_time_years / 1e-40) / log_ratio;
        
        let actual_time = cosmic_time.from_slider(expected_slider);
        
        assert!(
            (actual_time - target_time_years).abs() < EPSILON * target_time_years.abs(),
            "from_slider() at t=10^-6s: expected={}, got={}",
            target_time_years,
            actual_time
        );

        // Verify that 10^-6s is representable
        assert!(
            target_time_years > 1e-40,
            "10^-6s should be > MIN_YEARS (1e-40)"
        );
        assert!(
            actual_time > 0.0,
            "from_slider() should return positive value for 10^-6s"
        );
    }

    /// Test to_slider() returns negative values for pre-1-year times
    #[test]
    fn test_to_slider_negative_for_pre_1year() {
        let cosmic_time = CosmicTime::new();

        // Test various pre-1-year timescales
        let pre_1year_times = [
            1e-40,    // MIN_YEARS
            1e-30,    // Near Planck scale
            1e-20,    // Early universe
            1e-10,    // Still extremely small
            1e-5,     // Small fraction of a year
            0.1,      // Tenth of a year
            0.5,      // Half a year
            0.9,      // Close to 1 year
        ];

        for &time in &pre_1year_times {
            let slider_value = cosmic_time.to_slider(time);
            assert!(
                slider_value < 0.0,
                "to_slider() should return negative value for pre-1-year time {}: got={}",
                time,
                slider_value
            );
            assert!(
                slider_value >= -1.0,
                "to_slider() should return value >= -1.0 for pre-1-year time {}: got={}",
                time,
                slider_value
            );
        }
    }

    /// Test to_slider() returns 0.0 for exactly 1 year
    #[test]
    fn test_to_slider_zero_at_exactly_1_year() {
        let cosmic_time = CosmicTime::new();
        let slider_value = cosmic_time.to_slider(1.0);
        
        assert!(
            slider_value.abs() < EPSILON,
            "to_slider(1.0) should return approximately 0.0, got={}",
            slider_value
        );
    }

    /// Test to_slider() returns positive values for post-1-year times
    #[test]
    fn test_to_slider_positive_for_post_1year() {
        let cosmic_time = CosmicTime::new();

        // Test various post-1-year timescales
        let post_1year_times = [
            1.0,      // Exactly 1 year
            10.0,     // 10 years
            100.0,    // 100 years
            1_000.0,  // 1 millennium
            1_000_000.0, // 1 million years
            1_000_000_000.0, // 1 billion years
            13.8e9,   // 13.8 billion years (current age)
        ];

        for &time in &post_1year_times {
            let slider_value = cosmic_time.to_slider(time);
            assert!(
                slider_value >= 0.0,
                "to_slider() should return non-negative value for post-1-year time {}: got={}",
                time,
                slider_value
            );
            assert!(
                slider_value <= 1.0,
                "to_slider() should return value <= 1.0 for post-1-year time {}: got={}",
                time,
                slider_value
            );
        }
    }

    /// Test from_slider at key time points across the full range
    #[test]
    fn test_from_slider_at_key_time_points() {
        let cosmic_time = CosmicTime::new();

        // Test that from_slider covers the full range from MIN_YEARS to max_time
        let min_time = cosmic_time.from_slider(0.0);
        assert_eq!(min_time, 1e-40, "from_slider(0.0) should return MIN_YEARS");

        let max_time = cosmic_time.from_slider(1.0);
        let relative_error = (max_time - 13.8e9).abs() / 13.8e9;
        assert!(
            relative_error < 1e-10,
            "from_slider(1.0) should return approximately max_time"
        );

        // Test that from_slider at mid-point gives geometric mean
        let mid_time = cosmic_time.from_slider(0.5);
        // Geometric mean: sqrt(MIN_YEARS * max_time)
        let expected_mid = f64::sqrt(1e-40 * 13.8e9);
        let relative_error_mid = (mid_time - expected_mid).abs() / expected_mid;
        assert!(
            relative_error_mid < 1e-6,
            "from_slider(0.5) should return approximately geometric mean, got={}, expected={}",
            mid_time,
            expected_mid
        );
    }

    /// Test to_slider at key time points across the full range
    #[test]
    fn test_to_slider_at_key_time_points() {
        let cosmic_time = CosmicTime::new();

        // Test boundary points
        let slider_min = cosmic_time.to_slider(1e-40);
        assert!(
            (slider_min - (-1.0)).abs() < EPSILON,
            "to_slider(MIN_YEARS) should return -1.0"
        );

        let slider_1yr = cosmic_time.to_slider(1.0);
        assert!(
            slider_1yr.abs() < EPSILON,
            "to_slider(1.0) should return 0.0"
        );

        let slider_max = cosmic_time.to_slider(13.8e9);
        assert!(
            (slider_max - 1.0).abs() < EPSILON,
            "to_slider(max_time) should return 1.0"
        );

        // Test that to_slider at mid-point (geometric mean of 1 and max_time) gives 0.5
        let mid_time = f64::sqrt(1.0 * 13.8e9);
        let slider_mid = cosmic_time.to_slider(mid_time);
        assert!(
            (slider_mid - 0.5).abs() < 1e-6,
            "to_slider at geometric mean should return 0.5, got={}",
            slider_mid
        );
    }

    /// Test edge case: min_time = 0.0
    #[test]
    fn test_edge_case_min_time_zero() {
        let cosmic_time = CosmicTime::new();
        assert_eq!(cosmic_time.min_time, 0.0);

        // from_slider at 0.0 should give MIN_YEARS
        let min_slider = cosmic_time.from_slider(0.0);
        assert_eq!(min_slider, 1e-40, "from_slider(0.0) should return MIN_YEARS");

        // from_slider at 1.0 should give max_time (allowing for floating-point precision)
        let max_slider = cosmic_time.from_slider(1.0);
        let relative_error = (max_slider - 13.8e9).abs() / 13.8e9;
        assert!(
            relative_error < 1e-10,
            "from_slider(1.0) should return approximately max_time (13.8e9), got={}, relative_error={}",
            max_slider,
            relative_error
        );
    }

    /// Test edge case: min_time > 0
    #[test]
    fn test_edge_case_min_time_positive() {
        let mut cosmic_time = CosmicTime::new();
        cosmic_time.min_time = 0.1; // 0.1 years

        // from_slider at 0.0 should give min_time
        let min_slider = cosmic_time.from_slider(0.0);
        assert_eq!(min_slider, 0.1, "from_slider(0.0) should return min_time");

        // to_slider at min_time should return 0.0
        let slider_value = cosmic_time.to_slider(0.1);
        assert!(
            slider_value.abs() < EPSILON,
            "to_slider(min_time) should return approximately 0.0, got={}",
            slider_value
        );
    }

    /// Test edge case: max_time variations
    #[test]
    fn test_edge_case_max_time_variations() {
        let mut cosmic_time = CosmicTime::new();

        // Test with different max_time values (must be > 1.0 to avoid division by zero)
        let max_times = [100.0, 1_000.0, 1e9, 13.8e9];

        for &max_time in &max_times {
            cosmic_time.max_time = max_time;

            // from_slider at 1.0 should give max_time (allowing for floating-point precision)
            let result = cosmic_time.from_slider(1.0);
            let relative_error = (result - max_time).abs() / max_time;
            assert!(
                relative_error < 1e-10,
                "from_slider(1.0) should return approximately max_time {}, got={}, relative_error={}",
                max_time,
                result,
                relative_error
            );

            // to_slider at max_time should return 1.0
            let slider_value = cosmic_time.to_slider(max_time);
            assert!(
                (slider_value - 1.0).abs() < EPSILON,
                "to_slider(max_time) should return approximately 1.0 for max_time={}, got={}",
                max_time,
                slider_value
            );
        }
    }

    /// Test continuity around 1 year boundary
    #[test]
    fn test_continuity_around_1_year() {
        let cosmic_time = CosmicTime::new();

        // Test points just below and above 1 year
        let delta = 1e-6;
        let below_1yr = 1.0 - delta;
        let at_1yr = 1.0;
        let above_1yr = 1.0 + delta;

        let slider_below = cosmic_time.to_slider(below_1yr);
        let slider_at = cosmic_time.to_slider(at_1yr);
        let slider_above = cosmic_time.to_slider(above_1yr);

        assert!(
            slider_below < 0.0,
            "to_slider() should return negative for just below 1 year, got={}",
            slider_below
        );
        assert!(
            slider_at.abs() < EPSILON,
            "to_slider(1.0) should return approximately 0.0, got={}",
            slider_at
        );
        assert!(
            slider_above > 0.0,
            "to_slider() should return positive for just above 1 year, got={}",
            slider_above
        );
    }

    /// Test log10 scale boundaries
    #[test]
    fn test_log10_scale_boundaries() {
        let cosmic_time = CosmicTime::new();

        // Verify logarithmic range covers expected span
        let min_slider_time = cosmic_time.from_slider(0.0);
        let max_slider_time = cosmic_time.from_slider(1.0);

        assert_eq!(min_slider_time, 1e-40, "Min slider time should be MIN_YEARS");
        
        // Allow for floating-point precision in max_slider_time
        let relative_error = (max_slider_time - 13.8e9).abs() / 13.8e9;
        assert!(
            relative_error < 1e-10,
            "Max slider time should be approximately 13.8e9, got={}, relative_error={}",
            max_slider_time,
            relative_error
        );

        // Verify log10 range
        let log_span = f64::log10(max_slider_time / min_slider_time);
        let expected_span = f64::log10(13.8e9 / 1e-40);
        assert!(
            (log_span - expected_span).abs() < 1e-10,
            "Log span should match expected value, got={}, expected={}",
            log_span,
            expected_span
        );
    }
}
