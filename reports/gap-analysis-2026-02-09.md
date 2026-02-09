# Gap Analysis - 2026-02-09

## Overview

This document provides a comprehensive gap analysis comparing the PRD requirements to the current implementation (TODO.md, src/).

---

## Critical Gaps Identified

### 1. DisplayConfig Missing from genesis-core/src/config.rs

**Severity:** HIGH - Blocker

**Issue:**
- The genesis.toml configuration file contains a `[display]` section with fields:
  - `show_fps`
  - `show_particle_count`
  - `show_epoch_info`
- src/main.rs line 77-81 references `config.display.show_fps`, `config.display.show_particle_count`, `config.display.show_epoch_info`
- genesis-ui/src/overlay/mod.rs defines OverlayState with the same three fields
- However, genesis-core/src/config.rs does NOT define DisplayConfig struct

**Impact:**
- The configuration system is incomplete - loading from TOML will fail
- The application cannot deserialize the display section
- Config::load() will error when reading genesis.toml

**Required Fix:**
Add DisplayConfig struct to genesis-core/src/config.rs:

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct DisplayConfig {
    pub show_fps: bool,
    pub show_particle_count: bool,
    pub show_epoch_info: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_fps: true,
            show_particle_count: true,
            show_epoch_info: true,
        }
    }
}
```

And update Config struct to include DisplayConfig field:

```rust
pub struct Config {
    pub time: TimeConfig,
    pub particle: ParticleConfig,
    pub camera: CameraConfig,
    pub window: WindowConfig,
    pub display: DisplayConfig,  // ADD THIS
}
```

Also update impl Default for Config to include DisplayConfig.

---

### 2. Speed-to-Acceleration Mapping IS Implemented

**Severity:** NONE - Item Should Be Removed

**Issue:**
TODO.md item 82 states:
> "fix: Complete speed-to-acceleration mapping in genesis-ui/src/timeline/mod.rs (sync_time_resources() only syncs play/pause, not acceleration)"

**Reality Check:**
genesis-ui/src/timeline/mod.rs lines 205-208 ALREADY implement this mapping:

```rust
// Map PlaybackState.speed (0.1-10.0) to TimeAccumulator.acceleration (1.0-1e12)
let speed = playback_state.speed as f64;
let acceleration = 10_f64.powf((speed.log10() + 1.0) * 6.0);
time_accumulator.set_acceleration(acceleration);
```

**Required Action:**
- Remove TODO.md item 82 ("fix: Complete speed-to-acceleration mapping...")
- This item is already completed

---

### 3. BACKLOG.md Sprint 1 Has Many Completed Items

**Severity:** MEDIUM - Cleanup Issue

**Issue:**
BACKLOG.md Sprint 1 section contains many items marked as completed (~~strikethrough~~), including:
- Line 11: ~~Implement procedural singularity visualization~~
- Line 12: ~~Replace random particle spawning~~
- Line 21: ~~Calculate particle energy based on distance from origin~~
- Line 22: ~~Implement energy-based color mapping~~
- Line 26: ~~Implement scroll wheel zoom for orbit camera~~
- Line 35: ~~Add pan controls for orbit camera~~
- Line 36: ~~Implement smooth camera interpolation system~~
- Line 45: ~~Create CameraTween resource~~ (REPLACED BY)
- Line 46: ~~Implement camera tween update system~~
- Line 60: ~~Build FPS counter overlay~~
- Line 61: ~~Create particle count overlay~~
- Line 62: ~~Build time control UI~~
- Line 63: ~~Implement logarithmic timeline scrubber~~
- Line 64: ~~Update main.rs to initialize PlaybackState~~
- Line 75: ~~Create genesis-config module~~
- Line 76: ~~Add serde dependencies~~
- Line 77: ~~Implement TOML deserialization~~
- Line 78: ~~Create default Config constants~~
- Line 79: ~~Implement config file loader~~
- Line 80: ~~Implement clap argument parser~~
- Line 81: ~~Add ConfigResource and insert~~
- Line 89: ~~Implement epoch plugin registration system~~
- Line 90: ~~Define EpochPlugin trait~~
- Line 91: ~~Create SingularityEpoch plugin~~
- Line 92: ~~Implement EpochManager resource~~
- Line 99: ~~Register epoch plugins in main.rs~~
- Line 112: ~~Implement pause() method~~
- Line 113: ~~Implement smooth camera interpolation~~

**Impact:**
- BACKLOG.md is cluttered with completed work
- Makes it difficult to see what actually remains
- Violates principle that BACKLOG should contain future work only

**Required Action:**
- Remove all ~~strikethrough~~ completed items from BACKLOG.md Sprint 1 section
- Move truly pending Sprint 1 items to TODO.md if they're Phase 1 scope
- Keep Sprint 1 items in BACKLOG that are truly for future work (not Phase 1)

---

## Phase Scope Assessment

### Sprint 1 (Phase 1: The Singularity) - Current Status

**Completed Infrastructure:**
- [x] Bevy application scaffold with window and event loop
- [x] Bevy 0.15+ application scaffold
- [x] Basic input handling (keyboard, mouse)
- [x] Time integration system with f64 accumulator
- [x] Particle rendering system with custom point sprite shader
- [x] Particle spawning system
- [x] Basic outward expansion animation
- [x] Energy-based particle color system
- [x] Camera system with free-flight and orbit modes
- [x] Camera mode switching
- [x] Orbit camera zoom via scroll wheel
- [x] Camera interpolation support
- [x] Overlay UI with FPS, particle count, epoch info panels
- [x] Timeline UI with play/pause, logarithmic slider, speed control
- [x] Time synchronization between PlaybackState and TimeAccumulator
- [x] Speed-to-acceleration mapping (sync_time_resources)
- [x] Configuration system (partial - DisplayConfig missing)

**Pending Sprint 1 Work (from TODO.md):**

**Core Features:**
- [ ] Time controls: play/pause, reset, speed adjustment (1x to 10¹²x)
  - Status: Infrastructure exists, needs testing/verification
- [ ] Logarithmic timeline scrubber UI
  - Status: Implemented in CosmicTime::from_slider() and to_slider()
- [ ] Map timeline scrubbing to cosmic time simulation state
  - Status: Partially implemented - needs snapshot system for reverse/replay
- [ ] Epoch indicator display (current era, temperature, scale factor)
  - Status: OverlayState.show_epoch_info exists, needs Temperature/ScaleFactor resources

**Singularity Visualization:**
- [ ] Procedural singularity particle generation
  - Status: spawn_particles() spawns at origin with outward velocity
- [ ] Energy-based color mapping (white-hot → yellow → red cooling)
  - Status: energy_to_color() implemented
- [ ] Particle velocity expansion simulation
  - Status: update_particles() implements basic outward expansion
- [ ] Timeline scrubbing with reverse/replay capability
  - Status: Infrastructure exists, needs snapshot system

**UI Overlay:**
- [ ] FPS counter overlay - Status: Implemented
- [ ] Particle count display - Status: Implemented
- [ ] Epoch info panel (time, temperature, scale factor) - Status: Partial, missing Temperature/ScaleFactor resources
- [ ] Time control UI (play/pause, speed slider, reset button) - Status: Implemented
- [ ] Timeline scrubber widget - Status: Implemented

**Configuration & Initialization:**
- [ ] TOML deserialization for Config struct - Status: Implemented
- [ ] Default Config constants for "Standard Model" preset - Status: Implemented
- [ ] Config file loader with path resolution - Status: Implemented
- [ ] Clap argument parser for --config flag - Status: Implemented
- [ ] ConfigResource and add to main.rs - Status: Implemented
- [ ] **DisplayConfig struct missing** - CRITICAL GAP

---

## PRD Requirements vs. Reality

### PRD Phase 1 Deliverables:

1. **Bevy application scaffold with window, input handling, and basic 3D scene**
   - ✅ IMPLEMENTED

2. **Instanced particle renderer capable of displaying 100K–1M point sprites with position, color, and size**
   - ✅ IMPLEMENTED (infrastructure exists, per-instance sync pending)

3. **Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation**
   - ✅ IMPLEMENTED

4. **Cosmic time system: f64 time accumulator with adjustable acceleration (1x to 10¹²x), pause, and reset**
   - ✅ IMPLEMENTED

5. **Logarithmic timeline scrubber UI (bevy_egui) spanning 13.8 billion years**
   - ✅ IMPLEMENTED (CosmicTime::from_slider() and to_slider() use log scale)

6. **Procedural "singularity" visualization: particles spawned at origin with outward velocity, color-mapped by energy**
   - ✅ IMPLEMENTED

7. **FPS counter and particle count overlay**
   - ✅ IMPLEMENTED

### Gaps for PRD Phase 1:

1. **DisplayConfig missing** - Configuration incomplete
2. **Per-instance particle color/size sync** - Particle component data not transferred to GPU attributes
3. **Temperature and ScaleFactor resources** - Needed for epoch indicator display
4. **Timeline reverse/replay** - Needs snapshot system
5. **Particle scaling to 100K-1M** - Currently at 10K in config

---

## Recommendations

### Immediate Actions (Sprint 1):

1. **CRITICAL:** Add DisplayConfig to genesis-core/src/config.rs
   - Add DisplayConfig struct with show_fps, show_particle_count, show_epoch_info fields
   - Add DisplayConfig to Config struct
   - Update Config::default() to initialize DisplayConfig

2. **Cleanup:** Remove completed items from BACKLOG.md Sprint 1 section
   - Remove all ~~strikethrough~~ items
   - Keep only truly pending items

3. **Documentation:** Update TODO.md to reflect current state
   - Mark completed items as [x]
   - Remove already-completed items like speed-to-acceleration mapping
   - Clarify what "Map timeline scrubbing to cosmic time simulation state" requires (snapshot system)

### Sprint 1 Completion Criteria:

Before Sprint 1 can be marked complete (via .sprint_complete), the following must be done:

1. DisplayConfig added and configuration loads successfully from genesis.toml
2. Application builds without errors
3. Demo Moment works: "The Primordial Spark"
4. FPS counter displays
5. Particle count displays
6. Timeline slider works
7. Play/pause/reset buttons work
8. Speed control works and acceleration mapping verified
9. Camera modes switch correctly
10. Particles expand outward with color gradient

---

## Conclusion

The project is in good shape overall. The main gap is the missing DisplayConfig struct which is a critical blocker for configuration loading. Most Sprint 1 features are implemented, and the TODO.md's drift remediation section correctly identifies phase-inappropriate features that need refactoring.

The speed-to-acceleration mapping item in TODO.md is incorrect and should be removed as it's already implemented.

Next steps should be:
1. Add DisplayConfig to config.rs
2. Clean up BACKLOG.md by removing completed items
3. Update TODO.md to reflect current state
4. Complete remaining Sprint 1 items
5. Run Sprint QA and create .sprint_complete marker
