//! The Singularity/Planck Boundary epoch
//!
//! This epoch represents the earliest phase of cosmic evolution, from the Big Bang
//! (t = 0) to approximately 10⁻³² seconds (the Planck time boundary).
//!
//! According to the PRD, this is Phase 1 (The Singularity) where:
//! - Time duration: t < 10⁻³²s (before the Planck boundary)
//! - Characteristics: Particles spawn at the origin with outward velocity
//! - Physical regime: Quantum gravity effects dominate
//!
//! This epoch represents the birth of the universe, where all matter and energy
//! were concentrated at a single point (the singularity) before rapidly expanding
//! in the Big Bang.

use crate::epoch::{CameraMode, EpochPlugin, EpochCameraConfig};
use crate::time::seconds_to_years;
use bevy::math::{Quat, Vec3};
use bevy::prelude::*;

/// The Singularity epoch plugin
///
/// Implements the first epoch of cosmic evolution, spanning from time zero
/// to the Planck boundary (approximately 10⁻³² seconds).
pub struct SingularityEpoch;

impl EpochPlugin for SingularityEpoch {
    fn name(&self) -> &'static str {
        "Singularity"
    }

    fn start_year(&self) -> f64 {
        0.0
    }

    fn end_year(&self) -> f64 {
        // Planck boundary: 10⁻³² seconds converted to cosmic years
        seconds_to_years(1e-32)
    }

    fn build(&self, _app: &mut App) {
        // Singularity-specific systems: The Singularity epoch is currently a placeholder.
        // Future additions may include:
        // - Singularity visualization systems (particle clustering at origin)
        // - Singularity-specific particle physics (e.g., quantum gravity effects)
        // - Transition systems for entering/exiting the singularity phase
    }

    fn camera_config(&self) -> EpochCameraConfig {
        EpochCameraConfig {
            target_position: Some(Vec3::new(0.0, 0.0, 20.0)),
            target_rotation: Some(Quat::IDENTITY),
            target_mode: Some(CameraMode::Orbit),
            fade_duration: None,
        }
    }
}
