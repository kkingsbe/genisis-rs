# Drift Detection

Critical drift items identified from PRD analysis:

## Feature Drift (Missing PRD Features)
- feat: Implement PRD feature bevy_egui panels - Actual implementation only has resource definitions with "not yet implemented" comments; no actual UI panels or widgets exist for timeline and overlays
- feat: Implement PRD feature TOML configuration system - No Config struct, no TOML loading, no command-line arguments for --config flag or "Standard Model" preset
- feat: Implement PRD feature epoch plugins - EpochPlugin trait and EpochManager exist, but NO actual epoch plugins are registered or implemented
- feat: Implement PRD feature logarithmic timeline scrubber UI - Only PlaybackState.speed field exists, no actual timeline widget or logarithmic mapping spanning 13.8 billion years

## Implementation Drift (Contradicts PRD)
- fix: Align particle system with PRD requirements - genesis-core::physics::Particle uses [f32; 3] arrays while genesis-render::particle::Particle uses Vec3 and Color Bevy types; they are completely disconnected with no synchronization between simulation and rendering
- fix: Align singularity visualization with PRD requirements - PRD specifies particles spawned at origin with outward velocity and color-mapped by energy (white-hot core fading to red); implementation uses random particle spawning in a sphere with mostly white/blue colors, no energy mapping
- fix: Align resource initialization with PRD requirements - CameraState, OverlayState, PlaybackState resources are defined but not initialized in main.rs
- fix: Align camera systems with PRD requirements - PRD requires smooth interpolation between positions and camera transition crossfade for epoch changes; implementation only has basic free-flight and orbit movement, no interpolation or crossfade

---

# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Missing Features (Drift Analysis - Phase 1)

- [ ] implement: Add pause/reset UI controls - Expose TimeAccumulator pause/reset functionality through UI
- [ ] implement: Create procedural singularity visualization - Replace random particle spawning with energy-mapped coloring (white-hot core to red edges) as specified
- [ ] implement: Implement epoch plugins - Create actual epoch plugins (e.g., SingularityEpoch, InflationEpoch) and register them with EpochManager

---

## Sprint 1 - Phase 1: The Singularity

### Camera System
- [ ] Add smooth camera interpolation between positions
- [ ] Implement camera transition crossfade for epoch changes
- [ ] Add zoom and pan controls

### Time & Timeline
- [ ] Implement time controls: play/pause, reset, speed adjustment (1x to 10¹²x)
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

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Drift Analysis Results (Flagged Issues)

### Unrequested Features
- refactor: Consider deferring full EpochPlugin architecture to Phase 2 - PRD only mentions epoch UI indicator in Phase 2, but keeping for now as foundation (non-blocking)
- refactor: Remove CameraState resource and CameraMode enum architecture - resource-based state tracking not specified in PRD
- refactor: Remove InputState resource architecture - detailed input tracking system not specified in PRD
- refactor: Remove show_epoch_info flag from OverlayState - not specified in Phase 1 PRD requirements
- refactor: Remove PlaybackState resource - resource-based playback state tracking not specified in PRD
- refactor: Remove VERSION constants from genesis-core, genesis-render, genesis-ui crates - not specified in PRD
- refactor: Remove bytemuck dependency from genesis-render/Cargo.toml - not in PRD dependency specifications

### Contradictory Code
- fix: Align TimeAccumulator in genesis-core/src/time/mod.rs with PRD requirements - missing pause functionality (PRD Phase 1 requires "pause, and reset" but only reset is implemented) -> ADDED TASK: "Add pause() method to TimeAccumulator resource"
- fix: Align genesis-render/src/particle/mod.rs with PRD requirements - module doc claims "GPU-accelerated rendering of up to 1M particles using Bevy's instancing system with PBR materials" but update_particles is a stub TODO with no actual implementation -> EXISTING TODO covers this
- fix: Align genesis-render/src/camera/mod.rs with PRD requirements - module doc claims "Free-flight and orbit camera implementations with smooth interpolation" but only CameraMode enum and CameraState resource exist, no actual camera implementation -> EXISTING TODO covers this
- fix: Align genesis-ui/src/timeline/mod.rs with PRD requirements - module doc claims "UI widgets for controlling cosmic time flow, including logarithmic timeline scrubber and playback controls" but only PlaybackState resource exists, no actual timeline UI implementation -> EXISTING TODO covers this
- fix: Align genesis-ui/src/overlay/mod.rs with PRD requirements - module doc claims "FPS counter, particle count display, epoch info panels, and other HUD elements" but only OverlayState resource exists, no actual overlay implementation -> EXISTING TODO covers this
- fix: Align genesis-core/src/physics/mod.rs and genesis-render/src/particle/mod.rs - two different Particle types with inconsistent field types ([f32; 3] vs Vec3, [f32; 3] vs Color)
- fix: Align genesis-render/src/particle/mod.rs with PRD requirements - PRD specifies point sprites but implementation uses sphere meshes with StandardMaterial
- fix: Align genesis-render/src/particle/mod.rs singularity visualization with PRD - missing outward velocity and energy-based color mapping (white-hot core fading to red)

### Refined Task Definitions
- refined: Configuration & Initialization tasks broken down into 6 atomic subtasks
- refined: Architecture & Documentation tasks broken down into 5 atomic subtasks
- refined: Plugin Registration tasks broken down into 8 atomic subtasks

## New Drift Items (Added 2026-02-09)

### Unrequested Features
- refactor: Remove CameraState.target field from genesis-render/src/camera/mod.rs - Target point field not specified in PRD Phase 1 requirements
- refactor: Remove CameraState.current_orbit_target field from genesis-render/src/camera/mod.rs - Orbit target tracking not specified in PRD
- refactor: Remove OrbitController.min_distance and max_distance fields from genesis-render/src/camera/mod.rs - Zoom distance constraints not specified in PRD
- refactor: Remove InputState.mouse_buttons HashMap approach from genesis-render/src/input/mod.rs - HashMap-based button state tracking not specified in PRD; use Bevy's ButtonInput directly

### Contradictions
- fix: Align PlaybackState.speed with TimeAccumulator.acceleration from genesis-ui/src/timeline/mod.rs - UI speed slider (0.1-10.0x) doesn't connect to TimeAccumulator.acceleration (1x-10¹²x as specified in PRD Phase 1)
- fix: Remove obsolete drift items from TODO.md - Lines 100-104 in TODO.md reference outdated contradictions that have been fixed: TimeAccumulator.pause() is now implemented, camera systems are implemented, timeline UI is implemented, overlay UI is implemented

### Missing Requirements
- implement: Add smooth camera interpolation to genesis-render/src/camera/mod.rs - PRD Phase 1 specifies smooth interpolation between camera positions but current implementation only has direct control
- implement: Add time acceleration connection between PlaybackState.speed and TimeAccumulator.acceleration - PRD Phase 1 requires adjustable acceleration (1x to 10¹²x) but UI speed slider is not connected to TimeAccumulator

---

## Final Sprint QA

- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

