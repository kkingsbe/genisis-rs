// Point sprite shader for particle rendering
// Renders particles as GPU point sprites with size attenuation capability

// Vertex input from mesh
struct VertexInput {
    @location(0) position: vec3<f32>,
}

// Vertex output to fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @builtin(point_size) point_size: f32,
}

// Uniform bindings for the material
struct PointSpriteMaterial {
    color: vec4<f32>,
    base_size: f32,
}

@group(0) @binding(0)
var<uniform> material: PointSpriteMaterial;

// Bevy's view uniform containing camera world position
@group(0) @binding(1)
var<uniform> view: ViewUniform;

// Vertex shader
@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Calculate world position from input position and model matrix
    // Bevy provides the model matrix automatically at @location(1) for position
    let world_pos = vec4<f32>(input.position, 1.0);
    
    // Transform position to clip space for rendering
    output.clip_position = view.view_proj * world_pos;
    
    // Calculate distance from camera to particle
    let distance = distance(view.world_position, world_pos.xyz);
    
    // Apply size attenuation formula
    // Particles appear smaller as they move farther from the camera
    // Formula: size = base_size / (1.0 + distance * attenuation_factor)
    let attenuation_factor: f32 = 0.01;
    let attenuated_size = material.base_size / (1.0 + distance * attenuation_factor);
    
    // Clamp to minimum size (1.0 pixel) to prevent particles from vanishing
    output.point_size = max(attenuated_size, 1.0);
    
    // Pass material color to fragment shader
    output.color = material.color;
    
    return output;
}

// Fragment shader
@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simply output the color for the entire point
    // The GPU automatically handles point rasterization
    return input.color;
}
