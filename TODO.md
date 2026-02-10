# TODO - Drift Analysis (Janitor Task 2026-02-10)

**Analysis:** Comparison between PRD.md (v2.0) and src/ implementation for Phase 1 deliverables

**Analysis Date:** 2026-02-10
**Analyzer:** Architect Mode

---

## Drift Summary

### Unrequested Features (Refactor Candidates)
*Features implemented in src/ but NOT specified in PRD.md Phase 1*

- [ ] refactor: Remove middle mouse button tracking (genesis-render/src/input/mod.rs:91)
  - Middle mouse button state tracking not specified in PRD Phase 1
  - PRD Phase 1 only mentions "orbit camera (click-drag)" which uses left mouse button
  - Middle mouse state is only used by unrequested orbit pan feature

- [ ] refactor: Remove orbit pan functionality (genesis-render/src/camera/mod.rs:490-549)
  - PRD Phase 1 only mentions "orbit camera (click-drag)" for rotation
  - Pan via middle mouse button (handle_orbit_pan) is not specified in Phase 1

- [ ] refactor: Remove orbit camera zoom via scroll wheel (genesis-render/src/camera/mod.rs:408-430)
  - PRD Phase 1 doesn't explicitly mention zoom controls for orbit camera
  - handle_orbit_zoom() system is unrequested

- [ ] refactor: Remove free-flight camera zoom via scroll wheel (genesis-render/src/camera/mod.rs:450-488)
  - PRD Phase 1 doesn't mention zoom for free-flight camera
  - PRD states: "Free-flight camera (WASD + mouse)" - only movement and look specified
  - handle_free_flight_zoom() system is unrequested

- [ ] refactor: Update outdated comment about camera interpolation status (genesis-render/src/camera/mod.rs:30)
  - Comment states "Camera interpolation: NOT implemented (deferred to Phase 7)"
  - Actual implementation: interpolate_camera() system exists at lines 641-686 and is registered at line 694
  - Camera interpolation IS implemented, comment should be corrected

### Contradictions with PRD (Fix Candidates)
*Features that contradict PRD.md Phase 1 requirements*

- [ ] fix: Implement Q/E key movement for vertical camera control (genesis-render/src/input/mod.rs:50-75)
  - PRD camera documentation at genesis-render/src/camera/mod.rs:72 states: "**Q/E**: Move down/up"
  - Current behavior: handle_keyboard_input() only implements WASD, Q/E keys not handled
  - This is a documentation contradiction - the docs say Q/E should work, but it's not implemented

### Missing Features from PRD (Implement Candidates)
*Features from PRD.md Phase 1 that are missing or incomplete*

### Items Verified as Correct (No Drift)

- ✓ Bevy application scaffold with window, input handling, and basic 3D scene (src/main.rs)
- ✓ Instanced particle renderer capable of displaying 100K–1M point sprites (genesis-render/src/particle/mod.rs)
- ✓ Free-flight camera with WASD + mouse look (genesis-render/src/camera/mod.rs:304-341, genesis-render/src/input/mod.rs:50-75)
- ✓ Orbit camera with click-drag rotation (genesis-render/src/camera/mod.rs:351-388)
- ✓ Camera smooth interpolation (genesis-render/src/camera/mod.rs:641-686) - despite outdated comment
- ✓ Cosmic time system with acceleration (1x to 10¹²x), pause, and reset (genesis-core/src/time/mod.rs:70-143)
- ✓ Logarithmic timeline scrubber UI spanning 13.8 billion years (genesis-ui/src/timeline/mod.rs:150-220)
- ✓ Procedural singularity visualization with energy-based coloring (genesis-render/src/particle/mod.rs:202-380)
- ✓ FPS counter and particle count overlay (genesis-ui/src/overlay/mod.rs:45-81)

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

# TODO - Current Sprint (Sprint 1: Phase 1 Completion)

**Sprint Goal:** Complete Phase 1 implementation by resolving remaining critical issues and passing Sprint QA.

---

## Critical Issues (Must Fix Before Sprint QA)

- [ ] fix: Timeline minimum range enhancement (BACKLOG.md line 15-21)
  - Current: CosmicTime.from_slider() uses effective_min=1.0 when min_time=0.0 (line 86, 104)
  - Issue: Cannot represent very early universe (< 1 year) in logarithmic timeline
  - Impact: Timeline cannot properly display pre-year-1 epochs (Planck boundary at 10⁻³²s, inflation at 10⁻³⁶s-10⁻³²s)
  - [ ] Update CosmicTime::from_slider() to handle min_time=0.0 properly for sub-year logarithmic scale
  - [ ] Update CosmicTime::to_slider() to return values < 0 for pre-1-year timescales
  - [ ] Test timeline scrubbing at t=10⁻³⁰s, t=10⁻⁶s to verify early universe accessibility

- [ ] feature: Configuration validation at load time (BACKLOG.md line 22-33)
  - Current: No validation of genesis.toml values when loaded via Config::load()
  - Issue: Invalid config values can cause runtime issues or undefined behavior
  - Impact: User can set invalid particle counts, time accelerations, etc.
  - [ ] Add Config::validate() method that checks all config values are within valid ranges
  - [ ] Call validate() in Config::load() and log warnings/errors for invalid values
  - [ ] Define validation rules:
    - particle.initial_count: clamp to [1000, 10000000]
    - particle.base_size: clamp to [0.1, 10.0]
    - time.time_acceleration_max: clamp to [1.0, 1e12]
    - window.width/height: clamp to [640, 7680]
  - [ ] Add unit tests for Config::validate() covering edge cases

- [x] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - compilation error due to missing fields `initial_position` and `initial_velocity` in Particle struct initialization (line 867)

---

## Sprint 1 - Phase 1: The Singularity (Finalization)

### Phase 1 Deliverables Status

The following Phase 1 PRD deliverables are IMPLEMENTED and verified:

- ✅ Bevy application scaffold with window, input handling, and basic 3D scene
- ✅ Instanced particle renderer capable of displaying 100K–1M point sprites
- ✅ Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation
- ✅ Cosmic time system: a f64 time accumulator with adjustable acceleration (1x to 10¹²x), pause, and reset
- ✅ Logarithmic timeline scrubber UI spanning 13.8 billion years
- ✅ Procedural "singularity" visualization: particles spawned at origin with outward velocity, color-mapped by energy
- ✅ FPS counter and particle count overlay
- ✅ Q/E key vertical movement for free-flight camera
- ✅ Scroll wheel zoom controls for both free-flight and orbit cameras
- ✅ Timeline reverse/replay capability: update_particles_for_scrubbing() system implemented

### Remaining Work

- [ ] Resolve timeline minimum range issue (see Critical Issues above)
- [ ] Implement configuration validation (see Critical Issues above)

---

### Documentation

---

- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.
