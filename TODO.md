# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes (Blockers)
- [x] refactor: Remove camera interpolation - it's a Phase 7 feature per PRD but implemented in Phase 1
- [ ] refactor: Remove orbit camera zoom/pan - not specified in Phase 1 PRD but implemented
- [ ] refactor: Disable display.show_epoch_info config - this is a Phase 2+ feature enabled in Phase 1
- [ ] refactor: Remove unused config fields from genesis.toml that don't map to Phase 1 PRD
- [ ] fix: Align speed slider range with PRD - PRD specifies 1x-10¹²x but code has 0.1-10.0
- [ ] fix: Implement particle spawning at configured count - genesis.toml has 100K but only 1000 spawning
- [ ] fix: Update genesis.toml particle.initial_count to match Phase 1 testing (1000 instead of 100K)
- [ ] fix: Update genesis.toml time.initial_time_acceleration to match PRD Phase 1 starting range

### Phase 1 Completeness Items

#### Per-Instance Particle Attributes
- [ ] feature: Synchronize Particle component data with GPU instance attributes (Sprint 1)
  - Implement per-instance data transfer system for Particle.color and Particle.size
  - Update particle shaders to use instance_color and instance_size attributes
  - Ensure update_particle_energy_colors() changes affect rendering
  - This enables 10K-50K particle scaling in Sprint 1

### Code Cleanup

#### Remove Phase-Inappropriate Features
- [ ] refactor: Remove or simplify complex camera interpolation from CameraState
  - CameraState interpolation infrastructure is Phase 7 feature per PRD
  - Keep basic camera mode switching (FreeFlight ↔ Orbit) which is Phase 1
  - Document that full cinematic interpolation is deferred to Phase 7
- [ ] refactor: Remove unrequested camera smooth interpolation system
  - Remove interpolation fields, `start_interpolation_to_target()`, `start_interpolation_to_position_only()`, and `interpolate_camera()` system from `genesis-render/src/camera/mod.rs`, or verify if Phase 2+ requires this feature
- [ ] refactor: Remove unrequested orbit camera features
  - PRD Phase 1 only specifies "click-drag" for orbit camera control
  - Remove `handle_orbit_zoom()` system (scroll wheel zoom) from `genesis-render/src/camera/mod.rs` unless required for Phase 2+
  - Remove `handle_orbit_pan()` system (middle/right mouse button pan) from `genesis-render/src/camera/mod.rs` unless required for Phase 2+
  - Remove related OrbitController fields: `min_distance`, `max_distance`, `rotation_sensitivity`, `zoom_sensitivity`, `pan_sensitivity`
- [ ] refactor: Remove unrequested CameraConfig fields
  - Remove `initial_position`, `initial_target`, and `movement_speed` from `genesis-core/src/config.rs` unless required for Phase 2+
- [ ] refactor: Remove unrequested ParticleConfig fields
  - Remove `particle_size_variation`, `color_hot`, and `color_cool` from `genesis-core/src/config.rs` unless required for Phase 2+
- [ ] refactor: Remove duplicate CameraMode enum
  - Remove `genesis-core/src/epoch/camera_config.rs` and use the enum from `genesis-render/src/camera/mod.rs`
- [ ] refactor: Remove epoch info overlay from Phase 1
  - Comment out `show_epoch_info = true` in genesis.toml (line 32)
  - Remove `show_epoch_info` field and related placeholder from `genesis-ui/src/overlay/mod.rs` (unless it's intentional for later phases)
  - Keep DisplayConfig.show_epoch_info field for future use (Phase 2)
- [ ] fix: Align PlaybackState speed slider with PRD requirements (Sprint 1)
  - PRD Phase 1 specifies adjustable acceleration with range 1x to 10¹²x
  - Current UI slider range (0.1-10.0) in `genesis-ui/src/timeline/mod.rs` does not match PRD
  - Update slider range to match PRD specification (1x to 10¹²x)

#### Particle Scaling Implementation
- [ ] feature: Scale particle system to 10K-50K particles (configurable) (Sprint 1)
  - Implement configurable particle_count field in genesis.toml
  - Scale particle system to support 10K-50K particles
  - Add basic performance monitoring
  - Validate performance target at 50K particles (≥60 FPS)

#### Timeline Reverse/Replay
- [ ] feature: Implement basic timeline scrubbing to TimeAccumulator.years synchronization (Sprint 1)
  - Enable particles to move backward/forward when scrubbing the timeline
  - Basic synchronization with TimeAccumulator.years during timeline scrub
  - Note: Full snapshot-based reverse/replay system is Sprint 2 priority

### Drift Tracking (Code-PRD Gap)
- [ ] refactor: Remove camera interpolation system from CameraState (interpolation fields, interpolate_camera system) - Phase 1 only requires basic WASD+orbit controls, smooth interpolation is for Phase 7 cinematic mode
- [ ] refactor: Simplify particle rendering to remove per-instance GPU storage buffer architecture - Phase 1 only requires basic instanced rendering with position/color/size attributes
- [ ] refactor: Remove unused clap dependency from genesis-core/Cargo.toml - PRD doesn't specify command-line argument parsing
- [ ] fix: Implement Epoch Plugin Architecture per PRD section 4.1 - convert SingularityEpoch from marker struct to actual Bevy plugin, create EpochManager for epoch transitions
- [ ] fix: Add missing ParticleIndex component or remove the broken extract_particle_instances query in instance_buffer.rs

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
