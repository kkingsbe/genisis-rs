//! GENESIS - A real-time Big Bang and Cosmological Evolution Simulator
//!
//! Simulates the universe's evolution from singularity through 13.8 billion years.

use bevy::prelude::*;
use genesis_core::TimeIntegrationPlugin;
use genesis_core::epoch::EpochManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TimeIntegrationPlugin)
        .add_plugins(EpochManagerPlugin)
        .run();
}
