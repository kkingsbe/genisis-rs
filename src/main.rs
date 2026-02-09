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
        .insert_resource(CameraState::from_config(&config.camera))
        .add_plugins(GenesisUiPlugin)
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
