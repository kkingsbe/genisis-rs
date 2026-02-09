//! Camera mode definitions and control systems
//!
//! Defines camera mode enums, state tracking resources, and camera controller components.
//! Camera movement systems are implemented for both free-flight (update_free_flight_camera)
//! and orbit (update_orbit_camera) modes.

use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::{MouseButton, MouseWheel};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::time::Time;

use crate::input::InputState;
use genesis_core::config::CameraConfig;

/// Camera mode enumeration
///
/// Defines the available camera control modes for the simulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraMode {
    /// Free-flight camera mode with WASD movement and mouse look
    #[default]
    FreeFlight,
    /// Orbit camera mode that rotates around a target point
    Orbit,
}

/// Resource tracking camera state
///
/// Stores the current camera mode, optional target point, and current orbit target.
/// Camera movement systems are implemented for both free-flight and orbit modes.
/// Includes interpolation state for smooth camera transitions.
#[derive(Resource)]
pub struct CameraState {
    pub mode: CameraMode,
    pub target: Option<Vec3>,
    /// Current orbit target point for orbit camera mode
    pub current_orbit_target: Vec3,
    /// Target position for interpolation
    pub target_position: Option<Vec3>,
    /// Target rotation for interpolation
    pub target_rotation: Option<Quat>,
    /// Speed of interpolation
    pub interpolation_speed: f32,
    /// Tracks whether camera is currently interpolating
    pub is_interpolating: bool,
    /// Starting position for interpolation
    pub interpolation_start_pos: Vec3,
    /// Target position for interpolation
    pub interpolation_end_pos: Vec3,
    /// Starting rotation (quaternion) for interpolation
    pub interpolation_start_rot: Quat,
    /// Target rotation (quaternion) for interpolation
    pub interpolation_end_rot: Quat,
    /// Progress value from 0.0 to 1.0
    pub interpolation_progress: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            mode: CameraMode::default(),
            target: None,
            current_orbit_target: Vec3::ZERO,
            target_position: None,
            target_rotation: None,
            interpolation_speed: 2.0,
            is_interpolating: false,
            interpolation_start_pos: Vec3::ZERO,
            interpolation_end_pos: Vec3::ZERO,
            interpolation_start_rot: Quat::IDENTITY,
            interpolation_end_rot: Quat::IDENTITY,
            interpolation_progress: 0.0,
        }
    }
}

impl CameraState {
    /// Start camera interpolation to a target position and rotation.
    ///
    /// # Parameters
    /// * `target_pos` - The destination position in world space
    /// * `target_rot` - The destination rotation as a quaternion
    /// * `duration` - Duration of the interpolation in seconds (default: 1.0)
    /// * `current_transform` - The current camera transform to capture start position/rotation
    ///
    /// # Behavior
    /// Captures the current position and rotation from `current_transform`,
    /// sets them as the start points, and configures the target position/rotation.
    /// The `is_interpolating` flag is set to true, allowing the `interpolate_camera`
    /// system to smoothly transition the camera over the specified duration.
    pub fn start_interpolation_to_target(
        &mut self,
        target_pos: Vec3,
        target_rot: Quat,
        duration: f32,
        current_transform: &Transform,
    ) {
        self.interpolation_start_pos = current_transform.translation;
        self.interpolation_end_pos = target_pos;
        self.interpolation_start_rot = current_transform.rotation;
        self.interpolation_end_rot = target_rot;
        // Convert duration to speed (speed = 1.0 / duration)
        self.interpolation_speed = 1.0 / duration;
        self.interpolation_progress = 0.0;
        self.is_interpolating = true;
    }

    /// Start camera interpolation to a target position only (preserves current rotation).
    ///
    /// # Parameters
    /// * `target_pos` - The destination position in world space
    /// * `duration` - Duration of the interpolation in seconds (default: 1.0)
    /// * `current_transform` - The current camera transform
    ///
    /// # Behavior
    /// Convenience method that interpolates only the camera position while
    /// maintaining the current rotation. Internally calls `start_interpolation_to_target`
    /// with the current rotation from `current_transform`.
    pub fn start_interpolation_to_position_only(
        &mut self,
        target_pos: Vec3,
        duration: f32,
        current_transform: &Transform,
    ) {
        self.start_interpolation_to_target(
            target_pos,
            current_transform.rotation,
            duration,
            current_transform,
        );
    }

    /// Creates a CameraState from a CameraConfig.
    ///
    /// # Parameters
    /// * `config` - The camera configuration containing initial camera mode
    ///
    /// # Behavior
    /// Initializes the camera state with the mode specified in the configuration.
    /// All other fields are set to their default values. The orbit_distance from
    /// config is used to initialize the OrbitController in setup_camera().
    pub fn from_config(config: &CameraConfig) -> Self {
        let mode = match config.camera_mode.as_str() {
            "free" | "free_flight" | "FreeFlight" => CameraMode::FreeFlight,
            "orbit" | "Orbit" => CameraMode::Orbit,
            _ => CameraMode::default(),
        };
        Self {
            mode,
            ..Default::default()
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

/// Component controlling orbit camera behavior
///
/// Attached to camera entities to enable orbital movement around a target point.
/// Stores spherical coordinates (distance, yaw, pitch) relative to the target.
#[derive(Component, Debug)]
pub struct OrbitController {
    /// Distance from orbit target in world units
    pub distance: f32,
    /// Horizontal rotation angle in radians around Y axis
    pub yaw: f32,
    /// Vertical rotation angle in radians
    pub pitch: f32,
    /// The point the camera orbits around
    pub target: Vec3,
    /// Minimum zoom distance
    pub min_distance: f32,
    /// Maximum zoom distance
    pub max_distance: f32,
    /// Mouse drag sensitivity
    pub rotation_sensitivity: f32,
    /// Scroll wheel zoom sensitivity
    pub zoom_sensitivity: f32,
    /// Middle mouse button pan sensitivity
    pub pan_sensitivity: f32,
}

impl Default for OrbitController {
    fn default() -> Self {
        Self {
            distance: 50.0,
            yaw: 0.0,
            pitch: 0.3,
            target: Vec3::ZERO,
            min_distance: 5.0,
            max_distance: 200.0,
            rotation_sensitivity: 0.005,
            zoom_sensitivity: 0.1,
            pan_sensitivity: 0.5,
        }
    }
}

impl OrbitController {
    /// Returns the camera position in world space based on spherical coordinates
    ///
    /// Converts spherical coordinates (distance, yaw, pitch) to Cartesian coordinates
    /// relative to the target point.
    pub fn calculate_position(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.distance * self.pitch.cos() * self.yaw.sin(),
            self.target.y + self.distance * self.pitch.sin(),
            self.target.z + self.distance * self.pitch.cos() * self.yaw.cos(),
        )
    }
}

/// System to update free-flight camera based on input
///
/// Updates camera orientation from mouse movement and applies WASD movement
/// relative to the camera's current facing direction.
///
/// Note: This system runs for all cameras with CameraController, regardless of
/// the current CameraState.mode. Both controllers are present on the camera entity
/// and the appropriate system handles movement based on mode.
///
/// If camera is currently interpolating, this system returns early to prevent
/// user input from interfering with the interpolation.
pub fn update_free_flight_camera(
    mut cameras: Query<(&mut Transform, &mut CameraController)>,
    input: Res<InputState>,
    time: Res<Time>,
    camera_state: Res<CameraState>,
) {
    // Skip manual control during interpolation
    if camera_state.is_interpolating {
        return;
    }

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

/// System to update orbit camera based on input
///
/// Updates camera orientation from mouse drag movement and maintains
/// orbital position around a target point.
///
/// Note: This system runs for all cameras with OrbitController, but only
/// applies rotation when the left mouse button is pressed. Both controllers
/// are present on the camera entity for seamless mode switching.
///
/// If camera is currently interpolating, this system returns early to prevent
/// user input from interfering with the interpolation.
pub fn update_orbit_camera(
    mut cameras: Query<(&mut Transform, &mut OrbitController)>,
    input: Res<InputState>,
    mut camera_state: ResMut<CameraState>,
) {
    // Skip manual control during interpolation
    if camera_state.is_interpolating {
        return;
    }

    // Only update if left mouse button is pressed
    if !input
        .mouse_buttons
        .get(&MouseButton::Left)
        .copied()
        .unwrap_or(false)
    {
        return;
    }

    for (mut transform, mut controller) in cameras.iter_mut() {
        // Apply mouse drag rotation
        controller.yaw += input.mouse_delta.x * controller.rotation_sensitivity;
        controller.pitch += input.mouse_delta.y * controller.rotation_sensitivity;

        // Clamp pitch to avoid gimbal lock at +/- 89 degrees
        controller.pitch = controller.pitch.clamp(-1.55, 1.55);

        // Calculate new position using spherical coordinates
        let new_position = controller.calculate_position();
        transform.translation = new_position;

        // Make camera look at its target
        transform.look_at(controller.target, Vec3::Y);

        // Update CameraState.current_orbit_target to match OrbitController.target
        camera_state.current_orbit_target = controller.target;
    }
}

/// System to handle orbit camera zoom via scroll wheel
///
/// Adjusts camera distance based on scroll wheel input, clamped between
/// min_distance and max_distance.
fn handle_orbit_zoom(
    mut scroll_events: EventReader<MouseWheel>,
    mut orbit_query: Query<&mut OrbitController>,
) {
    let mut orbit = orbit_query.single_mut();
    for event in scroll_events.read() {
        orbit.distance -= event.y * orbit.zoom_sensitivity;
        orbit.distance = orbit.distance.clamp(orbit.min_distance, orbit.max_distance);
    }
}

/// System to handle orbit camera pan via middle or right mouse button
///
/// Moves the orbit target based on mouse drag when middle or right mouse button
/// is pressed. Pans horizontally and vertically relative to camera view.
fn handle_orbit_pan(
    mut orbit_q: Query<&mut OrbitController>,
    input: Res<InputState>,
) {
    // Check if middle or right mouse button is pressed
    let middle_pressed = input.mouse_buttons.get(&MouseButton::Middle).copied().unwrap_or(false);
    let right_pressed = input.mouse_buttons.get(&MouseButton::Right).copied().unwrap_or(false);

    if !middle_pressed && !right_pressed {
        return;
    }

    let mut orbit = orbit_q.single_mut();

    // Calculate right vector from yaw (projected onto XZ plane)
    let right = Vec3::new(orbit.yaw.cos(), 0.0, -orbit.yaw.sin()).normalize();

    // Calculate pan movement
    let scale = orbit.pan_sensitivity * 0.01;
    let pan_right = right * (input.mouse_delta.x * scale);
    let pan_up = Vec3::Y * (input.mouse_delta.y * scale);

    // Update target (subtract because we pan opposite to drag direction)
    orbit.target.x -= pan_right.x + pan_up.x;
    orbit.target.y -= pan_right.y + pan_up.y;
    orbit.target.z -= pan_right.z + pan_up.z;
}

/// System to handle camera interpolation
///
/// Updates camera position and rotation smoothly between start and end states
/// when CameraState.is_interpolating is true. Uses smoothstep easing for natural
/// motion and applies the interpolated transform to all 3D cameras.
///
/// This system runs after other camera movement systems, so interpolation
/// takes precedence when active.
fn interpolate_camera(
    mut camera_state: ResMut<CameraState>,
    time: Res<Time>,
    mut query: Query<&mut Transform>,
) {
    // Return early if not currently interpolating
    if !camera_state.is_interpolating {
        return;
    }

    // Update interpolation progress
    camera_state.interpolation_progress += time.delta_secs() * camera_state.interpolation_speed;

    // Check if interpolation is complete
    if camera_state.interpolation_progress >= 1.0 {
        // Clamp progress to 1.0
        camera_state.interpolation_progress = 1.0;
        camera_state.is_interpolating = false;

        // Apply final position and rotation to all cameras
        for mut transform in query.iter_mut() {
            transform.translation = camera_state.interpolation_end_pos;
            transform.rotation = camera_state.interpolation_end_rot;
        }
    } else {
        // Still interpolating - apply eased interpolation
        let t = camera_state.interpolation_progress;

        // Calculate eased progress using smoothstep: t * t * (3.0 - 2.0 * t)
        let eased_progress = t * t * (3.0 - 2.0 * t);

        // Interpolate position
        let interpolated_pos = camera_state
            .interpolation_start_pos
            .lerp(camera_state.interpolation_end_pos, eased_progress);

        // Interpolate rotation using spherical interpolation
        let interpolated_rot = camera_state
            .interpolation_start_rot
            .slerp(camera_state.interpolation_end_rot, eased_progress);

        // Apply interpolated position and rotation to all cameras
        for mut transform in query.iter_mut() {
            transform.translation = interpolated_pos;
            transform.rotation = interpolated_rot;
        }
    }
}

/// System to toggle between camera modes
///
/// Switches between FreeFlight and Orbit camera modes when the 'O' key is pressed.
///
/// Note: Both CameraController and OrbitController are always present on the camera entity.
/// This function only updates the CameraState.mode field. The actual camera behavior is
/// determined by which controller system responds to input - free-flight responds to WASD
/// regardless of mode, while orbit only responds when left mouse is pressed.
///
/// Both mode switches are instant for responsive user control.
fn toggle_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_state: ResMut<CameraState>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    if keys.just_pressed(KeyCode::KeyO) {
        match camera_state.mode {
            CameraMode::FreeFlight => {
                // Switching FROM FreeFlight TO Orbit - instant switch for now
                camera_state.mode = CameraMode::Orbit;

                // Set orbit target to a point in front of the camera
                if let Ok(camera_transform) = camera_query.get_single() {
                    let forward = camera_transform.forward();
                    camera_state.current_orbit_target =
                        camera_transform.translation + forward * 10.0;
                }

                info!("Camera mode switched to: Orbit (instant)");
            }
            CameraMode::Orbit => {
                // Switching from Orbit to FreeFlight - instant switch
                camera_state.mode = CameraMode::FreeFlight;
                info!("Camera mode switched to: FreeFlight");
            }
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraState>()
            .add_systems(Update, toggle_camera_mode)
            .add_systems(Update, update_free_flight_camera)
            .add_systems(Update, update_orbit_camera)
            .add_systems(Update, handle_orbit_zoom)
            .add_systems(Update, handle_orbit_pan)
            .add_systems(PostUpdate, interpolate_camera);
    }
}
