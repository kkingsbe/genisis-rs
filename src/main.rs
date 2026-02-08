//! GENESIS - A real-time Big Bang and Cosmological Evolution Simulator
//!
//! Simulates the universe's evolution from singularity through 13.8 billion years.

use bevy::prelude::*;
use genesis_core::TimeIntegrationPlugin;
use genesis_core::epoch::EpochManagerPlugin;
use genesis_render::input::InputPlugin;
use genesis_render::particle::ParticlePlugin;
use genesis_render::CameraPlugin;
use genesis_render::camera::OrbitController;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TimeIntegrationPlugin)
        .add_plugins(EpochManagerPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitController::default(),
    ));
}
