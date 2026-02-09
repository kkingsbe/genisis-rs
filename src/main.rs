//! GENESIS - A real-time Big Bang and Cosmological Evolution Simulator
//!
//! Simulates the universe's evolution from singularity through 13.8 billion years.

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use genesis_core::epoch::{EpochManager, EpochManagerPlugin, EpochPlugin};
use genesis_core::Config;
use genesis_core::SingularityEpoch;
use genesis_core::TimeIntegrationPlugin;
use genesis_render::camera::{CameraController, CameraState, CameraTarget, OrbitController};
use genesis_render::input::InputPlugin;
use genesis_render::particle::ParticlePlugin;
use genesis_render::CameraPlugin;
use genesis_ui::overlay::OverlayState;
use genesis_ui::GenesisUiPlugin;
use std::sync::Arc;

/// Wrapper for Config to enable it as a Bevy Resource
#[derive(Resource, Clone)]
pub struct ConfigResource(pub Config);

/// Plugin that registers the SingularityEpoch with the EpochManager
struct SingularityEpochPlugin;

impl Plugin for SingularityEpochPlugin {
    fn build(&self, app: &mut App) {
        // Register epoch plugins
        let singularity_epoch = Arc::new(SingularityEpoch);

        // Get mutable access to the EpochManager resource and register the plugin
        {
            let mut epoch_manager = app
                .world_mut()
                .get_resource_mut::<EpochManager>()
                .expect("EpochManager resource should be initialized by EpochManagerPlugin");

            // Register the SingularityEpoch
            epoch_manager.register_plugin(singularity_epoch.clone());
        }

        // Build the epoch's systems (separate call to avoid borrow conflict)
        singularity_epoch.build(app);
    }
}

fn main() {
    // Load configuration from CLI arguments or default locations
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
        .add_plugins(EpochManagerPlugin)
        .add_plugins(SingularityEpochPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(CameraPlugin)
        .insert_resource(ConfigResource(config.clone()))
        .init_resource::<CameraState>()
        .add_plugins(GenesisUiPlugin)
        .insert_resource(OverlayState {
            show_fps: config.display.show_fps,
            show_particle_count: config.display.show_particle_count,
            show_epoch_info: config.display.show_epoch_info,
        })
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_test_camera_target)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Spawn camera with both controllers to allow switching between modes
    // The active controller is determined by CameraState.mode
    // CameraState defaults to FreeFlight mode, but setup_camera does not read from config
    // Both controllers are always present; mode switching toggles which one responds to input
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitController::default(),
        CameraController::default(),
    ));
}

fn setup_test_camera_target(mut commands: Commands) {
    // Spawn a test CameraTarget entity to test camera interpolation
    // The camera will smoothly interpolate toward this position when the app runs
    commands.spawn(CameraTarget::new(Vec3::new(10.0, 5.0, 20.0)));
}
