//! Camera fade effect for epoch transitions
//!
//! Implements a visual crossfade effect that fades to white during epoch changes,
//! providing a smooth visual experience when transitioning between cosmological epochs.
//!
//! # Crossfade Synchronization
//!
//! The fade system is synchronized with camera interpolation through a shared
//! duration parameter passed via [`EpochChangeEvent`](genesis_core::epoch::EpochChangeEvent).
//! The synchronization ensures that:
//!
//! - Both the fade effect and camera interpolation start simultaneously
//! - The fade effect completes exactly when camera interpolation finishes
//! - The camera movement is visually hidden by the white screen at the fade's midpoint
//!
//! # Fade Duration
//!
//! The fade system receives `fade_duration` from the [`EpochChangeEvent`](genesis_core::epoch::EpochChangeEvent),
//! which contains the epoch-specific fade duration configured in
//! [`EpochCameraConfig`](genesis_core::epoch::EpochCameraConfig). This duration specifies
//! the time for ONE phase of the fade (either fade out or fade in).
//!
//! # Transition Timeline
//!
//! ```text
//! Time: 0                fade_duration      2 × fade_duration
//!      |-------------------|-------------------|
//!      Fade Out Phase     Fade In Phase
//!      (to white)         (from white)
//!      Camera moving...   Camera moving...
//!
//! Camera interpolation:  |===================================|
//!                         Total: 2 × fade_duration
//! ```
//!
//! # Default Behavior
//!
//! If the epoch doesn't specify a `fade_duration`, a default of 0.75 seconds
//! per phase is used, resulting in a 1.5-second total transition time.

use bevy::prelude::*;
use genesis_core::epoch::EpochChangeEvent;

/// Fade state for the camera transition effect
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FadeState {
    /// Not fading - fully transparent
    None,
    /// Fading out to white - opacity increasing from 0.0 to 1.0
    FadingOut,
    /// Fading back in from white - opacity decreasing from 1.0 to 0.0
    FadingIn,
}

/// Resource tracking the camera fade state during epoch transitions
#[derive(Resource)]
pub struct CameraFadeState {
    /// Current opacity (0.0 = fully transparent, 1.0 = fully opaque white)
    pub opacity: f32,
    /// Current fade state
    pub state: FadeState,
    /// Duration for each fade phase in seconds (default 0.75 seconds)
    ///
    /// This field is updated from the [`EpochChangeEvent`](genesis_core::epoch::EpochChangeEvent)
    /// when an epoch transition occurs. It contains the per-epoch `fade_duration`
    /// from the [`EpochCameraConfig`](genesis_core::epoch::EpochCameraConfig).
    ///
    /// # Synchronization Note
    ///
    /// This `fade_duration` is the duration for ONE phase (fade out OR fade in).
    /// The camera interpolation system uses `2 × fade_duration` to ensure the
    /// camera movement spans the entire fade sequence.

impl Default for CameraFadeState {
    fn default() -> Self {
        Self {
            opacity: 0.0,
            state: FadeState::None,
            fade_duration: 0.75,
        }
    }
}

/// Marker component to identify the camera fade overlay entity
#[derive(Component)]
pub struct CameraFade;

/// Startup system that creates the full-screen fade overlay
///
/// Spawns a white UI node that covers the entire screen and is initially
/// fully transparent. This overlay is used to create the fade effect during
/// epoch transitions.
pub fn setup_camera_fade_overlay(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                z_index: ZIndex::Global(i32::MAX),
                ..default()
            },
            background_color: Color::WHITE.with_a(0.0),
            ..default()
        },
        CameraFade,
    ));
}

/// System that updates the camera fade effect
///
/// Responds to [`EpochChangeEvent`](genesis_core::epoch::EpochChangeEvent) by starting a fade out to white,
/// then fades back in. Uses smoothstep easing for natural-looking transitions.
///
/// # Synchronization with Camera Interpolation
///
/// This system receives the `fade_duration` from the [`EpochChangeEvent`](genesis_core::epoch::EpochChangeEvent),
/// which is extracted from the epoch's [`EpochCameraConfig`](genesis_core::epoch::EpochCameraConfig).
/// The fade duration specifies the time for ONE phase of the fade:
///
/// ```text
/// Fade Out: 0 → fade_duration seconds (opacity: 0.0 → 1.0)
/// Fade In:  fade_duration → 2×fade_duration seconds (opacity: 1.0 → 0.0)
/// ```
///
/// The camera interpolation system (in `genesis-render`) uses `2 × fade_duration`
/// for its interpolation, ensuring both systems complete simultaneously.
///
/// # Event Handling
///
/// When an [`EpochChangeEvent`](genesis_core::epoch::EpochChangeEvent) is received:
///
/// 1. Extract `fade_duration` from the event (defaults to 0.75 if None)
/// 2. Store it in `CameraFadeState.fade_duration`
/// 3. Set state to `FadeState::FadingOut` with opacity starting at 0.0
/// 4. The fade animation proceeds automatically in subsequent frames
pub fn update_camera_fade(
    mut epoch_change_events: EventReader<EpochChangeEvent>,
    mut fade_state: ResMut<CameraFadeState>,
    mut fade_query: Query<&mut BackgroundColor, With<CameraFade>>,
    time: Res<Time>,
) {
    // Start fade on epoch change event
    for event in epoch_change_events.read() {
        // Update fade duration from the event (use epoch-specific or default to 0.75)
        //
        // This fade_duration is synchronized with camera interpolation:
        // - Camera interpolation duration = 2 × this fade_duration
        // - Both systems receive the same value from EpochCameraConfig
        // - Both start simultaneously and complete simultaneously
        fade_state.fade_duration = event.fade_duration.unwrap_or(0.75);
        fade_state.state = FadeState::FadingOut;
        fade_state.opacity = 0.0;
    }

    // Update fade based on current state
    match fade_state.state {
        FadeState::None => {
            // Not fading - do nothing
        }
        FadeState::FadingOut => {
            // Increase opacity from 0.0 to 1.0
            let delta = time.delta_seconds() / fade_state.fade_duration;
            fade_state.opacity += delta;

            // Clamp and update UI
            if fade_state.opacity >= 1.0 {
                fade_state.opacity = 1.0;
                fade_state.state = FadeState::FadingIn;
            }

            update_fade_overlay(&mut fade_query, fade_state.opacity);
        }
        FadeState::FadingIn => {
            // Decrease opacity from 1.0 to 0.0
            let delta = time.delta_seconds() / fade_state.fade_duration;
            fade_state.opacity -= delta;

            // Clamp and update UI
            if fade_state.opacity <= 0.0 {
                fade_state.opacity = 0.0;
                fade_state.state = FadeState::None;
            }

            update_fade_overlay(&mut fade_query, fade_state.opacity);
        }
    }
}

/// Helper function to update the fade overlay background color with smoothstep easing
///
/// Applies smoothstep easing to make the fade transition more natural.
/// smoothstep(t) = t * t * (3.0 - 2.0 * t)
fn update_fade_overlay(mut fade_query: Query<&mut BackgroundColor, With<CameraFade>>, opacity: f32) {
    if let Ok(mut bg_color) = fade_query.get_single_mut() {
        // Apply smoothstep easing for natural fade
        let eased_opacity = opacity * opacity * (3.0 - 2.0 * opacity);
        bg_color.0 = Color::WHITE.with_a(eased_opacity);
    }
}
