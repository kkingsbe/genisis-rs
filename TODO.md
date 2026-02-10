# TODO - Current Sprint (Sprint 1: Phase 1 Completion)

**Note:** Drift analysis was completed on 2026-02-10. All Phase 1 deliverables are verified as implemented. The only outstanding issue is a documentation fix for Q/E keys (fixed) and failing tests in camera module.

**Sprint Goal:** Complete Phase 1 implementation by resolving remaining critical issues and passing Sprint QA.

---

## Critical Issues (Must Fix Before Sprint QA)

- [ ] fix: Failing tests in genesis-render/src/camera/mod.rs - 7 test failures in handle_free_flight_zoom tests (test_handle_free_flight_zoom_comprehensive, test_handle_free_flight_zoom_respects_zoom_speed, test_handle_free_flight_zoom_clamps_maximum, test_handle_free_flight_zoom_clamps_minimum, test_handle_free_flight_zoom_moves_forward, test_handle_free_flight_zoom_respects_camera_rotation, test_handle_free_flight_zoom_moves_backward)
  - Error: Tests expect handle_free_flight_zoom to move camera but it's not being executed or not working correctly
  - Impact: Test suite fails, blocks verification of camera zoom functionality
  - Root cause: Need to investigate test execution and handle_free_flight_zoom implementation

- [x] feature: Configuration validation at load time (BACKLOG.md line 22-33)
  - Current: No validation of genesis.toml values when loaded via Config::load()
  - Issue: Invalid config values can cause runtime issues or undefined behavior
  - Impact: User can set invalid particle counts, time accelerations, etc.
  - [x] Add Config::validate() method that checks all config values are within valid ranges
  - [x] Call validate() in Config::load() and log warnings/errors for invalid values
  - [x] Define validation rules:
    - particle.initial_count: clamp to [1000, 10000000]
    - particle.base_size: clamp to [0.1, 10.0]
    - time.time_acceleration_max: clamp to [1.0, 1e12]
    - window.width/height: clamp to [640, 7680]
  - [x] Add unit tests for Config::validate() covering edge cases

---

## Sprint 1 - Phase 1: The Singularity (Finalization)

### Phase 1 Deliverables Status

All Phase 1 PRD deliverables have been completed and verified. See COMPLETED.md [2026-02-10] for details.

### Remaining Work

- [ ] Implement configuration validation (see Critical Issues above)

---

### Documentation

---

- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.
