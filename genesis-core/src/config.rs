//! Configuration for Genesis Engine
//!
//! This module defines the configuration structure for Phase 1 parameters.
//! The configuration supports TOML serialization and provides sensible defaults.

use serde::{Deserialize, Serialize};

/// Camera mode enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CameraMode {
    /// Free flight camera mode for unrestricted navigation
    FreeFlight,
    /// Orbit camera mode for rotating around a central point
    Orbit,
}

impl Default for CameraMode {
    fn default() -> Self {
        Self::Orbit
    }
}

/// Window configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window width in pixels
    pub width: u32,
    /// Window height in pixels
    pub height: u32,
    /// Window title
    pub title: String,
    /// Vertical synchronization (vsync) enabled
    pub vsync: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "Genesis Engine - Big Bang Simulator".to_string(),
            vsync: true,
        }
    }
}

/// Particle system configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleConfig {
    /// Initial number of particles to spawn
    pub initial_count: usize,
    /// Maximum number of particles allowed
    pub max_count: usize,
    /// Base size for particle rendering
    pub base_size: f32,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            initial_count: 100_000,
            max_count: 1_000_000,
            base_size: 2.0,
        }
    }
}

/// Camera configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    /// Initial camera mode
    pub initial_mode: CameraMode,
    /// Distance for orbit camera mode
    pub orbit_distance: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            initial_mode: CameraMode::Orbit,
            orbit_distance: 100.0,
        }
    }
}

/// Time configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConfig {
    /// Minimum time acceleration factor
    pub time_acceleration_min: f64,
    /// Maximum time acceleration factor (10^12 for cosmic scale)
    pub time_acceleration_max: f64,
    /// Initial time acceleration factor
    pub initial_time_acceleration: f64,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            time_acceleration_min: 0.1,
            time_acceleration_max: 1_000_000_000_000.0, // 10^12
            initial_time_acceleration: 1.0,
        }
    }
}

/// Display/HUD configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Show FPS counter
    pub show_fps: bool,
    /// Show particle count
    pub show_particle_count: bool,
    /// Show epoch information
    pub show_epoch_info: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_fps: true,
            show_particle_count: true,
            show_epoch_info: true,
        }
    }
}

/// Main configuration structure for Genesis Engine
///
/// This struct contains all Phase 1 parameters for the engine configuration.
/// It supports serialization to/from TOML format and provides sensible defaults
/// corresponding to the "Standard Model" preset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Window configuration
    pub window: WindowConfig,
    /// Particle system configuration
    pub particle: ParticleConfig,
    /// Camera configuration
    pub camera: CameraConfig,
    /// Time configuration
    pub time: TimeConfig,
    /// Display/HUD configuration
    pub display: DisplayConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            particle: ParticleConfig::default(),
            camera: CameraConfig::default(),
            time: TimeConfig::default(),
            display: DisplayConfig::default(),
        }
    }
}

impl Config {
    /// Creates a new configuration with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
