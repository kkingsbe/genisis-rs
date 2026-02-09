//! Input state tracking and handling systems
//!
//! Defines resource for tracking input device state (keyboard and mouse).
//! Input handling systems (handle_keyboard_input, handle_mouse_input) are implemented
//! and run in the PreUpdate schedule to ensure input is available before other systems.

use bevy::input::mouse::MouseMotion;
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
}

/// System to handle keyboard input and update InputState
pub fn handle_keyboard_input(keyboard: Res<ButtonInput<KeyCode>>, mut input_state: ResMut<InputState>) {
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
    mut input_state: ResMut<InputState>,
) {
    // Clear previous mouse button states
    input_state.mouse_buttons.clear();

    // Update mouse button states
    for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
        input_state.mouse_buttons.insert(button, mouse_buttons.pressed(button));
    }

    // Reset mouse delta
    input_state.mouse_delta = Vec2::ZERO;

    // Process mouse motion events
    let mut reader = mouse_motion.get_cursor();
    for event in reader.read(&mouse_motion) {
        input_state.mouse_delta.x += event.delta.x;
        input_state.mouse_delta.y += event.delta.y;
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
