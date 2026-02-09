# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes (Blockers)

### Phase 1 Completeness Items

#### Particle System Scaling
- [x] feature: Scale particle system from 1000 to 100K-1M particles
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
- [ ] refactor: Remove unrequested camera smooth interpolation system
  - Remove interpolation fields, `start_interpolation_to_target()`, `start_interpolation_to_position_only()`, and `interpolate_camera()` system from `genesis-render/src/camera/mod.rs`, or verify if Phase 2+ requires this feature
- [ ] refactor: Remove unrequested CameraConfig fields
  - Remove `initial_position`, `initial_target`, and `movement_speed` from `genesis-core/src/config.rs` unless required for Phase 2+
- [ ] refactor: Remove unrequested ParticleConfig fields
  - Remove `particle_size_variation`, `color_hot`, and `color_cool` from `genesis-core/src/config.rs` unless required for Phase 2+
- [ ] refactor: Remove duplicate CameraMode enum
  - Remove `genesis-core/src/epoch/camera_config.rs` and use the enum from `genesis-render/src/camera/mod.rs`
- [ ] refactor: Remove epoch info overlay from Phase 1
  - Remove `show_epoch_info` field and related placeholder from `genesis-ui/src/overlay/mod.rs` (unless it's intentional for later phases)

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
