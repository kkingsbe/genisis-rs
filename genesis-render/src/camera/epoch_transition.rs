//! Epoch transition system for camera interpolation
//!
//! Defines the system that handles epoch change events and triggers camera
//! transitions to the new epoch's camera configuration.
//!
//! # Crossfade Synchronization
//!
//! This system ensures that camera interpolation is properly synchronized with
//! the fade effect during epoch transitions. The key synchronization rule is:
//!
//! ```text
//! camera_interpolation_duration = 2 × fade_duration
//! ```
//!
//! Where:
//! - `fade_duration` is the duration for one phase of the fade (fade out or fade in)
//! - `camera_interpolation_duration` spans the entire fade sequence (fade out + fade in)
//!
//! # Transition Timeline
//!
//! When an epoch change occurs:
//!
//! 1. **Fade Out Phase (0 to fade_duration)**: The screen fades to white
//!    - Camera is moving toward target position
//!    - Movement is partially hidden by the increasing opacity
//!
//! 2. **Midpoint (fade_duration)**: Screen is fully white
//!    - Camera is at the midpoint of its trajectory
//!    - This is the ideal moment for visual cuts or scene changes
//!
//! 3. **Fade In Phase (fade_duration to 2×fade_duration)**: Screen fades from white
//!    - Camera continues moving to target position
//!    - Movement is revealed as opacity decreases
//!
//! 4. **Completion (2×fade_duration)**: Fade and interpolation complete
//!    - Camera is at target position
//!    - Screen is fully transparent, showing the new epoch
//!
//! # Implementation Details
//!
//! The [`handle_epoch_change_transition()`] function retrieves the epoch's
//! [`EpochCameraConfig`](genesis_core::epoch::EpochCameraConfig) and calculates
//! the interpolation duration as `2.0 * fade_duration.unwrap_or(0.75)`.
//! This value is then passed to [`CameraState::start_interpolation_to_target()`].
//!

use bevy::prelude::*;
use genesis_core::epoch::{EpochChangeEvent, EpochManager};

use super::CameraState;

/// System that handles epoch change events and triggers camera transitions
///
/// This system listens for `EpochChangeEvent` events and, when a transition occurs,
/// retrieves the new epoch's camera configuration and initiates camera interpolation
/// to the target position, rotation, and mode specified in the configuration.
///
/// # Crossfade Synchronization
///
/// The camera interpolation is synchronized with the fade effect by calculating
/// the duration based on the epoch's `fade_duration`:
///
/// ```text
/// interpolation_duration = 2.0 × fade_duration
/// ```
///
/// This ensures the camera movement spans the entire fade sequence (fade out + fade in).
/// The fade duration is retrieved from the epoch's [`EpochCameraConfig`](genesis_core::epoch::EpochCameraConfig),
/// with a default of 0.75 seconds if not specified.
///
/// # Synchronization Mechanism
///
/// When an epoch change is detected:
///
/// 1. The fade system receives the `EpochChangeEvent` with the `fade_duration`
/// 2. The fade system starts its fade out sequence
/// 3. This system calculates `interpolation_duration = 2.0 * fade_duration`
/// 4. Camera interpolation starts and will complete in exactly `2 × fade_duration` seconds
/// 5. Both the fade and interpolation complete simultaneously
///
/// # Behavior
///
/// - Reads `EpochChangeEvent` events to detect epoch transitions
/// - Looks up the new epoch's plugin via `EpochManager`
/// - Retrieves the epoch's camera configuration using `plugin.camera_config()`
/// - If `target_position` is set, calls `start_interpolation_to_target()` with
///   the target position, rotation, and the calculated interpolation duration
/// - If `target_mode` is set, updates `camera_state.mode` before starting interpolation
/// - Handles error cases gracefully (e.g., epoch plugin not found)
///
/// # Duration Examples
///
/// - Default transition: `fade_duration = 0.75s` → `interpolation_duration = 1.5s`
/// - Quick transition: `fade_duration = 0.5s` → `interpolation_duration = 1.0s`
/// - Slow transition: `fade_duration = 1.5s` → `interpolation_duration = 3.0s`
pub fn handle_epoch_change_transition(
    epoch_manager: Res<EpochManager>,
    mut events: EventReader<EpochChangeEvent>,
    mut camera_state: ResMut<CameraState>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    for event in events.read() {
        // Get the new epoch name from the event
        let new_epoch_name = &event.to_epoch;

        // Look up the epoch plugin by name
        if let Some(plugin) = epoch_manager.get_epoch_plugin(new_epoch_name) {
            // Get the camera configuration for this epoch
            let config = plugin.camera_config();

            // Update camera mode if specified
            if let Some(mode) = config.target_mode {
                camera_state.mode = mode;
            }

            // Start interpolation if target position is specified
            if let Some(target_position) = config.target_position {
                // Get the current camera transform
                if let Ok(current_transform) = camera_query.get_single() {
                    // Use target_rotation if specified, otherwise use current rotation
                    let target_rotation = config
                        .target_rotation
                        .unwrap_or(current_transform.rotation);

                    // Calculate interpolation duration: must span the entire fade sequence (fade out + fade in)
                    // Camera interpolation duration = 2 × fade_duration
                    //
                    // This synchronization ensures that:
                    // 1. Camera movement starts exactly when the fade begins
                    // 2. Camera movement completes exactly when the fade ends
                    // 3. The fade effect hides the visual discontinuity of camera repositioning
                    let interpolation_duration = 2.0 * config.fade_duration.unwrap_or(0.75);

                    // Start interpolation with the calculated duration
                    camera_state.start_interpolation_to_target(
                        target_position,
                        target_rotation,
                        interpolation_duration,
                        current_transform,
                    );
                }
            }
        } else {
            // Epoch plugin not found - log a warning
            warn!(
                "Epoch plugin '{}' not found during epoch change transition",
                new_epoch_name
            );
        }
    }
}
