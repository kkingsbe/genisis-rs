//! Camera configuration for epoch transitions
//!
//! Defines camera target position, rotation, and mode that should be applied
//! when transitioning to a specific epoch. This allows each epoch to specify
//! optimal camera settings for visualizing that phase of cosmic evolution.
//!
//! # Crossfade Synchronization
//!
//! The camera transition system uses a synchronized crossfade effect during epoch changes.
//! The fade effect and camera interpolation are carefully timed to create a smooth
//! visual experience:
//!
//! - The fade effect consists of two phases: fade out (to white) and fade in (from white)
//! - Each phase lasts `fade_duration` seconds, so the total fade sequence is `2 × fade_duration`
//! - Camera interpolation spans the entire fade sequence, taking exactly `2 × fade_duration` seconds
//! - This ensures the camera moves smoothly while the screen is transitioning between epochs
//!
//! # Per-Epoch Configuration
//!
//! Each epoch can specify its own `fade_duration` value via `EpochCameraConfig::fade_duration`.
//! If not specified, a default of 0.75 seconds per phase is used (1.5 seconds total transition).
//!
//! To configure a custom fade duration for an epoch, use:
//!
//! ```ignore
//! impl EpochPlugin for MyEpoch {
//!     fn camera_config(&self) -> EpochCameraConfig {
//!         EpochCameraConfig {
//!             fade_duration: Some(1.0), // 1.0 seconds per phase = 2.0 seconds total
//!             ..default()
//!         }
//!     }
//! }
//! ```

use bevy::math::{Quat, Vec3};
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

/// Camera configuration for an epoch
///
/// Specifies optional camera settings that should be applied when transitioning
/// to this epoch. All fields are optional to allow epochs to specify only the
/// aspects they care about (e.g., position only, mode only, or both).
///
/// # Crossfade Integration
///
/// The `fade_duration` field controls the timing of both the visual fade effect
/// and the camera interpolation. When an epoch transition occurs:
///
/// 1. The fade effect begins, fading to white over `fade_duration` seconds
/// 2. Camera interpolation starts, moving the camera to the target position
/// 3. At `fade_duration`, the fade reaches maximum opacity (white screen)
/// 4. The fade then reverses, fading in from white over another `fade_duration` seconds
/// 5. Camera interpolation completes exactly when the fade ends (total: `2 × fade_duration`)
///
/// This synchronization ensures smooth transitions where the camera movement
/// is hidden behind the fade effect, creating a professional-looking transition
/// between epochs.
#[derive(Debug, Clone, PartialEq)]
pub struct EpochCameraConfig {
    /// Target position for the camera
    ///
    /// If set, the camera will interpolate to this position during the epoch transition.
    /// If None, the camera position remains unchanged.
    pub target_position: Option<Vec3>,

    /// Target rotation for the camera
    ///
    /// If set, the camera will interpolate to this orientation during the epoch transition.
    /// If None, the camera rotation remains unchanged.
    pub target_rotation: Option<Quat>,

    /// Target camera mode
    ///
    /// If set, the camera will switch to this mode during the epoch transition.
    /// If None, the camera mode remains unchanged.
    pub target_mode: Option<CameraMode>,

    /// Duration for one phase of the fade effect
    ///
    /// Specifies the duration for ONE phase of the fade (either fade out or fade in).
    /// The total fade sequence duration will be 2 × fade_duration.
    ///
    /// # Synchronization with Camera Interpolation
    ///
    /// The fade duration is directly linked to camera interpolation timing:
    /// - Camera interpolation duration is calculated as: `2 × fade_duration`
    /// - This ensures the camera movement spans the entire fade sequence
    /// - A longer fade_duration results in both a slower fade and smoother camera movement
    ///
    /// # Default Behavior
    ///
    /// If None, a default value of 0.75 seconds per phase (1.5 seconds total) is used.
    /// This default is applied consistently across all epochs unless overridden.
    ///
    /// # Configuration Examples
    ///
    /// ```ignore
    /// // Quick transition (0.5s per phase = 1.0s total)
    /// fade_duration: Some(0.5)
    ///
    /// // Default transition (0.75s per phase = 1.5s total)
    /// fade_duration: None  // or Some(0.75)
    ///
    /// // Slow, cinematic transition (1.5s per phase = 3.0s total)
    /// fade_duration: Some(1.5)
    /// ```
    pub fade_duration: Option<f32>,
}

impl Default for EpochCameraConfig {
    fn default() -> Self {
        Self {
            target_position: None,
            target_rotation: None,
            target_mode: None,
            fade_duration: None,
        }
    }
}

impl EpochCameraConfig {
    /// Creates a new EpochCameraConfig with all fields set to None
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an EpochCameraConfig with specified fade duration
    ///
    /// Sets the duration for one phase of the fade effect (fade out or fade in).
    /// The total fade sequence will be 2 × duration.
    #[allow(dead_code)]
    pub fn with_fade_duration(mut self, duration: f32) -> Self {
        self.fade_duration = Some(duration);
        self
    }

    /// Creates an EpochCameraConfig with specified position
    #[allow(dead_code)]
    pub fn with_position(position: Vec3) -> Self {
        Self {
            target_position: Some(position),
            target_rotation: None,
            target_mode: None,
            fade_duration: None,
        }
    }

    /// Creates an EpochCameraConfig with specified rotation
    #[allow(dead_code)]
    pub fn with_rotation(rotation: Quat) -> Self {
        Self {
            target_position: None,
            target_rotation: Some(rotation),
            target_mode: None,
            fade_duration: None,
        }
    }

    /// Creates an EpochCameraConfig with specified mode
    #[allow(dead_code)]
    pub fn with_mode(mode: CameraMode) -> Self {
        Self {
            target_position: None,
            target_rotation: None,
            target_mode: Some(mode),
            fade_duration: None,
        }
    }
}
