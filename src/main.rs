//! GENESIS - A real-time Big Bang and Cosmological Evolution Simulator
//!
//! Simulates the universe's evolution from singularity through 13.8 billion years.

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

fn main() {
    // TODO: Config::load() method is not yet implemented in genesis-core/src/config.rs
    // Currently this will fail to compile. Need to implement Config::load() method
    // to load from ./genesis.toml, ~/.config/genesis/config.toml, or /etc/genesis/config.toml
    let config = Config::load();

    App::new()
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
        .add_plugins(TimeIntegrationPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(CameraPlugin)
        .insert_resource(ConfigResource(config.clone()))
        .insert_resource(config.particle.clone())
        // TODO: CameraState::from_config() expects config.initial_mode (CameraMode enum) but
        // CameraConfig has camera_mode (String). Either convert String to CameraMode or
        // change CameraConfig to use CameraMode enum directly.
        .insert_resource(CameraState::from_config(&config.camera))
        .add_plugins(GenesisUiPlugin)
        // TODO: OverlayState does not have show_epoch_info field (defined in genesis-ui/src/overlay/mod.rs)
        // Either add show_epoch_info to OverlayState struct or remove this field from config.display
        .insert_resource(OverlayState {
            show_fps: config.display.show_fps,
            show_particle_count: config.display.show_particle_count,
        })
        .add_systems(Startup, setup_camera)
        .run();
}

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
