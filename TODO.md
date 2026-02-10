# TODO - Current Sprint (Sprint 2: Singularity Refinement)

**Sprint Goal:** Complete Phase 1 Singularity implementation with particle velocity, position synchronization, and configuration alignment.

---

## Sprint 2 - Phase 1: The Singularity (Refinement)

### Critical Bug Fixes (Blockers for Demo Moment)

#### Particle Instance Buffer
- [x] fix: Compilation error in genesis-render/src/particle/instance_buffer.rs - missing `use bytemuck::Zeroable;` import at line 31 causes `ParticleInstanceData::zeroed()` to fail at line 315 in test_particle_instance_data_zeroable

#### Particle Velocity System
- [x] fix: Add velocity field to Particle component (CRITICAL - Blocks proper particle expansion per PRD Phase 1)
  - [ ] Add `velocity: Vec3` field to Particle struct in genesis-render/src/particle/mod.rs
  - [ ] Update spawn_particles() to store calculated velocity in Particle component (line 302-304)
  - [ ] Modify Particle component initialization: Particle { position, color, size, velocity }
  - [ ] Update update_particles() to use stored Particle.velocity instead of hardcoded speed (line 337)

#### Particle Position Synchronization
- [ ] fix: Sync Particle.position with Transform.translation (CRITICAL - Breaks energy-based coloring per PRD Phase 1)
  - [ ] Add sync_particle_position() system that copies Transform.translation to Particle.position each frame
  - [ ] Query (Entity, &Transform, &mut Particle) and update particle.position from transform.translation
  - [ ] Register sync_particle_position() system in Update schedule before update_particle_energy_colors
  - [ ] This ensures update_particle_energy_colors() calculates energy from actual particle positions

### Configuration Alignment
- [ ] clarify: Resolve genesis.toml initial_count discrepancy (GAP ANALYSIS 2026-02-10)
  - Current genesis.toml: initial_count = 1000
  - Code default (ParticleConfig::default()): initial_count = 100_000
  - PRD Phase 1: "100K–1M point sprites" capability
  - Decision needed: Should genesis.toml default be 100000 to match code default and PRD?
- [ ] fix: Update genesis.toml initial_count based on decision (AFTER CLARIFICATION)

### Particle Scaling
- [ ] feature: Scale particle system to 10K-50K particles (configurable)
  - [ ] Implement adaptive particle spawning system that scales based on config.particle.initial_count
  - [ ] Add performance monitoring to ensure target FPS with increasing particle counts
  - [ ] Optimize spawn_particles() to handle 10K+ entities efficiently (use batch spawning)
  - [ ] Validate performance target at 10K particles (≥60 FPS)

### Camera Controls
- [ ] feature: Implement scroll wheel zoom controls for orbit camera
  - [ ] Add scroll wheel event handling to orbit camera system
  - [ ] Implement zoom with distance clamping (min_distance=5.0, max_distance=200.0)
  - [ ] Add handle_orbit_zoom() system in genesis-render/src/camera/mod.rs
- [ ] feature: Implement pan controls for orbit camera
  - [ ] Add middle mouse button drag detection to InputState
  - [ ] Implement pan system that moves orbit target point based on mouse drag
  - [ ] Add handle_orbit_pan() system in genesis-render/src/camera/mod.rs

### Timeline Enhancements
- [ ] feature: Implement basic timeline scrubbing to TimeAccumulator synchronization
  - [ ] Enable particles to move backward/forward when scrubbing the timeline
  - [ ] Basic synchronization with TimeAccumulator.years during timeline scrub
  - [ ] Note: Full snapshot-based reverse/replay system is future sprint priority

### Code Cleanup (Non-Blocking)
- [ ] refactor: Remove debug print statements from genesis-render/src/particle/mod.rs
  - [ ] Remove println! statements at lines 266-272
  - [ ] Remove println! statements at lines 318-320
  - [ ] Debug output not required per PRD Phase 1 deliverables
- [ ] refactor: Remove debug print statements from genesis-render/src/camera/mod.rs
  - [ ] Remove info! statements at lines 269 and 274
  - [ ] Debug output not required per PRD Phase 1 deliverables

### Documentation
- [ ] doc: Update ARCHITECTURE.md to reflect Particle component changes
  - [ ] Document new velocity field in Particle component
  - [ ] Document sync_particle_position() system
  - [ ] Update Phase 1 implementation status

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint Status

**Current Sprint:** Sprint 2 - Phase 1: Singularity (Refinement)
**Status:** In Progress (no .sprint_complete file exists for Sprint 2)
**Previous Sprint:** Sprint 1 - Phase 1: The Singularity (Completed 2026-02-10)
**Next Sprint:** Sprint 3 - TBD (locked until current sprint completes)

---

## Sprint Finalization

- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Janitor: Drift from PRD.md

### Unrequested Features
- [ ] refactor: Remove unrequested custom vertex attributes (Location: genesis-render/src/particle/mod.rs)
- [ ] refactor: Remove unused particle update systems (Location: genesis-render/src/particle/mod.rs)
- [ ] refactor: Remove unrequested GPU storage buffer infrastructure (Location: genesis-render/src/particle/instance_buffer.rs)
- [ ] refactor: Remove unrequested unit tests (Location: genesis-render/src/particle/instance_buffer.rs)
- [ ] refactor: Remove unrequested DisplayConfig (Location: genesis-core/src/config.rs)

### Contradictions
- [ ] fix: Align particle count default with PRD requirement (genesis.toml has 1000 vs 100K minimum PRD requirement)
- [ ] fix: Align camera interpolation with PRD requirement (Phase 1 requires smooth interpolation)
- [ ] fix: Align particle movement with PRD "explosion" requirement (color cooling effect is non-functional)
- [ ] fix: Align timeline reverse replay with PRD requirement (no reverse replay on timeline scrubbing)
- [ ] fix: Align input with Q/E up/down movement requirement (Q/E not implemented but documented)
