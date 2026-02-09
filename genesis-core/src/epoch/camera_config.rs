//! Camera configuration
//!
//! Provides camera mode definitions for different viewing experiences.
//! This module defines the available camera modes used throughout
//! the simulation.

use serde::{Deserialize, Serialize};

/// Camera mode for different viewing experiences
///
/// Determines how the camera behaves in response to user input and
/// what controls are available to the user.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraMode {
    #[default]
    FreeFlight,
    Orbit,
}
