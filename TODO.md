# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Core Infrastructure
- [x] Implement basic input handling (keyboard events for WASD, mouse events for camera)
- [x] Register TimeIntegrationPlugin in main.rs for cosmic time updates

### Particle Rendering
- [x] Implement instanced particle renderer using Bevy PBR materials
- [x] Create particle component with position, color, size attributes
- [x] Implement particle spawner system for 100K-1M particles
- [x] Add GPU instancing support for efficient rendering
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
- [ ] FIX CRITICAL BUG: Remove duplicate TimeAccumulator initialization from main.rs (TimeIntegrationPlugin already initializes it)
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
- [ ] Define Config struct with all Phase 1 parameters (particle_count, time_acceleration, etc.)
- [ ] Implement TOML deserialization for Config struct
- [ ] Create default Config constants for "Standard Model" preset
- [ ] Implement config file loader with path resolution (default: genesis.toml, fallback: embedded defaults)
- [ ] Implement clap argument parser for --config flag to override default config path
- [ ] Create ConfigResource and add to main.rs via .insert_resource(config)

### Architecture & Documentation
- [ ] Update ARCHITECTURE.md with final crate structure and responsibilities
- [ ] Document epoch plugin architecture design patterns (trait-based plugin system)
- [ ] Add inline documentation for genesis-core public APIs (time::TimeAccumulator, epoch::EpochPlugin trait, physics::Particle)
- [ ] Add inline documentation for genesis-render public APIs (camera::CameraMode/State, input::InputState, particle::Particle component)
- [ ] Add inline documentation for genesis-ui public APIs (overlay::OverlayState, timeline::PlaybackState)

### Plugin Registration
- [ ] Create genesis-render::RenderPlugin (aggregates camera, input, particle systems)
- [ ] Add .add_plugins(genesis_render::RenderPlugin) to main.rs
- [ ] Create genesis-ui::UIPlugin (aggregates timeline, overlay systems)
- [ ] Add .add_plugins(genesis_ui::UIPlugin) to main.rs
- [ ] Add .init_resource::<CameraState>() to main.rs
- [ ] Add .init_resource::<OverlayState>() to main.rs
- [ ] Add .init_resource::<PlaybackState>() to main.rs
- [ ] Remove duplicate .init_resource::<TimeAccumulator>() from main.rs (already initialized by TimeIntegrationPlugin)

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

### Refined Task Definitions
- refined: Configuration & Initialization tasks broken down into 6 atomic subtasks
- refined: Architecture & Documentation tasks broken down into 5 atomic subtasks
- refined: Plugin Registration tasks broken down into 8 atomic subtasks

---

## Critical Bug: Duplicate TimeAccumulator Initialization

### Description
The `TimeAccumulator` resource is being initialized twice:
1. In `genesis-core/src/time/mod.rs` via `TimeIntegrationPlugin` which has a startup system `initialize_time_accumulator` that calls `commands.insert_resource(TimeAccumulator::default())`
2. In `src/main.rs` via `.init_resource::<TimeAccumulator>()`

### Root Cause
The `TimeIntegrationPlugin` already initializes the `TimeAccumulator` through its startup system. Adding `.init_resource::<TimeAccumulator>()` in `main.rs` creates a duplicate initialization path.

### Impact
This creates potential resource conflicts and violates Bevy's resource management patterns. While Bevy may handle this gracefully by ignoring duplicate init_resource calls, it's architecturally incorrect.

### Solution
Remove `.init_resource::<TimeAccumulator>()` from `main.rs` since `TimeIntegrationPlugin` already handles initialization properly. The startup system `initialize_time_accumulator` in the plugin should be the sole initialization point.

**Priority:** HIGH - Architectural inconsistency
