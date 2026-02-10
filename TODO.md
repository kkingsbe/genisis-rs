# TODO - Drift Analysis (Janitor Task 2026-02-10)

**Analysis:** Comparison between PRD.md (v2.0) and src/ implementation for Phase 1 deliverables

---

## Drift Summary

### Unrequested Features (Refactor Candidates)
*Features implemented in src/ but NOT specified in PRD.md Phase 1*

- [ ] refactor: Remove Q/E vertical camera movement (genesis-render/src/input/mod.rs:73-78)
  - PRD Phase 1 doesn't specify vertical movement controls
  - Only WASD horizontal movement mentioned in Phase 1 Deliverables

- [ ] refactor: Remove middle mouse button tracking (genesis-render/src/input/mod.rs:100)
  - Middle mouse button state tracking not specified in PRD Phase 1
  - Middle mouse panning is unrequested feature

- [ ] refactor: Remove orbit pan functionality (genesis-render/src/camera/mod.rs:409-445)
  - PRD Phase 1 only mentions "orbit camera (click-drag)" for rotation
  - Pan via middle mouse button is not specified

- [ ] refactor: Remove scroll wheel zoom for both camera modes (genesis-render/src/camera/mod.rs:319-389)
  - PRD Phase 1 doesn't explicitly mention zoom controls
  - Both handle_orbit_zoom() and handle_free_flight_zoom() are unrequested

### Contradictions with PRD (Fix Candidates)
*Features that contradict PRD.md Phase 1 requirements*

- [ ] fix: Implement smooth camera interpolation between modes (genesis-render/src/camera/mod.rs:28)
  - PRD Phase 1 Deliverable specifies: "Free-flight camera (WASD + mouse) and orbit camera (click-drag) with **smooth interpolation**"
  - Current implementation: "Camera interpolation: NOT implemented (deferred to Phase 7)"
  - This is a direct contradiction of Phase 1 requirements

### Missing Features from PRD (Implement Candidates)
*Features from PRD.md Phase 1 that are missing or incomplete*

- [ ] implement: Timeline reverse/replay for particle positions (genesis-ui/src/timeline/mod.rs:142-208)
  - PRD Phase 1 Demo Moment: "Scrub the timeline back and forth — the expansion reverses and replays"
  - Current behavior: Timeline scrubbing updates cosmic_time but particles don't move backward
  - Note: Already tracked in existing TODO below (Timeline Enhancements section)

---

## Notes on Non-Issues

### Items NOT Flagged as Drift (Correctly Deferred)
- genesis-physics crate: PRD Section 4.2 shows this is Phase 2 deliverable
- genesis-export crate: PRD Section 4.2 shows this is Phase 5 deliverable
- genesis-audio crate: PRD Section 4.2 shows this is Phase 6 deliverable
- genesis-bench crate: PRD Section 4.2 shows this is Phase 7 deliverable
- Epoch indicator UI: PRD Section 2 Deliverables show this is Phase 2 feature
- Phase 7 features (cinematic mode, data overlays, etc.): Correctly deferred to Phase 7

---

# TODO - Current Sprint (Sprint 2: Singularity Refinement)

**Sprint Goal:** Complete Phase 1 Singularity implementation with particle velocity, position synchronization, and configuration alignment.

---

## Sprint 2 - Phase 1: The Singularity (Refinement)

### Camera Controls (Phase 1 PRD Requirements)

### Timeline Enhancements (Phase 1 PRD Requirements)

- [ ] feature: Timeline reverse/replay capability (PRD Phase 1 Demo Moment requires "Scrub the timeline back and forth")
  - Location: genesis-ui/src/timeline/mod.rs
  - Current: Timeline scrubbing updates cosmic_time but particles don't move backward
  - PRD reference: Section 5, Phase 1 Demo Moment - "Scrub the timeline back and forth — the expansion reverses and replays"

### Code Cleanup (Non-Blocking)


### Documentation

---

- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

### Test Health - Failing Tests
- [x] fix: Failing compilation in genesis-render/src/camera/mod.rs from recent commit - Rust borrow checker errors at lines 595, 597, 657, 719, 781, 825, 875, 938 (completed 2026-02-10 - fixed by extracting controller values before mutable Transform borrow)
  - Error: cannot borrow `world` as mutable because it is also borrowed as immutable
  - Issue: world.get::<CameraController>() and world.get_mut::<Transform>() cannot be held simultaneously
  - Impact: Blocks test suite from running (cargo test fails with 8 compilation errors)
