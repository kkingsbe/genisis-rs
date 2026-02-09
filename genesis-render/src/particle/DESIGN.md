# Per-Instance Particle Attributes - Design Document

## Context and Problem Statement

### Current Situation
The Genesis particle system uses GPU-accelerated instanced rendering in Bevy 0.15. All particle entities share a single `PointMesh` and `PointSpriteMaterial`, and Bevy 0.15 automatically batches these entities for GPU instancing. However, there's a critical gap:

**The Problem:**
- Each particle has a `Particle` component with `position`, `color`, and `size` fields
- The shader expects per-instance `instance_size` (location 1) and `instance_color` (location 2)
- The current `sync_particle_instance_attributes()` function attempts to update mesh attributes, but since the mesh is shared across ALL instances, this affects them equally
- Result: All particles render with the same size and color, ignoring individual particle data

### Requirements
- Support 10K-100K particles efficiently
- Synchronize per-particle `color` and `size` from `Particle` component to GPU
- Maintain GPU instancing benefits
- Update data every frame as particle properties change
- Preserve existing shader and material infrastructure

---

## Bevy 0.15 Instancing Architecture Research

### Key Bevy 0.15 Rendering Concepts

#### 1. Mesh Attributes vs. Instance Data
Bevy 0.15 separates two types of vertex data:
- **Vertex Attributes**: Per-vertex data (e.g., position, normal, UV) - shared across all instances
- **Instance Data**: Per-instance data - can vary per mesh instance

#### 2. Available Bevy 0.15 APIs

##### bevy::render::mesh::Mesh
- `Mesh::insert_attribute()`: Adds vertex attributes to the mesh
- `MeshVertexAttribute`: Custom vertex attribute definition
- `RenderAssetUsages`: Controls CPU/GPU asset usage
- **Limitation**: Vertex attributes are shared across all instances of the same mesh

##### bevy::render::render_resource
- **`Buffer`**: Generic GPU buffer type
- **`BufferBindingType`**: Uniform, Storage, etc.
- **`BufferUsages`**: CopyDst, CopySrc, Storage, Vertex, etc.
- **`DynamicUniformBuffer`**: For frequently updated uniform data (limited by UBO size, typically 64KB)
- **`StorageBuffer`**: For larger datasets (up to GPU limits, typically 128MB+)

##### bevy::render::view::ViewTarget
- View-projection matrix access for shader uniforms

##### bevy::render::renderer::RenderDevice
- Low-level GPU buffer creation and management
- `create_buffer()`: Create GPU buffers
- `write_buffer()`: Update buffer contents

##### bevy::render::render_asset::RenderAssets
- Access to GPU-side asset handles (RenderMesh, RenderMaterial)

##### bevy::render::ExtractSchedule
- Phase 1: Extract data from main world to render world
- Systems can prepare data for GPU upload

##### bevy::render::Render
- Phase 2: Render world systems
- GPU buffer updates happen here

### Critical Finding: Bevy 0.15 Instancing Limitations

Bevy 0.15's automatic instancing works by:
1. Batching entities with the same `Mesh3d` and `MeshMaterial3d` handles
2. Using `Transform` component for per-instance positions
3. **No built-in support for custom per-instance attributes beyond Transform**

This means:
- Custom vertex attributes (like `ATTRIBUTE_INSTANCE_SIZE`) cannot be per-instance with automatic batching
- We need to provide per-instance data through alternative mechanisms

---

## Implementation Approaches

### Approach 1: Storage Buffer with Instance Index (RECOMMENDED)

#### Overview
Use a GPU Storage Buffer containing per-instance data (size and color), indexed by instance ID. Modify the shader to read instance data from the storage buffer using `gl_InstanceIndex` (or similar).

#### Architecture

##### Data Flow
```
Particle Components (CPU)
    ↓ [Extract System]
InstanceData Buffer (GPU Storage Buffer)
    ↓ [Vertex Shader via instance_index]
Per-Instance Rendering
```

##### Components
```rust
/// Per-instance particle data for GPU upload
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleInstanceData {
    pub size: f32,
    pub color: [f32; 4], // RGBA
}
```

##### Resources
```rust
/// GPU buffer containing per-instance particle data
#[derive(Resource)]
pub struct ParticleInstanceBuffer {
    pub buffer: Buffer,
    pub capacity: usize,
    pub count: usize,
}

/// Bind group for instance data storage buffer
#[derive(Resource)]
pub struct ParticleInstanceBindGroup {
    pub bind_group: BindGroup,
}
```

##### Shader Modifications
```wgsl
// Bind storage buffer for instance data
@group(0) @binding(3)
var<storage, read> instance_data: array<ParticleInstanceData>;

struct ParticleInstanceData {
    size: f32,
    color: vec4<f32>,
}

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Get instance data using instance ID
    let instance_idx = @builtin(instance_index);
    let instance = instance_data[instance_idx];
    
    // Calculate world position
    let world_pos = model * vec4<f32>(input.position, 1.0);
    output.clip_position = view.view_proj * world_pos;
    
    // Use instance color
    output.color = instance.color;
    
    return output;
}
```

##### Systems

**Extract System (ExtractSchedule):**
```rust
pub fn extract_particle_instance_data(
    mut commands: Commands,
    particles: Query<&Particle>,
    particle_count: Res<ParticleCount>,
) {
    if particle_count.0 == 0 {
        return;
    }
    
    let instance_data: Vec<ParticleInstanceData> = particles
        .iter()
        .map(|p| ParticleInstanceData {
            size: p.size,
            color: [
                p.color.to_linear().red,
                p.color.to_linear().green,
                p.color.to_linear().blue,
                p.color.to_linear().alpha,
            ],
        })
        .collect();
    
    commands.insert_resource(ExtractedParticleInstances(instance_data));
}
```

**Render System (Render Set):**
```rust
pub fn prepare_particle_instance_buffer(
    mut commands: Commands,
    extracted: Res<ExtractedParticleInstances>,
    render_device: Res<RenderDevice>,
    mut instance_buffer: ResMut<ParticleInstanceBuffer>,
    mut bind_group: ResMut<ParticleInstanceBindGroup>,
    pipeline_cache: Res<PipelineCache>,
    point_sprite_pipeline: Res<PointSpritePipeline>,
) {
    let data = &extracted.0;
    
    // Resize buffer if needed
    if data.len() > instance_buffer.capacity {
        let new_capacity = (data.len() * 2).next_power_of_two();
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("particle_instance_buffer"),
            size: (new_capacity * std::mem::size_of::<ParticleInstanceData>()) as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        instance_buffer.buffer = buffer;
        instance_buffer.capacity = new_capacity;
    }
    
    // Write data to buffer
    render_device.queue.write_buffer(
        &instance_buffer.buffer,
        0,
        bytemuck::cast_slice(data.as_slice()),
    );
    
    instance_buffer.count = data.len();
    
    // Create or update bind group
    // ...
}
```

#### Pros
✅ Scales to 100K+ particles (StorageBuffers can handle millions of entries)
✅ Direct GPU access from shader (fast reads)
✅ Clean separation of concerns
✅ Bevy 0.15 idiomatic approach
✅ Low CPU-GPU synchronization overhead (single buffer update per frame)
✅ Works with automatic instancing

#### Cons
❌ Requires custom shader modifications
❌ Requires render world systems and extract schedule integration
❌ More complex to implement initially
❌ Need to manage buffer resizing dynamically

#### Complexity: Medium-High
- Shader changes: Moderate
- Rust code changes: High (render world integration)
- Overall: Medium-High initial investment, but clean and scalable

#### Suitability for 10K-100K Particles: Excellent

---

### Approach 2: Per-Entity Material Instances

#### Overview
Create a separate material instance for each particle, each with different uniform values for size and color. This avoids custom buffers but is resource-intensive.

#### Architecture

##### Components
```rust
// No new components needed
// Use existing PointSpriteMaterial with per-entity material handles
```

##### Shader Modifications
```wgsl
// No shader changes needed
// Use material uniforms for size and color
struct PointSpriteMaterial {
    color: vec4<f32>,      // Per-material color
    base_size: f32,         // Per-material size
    attenuation_factor: f32,
}
```

##### Systems
```rust
pub fn update_particle_materials(
    mut materials: ResMut<Assets<PointSpriteMaterial>>,
    particles: Query<(&Particle, &MeshMaterial3d<PointSpriteMaterial>)>,
) {
    for (particle, material_handle) in particles.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.color = particle.color.to_linear();
            material.base_size = particle.size;
        }
    }
}
```

#### Pros
✅ Minimal shader changes (use existing uniforms)
✅ Simple implementation
✅ Works with Bevy's automatic instancing (sort of - per-material batching)

#### Cons
❌ **CRITICAL**: Breaks automatic instancing - each material is separate batch
❌ **CRITICAL**: O(N) draw calls instead of single instanced draw call
❌ **CRITICAL**: Extremely slow for 10K+ particles (10K+ draw calls)
❌ High CPU overhead (material updates)
❌ Cannot render 10K+ particles at acceptable framerates
❌ Ignores GPU instancing benefits entirely

#### Complexity: Low (but wrong approach)

#### Suitability for 10K-100K Particles: Poor (UNUSABLE)
- This approach is fundamentally incompatible with the requirements

---

### Approach 3: Dynamic Uniform Buffer with Instanced Batching

#### Overview
Use a Dynamic Uniform Buffer (DUB) containing per-instance data, with multiple draw calls batching subsets of instances. The DUB is bound with dynamic offsets for each batch.

#### Architecture

##### Components
```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleInstanceData {
    pub size: f32,
    pub color: [f32; 4],
}
```

##### Resources
```rust
#[derive(Resource)]
pub struct ParticleDynamicUniformBuffer {
    pub buffer: Buffer,
    pub capacity: usize,
}

#[derive(Resource)]
pub struct ParticleBatchConfig {
    pub batch_size: usize, // Number of instances per batch (e.g., 256 or 512)
}
```

##### Shader Modifications
```wgsl
@group(0) @binding(3)
var<uniform> instance_data: array<ParticleInstanceData>;

@vertex
fn vertex(input: VertexInput, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // Use instance index to access uniform array
    let instance = instance_data[instance_index];
    
    // ... rest of shader
}
```

##### Systems
```rust
pub fn prepare_particle_dynamic_uniforms(
    mut commands: Commands,
    particles: Query<&Particle>,
    particle_count: Res<ParticleCount>,
    render_device: Res<RenderDevice>,
    mut uniform_buffer: ResMut<ParticleDynamicUniformBuffer>,
    batch_config: Res<ParticleBatchConfig>,
) {
    // Collect instance data
    let instance_data: Vec<ParticleInstanceData> = particles
        .iter()
        .map(|p| ParticleInstanceData {
            size: p.size,
            color: /* ... */,
        })
        .collect();
    
    // Update buffer
    render_device.queue.write_buffer(
        &uniform_buffer.buffer,
        0,
        bytemuck::cast_slice(instance_data.as_slice()),
    );
}
```

##### Custom Draw System
```rust
pub fn draw_particles_in_batches(
    mut commands: Commands,
    particle_count: Res<ParticleCount>,
    batch_config: Res<ParticleBatchConfig>,
    point_mesh: Res<PointMesh>,
    // ... other resources
) {
    let num_instances = particle_count.0 as u32;
    let batch_size = batch_config.batch_size as u32;
    
    let mut instance_offset = 0;
    while instance_offset < num_instances {
        let batch_count = std::cmp::min(batch_size, num_instances - instance_offset);
        
        // Issue draw call for this batch
        commands.spawn(DrawIndirect {
            mesh: point_mesh.0.clone(),
            instance_count: batch_count,
            // Dynamic offset for this batch
            dynamic_offset: (instance_offset * std::mem::size_of::<ParticleInstanceData>()) as u32,
        });
        
        instance_offset += batch_count;
    }
}
```

#### Pros
✅ Leverages uniform buffers (GPU-optimized)
✅ Batching reduces draw calls (e.g., 100K particles with 256 batch size = ~400 draw calls)
✅ No custom shader complexity (uniform arrays are standard)
✅ Bevy supports dynamic uniform offsets

#### Cons
❌ UBO size limitation: Typically 64KB max per buffer
- `ParticleInstanceData` is 20 bytes (4 + 16)
- 64KB / 20 = ~3,200 instances per buffer max
- For 100K particles, need multiple buffers or frequent updates
❌ Multiple draw calls required (though fewer than approach 2)
❌ Complex batch management
❌ Dynamic offset calculation required
❌ May not be efficient for >10K particles due to UBO limits

#### Complexity: Medium-High

#### Suitability for 10K-100K Particles: Medium-Low
- Works for 1K-10K particles reasonably well
- Struggles beyond 10K due to UBO size limitations
- StorageBuffer (Approach 1) is superior for larger counts

---

### Approach 4: Compute Shader Indirect Draw

#### Overview
Use a compute shader to update particle positions and prepare an indirect draw buffer, then issue a single GPU-driven draw call. Most GPU-efficient but most complex.

#### Architecture

##### Components
```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleComputeData {
    pub position: Vec3,
    pub size: f32,
    pub color: [f32; 4],
}
```

##### Resources
```rust
#[derive(Resource)]
pub struct ParticleComputeBuffers {
    pub data_buffer: Buffer,      // Read-write storage buffer
    pub indirect_buffer: Buffer,  // Indirect draw parameters
}

#[derive(Resource)]
pub struct ParticleComputePipeline {
    pub pipeline: CachedComputePipelineId,
}
```

##### Compute Shader
```wgsl
@group(0) @binding(0)
var<storage, read_write> particle_data: array<ParticleComputeData>;

@group(0) @binding(1)
var<uniform> time_delta: f32;

@group(0) @binding(2)
var<storage, read_write> draw_indirect: DrawIndirectArgs;

@compute
@workgroup_size(256)
fn update_particles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&particle_data)) {
        return;
    }
    
    // Update particle position on GPU
    // ...
    
    // Update indirect draw args
    draw_indirect.vertex_count = 1;
    draw_indirect.instance_count = arrayLength(&particle_data);
    draw_indirect.first_vertex = 0;
    draw_indirect.first_instance = 0;
}
```

##### Vertex Shader
```wgsl
@group(0) @binding(0)
var<storage, read> particle_data: array<ParticleComputeData>;

@vertex
fn vertex(@builtin(instance_index) instance_index: u32) -> VertexOutput {
    var output: VertexOutput;
    let particle = particle_data[instance_index];
    
    let world_pos = vec4<f32>(particle.position, 1.0);
    output.clip_position = view.view_proj * world_pos;
    output.color = particle.color;
    
    return output;
}
```

#### Pros
✅ **Highest GPU efficiency** - Single draw call
✅ Physics and rendering both on GPU (zero CPU overhead)
✅ Scales to 1M+ particles
✅ Eliminates CPU-GPU synchronization entirely
✅ Best performance for large particle counts

#### Cons
❌ Most complex to implement
❌ Requires compute shader development
❌ Requires indirect draw API integration
❌ Debugging compute shaders is difficult
❌ Need GPU-side physics simulation (major architecture change)
❌ Bevy integration is complex (custom render pipeline)

#### Complexity: Very High

#### Suitability for 10K-100K Particles: Excellent (if implementation effort is acceptable)

---

### Approach 5: Transform-Only Instancing with Uniform Colors

#### Overview
Accept that per-instance sizes aren't feasible with Bevy's automatic instancing, and use uniform colors/sizes. Rely on Transform for individual positions and accept visual limitations.

#### Architecture

##### No Changes Needed
- Keep current implementation
- Remove `ATTRIBUTE_INSTANCE_SIZE` and `ATTRIBUTE_INSTANCE_COLOR` from mesh
- Use material uniforms for global color/size
- All particles render identically

#### Pros
✅ Zero implementation effort
✅ Maintains GPU instancing
✅ Simplest approach

#### Cons
❌ **Fails requirements**: Cannot support per-particle colors and sizes
❌ Thermal gradient visualization impossible
❌ All particles look identical
❌ Doesn't solve the actual problem

#### Complexity: Zero (but incorrect)

#### Suitability: Unacceptable

---

## Recommended Approach: Storage Buffer with Instance Index (Approach 1)

### Justification

**Approach 1 (Storage Buffer)** is recommended because:

1. **Scalability**: StorageBuffers can handle millions of entries, easily meeting 10K-100K requirement
2. **Performance**: Single buffer update per frame, single instanced draw call, direct GPU access
3. **Bevy Compatibility**: Uses standard Bevy 0.15 render APIs (ExtractSchedule, Render resources)
4. **Correctness**: Properly supports per-instance colors and sizes
5. **Complexity**: Medium-High but manageable with Bevy's render system
6. **Maintainability**: Clean separation between game logic (CPU) and rendering (GPU)
7. **Future-Proof**: Easy to extend with more per-instance attributes (e.g., velocity, energy)

### Comparison Summary

| Approach | 10K Particles | 100K Particles | Complexity | Draw Calls | GPU Efficiency | Recommended |
|----------|---------------|----------------|------------|------------|----------------|-------------|
| 1. Storage Buffer | ✓ Excellent | ✓ Excellent | Medium-High | 1 | High | ✓ YES |
| 2. Per-Material | ✗ Unusable | ✗ Unusable | Low | 10K-100K | None | ✗ NO |
| 3. Dynamic Uniform | ✓ Good | ✗ Poor | Medium-High | 40-400 | Medium | ✗ NO |
| 4. Compute Shader | ✓ Excellent | ✓ Excellent | Very High | 1 | Very High | Maybe (future) |
| 5. Transform Only | N/A | N/A | Zero | 1 | High | ✗ NO |

---

## Detailed Design: Storage Buffer Implementation

### Code Structure

#### File: `genesis-render/src/particle/instance_buffer.rs` (NEW)

```rust
//! Per-instance particle data buffer management
//!
//! This module handles the CPU-to-GPU synchronization of per-instance
//! particle data using storage buffers. Particles are indexed by their
//! entity ID or insertion order, and the shader reads instance data
//! using the instance index.

use bevy::render::render_resource::{Buffer, BufferDescriptor, BufferUsages, ShaderType};
use bevy::render::renderer::RenderDevice;
use bevy::prelude::*;

/// Per-instance particle data for GPU upload
///
/// This struct is #[repr(C)] and implements Pod for direct GPU memory mapping.
/// Total size: 20 bytes (1 * 4 + 1 * 16)
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleInstanceData {
    /// Particle size in world units
    pub size: f32,
    /// Particle color as RGBA (linear space)
    pub color: [f32; 4],
}

impl ShaderType for ParticleInstanceData {
    const METADATA: bevy::render::render_resource::ShaderMetadata = {
        bevy::render::render_resource::ShaderMetadata {
            min_size: std::mem::size_of::<Self>() as u64,
            uniform: false,
        }
    };
}

/// GPU buffer containing per-instance particle data
///
/// This buffer is updated every frame from the ExtractedParticleInstances resource.
#[derive(Resource)]
pub struct ParticleInstanceBuffer {
    /// The GPU buffer handle
    pub buffer: Buffer,
    /// Maximum number of instances the buffer can hold
    pub capacity: usize,
    /// Current number of instances in the buffer
    pub count: usize,
}

/// Extracted instance data from main world to render world
///
/// This resource is populated by an extract system and consumed by
/// render world systems to update the GPU buffer.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct ExtractedParticleInstances(pub Vec<ParticleInstanceData>);

/// Bind group for accessing instance data in shader
#[derive(Resource)]
pub struct ParticleInstanceBindGroup {
    pub bind_group: BindGroup,
}

/// Extract system: Collect particle data from main world
///
/// Runs in ExtractSchedule to move data from main world to render world.
/// This system queries all Particle entities and creates a compact array
/// of instance data indexed by iteration order.
pub fn extract_particle_instance_data(
    mut commands: Commands,
    particles: Query<&Particle>,
    particle_count: Res<ParticleCount>,
) {
    if particle_count.0 == 0 {
        return;
    }

    let mut instance_data = Vec::with_capacity(particle_count.0);
    
    for particle in particles.iter() {
        let linear_color = particle.color.to_linear();
        instance_data.push(ParticleInstanceData {
            size: particle.size,
            color: [linear_color.red, linear_color.green, linear_color.blue, linear_color.alpha],
        });
    }
    
    commands.insert_resource(ExtractedParticleInstances(instance_data));
}

/// Prepare system: Create/resize and populate GPU buffer
///
/// Runs in render world to update the GPU buffer with extracted data.
/// This system handles buffer resizing if the particle count exceeds capacity.
pub fn prepare_particle_instance_buffer(
    mut commands: Commands,
    extracted: Res<ExtractedParticleInstances>,
    render_device: Res<RenderDevice>,
    mut instance_buffer: ResMut<ParticleInstanceBuffer>,
) {
    let data = &extracted.0;
    
    // Resize buffer if needed (grow only, never shrink)
    if data.len() > instance_buffer.capacity {
        let new_capacity = (data.len() * 2).next_power_of_two();
        let buffer_size = new_capacity * std::mem::size_of::<ParticleInstanceData>();
        
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("particle_instance_buffer"),
            size: buffer_size as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        
        commands.insert_resource(ParticleInstanceBuffer {
            buffer,
            capacity: new_capacity,
            count: 0,
        });
        
        // Update the mutable reference
        instance_buffer.buffer = buffer;
        instance_buffer.capacity = new_capacity;
    }
    
    // Write data to buffer
    render_device.queue.write_buffer(
        &instance_buffer.buffer,
        0,
        bytemuck::cast_slice(data.as_slice()),
    );
    
    instance_buffer.count = data.len();
}
```

#### File: `genesis-render/src/particle/bind_group_layout.rs` (NEW)

```rust
//! Bind group layouts for particle instance data binding

use bevy::render::render_resource::{BindGroupLayout, BindGroupLayoutEntry, BindingType, ShaderStages};
use bevy::render::renderer::RenderDevice;
use bevy::prelude::*;

/// Global bind group layout for particle instance storage buffer
///
/// This layout defines how the instance storage buffer is bound to shaders.
/// It should be added to the Material shader via the `bind_group_layout` attribute.
#[derive(Resource, Clone)]
pub struct ParticleInstanceBindGroupLayout {
    pub bind_group_layout: BindGroupLayout,
}

impl FromWorld for ParticleInstanceBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        
        let bind_group_layout = render_device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("particle_instance_bind_group_layout"),
                entries: &[
                    // Binding 3: Instance data storage buffer
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: bevy::render::render_resource::BufferBindingType::Storage {
                                read_only: true,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: Some(
                                std::num::NonZeroU64::new(
                                    std::mem::size_of::<ParticleInstanceData>() as u64
                                ).unwrap()
                            ),
                        },
                        count: None,
                    },
                ],
            }
        );
        
        Self { bind_group_layout }
    }
}

/// System to create bind group for instance data
///
/// This system creates the bind group that connects the instance buffer
/// to the shader binding point.
pub fn create_particle_instance_bind_group(
    mut commands: Commands,
    instance_buffer: Res<ParticleInstanceBuffer>,
    bind_group_layout: Res<ParticleInstanceBindGroupLayout>,
    render_device: Res<RenderDevice>,
) {
    if instance_buffer.count == 0 {
        return;
    }
    
    let bind_group = render_device.create_bind_group(
        &BindGroupDescriptor {
            label: Some("particle_instance_bind_group"),
            layout: &bind_group_layout.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 3,
                    resource: instance_buffer.buffer.as_entire_binding(),
                },
            ],
        }
    );
    
    commands.insert_resource(ParticleInstanceBindGroup { bind_group });
}
```

#### File: `assets/point_sprite.wgsl` (MODIFIED)

```wgsl
// Point sprite shader for particle rendering
// Renders particles as GPU point sprites with per-instance attributes from storage buffer

// Vertex input from mesh (only position needed, other data from storage buffer)
struct VertexInput {
    @location(0) position: vec3<f32>,
}

// Per-instance data from storage buffer
struct ParticleInstanceData {
    size: f32,
    color: vec4<f32>,
}

// Vertex output to fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

// Uniform bindings for the material
struct PointSpriteMaterial {
    color: vec4<f32>,          // Fallback color (unused when instance data available)
    base_size: f32,             // Fallback size (unused when instance data available)
    attenuation_factor: f32,
}

// View uniform containing camera/view data
struct ViewUniform {
    view_proj: mat4x4<f32>,
    world_position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> material: PointSpriteMaterial;

@group(0) @binding(1)
var<uniform> view: ViewUniform;

@group(0) @binding(2)
var<uniform> model: mat4x4<f32>;

// STORAGE BUFFER: Per-instance particle data (NEW BINDING)
@group(0) @binding(3)
var<storage, read> instance_data: array<ParticleInstanceData>;

// Vertex shader
@vertex
fn vertex(input: VertexInput, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // Get per-instance data from storage buffer
    let instance = instance_data[instance_index];
    
    // Calculate world position from input position and model matrix
    let world_pos = model * vec4<f32>(input.position, 1.0);
    
    // Transform position to clip space for rendering
    output.clip_position = view.view_proj * world_pos;
    
    // Pass per-instance particle color to fragment shader
    output.color = instance.color;
    
    return output;
}

// Fragment shader
@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simply output the color for the entire point
    return input.color;
}
```

#### File: `genesis-render/src/particle/mod.rs` (MODIFIED)

Add imports and integrate new modules:

```rust
// At the top of the file
mod instance_buffer;
mod bind_group_layout;

pub use instance_buffer::{
    ParticleInstanceData,
    ParticleInstanceBuffer,
    ExtractedParticleInstances,
    ParticleInstanceBindGroup,
    extract_particle_instance_data,
    prepare_particle_instance_buffer,
};

pub use bind_group_layout::{
    ParticleInstanceBindGroupLayout,
    create_particle_instance_bind_group,
};

// In PointSpriteMaterial impl, add bind_group_layout method:
impl Material for PointSpriteMaterial {
    // ... existing methods ...

    fn bind_group_layout(render_device: &bevy::render::renderer::RenderDevice) -> bevy::render::render_resource::BindGroupLayout {
        // Import the instance bind group layout
        let layout = ParticleInstanceBindGroupLayout::from_world(&mut World::new());
        layout.bind_group_layout
    }
}
```

### Required Components

#### No New Components Required
- Existing `Particle` component provides all necessary data
- Instance data is extracted from existing components

### Required Resources

| Resource | Type | Purpose | Lifetime |
|----------|------|---------|----------|
| `ParticleInstanceBuffer` | Resource | GPU buffer handle and metadata | Persistent |
| `ExtractedParticleInstances` | Resource | Transferred data from main world | Per-frame (extracted) |
| `ParticleInstanceBindGroup` | Resource | Bind group for shader binding | Per-frame (render world) |
| `ParticleInstanceBindGroupLayout` | Resource | Bind group layout definition | Persistent |
| `ParticleCount` | Resource | Already exists, used for buffer sizing | Persistent |

### Required Systems

| System | Schedule | Purpose | Parameters |
|--------|----------|---------|------------|
| `extract_particle_instance_data` | ExtractSchedule | Transfer Particle data to render world | `Query<&Particle>`, `Res<ParticleCount>` |
| `prepare_particle_instance_buffer` | Render (Prepare) | Update GPU buffer with instance data | `Res<ExtractedParticleInstances>`, `ResMut<ParticleInstanceBuffer>`, `Res<RenderDevice>` |
| `create_particle_instance_bind_group` | Render (Prepare) | Create bind group for shader | `Res<ParticleInstanceBuffer>`, `Res<ParticleInstanceBindGroupLayout>`, `Res<RenderDevice>` |

### System Ordering

```rust
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PointSpriteMaterial>::default())
            .init_resource::<ParticleCount>()
            
            // Existing systems
            .add_systems(Startup, init_point_mesh)
            .add_systems(Startup, spawn_particles.after(init_point_mesh))
            .add_systems(Update, update_particles)
            .add_systems(Update, update_particle_energy_colors)
            
            // NEW: Extract system (main world -> render world)
            .add_systems(ExtractSchedule, extract_particle_instance_data)
            
            // NEW: Render world systems
            .add_systems(
                Render,
                prepare_particle_instance_buffer
                    .run_if(resource_exists::<ExtractedParticleInstances>)
            )
            .add_systems(
                Render,
                create_particle_instance_bind_group
                    .after(prepare_particle_instance_buffer)
                    .run_if(resource_exists::<ParticleInstanceBuffer>)
            );
            
        // Remove the old sync system
        // .add_systems(Update, sync_particle_instance_attributes ...) // DEPRECATED
    }
}
```

### Memory Requirements

#### CPU Side
- `ExtractedParticleInstances::0`: `Vec<ParticleInstanceData>` 
- 100K particles × 20 bytes = 2 MB (negligible)

#### GPU Side
- `ParticleInstanceBuffer.buffer`: GPU storage buffer
- 200K capacity (rounded up) × 20 bytes = 4 MB (minimal)

#### Total Memory Impact
- ~6 MB total for 100K particles
- Scales linearly with particle count

### Performance Characteristics

#### CPU
- **Extract system**: O(N) iteration over particles
- **Prepare system**: O(1) single buffer write (GPU operation)
- **Total per frame**: ~1-2 ms for 100K particles

#### GPU
- **Buffer update**: Single copy of ~2 MB data
- **Shader read**: Direct storage buffer access (fast, cached)
- **Draw call**: Single instanced draw call
- **Total per frame**: Negligible GPU overhead

#### Bottleneck Analysis
- Not CPU-bound (minimal per-frame work)
- Not GPU-bound (storage buffer reads are fast)
- Bandwidth-bound at 10M+ particles
- **Excellent fit for 10K-100K particles**

### Integration Points

#### 1. Material Plugin Integration
```rust
// In ParticlePlugin::build()
app.add_plugins(MaterialPlugin::<PointSpriteMaterial>::default())
```

The `MaterialPlugin` handles:
- Shader compilation and pipeline creation
- Material binding groups
- Automatic mesh-material batching

#### 2. Bind Group Integration
The storage buffer bind group must be integrated with the material's bind group:

```rust
// The material's vertex shader needs to access both:
// - @group(0) @binding(0) material uniforms
// - @group(0) @binding(1) view uniforms
// - @group(0) @binding(2) model transform
// - @group(0) @binding(3) instance storage buffer (NEW)
```

Bevy's `Material` trait supports custom bind group layouts. We'll need to ensure our storage buffer bind group is compatible with the material's existing bind group layout.

#### 3. Render World Integration
Bevy's render pipeline has two phases:
1. **Extract Schedule**: Transfer data from main world to render world
2. **Render**: GPU operations

Our systems fit into this pipeline:
- `extract_particle_instance_data`: ExtractSchedule
- `prepare_particle_instance_buffer`: Render (Prepare set)
- `create_particle_instance_bind_group`: Render (Prepare set)

### Testing Strategy

#### Unit Tests
```rust
#[test]
fn test_particle_instance_data_size() {
    assert_eq!(std::mem::size_of::<ParticleInstanceData>(), 20);
}

#[test]
fn test_particle_instance_data_alignment() {
    assert!(std::mem::align_of::<ParticleInstanceData>() >= 4);
}
```

#### Integration Tests
```rust
#[test]
fn test_extract_system_creates_correct_instance_count() {
    // Spawn 100 particles
    // Run extract system
    // Verify ExtractedParticleInstances has 100 entries
}

#[test]
fn test_buffer_resizing_logic() {
    // Test that buffer grows when capacity exceeded
}
```

#### Visual Verification
- Render particles with varying colors
- Verify thermal gradient visualization works
- Check that individual particle sizes are visible

### Migration Path

#### Phase 1: Infrastructure
1. Create `instance_buffer.rs` module
2. Create `bind_group_layout.rs` module
3. Implement data structures and systems

#### Phase 2: Shader Integration
1. Update `point_sprite.wgsl` with storage buffer binding
2. Test shader compilation

#### Phase 3: System Integration
1. Add systems to `ParticlePlugin`
2. Remove old `sync_particle_instance_attributes` system
3. Remove `ATTRIBUTE_INSTANCE_SIZE` and `ATTRIBUTE_INSTANCE_COLOR` from mesh

#### Phase 4: Testing
1. Unit tests
2. Integration tests
3. Visual verification

### Fallback Strategy

If storage buffer approach encounters issues:

1. **Fallback to Dynamic Uniform Buffer** (Approach 3)
   - Works for up to ~3K particles
   - Simpler implementation
   - Can be used as stopgap

2. **Fallback to Per-Material** (Approach 2) with limited particles
   - Only render 1K particles
   - Accept uniform colors
   - Use for testing/debugging

### Future Enhancements

#### Phase 2: Additional Per-Instance Attributes
Add support for:
- Particle energy (for more complex gradients)
- Velocity (for motion visualization)
- Age (for lifetime-based effects)
- Custom shader effects

#### Phase 3: GPU Physics (Compute Shader)
Migrate to Approach 4 for maximum performance:
- Compute shader updates particle positions
- Indirect draw calls
- Zero CPU overhead

---

## Appendix: Bevy 0.15 API Reference

### Key Types and Modules

#### bevy::render::render_resource
- `Buffer`: GPU buffer handle
- `BufferDescriptor`: Buffer creation parameters
- `BufferUsages`: Buffer usage flags (COPY_DST, STORAGE, VERTEX, etc.)
- `BufferBindingType`: Uniform, Storage, etc.
- `BindGroup`: GPU resource binding group
- `BindGroupLayout`: Bind group layout definition
- `BindGroupEntry`: Single binding in bind group
- `BindGroupDescriptor`: Bind group creation parameters
- `ShaderStages`: Which shader stages use a binding (VERTEX, FRAGMENT, COMPUTE)
- `ShaderType`: Trait for types that can be used in shaders

#### bevy::render::renderer
- `RenderDevice`: Low-level GPU device interface
- `RenderQueue`: Command queue for GPU operations
- `create_buffer()`: Create GPU buffer
- `create_bind_group()`: Create bind group

#### bevy::render::render_asset
- `RenderAssets<T>`: GPU-side assets
- `RenderApp`: Bevy's render sub-app

#### bevy::app::Schedule
- `ExtractSchedule`: Systems that transfer data from main world to render world
- `Render`: Render world systems

### Shader Type Support

For a struct to be used in WGSL storage buffers, it must:
1. Be `#[repr(C)]`
2. Be `Pod` and `Zeroable` (from bytemuck)
3. Implement `ShaderType` trait
4. Have explicit field sizes (no enums, no references)

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleInstanceData {
    pub size: f32,        // 4 bytes
    pub color: [f32; 4],  // 16 bytes
} // Total: 20 bytes
```

### WGSL Storage Buffer Syntax

```wgsl
// Declare storage buffer
@group(0) @binding(3)
var<storage, read> instance_data: array<ParticleInstanceData>;

// Access in shader
let instance = instance_data[instance_index];
let size = instance.size;
let color = instance.color;
```

### Bevy Material System Integration

For custom bind groups in materials:

```rust
impl Material for PointSpriteMaterial {
    fn vertex_shader() -> ShaderRef { /* ... */ }
    fn fragment_shader() -> ShaderRef { /* ... */ }
    
    // Optional: Custom bind group layout
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        // Return custom layout with storage buffer binding
    }
}
```

---

## Conclusion

This design document recommends **Approach 1 (Storage Buffer with Instance Index)** for implementing per-instance particle attributes in Genesis. This approach:

- Meets all requirements (10K-100K particles, per-instance colors/sizes)
- Leverages Bevy 0.15's render system idiomatically
- Provides excellent performance with minimal overhead
- Scales well beyond the initial requirements
- Maintains clean architecture and separation of concerns

The implementation requires:
1. New modules for buffer management (`instance_buffer.rs`, `bind_group_layout.rs`)
2. Shader updates to use storage buffer
3. Extract and prepare systems in Bevy's render pipeline
4. ~6 MB GPU memory for 100K particles

The next step is to implement this design following the phases outlined in the Migration Path section.
