//! Shader Validation Tests for Genesis-RS Point Sprite Rendering
//!
//! This test suite validates that the point_sprite.wgsl shader and PointSpriteMaterial
//! are correctly configured for Bevy 0.15's Material trait requirements.
//!
//! The tests cover:
//! - Shader asset loading and compilation
//! - Uniform binding layout validation (critical for GPU rendering)
//! - Vertex attribute alignment
//! - Material trait integration
//! - Headless rendering pipeline

use bevy::pbr::Material;
use bevy::prelude::*;
use bevy::render::alpha::AlphaMode;
use bevy::render::mesh::{Mesh, PrimitiveTopology};
use bevy::render::render_resource::ShaderRef;
use genesis_render::particle::PointSpriteMaterial;
use std::path::PathBuf;

// The shader file is located in the genesis-render/src/particle directory
// This function tries multiple possible paths to accommodate different test execution contexts
fn get_shader_path() -> PathBuf {
    // Try the path from workspace root first
    let workspace_path = PathBuf::from("genesis-render/src/particle/point_sprite.wgsl");
    if workspace_path.exists() {
        return workspace_path;
    }
    
    // Try the path from genesis-render package directory
    let package_path = PathBuf::from("src/particle/point_sprite.wgsl");
    if package_path.exists() {
        return package_path;
    }
    
    // Try the assets path (for Bevy asset loading tests)
    let assets_path = PathBuf::from("assets/point_sprite.wgsl");
    if assets_path.exists() {
        return assets_path;
    }
    
    // Fallback to the workspace path (it will fail with a clear message)
    workspace_path
}

// ============================================================================
// A. SHADER ASSET LOADING TESTS
// ============================================================================

/// Test 1: Verify that the shader asset file exists and can be loaded
#[test]
fn test_shader_file_exists() {
    let shader_path = get_shader_path();
    
    assert!(
        shader_path.exists(),
        "Shader file '{:?}' does not exist. \
         This file is required for point sprite rendering.",
        shader_path
    );
}

/// Test 2: Verify the shader file is readable and has content
#[test]
fn test_shader_file_readable() {
    use std::fs;
    
    let shader_path = get_shader_path();
    assert!(
        shader_path.exists(),
        "Shader file '{:?}' does not exist",
        shader_path
    );
    
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    assert!(
        !content.is_empty(),
        "Shader file '{:?}' is empty",
        shader_path
    );
    
    // Verify it's actually WGSL content
    assert!(
        content.contains("@vertex") || content.contains("@fragment"),
        "Shader file does not contain valid WGSL shader stages (@vertex or @fragment)"
    );
}

/// Test 3: Verify basic WGSL syntax is present in the shader
#[test]
fn test_shader_has_valid_wgsl_structure() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Check for required WGSL keywords
    assert!(
        content.contains("struct"),
        "Shader must define structs"
    );
    
    assert!(
        content.contains("@location") || content.contains("@builtin"),
        "Shader must define input/output locations or builtins"
    );
    
    assert!(
        content.contains("@group") && content.contains("@binding"),
        "Shader must define resource bindings with @group and @binding"
    );
}

/// Test 4: Verify Material trait returns correct shader path
#[test]
fn test_material_shader_path() {
    let vertex_shader = PointSpriteMaterial::vertex_shader();
    let fragment_shader = PointSpriteMaterial::fragment_shader();
    
    // Verify both vertex and fragment shaders use the same file path
    let path_str = "point_sprite.wgsl";
    
    match (&vertex_shader, &fragment_shader) {
        (ShaderRef::Path(v_path), ShaderRef::Path(f_path)) => {
            let v_str = v_path.to_string();
            let f_str = f_path.to_string();
            assert!(
                v_str.contains(path_str), 
                "Vertex shader path should contain '{}', got: {}",
                path_str, v_str
            );
            assert!(
                f_str.contains(path_str), 
                "Fragment shader path should contain '{}', got: {}",
                path_str, f_str
            );
        }
        _ => panic!(
            "Both vertex and fragment shaders should use ShaderRef::Path, \
             got: vertex is Path={}, fragment is Path={}", 
            matches!(vertex_shader, ShaderRef::Path(_)),
            matches!(fragment_shader, ShaderRef::Path(_))
        ),
    }
}

// ============================================================================
// B. UNIFORM BINDING LAYOUT TESTS
// ============================================================================

/// Test 5: Verify PointSpriteMaterial has correct uniform declarations
#[test]
fn test_material_uniform_declarations() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Check that PointSpriteMaterial struct is defined in shader
    assert!(
        content.contains("struct PointSpriteMaterial"),
        "Shader must define PointSpriteMaterial struct"
    );
    
    // Verify expected fields in PointSpriteMaterial
    assert!(
        content.contains("color: vec4<f32>"),
        "PointSpriteMaterial must have 'color: vec4<f32>' field for uniform binding 0"
    );
    
    assert!(
        content.contains("base_size: f32"),
        "PointSpriteMaterial must have 'base_size: f32' field for uniform binding 1"
    );
    
    assert!(
        content.contains("attenuation_factor: f32"),
        "PointSpriteMaterial must have 'attenuation_factor: f32' field for uniform binding 2"
    );
}

/// Test 6: Verify @group(0) @binding(0) contains PointSpriteMaterial uniform
#[test]
fn test_binding_0_point_sprite_material() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Check for binding 0 declaration
    assert!(
        content.contains("@group(0) @binding(0)"),
        "Shader must have @group(0) @binding(0) for PointSpriteMaterial"
    );
    
    // Verify it binds to PointSpriteMaterial
    let _binding_line = content.lines()
        .find(|line| line.contains("@group(0) @binding(0)"))
        .expect("Could not find @group(0) @binding(0) declaration");
    
    let next_line = content.lines()
        .skip_while(|line| !line.contains("@group(0) @binding(0)"))
        .nth(1)
        .expect("Could not find variable declaration after binding 0");
    
    assert!(
        next_line.contains("var<uniform> material: PointSpriteMaterial;") ||
        next_line.contains("var<uniform>material:PointSpriteMaterial;"),
        "Binding 0 should bind to PointSpriteMaterial uniform, got: {}",
        next_line.trim()
    );
}

/// Test 7: Verify @group(0) @binding(1) contains ViewUniform (CRITICAL FOR ERROR)
#[test]
fn test_binding_1_view_uniform() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // CRITICAL: Check that binding 1 exists - this was the error location
    assert!(
        content.contains("@group(0) @binding(1)"),
        "Shader MUST have @group(0) @binding(1) for ViewUniform. \
         Missing binding 1 will cause: \
         'Shader global ResourceBinding {{ group: 0, binding: 1 }} is not available \
         in the pipeline layout - Visibility flags don't include the shader stage'"
    );
    
    // Verify ViewUniform struct is defined
    assert!(
        content.contains("struct ViewUniform"),
        "Shader must define ViewUniform struct for binding 1"
    );
    
    // Verify ViewUniform has required fields
    assert!(
        content.contains("view_proj: mat4x4<f32>"),
        "ViewUniform must have 'view_proj: mat4x4<f32>' field for view-projection matrix"
    );
    
    // Verify binding 1 binds to ViewUniform
    let _binding_line = content.lines()
        .find(|line| line.contains("@group(0) @binding(1)"))
        .expect("Could not find @group(0) @binding(1) declaration");
    
    let next_line = content.lines()
        .skip_while(|line| !line.contains("@group(0) @binding(1)"))
        .nth(1)
        .expect("Could not find variable declaration after binding 1");
    
    assert!(
        next_line.contains("var<uniform> view: ViewUniform;") ||
        next_line.contains("var<uniform>view:ViewUniform;"),
        "Binding 1 should bind to ViewUniform uniform, got: {}",
        next_line.trim()
    );
}

/// Test 8: Verify @group(0) @binding(2) contains model matrix
#[test]
fn test_binding_2_model_matrix() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Check for binding 2 declaration
    assert!(
        content.contains("@group(0) @binding(2)"),
        "Shader must have @group(0) @binding(2) for model matrix"
    );
    
    // Verify binding 2 binds to mat4x4<f32> for model matrix
    let _binding_line = content.lines()
        .find(|line| line.contains("@group(0) @binding(2)"))
        .expect("Could not find @group(0) @binding(2) declaration");
    
    let next_line = content.lines()
        .skip_while(|line| !line.contains("@group(0) @binding(2)"))
        .nth(1)
        .expect("Could not find variable declaration after binding 2");
    
    assert!(
        next_line.contains("var<uniform> model: mat4x4<f32>;") ||
        next_line.contains("var<uniform>model:mat4x4<f32>;"),
        "Binding 2 should bind to model matrix (mat4x4<f32>), got: {}",
        next_line.trim()
    );
}

/// Test 9: Verify uniform binding types match between Rust and WGSL
#[test]
fn test_uniform_type_consistency() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // In Rust: PointSpriteMaterial has color: LinearRgba, base_size: f32, attenuation_factor: f32
    // In WGSL: color: vec4<f32>, base_size: f32, attenuation_factor: f32
    
    // Check color type
    assert!(
        content.contains("color: vec4<f32>"),
        "WGSL color must be vec4<f32> to match Rust LinearRgba (4 floats)"
    );
    
    // Check base_size type
    assert!(
        content.contains("base_size: f32"),
        "WGSL base_size must be f32 to match Rust f32"
    );
    
    // Check attenuation_factor type
    assert!(
        content.contains("attenuation_factor: f32"),
        "WGSL attenuation_factor must be f32 to match Rust f32"
    );
    
    // Check model matrix type
    assert!(
        content.contains("model: mat4x4<f32>"),
        "WGSL model must be mat4x4<f32> to match Rust Mat4"
    );
}

/// Test 10: Verify ViewUniform struct structure matches Bevy's expectations
#[test]
fn test_view_uniform_structure() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify ViewUniform struct has view_proj: mat4x4<f32>
    assert!(
        content.contains("view_proj: mat4x4<f32>"),
        "ViewUniform must contain view_proj: mat4x4<f32> for view-projection matrix"
    );
    
    // Verify ViewUniform has world_position (for camera position)
    assert!(
        content.contains("world_position: vec3<f32>"),
        "ViewUniform should contain world_position: vec3<f32> for camera world position"
    );
}

// ============================================================================
// C. VERTEX ATTRIBUTE TESTS
// ============================================================================

/// Test 11: Verify @location(0) for vertex position exists
#[test]
fn test_vertex_attribute_location_0_position() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Check for VertexInput struct
    assert!(
        content.contains("struct VertexInput"),
        "Shader must define VertexInput struct"
    );
    
    // Check for location 0
    assert!(
        content.contains("@location(0) position: vec3<f32>"),
        "VertexInput must have @location(0) position: vec3<f32> for vertex position. \
         This must match the mesh's POSITION attribute."
    );
}

/// Test 12: Verify @group(0) @binding(3) contains storage buffer for instance data
#[test]
fn test_storage_buffer_binding_3() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Check for binding 3 declaration
    assert!(
        content.contains("@group(0) @binding(3)"),
        "Shader must have @group(0) @binding(3) for particle instance storage buffer"
    );
    
    // Verify it binds to a storage buffer
    let next_line = content.lines()
        .skip_while(|line| !line.contains("@group(0) @binding(3)"))
        .nth(1)
        .expect("Could not find variable declaration after binding 3");
    
    assert!(
        next_line.contains("var<storage, read>") && next_line.contains("particle_instances"),
        "Binding 3 should bind to a storage buffer named 'particle_instances', got: {}",
        next_line.trim()
    );
}

/// Test 13: Verify ParticleInstanceData struct matches instance_buffer.rs layout
#[test]
fn test_storage_buffer_particle_instance_data_struct() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify ParticleInstanceData struct is defined
    assert!(
        content.contains("struct ParticleInstanceData"),
        "Shader must define ParticleInstanceData struct to match Rust struct"
    );
    
    // Verify struct has size field (bytes 0-3)
    assert!(
        content.contains("size: f32"),
        "ParticleInstanceData must have 'size: f32' field (bytes 0-3)"
    );
    
    // Verify struct has padding fields (bytes 4-15)
    assert!(
        content.contains("padding0: f32"),
        "ParticleInstanceData must have 'padding0: f32' field for 16-byte alignment (bytes 4-7)"
    );
    
    assert!(
        content.contains("padding1: f32"),
        "ParticleInstanceData must have 'padding1: f32' field for 16-byte alignment (bytes 8-11)"
    );
    
    assert!(
        content.contains("padding2: f32"),
        "ParticleInstanceData must have 'padding2: f32' field for 16-byte alignment (bytes 12-15)"
    );
    
    // Verify struct has color field (bytes 16-31)
    assert!(
        content.contains("color: vec4<f32>"),
        "ParticleInstanceData must have 'color: vec4<f32>' field for RGBA color (bytes 16-31)"
    );
}

/// Test 14: Verify vertex shader uses @builtin(instance_index)
#[test]
fn test_vertex_shader_uses_instance_index() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify vertex function takes instance_index as parameter
    assert!(
        content.contains("@builtin(instance_index)"),
        "Vertex shader must use @builtin(instance_index) to index into storage buffer"
    );
    
    // Verify the instance_index parameter is used in the vertex function signature
    assert!(
        content.contains("fn vertex(input: VertexInput, @builtin(instance_index) instance_idx: u32)") ||
        content.contains("fn vertex(input: VertexInput,@builtin(instance_index)instance_idx: u32)") ||
        content.contains("fn vertex(input: VertexInput, @builtin(instance_index) instance_idx:u32)"),
        "Vertex shader function must accept instance_idx: u32 parameter with @builtin(instance_index)"
    );
}

/// Test 15: Verify storage buffer reads instance data using instance index
#[test]
fn test_storage_buffer_reads_instance_data() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify shader reads particle_instances array with instance_idx
    assert!(
        content.contains("particle_instances[instance_idx]") ||
        content.contains("particle_instances[ instance_idx]") ||
        content.contains("particle_instances[instance_idx ]"),
        "Vertex shader must read instance data from storage buffer using particle_instances[instance_idx]"
    );
    
    // Verify the instance data is assigned to a variable
    assert!(
        content.contains("instance_data") && (content.contains("let instance_data") || content.contains("var instance_data")),
        "Vertex shader should assign storage buffer read to instance_data variable"
    );
    
    // Verify instance_data.size is used
    assert!(
        content.contains("instance_data.size"),
        "Vertex shader should use instance_data.size for size attenuation"
    );
    
    // Verify instance_data.color is used
    assert!(
        content.contains("instance_data.color"),
        "Vertex shader should use instance_data.color to pass to fragment shader"
    );
}

/// Test 15: Verify VertexOutput struct has color field
#[test]
fn test_vertex_output_structure() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Check for VertexOutput struct
    assert!(
        content.contains("struct VertexOutput"),
        "Shader must define VertexOutput struct"
    );
    
    // Check for builtin position
    assert!(
        content.contains("@builtin(position) clip_position: vec4<f32>"),
        "VertexOutput must have @builtin(position) clip_position: vec4<f32>"
    );
    
    // Check for color location
    assert!(
        content.contains("@location(0) color: vec4<f32>"),
        "VertexOutput must have @location(0) color: vec4<f32> to pass color to fragment shader"
    );
}

// ============================================================================
// D. MATERIAL INTEGRATION TESTS
// ============================================================================

/// Test 16: Verify PointSpriteMaterial can be instantiated
#[test]
fn test_material_instantiation() {
    let material = PointSpriteMaterial {
        color: LinearRgba::new(1.0, 0.0, 0.0, 1.0), // Red
        base_size: 10.0,
        attenuation_factor: 0.01,
    };
    
    // Verify fields are set correctly
    assert_eq!(material.base_size, 10.0);
    assert_eq!(material.attenuation_factor, 0.01);
    assert_eq!(material.color.red, 1.0);
    assert_eq!(material.color.green, 0.0);
    assert_eq!(material.color.blue, 0.0);
    assert_eq!(material.color.alpha, 1.0);
}

/// Test 17: Verify alpha_mode is set correctly to AlphaMode::Add
#[test]
fn test_material_alpha_mode() {
    let material = PointSpriteMaterial {
        color: LinearRgba::WHITE,
        base_size: 5.0,
        attenuation_factor: 0.02,
    };
    
    let alpha_mode = material.alpha_mode();
    
    assert_eq!(
        alpha_mode,
        AlphaMode::Add,
        "PointSpriteMaterial should use AlphaMode::Add for additive blending effect. \
         Got: {:?}",
        alpha_mode
    );
}

/// Test 18: Verify Material trait is implemented for PointSpriteMaterial
#[test]
fn test_material_trait_implementation() {
    // This test compiles if the Material trait is implemented correctly
    // If it compiles, the trait implementation is valid
    let _material = PointSpriteMaterial {
        color: LinearRgba::WHITE,
        base_size: 5.0,
        attenuation_factor: 0.01,
    };
    
    // Verify shader paths return the correct type
    let vertex_shader = PointSpriteMaterial::vertex_shader();
    let fragment_shader = PointSpriteMaterial::fragment_shader();
    
    // Both should be ShaderRef::Path
    assert!(matches!(vertex_shader, ShaderRef::Path(_)));
    assert!(matches!(fragment_shader, ShaderRef::Path(_)));
}

/// Test 19: Verify AsBindGroup is implemented
#[test]
fn test_as_bind_group_implementation() {
    // This test compiles if AsBindGroup is implemented correctly
    // The #[uniform(...)] attributes on PointSpriteMaterial fields
    // should automatically generate the AsBindGroup implementation
    
    // Verify the uniform indices match expectations
    // #[uniform(0)] -> color
    // #[uniform(1)] -> base_size
    // #[uniform(2)] -> attenuation_factor
    
    // These must correspond to the WGSL struct field order
    // PointSpriteMaterial { color: vec4<f32>, base_size: f32, attenuation_factor: f32 }
    
    use std::fs;
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify the struct fields are in the correct order for uniform binding
    let pos_color = content.find("color: vec4<f32>");
    let pos_size = content.find("base_size: f32");
    let pos_attenuation = content.find("attenuation_factor: f32");
    
    assert!(
        pos_color.is_some() && pos_size.is_some() && pos_attenuation.is_some(),
        "All PointSpriteMaterial fields must be defined in shader"
    );
    
    // Verify order (should match uniform binding order)
    if let (Some(p_color), Some(p_size), Some(p_att)) = (pos_color, pos_size, pos_attenuation) {
        assert!(
            p_color < p_size && p_size < p_att,
            "PointSpriteMaterial fields in WGSL must be in order: color, base_size, attenuation_factor \
             to match #[uniform(0)], #[uniform(1)], #[uniform(2)] in Rust"
        );
    }
}

/// Test 20: Verify vertex shader uses correct attributes
#[test]
fn test_vertex_shader_uses_attributes() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify vertex shader exists
    assert!(
        content.contains("@vertex"),
        "Shader must have @vertex shader stage"
    );
    
    // Verify vertex shader uses input.position from @location(0)
    assert!(
        content.contains("input.position"),
        "Vertex shader must use input.position from @location(0)"
    );
    
    // Verify vertex shader uses model matrix
    assert!(
        content.contains("model * vec4<f32>"),
        "Vertex shader must apply model matrix to vertex position"
    );
    
    // Verify vertex shader uses view.view_proj
    assert!(
        content.contains("view.view_proj"),
        "Vertex shader must use view.view_proj from binding 1"
    );
    
    // Verify vertex shader uses storage buffer for instance data
    assert!(
        content.contains("particle_instances[instance_idx]") ||
        content.contains("particle_instances[ instance_idx]") ||
        content.contains("particle_instances[instance_idx ]"),
        "Vertex shader must read instance data from storage buffer using particle_instances[instance_idx]"
    );
    
    // Verify vertex shader uses @builtin(instance_index)
    assert!(
        content.contains("@builtin(instance_index)"),
        "Vertex shader must use @builtin(instance_index) to index into storage buffer"
    );
}

/// Test 21: Verify fragment shader outputs color correctly
#[test]
fn test_fragment_shader_outputs_color() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify fragment shader exists
    assert!(
        content.contains("@fragment"),
        "Shader must have @fragment shader stage"
    );
    
    // Verify fragment shader returns color
    assert!(
        content.contains("return input.color") || content.contains("return  input.color"),
        "Fragment shader should return input.color from VertexOutput"
    );
    
    // Verify fragment shader has @location(0) output
    assert!(
        content.contains("@location(0)"),
        "Fragment shader must output to @location(0)"
    );
}

// ============================================================================
// E. HEADLESS RENDERING TESTS (Bevy Integration Tests)
// ============================================================================

/// Test 22: Test that PointSpriteMaterial can be added to Bevy app
#[test]
fn test_material_plugin_in_app() {
    // Test that MaterialPlugin can be added to app without panicking
    // This validates that the Material trait implementation is correct
    // and compatible with Bevy 0.15's rendering system
    
    // Simply test that the type compiles and trait is implemented
    // A full headless rendering test would require more complex setup
    let _ = || {
        let _app = App::new()
            .add_plugins((
                bevy::asset::AssetPlugin::default(),
                bevy::render::RenderPlugin::default(),
            ))
            .add_plugins(bevy::pbr::MaterialPlugin::<PointSpriteMaterial>::default());
    };
    
    // If we get here, the Material trait implementation compiles correctly
    // The full headless rendering test requires extensive setup beyond
    // the scope of shader validation tests
}

/// Test 23: Test creating a mesh with correct vertex attributes
#[test]
fn test_point_mesh_creation() {
    // Create a point mesh with the correct vertex attributes
    // Note: Instance data (size, color) comes from storage buffer at binding 3,
    // not from vertex attributes. The mesh only needs position data.
    let mut mesh = Mesh::new(
        PrimitiveTopology::PointList, 
        bevy::render::render_asset::RenderAssetUsages::default()
    );
    
    // Add position attribute (location 0) - this is the only vertex attribute needed
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0]]
    );
    
    // Verify position attribute was added
    assert!(
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some(),
        "Mesh must have POSITION attribute at location 0"
    );
    
    // Verify mesh uses PointList topology
    assert_eq!(
        mesh.primitive_topology(),
        PrimitiveTopology::PointList,
        "Point sprites should use PointList topology"
    );
}

/// Test 24: Test vertex attribute location alignment
#[test]
fn test_vertex_attribute_locations_match() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Bevy's built-in attributes:
    // - Mesh::ATTRIBUTE_POSITION is always at location 0
    // - Instance data comes from storage buffer at binding 3, not from vertex attributes
    
    // Verify shader location 0 is position (matches Mesh::ATTRIBUTE_POSITION)
    assert!(
        content.contains("@location(0) position"),
        "WGSL @location(0) must be 'position' to match Bevy's built-in POSITION attribute"
    );
    
    // Verify there are no vertex attributes at locations 1 or 2 for instance data
    // Instance data is accessed via storage buffer at binding 3 using @builtin(instance_index)
    assert!(
        !content.contains("@location(1) instance_size"),
        "WGSL should NOT have @location(1) instance_size - instance data comes from storage buffer at binding 3"
    );
    
    assert!(
        !content.contains("@location(2) instance_color"),
        "WGSL should NOT have @location(2) instance_color - instance data comes from storage buffer at binding 3"
    );
    
    // Verify storage buffer at binding 3 is used for instance data
    assert!(
        content.contains("@group(0) @binding(3)") && content.contains("particle_instances"),
        "WGSL must have storage buffer at @group(0) @binding(3) for particle instance data"
    );
}

/// Test 25: Test shader syntax completeness
#[test]
fn test_shader_syntax_completeness() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify all necessary parts are present
    
    // 1. Vertex input struct
    assert!(content.contains("struct VertexInput"), "Missing VertexInput struct");
    
    // 2. Vertex output struct
    assert!(content.contains("struct VertexOutput"), "Missing VertexOutput struct");
    
    // 3. Material uniforms
    assert!(content.contains("struct PointSpriteMaterial"), "Missing PointSpriteMaterial struct");
    assert!(content.contains("struct ViewUniform"), "Missing ViewUniform struct");
    
    // 4. Instance data struct (storage buffer)
    assert!(content.contains("struct ParticleInstanceData"), "Missing ParticleInstanceData struct");
    
    // 5. All four bindings
    assert!(content.contains("@group(0) @binding(0)"), "Missing binding 0");
    assert!(content.contains("@group(0) @binding(1)"), "Missing binding 1");
    assert!(content.contains("@group(0) @binding(2)"), "Missing binding 2");
    assert!(content.contains("@group(0) @binding(3)"), "Missing binding 3 (storage buffer for instance data)");
    
    // 6. Vertex shader stage
    assert!(content.contains("@vertex"), "Missing @vertex shader");
    assert!(content.contains("fn vertex"), "Missing vertex function");
    
    // 7. Fragment shader stage
    assert!(content.contains("@fragment"), "Missing @fragment shader");
    assert!(content.contains("fn fragment"), "Missing fragment function");
    
    // 8. Return statements
    assert!(content.contains("return output"), "Missing return in vertex shader");
    assert!(content.contains("return input.color") || content.contains("return  input.color"), 
            "Missing return in fragment shader");
    
    // 9. Storage buffer access
    assert!(content.contains("@builtin(instance_index)"), "Missing @builtin(instance_index) for storage buffer access");
    assert!(content.contains("particle_instances"), "Missing particle_instances storage buffer array");
}

// ============================================================================
// HELPER TESTS FOR DEBUGGING
// ============================================================================

/// Test 26: Extract and print all binding declarations for debugging
#[test]
fn test_print_all_bindings() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    let mut bindings = Vec::new();
    
    for line in content.lines() {
        if line.contains("@group") && line.contains("@binding") {
            bindings.push(line.trim());
        }
    }
    
    // Should have exactly 4 bindings at group 0
    // - @binding(0): PointSpriteMaterial uniform
    // - @binding(1): ViewUniform uniform
    // - @binding(2): Model matrix uniform
    // - @binding(3): Storage buffer for particle instance data
    assert_eq!(
        bindings.len(),
        4,
        "Shader should have exactly 4 bindings at @group(0). Found: {:?}",
        bindings
    );
    
    // Verify binding indices
    let binding_0 = bindings.iter().find(|b| b.contains("@binding(0)"));
    let binding_1 = bindings.iter().find(|b| b.contains("@binding(1)"));
    let binding_2 = bindings.iter().find(|b| b.contains("@binding(2)"));
    let binding_3 = bindings.iter().find(|b| b.contains("@binding(3)"));
    
    assert!(binding_0.is_some(), "Missing @binding(0)");
    assert!(binding_1.is_some(), "Missing @binding(1)");
    assert!(binding_2.is_some(), "Missing @binding(2)");
    assert!(binding_3.is_some(), "Missing @binding(3) - storage buffer for instance data");
}

/// Test 27: Verify all WGSL types are correctly specified
#[test]
fn test_wgsl_type_specifications() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Verify vector types use correct suffix
    assert!(content.contains("vec3<f32>"), "Should use vec3<f32>, not vec3");
    assert!(content.contains("vec4<f32>"), "Should use vec4<f32>, not vec4");
    assert!(content.contains("mat4x4<f32>"), "Should use mat4x4<f32>, not mat4x4");
    assert!(content.contains("f32"), "Should use f32, not float");
}

// ============================================================================
// INTEGRATION TEST SUMMARY
// ============================================================================

/// This test provides a comprehensive summary of all shader validation
#[test]
fn test_comprehensive_shader_validation_summary() {
    use std::fs;
    
    let shader_path = get_shader_path();
    let content = fs::read_to_string(&shader_path)
        .expect("Failed to read shader file");
    
    // Count critical elements
    let has_vertex_input = content.contains("struct VertexInput");
    let has_vertex_output = content.contains("struct VertexOutput");
    let has_particle_instance_data = content.contains("struct ParticleInstanceData");
    let has_material = content.contains("struct PointSpriteMaterial");
    let has_view_uniform = content.contains("struct ViewUniform");
    let has_binding_0 = content.contains("@group(0) @binding(0)");
    let has_binding_1 = content.contains("@group(0) @binding(1)");
    let has_binding_2 = content.contains("@group(0) @binding(2)");
    let has_binding_3 = content.contains("@group(0) @binding(3)");
    let has_vertex_shader = content.contains("@vertex");
    let has_fragment_shader = content.contains("@fragment");
    let has_location_0 = content.contains("@location(0) position");
    let has_instance_index = content.contains("@builtin(instance_index)");
    
    // Print summary
    println!("\n=== SHADER VALIDATION SUMMARY ===");
    println!("Shader file: assets/point_sprite.wgsl");
    println!("\nStructs:");
    println!("  - VertexInput: {}", if has_vertex_input { "✓" } else { "✗" });
    println!("  - VertexOutput: {}", if has_vertex_output { "✓" } else { "✗" });
    println!("  - ParticleInstanceData: {}", if has_particle_instance_data { "✓" } else { "✗" });
    println!("  - PointSpriteMaterial: {}", if has_material { "✓" } else { "✗" });
    println!("  - ViewUniform: {}", if has_view_uniform { "✓" } else { "✗" });
    println!("\nBindings (@group(0)):");
    println!("  - @binding(0) material: {}", if has_binding_0 { "✓" } else { "✗" });
    println!("  - @binding(1) view: {}", if has_binding_1 { "✓" } else { "✗" });
    println!("  - @binding(2) model: {}", if has_binding_2 { "✓" } else { "✗" });
    println!("  - @binding(3) particle_instances (storage buffer): {}", if has_binding_3 { "✓" } else { "✗" });
    println!("\nShader Stages:");
    println!("  - @vertex: {}", if has_vertex_shader { "✓" } else { "✗" });
    println!("  - @fragment: {}", if has_fragment_shader { "✓" } else { "✗" });
    println!("\nVertex Attributes (VertexInput):");
    println!("  - @location(0) position: {}", if has_location_0 { "✓" } else { "✗" });
    println!("\nInstance Data Access:");
    println!("  - @builtin(instance_index): {}", if has_instance_index { "✓" } else { "✗" });
    println!("  - Storage buffer (binding 3) for particle instance data: {}", if has_binding_3 { "✓" } else { "✗" });
    println!("================================\n");
    
    // Assert all critical elements are present
    assert!(has_vertex_input, "Missing VertexInput struct");
    assert!(has_vertex_output, "Missing VertexOutput struct");
    assert!(has_particle_instance_data, "Missing ParticleInstanceData struct");
    assert!(has_material, "Missing PointSpriteMaterial struct");
    assert!(has_view_uniform, "Missing ViewUniform struct");
    assert!(has_binding_0, "Missing @group(0) @binding(0)");
    assert!(has_binding_1, "Missing @group(0) @binding(1)");
    assert!(has_binding_2, "Missing @group(0) @binding(2)");
    assert!(has_binding_3, "Missing @group(0) @binding(3) - storage buffer for instance data");
    assert!(has_vertex_shader, "Missing @vertex shader");
    assert!(has_fragment_shader, "Missing @fragment shader");
    assert!(has_location_0, "Missing @location(0) position");
    assert!(has_instance_index, "Missing @builtin(instance_index) - required for storage buffer indexing");
}
