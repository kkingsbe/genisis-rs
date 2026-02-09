# Architect Communication: Phase 1 Feature Scope Clarification

## Date
2026-02-09

## Context

During the gap analysis and sprint review, several questions emerged regarding the correct interpretation of Phase 1 PRD requirements versus the current implementation. The current TODO.md contains multiple refactoring tasks that seek to "remove" features that are implemented but may not align with PRD Phase 1 scope.

Selected Option:
**Keep basic interpolation**: Retain camera mode switching interpolation in Phase 1, remove only advanced cinematic interpolation

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

**Selected Option**:
**Keep zoom/pan**: Retain standard orbit camera features (zoom/pan) as they enhance Phase 1 UX

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

**Selected Option**:
**Add performance scaling test**: Implement gradual particle count increase with performance monitoring
