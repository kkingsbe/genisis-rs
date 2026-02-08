//! GENESIS Render - Rendering systems and visuals
//!
//! This crate contains all rendering-related code using Bevy ECS,
//! including particle rendering, camera systems, and visual effects.

pub mod particle;
pub mod camera;

/// Version of the render library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
