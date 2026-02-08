//! Cosmic time management
//!
//! Handles the accumulation and scaling of cosmic time from the singularity
//! through 13.8 billion years of evolution.

pub struct TimeAccumulator {
    /// Accumulated cosmic time in years (f64 for precision)
    pub years: f64,
    /// Time acceleration factor (1x to 10^12x)
    pub acceleration: f64,
}

impl TimeAccumulator {
    pub fn new() -> Self {
        Self {
            years: 0.0,
            acceleration: 1.0,
        }
    }

    pub fn reset(&mut self) {
        self.years = 0.0;
    }

    pub fn set_acceleration(&mut self, accel: f64) {
        self.acceleration = accel.max(1.0).min(1e12);
    }
}

impl Default for TimeAccumulator {
    fn default() -> Self {
        Self::new()
    }
}
