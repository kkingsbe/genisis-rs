//! Instanced particle rendering
//!
//! GPU-accelerated rendering of up to 1M particles using Bevy's
//! instancing system with PBR materials.

use bevy::prelude::*;

/// Component marking an entity as a particle
#[derive(Component)]
pub struct ParticleComponent;

/// System to update particle positions based on physics
pub fn update_particles(
    mut query: Query<&mut Transform, With<ParticleComponent>>,
    time: Res<Time>,
) {
    // TODO: Implement particle physics updates
}
