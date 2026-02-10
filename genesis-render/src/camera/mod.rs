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
//! - **Orbit mode**: Rotation implemented (left mouse drag), zoom and pan NOT implemented
//! - **Mode switching**: Implemented via 'O' key toggle between FreeFlight and Orbit
//! - **Camera interpolation**: NOT implemented (deferred to Phase 7)
//!
//! # Orbit Camera Limitations
//!
//! The orbit camera currently only supports rotation around the current orbit target:
//! - Left mouse drag: Rotates camera around target (spherical coordinates)
//! - Scroll wheel zoom: NOT implemented (no handle_orbit_zoom system)
//! - Middle/right mouse pan: NOT implemented (no handle_orbit_pan system)
//! - Orbit target: Set by CameraState.current_orbit_target (updated on mode switch)

use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::time::Time;

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
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            mode: CameraMode::default(),
            target: None,
            current_orbit_target: Vec3::ZERO,
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

/// System to handle orbit camera pan via middle mouse button drag
///
/// Moves the orbit target point based on mouse drag when in ORBIT camera mode.
/// Panning moves the target point relative to the camera's orientation.
///
/// # Parameters
///
/// * `input` - Resource containing input state including mouse_delta and mouse_buttons
/// * `camera_state` - Resource tracking current camera mode
/// * `camera_query` - Query for camera Transform to get orientation
///
/// # Behavior
///
/// - Only operates when CameraState.mode is ORBIT
/// - Only operates when middle mouse button is pressed
/// - Moves CameraState.current_orbit_target based on mouse delta
/// - Calculates pan direction based on camera's right and up vectors
/// - Uses pan_speed multiplier for sensitivity control (0.05)
pub fn handle_orbit_pan(
    input: Res<InputState>,
    mut camera_state: ResMut<CameraState>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    // Only update when in ORBIT mode
    if camera_state.mode != CameraMode::Orbit {
        return;
    }

    // Only update when middle mouse button is pressed
    if !input
        .mouse_buttons
        .get(&MouseButton::Middle)
        .copied()
        .unwrap_or(false)
    {
        return;
    }

    // Get camera orientation
    if let Ok(camera_transform) = camera_query.get_single() {
        // Calculate right and up vectors from camera orientation
        let right = camera_transform.right();
        let up = Vec3::Y;

        // Calculate pan movement
        // Mouse X delta moves along camera's right vector (inverted for natural feel)
        // Mouse Y delta moves along world up vector
        let pan_speed = 0.05;
        let pan_movement = (right * -input.mouse_delta.x * pan_speed)
            + (up * -input.mouse_delta.y * pan_speed);

        // Update orbit target
        camera_state.current_orbit_target += pan_movement;
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
/// While the PRD mentions "smooth interpolation" between modes, camera interpolation is
/// currently deferred to Phase 7. Mode switches are instant for responsive user control.
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
            .add_systems(Update, handle_free_flight_zoom)
            .add_systems(Update, handle_orbit_pan);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InputState;
    use bevy::ecs::system::SystemState;

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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera position hasn't changed (early return occurred)
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);

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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera moved forward along its forward vector
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);

        // With yaw=0, pitch=0, forward is (0, 0, 1)
        // scroll_delta=10.0, zoom_speed=1.0 => movement = 10.0
        // Initial position: (0, 0, 50)
        // Expected position: (0, 0, 60)
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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera distance is clamped to minimum 1.0
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);
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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera distance is clamped to maximum 200.0
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);
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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 2.0, // Custom zoom speed
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera moved using the custom zoom_speed
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);

        // scroll_delta=5.0, zoom_speed=2.0 => movement = 10.0
        // Initial position: (0, 0, 50)
        // Expected position: (0, 0, 60)
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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera moved backward
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);

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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: std::f32::consts::PI / 2.0, // 90 degrees
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera moved along its rotated forward vector
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);

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
        world.spawn((
            Transform::from_translation(initial_position),
            CameraController {
                yaw: 0.0,
                pitch: 0.0,
                movement_speed: 10.0,
                mouse_sensitivity: 0.002,
                zoom_speed: 1.0,
            },
        ));

        // Run the system using SystemState with get_mut
        let mut system_state: SystemState<(
            Res<InputState>,
            Res<CameraState>,
            Query<(&mut Transform, &CameraController)>,
        )> = SystemState::new(&mut world);
        let (input, camera_state, mut cameras) = system_state.get_mut(&mut world);
        handle_free_flight_zoom(input, camera_state, cameras);

        // Verify camera moved forward
        let mut query = world.query::<&Transform>();
        let transform = query.single(&world);

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
