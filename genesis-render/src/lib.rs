//! GENESIS Render - Rendering components and systems
//!
//! This crate contains rendering-related components and system definitions.
//! GPU-accelerated rendering systems are implemented (custom point sprite shader with WGSL).

pub mod particle;
pub mod camera;
pub mod input;

pub use camera::CameraPlugin;

/// Version of the render library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
