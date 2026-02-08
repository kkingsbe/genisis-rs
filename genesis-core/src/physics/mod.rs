//! Particle physics and interactions
//!
//! Core physics calculations for particle movement, expansion,
//! and cosmological evolution.

/// A single particle in the simulation
pub struct Particle {
    /// Position in 3D space
    pub position: [f32; 3],
    /// Velocity vector
    pub velocity: [f32; 3],
    /// RGB color
    pub color: [f32; 3],
    /// Size in world units
    pub size: f32,
}

impl Particle {
    pub fn new(position: [f32; 3], velocity: [f32; 3], color: [f32; 3], size: f32) -> Self {
        Self {
            position,
            velocity,
            color,
            size,
        }
    }
}
