//! Configuration for Genesis Engine
//!
//! This module defines the configuration structure for Phase 1 parameters.
//! The configuration supports TOML serialization and provides sensible defaults.
//!
//! # TOML Configuration File Format
//!
//! The configuration file should use the following TOML structure:
//!
//! ```toml
//! # Window configuration
//! [window]
//! width = 1280
//! height = 720
//! title = "Genesis Engine - Big Bang Simulator"
//! vsync = true
//!
//! # Particle system configuration
//! [particle]
//! initial_count = 100000
//! max_count = 1000000
//! base_size = 2.0
//!
//! # Camera configuration
//! [camera]
//! initial_mode = "orbit"  # or "free_flight"
//! orbit_distance = 100.0
//!
//! # Time configuration
//! [time]
//! time_acceleration_min = 0.1
//! time_acceleration_max = 1000000000000.0
//! initial_time_acceleration = 1.0
//!
//! # Display/HUD configuration
//! [display]
//! show_fps = true
//! show_particle_count = true
//! show_epoch_info = true
//! ```

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

/// Command-line arguments for Genesis Engine
#[derive(Parser, Debug)]
#[command(name = "genesis")]
#[command(about = "Real-time Big Bang & Cosmological Evolution Simulator", long_about = None)]
pub struct CliArgs {
    /// Path to TOML configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,
}

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
            time_acceleration_min: 1.0,
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

    /// Loads configuration from a TOML file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Returns
    ///
    /// Returns `Ok(Config)` if the file was successfully read and parsed,
    /// or an error if the file cannot be read or contains invalid TOML.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be read (e.g., file not found, permission denied)
    /// - The file contains invalid TOML syntax
    /// - The TOML structure does not match the Config structure
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Loads configuration from an optional path or searches default locations.
    ///
    /// If a path is provided, it attempts to load the configuration from that path.
    /// If the path is `None`, it searches the following locations in order:
    ///
    /// 1. `./genesis.toml` (current directory)
    /// 2. `~/.config/genesis/config.toml` (user config directory)
    /// 3. `/etc/genesis/config.toml` (system-wide config)
    ///
    /// If no configuration file is found in any of the default locations,
    /// it returns the default configuration.
    ///
    /// # Arguments
    ///
    /// * `path` - Optional path to a specific configuration file
    ///
    /// # Returns
    ///
    /// Returns `Ok(Config)` with either the loaded configuration or the default
    /// if no file was found.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - A specific path is provided but the file cannot be read
    /// - The file contains invalid TOML syntax
    /// - The TOML structure does not match the Config structure
    pub fn load_from_path(path: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(p) = path {
            // Load from the specified path
            return Self::load_from_file(&p);
        }

        // Define default locations to search
        let default_locations = [
            "./genesis.toml".to_string(),
            "~/.config/genesis/config.toml".to_string(),
            "/etc/genesis/config.toml".to_string(),
        ];

        // Try each location in order
        for location in default_locations {
            // Expand tilde for home directory if present
            let expanded_path = if location.starts_with("~/") {
                match env::var("HOME") {
                    Ok(home) => format!("{}/{}", home, &location[2..]),
                    Err(_) => location,
                }
            } else {
                location
            };

            // Check if file exists before attempting to read
            if Path::new(&expanded_path).exists() {
                match Self::load_from_file(&expanded_path) {
                    Ok(config) => return Ok(config),
                    Err(_) => continue, // Try next location if this one fails
                }
            }
        }

        // No config file found, return default configuration
        Ok(Self::default())
    }

    /// Loads configuration from command-line arguments.
    ///
    /// This method parses the command-line arguments to extract the --config flag,
    /// then loads the configuration from the specified path (if provided) or searches
    /// default locations.
    ///
    /// # Returns
    ///
    /// Returns a tuple of `(Config, CliArgs)` containing the loaded configuration
    /// and the parsed CLI arguments.
    ///
    /// # Panics
    ///
    /// This method will panic if command-line argument parsing fails.
    pub fn from_cli_args() -> (Self, CliArgs) {
        let args = CliArgs::parse();
        let config =
            Self::load_from_path(args.config.clone()).expect("Failed to load configuration");
        (config, args)
    }

    /// Convenience method to load configuration from CLI arguments.
    ///
    /// This method parses command-line arguments and returns only the
    /// loaded configuration, discarding the parsed arguments.
    ///
    /// # Returns
    ///
    /// Returns the loaded `Config`.
    ///
    /// # Panics
    ///
    /// This method will panic if command-line argument parsing fails or
    /// configuration loading fails.
    #[must_use]
    pub fn load() -> Self {
        let (config, _args) = Self::from_cli_args();
        config
    }
}
