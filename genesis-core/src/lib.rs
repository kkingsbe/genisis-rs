//! GENESIS Core - Simulation logic and physics
//!
//! This crate contains the core simulation logic for the Big Bang and
//! Cosmological Evolution Simulator, including time management,
//! particle physics, and epoch tracking.

pub mod time;
pub mod physics;

/// Version of the core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
