//! Particle physics data structures
//!
//! Defines basic particle data structure for simulation state.
//! Physics calculations and cosmological evolution systems are not yet implemented.
//!
//! # Two-Level Particle Architecture
//!
//! This crate defines the **simulation-level Particle** that stores core physics data:
//! - Uses native Rust types ([f32; 3] for position/velocity/color, f32 for size)
//! - Designed for physics calculations in future cosmological simulation stages
//!
//! The rendering-level Particle (defined in `genesis-render::particle::Particle`) uses
//! Bevy ECS components (Vec3, Color) and is attached to entities for rendering.

/// A single particle in the simulation
///
/// Basic particle data structure storing position, velocity, color, and size.
/// Physics movement and interaction systems need to be implemented separately.
///
/// This is the **simulation-level** particle used for physics calculations.
/// It uses plain Rust types (arrays, not Bevy types) for efficient
/// computational processing during cosmological evolution simulations.
///
/// Fields:
/// - `position`: 3D coordinates in world space
/// - `velocity`: 3D velocity vector
/// - `color`: RGB color values [0.0, 1.0] for each channel
/// - `size`: Particle size in world units
///
/// # Note
///
/// This is distinct from [`genesis_render::particle::Particle`](../genesis_render/particle/struct.Particle.html),
/// which is the **rendering-level** component attached to Bevy entities for GPU rendering.
pub struct Particle {
    /// Position in 3D space [x, y, z]
    pub position: [f32; 3],
    /// Velocity vector [vx, vy, vz]
    pub velocity: [f32; 3],
    /// RGB color values [r, g, b], each in range [0.0, 1.0]
    pub color: [f32; 3],
    /// Size in world units
    pub size: f32,
}

impl Particle {
    /// Creates a new Particle with the specified parameters.
    ///
    /// # Arguments
    /// * `position` - 3D position in world space [x, y, z]
    /// * `velocity` - 3D velocity vector [vx, vy, vz]
    /// * `color` - RGB color values [r, g, b], each in range [0.0, 1.0]
    /// * `size` - Particle size in world units
    ///
    /// # Returns
    /// A new Particle instance with the specified properties
    pub fn new(position: [f32; 3], velocity: [f32; 3], color: [f32; 3], size: f32) -> Self {
        Self {
            position,
            velocity,
            color,
            size,
        }
    }
}
