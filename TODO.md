# TODO - Current Sprint (Sprint 2: Singularity Refinement)

**Sprint Goal:** Complete Phase 1 Singularity implementation with particle velocity, position synchronization, and configuration alignment.

---

## Sprint 2 - Phase 1: The Singularity (Refinement)

### Camera Controls (Phase 1 PRD Requirements)

- [ ] fix: Implement Q/E up/down movement for free-flight camera (PRD Phase 1 requirement)
  - Location: genesis-render/src/input/mod.rs handle_keyboard_input
  - Current: Q/E keys documented in CameraMode enum comments but not implemented
  - Add Q key for downward movement (negative Y direction)
  - Add E key for upward movement (positive Y direction)
  - Note: Scroll wheel zoom for free-flight camera is ALREADY IMPLEMENTED (handle_free_flight_zoom exists)

### Timeline Enhancements (Phase 1 PRD Requirements)

- [ ] feature: Implement basic timeline scrubbing to TimeAccumulator synchronization
  - [ ] Enable particles to move backward/forward when scrubbing the timeline
  - [ ] Basic synchronization with TimeAccumulator.years during timeline scrub
  - [ ] Note: Full snapshot-based reverse/replay system is future sprint priority
- [ ] feature: Timeline reverse/replay capability (PRD Phase 1 Demo Moment requires "Scrub the timeline back and forth")
  - Location: genesis-ui/src/timeline/mod.rs
  - Current: Timeline scrubbing updates cosmic_time but particles don't move backward
  - PRD reference: Section 5, Phase 1 Demo Moment - "Scrub the timeline back and forth — the expansion reverses and replays"

### Code Cleanup (Non-Blocking)

- [x] refactor: Remove debug print statements from genesis-render/src/particle/mod.rs
  - [ ] Remove println! statements at lines 266-272
  - [ ] Remove println! statements at lines 318-320
  - Debug output not required per PRD Phase 1 deliverables
- [x] refactor: Remove debug print statements from genesis-render/src/camera/mod.rs
  - [ ] Remove info! statements at lines 269 and 274
  - Debug output not required per PRD Phase 1 deliverables

### Documentation

- [x] doc: Update ARCHITECTURE.md to reflect Particle component changes
  - [x] Document new velocity field in Particle component
  - [x] Document sync_particle_position() system
  - [x] Update Phase 1 implementation status

---

## Drift Remediation (Unrequested Features)

The following features are implemented but NOT specified in PRD Phase 1 requirements:

- [ ] refactor: Remove orbit camera zoom controls (handle_orbit_zoom in genesis-render/src/camera/mod.rs)
  - PRD Phase 1 only specifies "orbit camera (click-drag)" for rotation, not zoom controls
  - Location: genesis-render/src/camera/mod.rs lines 319-336
- [ ] refactor: Remove orbit camera pan controls (handle_orbit_pan in genesis-render/src/camera/mod.rs)
  - PRD Phase 1 only specifies "orbit camera (click-drag)" for rotation, not pan controls
  - Location: genesis-render/src/camera/mod.rs lines 409-445
- [ ] refactor: Remove free-flight zoom controls (handle_free_flight_zoom in genesis-render/src/camera/mod.rs)
  - PRD Phase 1 only specifies "Free-flight camera (WASD + mouse)" without zoom controls
  - Location: genesis-render/src/camera/mod.rs lines 356-389

---

## SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

### Test Health - Failing Tests
