# Gap Analysis Report - 2026-02-10

## Summary
This report identifies gaps between the PRD Phase 1 requirements and the current implementation, comparing against TODO.md and BACKLOG.md to ensure all requirements are tracked.

---

## PRD Phase 1 Deliverables vs. Implementation Status

### 1. Bevy Application Scaffold ✓ COMPLETE
**PRD Requirement:** Bevy application scaffold with window, input handling, and basic 3D scene

**Status:** ✅ Implemented
- Window system with configurable resolution
- Input handling (keyboard, mouse) via InputPlugin
- Basic 3D scene with camera

**Location:** `src/main.rs`, `genesis-render/src/input/mod.rs`

---

### 2. Instanced Particle Renderer ✓ COMPLETE
**PRD Requirement:** Instanced particle renderer capable of displaying 100K–1M point sprites with position, color, and size

**Status:** ✅ Implemented
- Custom PointSpriteMaterial with WGSL shader
- GPU instancing via shared PointMesh resource
- Per-instance size and color attributes via storage buffer
- Configurable particle count (initial_count, max_count)

**Location:** `genesis-render/src/particle/mod.rs`, `genesis-render/src/particle/instance_buffer.rs`

**Configuration Gap:** 
- genesis.toml has `initial_count = 1000`
- Code default is `100_000`
- PRD specifies "100K–1M point sprites" capability
- **Action:** Clarify intended default value

---

### 3. Camera Controls ⚠️ PARTIAL
**PRD Requirement:** Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation

**Status:** ⚠️ Partially Implemented
- ✅ Free-flight camera (WASD + mouse): Implemented
- ✅ Orbit camera rotation (click-drag): Implemented
- ❌ Orbit camera zoom (scroll): NOT implemented
- ❌ Orbit camera pan (middle/right mouse): NOT implemented
- ❌ Smooth interpolation for camera mode switching: NOT implemented (deferred to Phase 7)

**Location:** `genesis-render/src/camera/mod.rs`

**Missing Features:**
1. Orbit camera zoom system (handle_orbit_zoom)
2. Orbit camera pan system (handle_orbit_pan)
3. Smooth interpolation for FreeFlight ↔ Orbit transitions

**BACKLOG Coverage:** ✅ Orbit zoom and pan tasks exist in BACKLOG.md (lines 25-35)
❌ Camera interpolation tasks exist but are marked as "deferred to Phase 7"

---

### 4. Cosmic Time System ✓ COMPLETE
**PRD Requirement:** f64 time accumulator with adjustable acceleration (1x to 10¹²x), pause, and reset

**Status:** ✅ Implemented
- TimeAccumulator resource with f64 precision
- Adjustable acceleration (1.0 to 1e12)
- Pause/resume methods
- Reset capability
- Speed-to-acceleration logarithmic mapping implemented

**Location:** `genesis-core/src/time/mod.rs`, `genesis-ui/src/timeline/mod.rs`

---

### 5. Timeline Scrubber ✓ COMPLETE
**PRD Requirement:** Logarithmic timeline scrubber UI spanning 13.8 billion years

**Status:** ✅ Implemented
- CosmicTime resource with max_time = 13.8e9
- Logarithmic slider mapping (from_slider/to_slider)
- Timeline UI panel with slider, play/pause, speed control

**Location:** `genesis-ui/src/timeline/mod.rs`

**PRD Alignment Note:**
- PRD timeline range: 10⁻³² years to 13.8 billion years
- Current implementation: 0.0 to 13.8e9 years
- Logarithmic mapping handles this range correctly

---

### 6. Singularity Visualization ❌ INCOMPLETE
**PRD Requirement:** Procedural singularity visualization: particles spawned at origin with outward velocity, color-mapped by energy

**Status:** ❌ Incomplete

**Implemented:**
- ✅ Particles spawned at origin
- ✅ Energy-based color mapping (white-hot → red)
- ✅ Basic outward expansion animation

**Critical Missing Feature:**
- ❌ **Outward velocity NOT stored in Particle component**

**Code Issue:**
In `spawn_particles()` (genesis-render/src/particle/mod.rs:302-304):
```rust
let velocity = direction * BASE_SPEED;
let _velocity_magnitude = velocity.length();  // Velocity calculated but NOT stored!
```

The Particle struct (lines 132-139) has NO `velocity` field:
```rust
pub struct Particle {
    pub position: Vec3,  // Position field exists
    pub color: Color,      // Color field exists
    pub size: f32,        // Size field exists
    // Missing: velocity field!
}
```

**Additional Issue:**
- ❌ **Particle.position not synced with Transform.translation**
  - `update_particle_energy_colors()` uses `particle.position.length()` to calculate energy
  - But `particle.position` is NEVER updated after spawning
  - Energy colors are calculated from original position (Vec3::ZERO), not actual position
  - This causes all particles to have white-hot color regardless of distance traveled

**Missing Implementation:**
1. Add `velocity: Vec3` field to Particle struct
2. Store velocity in spawn_particles()
3. Update Particle.position from Transform.translation each frame
4. Use stored velocity in update_particles() instead of hardcoded speed

**BACKLOG Coverage:** ❌ NOT captured in TODO.md or BACKLOG.md

---

### 7. FPS and Particle Count Overlay ✓ COMPLETE
**PRD Requirement:** FPS counter and particle count overlay

**Status:** ✅ Implemented
- FPS counter overlay system
- Particle count overlay system
- Configurable visibility via DisplayConfig

**Location:** `genesis-ui/src/overlay/mod.rs`

---

## Summary of Missing Requirements

### Critical (Blockers for Phase 1 Demo Moment)
1. **Particle velocity storage and usage** - Blocks proper particle expansion simulation
2. **Particle.position synchronization with Transform** - Breaks energy-based coloring system

### High Priority (Missing from TODO/BACKLOG)
3. **Orbit camera zoom controls** - In BACKLOG (Sprint 1)
4. **Orbit camera pan controls** - In BACKLOG (Sprint 1)

### Medium Priority (Clarification Needed)
5. **genesis.toml initial_count alignment** - Config has 1000, default is 100_000, PRD specifies 100K-1M capability

### Deferred to Future Phases (Documented)
6. **Camera smooth interpolation** - Deferred to Phase 7 (documented in ARCHITECTURE.md)

---

## Recommendations

### For Current Sprint (Sprint 1)
1. **Add particle velocity field to Particle component** (genesis-render/src/particle/mod.rs)
2. **Store velocity in spawn_particles()** (genesis-render/src/particle/mod.rs:302-304)
3. **Add particle position sync system** - Update Particle.position from Transform.translation each frame
4. **Fix update_particles() to use stored velocity** instead of hardcoded speed

### Clarification Requests
5. **genesis.toml initial_count** - Should default be 1000 or 100_000 to match PRD Phase 1 specs?

### For Future Sprints
- Orbit camera zoom and pan are already in BACKLOG.md (Sprint 1 section)
- Camera interpolation is correctly deferred to Phase 7

---

## Files Modified
This document is for informational purposes only. No code changes made.
