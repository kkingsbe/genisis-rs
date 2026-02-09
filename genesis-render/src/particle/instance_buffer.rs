//! Per-instance particle data synchronization for GPU rendering
//!
//! This module implements a Storage Buffer approach for synchronizing per-instance
//! particle attributes (size and color) from CPU Particle components to GPU storage
//! buffers for efficient instanced rendering.
//!
//! The implementation follows Bevy 0.15's ExtractSchedule and Render world phases:
//! 1. Extract system copies Particle data from Main world to Render world
//! 2. Prepare system writes data to GPU storage buffer and creates bind groups
//! 3. Shader accesses per-instance data via instance index
//!
//! # Architecture
//!
//! ```text
//! Particle Components (Main World)
//!     ↓ [extract_particle_instances - ExtractSchedule]
//! ExtractedParticleInstances (Render World)
//!     ↓ [prepare_particle_instance_buffers - RenderSet::Prepare]
//! ParticleInstanceBuffer + ParticleInstanceBindGroup (GPU)
//!     ↓ [Vertex Shader]
//! Per-Instance Rendering
//! ```

use bevy::prelude::*;
use bevy::render::render_resource::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer,
    BufferBindingType, BufferDescriptor, BufferUsages, ShaderStages,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};

use bytemuck::Zeroable;

use super::Particle;

// ============================================================================
// GPU Data Structures
// ============================================================================

/// Per-instance particle data for GPU upload
///
/// This struct must have a stable memory layout for GPU compatibility.
/// Uses #[repr(C, align(16))] for C-compatible layout with 16-byte alignment.
///
/// Fields are ordered for alignment:
/// - size: f32 (4 bytes)
/// - _pad1, _pad2, _pad3: f32 (4 bytes each) - padding for 16-byte alignment
/// - color: [f32; 4] (16 bytes)
///
/// Total: 32 bytes per instance (aligned to 16 bytes)
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleInstanceData {
    /// Particle size in world units
    pub size: f32,
    /// Padding for alignment (16-byte boundary before color)
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
    /// RGBA color of the particle (linear color space)
    pub color: [f32; 4],
}

/// Extracted per-instance particle data for GPU upload
///
/// This resource exists in the Render world and contains particle instance data
/// extracted from the Main world's Particle components. It's consumed by the
/// prepare system to upload data to the GPU.
///
/// Created by: `extract_particle_instances` system in ExtractSchedule
/// Consumed by: `prepare_particle_instance_buffers` system in RenderSet::Prepare
#[derive(Resource, Clone, Debug, Default)]
pub struct ExtractedParticleInstances(pub Vec<ParticleInstanceData>);

/// GPU buffer containing per-instance particle data
///
/// Wraps a Bevy StorageBuffer that holds all particle instance data on the GPU.
/// This buffer is read by the shader via a storage buffer binding.
///
/// The buffer is dynamically resized as needed when particle count changes.
///
/// Created and updated by: `prepare_particle_instance_buffers` system
#[derive(Resource, Clone, Debug)]
pub struct ParticleInstanceBuffer {
    /// The GPU buffer handle
    pub buffer: Buffer,
    /// Maximum number of instances the buffer can hold
    pub capacity: usize,
    /// Current number of instances in the buffer
    pub count: usize,
}

/// Bind group for instance data storage buffer
///
/// This bind group makes the ParticleInstanceBuffer accessible to the shader.
/// It's bound to the render pipeline at binding index 3.
///
/// Created and updated by: `prepare_particle_instance_buffers` system
#[derive(Resource, Clone, Debug)]
pub struct ParticleInstanceBindGroup {
    /// The bind group handle
    pub bind_group: BindGroup,
}

/// Bind group layout for instance data storage buffer
///
/// Defines the interface for binding the storage buffer to shaders.
/// This layout is used when creating bind groups and must match
/// the shader's @group/@binding declarations.
///
/// Created by: `init_particle_instance_bind_group_layout` render startup system
#[derive(Resource, Clone, Debug)]
pub struct ParticleInstanceBindGroupLayout {
    /// The bind group layout handle
    pub bind_group_layout: BindGroupLayout,
}

/// Startup system to initialize the particle instance bind group layout
///
/// This system runs in the render app's startup schedule after RenderDevice is available.
/// It creates the bind group layout that will be used to bind instance data
/// to shaders.
///
/// Runs in: RenderStartup schedule (render app)
/// Resources required: RenderDevice
pub fn init_particle_instance_bind_group_layout(mut commands: Commands, render_device: Res<RenderDevice>) {
    let bind_group_layout = render_device.create_bind_group_layout(
        Some("particle_instance_bind_group_layout"),
        &[
            // Binding 3: Instance data storage buffer
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: Some(std::num::NonZeroU64::new(32).unwrap()),
                },
                count: None,
            },
        ],
    );
    commands.insert_resource(ParticleInstanceBindGroupLayout { bind_group_layout });
}

// ============================================================================
// Extract System
// ============================================================================

/// Extract particle instance data from Main world to Render world
///
/// This system runs in the ExtractSchedule and copies per-instance data
/// from Particle components in the Main world into the Render world.
///
/// Data extracted:
/// - Particle.color converted to linear RGBA ([f32; 4])
/// - Particle.size (f32)
///
/// The extracted data is stored in ExtractedParticleInstances resource in
/// the Render world for subsequent GPU upload.
///
/// # Performance Notes
///
/// - Runs once per frame in ExtractSchedule
/// - O(N) iteration over all particles
/// - Memory allocation: allocates a new Vec<ParticleInstanceData> each frame
pub fn extract_particle_instances(
    mut commands: Commands,
    particles: Query<&Particle>,
) {
    let instance_data: Vec<ParticleInstanceData> = particles
        .iter()
        .map(|particle| {
            let linear_color = particle.color.to_linear();
            ParticleInstanceData {
                size: particle.size,
                _pad1: 0.0,
                _pad2: 0.0,
                _pad3: 0.0,
                color: [
                    linear_color.red,
                    linear_color.green,
                    linear_color.blue,
                    linear_color.alpha,
                ],
            }
        })
        .collect();

    commands.insert_resource(ExtractedParticleInstances(instance_data));
}

// ============================================================================
// Render Prepare System
// ============================================================================

/// Prepare GPU buffers and bind groups for particle instance data
///
/// This system runs in the RenderSet::Prepare phase and:
/// 1. Reads ExtractedParticleInstances from the Render world
/// 2. Resizes the GPU buffer if needed (when capacity exceeded)
/// 3. Writes instance data to the GPU buffer via RenderDevice
/// 4. Creates/updates the bind group for shader access
///
/// # Buffer Resizing Strategy
///
/// - Initial capacity: 1024 instances
/// - Resize trigger: when data.len() > capacity
/// - New capacity: next power of two of data.len()
/// - This amortizes reallocation costs for dynamic particle counts
///
/// # Performance Notes
///
/// - Runs once per frame in RenderSet::Prepare
/// - Buffer resize: O(1) amortized (infrequent)
/// - Buffer write: O(N) where N is particle count
/// - Bind group creation: O(1) (but involves GPU state change)
pub fn prepare_particle_instance_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    extracted: Res<ExtractedParticleInstances>,
    instance_buffer: Option<ResMut<ParticleInstanceBuffer>>,
    bind_group_layout: Res<ParticleInstanceBindGroupLayout>,
) {
    let data = &extracted.0;
    let data_size = std::mem::size_of::<ParticleInstanceData>();

    // Early return if no particles
    if data.is_empty() {
        return;
    }

    // Initialize or get existing buffer
    let mut instance_buffer = match instance_buffer {
        Some(buf) => buf,
        None => {
            // Create new buffer with initial capacity of at least 1024
            let capacity = data.len().max(1024).next_power_of_two();
            let buffer = render_device.create_buffer(&BufferDescriptor {
                label: Some("particle_instance_buffer"),
                size: (capacity * data_size) as u64,
                usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
                mapped_at_creation: false,
            });

            let buffer_resource = ParticleInstanceBuffer {
                buffer,
                capacity,
                count: 0,
            };

            commands.insert_resource(buffer_resource);
            return;
        }
    };

    // Resize buffer if needed
    if data.len() > instance_buffer.capacity {
        let new_capacity = data.len().next_power_of_two();
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("particle_instance_buffer"),
            size: (new_capacity * data_size) as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        instance_buffer.buffer = buffer;
        instance_buffer.capacity = new_capacity;
    }

    // Write instance data to GPU buffer
    render_queue.write_buffer(
        &instance_buffer.buffer,
        0,
        bytemuck::cast_slice(data.as_slice()),
    );

    instance_buffer.count = data.len();

    // Create bind group for shader access
    let bind_group = render_device.create_bind_group(
        Some("particle_instance_bind_group"),
        &bind_group_layout.bind_group_layout,
        &[BindGroupEntry {
            binding: 3,
            resource: instance_buffer.buffer.as_entire_binding(),
        }],
    );

    commands.insert_resource(ParticleInstanceBindGroup { bind_group });
}

// ============================================================================
// Tests (Subtask 4 will implement actual tests)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_instance_data_size() {
        // Verify memory layout: 32 bytes total
        assert_eq!(std::mem::size_of::<ParticleInstanceData>(), 32);
    }

    #[test]
    fn test_particle_instance_data_alignment() {
        // Verify alignment to 16 bytes
        assert_eq!(std::mem::align_of::<ParticleInstanceData>(), 16);
    }

    #[test]
    fn test_particle_instance_data_zeroable() {
        // Verify zeroable trait
        let zero = ParticleInstanceData::zeroed();
        assert_eq!(zero.color, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(zero.size, 0.0);
    }
}
