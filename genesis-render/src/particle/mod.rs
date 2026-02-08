//! Particle rendering components
//!
//! Defines the ParticleComponent marker for entities that represent
//! particles in the simulation. GPU-accelerated instanced rendering
//! is planned but not yet implemented.

use bevy::prelude::*;

/// Component marking an entity as a particle
#[derive(Component)]
pub struct ParticleComponent;

/// System to update particle positions based on physics
///
/// Currently a stub - actual particle physics updates need to be implemented.
pub fn update_particles(
    _query: Query<&mut Transform, With<ParticleComponent>>,
    _time: Res<Time>,
) {
    // TODO: Implement particle physics updates
}
