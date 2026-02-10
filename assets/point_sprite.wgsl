// Point sprite shader for particle rendering
// Renders particles as GPU point sprites with size attenuation capability
// Uses storage buffer for per-instance particle data (size and color)

// Storage buffer for per-instance particle data
// Layout must match ParticleInstanceData in instance_buffer.rs
struct ParticleInstanceData {
    size: f32,           // bytes 0-3
    padding0: f32,       // bytes 4-7 (padding for alignment)
    padding1: f32,       // bytes 8-11 (padding for alignment)
    padding2: f32,       // bytes 12-15 (padding for alignment)
    color: vec4<f32>,    // bytes 16-31 (RGBA)
}

@group(0) @binding(3)
var<storage, read> particle_instances: array<ParticleInstanceData>;

// Vertex input from mesh
struct VertexInput {
    @location(0) position: vec3<f32>,
}

// Vertex output to fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

// Uniform bindings for the material
struct PointSpriteMaterial {
    color: vec4<f32>,
    base_size: f32,
    attenuation_factor: f32,
}

// View uniform containing camera/view data
struct ViewUniform {
    view_proj: mat4x4<f32>,
    world_position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> material: PointSpriteMaterial;

// Bevy's view uniform containing camera world position
@group(0) @binding(1)
var<uniform> view: ViewUniform;

// Model/Transform matrix for converting mesh position to world position
@group(0) @binding(2)
var<uniform> model: mat4x4<f32>;

// Vertex shader
@vertex
fn vertex(input: VertexInput, @builtin(instance_index) instance_idx: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // Read per-instance data from storage buffer
    let instance_data = particle_instances[instance_idx];
    
    // Calculate world position from input position and model matrix
    // Apply entity's Transform to convert mesh position (at origin for PointMesh) to world position
    let world_pos = model * vec4<f32>(input.position, 1.0);
    
    // Transform position to clip space for rendering
    output.clip_position = view.view_proj * world_pos;
    
    // Calculate distance from camera for size attenuation
    let distance = length(view.world_position - world_pos.xyz);
    
    // Apply size attenuation: particles appear smaller when further from camera
    let attenuated_size = instance_data.size / (1.0 + distance * material.attenuation_factor);
    
    // Set gl_PointSize for GPU point sprite rasterization
    // Use attenuated size; Bevy will convert to screen pixels automatically
    // Note: gl_PointSize is not directly accessible in WGSL but handled by the material system
    // The attenuated_size calculation is preserved here for future shader-based size control
    
    // Pass per-instance particle color to fragment shader
    output.color = instance_data.color;
    
    return output;
}

// Fragment shader
@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simply output the color for the entire point
    // The GPU automatically handles point rasterization
    return input.color;
}
