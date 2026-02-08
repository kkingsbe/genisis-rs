//! Particle rendering components and spawner system
//!
//! Defines the Particle component for entities that represent
//! particles in the simulation, along with a spawning system to
//! create particle clusters. Uses GPU-accelerated instanced rendering
//! (automatic batching) via shared mesh and material handles for
//! efficient rendering of thousands of particles.

use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology, VertexAttribute, VertexFormat};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::ShaderRef;
use bevy::render::alpha::AlphaMode;
use bevy::render::render_resource::AsBindGroup;
use bevy::asset::Asset;
use bevy::pbr::Material;

/// Custom vertex attribute for per-instance particle size
const ATTRIBUTE_INSTANCE_SIZE: VertexAttribute = VertexAttribute::new("instance_size", VertexFormat::Float32);

/// Point sprite material for efficient particle rendering
///
/// Uses a custom WGSL shader to render particles as GPU point sprites
/// instead of mesh spheres. This is significantly more efficient for
/// rendering large numbers of particles (100K-1M).
///
/// The material uses additive blending for a glowing effect.
#[derive(Asset, TypePath, Clone, AsBindGroup)]
pub struct PointSpriteMaterial {
    /// Uniform color for all particles using this material
    #[uniform(0)]
    pub color: LinearRgba,
    /// Base size of particles in pixels before attenuation
    #[uniform(1)]
    pub base_size: f32,
    /// Attenuation factor for size attenuation
    /// Controls how quickly particles shrink with distance
    /// Formula: size = base_size / (1.0 + distance * attenuation_factor)
    #[uniform(2)]
    pub attenuation_factor: f32,
}

impl Material for PointSpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        "point_sprite.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "point_sprite.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

/// Resource storing the shared point mesh for all particles
///
/// Point sprites use a minimal mesh with a single vertex at the origin.
/// The Transform component provides the actual position for each particle.
/// This resource is initialized once at startup and shared across all
/// particle entities for efficient rendering.
#[derive(Resource, Clone)]
pub struct PointMesh(pub Handle<Mesh>);

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

/// Startup system to initialize the shared point mesh resource
///
/// Creates a simple point mesh with a single vertex at the origin.
/// This mesh is reused by all particle entities for efficient rendering.
/// The Transform component on each particle entity provides the actual
/// position in world space.
/// 
/// The mesh includes an instance_size attribute for per-instance particle size.
/// This allows the vertex shader to apply size attenuation based on each particle's size.
pub fn init_point_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Create a simple point mesh with PointList topology
    // Single vertex at origin since Transform provides actual position
    let mut mesh = Mesh::new(
        PrimitiveTopology::PointList,
        RenderAssetUsages::default(),
    );
    
    // Add a single vertex at the origin
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0]],
    );
    
    // Add instance_size attribute for per-instance particle size
    // This will be at location(1) to match the shader's VertexInput
    mesh.insert_attribute(
        ATTRIBUTE_INSTANCE_SIZE,
        vec![1.0f32],  // Default size, will be updated per-instance
    );
    
    let mesh_handle = meshes.add(mesh);
    commands.insert_resource(PointMesh(mesh_handle));
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
    mut materials: ResMut<Assets<PointSpriteMaterial>>,
    point_mesh: Res<PointMesh>,
) {
    // Create point sprite material for all particles (single material shared by all)
    // Using white color for visibility - individual particle colors will be
    // handled by the shader in future updates
    let particle_material = PointSpriteMaterial {
        color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        base_size: 100.0,  // Base size in pixels before attenuation
        attenuation_factor: 0.01,  // Size attenuation factor
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

        // Random particle size in range [0.5, 2.0]
        let size = 0.5 + ((fi * 567.891).fract()) * 1.5;

        // Spawn particle entity with shared mesh/material handles
        // Bevy 0.15 automatically batches entities with same mesh/material for GPU instancing
        commands.spawn((
            Mesh3d(point_mesh.0.clone()),  // Shared point mesh from resource
            MeshMaterial3d(material_handle.clone()),  // Shared point sprite material
            Transform::from_translation(position),  // Per-instance transform
            Particle { position, color, size },
        ));
    }
}

/// System to update particle positions based on physics
///
/// Currently implements basic outward expansion animation where particles
/// move away from the origin at a constant speed. This is a simple demonstration
/// of particle movement capability - full physics sync is a future TODO item.
pub fn update_particles(
    mut query: Query<(Entity, &mut Transform), With<Particle>>,
    time: Res<Time>,
) {
    let speed = 0.5;  // units per second
    let delta = time.delta_secs();
    
    for (entity, mut transform) in query.iter_mut() {
        let pos = transform.translation;
        
        // Calculate direction: normalize position to get outward direction
        let direction = if pos.length_squared() > f32::EPSILON {
            // Particle is away from origin - move outward along position vector
            pos.normalize()
        } else {
            // At origin - use pseudo-random direction based on entity index
            let index = entity.index() as f32;
            let x = ((index * 123.456).fract() - 0.5) * 2.0;
            let y = ((index * 789.012).fract() - 0.5) * 2.0;
            let z = ((index * 345.678).fract() - 0.5) * 2.0;
            Vec3::new(x, y, z).normalize()
        };
        
        // Move particle outward along its direction
        transform.translation += direction * speed * delta;
    }
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
        app.add_plugins(MaterialPlugin::<PointSpriteMaterial>::default())
            .add_systems(Startup, init_point_mesh)
            .add_systems(Startup, spawn_particles)
            .add_systems(Update, update_particles);
    }
}
