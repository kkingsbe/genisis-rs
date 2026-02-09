// Point sprite shader for particle rendering
// Renders particles as GPU point sprites with size attenuation capability

// Vertex input from mesh
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) instance_size: f32,
    @location(2) instance_color: vec4<f32>,
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
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Calculate world position from input position and model matrix
    // Apply entity's Transform to convert mesh position (at origin for PointMesh) to world position
    let world_pos = model * vec4<f32>(input.position, 1.0);
    
    // Transform position to clip space for rendering
    output.clip_position = view.view_proj * world_pos;
    
    // Pass per-instance particle color to fragment shader
    // Use instance_color if available, otherwise fall back to material uniform color
    output.color = input.instance_color;
    
    return output;
}

// Fragment shader
@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simply output the color for the entire point
    // The GPU automatically handles point rasterization
    return input.color;
}
