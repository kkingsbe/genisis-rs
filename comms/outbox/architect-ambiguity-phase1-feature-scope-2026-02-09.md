# Architect Communication: Phase 1 Feature Scope Clarification

## Date
2026-02-09

## Context

During the gap analysis and sprint review, several questions emerged regarding the correct interpretation of Phase 1 PRD requirements versus the current implementation. The current TODO.md contains multiple refactoring tasks that seek to "remove" features that are implemented but may not align with PRD Phase 1 scope.

## Questions for Resolution

### Question 1: Camera Smooth Interpolation Scope

**Issue**: There is an apparent contradiction regarding camera interpolation features.

**PRD Phase 1 (Line 112)**: States "free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation"

**TODO.md (Line 10)**: Lists "refactor: Remove camera interpolation - it's a Phase 7 feature per PRD but implemented in Phase 1"

**Analysis**:
- Phase 1 explicitly mentions "with smooth interpolation" for camera controls
- Phase 7 focuses on "Cinematic mode: pre-authored camera paths with keyframes and easing curves"
- Basic camera interpolation (smooth mode transitions) seems to be Phase 1 scope
- Advanced cinematic interpolation (pre-authored paths) is clearly Phase 7 scope

**Options**:
1. **Keep basic interpolation**: Retain camera mode switching interpolation in Phase 1, remove only advanced cinematic interpolation
2. **Remove all interpolation**: Remove all interpolation code and defer to Phase 7
3. **Clarify PRD intent**: Update PRD to explicitly state which interpolation features belong to which phase

**Recommendation**: Option 1 - Keep basic interpolation for mode switching (FreeFlight ↔ Orbit transitions) but remove or defer complex cinematic interpolation features.

---

### Question 2: Orbit Camera Zoom/Pan Features

**Issue**: TODO.md lists removing orbit camera zoom/pan features as "not specified in Phase 1 PRD".

**PRD Phase 1 (Line 112)**: States "orbit camera (click-drag)" - only explicitly mentions click-drag rotation

**TODO.md (Lines 38-41)**: Lists removing:
- `handle_orbit_zoom()` system (scroll wheel zoom)
- `handle_orbit_pan()` system (middle/right mouse button pan)
- Related `OrbitController` fields: `min_distance`, `max_distance`, `rotation_sensitivity`, `zoom_sensitivity`, `pan_sensitivity`

**Analysis**:
- The PRD only mentions "click-drag" for orbit camera
- Zoom and pan are standard 3D camera features that enhance usability
- Removing these may significantly impact UX for Phase 1 demo moment
- No explicit prohibition of these features in PRD

**Options**:
1. **Keep zoom/pan**: Retain standard orbit camera features (zoom/pan) as they enhance Phase 1 UX
2. **Remove zoom/pan**: Remove all features not explicitly mentioned in PRD Phase 1
3. **Move to Phase 2**: Defer zoom/pan implementation until Phase 2 when more features are added

**Recommendation**: Option 1 - Keep zoom/pan features as they are standard UX for 3D viewers and enhance the Phase 1 "demo moment" experience.

---

### Question 3: Particle Count for Phase 1 Demo

**Issue**: Mismatch between genesis.toml configuration and actual particle spawning.

**PRD Phase 1 (Line 113)**: "Instanced particle renderer capable of displaying 100K–1M point sprites"

**genesis.toml (Line 13)**: `initial_count = 100000`

**Actual Implementation**: Currently spawns only 1000 test particles

**TODO.md (Lines 15-16)**: Conflicting tasks:
- Line 15: "fix: Implement particle spawning at configured count - genesis.toml has 100K but only 1000 spawning"
- Line 16: "fix: Update genesis.toml particle.initial_count to match Phase 1 testing (1000 instead of 100K)"

**Analysis**:
- The PRD specifies a target capacity of 100K-1M particles
- However, testing should start with lower counts (1K-10K) to establish performance baseline
- The TODO.md has conflicting instructions about which side to adjust

**Options**:
1. **Adjust config to spawn count**: Update genesis.toml to have `initial_count = 1000` for current testing
2. **Implement full spawning capability**: Update spawn code to respect the 100K config value
3. **Add performance scaling test**: Implement gradual particle count increase with performance monitoring

**Recommendation**: Option 1 - Update genesis.toml to `initial_count = 1000` for current Sprint 1 development, add task to "Scale particle system to 10K-50K" (which already exists in TODO.md line 58).

---

## Summary

These questions represent interpretive ambiguities in PRD Phase 1 scope. Resolving them will help the team focus on the right tasks and avoid unnecessary refactoring work.

**Priority**: Medium - These questions impact the direction of refactoring tasks but do not block core functionality.

**Impact**: Resolution will determine which refactoring tasks in TODO.md (Critical Fixes section) are actually necessary versus which should be removed or modified.

---

## Suggested Next Steps

Once you provide answers to these questions, the Architect will:
1. Update TODO.md to remove any tasks that are no longer necessary
2. Add or modify tasks that are needed based on clarified scope
3. Ensure BACKLOG.md contains appropriate Phase 2+ tasks for any deferred features

**Expected Response Format**: Please answer each question by selecting one of the provided options or providing your own alternative. You may answer all questions at once or provide partial answers for clarification.
