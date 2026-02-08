//! Timeline playback state
//!
//! Defines resource for tracking playback state and speed.
//! Actual timeline UI widgets and controls are not yet implemented.

use bevy::prelude::*;

/// Resource tracking playback state
///
/// Stores playing state and speed factor for time control.
/// Timeline UI systems need to be implemented separately.
#[derive(Resource, Default)]
pub struct PlaybackState {
    pub playing: bool,
    pub speed: f32, // 0.0 to 1.0 for logarithmic mapping (when implemented)
}
