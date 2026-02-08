//! Camera systems
//!
//! Free-flight and orbit camera implementations with smooth
//! interpolation and transition effects.

use bevy::prelude::*;

/// Camera mode for different viewing experiences
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    #[default]
    FreeFlight,
    Orbit,
}

/// Resource tracking camera state
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
