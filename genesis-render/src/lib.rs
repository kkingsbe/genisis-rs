//! GENESIS Render - Rendering components and systems
//!
//! This crate contains rendering-related components and system definitions.
//! GPU-accelerated rendering systems are implemented with custom point sprite shader (WGSL).
//!
//! # Modules
//!
//! - [`camera`] - Camera mode definitions, controllers, and control systems (free-flight, orbit rotation)
//! - [`input`] - Input state tracking and handling systems (keyboard WASD, mouse motion)
//! - [`particle`] - Particle rendering components, spawner system, and GPU instancing
//!
//! # Public Exports
//!
//! This crate re-exports commonly-used types and plugins:
//! - `CameraPlugin` - Bevy plugin for camera systems (free-flight, orbit)
//! - `CameraState` - Resource tracking camera mode and orbit target
//!
//! # Camera System Implementation Status
//!
//! - **Free-flight mode**: Fully implemented with WASD movement and mouse look
//! - **Orbit mode**: Rotation implemented (left mouse drag), zoom and pan NOT implemented
//! - **Mode switching**: Implemented via 'O' key toggle
//! - **Camera interpolation**: NOT implemented (deferred to Phase 7)
//!
//! # Particle Rendering
//!
//! Implements GPU-accelerated point sprite rendering with:
//! - Custom WGSL shader for size attenuation and per-instance attributes
//! - Storage buffer infrastructure for per-instance data (size, color)
//! - Automatic GPU instancing via Bevy 0.15's shared mesh/material handles

pub mod camera;
pub mod input;
pub mod particle;

pub use camera::{CameraPlugin, CameraState};

/// Version of render library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
