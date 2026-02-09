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
//!
//! # Implementation Status
//!
//! This is currently just a **marker struct**. It does NOT implement an EpochPlugin trait
//! because such a trait does NOT exist in the codebase (see genesis-core/src/epoch/mod.rs).
//!
//! When the epoch management system is implemented, this struct would need to:
//! 1. Implement an EpochPlugin trait (with methods like name(), start_year(), end_year(), build(), camera_config())
//! 2. Be registered with the EpochManager (which also doesn't exist yet)
//! 3. Provide systems that run during this epoch's time range

use crate::time::seconds_to_years;

/// The Singularity epoch marker
///
/// Represents the first epoch of cosmic evolution, spanning from time zero
/// to the Planck boundary (approximately 10⁻³² seconds).
///
/// This is a marker struct used as a type identifier for the Singularity epoch.
///
/// **TODO**: When EpochPlugin trait is implemented, this struct will need to implement it
/// to define epoch time ranges, camera configuration, and build systems.
pub struct SingularityEpoch;

impl SingularityEpoch {
    /// The Planck boundary time in seconds (10⁻³² seconds)
    pub const PLANCK_BOUNDARY_SECONDS: f64 = 1e-32;

    /// The Planck boundary time converted to cosmic years
    pub fn planck_boundary_years() -> f64 {
        seconds_to_years(Self::PLANCK_BOUNDARY_SECONDS)
    }
}
