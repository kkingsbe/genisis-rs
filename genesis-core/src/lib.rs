//! GENESIS Core - Simulation logic and physics
//!
//! This crate contains core simulation logic for the Big Bang and
//! Cosmological Evolution Simulator, including time management resources,
//! particle physics data structures, and epoch tracking.
//!
//! # Modules
//!
//! - [`config`] - Configuration structures for Phase 1 parameters with TOML deserialization support
//! - [`epoch`] - Epoch markers representing different phases of cosmological evolution (currently: Singularity)
//! - [`physics`] - Particle physics data structures (simulation-level Particle for physics calculations)
//! - [`time`] - Cosmic time management with f64 precision accumulator and time integration systems
//!
//! # Public Exports
//!
//! This crate re-exports commonly-used types and plugins:
//! - `Config`, `CameraConfig`, `ParticleConfig`, `TimeConfig`, `WindowConfig` - Configuration types
//! - `SingularityEpoch` - Epoch marker for the Singularity phase
//! - `TimeIntegrationPlugin` - Bevy plugin for cosmic time accumulation
//!
//! # Two-Level Particle Architecture
//!
//! This crate provides **simulation-level Particle** (in [`physics::Particle`](physics/struct.Particle.html))
//! that uses plain Rust types for efficient physics calculations. The rendering-level Particle
//! (in `genesis-render::particle::Particle`) uses Bevy ECS components and is attached to
//! entities for GPU rendering.

pub mod config;
pub mod epoch;
pub mod physics;
pub mod time;

pub use config::{
    CameraConfig, Config, ParticleConfig,
    TimeConfig, WindowConfig,
};
pub use epoch::SingularityEpoch;
pub use time::TimeIntegrationPlugin;

/// Version of the core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
