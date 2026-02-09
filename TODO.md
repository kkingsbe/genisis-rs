# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

# Drift Remediation (Priority: CRITICAL)
*Complete these items before Sprint 1 to unblock implementation*

## Critical Fixes (Block Sprint 1 Completion)
- [ ] Remove EpochManager with automatic transitions from genesis-core/src/epoch/mod.rs
- [ ] Remove EpochCameraConfig from genesis-core/src/epoch/camera_config.rs
- [ ] Remove epoch transition camera handling from genesis-render/src/camera/
- [ ] Remove general-purpose camera interpolation from CameraState
- [ ] Remove or refactor sync_time_resources() system (CosmicTime resource is redundant)
- [ ] Implement Config::load() method for TOML file loading
- [ ] Fix camera config field name mismatch (orbit_distance → orbit_radius) in src/main.rs
- [ ] Align speed slider range with PRD (1x to 10¹²x) in genesis-ui/src/timeline/mod.rs
- [ ] Map PlaybackState.speed to TimeAccumulator.acceleration with proper logarithmic scaling

## Architectural Cleanup
- [ ] Mark "Build logarithmic timeline scrubber UI" as complete (already implemented in genesis-ui/src/timeline/mod.rs)
- [ ] Remove redundant configuration system umbrella task from BACKLOG.md
- [ ] Move future epoch plugin tasks to respective sprint sections

## Documentation Sync
- [ ] Update ARCHITECTURE.md to reflect Phase 1 scope (single epoch, no epoch transitions)
- [ ] Document why EpochManager and related features were removed (Phase 2+ scope)

## Decision Items (Resolve These)
- [ ] Sprint 1 item #4 "Add epoch indicator display" conflicts with drift remediation. Decision: Keep epoch indicator (PRD requirement) but simplify for Phase 1 (show temperature/scale factor only, no epoch name since only one epoch)
- [ ] Sprint 1 item #7 "Implement timeline scrubbing with reverse/replay" needs snapshot system. Decision: Defer to Sprint 2 or implement simple version?

---

## Sprint 1 - Phase 1: The Singularity

### Critical Blocker Resolution (Priority: IMMEDIATE)

### Camera System

### Time & Timeline
- [x] Implement time controls: play/pause, reset, speed adjustment (1x to 10¹²x)
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

<!-- ARCHIVAL: Original drift remediation analysis - items now tracked in Drift Remediation section above -->

## Drift Remediation (Identified 2026-02-09)

### Phase-Inappropriate Features
- [ ] refactor: Remove epoch manager architecture from Phase 1 code in genesis-core/src/epoch/mod.rs (complete EpochPlugin, EpochManager, and epoch transition systems are Phase 2+)
- [ ] refactor: Remove epoch-specific camera configuration from Phase 1 code in genesis-core/src/epoch/camera_config.rs (EpochCameraConfig with crossfade synchronization is Phase 2+)
- [ ] refactor: Remove epoch transition handling from Phase 1 code in genesis-render/src/camera/epoch_transition.rs (entire module handles multi-epoch transitions)
- [ ] fix: Align timeline UI with PRD logarithmic scrubber requirement in genesis-ui/src/timeline/mod.rs (currently uses linear slider instead of logarithmic scrubber spanning 13.8B years)

### Unrequested Features
- [ ] refactor: Remove general-purpose camera interpolation infrastructure from CameraState in genesis-render/src/camera/mod.rs (Phase 7 feature)
- [ ] refactor: Remove epoch transition camera handling system from genesis-render/src/camera/epoch_transition.rs (Phase 1 only has Singularity epoch)
- [ ] refactor: Simplify epoch management for Phase 1 - remove automatic transitions and event system from genesis-core/src/epoch/mod.rs
- [ ] refactor: Remove EpochCameraConfig from genesis-core/src/epoch/camera_config.rs (not needed for single epoch in Phase 1)
- [ ] refactor: Remove separate CosmicTime resource - timeline should read directly from TimeAccumulator in genesis-ui/src/timeline/mod.rs
- [ ] refactor: Remove sync_time_resources() system - timeline should directly control TimeAccumulator in genesis-ui/src/timeline/mod.rs
- [ ] refactor: Remove test camera target from src/main.rs (setup_test_camera_target() is test infrastructure not required by PRD)
- [ ] refactor: Remove epoch info overlay from genesis-ui/src/overlay/mod.rs (Phase 1 PRD only requires FPS counter and particle count overlay)

### PRD Contradictions
- [ ] fix: Clarify time_acceleration_min default value in genesis-core/src/config.rs:138 - should explicitly be 1.0 to match PRD "1x to 10¹²x"
- [ ] fix: Align timeline speed slider range with PRD specification (1x to 10¹²x) in genesis-ui/src/timeline/mod.rs:170-175
- [ ] fix: Timeline slider scrubbing should update TimeAccumulator.years in genesis-ui/src/timeline/mod.rs:155-163
- [ ] fix: Complete speed-to-acceleration mapping in genesis-ui/src/timeline/mod.rs (sync_time_resources() only syncs play/pause, not acceleration)
- [ ] fix: Map PlaybackState.speed to TimeAccumulator.acceleration with proper logarithmic scaling in genesis-ui/src/timeline/mod.rs:195-204
- [ ] fix: Camera initial mode should be set from config.initial_mode in src/main.rs:87-97
- [ ] fix: Particle rendering should use individual Particle.color values instead of single material color in genesis-render/src/particle/mod.rs:214-267
- [ ] fix: Particle.color changes in update_particle_energy_colors() should affect rendering in genesis-render/src/particle/mod.rs:314-328
- [ ] fix: SingularityEpoch time range should be extended to allow visualization (PRD describes visible particle explosion) in genesis-core/src/epoch/singularity.rs:32-38
- [ ] fix: Correct epoch time boundary documentation in ARCHITECTURE.md (line 26 lists 10⁻³² as Planck time, but correct value is 10⁻⁴³ seconds)

---

## Documentation Sync (Identified 2026-02-09)

### Critical Priority
- [ ] docs: Fix camera fade system registration in genesis-ui/src/lib.rs - add CameraFadeState, setup_camera_fade_overlay, and update_camera_fade to GenesisUiPlugin

### High Priority
- [ ] docs: Add missing camera systems to ARCHITECTURE.md:93 - include handle_orbit_pan and update_camera_targets
- [ ] docs: Document particle helper functions in ARCHITECTURE.md:109 - include energy_to_color() and lerp_rgb()

### Medium Priority
- [ ] docs: Add camera_fade module exports to genesis-ui/src/overlay/mod.rs:9

### Low Priority
- [ ] docs: Clarify default camera mode in ARCHITECTURE.md:187 - distinguish between enum default (FreeFlight) and initial setup (Orbit)
- [ ] docs: Fix TimeConfig min value inconsistency in ARCHITECTURE.md:31 - update from 0.1 to 1.0 to match code
- [ ] docs: Remove #[allow(dead_code)] or document public API methods in genesis-core/src/epoch/camera_config.rs and genesis-core/src/time/mod.rs
