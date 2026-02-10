# Orbit Camera Pan Implementation - Decomposition Plan

**Date:** 2026-02-10
**Sprint:** Sprint 2 (Singularity Refinement)
**TODO Item:** Implement pan controls for orbit camera

## Current State Analysis

### genesis-render/src/input/mod.rs
- `InputState.mouse_buttons` is a HashMap tracking mouse button states
- `handle_mouse_input()` system currently tracks only LEFT mouse button
- Middle and Right mouse buttons are NOT tracked

### genesis-render/src/camera/mod.rs
- `CameraState.current_orbit_target: Vec3` - the point orbit camera rotates around
- `OrbitController` - stores spherical coordinates (distance, yaw, pitch)
- `update_orbit_camera()` - handles rotation (left mouse drag)
- `handle_orbit_zoom()` - handles zoom (scroll wheel)
- NO `handle_orbit_pan()` system exists

## Decomposition

### Subtask 1: Add middle mouse button tracking to InputState
**File:** `genesis-render/src/input/mod.rs`
**Function:** `handle_mouse_input()`

Add `MouseButton::Middle` to the tracked buttons:
```rust
input_state.mouse_buttons.insert(MouseButton::Middle, mouse_buttons.pressed(MouseButton::Middle));
```

### Subtask 2: Implement handle_orbit_pan() system
**File:** `genesis-render/src/camera/mod.rs`

Create new system that:
- Only operates when `CameraState.mode` is ORBIT
- Only operates when middle mouse button is pressed
- Moves `CameraState.current_orbit_target` based on mouse delta
- Calculates pan direction based on camera's right and up vectors
- Uses pan_speed multiplier for sensitivity control

### Subtask 3: Register handle_orbit_pan system in CameraPlugin
**File:** `genesis-render/src/camera/mod.rs`
**Function:** `CameraPlugin::build()`

Add the new system to the Update schedule:
```rust
.add_systems(Update, handle_orbit_pan)
```

## Dependencies
- Subtask 1 must complete before Subtask 2 (needs middle button state available)
- Subtask 2 must complete before Subtask 3 (system must exist before registration)

## Verification
- Build compiles: `cargo build`
- Middle mouse button is tracked in InputState
- Middle mouse drag moves orbit target in orbit mode
