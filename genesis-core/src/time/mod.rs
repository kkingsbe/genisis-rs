//! Cosmic time management
//!
//! Defines resource for tracking and accumulating cosmic time.
//! Includes SECONDS_PER_YEAR constant and time scaling via acceleration factor.

use bevy::prelude::{Commands, Plugin, Res, ResMut, Resource, Startup, Update};
use bevy::time::Time;

/// Number of seconds in a cosmic year (365.25 days)
const SECONDS_PER_YEAR: f64 = 31_557_600.0;

#[derive(Resource)]
pub struct TimeAccumulator {
    /// Accumulated cosmic time in years (f64 for precision)
    pub years: f64,
    /// Time acceleration factor (1x to 10^12x)
    pub acceleration: f64,
    /// Pause state - when true, cosmic time does not accumulate
    paused: bool,
}

impl TimeAccumulator {
    pub fn new() -> Self {
        Self {
            years: 0.0,
            acceleration: 1.0,
            paused: false,
        }
    }

    pub fn reset(&mut self) {
        self.years = 0.0;
        self.paused = false;
    }

    pub fn set_acceleration(&mut self, accel: f64) {
        self.acceleration = accel.max(1.0).min(1e12);
    }

    /// Adds elapsed real time to the cosmic time accumulator.
    ///
    /// # Arguments
    /// * `delta_seconds` - Elapsed real time in seconds since the last frame
    ///
    /// The elapsed time is converted to cosmic years using the current
    /// acceleration factor. Higher acceleration values result in faster
    /// passage of cosmic time.
    ///
    /// Note: Time accumulation is skipped when `paused` is true.
    pub fn add_time(&mut self, delta_seconds: f64) {
        if self.paused {
            return;
        }
        let delta_years = delta_seconds * self.acceleration / SECONDS_PER_YEAR;
        self.years += delta_years;
    }

    /// Pauses cosmic time accumulation.
    ///
    /// When paused, the `add_time` method will not accumulate time.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resumes cosmic time accumulation.
    ///
    /// When resumed, the `add_time` method will accumulate time normally.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Toggles the pause state.
    ///
    /// If currently paused, resumes. If currently running, pauses.
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Returns whether cosmic time accumulation is currently paused.
    ///
    /// # Returns
    /// `true` if paused, `false` if running
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

impl Default for TimeAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin for integrating cosmic time accumulation with Bevy's time system.
///
/// Initializes the `TimeAccumulator` resource and adds a system that updates
/// the cosmic time each frame based on Bevy's delta time.
pub struct TimeIntegrationPlugin;

impl Plugin for TimeIntegrationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Initialize the TimeAccumulator resource with default values
        app.add_systems(Startup, initialize_time_accumulator);

        // Add update system to accumulate cosmic time each frame
        app.add_systems(Update, update_cosmic_time);
    }
}

/// Startup system that initializes the TimeAccumulator resource.
fn initialize_time_accumulator(mut commands: Commands) {
    commands.insert_resource(TimeAccumulator::default());
}

/// Update system that reads Bevy's delta time and adds it to the cosmic time accumulator.
fn update_cosmic_time(time: Res<Time>, mut time_accumulator: ResMut<TimeAccumulator>) {
    let delta_seconds = time.delta_secs() as f64;
    time_accumulator.add_time(delta_seconds);
}
