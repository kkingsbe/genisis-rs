//! Particle rendering components and spawner system
//!
//! Defines the Particle component for entities that represent
//! particles in the simulation, along with a spawning system to
//! create particle clusters. Uses GPU-accelerated instanced rendering
//! (automatic batching) via shared mesh and material handles for
//! efficient rendering of thousands of particles.

use bevy::prelude::*;
use bevy::render::mesh::{SphereMeshBuilder, SphereKind};

/// Component representing a particle in the simulation
///
/// Contains position, color, and size attributes for rendering.
/// This component can be used with instanced rendering for
/// GPU-accelerated particle systems.
#[derive(Component, Clone)]
pub struct Particle {
    /// World space position of the particle
    pub position: Vec3,
    /// RGBA color of the particle
    pub color: Color,
    /// Particle size in world units
    pub size: f32,
}

/// System to spawn a cluster of particles around the origin
///
/// Creates particle entities in a dense cluster representing the
/// early universe. Uses GPU instancing (automatic batching) where
/// all entities share the same mesh and material handles for
/// efficient rendering.
///
/// This function spawns a test count of 1000 particles for development.
/// GPU instancing allows efficient rendering of 100K-1M particles.
///
/// In Bevy 0.15, GPU instancing is automatic: when multiple entities
/// share the same Mesh3d and MeshMaterial3d handles, the renderer
/// batches them for GPU instancing without requiring explicit
/// instance components.
pub fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a simple sphere mesh for particles (single mesh shared by all)
    let sphere_mesh = SphereMeshBuilder::new(1.0, SphereKind::Ico { subdivisions: 2 }).build();
    let mesh_handle = meshes.add(sphere_mesh);

    // Create an unlit emissive material for visibility (single material shared by all)
    let particle_material = StandardMaterial {
        unlit: true,
        emissive: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        ..default()
    };
    let material_handle = materials.add(particle_material);

    // Spawn particles in a cluster around the origin
    // Using 1000 particles for testing - will scale to 100K-1M later
    let particle_count = 1000;

    for i in 0..particle_count {
        // Simple deterministic pseudo-random distribution using loop index
        // This provides variation without requiring the rand crate
        let fi = i as f32;
        let scale = 10.0;
        let offset_x = ((fi * 123.456).fract() - 0.5) * scale;
        let offset_y = ((fi * 789.012).fract() - 0.5) * scale;
        let offset_z = ((fi * 345.678).fract() - 0.5) * scale;

        let position = Vec3::new(offset_x, offset_y, offset_z);

        // Deterministic color variation (mostly white/blue for early universe feel)
        let r = 0.8 + ((fi * 11.11).fract()) * 0.2;
        let g = 0.8 + ((fi * 22.22).fract()) * 0.2;
        let b = 1.0;

        let color = Color::srgba(r, g, b, 1.0);

        // Uniform particle size
        let size = 0.1;

        // Spawn particle entity with shared mesh/material handles
        // Bevy 0.15 automatically batches entities with same mesh/material for GPU instancing
        commands.spawn((
            Mesh3d(mesh_handle.clone()),      // Shared mesh handle
            MeshMaterial3d(material_handle.clone()),  // Shared material handle
            Transform::from_translation(position),  // Per-instance transform
            Particle { position, color, size },
        ));
    }
}

/// System to update particle positions based on physics
///
/// Currently a stub - actual particle physics updates need to be implemented.
pub fn update_particles(
    _query: Query<&mut Transform, With<Particle>>,
    _time: Res<Time>,
) {
    // TODO: Implement particle physics updates
}

/// Plugin for registering particle systems with the Bevy app
///
/// This plugin sets up the particle rendering system by registering
/// the `spawn_particles` system in the Startup schedule. It provides
/// a simple interface for adding particle functionality to the main
/// Bevy application.
///
/// # Usage
/// ```ignore
/// use bevy::prelude::*;
/// use genesis_render::particle::ParticlePlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins((DefaultPlugins, ParticlePlugin))
///         .run();
/// }
/// ```
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_particles);
    }
}
