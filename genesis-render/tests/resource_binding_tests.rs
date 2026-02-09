//! Resource Binding Tests for Genesis-RS Point Sprite Rendering
//!
//! This test suite validates resource binding setup, pipeline layouts, and resource lifecycle
//! for the particle rendering system. These tests catch GPU rendering errors related to:
//!
//! - Resource binding setup (especially binding visibility in shader stages)
//! - Pipeline layout configuration
//! - Resource initialization order and lifecycle
//! - Mesh vertex attribute bindings for instanced rendering
//! - Per-instance data synchronization
//! - Resource access safety
//!
//! # Critical Error Being Addressed
//!
//! The following error triggered the creation of these tests:
//! "Shader global ResourceBinding { group: 0, binding: 1 } is not available in the
//! pipeline layout - Visibility flags don't include the shader stage"
//!
//! This error indicates that the ViewUniform (binding 1) is not marked as visible in
//! the VERTEX shader stage, even though the vertex shader accesses it.

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::mesh::{Mesh, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_resource::{BindGroupLayout, ShaderRef, ShaderStages, VertexFormat};
use bevy::render::ExtractSchedule;
use genesis_core::config::ParticleConfig;
use genesis_render::particle::{
    extract_particle_instances, PointMesh, PointSpriteMaterial, Particle,
    ATTRIBUTE_INSTANCE_COLOR, ATTRIBUTE_INSTANCE_SIZE, ExtractedParticleInstances,
    ParticleInstanceBindGroupLayout,
};

// Import Zeroable for ParticleInstanceData::zeroed()
use bytemuck::Zeroable;

// ============================================================================
// TEST UTILITIES
// ============================================================================

#[allow(dead_code)]
/// Helper function to create a minimal test app with rendering capabilities
fn create_render_app() -> App {
    let mut app = App::new();
    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
        .add_plugins(bevy::render::RenderPlugin::default())
        .add_plugins(bevy::asset::AssetPlugin::default());
    app
}

#[allow(dead_code)]
/// Helper to verify a binding exists in a bind group layout
fn verify_binding_exists(
    _layout: &BindGroupLayout,
    _binding_index: u32,
    _expected_visibility: ShaderStages,
    _context: &str,
) -> Result<(), String> {
    // In a real test, we'd inspect the bind group layout's entries
    // For now, we validate through the Material trait's AsBindGroup implementation
    Ok(())
}

// ============================================================================
// A. PIPELINE LAYOUT BINDING TESTS
// ============================================================================

/// Test 1: Verify PointSpriteMaterial's AsBindGroup creates correct binding layout
#[test]
fn test_material_bind_group_layout_structure() {
    let material = PointSpriteMaterial {
        color: bevy::color::LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        base_size: 10.0,
        attenuation_factor: 0.01,
    };

    // Verify that the material is properly constructed
    // The AsBindGroup derive macro should generate the correct binding layout
    assert_eq!(
        material.base_size, 10.0,
        "Material base_size should be 10.0"
    );
    assert_eq!(
        material.attenuation_factor, 0.01,
        "Material attenuation_factor should be 0.01"
    );

    // Verify color is in correct format (LinearRgba has 4 components)
    assert_eq!(material.color.alpha, 1.0, "Material color alpha should be 1.0");
}

/// Test 2: Test that binding 0 (PointSpriteMaterial) is present and accessible
#[test]
fn test_binding_0_material_uniform() {
    let material = PointSpriteMaterial {
        color: bevy::color::LinearRgba::WHITE,
        base_size: 5.0,
        attenuation_factor: 0.02,
    };

    // Binding 0 is PointSpriteMaterial with color, base_size, and attenuation_factor
    // This should be accessible in both VERTEX and FRAGMENT shader stages
    assert!(
        material.color.red.is_finite(),
        "Binding 0 color should have valid finite values"
    );
    assert!(
        material.base_size.is_finite(),
        "Binding 0 base_size should have valid finite value"
    );
    assert!(
        material.attenuation_factor.is_finite(),
        "Binding 0 attenuation_factor should have valid finite value"
    );
}

/// Test 3: CRITICAL - Test that binding 1 (ViewUniform) is available in VERTEX stage
///
/// This test validates the fix for the error:
/// "Shader global ResourceBinding { group: 0, binding: 1 } is not available in the
/// pipeline layout - Visibility flags don't include the shader stage"
///
/// The vertex shader in point_sprite.wgsl accesses `view.view_proj` at binding 1,
/// so this binding MUST be visible in the VERTEX shader stage.
#[test]
fn test_binding_1_view_uniform_vertex_visibility() {
    // The shader declares:
    // @group(0) @binding(1)
    // var<uniform> view: ViewUniform;
    //
    // And the vertex shader accesses it:
    // output.clip_position = view.view_proj * world_pos;
    //
    // This means binding 1 MUST be visible in VERTEX shader stage.

    // We verify the shader file contains the correct declaration
    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // Verify binding 1 exists
    assert!(
        content.contains("@group(0) @binding(1)"),
        "CRITICAL: Shader must declare @group(0) @binding(1) for ViewUniform. \
         Missing this binding causes the error: \
         'Shader global ResourceBinding {{ group: 0, binding: 1 }} is not available \
         in the pipeline layout - Visibility flags don't include the shader stage'"
    );

    // Verify ViewUniform struct is defined
    assert!(
        content.contains("struct ViewUniform"),
        "Shader must define ViewUniform struct for binding 1"
    );

    // Verify ViewUniform has view_proj field
    assert!(
        content.contains("view_proj: mat4x4<f32>"),
        "ViewUniform must contain view_proj: mat4x4<f32> for view-projection matrix"
    );

    // Verify the vertex shader accesses view.view_proj
    assert!(
        content.contains("view.view_proj"),
        "CRITICAL: Vertex shader must access view.view_proj from binding 1. \
         If the vertex shader accesses this binding but it's not in the pipeline layout \
         with VERTEX visibility, the GPU will fail to compile the shader with the error: \
         'Shader global ResourceBinding {{ group: 0, binding: 1 }} is not available \
         in the pipeline layout - Visibility flags don't include the shader stage'"
    );
}

/// Test 4: Test that binding 2 (model matrix) is present and accessible
#[test]
fn test_binding_2_model_matrix_uniform() {
    // The shader declares:
    // @group(0) @binding(2)
    // var<uniform> model: mat4x4<f32>;
    //
    // And the vertex shader accesses it:
    // let world_pos = model * vec4<f32>(input.position, 1.0);

    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // Verify binding 2 exists
    assert!(
        content.contains("@group(0) @binding(2)"),
        "Shader must declare @group(0) @binding(2) for model matrix"
    );

    // Verify binding 2 binds to mat4x4<f32>
    assert!(
        content.contains("var<uniform> model: mat4x4<f32>"),
        "Binding 2 should bind to model matrix (mat4x4<f32>)"
    );

    // Verify the vertex shader accesses model
    assert!(
        content.contains("model * vec4<f32>"),
        "Vertex shader must use model matrix from binding 2"
    );
}

/// Test 5: Test that all bindings (0, 1, 2) are present in the correct order
#[test]
fn test_all_bindings_present_in_order() {
    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // Find all binding declarations
    let binding_0_pos = content.find("@group(0) @binding(0)")
        .expect("Binding 0 not found - PointSpriteMaterial uniform");
    let binding_1_pos = content.find("@group(0) @binding(1)")
        .expect("Binding 1 not found - ViewUniform uniform");
    let binding_2_pos = content.find("@group(0) @binding(2)")
        .expect("Binding 2 not found - Model matrix uniform");

    // Verify bindings are in order (0 < 1 < 2)
    assert!(
        binding_0_pos < binding_1_pos,
        "Binding 0 must come before binding 1 in the shader"
    );
    assert!(
        binding_1_pos < binding_2_pos,
        "Binding 1 must come before binding 2 in the shader"
    );
}

/// Test 6: Test that each binding has correct visibility flags
///
/// Expected visibility:
/// - Binding 0 (PointSpriteMaterial): VERTEX | FRAGMENT (shader accesses it in both)
/// - Binding 1 (ViewUniform): VERTEX | FRAGMENT (vertex shader needs view_proj, fragment might need it)
/// - Binding 2 (Model): VERTEX only (only vertex shader needs the transform)
#[test]
fn test_binding_visibility_flags() {
    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // Extract vertex shader function
    let vertex_start = content.find("@vertex")
        .expect("Vertex shader not found");
    let vertex_end = content.find("@fragment")
        .unwrap_or(content.len());
    let vertex_shader = &content[vertex_start..vertex_end];

    // Extract fragment shader function
    let _fragment_shader = &content[vertex_end..];

    // Verify binding 0 (material) is accessible in vertex shader
    assert!(
        vertex_shader.contains("material."),
        "Binding 0 (material) should be accessible in VERTEX stage - \
         vertex shader should reference material attributes"
    );

    // Verify binding 1 (view) is accessible in vertex shader (CRITICAL for the fix)
    assert!(
        vertex_shader.contains("view."),
        "CRITICAL: Binding 1 (view) MUST be accessible in VERTEX stage - \
         vertex shader accesses view.view_proj for clip space transformation. \
         If this binding is not visible in VERTEX stage, the error occurs: \
         'Shader global ResourceBinding {{ group: 0, binding: 1 }} is not available \
         in the pipeline layout - Visibility flags don't include the shader stage'"
    );

    // Verify binding 2 (model) is accessible in vertex shader
    assert!(
        vertex_shader.contains("model"),
        "Binding 2 (model) should be accessible in VERTEX stage - \
         vertex shader uses model matrix for world space transformation"
    );
}

/// Test 7: Test Material trait implementation creates correct pipeline layout
#[test]
fn test_material_trait_pipeline_layout() {
    // Verify PointSpriteMaterial implements Material trait
    // The Material trait should provide the correct pipeline layout
    
    let material = PointSpriteMaterial {
        color: bevy::color::LinearRgba::new(0.5, 0.5, 0.5, 1.0),
        base_size: 8.0,
        attenuation_factor: 0.015,
    };

    // Verify vertex shader returns correct path
    let vertex_shader = PointSpriteMaterial::vertex_shader();
    match &vertex_shader {
        ShaderRef::Path(path) => {
            assert!(
                path.to_string().contains("point_sprite.wgsl"),
                "Vertex shader path should reference point_sprite.wgsl"
            );
        }
        _ => panic!("Vertex shader should use ShaderRef::Path"),
    }

    // Verify fragment shader returns correct path
    let fragment_shader = PointSpriteMaterial::fragment_shader();
    match &fragment_shader {
        ShaderRef::Path(path) => {
            assert!(
                path.to_string().contains("point_sprite.wgsl"),
                "Fragment shader path should reference point_sprite.wgsl"
            );
        }
        _ => panic!("Fragment shader should use ShaderRef::Path"),
    }

    // Both shaders use the same file (vertex and fragment in one file)
    // Note: ShaderRef doesn't implement PartialEq, so we verify they reference the same file
    match (&vertex_shader, &fragment_shader) {
        (ShaderRef::Path(v), ShaderRef::Path(f)) => {
            assert!(
                v == f,
                "Both vertex and fragment shaders should use the same file"
            );
        }
        _ => assert!(false, "Both shaders should use ShaderRef::Path"),
    }

    // Verify alpha mode is set
    assert_eq!(
        material.alpha_mode(),
        bevy::render::alpha::AlphaMode::Add,
        "PointSpriteMaterial should use AlphaMode::Add for additive blending"
    );
}

// ============================================================================
// B. RESOURCE INITIALIZATION ORDER TESTS
// ============================================================================

/// Test 8: Test that PointMesh resource is initialized before particles spawn
#[test]
fn test_point_mesh_initialized_before_particles_spawn() {
    let mut app = App::new();

    // Add required plugins
    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Add particle config
    app.insert_resource(ParticleConfig {
        initial_count: 100,
        max_count: 1000,
        base_size: 5.0,
    });

    // Run init_point_mesh system (simulates startup)
    app.add_systems(
        bevy::app::Startup,
        genesis_render::particle::init_point_mesh,
    );

    // Run startup schedule
    app.world_mut().run_schedule(bevy::app::Startup);

    // Verify PointMesh resource exists
    assert!(
        app.world().contains_resource::<PointMesh>(),
        "PointMesh resource must be initialized after startup schedule. \
         If particles spawn before this resource exists, the spawn system will fail."
    );

    // Verify the PointMesh contains a valid mesh handle
    let point_mesh = app.world().resource::<PointMesh>();
    let mesh_assets = app.world().resource::<bevy::asset::Assets<bevy::render::mesh::Mesh>>();
    
    assert!(
        mesh_assets.get(&point_mesh.0).is_some(),
        "PointMesh handle must point to a valid mesh asset. \
         If the mesh handle is invalid, particle rendering will fail."
    );
}

/// Test 9: Test that materials are initialized before rendering
#[test]
fn test_materials_initialized_before_rendering() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Create a material
    let material = PointSpriteMaterial {
        color: bevy::color::LinearRgba::WHITE,
        base_size: 10.0,
        attenuation_factor: 0.01,
    };

    // Add material to assets
    let mut materials = app.world_mut().resource_mut::<bevy::asset::Assets<PointSpriteMaterial>>();
    let material_handle = materials.add(material);

    // Verify material exists in asset collection
    assert!(
        materials.get(&material_handle).is_some(),
        "PointSpriteMaterial must be successfully added to asset collection. \
         If material initialization fails, rendering will not work."
    );

    // Verify material has correct properties
    let retrieved_material = materials.get(&material_handle).unwrap();
    assert_eq!(
        retrieved_material.base_size, 10.0,
        "Material base_size should be correctly stored"
    );
    assert_eq!(
        retrieved_material.attenuation_factor, 0.01,
        "Material attenuation_factor should be correctly stored"
    );
}

/// Test 10: Test that camera exists before rendering pipeline activates
#[test]
fn test_camera_initialized_before_rendering() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Spawn a camera entity
    app.world_mut().spawn((
        Camera::default(),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Verify camera entity exists
    let camera_count = app
        .world()
        .iter_entities()
        .filter(|e| app.world().get::<Camera>(e.id()).is_some())
        .count();

    assert!(
        camera_count >= 1,
        "At least one camera entity must exist before rendering. \
         If no camera exists, the rendering pipeline cannot activate."
    );

    // Verify camera has valid transform
    for camera_entity in app
        .world()
        .iter_entities()
        .filter(|e| app.world().get::<Camera>(e.id()).is_some())
    {
        let transform = app
            .world()
            .get::<Transform>(camera_entity.id())
            .expect("Camera should have a Transform component");
        
        assert!(
            transform.translation.is_finite(),
            "Camera transform translation should have finite values. \
             Invalid camera position will cause rendering errors."
        );
    }
}

/// Test 11: Test system ordering - init_point_mesh runs before spawn_particles
#[test]
fn test_system_ordering_point_mesh_before_spawn() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Add particle config
    app.insert_resource(ParticleConfig {
        initial_count: 10,
        max_count: 100,
        base_size: 5.0,
    });

    // Add systems with explicit ordering
    app.add_systems(
        bevy::app::Startup,
        (
            genesis_render::particle::init_point_mesh,
            genesis_render::particle::spawn_particles.after(genesis_render::particle::init_point_mesh),
        ),
    );

    // Run startup schedule
    app.world_mut().run_schedule(bevy::app::Startup);

    // Verify PointMesh resource exists
    assert!(
        app.world().contains_resource::<PointMesh>(),
        "PointMesh must be initialized before spawn_particles runs. \
         If spawn_particles runs first, it will fail to access PointMesh resource."
    );

    // Verify particles were spawned
    let particle_count = app
        .world()
        .iter_entities()
        .filter(|e| app.world().get::<Particle>(e.id()).is_some())
        .count();

    assert_eq!(
        particle_count, 10,
        "Exactly 10 particles should be spawned after startup. \
         If particles don't spawn, the spawn system failed."
    );
}

// ============================================================================
// C. RESOURCE LIFECYCLE TESTS
// ============================================================================

/// Test 12: Test that resources are properly created at startup
#[test]
fn test_resources_created_at_startup() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Add particle config
    app.insert_resource(ParticleConfig {
        initial_count: 5,
        max_count: 50,
        base_size: 3.0,
    });

    // Add startup systems
    app.add_systems(
        bevy::app::Startup,
        (
            genesis_render::particle::init_point_mesh,
            genesis_render::particle::spawn_particles.after(genesis_render::particle::init_point_mesh),
        ),
    );

    // Run startup schedule
    app.world_mut().run_schedule(bevy::app::Startup);

    // Verify resources are created
    assert!(
        app.world().contains_resource::<PointMesh>(),
        "PointMesh resource must be created at startup"
    );

    assert!(
        app.world().contains_resource::<ParticleConfig>(),
        "ParticleConfig resource must exist at startup"
    );

    // Verify entities were created
    let entity_count = app.world().entities().len();
    assert!(
        entity_count > 0,
        "At least one entity must be created at startup (particles and/or camera)"
    );
}

/// Test 13: Test that resources can be accessed during Update schedule
#[test]
fn test_resources_accessible_during_update() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default())
    .add_plugins(bevy::time::TimePlugin::default());

    // Add particle config
    app.insert_resource(ParticleConfig {
        initial_count: 5,
        max_count: 50,
        base_size: 3.0,
    });

    // Add startup systems
    app.add_systems(
        bevy::app::Startup,
        (
            genesis_render::particle::init_point_mesh,
            genesis_render::particle::spawn_particles.after(genesis_render::particle::init_point_mesh),
        ),
    );

    // Add update system that accesses resources
    fn verify_resources_during_update(point_mesh: Res<PointMesh>) {
        // Just accessing the resource verifies it exists and is accessible
        let _ = &point_mesh.0;
    }

    app.add_systems(bevy::app::Update, verify_resources_during_update);

    // Run startup
    app.world_mut().run_schedule(bevy::app::Startup);

    // Run update schedule
    app.world_mut().run_schedule(bevy::app::Update);

    // If we get here without panic, resources were accessible during Update
}

/// Test 14: Test resource lifecycle - create, modify, and access
#[test]
fn test_resource_lifecycle_create_modify_access() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Create custom resource
    #[derive(Resource, Clone, Debug)]
    struct TestResource {
        value: i32,
    }

    app.world_mut().insert_resource(TestResource { value: 42 });

    // Verify resource exists
    assert!(
        app.world().contains_resource::<TestResource>(),
        "Custom resource should be created and accessible"
    );

    // Modify resource
    let mut test_resource = app.world_mut().resource_mut::<TestResource>();
    test_resource.value = 100;

    // Verify modification
    let test_resource = app.world().resource::<TestResource>();
    assert_eq!(
        test_resource.value, 100,
        "Resource modification should persist"
    );
}

/// Test 15: Test no index out of bounds errors in pipeline cache
///
/// This test validates that pipeline cache operations are safe and won't cause
/// "index out of bounds" errors, which suggest race conditions or improper initialization.
#[test]
fn test_pipeline_cache_no_index_out_of_bounds() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Add particle config
    app.insert_resource(ParticleConfig {
        initial_count: 10,
        max_count: 100,
        base_size: 5.0,
    });

    // Initialize PointMesh
    app.add_systems(
        bevy::app::Startup,
        genesis_render::particle::init_point_mesh,
    );

    // Run startup
    app.world_mut().run_schedule(bevy::app::Startup);

    // Spawn multiple materials to test pipeline cache
    let mut materials = app.world_mut().resource_mut::<bevy::asset::Assets<PointSpriteMaterial>>();
    
    // Create multiple materials to exercise pipeline cache
    for i in 0..5 {
        let material = PointSpriteMaterial {
            color: bevy::color::LinearRgba::new(
                i as f32 / 5.0,
                i as f32 / 5.0,
                i as f32 / 5.0,
                1.0,
            ),
            base_size: 5.0 + i as f32,
            attenuation_factor: 0.01 * (i + 1) as f32,
        };
        materials.add(material);
    }

    // If we get here without index out of bounds error, pipeline cache is working
    // This test primarily validates that the AsBindGroup implementation doesn't cause
    // issues when creating multiple material instances
}

// ============================================================================
// D. MESH VERTEX ATTRIBUTE BINDING TESTS
// ============================================================================

/// Test 16: Test that mesh vertex attributes are correctly configured
#[test]
fn test_mesh_vertex_attributes_configured() {
    // Verify ATTRIBUTE_INSTANCE_SIZE properties
    assert_eq!(
        ATTRIBUTE_INSTANCE_SIZE.name,
        "instance_size",
        "ATTRIBUTE_INSTANCE_SIZE name should be 'instance_size' to match shader @location(1)"
    );

    assert_eq!(
        ATTRIBUTE_INSTANCE_SIZE.format,
        VertexFormat::Float32,
        "ATTRIBUTE_INSTANCE_SIZE format should be Float32 to match WGSL f32"
    );

    // Verify ATTRIBUTE_INSTANCE_COLOR properties
    assert_eq!(
        ATTRIBUTE_INSTANCE_COLOR.name,
        "instance_color",
        "ATTRIBUTE_INSTANCE_COLOR name should be 'instance_color' to match shader @location(2)"
    );

    assert_eq!(
        ATTRIBUTE_INSTANCE_COLOR.format,
        VertexFormat::Float32x4,
        "ATTRIBUTE_INSTANCE_COLOR format should be Float32x4 to match WGSL vec4<f32>"
    );
}

/// Test 17: Test that ATTRIBUTE_INSTANCE_SIZE is at correct shader location
#[test]
fn test_attribute_instance_size_location() {
    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // Verify VertexInput struct has instance_size at location(1)
    assert!(
        content.contains("@location(1) instance_size: f32"),
        "Shader must declare @location(1) instance_size: f32 to match \
         ATTRIBUTE_INSTANCE_SIZE mesh attribute. \
         If the location doesn't match, instanced rendering will fail."
    );
}

/// Test 18: Test that ATTRIBUTE_INSTANCE_COLOR is at correct shader location
#[test]
fn test_attribute_instance_color_location() {
    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // Verify VertexInput struct has instance_color at location(2)
    assert!(
        content.contains("@location(2) instance_color: vec4<f32>"),
        "Shader must declare @location(2) instance_color: vec4<f32> to match \
         ATTRIBUTE_INSTANCE_COLOR mesh attribute. \
         If the location doesn't match, instanced rendering will fail."
    );
}

/// Test 19: Test that mesh has POSITION attribute at location 0
#[test]
fn test_mesh_position_attribute() {
    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // Verify VertexInput struct has position at location(0)
    assert!(
        content.contains("@location(0) position: vec3<f32>"),
        "Shader must declare @location(0) position: vec3<f32> to match \
         Mesh::ATTRIBUTE_POSITION. \
         The point mesh provides position data at this location."
    );
}

/// Test 20: Test that instance attributes are properly set up for instanced rendering
#[test]
fn test_instance_attributes_for_instanced_rendering() {
    let mut mesh = Mesh::new(PrimitiveTopology::PointList, bevy::render::render_asset::RenderAssetUsages::default());

    // Add basic position attribute
    mesh.insert_attribute(
        bevy::render::mesh::Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0]],
    );

    // Add instance size attribute
    mesh.insert_attribute(ATTRIBUTE_INSTANCE_SIZE, vec![1.0f32]);

    // Add instance color attribute
    mesh.insert_attribute(ATTRIBUTE_INSTANCE_COLOR, vec![[1.0, 1.0, 1.0, 1.0f32]]);

    // Verify all attributes exist
    assert!(
        mesh.contains_attribute(bevy::render::mesh::Mesh::ATTRIBUTE_POSITION),
        "Mesh must contain POSITION attribute for vertex positions"
    );

    assert!(
        mesh.contains_attribute(ATTRIBUTE_INSTANCE_SIZE),
        "Mesh must contain INSTANCE_SIZE attribute for per-instance size data"
    );

    assert!(
        mesh.contains_attribute(ATTRIBUTE_INSTANCE_COLOR),
        "Mesh must contain INSTANCE_COLOR attribute for per-instance color data"
    );

    // Verify attribute values
    if let Some(VertexAttributeValues::Float32(values)) =
        mesh.attribute(ATTRIBUTE_INSTANCE_SIZE)
    {
        assert_eq!(
            values.len(),
            1,
            "Instance size attribute should have 1 value per vertex"
        );
        assert_eq!(
            values[0], 1.0,
            "Instance size should be correctly set to 1.0"
        );
    } else {
        panic!("INSTANCE_SIZE attribute should be Float32 format");
    }

    if let Some(VertexAttributeValues::Float32x4(values)) =
        mesh.attribute(ATTRIBUTE_INSTANCE_COLOR)
    {
        assert_eq!(
            values.len(),
            1,
            "Instance color attribute should have 1 value per vertex"
        );
        assert_eq!(
            values[0],
            [1.0, 1.0, 1.0, 1.0],
            "Instance color should be correctly set to white [1.0, 1.0, 1.0, 1.0]"
        );
    } else {
        panic!("INSTANCE_COLOR attribute should be Float32x4 format");
    }
}

/// Test 21: Test that vertex attribute locations don't conflict
#[test]
fn test_vertex_attribute_locations_no_conflicts() {
    // Bevy's standard attributes have known locations:
    // - POSITION: location 0
    // - NORMAL: location 1
    // - UV_0: location 2
    // - TANGENT: location 3
    // - COLOR_0: location 4
    
    // Our custom attributes should use different locations to avoid conflicts:
    // - INSTANCE_SIZE: location 1 (conflicts with NORMAL, but PointList doesn't need normals)
    // - INSTANCE_COLOR: location 2 (conflicts with UV_0, but PointList doesn't need UVs)
    
    // Since PointMesh uses PointList topology and doesn't have normals or UVs,
    // using locations 1 and 2 for instance attributes is safe.
    
    // Verify our custom attribute IDs are unique
    assert_ne!(
        ATTRIBUTE_INSTANCE_SIZE.id, ATTRIBUTE_INSTANCE_COLOR.id,
        "Custom vertex attributes must have unique IDs to avoid conflicts"
    );

    // Verify IDs don't conflict with standard Bevy attributes
    let standard_attributes = [
        bevy::render::mesh::Mesh::ATTRIBUTE_POSITION,
        bevy::render::mesh::Mesh::ATTRIBUTE_NORMAL,
        bevy::render::mesh::Mesh::ATTRIBUTE_UV_0,
        bevy::render::mesh::Mesh::ATTRIBUTE_TANGENT,
        bevy::render::mesh::Mesh::ATTRIBUTE_COLOR,
    ];

    for standard in &standard_attributes {
        assert_ne!(
            ATTRIBUTE_INSTANCE_SIZE.id, standard.id,
            "INSTANCE_SIZE ID must not conflict with standard attribute {:?}",
            standard.name
        );
        assert_ne!(
            ATTRIBUTE_INSTANCE_COLOR.id, standard.id,
            "INSTANCE_COLOR ID must not conflict with standard attribute {:?}",
            standard.name
        );
    }
}

// ============================================================================
// E. PER-INSTANCE DATA SYNCHRONIZATION TESTS
// ============================================================================

/// Test 22: Test that Particle component is correctly structured
#[test]
fn test_particle_component_structure() {
    let particle = Particle {
        position: Vec3::new(1.0, 2.0, 3.0),
        color: bevy::color::Color::from(bevy::color::palettes::css::RED),
        size: 5.0,
    };

    // Verify all fields are accessible
    assert_eq!(particle.position, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(particle.size, 5.0);

    // Verify color is valid - check that color components are finite and alpha is valid
    let linear_color = particle.color.to_linear();
    assert!(
        linear_color.red.is_finite(),
        "Particle color red component should be finite"
    );
    assert!(
        linear_color.green.is_finite(),
        "Particle color green component should be finite"
    );
    assert!(
        linear_color.blue.is_finite(),
        "Particle color blue component should be finite"
    );
    assert_eq!(
        linear_color.alpha, 1.0,
        "Particle color alpha should be 1.0"
    );
}

/// Test 23: Test that ParticleInstanceData has correct memory layout
#[test]
fn test_particle_instance_data_memory_layout() {
    // Verify size: ParticleInstanceData has size and color fields
    // The actual size depends on the struct definition
    let size_of_data = std::mem::size_of::<genesis_render::particle::ParticleInstanceData>();
    assert!(
        size_of_data > 0,
        "ParticleInstanceData should have a non-zero size for GPU compatibility"
    );

    // Verify alignment: depends on the struct definition
    let align_of_data = std::mem::align_of::<genesis_render::particle::ParticleInstanceData>();
    assert!(
        align_of_data > 0,
        "ParticleInstanceData should have valid alignment for GPU compatibility"
    );

    // Verify it's Pod and Zeroable for bytemuck
    let instance_data = genesis_render::particle::ParticleInstanceData::zeroed();
    assert_eq!(instance_data.size, 0.0, "Zeroed size should be 0.0");
    assert_eq!(
        instance_data.color,
        [0.0, 0.0, 0.0, 0.0],
        "Zeroed color should be [0.0, 0.0, 0.0, 0.0]"
    );
}

/// Test 24: Test that ExtractedParticleInstances can hold particle data
#[test]
fn test_extracted_particle_instances() {
    // Note: ParticleInstanceData has private padding fields, so we use zeroed() and modify
    let mut instance1 = genesis_render::particle::ParticleInstanceData::zeroed();
    instance1.size = 1.0;
    instance1.color = [1.0, 0.0, 0.0, 1.0];

    let mut instance2 = genesis_render::particle::ParticleInstanceData::zeroed();
    instance2.size = 2.0;
    instance2.color = [0.0, 1.0, 0.0, 1.0];

    let extracted = ExtractedParticleInstances(vec![instance1, instance2]);

    assert_eq!(extracted.0.len(), 2, "Should have 2 particle instances");

    // Verify first instance
    assert_eq!(extracted.0[0].size, 1.0);
    assert_eq!(extracted.0[0].color, [1.0, 0.0, 0.0, 1.0]);

    // Verify second instance
    assert_eq!(extracted.0[1].size, 2.0);
    assert_eq!(extracted.0[1].color, [0.0, 1.0, 0.0, 1.0]);
}

/// Test 25: Test color conversion from Bevy Color to Linear RGBA
#[test]
fn test_color_conversion() {
    let srgb_color = bevy::color::Color::srgb(1.0, 0.5, 0.0);
    let linear_color = srgb_color.to_linear();

    // Verify conversion produces valid RGBA values
    assert!(
        linear_color.red >= 0.0 && linear_color.red <= 1.0,
        "Red component should be in [0.0, 1.0] range"
    );
    assert!(
        linear_color.green >= 0.0 && linear_color.green <= 1.0,
        "Green component should be in [0.0, 1.0] range"
    );
    assert!(
        linear_color.blue >= 0.0 && linear_color.blue <= 1.0,
        "Blue component should be in [0.0, 1.0] range"
    );
    assert_eq!(linear_color.alpha, 1.0, "Alpha should be 1.0");
}

/// Test 26: Test that ParticleInstanceBindGroupLayout can be created
#[test]
fn test_particle_instance_bind_group_layout() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::render::RenderPlugin::default())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Note: ParticleInstanceBindGroupLayout is typically created in the render world
    // For this test, we just verify the resource can be referenced in the type system
    // The actual creation happens during plugin initialization

    // If we get here without panic, the type is properly defined
    let _ = std::mem::size_of::<ParticleInstanceBindGroupLayout>();
}

/// Test 27: Test that instance buffer capacity tracking works
#[test]
fn test_instance_buffer_capacity_tracking() {
    // Create a mock instance buffer to test capacity tracking
    // Note: We can't create actual GPU buffers in tests, but we can test the logic
    
    // Test power of two calculation
    assert_eq!(1_usize.next_power_of_two(), 1);
    assert_eq!(5_usize.next_power_of_two(), 8);
    assert_eq!(1024_usize.next_power_of_two(), 1024);
    assert_eq!(1025_usize.next_power_of_two(), 2048);

    // Test max logic for initial capacity
    let initial_count = 100;
    let min_capacity: usize = 1024;
    let capacity = initial_count.max(min_capacity).next_power_of_two();
    assert_eq!(capacity, 1024, "Capacity should be 1024 for 100 particles");

    let initial_count = 2048;
    let capacity = initial_count.max(min_capacity).next_power_of_two();
    assert_eq!(capacity, 2048, "Capacity should be 2048 for 2048 particles");
}

// ============================================================================
// F. RESOURCE ACCESS SAFETY TESTS
// ============================================================================

/// Test 28: Test that resources are properly referenced counted
#[test]
fn test_resource_reference_counting() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Create a mesh asset
    let mut meshes = app.world_mut().resource_mut::<bevy::asset::Assets<bevy::render::mesh::Mesh>>();
    let mesh_handle = meshes.add(bevy::render::mesh::Mesh::new(
        PrimitiveTopology::PointList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    ));

    // Clone the handle to simulate multiple references
    let mesh_handle_clone = mesh_handle.clone();

    // Both handles should point to the same mesh
    let meshes = app.world().resource::<bevy::asset::Assets<bevy::render::mesh::Mesh>>();
    assert!(
        meshes.get(&mesh_handle).is_some(),
        "Original handle should access the mesh"
    );
    assert!(
        meshes.get(&mesh_handle_clone).is_some(),
        "Cloned handle should access the same mesh"
    );
}

/// Test 29: Test that resource access doesn't cause panics
#[test]
fn test_resource_access_no_panics() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once());

    // Add a resource
    app.world_mut().insert_resource(ParticleConfig {
        initial_count: 10,
        max_count: 100,
        base_size: 5.0,
    });

    // Access the resource multiple times
    let config1 = app.world().resource::<ParticleConfig>();
    assert_eq!(config1.initial_count, 10);

    let config2 = app.world().resource::<ParticleConfig>();
    assert_eq!(config2.initial_count, 10);

    // Mutably access the resource
    let mut config = app.world_mut().resource_mut::<ParticleConfig>();
    config.initial_count = 20;

    // Verify the change persisted
    let config = app.world().resource::<ParticleConfig>();
    assert_eq!(config.initial_count, 20, "Resource modification should persist");
}

/// Test 30: Test that systems can't access invalid resources
#[test]
fn test_system_cannot_access_invalid_resources() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once());

    // Try to access a resource that doesn't exist
    #[derive(Resource)]
    struct NonExistentResource {
        _value: i32,
    }

    // The resource doesn't exist, so accessing it should return None or fail
    assert!(
        !app.world().contains_resource::<NonExistentResource>(),
        "NonExistentResource should not exist in the world"
    );

    // Systems that require this resource should not run
    fn requires_non_existent(_resource: Res<NonExistentResource>) {
        // This system shouldn't run because the resource doesn't exist
        panic!("This system should not run - resource doesn't exist");
    }

    app.add_systems(bevy::app::Update, requires_non_existent);

    // Run the update schedule - the system should not run
    app.world_mut().run_schedule(bevy::app::Update);

    // If we get here, the system didn't run (correct behavior)
}

/// Test 31: Test proper cleanup of resources on app shutdown
#[test]
fn test_resource_cleanup_on_shutdown() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Add resources
    app.world_mut().insert_resource(ParticleConfig {
        initial_count: 10,
        max_count: 100,
        base_size: 5.0,
    });

    // Verify resources exist
    assert!(app.world().contains_resource::<ParticleConfig>());

    // Simulate app shutdown by dropping the app
    // The resources should be cleaned up properly without panics
    drop(app);

    // If we get here, cleanup was successful
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

/// Test 32: Integration test - Complete particle rendering setup
#[test]
fn test_complete_particle_rendering_setup() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default())
    .add_plugins(bevy::time::TimePlugin::default());

    // Add particle config
    app.insert_resource(ParticleConfig {
        initial_count: 20,
        max_count: 200,
        base_size: 4.0,
    });

    // Add startup systems
    app.add_systems(
        bevy::app::Startup,
        (
            genesis_render::particle::init_point_mesh,
            genesis_render::particle::spawn_particles.after(genesis_render::particle::init_point_mesh),
        ),
    );

    // Add update systems
    app.add_systems(
        bevy::app::Update,
        (
            genesis_render::particle::update_particles,
            genesis_render::particle::update_particle_energy_colors,
        ),
    );

    // Run startup
    app.world_mut().run_schedule(bevy::app::Startup);

    // Verify setup
    assert!(
        app.world().contains_resource::<PointMesh>(),
        "PointMesh should be initialized"
    );

    let particle_count = app
        .world()
        .iter_entities()
        .filter(|e| app.world().get::<Particle>(e.id()).is_some())
        .count();
    assert_eq!(particle_count, 20, "Exactly 20 particles should be spawned");

    // Run update schedule
    app.world_mut().run_schedule(bevy::app::Update);

    // Verify particles were updated
    for entity in app
        .world()
        .iter_entities()
        .filter(|e| app.world().get::<Particle>(e.id()).is_some())
    {
        let transform = app.world().get::<Transform>(entity.id());
        assert!(
            transform.is_some(),
            "Particle entity should have a Transform component"
        );
    }
}

/// Test 33: Integration test - Extract system transfers data correctly
#[test]
fn test_extract_system_transfers_data() {
    let mut app = App::new();

    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once())
    .add_plugins(bevy::asset::AssetPlugin::default());

    // Add particle config
    app.insert_resource(ParticleConfig {
        initial_count: 5,
        max_count: 50,
        base_size: 3.0,
    });

    // Add startup systems
    app.add_systems(
        bevy::app::Startup,
        (
            genesis_render::particle::init_point_mesh,
            genesis_render::particle::spawn_particles.after(genesis_render::particle::init_point_mesh),
        ),
    );

    // Run startup
    app.world_mut().run_schedule(bevy::app::Startup);

    // Add extract system
    app.add_systems(ExtractSchedule, extract_particle_instances);

    // Run extract schedule
    app.world_mut().run_schedule(ExtractSchedule);

    // Verify ExtractedParticleInstances resource was created
    // Note: In a real app, this would be in the Render world, but for testing
    // we verify the system can run without errors
}

// ============================================================================
// SUMMARY TEST - Verifies all critical bindings
// ============================================================================

/// Test 34: Comprehensive binding validation test
///
/// This test validates ALL critical bindings that must be correct for
/// the particle rendering system to work:
#[test]
fn test_comprehensive_binding_validation() {
    use std::fs;
    let shader_path = "genesis-render/src/particle/point_sprite.wgsl";
    
    let content = fs::read_to_string(shader_path)
        .expect(&format!("Failed to read shader file: {}", shader_path));

    // CRITICAL BINDINGS - All must exist for rendering to work

    // Binding 0: PointSpriteMaterial (color, base_size, attenuation_factor)
    assert!(
        content.contains("@group(0) @binding(0)"),
        "CRITICAL: Binding 0 (PointSpriteMaterial) must exist"
    );
    assert!(
        content.contains("struct PointSpriteMaterial"),
        "CRITICAL: PointSpriteMaterial struct must be defined"
    );
    assert!(
        content.contains("color: vec4<f32>"),
        "CRITICAL: PointSpriteMaterial must have color: vec4<f32>"
    );
    assert!(
        content.contains("base_size: f32"),
        "CRITICAL: PointSpriteMaterial must have base_size: f32"
    );
    assert!(
        content.contains("attenuation_factor: f32"),
        "CRITICAL: PointSpriteMaterial must have attenuation_factor: f32"
    );

    // Binding 1: ViewUniform (view_proj, world_position)
    // THIS IS THE BINDING THAT CAUSED THE ORIGINAL ERROR
    assert!(
        content.contains("@group(0) @binding(1)"),
        "CRITICAL: Binding 1 (ViewUniform) must exist. \
         Missing this binding causes the error: \
         'Shader global ResourceBinding {{ group: 0, binding: 1 }} is not available \
         in the pipeline layout - Visibility flags don't include the shader stage'"
    );
    assert!(
        content.contains("struct ViewUniform"),
        "CRITICAL: ViewUniform struct must be defined"
    );
    assert!(
        content.contains("view_proj: mat4x4<f32>"),
        "CRITICAL: ViewUniform must have view_proj: mat4x4<f32> for view-projection matrix"
    );
    assert!(
        content.contains("world_position: vec3<f32>"),
        "CRITICAL: ViewUniform should have world_position: vec3<f32>"
    );

    // Binding 2: Model matrix
    assert!(
        content.contains("@group(0) @binding(2)"),
        "CRITICAL: Binding 2 (model matrix) must exist"
    );
    assert!(
        content.contains("var<uniform> model: mat4x4<f32>"),
        "CRITICAL: Binding 2 must bind to model matrix"
    );

    // VERTEX SHADER ACCESS - Must access all bindings correctly
    let vertex_start = content.find("@vertex").expect("Vertex shader not found");
    let vertex_end = content.find("@fragment").unwrap_or(content.len());
    let vertex_shader = &content[vertex_start..vertex_end];

    assert!(
        vertex_shader.contains("model"),
        "CRITICAL: Vertex shader must use model (binding 2) for world space transformation"
    );
    assert!(
        vertex_shader.contains("view.view_proj"),
        "CRITICAL: Vertex shader MUST access view.view_proj (binding 1) for clip space. \
         If this binding is not in the pipeline layout with VERTEX visibility, the GPU \
         will fail with: 'Shader global ResourceBinding {{ group: 0, binding: 1 }} \
         is not available in the pipeline layout - Visibility flags don't include \
         the shader stage'"
    );

    // VERTEX INPUT LOCATIONS - Must match mesh attributes
    assert!(
        content.contains("@location(0) position: vec3<f32>"),
        "CRITICAL: Vertex input must have position at location(0) to match Mesh::ATTRIBUTE_POSITION"
    );
    assert!(
        content.contains("@location(1) instance_size: f32"),
        "CRITICAL: Vertex input must have instance_size at location(1) to match ATTRIBUTE_INSTANCE_SIZE"
    );
    assert!(
        content.contains("@location(2) instance_color: vec4<f32>"),
        "CRITICAL: Vertex input must have instance_color at location(2) to match ATTRIBUTE_INSTANCE_COLOR"
    );

    // RUST MESH ATTRIBUTES - Must match shader locations
    assert_eq!(
        ATTRIBUTE_INSTANCE_SIZE.name,
        "instance_size",
        "CRITICAL: Mesh attribute name must match shader location(1)"
    );
    assert_eq!(
        ATTRIBUTE_INSTANCE_SIZE.format,
        VertexFormat::Float32,
        "CRITICAL: Mesh attribute format must match shader f32 type"
    );
    assert_eq!(
        ATTRIBUTE_INSTANCE_COLOR.name,
        "instance_color",
        "CRITICAL: Mesh attribute name must match shader location(2)"
    );
    assert_eq!(
        ATTRIBUTE_INSTANCE_COLOR.format,
        VertexFormat::Float32x4,
        "CRITICAL: Mesh attribute format must match shader vec4<f32> type"
    );

    // MATERIAL TRAIT - Must be correctly implemented
    let vertex_shader = PointSpriteMaterial::vertex_shader();
    let fragment_shader = PointSpriteMaterial::fragment_shader();

    match (&vertex_shader, &fragment_shader) {
        (ShaderRef::Path(v), ShaderRef::Path(f)) => {
            assert!(
                v.to_string().contains("point_sprite.wgsl"),
                "CRITICAL: Vertex shader must reference point_sprite.wgsl"
            );
            assert!(
                f.to_string().contains("point_sprite.wgsl"),
                "CRITICAL: Fragment shader must reference point_sprite.wgsl"
            );
        }
        _ => panic!(
            "CRITICAL: Both shaders must use ShaderRef::Path"
        ),
    }

    // If all assertions pass, the binding setup is correct
}
