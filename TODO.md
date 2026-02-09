# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes (Blockers)

### Phase 1 Completeness Items
- [x] Run all tests to verify current state
  - Tests exist in genesis-render/tests/ directory (resource_binding_tests.rs and shader_tests.rs)
  - Tests are properly configured with dev-dependencies in Cargo.toml
  - Tests fail to compile due to Bevy 0.15 API migration issues (66 compilation errors)
  - Previous cargo test showed "running 0 tests" because tests couldn't compile, not because they didn't exist

#### Timeline Speed Integration
- [x] feature: Map PlaybackState.speed slider to TimeAccumulator.acceleration with PRD-specified range (Sprint 1)
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
- [ ] refactor: Remove unrequested CameraConfig fields (Phase 2+ features in Phase 1)
  - Remove `initial_position`, `initial_target`, and `movement_speed` from `genesis-core/src/config.rs` unless required for Phase 2+
- [ ] refactor: Remove unrequested ParticleConfig fields (Phase 2+ features in Phase 1)
  - Remove `particle_size_variation`, `color_hot`, and `color_cool` from `genesis-core/src/config.rs` unless required for Phase 2+
- [ ] refactor: Remove duplicate CameraMode enum
  - Remove `genesis-core/src/epoch/camera_config.rs` and use the enum from `genesis-render/src/camera/mod.rs`
- [ ] refactor: Remove epoch info overlay from Phase 1 (Phase 2+ feature)
  - Comment out `show_epoch_info = true` in genesis.toml (line 32)
  - Remove `show_epoch_info` field and related placeholder from `genesis-ui/src/overlay/mod.rs` (unless it's intentional for later phases)
  - Keep DisplayConfig.show_epoch_info field for future use (Phase 2)
- [ ] refactor: Remove unrequested time conversion functions from genesis-core/src/time/mod.rs
  - Remove seconds_to_years(), minutes_to_years() (not required for Phase 1)
- [ ] refactor: Remove unrequested time constants from genesis-core/src/time/mod.rs
  - Remove SECONDS_PER_MINUTE, SECONDS_PER_HOUR, SECONDS_PER_DAY (not in PRD Phase 1)
- [ ] refactor: Remove unrequested TimeConfig fields from genesis-core/src/config.rs
  - Remove initial_time, initial_time_acceleration (not used in Phase 1)

#### Camera System Cleanup (Defer Non-Phase 1 Features)
- [ ] refactor: Document camera mode switching as Phase 1 feature
  - Keep basic camera mode switching interpolation (FreeFlight ↔ Orbit) - this is PRD Phase 1 requirement
  - Document that advanced cinematic interpolation is Phase 7 feature
  - Ensure current CameraState.interpolation infrastructure serves only mode switching
- [ ] refactor: Verify orbit camera controls align with Phase 1 PRD
  - Per previous communication (architect-ambiguity-phase1-feature-scope), keep zoom and pan
  - These enhance UX for Phase 1 demo moment
  - Not explicitly prohibited in PRD
- [ ] refactor: Remove test functions from camera module
  - Remove `test_interpolation()` development testing function (triggered by 'T' key)
  - This is not specified in PRD

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

### Drift Tracking (Code-PRD Gap Consolidated)

#### Items Deferred to Future Phases
- [ ] fix: Implement Epoch Plugin Architecture per PRD section 4.1 (Phase 2+ feature)
  - Convert SingularityEpoch from marker struct to actual Bevy plugin
  - Create EpochManager for epoch transitions
  - PRD section 4.1 specifies Epoch Plugin Architecture for Phase 2+
  - Sprint 1 uses single epoch (Singularity) without transitions
- [ ] fix: Implement EpochPlugin trait and EpochManager system (Phase 2+ feature)
  - Support epoch transitions and timeline scrubbing across phases
  - PRD section 4.1 specifies this for Phase 2+

#### Items for Investigation (Non-Blocking)
- [ ] refactor: Simplify particle rendering architecture
  - Review per-instance GPU storage buffer architecture
  - Phase 1 requires basic instanced rendering with position/color/size attributes
  - Determine if current implementation can be simplified without breaking PRD requirements
- [ ] refactor: Remove unused clap dependency
  - Review genesis-core/Cargo.toml for unused dependencies
  - PRD doesn't specify command-line argument parsing
- [ ] fix: Remove "Epoch: Not implemented" placeholder
  - Remove from genesis-ui/src/overlay/mod.rs
  - Unnecessary visual clutter for Phase 1

### Sprint QA
- [ ] fix: Migrate genesis-render/tests to Bevy 0.15 API (moved from BLOCKERS)
  - ANALYSIS COMPLETE: 66 compilation errors found across both test files
  - Tests ARE properly configured and would run if they compiled successfully
  - The "running 0 tests" output was due to compilation failures, not missing test configuration
  - Key API changes needed (confirmed by compilation errors):
    * ScheduleRunnerSettings no longer exists - use ScheduleRunnerPlugin::run_once() without arguments (lines 44-47, 348-351, 394-397, 436-439, 682-685, 1039-1042, 1092-1095, 1126-1129, 1160-1163, 1197-1200, 1230-1233, 1303-1306)
    * Camera3d renamed to Camera - use bevy::render::camera::Camera::default() (line 445)
    * World::add_systems() doesn't exist - use App::add_systems() instead (lines 697, 1184, 1246, 1255, 1318, 1330)
    * Entities::iter() method removed - use new entity query methods (lines 1275, 1287)
    * Mesh::ATTRIBUTE_COLOR_0 changed to Mesh::ATTRIBUTE_COLOR (line 913)
    * Color::RED renamed to bevy::color::palettes::css::RED (line 939)
    * LinearRgba::is_normalized() method removed (line 949)
    * AssetPath::to_str() changed to to_string() (lines 1473, 1477)
    * ParticleInstanceData::zeroed() requires Zeroable trait import (line 972)
    * Type inference issues with next_power_of_two() - specify type (lines 1075, 1079)
    * Additional minor warnings for unused variables and imports
  - Update both resource_binding_tests.rs and shader_tests.rs
  - Verify all test code compiles successfully with Bevy 0.15 APIs
  - Run updated tests to confirm they pass
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
