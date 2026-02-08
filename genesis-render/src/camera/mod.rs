//! Camera mode definitions
//!
//! Defines camera mode enums and state tracking resources.
//! Actual camera movement and input handling systems are not yet implemented.

use bevy::prelude::*;

/// Camera mode for different viewing experiences
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    #[default]
    FreeFlight,
    Orbit,
}

/// Resource tracking camera state
///
/// Stores the current camera mode and optional target point.
/// Camera movement and input handling systems need to be implemented separately.
#[derive(Resource)]
pub struct CameraState {
    pub mode: CameraMode,
    pub target: Option<Vec3>,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            mode: CameraMode::default(),
            target: None,
        }
    }
}
