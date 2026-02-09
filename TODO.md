# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Critical Fixes (Blockers)

#### Shader & Assets
- [x] fix: Resolve ViewUniform shader compilation error
  - Add ViewUniform struct definition to genesis-render/src/particle/point_sprite.wgsl
  - Define struct with view_proj: mat4x4<f32> and world_position: vec3<f32>
  - See ARCHITECTURE.md lines 388-425 for details

- [x] fix: Create assets/ directory and copy point_sprite.wgsl
  - Recreate assets/ directory at project root
  - Copy genesis-render/src/particle/point_sprite.wgsl to assets/point_sprite.wgsl
  - Follows Bevy's standard asset path convention
  - See ARCHITECTURE.md lines 369-386 for details

#### Configuration Field Mismatches
- [ ] fix: Reconcile ParticleConfig field names between genesis.toml and struct
  - genesis.toml: initial_count, max_count, base_size
  - ParticleConfig struct: particle_count, particle_size_base, particle_size_variation, color_hot, color_cool
  - Update genesis.toml to match struct fields OR update struct to match genesis.toml
  - Ensure spawn_particles() uses correct field names

- [ ] fix: Reconcile TimeConfig field names between genesis.toml and struct
  - genesis.toml: initial_time_acceleration
  - TimeConfig struct: default_time_acceleration
  - Add initial_time_acceleration field to TimeConfig struct
  - Update genesis-core/src/config.rs lines 39-60

#### OverlayState Missing Field
- [ ] fix: Add show_epoch_info field to OverlayState struct
  - Add `pub show_epoch_info: bool` field to genesis-ui/src/overlay/mod.rs OverlayState struct
  - Update update_overlay_ui() system to display epoch information when enabled
  - Remove main.rs TODO comment about missing field (lines 54-55)
  - Ensure OverlayState::default() initializes show_epoch_info = true

#### Timeline Scrubbing Synchronization
- [ ] fix: Synchronize timeline scrubbing with TimeAccumulator.years
  - When timeline slider changes, update TimeAccumulator.years to match CosmicTime.cosmic_time
  - Modify timeline_panel_ui() in genesis-ui/src/timeline/mod.rs
  - Add event or direct assignment: time_accumulator.set_years(cosmic_time.get_time())
  - Remove main.rs TODO comment about dual time system (lines 21-22)

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
- [ ] docs: Update ARCHITECTURE.md to reflect Phase 1 scope
  - Remove references to epoch transition systems that don't exist
  - Document that epoch management is Phase 2+ scope
  - Clarify current scope: single epoch (Singularity) only

- [ ] docs: Remove outdated TODO comments from main.rs
  - Lines 21-22: Config::load() TODO (will be implemented)
  - Lines 49-51: CameraConfig field access TODO (fields already match)
  - Lines 54-55: OverlayState show_epoch_info TODO (will be implemented)

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint Status

**Current Sprint:** Sprint 1 - Phase 1: The Singularity
**Status:** In Progress (no .sprint_complete file exists)
**Next Sprint:** Sprint 2 - Phase 2: Inflation & Quantum Seeds (locked until current sprint completes)
