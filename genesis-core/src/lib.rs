//! GENESIS Core - Simulation logic and physics
//!
//! This crate contains core simulation logic for the Big Bang and
//! Cosmological Evolution Simulator, including time management resources,
//! particle physics data structures, and epoch tracking.

pub mod config;
pub mod epoch;
pub mod physics;
pub mod time;

pub use config::{
    CameraConfig, CameraMode, CliArgs, Config, DisplayConfig, ParticleConfig, ParticleConfigResource,
    TimeConfig, WindowConfig,
};
pub use epoch::singularity::SingularityEpoch;
pub use epoch::{EpochChangeEvent, EpochManager, EpochManagerPlugin};
pub use time::TimeIntegrationPlugin;

/// Version of the core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
