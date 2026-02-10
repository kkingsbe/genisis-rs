# TODO - Current Sprint (Sprint 2: Singularity Refinement)

**Sprint Goal:** Complete Phase 1 Singularity implementation with particle velocity, position synchronization, and configuration alignment.

---

## Sprint 2 - Phase 1: The Singularity (Refinement)

### Camera Controls (Phase 1 PRD Requirements)

- [x] feature: Implement scroll wheel zoom controls for orbit camera
  - [x] Add scroll wheel event handling to orbit camera system
  - [x] Implement zoom with distance clamping (min_distance=5.0, max_distance=200.0)
  - [x] Add handle_orbit_zoom() system in genesis-render/src/camera/mod.rs
- [x] feature: Implement pan controls for orbit camera (PRD Phase 1 requires complete orbit camera controls)
  - [x] Add middle mouse button drag detection to InputState
  - [x] Implement pan system that moves orbit target point based on mouse drag
  - [x] Add handle_orbit_pan() system in genesis-render/src/camera/mod.rs
- [ ] feature: Implement scroll wheel zoom controls for free-flight camera (PRD Phase 1 requirement)
  - [ ] Add scroll wheel event handling to free-flight camera system (genesis-render/src/camera/mod.rs update_free_flight_camera)
  - [ ] Implement zoom speed parameter in CameraController (zoom_speed: f32)
  - [ ] Apply scroll delta to move camera along forward vector (translation += forward * scroll_delta * zoom_speed)
  - [ ] Clamp zoom movement to prevent camera passing through origin or flying too far
- [ ] fix: Implement Q/E up/down movement for free-flight camera (documented but not implemented)
  - Location: genesis-render/src/camera/mod.rs handle_keyboard_input
  - Current: Q/E keys documented in comments but not implemented
  - PRD reference: genesis-render/src/camera/mod.rs:68-71

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

- [ ] refactor: Remove debug print statements from genesis-render/src/particle/mod.rs
  - [ ] Remove println! statements at lines 266-272
  - [ ] Remove println! statements at lines 318-320
  - Debug output not required per PRD Phase 1 deliverables
- [ ] refactor: Remove debug print statements from genesis-render/src/camera/mod.rs
  - [ ] Remove info! statements at lines 269 and 274
  - Debug output not required per PRD Phase 1 deliverables

### Documentation

- [ ] doc: Update ARCHITECTURE.md to reflect Particle component changes
  - [ ] Document new velocity field in Particle component
  - [ ] Document sync_particle_position() system
  - [ ] Update Phase 1 implementation status

---

## SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

### Test Health - Failing Tests

- [x] fix: Failing test in genesis-render/tests/shader_tests.rs from commit d0929b9
  - Error: `PointSpriteMaterial::vertex_shader()` function not found (lines 120, 600)
  - Error: `PointSpriteMaterial::fragment_shader()` function not found (lines 121, 601)
  - Error: `PointSpriteMaterial::alpha_mode()` method not found (line 577)
  - Error: `PointSpriteMaterial` does not implement `Material` trait (line 743)
  - Root cause: PointSpriteMaterial uses `AsBindGroup` instead of implementing the `Material` trait
- [x] fix: Failing test in genesis-render/tests/resource_binding_tests.rs from commit d0929b9
  - Error: `PointSpriteMaterial::vertex_shader()` function not found (lines 315, 1404)
  - Error: `PointSpriteMaterial::fragment_shader()` function not found (lines 327, 1405)
  - Error: `PointSpriteMaterial::alpha_mode()` method not found (line 352)
  - Root cause: PointSpriteMaterial uses `AsBindGroup` instead of implementing the `Material` trait
