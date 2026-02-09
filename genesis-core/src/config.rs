//! Configuration for Genesis Engine
//!
//! This module defines the configuration structure for Phase 1 parameters.
//! The configuration supports TOML deserialization and provides sensible defaults.
//!
//! # Implementation Status
//!
//! - Config structs are fully defined with Default implementations
//! - TOML serialization/deserialization via serde is configured
//! - Config::load() method is implemented and reads from ./genesis.toml
//! - ParticleConfig field names match genesis.toml (initial_count, max_count, base_size)
//! - CameraConfig field names match genesis.toml (initial_mode, orbit_distance)

use serde::{Deserialize, Serialize};
use bevy::prelude::Resource;
use std::fs;
use std::path::Path;

/// Time configuration settings for cosmic simulation
#[derive(Debug, Clone, Deserialize)]
pub struct TimeConfig {
    /// Minimum time acceleration factor (1.0 = no acceleration)
    pub time_acceleration_min: f64,
    /// Maximum time acceleration factor (10¹² = maximum acceleration)
    pub time_acceleration_max: f64,
    /// Initial time acceleration factor
    pub initial_time_acceleration: f64,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            time_acceleration_min: 1.0,
            time_acceleration_max: 1e12,
            initial_time_acceleration: 1.0,
        }
    }
}

/// Particle system configuration settings
#[derive(Debug, Clone, Deserialize, Resource)]
pub struct ParticleConfig {
    /// Initial number of particles to simulate
    pub initial_count: usize,
    /// Maximum number of particles allowed
    pub max_count: usize,
    /// Base size for particle rendering in world units
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

/// Camera system configuration settings
#[derive(Debug, Clone, Deserialize)]
pub struct CameraConfig {
    /// Initial camera mode: "free" or "orbit"
    pub initial_mode: String,
    /// Default orbit distance for orbit camera mode
    pub orbit_distance: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            initial_mode: "orbit".to_string(),
            orbit_distance: 100.0,
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
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_fps: true,
            show_particle_count: true,
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

    /// Loads configuration from a TOML file
    ///
    /// This method searches for configuration files in the following order:
    /// 1. `./genesis.toml` - Workspace-local configuration
    /// 2. `~/.config/genesis/config.toml` - User-specific configuration
    /// 3. `/etc/genesis/config.toml` - System-wide configuration
    ///
    /// If no configuration file is found, default values are returned.
    /// If a file is found but parsing fails, a warning is printed and default values are returned.
    #[must_use]
    pub fn load() -> Self {
        // Define the search paths in priority order
        let paths = vec![
            "./genesis.toml".to_string(),
            dirs::config_local_dir()
                .map(|p| p.join("genesis").join("config.toml"))
                .and_then(|p| p.to_str().map(|s| s.to_string()))
                .unwrap_or_else(|| "".to_string()),
            "/etc/genesis/config.toml".to_string(),
        ];

        // Try each path in order
        for path in paths {
            if path.is_empty() {
                continue;
            }

            let config_path = Path::new(&path);
            if !config_path.exists() {
                continue;
            }

            // Read the file content
            match fs::read_to_string(config_path) {
                Ok(content) => {
                    // Parse the TOML content
                    match toml::from_str::<Self>(&content) {
                        Ok(config) => {
                            return config;
                        }
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to parse configuration file '{}': {}",
                                path, e
                            );
                            eprintln!("Falling back to default configuration.");
                            return Self::default();
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read configuration file '{}': {}",
                        path, e
                    );
                    continue;
                }
            }
        }

        // No configuration file found, return defaults
        Self::default()
    }
}
