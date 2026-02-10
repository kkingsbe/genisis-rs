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
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            time_acceleration_min: 1.0,
            time_acceleration_max: 1e12,
        }
    }
}

impl TimeConfig {
    /// Validates the time configuration
    ///
    /// # Returns
    /// * `Ok(())` if all validations pass
    /// * `Err(String)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        if self.time_acceleration_min <= 0.0 {
            return Err(format!(
                "TimeConfig.time_acceleration_min must be positive, got {}",
                self.time_acceleration_min
            ));
        }
        if self.time_acceleration_max <= 0.0 {
            return Err(format!(
                "TimeConfig.time_acceleration_max must be positive, got {}",
                self.time_acceleration_max
            ));
        }
        if self.time_acceleration_min >= self.time_acceleration_max {
            return Err(format!(
                "TimeConfig.time_acceleration_min ({}) must be less than time_acceleration_max ({})",
                self.time_acceleration_min, self.time_acceleration_max
            ));
        }
        Ok(())
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

impl ParticleConfig {
    /// Validates the particle configuration
    ///
    /// # Returns
    /// * `Ok(())` if all validations pass
    /// * `Err(String)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        if self.initial_count == 0 {
            return Err(format!(
                "ParticleConfig.initial_count must be positive, got {}",
                self.initial_count
            ));
        }
        if self.max_count == 0 {
            return Err(format!(
                "ParticleConfig.max_count must be positive, got {}",
                self.max_count
            ));
        }
        if self.initial_count > self.max_count {
            return Err(format!(
                "ParticleConfig.initial_count ({}) cannot exceed max_count ({})",
                self.initial_count, self.max_count
            ));
        }
        if self.base_size <= 0.0 {
            return Err(format!(
                "ParticleConfig.base_size must be positive, got {}",
                self.base_size
            ));
        }
        Ok(())
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

impl CameraConfig {
    /// Validates the camera configuration
    ///
    /// # Returns
    /// * `Ok(())` if all validations pass
    /// * `Err(String)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        if self.initial_mode != "free" && self.initial_mode != "orbit" {
            return Err(format!(
                "CameraConfig.initial_mode must be \"free\" or \"orbit\", got \"{}\"",
                self.initial_mode
            ));
        }
        if self.orbit_distance <= 0.0 {
            return Err(format!(
                "CameraConfig.orbit_distance must be positive, got {}",
                self.orbit_distance
            ));
        }
        Ok(())
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

impl WindowConfig {
    /// Validates the window configuration
    ///
    /// # Returns
    /// * `Ok(())` if all validations pass
    /// * `Err(String)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        if self.width == 0 {
            return Err(format!(
                "WindowConfig.width must be positive, got {}",
                self.width
            ));
        }
        if self.width > 3840 {
            return Err(format!(
                "WindowConfig.width ({}) exceeds maximum of 3840",
                self.width
            ));
        }
        if self.height == 0 {
            return Err(format!(
                "WindowConfig.height must be positive, got {}",
                self.height
            ));
        }
        if self.height > 2160 {
            return Err(format!(
                "WindowConfig.height ({}) exceeds maximum of 2160",
                self.height
            ));
        }
        if self.title.is_empty() {
            return Err("WindowConfig.title cannot be empty".to_string());
        }
        if self.title.len() > 200 {
            return Err(format!(
                "WindowConfig.title length ({}) exceeds maximum of 200",
                self.title.len()
            ));
        }
        Ok(())
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

impl DisplayConfig {
    /// Validates the display configuration
    ///
    /// # Returns
    /// * `Ok(())` if all validations pass
    /// * `Err(String)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        // DisplayConfig only contains boolean fields which are always valid
        Ok(())
    }
}

/// Physics configuration settings for cosmological parameters
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PhysicsConfig {
    /// Spectral index (n_s) for power spectrum P(k) ∝ k^(n_s – 1)
    /// Default value of 0.96 corresponds to the standard ΛCDM model
    pub spectral_index: f64,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            spectral_index: 0.96,
        }
    }
}

impl PhysicsConfig {
    /// Validates the physics configuration
    ///
    /// # Returns
    /// * `Ok(())` if all validations pass
    /// * `Err(String)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        // Spectral index should be in reasonable range for ΛCDM (approximately 0.9 to 1.1)
        // but we only enforce that it's non-negative here to allow flexibility
        if self.spectral_index < 0.0 {
            return Err(format!(
                "PhysicsConfig.spectral_index must be non-negative, got {}",
                self.spectral_index
            ));
        }
        Ok(())
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
    /// Physics configuration
    pub physics: PhysicsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            time: TimeConfig::default(),
            particle: ParticleConfig::default(),
            camera: CameraConfig::default(),
            window: WindowConfig::default(),
            display: DisplayConfig::default(),
            physics: PhysicsConfig::default(),
        }
    }
}

impl Config {
    /// Creates a new configuration with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates the configuration
    ///
    /// This method validates all sub-configurations: time, particle, camera, window, display, and physics.
    /// It short-circuits and returns the first error encountered.
    ///
    /// # Returns
    /// * `Ok(())` if all validations pass
    /// * `Err(String)` with a descriptive error message if any validation fails
    pub fn validate(&self) -> Result<(), String> {
        // Validate each sub-config, returning the first error encountered
        self.time.validate()?;
        self.particle.validate()?;
        self.camera.validate()?;
        self.window.validate()?;
        self.display.validate()?;
        self.physics.validate()?;
        Ok(())
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
    /// If a file is found but validation fails, a warning is printed and the next path is tried.
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
                            // Validate the parsed configuration
                            match config.validate() {
                                Ok(()) => {
                                    return config;
                                }
                                Err(error) => {
                                    eprintln!(
                                        "Configuration file '{}' failed validation: {}. Using default configuration.",
                                        path, error
                                    );
                                    // Continue to try the next path
                                }
                            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeconfig_validate_valid() {
        let config = TimeConfig {
            time_acceleration_min: 1.0,
            time_acceleration_max: 1e12,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_timeconfig_validate_min_zero() {
        let config = TimeConfig {
            time_acceleration_min: 0.0,
            time_acceleration_max: 1e12,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be positive"));
    }

    #[test]
    fn test_timeconfig_validate_max_zero() {
        let config = TimeConfig {
            time_acceleration_min: 1.0,
            time_acceleration_max: 0.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be positive"));
    }

    #[test]
    fn test_timeconfig_validate_min_greater_than_max() {
        let config = TimeConfig {
            time_acceleration_min: 1000.0,
            time_acceleration_max: 100.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be less than"));
    }

    #[test]
    fn test_timeconfig_validate_min_equals_max() {
        let config = TimeConfig {
            time_acceleration_min: 100.0,
            time_acceleration_max: 100.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be less than"));
    }

    #[test]
    fn test_timeconfig_validate_negative_min() {
        let config = TimeConfig {
            time_acceleration_min: -1.0,
            time_acceleration_max: 1e12,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be positive"));
    }

    #[test]
    fn test_timeconfig_validate_negative_max() {
        let config = TimeConfig {
            time_acceleration_min: 1.0,
            time_acceleration_max: -1.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be positive"));
    }

    #[test]
    fn test_timeconfig_default_is_valid() {
        let config = TimeConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_particleconfig_validate_valid() {
        let config = ParticleConfig {
            initial_count: 100_000,
            max_count: 1_000_000,
            base_size: 2.0,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_particleconfig_validate_initial_count_zero() {
        let config = ParticleConfig {
            initial_count: 0,
            max_count: 1_000_000,
            base_size: 2.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("initial_count must be positive"));
    }

    #[test]
    fn test_particleconfig_validate_max_count_zero() {
        let config = ParticleConfig {
            initial_count: 100_000,
            max_count: 0,
            base_size: 2.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("max_count must be positive"));
    }

    #[test]
    fn test_particleconfig_validate_initial_exceeds_max() {
        let config = ParticleConfig {
            initial_count: 1_000_000,
            max_count: 100_000,
            base_size: 2.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot exceed max_count"));
    }

    #[test]
    fn test_particleconfig_validate_base_size_zero() {
        let config = ParticleConfig {
            initial_count: 100_000,
            max_count: 1_000_000,
            base_size: 0.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("base_size must be positive"));
    }

    #[test]
    fn test_particleconfig_validate_base_size_negative() {
        let config = ParticleConfig {
            initial_count: 100_000,
            max_count: 1_000_000,
            base_size: -1.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("base_size must be positive"));
    }

    #[test]
    fn test_particleconfig_default_is_valid() {
        let config = ParticleConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_cameraconfig_validate_valid_free() {
        let config = CameraConfig {
            initial_mode: "free".to_string(),
            orbit_distance: 100.0,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_cameraconfig_validate_valid_orbit() {
        let config = CameraConfig {
            initial_mode: "orbit".to_string(),
            orbit_distance: 100.0,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_cameraconfig_validate_invalid_mode() {
        let config = CameraConfig {
            initial_mode: "invalid".to_string(),
            orbit_distance: 100.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("initial_mode must be \"free\" or \"orbit\""));
    }

    #[test]
    fn test_cameraconfig_validate_orbit_distance_zero() {
        let config = CameraConfig {
            initial_mode: "orbit".to_string(),
            orbit_distance: 0.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("orbit_distance must be positive"));
    }

    #[test]
    fn test_cameraconfig_validate_orbit_distance_negative() {
        let config = CameraConfig {
            initial_mode: "orbit".to_string(),
            orbit_distance: -50.0,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("orbit_distance must be positive"));
    }

    #[test]
    fn test_cameraconfig_default_is_valid() {
        let config = CameraConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_windowconfig_validate_valid() {
        let config = WindowConfig {
            width: 1920,
            height: 1080,
            title: "Test Window".to_string(),
            vsync: true,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_windowconfig_validate_width_zero() {
        let config = WindowConfig {
            width: 0,
            height: 1080,
            title: "Test Window".to_string(),
            vsync: true,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("width must be positive"));
    }

    #[test]
    fn test_windowconfig_validate_width_exceeds_max() {
        let config = WindowConfig {
            width: 4000,
            height: 1080,
            title: "Test Window".to_string(),
            vsync: true,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("width (4000) exceeds maximum of 3840"));
    }

    #[test]
    fn test_windowconfig_validate_height_zero() {
        let config = WindowConfig {
            width: 1920,
            height: 0,
            title: "Test Window".to_string(),
            vsync: true,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("height must be positive"));
    }

    #[test]
    fn test_windowconfig_validate_height_exceeds_max() {
        let config = WindowConfig {
            width: 1920,
            height: 3000,
            title: "Test Window".to_string(),
            vsync: true,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("height (3000) exceeds maximum of 2160"));
    }

    #[test]
    fn test_windowconfig_validate_empty_title() {
        let config = WindowConfig {
            width: 1920,
            height: 1080,
            title: "".to_string(),
            vsync: true,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("title cannot be empty"));
    }

    #[test]
    fn test_windowconfig_validate_title_exceeds_max_length() {
        let long_title = "a".repeat(201);
        let config = WindowConfig {
            width: 1920,
            height: 1080,
            title: long_title,
            vsync: true,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("title length (201) exceeds maximum of 200"));
    }

    #[test]
    fn test_windowconfig_default_is_valid() {
        let config = WindowConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_displayconfig_validate_valid() {
        let config = DisplayConfig {
            show_fps: true,
            show_particle_count: true,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_displayconfig_validate_all_false() {
        let config = DisplayConfig {
            show_fps: false,
            show_particle_count: false,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_displayconfig_default_is_valid() {
        let config = DisplayConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_valid() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_invalid_timeconfig_min_zero() {
        let config = Config {
            time: TimeConfig {
                time_acceleration_min: 0.0,
                time_acceleration_max: 1e12,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("TimeConfig"));
    }

    #[test]
    fn test_config_validate_invalid_particleconfig_initial_count_zero() {
        let config = Config {
            particle: ParticleConfig {
                initial_count: 0,
                max_count: 1_000_000,
                base_size: 2.0,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("ParticleConfig"));
    }

    #[test]
    fn test_config_validate_invalid_cameraconfig_mode() {
        let config = Config {
            camera: CameraConfig {
                initial_mode: "invalid".to_string(),
                orbit_distance: 100.0,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("CameraConfig"));
    }

    #[test]
    fn test_config_validate_invalid_windowconfig_width_zero() {
        let config = Config {
            window: WindowConfig {
                width: 0,
                height: 1080,
                title: "Test Window".to_string(),
                vsync: true,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("WindowConfig"));
    }

    #[test]
    fn test_config_load_with_invalid_config_file() {
        use std::fs;
        use std::path::PathBuf;

        // Create a temporary config file with invalid camera mode
        let test_path = PathBuf::from("test_genesis_invalid.toml");
        let test_content = r#"
[window]
width = 1280
height = 720
title = "Test Window"
vsync = true

[particle]
initial_count = 100000
max_count = 1000000
base_size = 2.0

[camera]
initial_mode = "invalid_mode"
orbit_distance = 100.0

[time]
time_acceleration_min = 1.0
time_acceleration_max = 1000000000000.0

[display]
show_fps = true
show_particle_count = true
"#;
        fs::write(&test_path, test_content).expect("Failed to write test config file");

        // Temporarily move genesis.toml out of the way if it exists
        let backup_path = PathBuf::from("genesis.toml.bak");
        if PathBuf::from("genesis.toml").exists() {
            fs::rename("genesis.toml", &backup_path).expect("Failed to backup genesis.toml");
        }

        // Create a symbolic link from genesis.toml to the test file
        #[cfg(unix)]
        std::os::unix::fs::symlink("test_genesis_invalid.toml", "genesis.toml")
            .expect("Failed to create symlink");

        #[cfg(windows)]
        std::os::windows::fs::symlink_file("test_genesis_invalid.toml", "genesis.toml")
            .expect("Failed to create symlink");

        // Load the config - it should fail validation and return defaults
        let config = Config::load();

        // Verify we got default values
        assert_eq!(config.camera.initial_mode, "orbit");
        assert_eq!(config.camera.orbit_distance, 100.0);

        // Clean up
        let _ = fs::remove_file("genesis.toml");
        let _ = fs::remove_file(&test_path);

        // Restore original genesis.toml if it existed
        if backup_path.exists() {
            let _ = fs::rename(&backup_path, "genesis.toml");
        }
    }
}
