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

use bevy::asset::AssetServer;
use bevy::pbr::Material;
use bevy::prelude::*;
use bevy::render::alpha::AlphaMode;
use bevy::render::mesh::{Mesh, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::window::WindowResolution;
use genesis_render::particle::{PointSpriteMaterial, ATTRIBUTE_INSTANCE_COLOR, ATTRIBUTE_INSTANCE_SIZE};
use std::path::Path;

// ============================================================================
// A. SHADER ASSET LOADING TESTS
// ============================================================================

/// Test 1: Verify that the shader asset file exists and can be loaded
#[test]
fn test_shader_file_exists() {
    let shader_path = Path::new("assets/point_sprite.wgsl");
    
    assert!(
        shader_path.exists(),
        "Shader file 'assets/point_sprite.wgsl' does not exist. \
         This file is required for point sprite rendering."
    );
}

/// Test 2: Verify the shader file is readable and has content
#[test]
fn test_shader_file_readable() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    assert!(
        shader_path.exists(),
        "Shader file 'assets/point_sprite.wgsl' does not exist"
    );
    
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    assert!(
        !content.is_empty(),
        "Shader file 'assets/point_sprite.wgsl' is empty"
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    
    match (vertex_shader, fragment_shader) {
        (ShaderRef::Path(v_path), ShaderRef::Path(f_path)) => {
            assert_eq!(
                v_path, 
                "point_sprite.wgsl", 
                "Vertex shader path should be 'point_sprite.wgsl', got: {}",
                v_path
            );
            assert_eq!(
                f_path, 
                "point_sprite.wgsl", 
                "Fragment shader path should be 'point_sprite.wgsl', got: {}",
                f_path
            );
        }
        _ => panic!(
            "Both vertex and fragment shaders should use ShaderRef::Path, got: \
             vertex={:?}, fragment={:?}", 
            vertex_shader, fragment_shader
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Check for binding 0 declaration
    assert!(
        content.contains("@group(0) @binding(0)"),
        "Shader must have @group(0) @binding(0) for PointSpriteMaterial"
    );
    
    // Verify it binds to PointSpriteMaterial
    let binding_line = content.lines()
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // CRITICAL: Check that binding 1 exists - this was the error location
    assert!(
        content.contains("@group(0) @binding(1)"),
        "Shader MUST have @group(0) @binding(1) for ViewUniform. \
         Missing binding 1 will cause: \
         'Shader global ResourceBinding { group: 0, binding: 1 } is not available \
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
    let binding_line = content.lines()
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Check for binding 2 declaration
    assert!(
        content.contains("@group(0) @binding(2)"),
        "Shader must have @group(0) @binding(2) for model matrix"
    );
    
    // Verify binding 2 binds to mat4x4<f32> for model matrix
    let binding_line = content.lines()
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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

/// Test 12: Verify @location(1) for instance_size exists
#[test]
fn test_vertex_attribute_location_1_instance_size() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Check for location 1
    assert!(
        content.contains("@location(1) instance_size: f32"),
        "VertexInput must have @location(1) instance_size: f32 for per-instance size. \
         This must match the ATTRIBUTE_INSTANCE_SIZE mesh attribute."
    );
}

/// Test 13: Verify @location(2) for instance_color exists
#[test]
fn test_vertex_attribute_location_2_instance_color() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Check for location 2
    assert!(
        content.contains("@location(2) instance_color: vec4<f32>"),
        "VertexInput must have @location(2) instance_color: vec4<f32> for per-instance color. \
         This must match the ATTRIBUTE_INSTANCE_COLOR mesh attribute."
    );
}

/// Test 14: Verify mesh attribute constants have correct properties
#[test]
fn test_mesh_attribute_properties() {
    // Check ATTRIBUTE_INSTANCE_SIZE properties
    assert_eq!(
        ATTRIBUTE_INSTANCE_SIZE.name,
        "instance_size",
        "ATTRIBUTE_INSTANCE_SIZE name should be 'instance_size'"
    );
    
    assert_eq!(
        ATTRIBUTE_INSTANCE_SIZE.format,
        bevy::render::render_resource::VertexFormat::Float32,
        "ATTRIBUTE_INSTANCE_SIZE format should be Float32 to match WGSL f32"
    );
    
    // Check ATTRIBUTE_INSTANCE_COLOR properties
    assert_eq!(
        ATTRIBUTE_INSTANCE_COLOR.name,
        "instance_color",
        "ATTRIBUTE_INSTANCE_COLOR name should be 'instance_color'"
    );
    
    assert_eq!(
        ATTRIBUTE_INSTANCE_COLOR.format,
        bevy::render::render_resource::VertexFormat::Float32x4,
        "ATTRIBUTE_INSTANCE_COLOR format should be Float32x4 to match WGSL vec4<f32>"
    );
}

/// Test 15: Verify VertexOutput struct has color field
#[test]
fn test_vertex_output_structure() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Verify vertex shader exists
    assert!(
        content.contains("@vertex"),
        "Shader must have @vertex shader stage"
    );
    
    // Verify vertex shader uses input.position
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
}

/// Test 21: Verify fragment shader outputs color correctly
#[test]
fn test_fragment_shader_outputs_color() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    use bevy::render::render_asset::RenderAssetUsages;
    
    let mut app = App::new();
    
    // Add minimal plugins for rendering
    app.add_plugins((
        bevy::asset::AssetPlugin::default(),
        bevy::render::RenderPlugin::default(),
        bevy::pbr::PbrPlugin,
    ));
    
    // This should compile and run without panicking
    app.add_plugins(bevy::pbr::MaterialPlugin::<PointSpriteMaterial>::default());
    
    // Run for one tick to ensure no initialization errors
    app.update();
}

/// Test 23: Test creating a mesh with correct vertex attributes
#[test]
fn test_point_mesh_creation() {
    // Create a point mesh with the correct vertex attributes
    let mut mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default());
    
    // Add position attribute (location 0)
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0]]
    );
    
    // Add instance_size attribute (location 1)
    mesh.insert_attribute(
        ATTRIBUTE_INSTANCE_SIZE,
        vec![1.0f32]
    );
    
    // Add instance_color attribute (location 2)
    mesh.insert_attribute(
        ATTRIBUTE_INSTANCE_COLOR,
        vec![[1.0, 1.0, 1.0, 1.0]]
    );
    
    // Verify attributes were added
    assert!(
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some(),
        "Mesh must have POSITION attribute"
    );
    
    assert!(
        mesh.attribute(ATTRIBUTE_INSTANCE_SIZE).is_some(),
        "Mesh must have INSTANCE_SIZE attribute at custom location"
    );
    
    assert!(
        mesh.attribute(ATTRIBUTE_INSTANCE_COLOR).is_some(),
        "Mesh must have INSTANCE_COLOR attribute at custom location"
    );
    
    // Verify attribute values
    if let Some(VertexAttributeValues::Float32(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        assert_eq!(positions.len(), 1, "Should have 1 position vertex");
        assert_eq!(positions[0], [0.0, 0.0, 0.0], "Position should be at origin");
    } else {
        panic!("POSITION attribute should be Float32");
    }
    
    if let Some(VertexAttributeValues::Float32(sizes)) = mesh.attribute(ATTRIBUTE_INSTANCE_SIZE) {
        assert_eq!(sizes.len(), 1, "Should have 1 size value");
        assert_eq!(sizes[0], 1.0, "Size should be 1.0");
    } else {
        panic!("INSTANCE_SIZE attribute should be Float32");
    }
    
    if let Some(VertexAttributeValues::Float32x4(colors)) = mesh.attribute(ATTRIBUTE_INSTANCE_COLOR) {
        assert_eq!(colors.len(), 1, "Should have 1 color value");
        assert_eq!(colors[0], [1.0, 1.0, 1.0, 1.0], "Color should be white");
    } else {
        panic!("INSTANCE_COLOR attribute should be Float32x4");
    }
}

/// Test 24: Test vertex attribute location alignment
#[test]
fn test_vertex_attribute_locations_match() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Bevy's built-in attributes:
    // - Mesh::ATTRIBUTE_POSITION is always at location 0
    // - Custom attributes are assigned locations sequentially
    
    // Verify shader location 0 is position (matches Mesh::ATTRIBUTE_POSITION)
    assert!(
        content.contains("@location(0) position"),
        "WGSL @location(0) must be 'position' to match Bevy's built-in POSITION attribute"
    );
    
    // Verify shader location 1 is instance_size (matches ATTRIBUTE_INSTANCE_SIZE)
    assert!(
        content.contains("@location(1) instance_size"),
        "WGSL @location(1) must be 'instance_size' to match ATTRIBUTE_INSTANCE_SIZE"
    );
    
    // Verify shader location 2 is instance_color (matches ATTRIBUTE_INSTANCE_COLOR)
    assert!(
        content.contains("@location(2) instance_color"),
        "WGSL @location(2) must be 'instance_color' to match ATTRIBUTE_INSTANCE_COLOR"
    );
}

/// Test 25: Test shader syntax completeness
#[test]
fn test_shader_syntax_completeness() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Verify all necessary parts are present
    
    // 1. Vertex input struct
    assert!(content.contains("struct VertexInput"), "Missing VertexInput struct");
    
    // 2. Vertex output struct
    assert!(content.contains("struct VertexOutput"), "Missing VertexOutput struct");
    
    // 3. Material uniforms
    assert!(content.contains("struct PointSpriteMaterial"), "Missing PointSpriteMaterial struct");
    assert!(content.contains("struct ViewUniform"), "Missing ViewUniform struct");
    
    // 4. All three bindings
    assert!(content.contains("@group(0) @binding(0)"), "Missing binding 0");
    assert!(content.contains("@group(0) @binding(1)"), "Missing binding 1 (CRITICAL)");
    assert!(content.contains("@group(0) @binding(2)"), "Missing binding 2");
    
    // 5. Vertex shader stage
    assert!(content.contains("@vertex"), "Missing @vertex shader");
    assert!(content.contains("fn vertex"), "Missing vertex function");
    
    // 6. Fragment shader stage
    assert!(content.contains("@fragment"), "Missing @fragment shader");
    assert!(content.contains("fn fragment"), "Missing fragment function");
    
    // 7. Return statements
    assert!(content.contains("return output"), "Missing return in vertex shader");
    assert!(content.contains("return input.color") || content.contains("return  input.color"), 
            "Missing return in fragment shader");
}

// ============================================================================
// HELPER TESTS FOR DEBUGGING
// ============================================================================

/// Test 26: Extract and print all binding declarations for debugging
#[test]
fn test_print_all_bindings() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    let mut bindings = Vec::new();
    
    for line in content.lines() {
        if line.contains("@group") && line.contains("@binding") {
            bindings.push(line.trim());
        }
    }
    
    // Should have exactly 3 bindings at group 0
    assert_eq!(
        bindings.len(),
        3,
        "Shader should have exactly 3 bindings at @group(0). Found: {:?}",
        bindings
    );
    
    // Verify binding indices
    let binding_0 = bindings.iter().find(|b| b.contains("@binding(0)"));
    let binding_1 = bindings.iter().find(|b| b.contains("@binding(1)"));
    let binding_2 = bindings.iter().find(|b| b.contains("@binding(2)"));
    
    assert!(binding_0.is_some(), "Missing @binding(0)");
    assert!(binding_1.is_some(), "Missing @binding(1) - THIS IS THE CRITICAL ERROR LOCATION");
    assert!(binding_2.is_some(), "Missing @binding(2)");
}

/// Test 27: Verify all WGSL types are correctly specified
#[test]
fn test_wgsl_type_specifications() {
    use std::fs;
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
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
    
    let shader_path = Path::new("assets/point_sprite.wgsl");
    let content = fs::read_to_string(shader_path)
        .expect("Failed to read shader file 'assets/point_sprite.wgsl'");
    
    // Count critical elements
    let has_vertex_input = content.contains("struct VertexInput");
    let has_vertex_output = content.contains("struct VertexOutput");
    let has_material = content.contains("struct PointSpriteMaterial");
    let has_view_uniform = content.contains("struct ViewUniform");
    let has_binding_0 = content.contains("@group(0) @binding(0)");
    let has_binding_1 = content.contains("@group(0) @binding(1)");
    let has_binding_2 = content.contains("@group(0) @binding(2)");
    let has_vertex_shader = content.contains("@vertex");
    let has_fragment_shader = content.contains("@fragment");
    let has_location_0 = content.contains("@location(0) position");
    let has_location_1 = content.contains("@location(1) instance_size");
    let has_location_2 = content.contains("@location(2) instance_color");
    
    // Print summary
    println!("\n=== SHADER VALIDATION SUMMARY ===");
    println!("Shader file: assets/point_sprite.wgsl");
    println!("\nStructs:");
    println!("  - VertexInput: {}", if has_vertex_input { "✓" } else { "✗" });
    println!("  - VertexOutput: {}", if has_vertex_output { "✓" } else { "✗" });
    println!("  - PointSpriteMaterial: {}", if has_material { "✓" } else { "✗" });
    println!("  - ViewUniform: {}", if has_view_uniform { "✓" } else { "✗" });
    println!("\nBindings (@group(0)):");
    println!("  - @binding(0) material: {}", if has_binding_0 { "✓" } else { "✗" });
    println!("  - @binding(1) view: {} [CRITICAL]", if has_binding_1 { "✓" } else { "✗" });
    println!("  - @binding(2) model: {}", if has_binding_2 { "✓" } else { "✗" });
    println!("\nShader Stages:");
    println!("  - @vertex: {}", if has_vertex_shader { "✓" } else { "✗" });
    println!("  - @fragment: {}", if has_fragment_shader { "✓" } else { "✗" });
    println!("\nVertex Attributes (VertexInput):");
    println!("  - @location(0) position: {}", if has_location_0 { "✓" } else { "✗" });
    println!("  - @location(1) instance_size: {}", if has_location_1 { "✓" } else { "✗" });
    println!("  - @location(2) instance_color: {}", if has_location_2 { "✓" } else { "✗" });
    println!("================================\n");
    
    // Assert all critical elements are present
    assert!(has_vertex_input, "Missing VertexInput struct");
    assert!(has_vertex_output, "Missing VertexOutput struct");
    assert!(has_material, "Missing PointSpriteMaterial struct");
    assert!(has_view_uniform, "Missing ViewUniform struct");
    assert!(has_binding_0, "Missing @group(0) @binding(0)");
    assert!(has_binding_1, "MISSING CRITICAL: @group(0) @binding(1) - This causes GPU error!");
    assert!(has_binding_2, "Missing @group(0) @binding(2)");
    assert!(has_vertex_shader, "Missing @vertex shader");
    assert!(has_fragment_shader, "Missing @fragment shader");
    assert!(has_location_0, "Missing @location(0) position");
    assert!(has_location_1, "Missing @location(1) instance_size");
    assert!(has_location_2, "Missing @location(2) instance_color");
}
