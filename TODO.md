# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes
- [x] fix: Failing test in genesis-render/src/particle/instance_buffer.rs - test_particle_instance_data_alignment failed (expected alignment 16, got 4)

### Phase 1 Completeness Items

### Code Cleanup

#### Remove Phase-Inappropriate Features
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

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
