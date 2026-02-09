# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes
- [x] fix: Failing test in genesis-render/src/particle/instance_buffer.rs - test_particle_instance_data_alignment failed (expected alignment 16, got 4)

#### Failing Integration Tests (from commit 8578141)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_comprehensive_binding_validation - Vertex input must have instance_size at location(1) to match ATTRIBUTE_INSTANCE_SIZE
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_system_cannot_access_invalid_resources - requires_non_existent could not access system parameter Res<'_, NonExistentResource>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_materials_initialized_before_rendering - Assets<PointSpriteMaterial> does not exist in the World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resource_reference_counting - Assets<Mesh> does not exist in the World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resources_created_at_startup - init_point_mesh could not access system parameter ResMut<'_, Assets<Mesh>>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_extract_system_transfers_data - init_point_mesh could not access system parameter ResMut<'_, Assets<Mesh>>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_pipeline_cache_no_index_out_of_bounds - init_point_mesh could not access system parameter ResMut<'_, Assets<Mesh>>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_point_mesh_initialized_before_particles_spawn - init_point_mesh could not access system parameter ResMut<'_, Assets<Mesh>>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_system_ordering_point_mesh_before_spawn - init_point_mesh could not access system parameter ResMut<'_, Assets<Mesh>>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_complete_particle_rendering_setup - init_point_mesh could not access system parameter ResMut<'_, Assets<Mesh>>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resources_accessible_during_update - init_point_mesh could not access system parameter ResMut<'_, Assets<Mesh>>
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_particle_instance_bind_group_layout - AssetServer does not exist in the World
- [ ] fix: Failing test in genesis-render/tests/shader_tests.rs - test_vertex_attribute_locations_match - WGSL @location(1) must be 'instance_size' to match ATTRIBUTE_INSTANCE_SIZE
- [ ] fix: Failing test in genesis-render/tests/shader_tests.rs - test_print_all_bindings - Shader should have exactly 3 bindings at @group(0), found 4
- [ ] fix: Failing test in genesis-render/tests/shader_tests.rs - test_comprehensive_shader_validation_summary - Missing @location(1) instance_size

### Phase 1 Completeness Items

### Code Cleanup

#### Remove Phase-Inappropriate Features
- [x] refactor: Remove unrequested time conversion functions from genesis-core/src/time/mod.rs
  - Remove seconds_to_years(), minutes_to_years() (not required for Phase 1)
- [x] refactor: Remove unrequested time constants from genesis-core/src/time/mod.rs
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

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

### Repository Cleanup

#### Temporary Output Files
- [x] chore: Remove .architect-output-1770673436052.md - temporary architect mode output file
- [x] chore: Remove .architect-output-1770673991273.md - temporary architect mode output file
- [x] chore: Remove .janitor-output-1770672479399.md - temporary janitor mode output file
- [x] chore: Remove .janitor-output-1770673025376.md - temporary janitor mode output file

#### Leftover/Unused Files
- [x] chore: Remove bin/run.bat - contains hardcoded paths to another user's directory (c:\Users\Kyle\Documents\code\agent-coding-container\automation-parallel), not part of this Rust project
- [x] chore: Remove commit-msg.md - saved commit message from past commit, not a template file

#### Code Review Candidates
- [ ] review: genesis-core/src/physics/Particle struct is not used anywhere in codebase - consider if needed for future physics implementation or remove

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
