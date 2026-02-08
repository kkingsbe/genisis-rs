# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Core Infrastructure
- [x] Create epoch manager plugin architecture with registration system
- [ ] Set up time integration system with f64 accumulator
- [x] **CRITICAL: Fix root workspace binary target** - Root Cargo.toml is configured as workspace-only manifest without a [package] section, preventing `cargo run` from working at the root level. The src/main.rs exists but is orphaned without a package to link to. Solution: Add [package] section to root Cargo.toml with appropriate dependencies (name, version, bevy dependency, and member workspace crates) to enable running the application directly from project root

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

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Drift Analysis Results (Flagged Issues)

### Unrequested Features
- refactor: Remove unrequested feature - complete EpochPlugin architecture in genesis-core/src/epoch/mod.rs (EpochPlugin trait with build method, EpochManager with register_plugin, register_and_build_plugin, get_current_epoch, set_current_epoch, epoch_names, epoch_count methods, and update_epoch_transition system) - not requested in Phase 1, PRD only mentions epoch UI indicator in Phase 2

### Contradictory Code
- fix: Align TimeAccumulator in genesis-core/src/time/mod.rs with PRD requirements - missing pause functionality (PRD Phase 1 requires "pause, and reset" but only reset is implemented)
- fix: Align genesis-render/src/particle/mod.rs with PRD requirements - module doc claims "GPU-accelerated rendering of up to 1M particles using Bevy's instancing system with PBR materials" but update_particles is a stub TODO with no actual implementation
- fix: Align genesis-render/src/camera/mod.rs with PRD requirements - module doc claims "Free-flight and orbit camera implementations with smooth interpolation" but only CameraMode enum and CameraState resource exist, no actual camera implementation
- fix: Align genesis-ui/src/timeline/mod.rs with PRD requirements - module doc claims "UI widgets for controlling cosmic time flow, including logarithmic timeline scrubber and playback controls" but only PlaybackState resource exists, no actual timeline UI implementation
- fix: Align genesis-ui/src/overlay/mod.rs with PRD requirements - module doc claims "FPS counter, particle count display, epoch info panels, and other HUD elements" but only OverlayState resource exists, no actual overlay implementation
