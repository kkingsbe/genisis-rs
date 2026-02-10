//! GENESIS - A real-time Big Bang and Cosmological Evolution Simulator
//!
//! Simulates the universe's evolution from singularity through 13.8 billion years.
//!
//! # Application Architecture
//!
//! This is the main entry point that initializes the Bevy engine and registers all plugins:
//!
//! - **TimeIntegrationPlugin** (genesis-core): Cosmic time accumulation with f64 precision
//! - **InputPlugin** (genesis-render): Keyboard and mouse input handling (PreUpdate schedule)
//! - **ParticlePlugin** (genesis-render): Particle spawning, rendering, and GPU instancing
//! - **CameraPlugin** (genesis-render): Camera control systems (free-flight, orbit rotation)
//! - **GenesisUiPlugin** (genesis-ui): UI overlay and timeline controls with bevy_egui
//!
//! # Resources Initialized
//!
//! - `ConfigResource`: Wraps Config for Bevy resource system
//! - `ParticleConfig`: Resource for particle spawning (initial_count, max_count, base_size)
//! - `CameraState`: Tracks camera mode (FreeFlight/Orbit) and orbit target
//! - `OverlayState`: Controls overlay visibility (show_fps, show_particle_count)
//!
//! # Camera System
//!
//! The camera entity is spawned with both `CameraController` and `OrbitController` components.
//! Mode switching toggles `CameraState.mode`, and the appropriate controller responds to input.
//! - Free-flight: WASD movement + mouse look (always active when mode is FreeFlight)
//! - Orbit: Left mouse drag to rotate (only active when mode is Orbit)
//! - Zoom/Pan: NOT implemented (handle_orbit_zoom and handle_orbit_pan systems do not exist)
//!
//! # Configuration
//!
//! Configuration is loaded from `genesis.toml` via `Config::load()` which searches:
//! 1. `./genesis.toml` - Workspace-local configuration
//! 2. `~/.config/genesis/config.toml` - User-specific configuration
//! 3. `/etc/genesis/config.toml` - System-wide configuration

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use genesis_core::Config;
use genesis_core::TimeIntegrationPlugin;
use genesis_render::camera::{CameraController, CameraState, OrbitController};
use genesis_render::input::InputPlugin;
use genesis_render::particle::ParticlePlugin;
use genesis_render::CameraPlugin;
use genesis_ui::overlay::OverlayState;
use genesis_ui::GenesisUiPlugin;

/// Wrapper for Config to enable it as a Bevy Resource
#[derive(Resource, Clone)]
pub struct ConfigResource(pub Config);

/// Main application entry point
///
/// Initializes the Bevy engine, loads configuration, registers plugins,
/// and sets up the camera system.
fn main() {
    // Load configuration from genesis.toml (searches local, user, and system paths)
    let config = Config::load();

    App::new()
        // Configure window from config settings
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.window.title.clone(),
                resolution: WindowResolution::new(
                    config.window.width as f32,
                    config.window.height as f32,
                ),
                present_mode: if config.window.vsync {
                    PresentMode::AutoVsync
                } else {
                    PresentMode::AutoNoVsync
                },
                ..default()
            }),
            ..default()
        }))
        // Core simulation systems
        .add_plugins(TimeIntegrationPlugin)
        // Input handling (WASD, mouse motion, mouse buttons)
        .add_plugins(InputPlugin)
        // Particle rendering with GPU instancing
        .add_plugins(ParticlePlugin)
        // Camera control systems (free-flight, orbit)
        .add_plugins(CameraPlugin)
        // Configuration resources
        .insert_resource(ConfigResource(config.clone()))
        .insert_resource(config.particle.clone())
        .insert_resource(CameraState::from_config(&config.camera))
        // UI systems (timeline, overlay)
        .add_plugins(GenesisUiPlugin)
        .insert_resource(OverlayState {
            show_fps: config.display.show_fps,
            show_particle_count: config.display.show_particle_count,
        })
        // Camera setup system
        .add_systems(Startup, setup_camera)
        .run();
}

/// Camera setup system
///
/// Spawns a 3D camera entity with both camera controllers for mode switching.
/// The active controller is determined by `CameraState.mode` (toggle via 'O' key).
///
/// # Camera Components
///
/// - `Camera3d`: Bevy 3D camera component
/// - `Transform`: Position and orientation (initial: orbit_distance on Z axis, looking at origin)
/// - `OrbitController`: Orbit distance, yaw, pitch for orbit mode
/// - `CameraController`: Yaw, pitch, movement speed for free-flight mode
///
/// # Configuration
///
/// - Orbit distance is loaded from `config.camera.orbit_distance` (genesis.toml field)
/// - Initial mode is loaded from `config.camera.initial_mode` (via CameraState::from_config)
fn setup_camera(mut commands: Commands, config: Res<ConfigResource>) {
    // Spawn camera with both controllers to allow switching between modes
    // The active controller is determined by CameraState.mode
    // Both controllers are always present; mode switching toggles which one responds to input
    // Camera configuration is loaded from config.camera
    let orbit_distance: f32 = config.0.camera.orbit_distance as f32;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, orbit_distance).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitController { distance: orbit_distance, ..default() },
        CameraController::default(),
    ));
}
