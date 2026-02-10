# TODO - Current Sprint (Sprint 2: Singularity Refinement)

**Sprint Goal:** Complete Phase 1 Singularity implementation with particle velocity, position synchronization, and configuration alignment.

---

## Janitor: Flag Drift (2026-02-10)

### Phase 1 PRD Contradictions

- [ ] fix: Implement orbit camera zoom (scroll wheel) - PRD Phase 1 requires "orbit camera (click-drag)" with zoom controls
  - Location: genesis-render/src/camera/mod.rs, genesis-render/src/input/mod.rs
  - Current: handle_orbit_zoom system does not exist, scroll wheel input not tracked
  - PRD reference: Section 5, Phase 1 deliverables
- [ ] fix: Implement orbit camera pan - PRD Phase 1 requires complete orbit camera controls
  - Location: genesis-render/src/camera/mod.rs, genesis-render/src/input/mod.rs
  - Current: handle_orbit_pan system does not exist, middle/right mouse not tracked
  - PRD reference: Section 5, Phase 1 deliverables
- [ ] fix: Implement Q/E up/down movement for free-flight camera
  - Location: genesis-render/src/camera/mod.rs
  - Current: Q/E keys documented in comments but not implemented in handle_keyboard_input
  - PRD reference: genesis-render/src/camera/mod.rs:68-71

### Missing PRD-Specified Features (Phase 1)

- [ ] implement: Time acceleration initial value configuration
  - Location: genesis-core/src/config.rs TimeConfig struct
  - Current: No initial_time_acceleration field, acceleration always starts at 1.0
  - PRD reference: Section 5, Phase 1 deliverables - "Cosmic time system: ... with adjustable acceleration"
- [ ] implement: Timeline reverse replay on scrubbing
  - Location: genesis-ui/src/timeline/mod.rs
  - Current: Timeline scrubbing updates cosmic_time but particles don't move backward
  - PRD reference: Section 5, Phase 1 Demo Moment - "Scrub the timeline back and forth — the expansion reverses and replays"

### Architecture Drift (Epoch Plugin System)

- [ ] implement: EpochPlugin trait and EpochManager
  - Location: genesis-core/src/epoch/
  - Current: SingularityEpoch is only a marker struct; no EpochPlugin trait or EpochManager
  - PRD reference: Section 4.1 "Epoch Plugin Architecture" - specifies each epoch as a Bevy plugin that registers systems, renderers, and UI panels
- [ ] implement: Epoch Manager for transitions
  - Location: genesis-core/src/epoch/mod.rs
  - Current: No EpochManager exists
  - PRD reference: Section 4.1 - "The manager handles transitions (crossfade blending, parameter interpolation)"

### Missing Tech Stack Components

- [ ] implement: Add nalgebra crate for scientific linear algebra
  - PRD reference: Section 4 - "Math: glam + nalgebra"
  - Current: Only glam is used; nalgebra not in Cargo.toml
- [ ] implement: Add hdf5-rust crate for snapshot export
  - PRD reference: Section 4 - "Serialization: serde + hdf5-rust"
  - Current: serde present; hdf5-rust not in Cargo.toml
- [ ] implement: Add kira (bevy_kira_audio) crate for audio
  - PRD reference: Section 4 - "Audio: kira (bevy_kira_audio)"
  - Current: Audio crate not present

### Missing Future Phase Crates

- [ ] implement: genesis-physics crate
  - PRD reference: Section 4.2 - "First Used: Phase 2"
  - Components: Gravity, SPH, nucleosynthesis, inflation, perturbations
- [ ] implement: genesis-export crate
  - PRD reference: Section 4.2 - "First Used: Phase 5"
  - Components: HDF5, VTK, PNG/EXR, CSV export pipelines
- [ ] implement: genesis-audio crate
  - PRD reference: Section 4.2 - "First Used: Phase 6"
  - Components: Procedural soundscape, epoch-aware audio mixing
- [ ] implement: genesis-bench crate
  - PRD reference: Section 4.2 - "First Used: Phase 7"
  - Components: Benchmarking harness, performance regression tests

### Investigate Potential Drift (Low Priority)

- [ ] clarify: Verify energy_to_color() color gradient matches PRD requirements
  - Location: genesis-render/src/particle/mod.rs
  - Current: WHITE → YELLOW → ORANGE → RED → DARK_RED gradient
  - PRD reference: Section 5, Phase 1 Demo Moment - "cooling from white to yellow to red"
  - Note: PRD mentions "color-mapped by energy (white-hot core fading to red)" - current implementation seems aligned

---

## Sprint 2 - Phase 1: The Singularity (Refinement)

### Critical Bug Fixes (Blockers for Demo Moment)

#### Test Compilation Errors
(No pending items)

#### Particle Position Synchronization
(No pending items)

### Configuration Alignment
(No pending items)

### Particle Scaling
- [x] feature: Scale particle system to 10K-50K particles (configurable)
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

---

[ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.
