# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes (Blockers)
- [x] fix: Update genesis.toml time.initial_time_acceleration to match PRD Phase 1 starting range

### Phase 1 Completeness Items

#### Timeline Scrubber - Logarithmic Scale
- [ ] feature: Implement logarithmic timeline scrubber spanning 13.8 billion years (Sprint 1)
  - Replace linear timeline slider with logarithmic scrubber in genesis-ui/src/timeline/mod.rs
  - Update CosmicTime::from_slider() to use logarithmic mapping
  - Update CosmicTime::to_slider() to use logarithmic mapping
  - Formula: log_slider = log10(years / min_years) / log10(max_years / min_years)
  - Map slider range [0.0, 1.0] to years [10⁻³², 13.8×10⁹]
  - Add decade tick marks to timeline (10⁻³²s, 10⁻²⁰s, 1s, 1yr, 1Myr, 1Gyr, 13.8Gyr)
  - PRD Requirement: Line 116 - "Logarithmic timeline scrubber UI (bevy_egui) spanning 13.8 billion years"

#### Timeline Speed Integration
- [ ] feature: Map PlaybackState.speed slider to TimeAccumulator.acceleration with PRD-specified range (Sprint 1)
  - Implement logarithmic speed mapping: slider (0.1 to 10.0) → acceleration (1.0 to 1e12)
  - Formula: acceleration = 10^(slider_value * log10(1e12/1.0)) or similar logarithmic scale
  - Add system in sync_time_resources() to update acceleration when speed slider changes
  - Add visual feedback for current acceleration factor (display "10ⁿx" where n is exponent)
  - Document speed-to-acceleration mapping in timeline/mod.rs comments
  - PRD Requirement: Line 115 - "adjustable acceleration (1x to 10¹²x)"

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
- [ ] fix: Add smooth interpolation for camera mode switching between free-flight and orbit modes (PRD line 114 requires "smooth interpolation")
- [ ] fix: Implement EpochPlugin trait and EpochManager system as specified in PRD section 4.1 to support epoch transitions and timeline scrubbing
- [ ] fix: Add missing ParticleIndex component or remove the broken extract_particle_instances query in instance_buffer.rs
- [ ] refactor: Remove unrequested time conversion functions from genesis-core/src/time/mod.rs - seconds_to_years(), minutes_to_years() are not required for Phase 1
- [ ] refactor: Remove unrequested time constants from genesis-core/src/time/mod.rs - SECONDS_PER_MINUTE, SECONDS_PER_HOUR, SECONDS_PER_DAY are not in PRD Phase 1
- [ ] refactor: Remove unrequested TimeConfig fields from genesis-core/src/config.rs - initial_time, initial_time_acceleration are not used in Phase 1
- [ ] refactor: Remove unrequested CameraConfig fields from genesis-core/src/config.rs - initial_position, initial_target, movement_speed are not specified in PRD Phase 1
- [ ] refactor: Remove unrequested ParticleConfig fields from genesis-core/src/config.rs - particle_size_variation, color_hot, color_cool are not in PRD Phase 1
- [ ] refactor: Remove unrequested particle update systems from genesis-render/src/particle/mod.rs - update_particles() and update_particle_energy_colors() add physics and energy-based coloring not specified in Phase 1 PRD
- [ ] refactor: Remove "Epoch: Not implemented" placeholder from genesis-ui/src/overlay/mod.rs - unnecessary visual clutter for Phase 1

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
