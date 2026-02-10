# Camera Module Interpolation Analysis

**Date:** 2026-02-10  
**File:** `genesis-render/src/camera/mod.rs`  
**Phase:** 1 Implementation Analysis

---

## 1. Current Camera Architecture

### 1.1 Core Components

The camera system is built around four main types:

#### [`CameraMode`](../genesis-render/src/camera/mod.rs:61-83) (Enum)
```rust
pub enum CameraMode {
    #[default]
    FreeFlight,  // WASD + mouse look navigation
    Orbit,       // Click-drag rotation around target
}
```

**Purpose:** Defines the current active camera control mode.  
**Switching:** Toggled via 'O' key in [`toggle_camera_mode()`](../genesis-render/src/camera/mod.rs:474-498).

#### [`CameraState`](../genesis-render/src/camera/mod.rs:90-95) (Resource)
```rust
#[derive(Resource)]
pub struct CameraState {
    pub mode: CameraMode,
    pub target: Option<Vec3>,              // Generic target (unused)
    pub current_orbit_target: Vec3,        // Target point for orbit mode
}
```

**Purpose:** Global resource tracking camera state across systems.  
**Default:** [`FreeFlight`](../genesis-render/src/camera/mod.rs:73) mode with [`Vec3::ZERO`](../genesis-render/src/camera/mod.rs:102) orbit target.

#### [`CameraController`](../genesis-render/src/camera/mod.rs:135-146) (Component)
```rust
#[derive(Component, Debug)]
pub struct CameraController {
    pub yaw: f32,                 // Horizontal rotation (radians)
    pub pitch: f32,               // Vertical rotation (radians, clamped ±1.55)
    pub movement_speed: f32,      // WASD translation speed (default: 10.0)
    pub mouse_sensitivity: f32,   // Look sensitivity (default: 0.002)
    pub zoom_speed: f32,          // Scroll zoom multiplier (default: 1.0)
}
```

**Purpose:** Controls free-flight camera behavior (WASD movement + mouse look).  
**Active when:** [`CameraState.mode`](../genesis-render/src/camera/mod.rs:91) is `FreeFlight` (or always responsive to WASD).

**Key methods:**
- [`forward()`](../genesis-render/src/camera/mod.rs:162-169): Calculates forward direction from yaw/pitch
- [`right()`](../genesis-render/src/camera/mod.rs:172-174): Calculates right vector (cross product)

#### [`OrbitController`](../genesis-render/src/camera/mod.rs:182-191) (Component)
```rust
#[derive(Component, Debug)]
pub struct OrbitController {
    pub distance: f32,                // Distance from target (default: 50.0)
    pub yaw: f32,                    // Horizontal rotation (default: 0.0)
    pub pitch: f32,                  // Vertical rotation (default: 0.3)
    pub rotation_sensitivity: f32,   // Mouse drag sensitivity (default: 0.005)
}
```

**Purpose:** Controls orbit camera behavior (spherical coordinate system).  
**Active when:** [`CameraState.mode`](../genesis-render/src/camera/mod.rs:91) is `Orbit` AND left mouse button pressed.

**Key methods:**
- [`calculate_position()`](../genesis-render/src/camera/mod.rs:209-215): Converts spherical coordinates to Cartesian position

### 1.2 Camera Entity Setup (from [`src/main.rs:118-130`](../src/main.rs:118))

```rust
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 0.0, orbit_distance).looking_at(Vec3::ZERO, Vec3::Y),
    OrbitController { distance: orbit_distance, ..default() },
    CameraController::default(),
));
```

**Key insight:** Both controllers are **always present** on the camera entity. Mode switching determines which one responds to input.

---

## 2. Mode Implementation Details

### 2.1 Free-Flight Mode

**System:** [`update_free_flight_camera()`](../genesis-render/src/camera/mod.rs:226-257)  
**Schedule:** Update  
**Behavior:**
1. Updates [`yaw`](../genesis-render/src/camera/mod.rs:137)/[`pitch`](../genesis-render/src/camera/mod.rs:138) from mouse delta
2. Clamps pitch to `[-1.55, 1.55]` (~±89°) to prevent gimbal lock
3. Calculates forward/right/up vectors from orientation
4. Applies WASD movement along these vectors
5. Updates camera Transform with [`look_at()`](../genesis-render/src/camera/mod.rs:255)

**Key note:** This system runs for all cameras with `CameraController`, **regardless of mode**. The camera always responds to WASD/mouse look.

### 2.2 Orbit Mode

**System:** [`update_orbit_camera()`](../genesis-render/src/camera/mod.rs:267-299)  
**Schedule:** Update  
**Behavior:**
1. Only runs when left mouse button is pressed
2. Updates [`yaw`](../genesis-render/src/camera/mod.rs:186)/[`pitch`](../genesis-render/src/camera/mod.rs:187) from mouse delta
3. Clamps pitch to `[-1.55, 1.55]`
4. Calculates new position via [`calculate_position()`](../genesis-render/src/camera/mod.rs:209-215)
5. Sets Transform translation to calculated position
6. Makes camera look at [`camera_state.current_orbit_target`](../genesis-render/src/camera/mod.rs:295)

**Additional systems:**
- [`handle_orbit_zoom()`](../genesis-render/src/camera/mod.rs:319-336): Scroll wheel zoom (distance clamped to `[1.0, 200.0]`)
- [`handle_orbit_pan()`](../genesis-render/src/camera/mod.rs:409-445): Middle mouse drag panning

---

## 3. Mode Switching Mechanism

### 3.1 Current Implementation: [`toggle_camera_mode()`](../genesis-render/src/camera/mod.rs:474-498)

```rust
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
            }
            CameraMode::Orbit => {
                // Switching from Orbit to FreeFlight - instant switch
                camera_state.mode = CameraMode::FreeFlight;
            }
        }
    }
}
```

**Status:** ✅ Implemented (instant switching)  
**Deferred:** Smooth interpolation (documented as deferred to Phase 7 at line 30)

### 3.2 Switch Behavior

| Direction | Mode Change | Target Setting | Interpolation |
|-----------|-------------|----------------|--------------|
| FreeFlight → Orbit | Sets [`mode`](../genesis-render/src/camera/mod.rs:483) to `Orbit` | Sets orbit target to 10 units in front of camera | **None (instant)** |
| Orbit → FreeFlight | Sets [`mode`](../genesis-render/src/camera/mod.rs:494) to `FreeFlight` | No target change | **None (instant)** |

---

## 4. Interpolation Requirements Analysis

### 4.1 Values Requiring Interpolation

When switching **FreeFlight → Orbit**, the following need smooth transition:

1. **Camera Position:**
   - Start: Current [`Transform.translation`](../genesis-render/src/camera/mod.rs:250)
   - End: Orbit position calculated from [`OrbitController.distance`](../genesis-render/src/camera/mod.rs:184) at [`camera_state.current_orbit_target`](../genesis-render/src/camera/mod.rs:295)
   - **Issue:** Orbit position depends on spherical coordinates relative to origin, not the current camera position

2. **Camera Rotation:**
   - Start: Current [`Transform.rotation`](../genesis-render/src/camera/mod.rs:255)
   - End: Rotation that looks at [`camera_state.current_orbit_target`](../genesis-render/src/camera/mod.rs:295) from orbit position
   - Method: Quaternion spherical interpolation (slerp)

3. **Orbit Target Point:**
   - Start: Previously set [`current_orbit_target`](../genesis-render/src/camera/mod.rs:488)
   - End: New target calculated at 10 units in front of camera (current implementation)
   - Can be interpolated linearly: `lerp(old_target, new_target, progress)`

When switching **Orbit → FreeFlight**, the following need smooth transition:

1. **Camera Position:**
   - Start: Current orbit position (maintain)
   - End: Current position (no change needed)
   - **No interpolation required** - camera stays in place

2. **Camera Rotation:**
   - Start: Current looking-at-target orientation
   - End: Free-flight orientation from [`CameraController.yaw`](../genesis-render/src/camera/mod.rs:137)/[`pitch`](../genesis-render/src/camera/mod.rs:138)
   - **Potentially needs interpolation** to align [`CameraController`](../genesis-render/src/camera/mod.rs:135) with actual camera rotation

### 4.2 The Coordinate System Mismatch

**Critical Issue:** The two controllers use **different coordinate representations**:

| Controller | Position Representation | Rotation Representation |
|------------|------------------------|------------------------|
| [`CameraController`](../genesis-render/src/camera/mod.rs:135) | Cartesian world position (stored in [`Transform.translation`](../genesis-render/src/camera/mod.rs:250)) | [`yaw`](../genesis-render/src/camera/mod.rs:137), [`pitch`](../genesis-render/src/camera/mod.rs:138) angles |
| [`OrbitController`](../genesis-render/src/camera/mod.rs:182) | Spherical (distance, yaw, pitch) relative to [`Vec3::ZERO`](../genesis-render/src/camera/mod.rs:211) | Implicit: always looks at target |

**Problem:** When switching from FreeFlight to Orbit, we need to:
1. Convert current Cartesian position to spherical coordinates
2. Interpolate the spherical coordinates to the desired orbit position
3. Meanwhile, the free-flight controller's yaw/pitch are not synchronized with actual camera rotation

### 4.3 PRD Requirement Context

From [`PRD.md:114`](../PRD.md:114):
> "Free-flight camera (WASD + mouse) and orbit camera (click-drag) with **smooth interpolation**"

From [`genesis-render/src/camera/mod.rs:30`](../genesis-render/src/camera/mod.rs:30):
> "Camera interpolation: NOT implemented (deferred to Phase 7)"

**Conflict:** Phase 1 explicitly requires smooth interpolation, but implementation defers it.

---

## 5. Relevant Types and Functions

### 5.1 Public Types (from [`camera/mod.rs`](../genesis-render/src/camera/mod.rs))

| Type | Lines | Fields | Purpose |
|------|-------|--------|---------|
| [`CameraMode`](../genesis-render/src/camera/mod.rs:61) | 61-83 | `FreeFlight`, `Orbit` | Camera mode enumeration |
| [`CameraState`](../genesis-render/src/camera/mod.rs:90) | 90-95 | `mode`, `target`, `current_orbit_target` | Resource tracking camera state |
| [`CameraController`](../genesis-render/src/camera/mod.rs:135) | 135-146 | `yaw`, `pitch`, `movement_speed`, `mouse_sensitivity`, `zoom_speed` | Component for free-flight |
| [`OrbitController`](../genesis-render/src/camera/mod.rs:182) | 182-191 | `distance`, `yaw`, `pitch`, `rotation_sensitivity` | Component for orbit |

### 5.2 Public Functions (from [`camera/mod.rs`](../genesis-render/src/camera/mod.rs))

| Function | Lines | Signature | Purpose |
|----------|-------|-----------|---------|
| [`CameraController::forward()`](../genesis-render/src/camera/mod.rs:162) | 162-169 | `&self -> Vec3` | Calculate forward direction from yaw/pitch |
| [`CameraController::right()`](../genesis-render/src/camera/mod.rs:172) | 172-174 | `&self -> Vec3` | Calculate right vector |
| [`OrbitController::calculate_position()`](../genesis-render/src/camera/mod.rs:209) | 209-215 | `&self -> Vec3` | Convert spherical to Cartesian |
| [`update_free_flight_camera()`](../genesis-render/src/camera/mod.rs:226) | 226-257 | System | Handle WASD + mouse input |
| [`update_orbit_camera()`](../genesis-render/src/camera/mod.rs:267) | 267-299 | System | Handle orbit rotation |
| [`handle_orbit_zoom()`](../genesis-render/src/camera/mod.rs:319) | 319-336 | System | Handle scroll zoom |
| [`handle_free_flight_zoom()`](../genesis-render/src/camera/mod.rs:356) | 359-389 | System | Handle scroll zoom in free-flight |
| [`handle_orbit_pan()`](../genesis-render/src/camera/mod.rs:409) | 409-445 | System | Handle middle mouse pan |
| [`toggle_camera_mode()`](../genesis-render/src/camera/mod.rs:474) | 474-498 | System | Toggle 'O' key handler |

### 5.3 External Dependencies

- [`bevy::input::ButtonInput`](../genesis-render/src/camera/mod.rs:40) - Key/button state tracking
- [`bevy::time::Time`](../genesis-render/src/camera/mod.rs:44) - Delta time for frame-rate independent movement
- [`crate::input::InputState`](../genesis-render/src/camera/mod.rs:46) - Keyboard direction, mouse delta, scroll delta, button states
- [`genesis_core::config::CameraConfig`](../genesis-render/src/camera/mod.rs:47) - Configuration for initial mode and orbit distance

---

## 6. Implementation Recommendations

### 6.1 Recommended Approach: Custom Interpolation System

Given the complexity of the coordinate system mismatch and the need for PRD Phase 1 compliance, I recommend implementing a **custom interpolation system** rather than using external crates like `bevy_tweening`.

**Rationale:**
1. **Minimal dependencies:** No additional crates required
2. **Fine-grained control:** Can handle the coordinate system conversion explicitly
3. **Phase-appropriate complexity:** Simple linear interpolation with ease-in/out is sufficient for mode switching
4. **Bevy-native:** Uses standard Bevy ECS patterns

### 6.2 Architecture Design

#### Step 1: Extend [`CameraState`](../genesis-render/src/camera/mod.rs:90) with Interpolation Fields

```rust
#[derive(Resource)]
pub struct CameraState {
    pub mode: CameraMode,
    pub target: Option<Vec3>,
    pub current_orbit_target: Vec3,
    
    // NEW: Interpolation state
    pub is_interpolating: bool,
    pub interpolation_progress: f32,        // 0.0 to 1.0
    pub interpolation_duration: f32,        // Seconds (default: 0.5)
    pub interpolation_elapsed: f32,          // Time elapsed in seconds
    
    // Start state (before transition)
    pub start_position: Vec3,
    pub start_rotation: Quat,
    pub start_orbit_target: Vec3,
    
    // End state (after transition)
    pub end_position: Vec3,
    pub end_rotation: Quat,
    pub end_orbit_target: Vec3,
}
```

#### Step 2: Add Interpolation Methods to [`CameraState`](../genesis-render/src/camera/mod.rs:90)

```rust
impl CameraState {
    /// Start interpolation to a new camera state
    pub fn start_interpolation(&mut self, 
        start_pos: Vec3, start_rot: Quat,
        end_pos: Vec3, end_rot: Quat,
        duration: f32
    ) {
        self.is_interpolating = true;
        self.interpolation_progress = 0.0;
        self.interpolation_elapsed = 0.0;
        self.interpolation_duration = duration;
        self.start_position = start_pos;
        self.start_rotation = start_rot;
        self.end_position = end_pos;
        self.end_rotation = end_rot;
    }
    
    /// Cubic ease-in-out function for smooth transitions
    pub fn ease_cubic(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}
```

#### Step 3: Create [`interpolate_camera()`](../genesis-render/src/camera/mod.rs) System

```rust
/// System to handle smooth camera interpolation between modes
///
/// Runs in PostUpdate to ensure it processes after input systems
/// but before frame rendering.
fn interpolate_camera(
    mut camera_state: ResMut<CameraState>,
    mut cameras: Query<(&mut Transform, &mut CameraController, &mut OrbitController)>,
    time: Res<Time>,
) {
    if !camera_state.is_interpolating {
        return;
    }
    
    // Update elapsed time
    camera_state.interpolation_elapsed += time.delta_secs();
    
    // Calculate progress (0.0 to 1.0)
    let raw_progress = (camera_state.interpolation_elapsed / camera_state.interpolation_duration).clamp(0.0, 1.0);
    
    // Apply easing
    camera_state.interpolation_progress = CameraState::ease_cubic(raw_progress);
    let t = camera_state.interpolation_progress;
    
    for (mut transform, mut controller, mut orbit) in cameras.iter_mut() {
        // Interpolate position
        transform.translation = camera_state.start_position.lerp(camera_state.end_position, t);
        
        // Interpolate rotation (slerp for smooth angular transitions)
        transform.rotation = camera_state.start_rotation.slerp(camera_state.end_rotation, t);
        
        // Update CameraController yaw/pitch from current rotation (for continuity)
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        controller.yaw = yaw;
        controller.pitch = pitch;
        
        // Update OrbitController from current position (for continuity)
        let relative_pos = transform.translation - camera_state.current_orbit_target;
        orbit.distance = relative_pos.length();
        orbit.yaw = relative_pos.y.atan2(relative_pos.x);
        orbit.pitch = (relative_pos.y / orbit.distance).asin().clamp(-1.55, 1.55);
    }
    
    // Check if interpolation is complete
    if raw_progress >= 1.0 {
        camera_state.is_interpolating = false;
        camera_state.interpolation_progress = 1.0;
    }
}
```

#### Step 4: Modify [`toggle_camera_mode()`](../genesis-render/src/camera/mod.rs:474) to Trigger Interpolation

```rust
fn toggle_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_state: ResMut<CameraState>,
    camera_query: Query<(&Transform, &CameraController, &OrbitController), With<Camera3d>>,
) {
    if keys.just_pressed(KeyCode::KeyO) {
        if let Ok((transform, controller, orbit)) = camera_query.get_single() {
            match camera_state.mode {
                CameraMode::FreeFlight => {
                    // Start interpolation to orbit mode
                    camera_state.mode = CameraMode::Orbit;
                    
                    // Calculate orbit target (10 units in front)
                    let forward = transform.forward();
                    let new_orbit_target = transform.translation + forward * 10.0;
                    
                    // Calculate end position for orbit mode
                    // Use current OrbitController settings relative to new target
                    let end_pos = new_orbit_target + 
                        Vec3::new(
                            orbit.distance * orbit.pitch.cos() * orbit.yaw.sin(),
                            orbit.distance * orbit.pitch.sin(),
                            orbit.distance * orbit.pitch.cos() * orbit.yaw.cos(),
                        );
                    
                    // Calculate end rotation (looking at target)
                    let end_rot = Quat::look_at(new_orbit_target - end_pos, Vec3::Y);
                    
                    // Start interpolation
                    camera_state.start_interpolation(
                        transform.translation,
                        transform.rotation,
                        end_pos,
                        end_rot,
                        0.5, // 0.5 second transition
                    );
                    
                    camera_state.current_orbit_target = new_orbit_target;
                }
                CameraMode::Orbit => {
                    // Start interpolation to free-flight mode
                    camera_state.mode = CameraMode::FreeFlight;
                    
                    // Current position stays the same
                    let current_pos = transform.translation;
                    
                    // End rotation from CameraController
                    let forward = controller.forward();
                    let end_rot = Quat::look_at(forward, Vec3::Y);
                    
                    // Start interpolation (no position change needed)
                    camera_state.start_interpolation(
                        current_pos,
                        transform.rotation,
                        current_pos, // Same position
                        end_rot,     // New orientation
                        0.5,
                    );
                }
            }
        }
    }
}
```

#### Step 5: Register the Interpolation System in [`CameraPlugin`](../genesis-render/src/camera/mod.rs:500)

```rust
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraState>()
            .add_systems(Update, toggle_camera_mode)
            .add_systems(Update, update_free_flight_camera)
            .add_systems(Update, update_orbit_camera)
            .add_systems(Update, handle_orbit_zoom)
            .add_systems(Update, handle_free_flight_zoom)
            .add_systems(Update, handle_orbit_pan)
            .add_systems(PostUpdate, interpolate_camera); // NEW: Add interpolation system
    }
}
```

### 6.3 Alternative Approaches Considered

#### Alternative 1: Bevy Tweening Crate (`bevy_tweening`)

**Pros:**
- Well-tested, feature-rich
- Supports multiple easing functions
- Handles interpolation scheduling

**Cons:**
- Additional dependency
- May be overkill for simple mode switching
- Doesn't solve the coordinate system mismatch natively

#### Alternative 2: Instant Switch Only

**Pros:**
- Simple, no additional code
- Responsive (no delay)

**Cons:**
- Does **not** meet PRD Phase 1 requirement
- Jarring visual transition
- User experience suffers

#### Alternative 3: Animated Transitions Without Interpolation

**Pros:**
- Simpler than full interpolation
- Can use camera shake or fade effects

**Cons:**
- Still not "smooth interpolation" as per PRD
- Doesn't solve the fundamental position/rotation continuity issue

### 6.4 Recommended Timeline

| Phase | Task | Priority |
|-------|------|----------|
| Now | Implement custom interpolation system as described | **Critical** (Phase 1 requirement) |
| Phase 7 | Consider upgrading to `bevy_tweening` if cinematic paths are needed | Future enhancement |

---

## 7. Acceptance Criteria Verification

- [x] `genesis-render/src/camera/mod.rs` is fully analyzed
- [x] Report identifies all camera-related types and their fields
- [x] Report explains current mode switching mechanism
- [x] Report provides a clear implementation recommendation for smooth interpolation

---

## 8. Summary

### Current State
- Both camera controllers exist on the camera entity
- Mode switching is **instant** (no interpolation)
- Smooth interpolation is documented as deferred to Phase 7, but PRD Phase 1 explicitly requires it

### Critical Issue
The coordinate system mismatch between Cartesian (free-flight) and spherical (orbit) representations means:
1. Interpolation requires explicit conversion between coordinate systems
2. Controllers' internal state (yaw/pitch/distance) must stay synchronized with actual camera Transform during interpolation

### Recommended Solution
Implement a **custom interpolation system** with:
1. Extended [`CameraState`](../genesis-render/src/camera/mod.rs:90) with interpolation fields
2. Cubic ease-in-out for smooth transitions
3. Separate [`interpolate_camera()`](../genesis-render/src/camera/mod.rs) system in PostUpdate schedule
4. Synchronization of both controllers' internal state with Transform during interpolation
5. 0.5 second transition duration for mode switching

This approach:
- ✅ Meets PRD Phase 1 requirement
- ✅ Handles coordinate system complexity
- ✅ Uses Bevy-native patterns
- ✅ Adds minimal code complexity
- ✅ Provides extensible foundation for Phase 7 cinematic interpolation
