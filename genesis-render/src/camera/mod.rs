//! Camera mode definitions
//!
//! Defines camera mode enums and state tracking resources.
//! Actual camera movement and input handling systems are not yet implemented.

use bevy::prelude::*;
use bevy::time::Time;

use crate::input::InputState;

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

/// Component controlling free-flight camera behavior
///
/// Attached to camera entities to enable WASD movement and mouse look.
/// Stores rotation angles and movement parameters for the camera.
#[derive(Component, Debug)]
pub struct CameraController {
    /// Horizontal rotation angle (radians)
    pub yaw: f32,
    /// Vertical rotation angle (radians), clamped to +/- 89 degrees
    pub pitch: f32,
    /// Movement speed for WASD translation (units per second)
    pub movement_speed: f32,
    /// Mouse look sensitivity (radians per pixel)
    pub mouse_sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            movement_speed: 10.0,
            mouse_sensitivity: 0.002,
        }
    }
}

impl CameraController {
    /// Calculate forward direction from yaw and pitch
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    /// Calculate right direction (perpendicular to forward and world up)
    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }
}

/// System to update free-flight camera based on input
///
/// Updates camera orientation from mouse movement and applies WASD movement
/// relative to the camera's current facing direction.
pub fn update_free_flight_camera(
    mut cameras: Query<(&mut Transform, &mut CameraController)>,
    input: Res<InputState>,
    time: Res<Time>,
) {
    for (mut transform, mut controller) in cameras.iter_mut() {
        // Apply mouse look
        controller.yaw -= input.mouse_delta.x * controller.mouse_sensitivity;
        controller.pitch -= input.mouse_delta.y * controller.mouse_sensitivity;
        
        // Clamp pitch to avoid gimbal lock at +/- 90 degrees
        controller.pitch = controller.pitch.clamp(-1.55, 1.55);
        
        // Calculate direction vectors from orientation
        let forward = controller.forward();
        let right = controller.right();
        let up = Vec3::Y;
        
        // Apply WASD movement relative to camera direction
        let movement = (forward * input.keyboard_direction.z) 
            + (right * input.keyboard_direction.x)
            + (up * input.keyboard_direction.y);
        
        if movement.length_squared() > 0.0 {
            transform.translation += movement * controller.movement_speed * time.delta_secs();
        }
        
        // Update camera rotation to look in the direction of movement
        let target = transform.translation + forward;
        transform.look_at(target, Vec3::Y);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_free_flight_camera);
    }
}
