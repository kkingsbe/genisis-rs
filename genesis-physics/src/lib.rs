//! GENESIS Physics - Physics simulation systems for cosmological evolution
//!
//! This crate contains physics systems for the Big Bang and Cosmological
//! Evolution Simulator, including gravitational physics, inflaton field dynamics,
//! density perturbations, and nucleosynthesis physics.
//!
//! # Modules
//!
//! - [`integrator`] - Generic numerical integrators for differential equations
//! - [`gravity`] - Gravitational physics for particle interactions and structure formation
//! - [`inflaton`] - Inflaton field dynamics for cosmic inflation epoch
//! - [`perturbations`] - Density perturbations and quantum fluctuations seeding structure
//! - [`nucleosynthesis`] - Element formation physics for primordial nucleosynthesis
//! - [`cosmology`] - Cosmological physics for cosmic expansion and Friedmann equations
//!
//! # Physics Systems
//!
//! These systems implement the fundamental physical processes that drive
//! cosmological evolution from the initial singularity through structure formation.

use bevy::prelude::*;

pub mod integrator;
pub mod gravity;
pub mod inflaton;
pub mod perturbations;
pub mod nucleosynthesis;
pub mod cosmology;

// Re-export commonly used types
pub use inflaton::InflatonPlugin;
pub use crate::cosmology::{ScaleFactor, CosmicEpoch, Temperature};

/// Version of the physics library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Genesis Physics Plugin
///
/// This Bevy plugin registers all physics systems for cosmological simulation.
/// Physics systems will be registered here in future tasks.
pub struct GenesisPhysicsPlugin;

impl Plugin for GenesisPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Register cosmological physics
        app.add_plugins(cosmology::CosmologyPlugin);
    }
}
