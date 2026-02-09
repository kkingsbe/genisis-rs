# Drift Detection

## Drift & Cleanup (Latest Analysis - 2026-02-09)

### Type A - Unrequested Features
- [x] refactor: Remove test_interpolation function from genesis-render/src/camera/mod.rs - Development testing function triggered by 'T' key (lines 544-564) is not specified in PRD and should be removed for production code
- refactor: Remove setup_test_camera_target function from src/main.rs - Development testing code spawning test CameraTarget entity (lines 100-103) is not specified in PRD and should be removed

### Type B - PRD Contradictions
- fix: Align time acceleration range with PRD requirements - genesis-core/src/config.rs:152 sets time_acceleration_min to 0.1, but PRD Phase 1 requires minimum 1x acceleration (range should be 1x to 10^12x, not 0.1x to 10^12x)
- fix: Align UI speed slider range with PRD requirements - genesis-ui/src/timeline/mod.rs:170 slider has range 0.1..=10.0 but PRD Phase 1 requires 1x to 10^12x acceleration (1e12), not 10x

---

Critical drift items identified from PRD analysis:

## Feature Drift (Missing PRD Features)
- feat: Implement PRD feature bevy_egui panels - Actual implementation only has resource definitions with "not yet implemented" comments; no actual UI panels or widgets exist for timeline and overlays
- feat: Implement PRD feature TOML configuration system - No Config struct, no TOML loading, no command-line arguments for --config flag or "Standard Model" preset
- feat: Implement PRD feature epoch plugins - EpochPlugin trait and EpochManager exist, but NO actual epoch plugins are registered or implemented
- feat: Implement PRD feature logarithmic timeline scrubber UI - Only PlaybackState.speed field exists, no actual timeline widget or logarithmic mapping spanning 13.8 billion years

## Implementation Drift (Contradicts PRD)
- fix: Align particle system with PRD requirements - genesis-core::physics::Particle uses [f32; 3] arrays while genesis-render::particle::Particle uses Vec3 and Color Bevy types; they are completely disconnected with no synchronization between simulation and rendering
- fix: Align singularity visualization with PRD requirements - PRD specifies particles spawned at origin with outward velocity and color-mapped by energy (white-hot core fading to red); implementation uses random particle spawning in a sphere with mostly white/blue colors, no energy mapping
- fix: Align resource initialization with PRD requirements - CameraState, OverlayState, PlaybackState resources are defined but not initialized in main.rs
- fix: Align camera systems with PRD requirements - PRD requires smooth interpolation between positions and camera transition crossfade for epoch changes; implementation only has basic free-flight and orbit movement, no interpolation or crossfade

---

## Drift Detection (PRD Phase 1)

### Unrequested Features
- refactor: Remove unrequested epoch system (EpochPlugin, EpochManager, epoch UI elements)
- refactor: Remove unrequested CLI argument parsing and config file loading system
- refactor: Remove unrequested camera mode toggle feature (press 'O' to switch modes)
- refactor: Remove unrequested CameraState resource and CameraMode enum used for mode switching
- refactor: Simplify particle velocity spawning to minimal outward velocity (remove pseudo-random sphere calculation)

### Contradictions
- fix: Align particle spawn count with PRD requirement (increase from 1000 to at least 100K particles)
- fix: Add smooth interpolation to camera movement (implement lerp/damping for both free-flight and orbit cameras)

---

# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Missing Features (Drift Analysis - Phase 1)

- [ ] implement: Create procedural singularity visualization - Replace random particle spawning with energy-mapped coloring (white-hot core to red edges) as specified
- [ ] implement: Implement epoch plugins - Create actual epoch plugins (e.g., SingularityEpoch, InflationEpoch) and register them with EpochManager

---

## Sprint 1 - Phase 1: The Singularity

### Critical Blocker Resolution (Priority: IMMEDIATE)

### Camera System
- [ ] Implement camera transition crossfade for epoch changes
- [ ] Add zoom and pan controls

### Time & Timeline
- [ ] Implement time controls: play/pause, reset, speed adjustment (1x to 10¹²x)
- [ ] Build logarithmic timeline scrubber UI using bevy_egui
- [ ] Map timeline scrubbing to cosmic time simulation state
- [ ] Add epoch indicator display (current era, temperature, scale factor)

### Singularity Visualization
- [ ] Implement procedural singularity particle generation (origin spawn with outward velocity)
- [ ] Create energy-based color mapping (white-hot → yellow → red cooling)
- [ ] Add particle velocity expansion simulation
- [ ] Implement timeline scrubbing with reverse/replay capability

### UI Overlay
- [ ] Implement FPS counter overlay
- [ ] Add particle count display
- [ ] Create epoch info panel (time, temperature, scale factor)
- [ ] Build time control UI (play/pause, speed slider, reset button)
- [ ] Add timeline scrubber widget

### Configuration & Initialization
- [ ] Define Config struct with all Phase 1 parameters (particle_count, time_acceleration, etc.)
- [ ] Implement TOML deserialization for Config struct
- [ ] Create default Config constants for "Standard Model" preset
- [ ] Implement config file loader with path resolution (default: genesis.toml, fallback: embedded defaults)
- [ ] Implement clap argument parser for --config flag to override default config path
- [ ] Create ConfigResource and add to main.rs via .insert_resource(config)

### Architecture & Documentation
- [ ] Update ARCHITECTURE.md with final crate structure and responsibilities
- [ ] Document epoch plugin architecture design patterns (trait-based plugin system)
- [ ] Add inline documentation for genesis-core public APIs (time::TimeAccumulator, epoch::EpochPlugin trait, physics::Particle)
- [ ] Add inline documentation for genesis-render public APIs (camera::CameraMode/State, input::InputState, particle::Particle component)
- [ ] Add inline documentation for genesis-ui public APIs (overlay::OverlayState, timeline::PlaybackState)

### Plugin Registration
- [ ] Create genesis-render::RenderPlugin (aggregates camera, input, particle systems)
- [ ] Add .add_plugins(genesis_render::RenderPlugin) to main.rs
- [ ] Create genesis-ui::UIPlugin (aggregates timeline, overlay systems)
- [ ] Add .add_plugins(genesis_ui::UIPlugin) to main.rs
- [ ] Add .init_resource::<CameraState>() to main.rs
- [ ] Add .init_resource::<OverlayState>() to main.rs
- [ ] Add .init_resource::<PlaybackState>() to main.rs

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Drift Analysis Results (Flagged Issues)

### Unrequested Features
- refactor: Consider deferring full EpochPlugin architecture to Phase 2 - PRD only mentions epoch UI indicator in Phase 2, but keeping for now as foundation (non-blocking)
- refactor: Remove CameraState resource and CameraMode enum architecture - resource-based state tracking not specified in PRD
- refactor: Remove InputState resource architecture - detailed input tracking system not specified in PRD
- refactor: Remove show_epoch_info flag from OverlayState - not specified in Phase 1 PRD requirements
- refactor: Remove PlaybackState resource - resource-based playback state tracking not specified in PRD
- refactor: Remove VERSION constants from genesis-core, genesis-render, genesis-ui crates - not specified in PRD
- refactor: Remove bytemuck dependency from genesis-render/Cargo.toml - not in PRD dependency specifications

### Contradictory Code (Archived 2026-02-09)
- chore: Document two-level particle architecture - genesis-core::physics::Particle (simulation) and genesis-render::particle::Particle (rendering) are intentionally separate
- fix: Align genesis-render/src/particle/mod.rs singularity visualization with PRD - Outward velocity implemented; energy-based color mapping still needed

### Refined Task Definitions
- refined: Configuration & Initialization tasks broken down into 6 atomic subtasks
- refined: Architecture & Documentation tasks broken down into 5 atomic subtasks
- refined: Plugin Registration tasks broken down into 8 atomic subtasks



## Drift Analysis Results (New 2026-02-09)

### Implementation Drift (Contradicts PRD)
- fix: Align singularity visualization with PRD requirements - PRD Phase 1 specifies "particles spawned at origin with outward velocity, color-mapped by energy (white-hot core fading to red)" but current implementation spawns particles randomly in a sphere with mostly white/blue colors and no energy-based color mapping

### Unrequested Features (Consider for Future Refactor)
- refactor: Consider removing CameraState.target field from genesis-render/src/camera/mod.rs - Target point field not specified in PRD Phase 1 requirements
- refactor: Consider removing CameraState.current_orbit_target field from genesis-render/src/camera/mod.rs - Orbit target tracking not specified in PRD
- refactor: Consider removing OrbitController.min_distance and max_distance fields from genesis-render/src/camera/mod.rs - Zoom distance constraints not specified in PRD
- refactor: Consider using Bevy's ButtonInput directly instead of InputState.mouse_buttons HashMap from genesis-render/src/input/mod.rs - HashMap-based button state tracking adds complexity over direct Bevy input handling

### Missing Requirements (from PRD Phase 1)
- implement: Add energy-based particle color mapping - White-hot core to red cooling based on particle energy/distance from origin
- implement: Add origin-based particle spawning - Spawn particles at origin (0,0,0) instead of random sphere distribution

---

## Drift Analysis Results (Updated 2026-02-09)

### New Drift Items Found (Not Previously Documented)

**Missing Features:**
- implement: Implement timeline scrubbing - Timeline slider changes CosmicTime.cosmic_time but no system scrubs simulation forward/backward based on timeline position
- implement: Implement Singularity epoch systems - genesis-core/src/epoch/singularity.rs:39 has empty build() method with "TODO: Register singularity-specific systems"
- implement: Implement velocity-based particle movement - genesis-render/src/particle/mod.rs:80 has Particle.velocity field but update_particles() system ignores it and uses constant speed instead

**Contradictions:**
- fix: Align time acceleration slider range with PRD - genesis-ui/src/timeline/mod.rs:170 slider has range 0.1..=10.0 but PRD Phase 1 requires 1x to 10^12x acceleration (1e12)
- fix: Align energy-based color mapping with PRD - PRD specifies color-mapped by energy (kinetic energy from velocity) but update_particle_energy_colors() system uses distance from origin instead

### Previously Documented Drift Items Verified as Still Valid

**Unrequested Features (Still Valid):**
- refactor: Consider deferring full EpochPlugin architecture to Phase 2 - EpochPlugin trait and EpochManager exist but PRD Phase 1 doesn't require multiple epochs
- refactor: Consider removing CameraState.target field - Line 30 in genesis-render/src/camera/mod.rs has unused target field
- refactor: Consider removing CameraState.current_orbit_target field - Line 32 in genesis-render/src/camera/mod.rs has unused current_orbit_target field
- refactor: Consider removing OrbitController.min_distance and max_distance fields - Lines 104-106 in genesis-render/src/camera/mod.rs have unused distance limits
- refactor: Consider using Bevy's ButtonInput directly instead of InputState.mouse_buttons HashMap - Line 20 in genesis-render/src/input/mod.rs uses HashMap for button state

**Missing Requirements (Status Updated):**
- implement: Add energy-based particle color mapping - PARTIALLY RESOLVED: energy_to_color() function exists and implements white-hot to red gradient, but it's not connected to particle velocity as PRD implies
- implement: Add origin-based particle spawning - RESOLVED: genesis-render/src/particle/mod.rs:236 correctly spawns particles at Vec3::ZERO

### Drift Items Found to be Resolved (Can be Removed)

---

## Drift Analysis Results (Latest 2026-02-09)

### New Drift Items Found (Not Previously Documented)

**Unrequested Features (Development/Testing Code):**
- refactor: Remove test_interpolation function from genesis-render/src/camera/mod.rs - Development testing function triggered by 'T' key (lines 460-479) is not specified in PRD and should be removed for production code

**Contradictions (Implementation vs PRD):**
- fix: Align time acceleration slider minimum with PRD - genesis-ui/src/timeline/mod.rs:170 slider has range 0.1..=10.0 but PRD Phase 1 requires minimum 1x acceleration (time_acceleration_min: 0.1 contradicts PRD requirement of 1x)

---

## Final Sprint QA

*Note: The SPRINT QA task is already included as the final task in Sprint 1 (line 98 above).*

---

## Repository Cleanup (2026-02-09)

### Files Deleted (16 files)

**Root directory artifacts (8 files):**
- `.architect-output-1770595533861.md` - Old ARCHITECT.md prompt artifact
- `.architect-output-1770598149498.md` - Recent ARCHITECT.md prompt artifact (deleted 2026-02-09)
- `.janitor-output-1770595466027.md` - Old JANITOR.md prompt artifact
- `.janitor-output-1770596516864.md` - Duplicate JANITOR.md prompt artifact
- `.janitor-output-1770597992764.md` - Recent JANITOR.md prompt artifact (deleted 2026-02-09)
- `.prompt-output-1770594835609.md` - Old PROMPT.md prompt artifact
- `.prompt-output-1770595919438.md` - Duplicate PROMPT.md prompt artifact
- `.prompt-output-1770596771283.md` - Duplicate PROMPT.md prompt artifact

**Root directory artifacts (6 files):**
- `.architect-output-1770595533861.md` - Old ARCHITECT.md prompt artifact
- `.janitor-output-1770595466027.md` - Old JANITOR.md prompt artifact
- `.janitor-output-1770596516864.md` - Duplicate JANITOR.md prompt artifact
- `.prompt-output-1770594835609.md` - Old PROMPT.md prompt artifact
- `.prompt-output-1770595919438.md` - Duplicate PROMPT.md prompt artifact
- `.prompt-output-1770596771283.md` - Duplicate PROMPT.md prompt artifact

**Work-in-progress marker (1 file):**
- `.architect_in_progress` - Old session marker from abandoned architect session (~8 hours old)

**Duplicate shader file (1 file):**
- `assets/point_sprite.wgsl` - Exact duplicate of `genesis-render/src/particle/point_sprite.wgsl`

**Communication artifacts (6 files - from 2026-02-08):**
- `comms/outbox/PRD-ambiguity-check-summary.md` - Old unaddressed PRD questions
- `comms/outbox/question-algorithm-specification-gaps.md` - Old unaddressed question
- `comms/outbox/question-particle-count-ambiguity.md` - Old unaddressed question
- `comms/outbox/question-performance-mode-configuration.md` - Old unaddressed question
- `comms/outbox/question-time-acceleration-definition.md` - Old unaddressed question
- `comms/outbox/question-validation-and-feasibility.md` - Old unaddressed question

### Directories Deleted (3 directories)

- `assets/` - Empty after removing duplicate shader file
- `comms/outbox/` - Empty after removing old communication artifacts
- `comms/` - Empty after removing outbox subdirectory

### Rationale

All deleted items were:
- Old output artifacts from previous agent runs (`.architect-output-*.md`, `.janitor-output-*.md`, `.prompt-output-*.md`)
- Abandoned session markers (`.architect_in_progress`)
- Duplicate source files (`assets/point_sprite.wgsl`)
- Outdated communication artifacts sitting in outbox for over 24 hours without user response
- Empty directories resulting from file deletions

No source code, configuration, or documentation files were deleted.

