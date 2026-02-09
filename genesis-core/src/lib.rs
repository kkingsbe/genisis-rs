//! GENESIS Core - Simulation logic and physics
//!
//! This crate contains core simulation logic for the Big Bang and
//! Cosmological Evolution Simulator, including time management resources,
//! particle physics data structures, and epoch tracking.

pub mod config;
pub mod time;
pub mod physics;
pub mod epoch;

pub use time::TimeIntegrationPlugin;
pub use config::{Config, CameraMode, WindowConfig, ParticleConfig, CameraConfig, TimeConfig, DisplayConfig};

/// Version of the core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
