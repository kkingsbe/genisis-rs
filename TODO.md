# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes (Blockers)

#### Camera Configuration
- [ ] fix: Resolve CameraConfig field access in setup_camera
  - main.rs line 69 uses config.camera.orbit_distance which EXISTS
  - Remove outdated TODO comment in main.rs (lines 49-51)
  - Confirm CameraState::from_config() correctly handles camera_mode String

### Phase 1 Completeness Items

#### Particle System Scaling
- [ ] feature: Scale particle system from 1000 to 100K-1M particles
  - Implement adaptive particle spawning based on config.particle.initial_count
  - Add performance monitoring to ensure target FPS with increasing particle counts
  - Optimize spawn_particles() to handle 100K+ entities efficiently

#### Per-Instance Particle Attributes
- [ ] feature: Synchronize Particle component data with GPU instance attributes
  - Implement per-instance data transfer system for Particle.color and Particle.size
  - Update particle shaders to use instance_color and instance_size attributes
  - Ensure update_particle_energy_colors() changes affect rendering

### Code Cleanup

#### Remove Phase-Inappropriate Features
- [ ] refactor: Remove or simplify complex camera interpolation from CameraState
  - CameraState interpolation infrastructure is Phase 7 feature per PRD
  - Keep basic camera mode switching (FreeFlight ↔ Orbit) which is Phase 1
  - Document that full cinematic interpolation is deferred to Phase 7

#### Documentation Cleanup
- [x] docs: Remove outdated TODO comments from main.rs
  - Lines 21-22: Config::load() TODO (will be implemented) - REMOVED
  - Lines 49-51: CameraConfig field access TODO (fields already match) - NOT FOUND (already implemented)
  - Lines 54-55: OverlayState show_epoch_info TODO (will be implemented) - NOT FOUND (already implemented)

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
