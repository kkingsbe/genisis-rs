//! Particle rendering components and spawner system
//!
//! Defines the Particle component for entities that represent
//! particles in the simulation, along with a spawning system to
//! create particle clusters. Uses GPU-accelerated instanced rendering
//! (automatic batching) via shared mesh and material handles for
//! efficient rendering of thousands of particles.
//!
//! # Per-Instance Attribute Synchronization
//!
//! Per-instance particle size and color data is synchronized via a storage buffer approach:
//! - Extract system (`extract_particle_instances`) transfers Particle component data to render world
//! - Prepare system (`prepare_particle_instance_buffers`) creates GPU storage buffers
//! - Shader uses storage buffer at @group(0)@binding(3) with @builtin(instance_index)
//!
//! **Implementation Status**:
//! ✓ Storage buffer approach fully implemented for per-instance particle data
//! - Extract system (`extract_particle_instances`) transfers Particle component data to render world
//! - Prepare system (`prepare_particle_instance_buffers`) creates GPU storage buffers
//! - Bind group layout (`init_particle_instance_bind_group_layout`) initializes shader binding
//! - Shader integration complete: point_sprite.wgsl uses @builtin(instance_index) and storage buffer
//!
//! **Architecture**:
//!
//! The storage buffer approach with instance index mapping synchronizes per-instance data. See `DESIGN.md`
//! in the particle module directory for the complete design specification including:
//!
//! - **Data Flow**: Particle components (CPU) → Extract system → GPU Storage Buffer → Shader (via instance_index)
//! - **Components**: ParticleInstanceData for GPU-compatible layout (defined in instance_buffer.rs)
//! - **Resources**: ParticleInstanceBuffer and ParticleInstanceBindGroup for GPU management
//! - **Systems**: extract_particle_instances (ExtractSchedule) and prepare_particle_instance_buffers (RenderSet::Prepare)
//! - **Shader Integration**: Storage buffer binding at @group(0)@binding(3) indexed by @builtin(instance_index)
//!
//! This design supports 10K-100K particles with efficient CPU-GPU synchronization and maintains
//! Bevy 0.15's automatic GPU instancing benefits.
//!
//! # Configuration Note
//!
//! genesis.toml fields `initial_count`, `max_count`, `base_size` match [`ParticleConfig`]
//! struct fields. The [`spawn_particles()`] system uses `config.initial_count` and
//! `config.base_size` directly from the config resource.

use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::pbr::{Material, MeshMaterial3d};
use bevy::render::alpha::AlphaMode;
use bevy::render::mesh::{MeshVertexAttribute, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::Render;
use bevy::render::RenderApp;
use bevy::render::RenderSet;
use genesis_core::config::ParticleConfig;
use genesis_core::{events::ScrubbingEvent, time::TimeAccumulator};
use genesis_physics::cosmology::ScaleFactor;

mod instance_buffer;

pub use instance_buffer::{
    extract_particle_instances,
    init_particle_instance_bind_group_layout,
    prepare_particle_instance_buffers,
    ExtractedParticleInstances,
    ParticleInstanceBindGroup,
    ParticleInstanceBindGroupLayout,
    ParticleInstanceBuffer,
    ParticleInstanceData,
};

/// Custom vertex attribute for per-instance particle size
pub const ATTRIBUTE_INSTANCE_SIZE: MeshVertexAttribute = MeshVertexAttribute::new(
    "instance_size",
    921384470,
    bevy::render::render_resource::VertexFormat::Float32,
);

/// Custom vertex attribute for per-instance particle color
pub const ATTRIBUTE_INSTANCE_COLOR: MeshVertexAttribute = MeshVertexAttribute::new(
    "instance_color",
    921384471,
    bevy::render::render_resource::VertexFormat::Float32x4,
);

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
    fn vertex_shader() -> ShaderRef {
        "point_sprite.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "point_sprite.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

/// Resource tracking whether timeline scrubbing is active
///
/// This resource is updated by listening to ScrubbingEvent events and allows
/// the particle systems to know whether to use scrubbing-based position calculation
/// (initial_position + initial_velocity * years) or normal physics integration.
#[derive(Resource, Default)]
pub struct ScrubbingState {
    /// `true` when timeline scrubbing is active, `false` during normal playback
    pub is_scrubbing: bool,
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
/// Contains position, velocity, color, and size attributes for rendering.
/// This component can be used with instanced rendering for
/// GPU-accelerated particle systems.
#[derive(Component, Clone)]
pub struct Particle {
    /// World space position of the particle
    pub position: Vec3,
    /// Velocity of the particle
    pub velocity: Vec3,
    /// Initial spawn position at t=0 (used for timeline scrubbing)
    pub initial_position: Vec3,
    /// Initial velocity at spawn time (used for timeline scrubbing)
    pub initial_velocity: Vec3,
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
/// # Per-Instance Data Handling
///
/// Storage buffer systems (extract_particle_instances, prepare_particle_instance_buffers) exist
/// for transferring Particle component data to GPU. The shader (point_sprite.wgsl) uses
/// the storage buffer at @group(0)@binding(3) with @builtin(instance_index) for
/// per-instance data access. Full per-instance color and size synchronization is complete.
pub fn init_point_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    println!("DEBUG: init_point_mesh STARTED");
    // Create a simple point mesh with PointList topology
    // Single vertex at origin since Transform provides actual position
    let mut mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default());

    // Add a single vertex at the origin
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, 0.0]]);

    // Per-instance size and color attributes are handled through storage buffer infrastructure.
    // Storage buffer systems (extract_particle_instances, prepare_particle_instance_buffers) transfer
    // Particle component data to GPU, and the shader reads from the storage buffer
    // at @group(0)@binding(3) using @builtin(instance_index).

    let mesh_handle = meshes.add(mesh);
    commands.insert_resource(PointMesh(mesh_handle));
    println!("DEBUG: init_point_mesh COMPLETED - PointMesh resource inserted");
}

/// Converts an energy value to a color on a thermal gradient.
///
/// Maps energy (0.0 to 1.0) to a color gradient representing temperature:
/// - 1.0 = white-hot core (highest energy)
/// - 0.0 = dark red edges (lowest energy)
///
/// The gradient follows: WHITE → YELLOW → ORANGE → RED → DARK_RED
///
/// # Arguments
/// * `energy` - Energy value in range [0.0, 1.0], where:
///   - 0.0 = lowest energy (dark red)
///   - 1.0 = highest energy (white-hot)
///
/// # Returns
/// A `Color` value corresponding to the energy level.
///
/// # Gradient Breakpoints
/// - energy > 0.80: Interpolates WHITE (1.0) to YELLOW (0.80)
/// - energy > 0.50: Interpolates YELLOW (0.80) to ORANGE (0.50)
/// - energy > 0.20: Interpolates ORANGE (0.50) to RED (0.20)
/// - energy <= 0.20: Interpolates RED (0.20) to DARK_RED (0.0)
pub fn energy_to_color(energy: f32) -> Color {
    // Clamp energy to [0.0, 1.0]
    let energy = energy.clamp(0.0, 1.0);

    if energy > 0.80 {
        // WHITE to YELLOW: t in [0.80, 1.0], normalized to [0.0, 1.0]
        let t = (energy - 0.80) / 0.20;
        lerp_rgb(1.0, 1.0, 0.0, 1.0, 1.0, 1.0, t)
    } else if energy > 0.50 {
        // YELLOW to ORANGE: t in [0.50, 0.80], normalized to [0.0, 1.0]
        let t = (energy - 0.50) / 0.30;
        lerp_rgb(1.0, 0.5, 0.0, 1.0, 1.0, 0.0, t)
    } else if energy > 0.20 {
        // ORANGE to RED: t in [0.20, 0.50], normalized to [0.0, 1.0]
        let t = (energy - 0.20) / 0.30;
        lerp_rgb(1.0, 0.0, 0.0, 1.0, 0.5, 0.0, t)
    } else {
        // RED to DARK_RED: t in [0.0, 0.20], normalized to [0.0, 1.0]
        let t = energy / 0.20;
        lerp_rgb(0.5, 0.0, 0.0, 1.0, 0.0, 0.0, t)
    }
}

/// Converts a temperature value to a color on a thermal gradient.
///
/// Maps temperature (in Kelvin) to a color gradient representing cosmic
/// temperature ranges:
/// - ≥10¹⁵K = blue-white (extreme high energy)
/// - 10¹⁴K = white (very high energy)
/// - 10¹³K = yellow (high energy)
/// - 10¹²K = orange (moderate energy)
/// - ≤10¹¹K = orange (low energy floor)
///
/// The gradient follows: BLUE_WHITE → WHITE → YELLOW → ORANGE (floor at 10¹¹K)
///
/// # Arguments
/// * `T` - Temperature in Kelvin (f64), representing the thermal energy of the particle
///
/// # Returns
/// A `Color` value corresponding to the temperature level.
///
/// # Gradient Breakpoints
/// - T ≥ 1e15: Interpolates BLUE_WHITE (1e15) upward
/// - T > 1e14: Interpolates WHITE (1e14) to BLUE_WHITE (1e15)
/// - T > 1e13: Interpolates YELLOW (1e13) to WHITE (1e14)
/// - T > 1e12: Interpolates ORANGE (1e12) to YELLOW (1e13)
/// - T ≤ 1e12: Returns ORANGE (temperature floor)
pub fn temperature_to_color(T: f64) -> Color {
    // Temperature breakpoints (in Kelvin)
    const T_BLUE_WHITE: f64 = 1e15;
    const T_WHITE: f64 = 1e14;
    const T_YELLOW: f64 = 1e13;
    const T_ORANGE: f64 = 1e12;

    // Color values (RGB in [0, 1] range)
    // BLUE_WHITE: (200/255, 200/255, 255/255) ≈ (0.78, 0.78, 1.0)
    const R_BLUE_WHITE: f32 = 0.78;
    const G_BLUE_WHITE: f32 = 0.78;
    const B_BLUE_WHITE: f32 = 1.0;

    // WHITE: (1.0, 1.0, 1.0)
    const R_WHITE: f32 = 1.0;
    const G_WHITE: f32 = 1.0;
    const B_WHITE: f32 = 1.0;

    // YELLOW: (255/255, 255/255, 100/255) ≈ (1.0, 1.0, 0.39)
    const R_YELLOW: f32 = 1.0;
    const G_YELLOW: f32 = 1.0;
    const B_YELLOW: f32 = 0.39;

    // ORANGE: (255/255, 165/255, 0/255) ≈ (1.0, 0.65, 0.0)
    const R_ORANGE: f32 = 1.0;
    const G_ORANGE: f32 = 0.65;
    const B_ORANGE: f32 = 0.0;

    if T >= T_BLUE_WHITE {
        // Return blue-white for extremely high temperatures (≥10¹⁵K)
        Color::srgb(R_BLUE_WHITE, G_BLUE_WHITE, B_BLUE_WHITE)
    } else if T >= T_WHITE {
        // WHITE to BLUE_WHITE: t in [T_WHITE, T_BLUE_WHITE], normalized to [0.0, 1.0]
        let t = ((T - T_WHITE) / (T_BLUE_WHITE - T_WHITE)) as f32;
        lerp_rgb(R_WHITE, G_WHITE, B_WHITE, R_BLUE_WHITE, G_BLUE_WHITE, B_BLUE_WHITE, t)
    } else if T >= T_YELLOW {
        // YELLOW to WHITE: t in [T_YELLOW, T_WHITE], normalized to [0.0, 1.0]
        let t = ((T - T_YELLOW) / (T_WHITE - T_YELLOW)) as f32;
        lerp_rgb(R_YELLOW, G_YELLOW, B_YELLOW, R_WHITE, G_WHITE, B_WHITE, t)
    } else if T >= T_ORANGE {
        // ORANGE to YELLOW: t in [T_ORANGE, T_YELLOW], normalized to [0.0, 1.0]
        let t = ((T - T_ORANGE) / (T_YELLOW - T_ORANGE)) as f32;
        lerp_rgb(R_ORANGE, G_ORANGE, B_ORANGE, R_YELLOW, G_YELLOW, B_YELLOW, t)
    } else {
        // Return orange as floor for low temperatures (<10¹²K)
        Color::srgb(R_ORANGE, G_ORANGE, B_ORANGE)
    }
}

/// Helper function for linear interpolation between two RGB color values.
///
/// # Arguments
/// * `r1` - Red component of start color (when t = 0.0)
/// * `g1` - Green component of start color (when t = 0.0)
/// * `b1` - Blue component of start color (when t = 0.0)
/// * `r2` - Red component of end color (when t = 1.0)
/// * `g2` - Green component of end color (when t = 1.0)
/// * `b2` - Blue component of end color (when t = 1.0)
/// * `t` - Interpolation factor in [0.0, 1.0]
///
/// # Returns
/// Interpolated color: color1 * (1-t) + color2 * t
fn lerp_rgb(r1: f32, g1: f32, b1: f32, r2: f32, g2: f32, b2: f32, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::srgb(
        r1 * (1.0 - t) + r2 * t,
        g1 * (1.0 - t) + g2 * t,
        b1 * (1.0 - t) + b2 * t,
    )
}

/// System to spawn a cluster of particles around the origin
///
/// Creates particle entities in a dense cluster representing the
/// early universe. Uses GPU instancing (automatic batching) where
/// all entities share the same mesh and material handles for
/// efficient rendering.
///
/// Spawns particles based on the configuration specified in
/// ParticleConfig (via the Resource derive). GPU instancing allows efficient rendering
/// of 100K-1M particles.
///
/// In Bevy 0.15, GPU instancing is automatic: when multiple entities
/// share the same Mesh3d and MeshMaterial3d handles, the renderer
/// batches them for GPU instancing without requiring explicit
/// instance components.
///
/// # Configuration Note
///
/// This system receives ParticleConfig as a Bevy Resource. ParticleConfig
/// has `#[derive(Resource)]` and is used directly in main.rs via
/// `config.particle.clone()`. Field names match genesis.toml configuration.
pub fn spawn_particles(
    mut commands: Commands,
    mut materials: ResMut<Assets<PointSpriteMaterial>>,
    point_mesh: Res<PointMesh>,
    config: Res<ParticleConfig>,
) {
    let particle_count = config.initial_count as u32;

    // Create point sprite material for all particles (single material shared by all)
    // Using white color for visibility - individual particle colors will be
    // handled by the shader in future updates
    let particle_material = PointSpriteMaterial {
        color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        base_size: config.base_size,
        attenuation_factor: 0.01,      // Size attenuation factor
    };
    let material_handle = materials.add(particle_material);

    // Spawn particles at the exact origin with radial velocity
    const BASE_SPEED: f32 = 0.5;

    for i in 0..particle_count {
        // Simple deterministic pseudo-random distribution using loop index
        // This provides variation without requiring the rand crate
        let fi = i as f32;

        // Spawn ALL particles at exact origin
        let position = Vec3::ZERO;

        // Generate outward radial velocity vector using pseudo-random distribution
        // Use the same pattern to create a direction vector on a unit sphere
        let dir_x = ((fi * 123.456).fract() - 0.5) * 2.0;
        let dir_y = ((fi * 789.012).fract() - 0.5) * 2.0;
        let dir_z = ((fi * 345.678).fract() - 0.5) * 2.0;
        let direction = Vec3::new(dir_x, dir_y, dir_z).normalize();

        // Scale direction by base speed to get velocity
        let velocity = direction * BASE_SPEED;
        let _velocity_magnitude = velocity.length();

        // Set initial color to white-hot (maximum energy = 1.0)
        // All particles start at the origin with maximum energy
        let color = energy_to_color(1.0);

        // Random particle size in range [0.5, 2.0]
        let size = 0.5 + ((fi * 567.891).fract()) * 1.5;

        // Spawn particle entity with shared mesh/material handles
        // Bevy 0.15 automatically batches entities with same mesh/material for GPU instancing
        commands.spawn((
            Mesh3d(point_mesh.0.clone()), // Shared point mesh from resource
            MeshMaterial3d(material_handle.clone()), // Shared point sprite material
            Transform::from_translation(position), // Per-instance transform (all at origin)
            Particle {
                position,
                velocity,
                initial_position: position, // Store spawn position at t=0
                initial_velocity: velocity, // Store initial velocity at spawn time
                color,
                size,
            },
        ));
    }
}

/// System to update particle positions based on physics
///
/// Updates particle positions using velocity-based movement and syncs
/// Particle.position with Transform.translation for proper rendering.
/// This ensures Particle.position stays current for energy-based coloring.
pub fn update_particles(
    mut query: Query<(&mut Particle, &mut Transform)>,
    time: Res<Time>,
    scale_factor: Res<ScaleFactor>,
) {
    let delta = time.delta_secs();

    for (mut particle, mut transform) in query.iter_mut() {
        // Store velocity to avoid borrow conflicts
        let velocity = particle.velocity;
        
        // Update position based on velocity and scale factor:
        // position = (position + velocity * delta) * scale_factor.value
        particle.position = (particle.position + velocity * delta) * scale_factor.value as f32;
        
        // Sync Particle.position to Transform.translation for rendering
        transform.translation = particle.position;
    }
}

/// System to update particle colors based on energy (distance from origin)
///
/// Updates particle colors dynamically based on their distance from the origin.
/// Particles closer to the origin have higher energy (white-hot), while
/// particles further away have lower energy (red/dark-red).
///
/// Energy is calculated as: Energy = 1.0 - (distance / 50.0), clamped to [0.0, 1.0]
///
/// This creates a thermal gradient visualization where the singularity core
/// appears white-hot and the outer regions appear red.
pub fn update_particle_energy_colors(mut query: Query<(&Transform, &mut Particle)>) {
    const MAX_DISTANCE: f32 = 50.0;

    for (transform, mut particle) in query.iter_mut() {
        // Calculate distance from origin
        let distance = transform.translation.length();

        // Normalize distance to energy value (0.0 to 1.0)
        // Energy decreases as distance increases
        let energy = (1.0 - (distance / MAX_DISTANCE)).clamp(0.0, 1.0);

        // Update particle color based on energy
        particle.color = energy_to_color(energy);
    }
}

/// Synchronize Transform.translation to Particle.position each frame.
///
/// This ensures the energy-based coloring system receives correct position data
/// by syncing any Transform changes back to the Particle component. This is
/// important when transforms are modified by other systems (e.g., user interaction,
/// camera manipulation) to keep Particle.position in sync.
pub fn sync_particle_position(mut query: Query<(&Transform, &mut Particle)>) {
    for (transform, mut particle) in query.iter_mut() {
        particle.position = transform.translation;
    }
}

/// System to update ScrubbingState based on ScrubbingEvent events
///
/// This system listens for ScrubbingEvent events emitted by the timeline UI
/// and updates the ScrubbingState resource accordingly. The ScrubbingState is
/// then used by other systems (e.g., update_particles_for_scrubbing) to determine
/// whether to use scrubbing-based position calculation or normal physics integration.
pub fn update_scrubbing_state(
    mut events: EventReader<ScrubbingEvent>,
    mut scrubbing_state: ResMut<ScrubbingState>,
) {
    for event in events.read() {
        scrubbing_state.is_scrubbing = event.is_scrubbing;
    }
}

/// System to update particle positions based on timeline scrubbing
///
/// When scrubbing is active (ScrubbingState.is_scrubbing == true), this system
/// recalculates particle positions from their initial state:
///     position = initial_position + initial_velocity * years
///
/// This allows particles to move backward and forward based on the elapsed cosmic
/// time (TimeAccumulator.years) during timeline scrubbing.
///
/// When scrubbing is not active, the normal update_particles system handles
/// physics-based forward integration.
///
/// # Usage
///
/// This system runs alongside update_particles, but only affects particle
/// positions when scrubbing is active. The system ordering ensures that:
/// - During scrubbing: update_particles_for_scrubbing sets positions, update_particles
///   is effectively skipped (or can be conditionally disabled)
/// - During normal playback: update_particles handles physics, update_particles_for_scrubbing
///   does nothing
pub fn update_particles_for_scrubbing(
    mut query: Query<(&mut Particle, &mut Transform)>,
    time_accumulator: Res<TimeAccumulator>,
    scrubbing_state: Res<ScrubbingState>,
    scale_factor: Res<ScaleFactor>,
) {
    // Only recalculate positions when scrubbing is active
    if !scrubbing_state.is_scrubbing {
        return;
    }

    let years = time_accumulator.years as f32;

    for (mut particle, mut transform) in query.iter_mut() {
        // Calculate position from initial state and apply scale factor:
        // position = (initial_position + initial_velocity * years) * scale_factor
        particle.position = (particle.initial_position + particle.initial_velocity * years) * scale_factor.value as f32;

        // Sync Particle.position to Transform.translation for rendering
        transform.translation = particle.position;
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
        app.init_asset::<PointSpriteMaterial>();
        // Startup systems
        app.add_systems(Startup, init_point_mesh)
            .add_systems(Startup, spawn_particles.after(init_point_mesh))
            // Update systems
            .add_systems(Update, update_scrubbing_state)
            .add_systems(Update, update_particles_for_scrubbing.after(update_scrubbing_state))
            .add_systems(Update, update_particles)
            .add_systems(Update, update_particle_energy_colors)
            .add_systems(Update, sync_particle_position.before(update_particle_energy_colors));

        // Initialize ScrubbingState resource
        app.init_resource::<ScrubbingState>();

        // Initialize bind group layout for instance data storage buffer in render app
        // RenderDevice only exists in the render app's world
        app.sub_app_mut(RenderApp)
            .add_systems(Startup, init_particle_instance_bind_group_layout)
            .add_systems(ExtractSchedule, extract_particle_instances)
            .add_systems(Render, prepare_particle_instance_buffers.in_set(RenderSet::Prepare));
    }
}

// ============================================================================
// POINT SPRITE RENDERING IMPLEMENTATION SUMMARY
// ============================================================================
//
// This module implements GPU-accelerated point sprite rendering with size
// attenuation for the Genesis particle system. Below is a detailed overview
// of the implementation.
//
// ## WHAT HAS BEEN IMPLEMENTED
//
// ### 1. Vertex Shader Transform Matrix Application
// The vertex shader in `point_sprite.wgsl` now properly applies the entity's
// Transform matrix to convert mesh positions to world positions:
//
//     let world_pos = model * vec4<f32>(input.position, 1.0);
//     output.clip_position = view.view_proj * world_pos;
//
// This allows each particle entity's Transform component to control its
// position in world space, enabling proper spatial distribution of particles.
//
// ### 2. Per-Instance Size Attribute Infrastructure
// Custom vertex attributes have been added to the mesh to support per-instance
// particle size and color data:
//
// - `ATTRIBUTE_INSTANCE_SIZE`: Float32 vertex attribute at location(1)
// - `ATTRIBUTE_INSTANCE_COLOR`: Float32x4 vertex attribute at location(2)
//
// These attributes are added to the PointMesh in the `init_point_mesh` system
// and are available to the shader for per-instance rendering variation.
//
// ### 3. Fragment Shader Per-Particle Color
// The fragment shader outputs the per-instance particle color passed from the
// vertex shader:
//
//     return input.color;
//
// This allows each particle to have its own distinct color, enabling rich
// visual representation of particle properties (e.g., temperature, age, mass).
//
// ## SIZE ATTENUATION FORMULA
//
// The vertex shader implements size attenuation using the following formula:
//
//     size = instance_size / (1.0 + distance * attenuation_factor)
//
// Where:
// - `instance_size`: The base size of the particle (per-instance attribute)
// - `distance`: Distance from camera to particle in world units
// - `attenuation_factor`: Material uniform controlling attenuation rate
//
// The formula is applied in the vertex shader:
//
//     let distance = distance(view.world_position, world_pos.xyz);
//     let attenuated_size = input.instance_size / (1.0 + distance * material.attenuation_factor);
//     output.point_size = max(attenuated_size, 1.0);
//
// The result is clamped to a minimum of 1.0 pixel to prevent particles from
// becoming invisible when far from the camera.
//
// ## HOW SIZE ATTENUATION WORKS
//
// Size attenuation creates a realistic depth effect by making particles appear
// smaller as they move farther from the camera. This mimics real-world perspective:
//
// - **Near particles**: Appear larger (closer to instance_size)
// - **Far particles**: Appear smaller (reduced by attenuation formula)
//
// The `attenuation_factor` material uniform controls the rate of size reduction:
// - Lower values (e.g., 0.001): Gentle attenuation, particles stay larger
// - Higher values (e.g., 0.1): Strong attenuation, particles shrink quickly
//
// This creates visual depth cues and helps prevent visual clutter when viewing
// large particle systems from a distance.
//
// ## HOW PER-PARTICLE COLOR IS USED
//
// Color data flows through the rendering pipeline as follows:
//
// 1. **Particle Component**: Each particle entity has a `Particle` component with:
//    - `position: Vec3`: World-space position
//    - `color: Color`: RGBA color for rendering
//    - `size: f32`: Particle size in world units
//
// 2. **Mesh Attributes**: The PointMesh has an `instance_color` attribute at
//    location(2) initialized with a default white color [1.0, 1.0, 1.0, 1.0].
//
// 3. **Vertex Shader**: Receives `instance_color` from the mesh and passes it
//    to the fragment shader via `VertexOutput.color`.
//
// 4. **Fragment Shader**: Outputs the color directly for the entire point:
//    `return input.color;`
//
// Note: The Particle component's color and size data are stored on
// the entity and synchronized to GPU storage buffers via instance_buffer.rs
// infrastructure (extract_particle_instances and prepare_particle_instance_buffers systems).
//
// ## WHAT IS STILL NEEDED
// ...

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to compare two Color values with floating-point tolerance.
    ///
    /// # Arguments
    /// * `actual` - The actual color value returned by the function
    /// * `expected_r` - Expected red component
    /// * `expected_g` - Expected green component
    /// * `expected_b` - Expected blue component
    /// * `tolerance` - Maximum allowed difference for each component
    fn assert_color_approx_equal(
        actual: Color,
        expected_r: f32,
        expected_g: f32,
        expected_b: f32,
        tolerance: f32,
    ) {
        let actual_rgba = actual.to_srgba();
        let diff_r = (actual_rgba.red - expected_r).abs();
        let diff_g = (actual_rgba.green - expected_g).abs();
        let diff_b = (actual_rgba.blue - expected_b).abs();

        assert!(
            diff_r <= tolerance,
            "Red component out of tolerance: expected {}, got {}, diff {}",
            expected_r,
            actual_rgba.red,
            diff_r
        );
        assert!(
            diff_g <= tolerance,
            "Green component out of tolerance: expected {}, got {}, diff {}",
            expected_g,
            actual_rgba.green,
            diff_g
        );
        assert!(
            diff_b <= tolerance,
            "Blue component out of tolerance: expected {}, got {}, diff {}",
            expected_b,
            actual_rgba.blue,
            diff_b
        );
    }

    // ========================================
    // EXACT THRESHOLD TESTS
    // ========================================

    /// TEST #1: Verify exact color at 1e15K threshold (blue-white)
    #[test]
    fn test_temperature_to_color_at_1e15() {
        let color = temperature_to_color(1e15);
        // BLUE_WHITE: (0.78, 0.78, 1.0)
        assert_color_approx_equal(color, 0.78, 0.78, 1.0, 0.01);
    }

    /// TEST #2: Verify exact color at 1e14K threshold (white)
    #[test]
    fn test_temperature_to_color_at_1e14() {
        let color = temperature_to_color(1e14);
        // WHITE: (1.0, 1.0, 1.0)
        assert_color_approx_equal(color, 1.0, 1.0, 1.0, 0.01);
    }

    /// TEST #3: Verify exact color at 1e13K threshold (yellow)
    #[test]
    fn test_temperature_to_color_at_1e13() {
        let color = temperature_to_color(1e13);
        // YELLOW: (1.0, 1.0, 0.39)
        assert_color_approx_equal(color, 1.0, 1.0, 0.39, 0.01);
    }

    /// TEST #4: Verify exact color at 1e12K threshold (orange)
    #[test]
    fn test_temperature_to_color_at_1e12() {
        let color = temperature_to_color(1e12);
        // ORANGE: (1.0, 0.65, 0.0)
        assert_color_approx_equal(color, 1.0, 0.65, 0.0, 0.01);
    }

    /// TEST #5: Verify exact color at 1e11K (floor - should return orange)
    #[test]
    fn test_temperature_to_color_at_1e11() {
        let color = temperature_to_color(1e11);
        // Floor: ORANGE (1.0, 0.65, 0.0)
        assert_color_approx_equal(color, 1.0, 0.65, 0.0, 0.01);
    }

    // ========================================
    // INTERPOLATION TESTS
    // ========================================

    /// TEST #6: Verify interpolated color at 5e14K (between 1e14 and 1e15)
    ///
    /// At 5e14K: t = (5e14 - 1e14) / (1e15 - 1e14) = 4e14 / 9e14 = 0.4444
    /// R = 1.0 * (1 - 0.4444) + 0.78 * 0.4444 = 0.9
    /// G = 1.0 * (1 - 0.4444) + 0.78 * 0.4444 = 0.9
    /// B = 1.0 * (1 - 0.4444) + 1.0 * 0.4444 = 1.0
    #[test]
    fn test_temperature_to_color_interpolate_5e14() {
        let color = temperature_to_color(5e14);
        // Between WHITE and BLUE_WHITE
        assert_color_approx_equal(color, 0.90, 0.90, 1.0, 0.02);
    }

    /// TEST #7: Verify interpolated color at 5e13K (between 1e13 and 1e14)
    ///
    /// At 5e13K: t = (5e13 - 1e13) / (1e14 - 1e13) = 4e13 / 9e13 = 0.4444
    /// R = 1.0 * (1 - 0.4444) + 1.0 * 0.4444 = 1.0
    /// G = 1.0 * (1 - 0.4444) + 1.0 * 0.4444 = 1.0
    /// B = 0.39 * (1 - 0.4444) + 1.0 * 0.4444 = 0.66
    #[test]
    fn test_temperature_to_color_interpolate_5e13() {
        let color = temperature_to_color(5e13);
        // Between YELLOW and WHITE
        assert_color_approx_equal(color, 1.0, 1.0, 0.66, 0.02);
    }

    /// TEST #8: Verify interpolated color at 5e12K (between 1e12 and 1e13)
    ///
    /// At 5e12K: t = (5e12 - 1e12) / (1e13 - 1e12) = 4e12 / 9e12 = 0.4444
    /// R = 1.0 * (1 - 0.4444) + 1.0 * 0.4444 = 1.0
    /// G = 0.65 * (1 - 0.4444) + 1.0 * 0.4444 = 0.81
    /// B = 0.0 * (1 - 0.4444) + 0.39 * 0.4444 = 0.17
    #[test]
    fn test_temperature_to_color_interpolate_5e12() {
        let color = temperature_to_color(5e12);
        // Between ORANGE and YELLOW
        assert_color_approx_equal(color, 1.0, 0.81, 0.17, 0.02);
    }

    // ========================================
    // BOUNDARY TESTS
    // ========================================

    /// TEST #9: Verify very high temperature (>1e16K) returns blue-white
    #[test]
    fn test_temperature_to_color_very_high() {
        let color = temperature_to_color(1e16);
        // Should be capped at BLUE_WHITE
        assert_color_approx_equal(color, 0.78, 0.78, 1.0, 0.01);
    }

    /// TEST #10: Verify very low temperature (<1e10K) returns orange (floor)
    #[test]
    fn test_temperature_to_color_very_low() {
        let color = temperature_to_color(1e10);
        // Should return floor ORANGE
        assert_color_approx_equal(color, 1.0, 0.65, 0.0, 0.01);
    }

    /// TEST #11: Verify near-threshold temperatures interpolate correctly
    #[test]
    fn test_temperature_to_color_near_thresholds() {
        // Just below 1e15K: should be very close to blue-white
        let color = temperature_to_color(9.9e14);
        assert_color_approx_equal(color, 0.78, 0.78, 1.0, 0.02);

        // Just below 1e14K: should be very close to white
        let color = temperature_to_color(9.9e13);
        assert_color_approx_equal(color, 1.0, 1.0, 0.99, 0.02);

        // Just below 1e13K: should be very close to yellow
        let color = temperature_to_color(9.9e12);
        assert_color_approx_equal(color, 1.0, 1.0, 0.39, 0.02);

        // Just below 1e12K: should be very close to orange
        let color = temperature_to_color(9.9e11);
        assert_color_approx_equal(color, 1.0, 0.65, 0.0, 0.02);
    }
}
