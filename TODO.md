# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Failing Tests & Warnings (Latest Test Run)

**Failing Tests (9 tests - AssetServer not initialized):**
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_materials_initialized_before_rendering (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_pipeline_cache_no_index_out_of_bounds (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resources_created_at_startup (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_complete_particle_rendering_setup (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_system_ordering_point_mesh_before_spawn (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_extract_system_transfers_data (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_particle_instance_bind_group_layout (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_camera_initialized_before_rendering (AssetServer not initialized)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resources_accessible_during_update (AssetServer not initialized)

**Compilation Warnings:**
- [x] fix: Remove unused import bytemuck::Zeroable from genesis-render/src/particle/instance_buffer.rs:31
- [x] fix: Remove unused import EguiSet from genesis-ui/src/overlay/mod.rs:7
- [x] fix: Remove unused manifest key workspace.dev-dependencies from Cargo.toml

**GPU Infrastructure Issue:**
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resource_reference_counting (GPU not available in CI/testing environment)

---

## Drift TODO Items (From Drift Analysis)

### Refactor Items
- [ ] refactor: Remove unrequested storage buffer infrastructure for Phase 1 - The GPU storage buffer system in genesis-render/src/particle/instance_buffer.rs is over-engineering for Phase 1 requirements

### Fix Items
- [ ] fix: Implement orbit camera zoom and pan - Orbit camera is missing zoom (scroll) and pan (middle/right mouse) controls specified in Phase 1
- [ ] fix: Implement smooth camera interpolation - Camera mode switching is instant, not smooth as specified in Phase 1
- [ ] fix: Implement epoch transition system - No EpochManager or EpochPlugin trait exists; epoch transitions are not implemented
- [ ] fix: Integrate timeline with epoch transitions - Timeline scrubbing doesn't trigger epoch transitions
- [ ] fix: Add epoch time range enforcement - Epoch time boundaries exist but are not enforced or integrated with the timeline

---

## Sprint 1 - Phase 1: The Singularity

### Phase 1 Completeness Items

### Code Cleanup

#### Remove Phase-Inappropriate Features
- [x] refactor: Remove unrequested TimeConfig fields from genesis-core/src/config.rs
  - Remove initial_time, initial_time_acceleration (not used in Phase 1)

#### Camera System Cleanup (Defer Non-Phase 1 Features)
- [x] refactor: Document camera mode switching as Phase 1 feature
  - Keep basic camera mode switching interpolation (FreeFlight ↔ Orbit) - this is PRD Phase 1 requirement
  - Document that advanced cinematic interpolation is Phase 7 feature
  - Ensure current CameraState.interpolation infrastructure serves only mode switching
- [ ] refactor: Verify orbit camera controls align with Phase 1 PRD
  - Per previous communication (architect-ambiguity-phase1-feature-scope), keep zoom and pan
  - These enhance UX for Phase 1 demo moment
  - Not explicitly prohibited in PRD
- [x] refactor: Remove test functions from camera module
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

#### Configuration Alignment
- [ ] fix: Align genesis.toml default particle count with PRD Phase 1 target
  - Change initial_count from 1000 to 100000 (100K minimum per PRD)
  - PRD Phase 1 deliverables specify "100K–1M point sprites" capability
- [ ] fix: Align genesis.toml default time acceleration with code default
  - Change initial_time_acceleration from 1e9 to 1.0 (matches TimeConfig::default())
  - Remove initial_time_acceleration field entirely if not required per PRD

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
- [ ] refactor: Remove debug print statements from genesis-render/src/particle/mod.rs
  - Remove println! statements at lines 161-162 and 272-278
  - Debug output not required per PRD Phase 1 deliverables
- [ ] refactor: Remove debug print statements from genesis-render/src/camera/mod.rs
  - Remove info! statements at lines 269 and 274
  - Debug output not required per PRD Phase 1 deliverables

### Repository Cleanup

#### Temporary Output Files

#### Leftover/Unused Files

#### Code Review Candidates
- [ ] review: genesis-core/src/physics/Particle struct is not used anywhere in codebase - consider if needed for future physics implementation or remove

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

### Drift Tracking (PRD vs Code Alignment)

#### Camera System Drift
- [ ] fix: Add smooth interpolation to camera mode switching - PRD Phase 1 specifies "smooth interpolation" for FreeFlight ↔ Orbit camera transitions, but toggle_camera_mode() uses instant switching (genesis-render/src/camera/mod.rs:251-277)

#### Particle System Drift
- [ ] fix: Add outward velocity to particle spawning - PRD Phase 1 specifies "particles spawned at origin with outward velocity", but spawn_particles() only spawns at origin without velocity (genesis-render/src/particle/mod.rs:299-300)
- [ ] fix: Implement timeline scrubbing with particle position reversal - PRD Demo Moment specifies "Scrub the timeline back and forth — the expansion reverses and replays", but timeline scrubbing only updates time display without reversing particle positions (genesis-ui/src/timeline/mod.rs:184-188)
- [ ] fix: Update Particle.position in update_particle_energy_colors - System uses particle.position to calculate energy but Particle.position is never updated with Transform.translation, creating synchronization issues (genesis-render/src/particle/mod.rs:377-390)

### New Drift Items (Code-PRD Gap - 2026-02-09)

#### Unrequested Features (Phase-Inappropriate Code)
- [ ] refactor: Remove GPU storage buffer infrastructure (genesis-render/src/particle/instance_buffer.rs) - PRD Phase 1 only requires "basic instanced rendering with position/color/size attributes", storage buffer with ExtractSchedule/Render world is advanced implementation not required for Phase 1
- [ ] refactor: Remove initial_time_acceleration field from TimeConfig - PRD Phase 1 only specifies "adjustable acceleration (1x to 10¹²x)", initial_time_acceleration is unrequested and code default is 1.0 (genesis-core/src/config.rs:27)
- [ ] refactor: Remove debug print statements from particle module - println! at lines 161-162 and 272-278 in genesis-render/src/particle/mod.rs are development artifacts not required per PRD Phase 1 deliverables
- [ ] refactor: Remove debug print statements from camera module - info! at lines 269 and 274 in genesis-render/src/camera/mod.rs are development artifacts not required per PRD Phase 1 deliverables
- [ ] refactor: Remove unused velocity calculation in spawn_particles - Variable 'velocity' is calculated but never stored on Particle component, adding unnecessary complexity (genesis-render/src/particle/mod.rs:309-310)

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)

---

## Sprint Finalization

- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.
