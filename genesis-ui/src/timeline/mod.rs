//! Timeline scrubber and time controls
//!
//! UI widgets for controlling cosmic time flow, including
//! logarithmic timeline scrubber and playback controls.

use bevy::prelude::*;

/// Resource tracking playback state
#[derive(Resource, Default)]
pub struct PlaybackState {
    pub playing: bool,
    pub speed: f32, // 0.0 to 1.0 for logarithmic mapping
}
