//! GENESIS Render - Rendering components and systems
//!
//! This crate contains rendering-related components and system definitions.
//! Full GPU-accelerated rendering systems are not yet implemented.

pub mod particle;
pub mod camera;
pub mod input;

pub use camera::CameraPlugin;

/// Version of the render library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
