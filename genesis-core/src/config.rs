//! Configuration for Genesis Engine
//!
//! This module defines the configuration structure for Phase 1 parameters.
//! The configuration supports TOML deserialization and provides sensible defaults.

use serde::{Deserialize, Serialize};
use bevy::prelude::Resource;

/// Time configuration settings for cosmic simulation
#[derive(Debug, Clone, Deserialize)]
pub struct TimeConfig {
    /// Initial cosmic time in seconds (e.g., 10⁻⁴³s for singularity)
    pub initial_time: f64,
    /// Minimum time acceleration factor (1.0 = no acceleration)
    pub time_acceleration_min: f64,
    /// Maximum time acceleration factor (10¹² = maximum acceleration)
    pub time_acceleration_max: f64,
    /// Default time acceleration factor
    pub default_time_acceleration: f64,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            initial_time: 1e-43,
            time_acceleration_min: 1.0,
            time_acceleration_max: 1e12,
            default_time_acceleration: 1.0,
        }
    }
}

/// Particle system configuration settings
#[derive(Debug, Clone, Deserialize, Resource)]
pub struct ParticleConfig {
    /// Number of particles to simulate
    pub particle_count: usize,
    /// Base size for particle rendering in world units
    pub particle_size_base: f32,
    /// Random variation factor for particle sizes
    pub particle_size_variation: f32,
    /// RGBA color for hot particles (e.g., white-hot)
    pub color_hot: [f32; 4],
    /// RGBA color for cooled particles
    pub color_cool: [f32; 4],
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            particle_count: 10_000,
            particle_size_base: 2.0,
            particle_size_variation: 0.5,
            color_hot: [1.0, 1.0, 1.0, 1.0],
            color_cool: [1.0, 0.3, 0.0, 1.0],
        }
    }
}

/// Camera system configuration settings
#[derive(Debug, Clone, Deserialize)]
pub struct CameraConfig {
    /// Initial camera position [x, y, z]
    pub initial_position: [f64; 3],
    /// Initial camera target/look-at point [x, y, z]
    pub initial_target: [f64; 3],
    /// Camera mode: "free" or "orbit"
    pub camera_mode: String,
    /// Movement speed for free-flight camera mode
    pub movement_speed: f64,
    /// Default orbit radius for orbit camera mode
    pub orbit_radius: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            initial_position: [0.0, 0.0, 100.0],
            initial_target: [0.0, 0.0, 0.0],
            camera_mode: "orbit".to_string(),
            movement_speed: 10.0,
            orbit_radius: 100.0,
        }
    }
}

/// Window configuration settings
#[derive(Debug, Clone, Deserialize)]
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

/// Display/HUD configuration settings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DisplayConfig {
    /// Display FPS counter in HUD
    pub show_fps: bool,
    /// Display particle count
    pub show_particle_count: bool,
    /// Display current epoch information
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
/// It supports deserialization from TOML format and provides sensible defaults.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Time configuration
    pub time: TimeConfig,
    /// Particle system configuration
    pub particle: ParticleConfig,
    /// Camera configuration
    pub camera: CameraConfig,
    /// Window configuration
    pub window: WindowConfig,
    /// Display/HUD configuration
    pub display: DisplayConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            time: TimeConfig::default(),
            particle: ParticleConfig::default(),
            camera: CameraConfig::default(),
            window: WindowConfig::default(),
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
