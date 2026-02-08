# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Core Infrastructure
- [x] FIX BLOCKER: Initialize TimeAccumulator resource in main.rs (add `.init_resource::<TimeAccumulator>()`)
- [ ] Implement basic input handling (keyboard events for WASD, mouse events for camera)
- [ ] Register TimeIntegrationPlugin in main.rs for cosmic time updates

### Particle Rendering
- [ ] Implement instanced particle renderer using Bevy PBR materials
- [ ] Create particle component with position, color, size attributes
- [ ] Implement particle spawner system for 100K-1M particles
- [ ] Add GPU instancing support for efficient rendering
- [ ] Implement point sprite rendering with size attenuation

### Camera System
- [ ] Implement free-flight camera (WASD + mouse look) system
- [ ] Implement orbit camera (click-drag rotation) system
- [ ] Add smooth camera interpolation between positions
- [ ] Implement camera transition crossfade for epoch changes
- [ ] Add zoom and pan controls

### Time & Timeline
- [ ] Create cosmic time accumulator (f64) with adjustable acceleration
- [ ] Implement time controls: play/pause, reset, speed adjustment (1x to 10¹²x)
- [ ] Add pause() method to TimeAccumulator resource
- [ ] Build logarithmic timeline scrubber UI using bevy_egui
- [ ] Map timeline scrubbing to cosmic time simulation state
- [ ] Add epoch indicator display (current era, temperature, scale factor)

### Singularity Visualization
- [ ] Implement procedural singularity particle generation (origin spawn with outward velocity)
- [ ] Create energy-based color mapping (white-hot → yellow → red cooling)
- [ ] Add particle velocity expansion simulation
- [ ] Implement timeline scrubbing with reverse/replay capability

### UI Overlay
- [ ] Implement FPS counter overlay
- [ ] Add particle count display
- [ ] Create epoch info panel (time, temperature, scale factor)
- [ ] Build time control UI (play/pause, speed slider, reset button)
- [ ] Add timeline scrubber widget

### Configuration & Initialization
- [ ] Create TOML configuration system
- [ ] Implement config file loading and default values
- [ ] Add command-line argument parsing for config override

### Architecture & Documentation
- [ ] Write ARCHITECTURE.md with crate structure and responsibilities
- [ ] Document epoch plugin architecture
- [ ] Add inline code documentation for all public APIs

### Plugin Registration
- [ ] Register genesis-render plugin in main.rs (add .add_plugins(RenderPlugin))
- [ ] Register genesis-ui plugin in main.rs (add .add_plugins(UIPlugin))
- [ ] Initialize all required resources (CameraState, OverlayState, PlaybackState) in main.rs

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Drift Analysis Results (Flagged Issues)

### Unrequested Features
- refactor: Consider deferring full EpochPlugin architecture to Phase 2 - PRD only mentions epoch UI indicator in Phase 2, but keeping for now as foundation (non-blocking)

### Contradictory Code
- fix: Align TimeAccumulator in genesis-core/src/time/mod.rs with PRD requirements - missing pause functionality (PRD Phase 1 requires "pause, and reset" but only reset is implemented) -> ADDED TASK: "Add pause() method to TimeAccumulator resource"
- fix: Align genesis-render/src/particle/mod.rs with PRD requirements - module doc claims "GPU-accelerated rendering of up to 1M particles using Bevy's instancing system with PBR materials" but update_particles is a stub TODO with no actual implementation -> EXISTING TODO covers this
- fix: Align genesis-render/src/camera/mod.rs with PRD requirements - module doc claims "Free-flight and orbit camera implementations with smooth interpolation" but only CameraMode enum and CameraState resource exist, no actual camera implementation -> EXISTING TODO covers this
- fix: Align genesis-ui/src/timeline/mod.rs with PRD requirements - module doc claims "UI widgets for controlling cosmic time flow, including logarithmic timeline scrubber and playback controls" but only PlaybackState resource exists, no actual timeline UI implementation -> EXISTING TODO covers this
- fix: Align genesis-ui/src/overlay/mod.rs with PRD requirements - module doc claims "FPS counter, particle count display, epoch info panels, and other HUD elements" but only OverlayState resource exists, no actual overlay implementation -> EXISTING TODO covers this

---

## Runtime Issues / Bugs

### TimeAccumulator Resource Initialization Panic
**Error:** `genesis_core::epoch::update_epoch_transition could not access system parameter Res<TimeAccumulator>`

**Description:** The application crashes with a runtime panic when `cargo run` is executed. The `update_epoch_transition` system in `genesis_core::epoch` attempts to access the `TimeAccumulator` resource, but the resource has not been registered or initialized in the Bevy App before the system runs.

**Root Cause:** The `TimeAccumulator` resource is defined in `genesis_core::time` but is not being added to the app's world resource registry during initialization.

**Status:** TASK ADDED - "FIX BLOCKER: Initialize TimeAccumulator resource in main.rs (add `.init_resource::<TimeAccumulator>()`)"

**Remaining Actions:**
1. Register TimeIntegrationPlugin in main.rs for cosmic time updates
2. Ensure resources are initialized before dependent systems run

**Priority:** BLOCKER - Prevents application from running
