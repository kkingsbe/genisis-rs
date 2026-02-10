# Project State Report - 2026-02-09

## Project Phase
**IMPLEMENTATION** - Phase 1 (The Singularity) is in progress with failing integration tests and multiple uncompleted features. Sprint 1 has not been completed (no `.sprint_complete` file exists).

---

## TODO.md - Unchecked Items

### Failing Integration Tests (2026-02-09)
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_point_mesh_initialized_before_particles_spawn - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_materials_initialized_before_rendering - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_camera_initialized_before_rendering - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_extract_system_transfers_data - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_pipeline_cache_no_index_out_of_bounds - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resources_accessible_during_update - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_particle_instance_bind_group_layout - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resources_created_at_startup - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_system_ordering_point_mesh_before_spawn - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_complete_particle_rendering_setup - AssetServer resource does not exist in World
- [ ] fix: Failing test in genesis-render/tests/resource_binding_tests.rs - test_resource_reference_counting - Unable to find GPU (GPU required for rendering tests)

### Code Cleanup - Phase 1 Completeness Items

#### Remove Phase-Inappropriate Features
- [ ] refactor: Remove unrequested TimeConfig fields from genesis-core/src/config.rs
  - Remove initial_time, initial_time_acceleration (not used in Phase 1)

#### Camera System Cleanup (Defer Non-Phase 1 Features)
- [ ] refactor: Document camera mode switching as Phase 1 feature
  - Keep basic camera mode switching interpolation (FreeFlight ↔ Orbit) - this is PRD Phase 1 requirement
  - Document that advanced cinematic interpolation is Phase 7 feature
  - Ensure current CameraState.interpolation infrastructure serves only mode switching
- [ ] refactor: Verify orbit camera controls align with Phase 1 PRD
  - Per previous communication (architect-ambiguity-phase1-feature-scope), keep zoom and pan
  - These enhance UX for Phase 1 demo moment
  - Not explicitly prohibited in PRD
- [ ] refactor: Remove test functions from camera module
  - Remove `test_interpolation()` development testing function (triggered by 'T' key)
  - This is not specified in PRD

#### Particle Scaling Implementation
- [ ] feature: Scale particle system to 10K-50K particles (configurable) (Sprint 1)
  - Implement configurable particle_count field in genesis.toml
  - Scale particle system to support 10K-50K particles
  - Add basic performance monitoring
  - Validate performance target at 50K particles (≥60 FPS)

#### Timeline Reverse/Replay
- [ ] feature: Implement basic timeline scrubbing to TimeAccumulator.years synchronization (Sprint 1)
  - Enable particles to move backward/forward when scrubbing the timeline
  - Basic synchronization with TimeAccumulator.years during timeline scrub
  - Note: Full snapshot-based reverse/replay system is Sprint 2 priority

### Drift Tracking (Code-PRD Gap Consolidated)

#### Items Deferred to Future Phases
- [ ] fix: Implement Epoch Plugin Architecture per PRD section 4.1 (Phase 2+ feature)
  - Convert SingularityEpoch from marker struct to actual Bevy plugin
  - Create EpochManager for epoch transitions
  - PRD section 4.1 specifies Epoch Plugin Architecture for Phase 2+
  - Sprint 1 uses single epoch (Singularity) without transitions
- [ ] fix: Implement EpochPlugin trait and EpochManager system (Phase 2+ feature)
  - Support epoch transitions and timeline scrubbing across phases
  - PRD section 4.1 specifies this for Phase 2+

#### Configuration Alignment
- [ ] fix: Align genesis.toml default particle count with PRD Phase 1 target
  - Change initial_count from 1000 to 100000 (100K minimum per PRD)
  - PRD Phase 1 deliverables specify "100K–1M point sprites" capability
- [ ] fix: Align genesis.toml default time acceleration with code default
  - Change initial_time_acceleration from 1e9 to 1.0 (matches TimeConfig::default())
  - Remove initial_time_acceleration field entirely if not required per PRD

#### Items for Investigation (Non-Blocking)
- [ ] refactor: Simplify particle rendering architecture
  - Review per-instance GPU storage buffer architecture
  - Phase 1 requires basic instanced rendering with position/color/size attributes
  - Determine if current implementation can be simplified without breaking PRD requirements
- [ ] refactor: Remove unused clap dependency
  - Review genesis-core/Cargo.toml for unused dependencies
  - PRD doesn't specify command-line argument parsing
- [ ] fix: Remove "Epoch: Not implemented" placeholder
  - Remove from genesis-ui/src/overlay/mod.rs
  - Unnecessary visual clutter for Phase 1
- [ ] refactor: Remove debug print statements from genesis-render/src/particle/mod.rs
  - Remove println! statements at lines 161-162 and 272-278
  - Debug output not required per PRD Phase 1 deliverables
- [ ] refactor: Remove debug print statements from genesis-render/src/camera/mod.rs
  - Remove info! statements at lines 269 and 274
  - Debug output not required per PRD Phase 1 deliverables

### Repository Cleanup

#### Code Review Candidates
- [ ] review: genesis-core/src/physics/Particle struct is not used anywhere in codebase - consider if needed for future physics implementation or remove

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

### Drift Tracking (PRD vs Code Alignment)

#### Camera System Drift
- [ ] fix: Add smooth interpolation to camera mode switching - PRD Phase 1 specifies "smooth interpolation" for FreeFlight ↔ Orbit camera transitions, but toggle_camera_mode() uses instant switching (genesis-render/src/camera/mod.rs:251-277)

#### Particle System Drift
- [ ] fix: Add outward velocity to particle spawning - PRD Phase 1 specifies "particles spawned at origin with outward velocity", but spawn_particles() only spawns at origin without velocity (genesis-render/src/particle/mod.rs:299-300)
- [ ] fix: Implement timeline scrubbing with particle position reversal - PRD Demo Moment specifies "Scrub the timeline back and forth — the expansion reverses and replays", but timeline scrubbing only updates time display without reversing particle positions (genesis-ui/src/timeline/mod.rs:184-188)
- [ ] fix: Update Particle.position in update_particle_energy_colors - System uses particle.position to calculate energy but Particle.position is never updated with Transform.translation, creating synchronization issues (genesis-render/src/particle/mod.rs:377-390)

### New Drift Items (Code-PRD Gap - 2026-02-09)

#### Unrequested Features (Phase-Inappropriate Code)
- [ ] refactor: Remove GPU storage buffer infrastructure (genesis-render/src/particle/instance_buffer.rs) - PRD Phase 1 only requires "basic instanced rendering with position/color/size attributes", storage buffer with ExtractSchedule/Render world is advanced implementation not required for Phase 1
- [ ] refactor: Remove initial_time_acceleration field from TimeConfig - PRD Phase 1 only specifies "adjustable acceleration (1x to 10¹²x)", initial_time_acceleration is unrequested and code default is 1.0 (genesis-core/src/config.rs:27)
- [ ] refactor: Remove debug print statements from particle module - println! at lines 161-162 and 272-278 in genesis-render/src/particle/mod.rs are development artifacts not required per PRD Phase 1 deliverables
- [ ] refactor: Remove debug print statements from camera module - info! at lines 269 and 274 in genesis-render/src/camera/mod.rs are development artifacts not required per PRD Phase 1 deliverables
- [ ] refactor: Remove unused velocity calculation in spawn_particles - Variable 'velocity' is calculated but never stored on Particle component, adding unnecessary complexity (genesis-render/src/particle/mod.rs:309-310)

---

## PRD.md Summary

### Project Purpose
Genesis is a real-time Big Bang & Cosmological Evolution Simulator built in Rust that simulates the birth and evolution of the universe from the initial singularity through cosmic inflation, nucleosynthesis, recombination, and large-scale structure formation. It combines N-body gravitational dynamics, thermodynamic modeling, and particle physics approximations to produce a visually compelling, physically grounded real-time experience.

### Key Features/Phases

**Phase 1: The Singularity** (Current Sprint)
- Goal: Running Bevy application with 3D particle system, camera controls, and time slider
- Deliverables: 100K–1M point sprites, free-flight and orbit cameras with smooth interpolation, cosmic time system (1x to 10¹²x acceleration), logarithmic timeline scrubber, singularity visualization (particles with outward velocity, energy-based color mapping)

**Phase 2: Inflation & Quantum Seeds**
- Friedmann equation integrator for scale factor a(t)
- Exponential expansion during inflation with density perturbations
- 3D Gaussian random field generator with power spectrum
- Quark-gluon plasma visualization

**Phase 3: Nucleosynthesis**
- 12-species nuclear reaction network with ODE solver
- Live composition chart overlay showing element abundances
- Particle color transitions by dominant composition

**Phase 4: Recombination & CMB**
- Saha equation solver for ionization fraction
- Volumetric fog renderer (opaque to transparent transition)
- CMB surface projection with temperature anisotropies

**Phase 5: Dark Ages & First Structures**
- N-body gravity (direct-sum and Barnes-Hut)
- Dark matter halo formation with Friends-of-Friends algorithm
- Cosmic web visualization
- HDF5 snapshot export

**Phase 6: Cosmic Dawn & Galaxy Formation**
- SPH for baryonic gas dynamics
- Star formation and Pop III stars
- Reionization visualization with ionization bubbles
- Galaxy billboard sprites and procedural audio

**Phase 7: Polish & Cinematic Mode**
- Performance optimization pass
- Pre-authored camera paths with narration
- Full cosmological parameter panel
- Cross-platform release builds

### Design Principles
- Always Runnable: Every phase produces a runnable application
- Vertical Slices: Each phase touches full stack (input, physics, rendering, UI)
- Progressive Enhancement: Algorithms improve in fidelity while maintaining visual output
- Demo-Ready at Every Merge: Each phase has defined "Demo Moment"

---

## ARCHITECTURE.md Summary

### Key Architectural Decisions

1. **Modular Crate Architecture**
   - `genesis-core`: Core simulation logic (epoch, physics, time)
   - `genesis-render`: Rendering systems using Bevy ECS (camera, particle components)
   - `genesis-ui`: UI state resources using Bevy ECS (timeline, overlay)

2. **Bevy ECS Pattern**
   - Components: `Particle` (rendering), `CameraController`, `OrbitController`
   - Resources: TimeAccumulator, CameraState, InputState, CosmicTime, PlaybackState, OverlayState
   - Systems: Particle spawning/update, camera control, input handling, time integration, UI updates
   - Plugins: TimeIntegrationPlugin, InputPlugin, CameraPlugin, ParticlePlugin, TimelinePlugin, GenesisUiPlugin

3. **Instanced Particle Rendering**
   - GPU instancing with custom PointSpriteMaterial using WGSL shaders
   - Capacity: 100K - 1M particles (planned)
   - Attributes: Position, Velocity, Color, Size
   - Point sprite rendering with custom shader: Implemented
   - Storage buffer synchronization infrastructure exists but shader integration pending

4. **Cosmic Time System**
   - Dual time system: TimeAccumulator.years (physics) and CosmicTime.cosmic_time (UI)
   - Synchronization system links playback controls with accumulator
   - Logarithmic slider mapping for 13.8 billion year timeline
   - Speed-to-acceleration mapping implemented

5. **Camera System**
   - Modes: FreeFlight and Orbit (default)
   - Dual-controller architecture (both components always present)
   - Orbit camera zoom and pan NOT implemented
   - Camera interpolation NOT implemented

6. **Configuration Management**
   - TOML format (genesis.toml)
   - Config structs: WindowConfig, ParticleConfig, CameraConfig, TimeConfig, DisplayConfig
   - Config::load() implemented with file path search logic

7. **Epoch Plugin Architecture (Planned for Future Phases)**
   - EpochPlugin trait, EpochManager, EpochManagerPlugin NOT implemented
   - SingularityEpoch marker struct exists but does not implement EpochPlugin

### Current Implementation Status

**Implemented (Phase 1):**
- Bevy 0.15+ application scaffold with window
- Basic input handling (keyboard, mouse)
- Time integration system with f64 accumulator
- Particle rendering system with custom point sprite shader
- Particle spawning and basic outward expansion
- Energy-based particle color system (thermal gradient)
- Camera system with free-flight and orbit modes
- Camera mode switching via 'O' key
- Overlay UI with FPS and particle count
- Timeline UI with play/pause, logarithmic slider, speed control
- Time synchronization between playback controls and accumulator

**Partially Implemented:**
- SingularityEpoch defined but does NOT implement EpochPlugin trait
- Per-instance particle attributes (storage buffer systems exist, shader integration pending)

**NOT Implemented (deferred to Phase 2+):**
- Orbit camera zoom and pan
- Camera smooth interpolation
- Epoch management system
- Physics-based particle simulation (beyond basic expansion)
- Timeline reverse/replay with snapshot history
- All Phase 2-7 features

### Two-Level Particle Architecture
- **Simulation-Level Particle** (genesis-core::physics::Particle): Rust arrays for physics calculations - defined but not used
- **Rendering-Level Particle** (genesis-render::particle::Particle): Bevy ECS component for rendering - fully implemented

---

## Inbox Status
No new items - comms/inbox/ directory is empty.
