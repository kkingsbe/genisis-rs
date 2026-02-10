//! Input state tracking and handling systems
//!
//! Defines resource for tracking input device state (keyboard and mouse).
//! Input handling systems (handle_keyboard_input, handle_mouse_input) are implemented
//! and run in the PreUpdate schedule to ensure input is available before other systems.
//!
//! # Input Systems
//!
//! - `handle_keyboard_input` (PreUpdate): Maps WASD keys to directional vector
//! - `handle_mouse_input` (PreUpdate): Tracks mouse motion delta, button states, and scroll wheel
//!
//! # Input Limitations
//!
//! - **Keyboard**: Only WASD keys are mapped (W, A, S, D)
//! - **Mouse**: Only left mouse button state is tracked (used for orbit rotation)
//! - **Middle/right mouse**: NOT tracked (orbit pan not implemented)

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use std::collections::HashMap;

/// Resource tracking the current state of input devices
#[derive(Resource, Default)]
pub struct InputState {
    /// Direction vector from WASD keyboard input
    pub keyboard_direction: Vec3,
    /// Mouse movement delta since last frame
    pub mouse_delta: Vec2,
    /// State of mouse buttons (pressed = true, released = false)
    pub mouse_buttons: HashMap<MouseButton, bool>,
    /// Scroll wheel delta (y-axis) since last frame
    pub scroll_delta: f32,
}

/// System to handle keyboard input and update InputState
///
/// This system runs in the PreUpdate schedule and maps WASD key presses
/// to a directional vector stored in InputState.keyboard_direction.
///
/// # Key Mappings
///
/// - **W**: Forward (negative Z direction)
/// - **S**: Backward (positive Z direction)
/// - **A**: Left (negative X direction)
/// - **D**: Right (positive X direction)
///
/// The resulting direction vector is normalized to ensure consistent
/// movement speed regardless of diagonal movement.
pub fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input_state: ResMut<InputState>,
) {
    // Reset direction to zero
    input_state.keyboard_direction = Vec3::ZERO;

    // Handle WASD movement
    if keyboard.pressed(KeyCode::KeyW) {
        input_state.keyboard_direction += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard.pressed(KeyCode::KeyS) {
        input_state.keyboard_direction += Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard.pressed(KeyCode::KeyA) {
        input_state.keyboard_direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard.pressed(KeyCode::KeyD) {
        input_state.keyboard_direction += Vec3::new(1.0, 0.0, 0.0);
    }

    // Normalize direction if not zero
    if input_state.keyboard_direction != Vec3::ZERO {
        input_state.keyboard_direction = input_state.keyboard_direction.normalize();
    }
}

/// System to handle mouse input and update InputState
pub fn handle_mouse_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<Events<MouseMotion>>,
    mouse_wheel: Res<Events<MouseWheel>>,
    mut input_state: ResMut<InputState>,
) {
    // Clear previous mouse button states
    input_state.mouse_buttons.clear();

    // Update mouse button state (Left only - used for orbit camera rotation)
    input_state
        .mouse_buttons
        .insert(MouseButton::Left, mouse_buttons.pressed(MouseButton::Left));

    // Reset mouse delta
    input_state.mouse_delta = Vec2::ZERO;

    // Reset scroll delta
    input_state.scroll_delta = 0.0;

    // Process mouse motion events
    let mut reader = mouse_motion.get_cursor();
    for event in reader.read(&mouse_motion) {
        input_state.mouse_delta.x += event.delta.x;
        input_state.mouse_delta.y += event.delta.y;
    }

    // Process scroll wheel events
    let mut reader = mouse_wheel.get_cursor();
    for event in reader.read(&mouse_wheel) {
        input_state.scroll_delta += event.y;
    }
}

/// Plugin to register input systems and resources
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_systems(PreUpdate, handle_keyboard_input)
            .add_systems(PreUpdate, handle_mouse_input);
    }
}
