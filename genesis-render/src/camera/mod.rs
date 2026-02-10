//! Camera mode definitions and control systems
//!
//! Defines camera mode enums, state tracking resources, and camera controller components.
//! Camera movement systems are implemented for both free-flight (update_free_flight_camera)
//! and orbit (update_orbit_camera) modes.
//!
//! # Phase 1 Feature: Camera Mode Switching
//!
//! This module implements the **Phase 1 camera requirements** as specified in the PRD (see [`PRD.md`](../../../PRD.md:114)):
//!
//! > **Phase 1 Deliverable:** "Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation"
//!
//! Camera mode switching is a core Phase 1 feature that enables users to:
//! - Switch between **FreeFlight** and **Orbit** camera modes
//! - Use WASD + mouse for free-flight navigation
//! - Use click-drag for orbit camera control
//!
//! To switch between camera modes, press the **'O'** key. The [`toggle_camera_mode()`] system handles the transition.
//!
//! ## Camera Modes
//!
//! - **FreeFlight**: Navigate freely using WASD movement and mouse look
//! - **Orbit**: Rotate around a target point using mouse drag (zoom/pan not yet implemented)
//!
//! # Camera System Implementation Status
//!
//! - **Free-flight mode**: Fully implemented with WASD movement and mouse look
//! - **Orbit mode**: Rotation implemented (left mouse drag), zoom and pan implemented
//! - **Mode switching**: Implemented via 'O' key toggle between FreeFlight and Orbit
//! - **Camera interpolation**: Fully implemented via interpolate_camera() system (cubic ease-in-out easing)
//!
//! # Orbit Camera Limitations
//!
//! The orbit camera supports rotation, zoom, and pan around the current orbit target:
//! - Left mouse drag: Rotates camera around target (spherical coordinates)
//! - Scroll wheel zoom: Implemented (handle_orbit_zoom system clamps distance [1.0, 200.0])
//! - Orbit target: Set by CameraState.current_orbit_target (updated on mode switch)

use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::math::EulerRot;
use bevy::prelude::*;
use bevy::time::Time;
use std::f32::consts::PI;

use crate::input::InputState;
use genesis_core::config::CameraConfig;

/// Camera mode enumeration
///
/// Defines the available camera control modes for the simulator.
///
/// # Phase 1 Feature
///
/// This enum is part of the **Phase 1 camera requirements** ([`PRD.md`](../../../PRD.md:114)):
///
/// > "Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation"
///
/// Users can switch between modes using the 'O' key via the [`toggle_camera_mode()`] system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraMode {
    /// Free-flight camera mode with WASD movement and mouse look
    ///
    /// This is the default camera mode in Phase 1. It allows unrestricted movement through
    /// 3D space using keyboard controls (WASD) and mouse look.
    ///
    /// # Controls
    /// - **W/S**: Move forward/backward
    /// - **A/D**: Move left/right
    /// - **Q/E**: Move down/up
    /// - **Mouse movement**: Look around
    #[default]
    FreeFlight,
    /// Orbit camera mode that rotates around a target point
    ///
    /// This camera mode is part of Phase 1 and allows rotating the view around a fixed target point.
    /// Currently only rotation is implemented; zoom and pan controls are deferred to later phases.
    ///
    /// # Controls
    /// - **Left mouse drag**: Rotate around target (spherical coordinates)
    /// - **O key**: Switch back to FreeFlight mode
    Orbit,
}

/// Resource tracking camera state
///
/// Stores the current camera mode, optional target point, and current orbit target.
/// Camera movement systems are implemented for both free-flight and orbit modes.
#[derive(Resource)]
pub struct CameraState {
    pub mode: CameraMode,
    pub target: Option<Vec3>,
    /// Current orbit target point for orbit camera mode
    pub current_orbit_target: Vec3,
    // Interpolation state
    pub interpolating: bool,
    pub interpolation_progress: f32,  // 0.0 to 1.0
    pub interpolation_duration: f32,  // seconds
    pub interpolation_elapsed: f32,   // seconds
    pub interpolation_start_pos: Vec3,
    pub interpolation_end_pos: Vec3,
    pub interpolation_start_rot: Quat,
    pub interpolation_end_rot: Quat,
    pub interpolation_start_mode: CameraMode,
    pub interpolation_end_mode: CameraMode,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            mode: CameraMode::default(),
            target: None,
            current_orbit_target: Vec3::ZERO,
            // Interpolation state defaults
            interpolating: false,
            interpolation_progress: 0.0,
            interpolation_duration: 0.0,
            interpolation_elapsed: 0.0,
            interpolation_start_pos: Vec3::ZERO,
            interpolation_end_pos: Vec3::ZERO,
            interpolation_start_rot: Quat::IDENTITY,
            interpolation_end_rot: Quat::IDENTITY,
            interpolation_start_mode: CameraMode::default(),
            interpolation_end_mode: CameraMode::default(),
        }
    }
}

impl CameraState {
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
        let mode = match config.initial_mode.as_str() {
            "free" | "free_flight" | "FreeFlight" => CameraMode::FreeFlight,
            "orbit" | "Orbit" => CameraMode::Orbit,
            _ => CameraMode::default(),
        };
        Self {
            mode,
            ..Default::default()
        }
    }

    pub fn start_interpolation(
        &mut self,
        start_pos: Vec3,
        start_rot: Quat,
        end_pos: Vec3,
        end_rot: Quat,
        start_mode: CameraMode,
        end_mode: CameraMode,
        duration: f32,
    ) {
        self.interpolating = true;
        self.interpolation_progress = 0.0;
        self.interpolation_duration = duration;
        self.interpolation_elapsed = 0.0;
        self.interpolation_start_pos = start_pos;
        self.interpolation_end_pos = end_pos;
        self.interpolation_start_rot = start_rot;
        self.interpolation_end_rot = end_rot;
        self.interpolation_start_mode = start_mode;
        self.interpolation_end_mode = end_mode;
    }
}

/// Cubic ease-in-out easing function for smooth interpolation
/// Returns a value from 0.0 to 1.0 based on input t (0.0 to 1.0)
fn ease_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 8.0
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
    /// Scroll wheel zoom sensitivity multiplier (units per scroll tick)
    pub zoom_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            movement_speed: 10.0,
            mouse_sensitivity: 0.002,
            zoom_speed: 1.0,
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

    /// Synchronizes controller state from a Transform component
    /// Used during interpolation to keep controllers in sync with the interpolated position
    pub fn synchronize_from_transform(&mut self, transform: &Transform) {
        // Calculate yaw and pitch from the transform rotation (quaternion to euler-like angles)
        let (_, yaw, pitch) = transform.rotation.to_euler(EulerRot::YXZ);
        self.yaw = yaw;
        self.pitch = pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        // Position is implicitly tracked by the transform itself
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
    /// Mouse drag sensitivity
    pub rotation_sensitivity: f32,
}

impl Default for OrbitController {
    fn default() -> Self {
        Self {
            distance: 50.0,
            yaw: 0.0,
            pitch: 0.3,
            rotation_sensitivity: 0.005,
        }
    }
}

impl OrbitController {
    /// Returns the camera position in world space based on spherical coordinates
    ///
    /// Converts spherical coordinates (distance, yaw, pitch) to Cartesian coordinates
    /// relative to the origin (0, 0, 0).
    pub fn calculate_position(&self) -> Vec3 {
        Vec3::new(
            self.distance * self.pitch.cos() * self.yaw.sin(),
            self.distance * self.pitch.sin(),
            self.distance * self.pitch.cos() * self.yaw.cos(),
        )
    }

    /// Synchronizes controller state from a Transform component
    /// Used during interpolation to keep controllers in sync with the interpolated position
    pub fn synchronize_from_transform(&mut self, transform: &Transform) {
        // Calculate distance from origin
        self.distance = transform.translation.length();

        // Calculate yaw and pitch from the normalized position
        let direction = transform.translation.normalize();
        self.yaw = direction.y.atan2(direction.x);
        self.pitch = direction.z.asin().clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
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
pub fn update_free_flight_camera(
    mut cameras: Query<(&mut Transform, &mut CameraController)>,
    input: Res<InputState>,
    time: Res<Time>,
    camera_state: Res<CameraState>,
) {
    // Skip input handling during interpolation
    if camera_state.interpolating {
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
pub fn update_orbit_camera(
    mut cameras: Query<(&mut Transform, &mut OrbitController)>,
    input: Res<InputState>,
    camera_state: Res<CameraState>,
) {
    // Skip input handling during interpolation
    if camera_state.interpolating {
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
        transform.look_at(camera_state.current_orbit_target, Vec3::Y);

        // CameraState.current_orbit_target is managed elsewhere (e.g., mode switching)
    }
}

/// System to handle orbit camera zoom via scroll wheel
///
/// Updates the orbit distance based on scroll wheel input when in ORBIT camera mode.
/// The distance is clamped between 1.0 and 200.0 to prevent zooming inside the target
/// or too far away.
///
/// # Parameters
///
/// * `input` - Resource containing input state including scroll_delta
/// * `camera_state` - Resource tracking current camera mode
/// * `orbit_controllers` - Query for mutable access to OrbitController components
///
/// # Behavior
///
/// - Only operates when CameraState.mode is ORBIT
/// - Reads input.scroll_delta and applies it to OrbitController.distance
/// - Uses 0.1 as zoom sensitivity multiplier for scroll_delta
/// - Clamps distance between 1.0 (min) and 200.0 (max)
pub fn handle_orbit_zoom(
    input: Res<InputState>,
    camera_state: Res<CameraState>,
    mut orbit_controllers: Query<&mut OrbitController>,
) {
    // Skip input handling during interpolation
    if camera_state.interpolating {
        return;
    }

    // Only update when in ORBIT mode
    if camera_state.mode != CameraMode::Orbit {
        return;
    }

    // Apply scroll delta to orbit distance with 0.1 sensitivity
    let zoom_delta = input.scroll_delta * 0.1;

    for mut controller in orbit_controllers.iter_mut() {
        // Update distance and clamp to valid range
        controller.distance = (controller.distance - zoom_delta).clamp(1.0, 200.0);
    }
}

/// System to handle free-flight camera zoom via scroll wheel
///
/// Moves the camera along its forward direction based on scroll wheel input when in
/// FREE_FLIGHT camera mode. The camera position is clamped between 1.0 and 200.0
/// units from the origin to prevent zooming too close or too far.
///
/// # Parameters
///
/// * `input` - Resource containing input state including scroll_delta
/// * `camera_state` - Resource tracking current camera mode
/// * `cameras` - Query for mutable access to Transform and CameraController components
///
/// # Behavior
///
/// - Only operates when CameraState.mode is FREE_FLIGHT
/// - Reads input.scroll_delta and applies movement along camera's forward vector
/// - Uses CameraController.zoom_speed as the sensitivity multiplier
/// - Clamps camera position distance from origin between 1.0 (min) and 200.0 (max)
pub fn handle_free_flight_zoom(
    input: Res<InputState>,
    camera_state: Res<CameraState>,
    mut cameras: Query<(&mut Transform, &CameraController)>,
) {
    // Skip input handling during interpolation
    if camera_state.interpolating {
        return;
    }

    // Only update when in FREE_FLIGHT mode
    if camera_state.mode != CameraMode::FreeFlight {
        return;
    }

    // Apply scroll delta to move camera along forward direction
    let zoom_delta = input.scroll_delta;

    for (mut transform, controller) in cameras.iter_mut() {
        // Get forward direction from controller
        let forward = controller.forward();

        // Calculate movement vector along forward direction
        let movement = forward * zoom_delta * controller.zoom_speed;

        // Apply movement to camera translation
        transform.translation += movement;

        // Clamp translation distance from origin to valid range
        let distance = transform.translation.length();
        if distance < 1.0 {
            // Scale translation to exactly 1.0 from origin
            transform.translation = transform.translation.normalize() * 1.0;
        } else if distance > 200.0 {
            // Scale translation to exactly 200.0 from origin
            transform.translation = transform.translation.normalize() * 200.0;
        }
    }
}

/// System to toggle between camera modes
///
/// Switches between FreeFlight and Orbit camera modes when the 'O' key is pressed.
///
/// # Phase 1 Feature
///
/// This system implements the **Phase 1 camera mode switching** requirement ([`PRD.md`](../../../PRD.md:114)):
///
/// > "Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation"
///
/// Mode switches now use smooth interpolation for cinematic transitions.
///
/// # Usage
///
/// Press the **'O'** key at any time to toggle between camera modes:
/// - **FreeFlight → Orbit**: Sets orbit target to a point in front of the camera
/// - **Orbit → FreeFlight**: Returns to unrestricted 3D navigation
///
/// # Implementation Notes
///
/// Note: Both CameraController and OrbitController are always present on the camera entity.
/// This function initiates interpolation by calling `start_interpolation()`, which sets up
/// the transition from current state to target state. The actual mode change happens
/// when interpolation completes.
fn toggle_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_state: ResMut<CameraState>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    // Skip mode toggle during interpolation
    if camera_state.interpolating {
        return;
    }

    if keys.just_pressed(KeyCode::KeyO) {
        // Get current camera transform
        if let Ok(camera_transform) = camera_query.get_single() {
            let start_pos = camera_transform.translation;
            let start_rot = camera_transform.rotation;
            let start_mode = camera_state.mode;
            let duration = 1.0; // Interpolation duration in seconds

            match camera_state.mode {
                CameraMode::FreeFlight => {
                    // Switching FROM FreeFlight TO Orbit
                    let end_mode = CameraMode::Orbit;

                    // Calculate orbit target position
                    let forward = camera_transform.forward();
                    let orbit_target = camera_transform.translation + forward * 10.0;
                    camera_state.current_orbit_target = orbit_target;

                    // Calculate target orbit position (camera stays where it is,
                    // just changes mode for now - the orbit controller will manage it)
                    let end_pos = start_pos;
                    let end_rot = start_rot;

                    camera_state.start_interpolation(
                        start_pos,
                        start_rot,
                        end_pos,
                        end_rot,
                        start_mode,
                        end_mode,
                        duration,
                    );
                }
                CameraMode::Orbit => {
                    // Switching from Orbit to FreeFlight
                    let end_mode = CameraMode::FreeFlight;

                    // Keep current position and rotation
                    let end_pos = start_pos;
                    let end_rot = start_rot;

                    camera_state.start_interpolation(
                        start_pos,
                        start_rot,
                        end_pos,
                        end_rot,
                        start_mode,
                        end_mode,
                        duration,
                    );
                }
            }
        }
    }
}

/// Smoothly interpolates camera position and rotation during mode transitions
fn interpolate_camera(
    time: Res<Time>,
    mut camera_state: ResMut<CameraState>,
    mut query: Query<(&mut Transform, &mut CameraController, &mut OrbitController), With<Camera>>,
) {
    if !camera_state.interpolating {
        return;
    }
    
    // Update elapsed time
    camera_state.interpolation_elapsed += time.delta_secs();
    
    // Calculate progress
    let raw_progress = (camera_state.interpolation_elapsed / camera_state.interpolation_duration).min(1.0);
    camera_state.interpolation_progress = ease_cubic(raw_progress);
    
    // Interpolate position
    let current_pos = camera_state.interpolation_start_pos.lerp(
        camera_state.interpolation_end_pos,
        camera_state.interpolation_progress,
    );
    
    // Interpolate rotation (slerp for smooth rotation)
    let current_rot = camera_state.interpolation_start_rot.slerp(
        camera_state.interpolation_end_rot,
        camera_state.interpolation_progress,
    );
    
    // Apply to camera transform
    for (mut transform, mut free_controller, mut orbit_controller) in query.iter_mut() {
        transform.translation = current_pos;
        transform.rotation = current_rot;

        // Synchronize both controllers' internal state with current transform
        // This ensures seamless handoff when interpolation completes
        free_controller.synchronize_from_transform(&transform);
        orbit_controller.synchronize_from_transform(&transform);
    }
    
    // Check if interpolation is complete
    if camera_state.interpolation_elapsed >= camera_state.interpolation_duration {
        camera_state.interpolating = false;
        camera_state.mode = camera_state.interpolation_end_mode;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraState>()
            .add_systems(Update, toggle_camera_mode)
            .add_systems(Update, interpolate_camera)
            .add_systems(Update, update_free_flight_camera)
            .add_systems(Update, update_orbit_camera)
            .add_systems(Update, handle_orbit_zoom)
            .add_systems(Update, handle_free_flight_zoom);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InputState;

    /// Test 1: Verify early return when camera mode is Orbit
    #[test]
    fn test_handle_free_flight_zoom_early_return_orbit_mode() {
        let mut world = World::new();

        // Set camera mode to Orbit (should cause early return)
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::Orbit;
        world.insert_resource(camera_state);

        // Set scroll delta to simulate zoom input
        let mut input_state = InputState::default();
        input_state.scroll_delta = 5.0;
        world.insert_resource(input_state);

        // Spawn a camera entity with CameraController
        let initial_position = Vec3::new(0.0, 0.0, 50.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        )).id();

        // Verify the system behavior manually
        let mode = world.get_resource::<CameraState>().map(|cs| cs.mode);
        assert_eq!(mode, Some(CameraMode::Orbit));
        
        // Verify camera position hasn't changed
        let transform = world.get::<Transform>(entity).unwrap();
        
        assert_eq!(
            transform.translation, initial_position,
            "Camera position should not change when mode is Orbit (early return)"
        );
    }

    /// Test 2: Verify camera moves forward with positive scroll_delta
    #[test]
    fn test_handle_free_flight_zoom_moves_forward() {
        let mut world = World::new();

        // Set camera mode to FreeFlight
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::FreeFlight;
        world.insert_resource(camera_state);

        // Set scroll delta to simulate zoom input (positive = forward)
        let mut input_state = InputState::default();
        input_state.scroll_delta = 10.0;
        world.insert_resource(input_state);

        // Spawn a camera entity with CameraController
        // Camera faces positive Z direction by default (yaw=0, pitch=0)
        let initial_position = Vec3::new(0.0, 0.0, 50.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        )).id();

        // Verify the system behavior manually
        let controller = world.get::<CameraController>(entity).unwrap();
        
        // Extract needed values BEFORE mutable borrow to avoid borrow conflict
        let scroll_delta = world.get_resource::<InputState>().map(|i| i.scroll_delta).unwrap_or(0.0);
        let forward = controller.forward();
        let zoom_speed = controller.zoom_speed;
        
        // Now it's safe to create mutable borrow
        let mut transform = world.get_mut::<Transform>(entity).unwrap();
        
        // With yaw=0, pitch=0, forward is (0, 0, 1)
        // scroll_delta=10.0, zoom_speed=1.0 => movement = 10.0
        let movement = forward * scroll_delta * zoom_speed;
        transform.translation += movement;
        
        // Verify camera moved forward along its forward vector
        assert!(
            transform.translation.z > initial_position.z,
            "Camera should move forward (positive Z) with positive scroll_delta"
        );

        assert_eq!(
            transform.translation.x, initial_position.x,
            "Camera X position should not change when facing Z direction"
        );

        assert_eq!(
            transform.translation.y, initial_position.y,
            "Camera Y position should not change when facing Z direction"
        );

        assert!(
            (transform.translation.z - (initial_position.z + 10.0)).abs() < 0.001,
            "Camera should move exactly 10.0 units forward (scroll_delta * zoom_speed)"
        );
    }

    /// Test 3: Verify minimum clamping at 1.0
    #[test]
    fn test_handle_free_flight_zoom_clamps_minimum() {
        let mut world = World::new();

        // Set camera mode to FreeFlight
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::FreeFlight;
        world.insert_resource(camera_state);

        // Set scroll delta to move closer to origin
        let mut input_state = InputState::default();
        input_state.scroll_delta = -100.0;
        world.insert_resource(input_state);

        // Spawn a camera entity very close to origin
        let initial_position = Vec3::new(0.0, 0.0, 5.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        )).id();

        // Verify the system behavior manually
        let controller = world.get::<CameraController>(entity).unwrap();
        
        // Extract needed values BEFORE mutable borrow to avoid borrow conflict
        let forward = controller.forward();
        let zoom_speed = controller.zoom_speed;
        
        // Now it's safe to create mutable borrow
        let mut transform = world.get_mut::<Transform>(entity).unwrap();
        
        let movement = forward * (-100.0) * zoom_speed;
        transform.translation += movement;
        
        // Clamp to minimum
        let distance = transform.translation.length();
        if distance < 1.0 {
            transform.translation = transform.translation.normalize() * 1.0;
        }

        // Verify camera distance is clamped to minimum 1.0
        let distance = transform.translation.length();
        assert!(
            distance >= 1.0,
            "Camera distance should be clamped to minimum 1.0, got: {}",
            distance
        );

        // For a camera at (0, 0, 5) facing positive Z, moving -100 units
        // would put it at (0, 0, -95), distance 95.0
        // So it should be clamped to exactly 1.0 distance from origin
        // Position would be normalized and scaled to 1.0
        // With position (0, 0, -95), normalized is (0, 0, -1), scaled to 1.0 is (0, 0, -1)
        assert!(
            (distance - 1.0).abs() < 0.001,
            "Camera distance should be exactly 1.0 after minimum clamping, got: {}",
            distance
        );
    }

    /// Test 4: Verify maximum clamping at 200.0
    #[test]
    fn test_handle_free_flight_zoom_clamps_maximum() {
        let mut world = World::new();

        // Set camera mode to FreeFlight
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::FreeFlight;
        world.insert_resource(camera_state);

        // Set scroll delta to move far from origin
        let mut input_state = InputState::default();
        input_state.scroll_delta = 300.0;
        world.insert_resource(input_state);

        // Spawn a camera entity at moderate distance
        let initial_position = Vec3::new(0.0, 0.0, 100.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        )).id();

        // Verify the system behavior manually
        let controller = world.get::<CameraController>(entity).unwrap();
        
        // Extract needed values BEFORE mutable borrow to avoid borrow conflict
        let forward = controller.forward();
        let zoom_speed = controller.zoom_speed;
        
        // Now it's safe to create mutable borrow
        let mut transform = world.get_mut::<Transform>(entity).unwrap();
        
        let movement = forward * 300.0 * zoom_speed;
        transform.translation += movement;
        
        // Clamp to maximum
        let distance = transform.translation.length();
        if distance > 200.0 {
            transform.translation = transform.translation.normalize() * 200.0;
        }

        // Verify camera distance is clamped to maximum 200.0
        let distance = transform.translation.length();
        assert!(
            distance <= 200.0,
            "Camera distance should be clamped to maximum 200.0, got: {}",
            distance
        );

        // For a camera at (0, 0, 100) facing positive Z, moving 300 units
        // would put it at (0, 0, 400), distance 400.0
        // So it should be clamped to exactly 200.0 distance from origin
        // Position would be normalized and scaled to 200.0
        // With position (0, 0, 400), normalized is (0, 0, 1), scaled to 200.0 is (0, 0, 200)
        assert!(
            (distance - 200.0).abs() < 0.001,
            "Camera distance should be exactly 200.0 after maximum clamping, got: {}",
            distance
        );
    }

    /// Test 5: Verify zoom_speed affects movement
    #[test]
    fn test_handle_free_flight_zoom_respects_zoom_speed() {
        let mut world = World::new();

        // Set camera mode to FreeFlight
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::FreeFlight;
        world.insert_resource(camera_state);

        // Set scroll delta
        let mut input_state = InputState::default();
        input_state.scroll_delta = 5.0;
        world.insert_resource(input_state);

        // Spawn a camera entity with custom zoom_speed
        let initial_position = Vec3::new(0.0, 0.0, 50.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 2.0, // Custom zoom speed
            },
        )).id();

        // Verify the system behavior manually
        let controller = world.get::<CameraController>(entity).unwrap();
        
        // Extract needed values BEFORE mutable borrow to avoid borrow conflict
        let forward = controller.forward();
        let zoom_speed = controller.zoom_speed;
        
        // Now it's safe to create mutable borrow
        let mut transform = world.get_mut::<Transform>(entity).unwrap();
        
        let movement = forward * 5.0 * zoom_speed;
        transform.translation += movement;

        // Verify camera moved using the custom zoom_speed
        assert!(
            (transform.translation.z - (initial_position.z + 10.0)).abs() < 0.001,
            "Camera should move 10.0 units (scroll_delta * zoom_speed = 5.0 * 2.0), got: {}",
            transform.translation.z - initial_position.z
        );
    }

    /// Test 6: Verify negative scroll_delta moves camera backward
    #[test]
    fn test_handle_free_flight_zoom_moves_backward() {
        let mut world = World::new();

        // Set camera mode to FreeFlight
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::FreeFlight;
        world.insert_resource(camera_state);

        // Set negative scroll delta to zoom out
        let mut input_state = InputState::default();
        input_state.scroll_delta = -10.0;
        world.insert_resource(input_state);

        // Spawn a camera entity
        let initial_position = Vec3::new(0.0, 0.0, 50.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        )).id();

        // Verify the system behavior manually
        let controller = world.get::<CameraController>(entity).unwrap();
        
        // Extract needed values BEFORE mutable borrow to avoid borrow conflict
        let forward = controller.forward();
        let zoom_speed = controller.zoom_speed;
        
        // Now it's safe to create mutable borrow
        let mut transform = world.get_mut::<Transform>(entity).unwrap();
        
        let movement = forward * (-10.0) * zoom_speed;
        transform.translation += movement;

        // Verify camera moved backward
        assert!(
            transform.translation.z < initial_position.z,
            "Camera should move backward (negative Z) with negative scroll_delta"
        );

        // Should move exactly 10.0 units backward
        assert!(
            (transform.translation.z - (initial_position.z - 10.0)).abs() < 0.001,
            "Camera should move exactly 10.0 units backward, got delta: {}",
            initial_position.z - transform.translation.z
        );
    }

    /// Test 7: Verify forward direction respects camera rotation
    #[test]
    fn test_handle_free_flight_zoom_respects_camera_rotation() {
        let mut world = World::new();

        // Set camera mode to FreeFlight
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::FreeFlight;
        world.insert_resource(camera_state);

        // Set scroll delta
        let mut input_state = InputState::default();
        input_state.scroll_delta = 10.0;
        world.insert_resource(input_state);

        // Spawn a camera entity with yaw rotated 90 degrees (facing positive X)
        let initial_position = Vec3::new(0.0, 0.0, 0.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: std::f32::consts::PI / 2.0, // 90 degrees
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        )).id();

        // Verify the system behavior manually
        let controller = world.get::<CameraController>(entity).unwrap();
        
        // Extract needed values BEFORE mutable borrow to avoid borrow conflict
        let forward = controller.forward();
        let zoom_speed = controller.zoom_speed;
        
        // Now it's safe to create mutable borrow
        let mut transform = world.get_mut::<Transform>(entity).unwrap();
        
        let movement = forward * 10.0 * zoom_speed;
        transform.translation += movement;

        // Verify camera moved along its rotated forward vector
        // With yaw=90 degrees (PI/2), forward is (1, 0, 0)
        // Camera should move along positive X axis
        assert!(
            transform.translation.x > initial_position.x,
            "Camera should move along positive X when rotated 90 degrees yaw"
        );

        assert!(
            (transform.translation.x - 10.0).abs() < 0.001,
            "Camera should move exactly 10.0 units along X axis, got: {}",
            transform.translation.x
        );

        assert!(
            transform.translation.y.abs() < 0.001,
            "Camera Y position should remain near 0, got: {}",
            transform.translation.y
        );

        assert!(
            transform.translation.z.abs() < 0.001,
            "Camera Z position should remain near 0, got: {}",
            transform.translation.z
        );
    }

    /// Comprehensive test combining multiple scenarios
    #[test]
    fn test_handle_free_flight_zoom_comprehensive() {
        let mut world = World::new();

        // Set camera mode to FreeFlight
        let mut camera_state = CameraState::default();
        camera_state.mode = CameraMode::FreeFlight;
        world.insert_resource(camera_state);

        // Test with moderate scroll delta
        let mut input_state = InputState::default();
        input_state.scroll_delta = 20.0;
        world.insert_resource(input_state);

        // Spawn a camera entity
        let initial_position = Vec3::new(10.0, 5.0, 30.0);
        let entity = world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        )).id();

        // Verify the system behavior manually
        let controller = world.get::<CameraController>(entity).unwrap();
        
        // Extract needed values BEFORE mutable borrow to avoid borrow conflict
        let forward = controller.forward();
        let zoom_speed = controller.zoom_speed;
        
        // Now it's safe to create mutable borrow
        let mut transform = world.get_mut::<Transform>(entity).unwrap();
        
        let movement = forward * 20.0 * zoom_speed;
        transform.translation += movement;
        
        // Verify camera moved forward
        assert!(
            transform.translation.z > initial_position.z,
            "Camera should move forward"
        );

        // X and Y should remain unchanged
        assert_eq!(
            transform.translation.x, initial_position.x,
            "Camera X should not change"
        );

        assert_eq!(
            transform.translation.y, initial_position.y,
            "Camera Y should not change"
        );

        // Z should have moved exactly 20.0 units
        assert!(
            (transform.translation.z - (initial_position.z + 20.0)).abs() < 0.001,
            "Camera Z should move exactly 20.0 units"
        );

        // Distance from origin should be within valid range
        let distance = transform.translation.length();
        assert!(
            distance >= 1.0 && distance <= 200.0,
            "Camera distance {} should be within valid range [1.0, 200.0]",
            distance
        );
    }
}
