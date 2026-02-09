# GENESIS Architecture

## Overview
GENESIS is a real-time Big Bang and Cosmological Evolution Simulator built in Rust using Bevy 0.15+ engine. The project simulates the universe's evolution from singularity through 13.8 billion years across 7 phases.

## Technology Stack
- **Language**: Rust 2021 edition
- **Engine**: Bevy 0.15+ (ECS-based game engine)
- **GPU Compute**: wgpu (via Bevy)
- **UI Framework**: bevy_egui
- **Configuration**: TOML

## Workspace Structure

### Crates (Cargo Workspace)
```
genesis/
├── Cargo.toml (workspace manifest)
├── genesis-core/      # Core simulation logic and physics
│   └── src/
│       ├── time/         # Cosmic time accumulator and time integration
│       │   └── mod.rs   # TimeAccumulator resource, TimeIntegrationPlugin, time conversion functions
│       ├── physics/      # Particle physics data structures
│       │   └── mod.rs   # Particle struct (data structure only, physics calculations not implemented)
│       ├── epoch/        # Epoch markers
│       │   ├── mod.rs        # Exports: SingularityEpoch marker struct
│       │   └── singularity.rs    # Singularity epoch marker struct (does NOT implement EpochPlugin trait)
│       ├── config.rs     # Configuration structures (WindowConfig, ParticleConfig, CameraConfig, etc.)
│       └── lib.rs
├── genesis-render/   # Rendering systems and visuals
│   └── src/
│       ├── particle/     # Instanced particle rendering
│       │   ├── mod.rs           # PointSpriteMaterial, PointMesh, Particle component, spawn/update systems
│       │   └── point_sprite.wgsl  # WGSL shader for point sprite rendering
│       ├── camera/       # Camera mode definitions and state
│       │   └── mod.rs   # CameraMode enum, CameraState, CameraController, OrbitController, camera control systems
│       ├── input/        # Keyboard and mouse input handling
│       │   └── mod.rs   # InputState resource, handle_keyboard_input, handle_mouse_input
│       └── lib.rs
└── genesis-ui/        # User interface components
    └── src/
        ├── timeline/     # Timeline scrubber and time controls
        │   └── mod.rs   # CosmicTime resource, PlaybackState resource, timeline_panel_ui system
        ├── overlay/      # FPS and particle count panels
        │   └── mod.rs   # OverlayState resource, update_overlay_ui system
        └── lib.rs
```

### Application Structure
```
src/
└── main.rs              # Application entry point
```

The application registers the following plugins and resources:
- **DefaultPlugins**: Bevy's default set of plugins
- **TimeIntegrationPlugin** (genesis-core): Cosmic time accumulation with f64 precision
- **InputPlugin** (genesis-render): Keyboard and mouse input handling with InputState resource
- **ParticlePlugin** (genesis-render): Particle system initialization and spawning (with PointSpriteMaterial and PointMesh resources)
- **CameraPlugin** (genesis-render): Camera control systems (free-flight, orbit, interpolation, orbit pan/zoom) with CameraState resource
- **GenesisUiPlugin** (genesis-ui): UI system with bevy_egui integration, overlay, and timeline controls (includes TimelinePlugin internally)
- **ConfigResource** (main.rs): Wrapper for Config as a Bevy Resource (NOTE: Config::load() IS implemented - reads from genesis.toml with file path search logic)
- **ParticleConfig** (genesis-core): Resource for particle spawning configuration (correctly used directly with Resource derive in main.rs line 48)
- **CameraState** (genesis-render): Resource for tracking camera mode, target, and interpolation state (initialized from CameraConfig)
- **OverlayState** (genesis-ui): Resource for overlay visibility (initialized from DisplayConfig: show_fps, show_particle_count, show_epoch_info)
  - Note: OverlayState.show_epoch_info field EXISTS in genesis-ui/src/overlay/mod.rs line 17
- **CosmicTime** (genesis-ui): Resource for timeline state management with logarithmic slider mapping (auto-initialized by TimelinePlugin)
- **PlaybackState** (genesis-ui): Resource for playback control (auto-initialized by TimelinePlugin)
- **TimeAccumulator** (genesis-core): Resource for tracking cosmic years (auto-initialized by TimeIntegrationPlugin)
- **setup_camera**: Camera setup system that spawns 3D camera at orbit_distance looking at origin with OrbitController and CameraController components (correctly uses config.camera.orbit_distance which matches CameraConfig struct)

**Epoch Infrastructure Status**: The following epoch-related types are defined:
- CameraMode enum (FreeFlight, Orbit) - defined in genesis-render/src/camera/mod.rs for camera mode transitions
- SingularityEpoch marker struct - epoch marker for Singularity phase (defined in genesis-core/epoch/singularity.rs)

**The following epoch management infrastructure is NOT implemented and is planned for future phases**:
- EpochCameraConfig struct - NOT defined
- EpochManager resource - NOT defined
- EpochManagerPlugin - NOT defined
- EpochPlugin trait - NOT defined
- update_epoch_transition system - NOT defined
- EpochChangeEvent - NOT defined
- SingularityEpoch does NOT implement EpochPlugin trait (trait doesn't exist)

## Core Architectural Decisions

### 1. Modular Crate Architecture
- **Rationale**: Separates concerns into core, render, and UI domains
- **Benefit**: Clear dependency boundaries, easier testing, parallel development
- **genesis-core**: Pure simulation logic (epoch, physics, time), depends on Bevy for Resource trait
  - Exports: Config, ParticleConfig, CameraConfig, TimeConfig, WindowConfig, DisplayConfig, TimeIntegrationPlugin, SingularityEpoch
  - **NOTE**: CameraMode is exported from genesis-render, not genesis-core. EpochCameraConfig, EpochManager, EpochManagerPlugin, EpochPlugin, and EpochChangeEvent are NOT exported (not defined in codebase)
- **genesis-render**: Rendering systems using Bevy ECS (camera, particle components)
  - Exports: CameraMode, CameraPlugin, CameraState, InputPlugin, ParticlePlugin
- **genesis-ui**: UI state resources using Bevy ECS (timeline, overlay)
  - Exports: GenesisUiPlugin, TimelinePlugin, CosmicTime, PlaybackState, OverlayState

### 2. Bevy ECS Pattern
- **Components**: `Particle` (rendering component with position: Vec3, color: Color, size: f32)
  - Particles are spawned with Mesh3d, MeshMaterial3d<PointSpriteMaterial>, Transform, and Particle components
  - Camera components: `CameraController` (free-flight), `OrbitController` (orbit)
- **Resources**: Global state organized by crate:
   - genesis-core: TimeAccumulator
   - genesis-render: CameraState, InputState, PointMesh
   - genesis-ui: CosmicTime, OverlayState, PlaybackState
- **Systems**:
   - Core: (none - epoch infrastructure does NOT exist)
   - Particle: init_point_mesh (Startup), spawn_particles (Startup), update_particles (basic outward expansion animation), update_particle_energy_colors (thermal gradient coloring), extract_particle_instances (ExtractSchedule), prepare_particle_instance_buffers (Render)
   - Camera: update_free_flight_camera (Update), update_orbit_camera (Update), toggle_camera_mode (Update), handle_orbit_zoom (Update), handle_orbit_pan (Update)
   - Input: handle_keyboard_input (PreUpdate), handle_mouse_input (PreUpdate)
   - Time: initialize_time_accumulator (Startup), update_cosmic_time (Update)
   - UI: update_overlay_ui (Update), timeline_panel_ui (PostUpdate), sync_time_resources (Update)
- **Plugins**:
   - TimeIntegrationPlugin (implemented): Cosmic time accumulation with Bevy integration
   - InputPlugin (implemented): Keyboard and mouse input processing
   - CameraPlugin (implemented): Camera control systems for free-flight, orbit, interpolation, orbit pan, and orbit zoom
   - ParticlePlugin (implemented): Particle spawning and rendering systems with custom point sprite shader
   - TimelinePlugin (implemented within GenesisUiPlugin): Timeline UI with play/pause, logarithmic slider, and speed control
   - GenesisUiPlugin (implemented): UI system with EguiPlugin integration, overlay, and timeline controls
   - **NOTE**: EpochManagerPlugin does NOT exist (not defined in codebase)

### 3. Instanced Particle Rendering
- **Design**: GPU instancing with custom PointSpriteMaterial using WGSL shaders
- **Capacity**: 100K - 1M particles (planned)
- **Attributes**: Position ([f32; 3]), Velocity ([f32; 3]), Color ([f32; 3]), Size (f32)
- **Status**:
  - Point sprite rendering with custom shader: **Implemented**
  - Particle spawning system (spawn_particles): **Implemented** (100,000 default particles)
  - Shared mesh resource initialization: **Implemented**
  - Physics-based particle updates: **Implemented (basic outward expansion animation)**
  - Per-instance particle size and color attributes: **Infrastructure implemented (ATTRIBUTE_INSTANCE_SIZE, ATTRIBUTE_INSTANCE_COLOR in mesh)**
    - Storage buffer synchronization system exists (`instance_buffer.rs` with extract_particle_instances and prepare_particle_instance_buffers)
    - These systems are registered in ParticlePlugin (ExtractSchedule and Render phases)
    - Current shader uses mesh attributes @location(1) and @location(2) instead of storage buffer
    - Full synchronization from Particle component → GPU attributes is partially complete (extract system runs, shader uses mesh attributes)
  - Energy-based particle color mapping for thermal gradient: **Implemented** (white-hot core → red edges)

### 3.1 Two-Level Particle Architecture

The project uses two distinct particle types at different layers:

**Simulation-Level Particle (genesis-core::physics::Particle)**
- Located in: `genesis-core/src/physics/mod.rs`
- Data structure: Rust arrays (not Bevy types)
  - `position: [f32; 3]` - Position in 3D space
  - `velocity: [f32; 3]` - Velocity vector
  - `color: [f32; 3]` - RGB color
  - `size: f32` - Size in world units
- Purpose: Core physics calculations and cosmological simulation state
- Status: Data structure defined; physics calculations not yet implemented

**Rendering-Level Particle (genesis-render::particle::Particle)**
- Located in: `genesis-render/src/particle/mod.rs`
- Data structure: Bevy ECS component with Bevy types
  - `position: Vec3` - World space position
  - `color: Color` - RGBA color (Bevy type)
  - `size: f32` - Particle size in world units
- Purpose: Rendering component attached to particle entities
- Status: Fully implemented with spawning and update systems

**Data Flow**
Currently, the rendering-level Particle is directly populated in [`spawn_particles()`](genesis-render/src/particle/mod.rs:245) with test data. The planned architecture will:
1. Run physics calculations on simulation-level particles
2. Convert simulation state to render components
3. Update entity Transforms and Particle components with new positions

### 3.2 Point Sprite Rendering Resources

**PointSpriteMaterial**
- Custom Bevy material implementing `Material` trait
- Uses a custom WGSL shader (`point_sprite.wgsl`) for vertex and fragment processing
- Uniform parameters:
  - `color: LinearRgba` - Base color for all particles
  - `base_size: f32` - Base size in pixels before distance attenuation
  - `attenuation_factor: f32` - Controls size attenuation with distance
- Uses additive blending (AlphaMode::Add) for a glowing effect
- Registered via `MaterialPlugin::<PointSpriteMaterial>::default()` in ParticlePlugin

**PointMesh**
- Resource containing a shared `Handle<Mesh>` for all particle entities
- Mesh topology: `PrimitiveTopology::PointList` with a single vertex at origin
- The `Transform` component on each entity provides actual position
- Custom vertex attributes:
  - `ATTRIBUTE_INSTANCE_SIZE`: Float32 at location(1) for per-instance particle size
  - `ATTRIBUTE_INSTANCE_COLOR`: Float32x4 at location(2) for per-instance particle color
- Initialized once at startup via `init_point_mesh` system
- Shared across all particles for efficient GPU instancing

### 4. Cosmic Time System
- **Type**: f64 accumulator for precision over 13.8B years
- **Dual Time System**: The application uses two separate time resources:
  - **TimeAccumulator.years** (genesis-core): Tracks accumulated cosmic time in years, updated each frame via add_time() with acceleration factor
  - **CosmicTime.cosmic_time** (genesis-ui): Stores timeline position used by the slider UI, updated by timeline scrubbing
  - **Synchronization**: The sync_time_resources system synchronizes:
    - TimeAccumulator's paused state with PlaybackState.playing
    - PlaybackState.speed to TimeAccumulator.acceleration (logarithmic mapping)
  - **Timeline Scrubbing**: Timeline scrubbing updates CosmicTime.cosmic_time and syncs to TimeAccumulator.years via timeline_panel_ui system (line 180: `time_accumulator.years = cosmic_time.cosmic_time;`).
- **Acceleration**:
  - TimeAccumulator.acceleration handles the actual 1x-10¹²x scaling
  - TimeAccumulator provides pause() and resume() methods for playback control
  - PlaybackState.speed (f32, 0.1-10.0) controls time acceleration via speed control slider (logarithmic scale)
- **UI Integration**:
  - CosmicTime resource provides logarithmic slider mapping via from_slider() and to_slider() methods
  - Timeline UI panel (timeline_panel_ui) renders play/pause button, timeline slider, and speed control (runs in PostUpdate schedule)
  - sync_time_resources system synchronizes TimeAccumulator with PlaybackState (runs in Update schedule)
- **Status**:
  - TimeAccumulator resource fully implemented with pause/resume/toggle/is_paused methods
  - TimeIntegrationPlugin integrates with Bevy's time system
  - Timeline UI controls fully implemented with logarithmic slider mapping
  - Time synchronization between UI playback controls and accumulator fully implemented
  - Speed-to-acceleration mapping: **Implemented** - PlaybackState.speed (0.1-10.0) maps to TimeAccumulator.acceleration (1.0-1e12) using logarithmic scaling
  - Timeline scrubbing to TimeAccumulator synchronization: **Implemented** - slider changes update both CosmicTime resource and TimeAccumulator.years via timeline_panel_ui

### 5. Camera System Design
- **Camera Modes**: FreeFlight and Orbit enum variants defined (Orbit is default)
- **State Tracking**: CameraState resource with mode, target, and current_orbit_target fields
- **Components**:
  - CameraController: Free-flight camera with yaw, pitch, movement_speed, mouse_sensitivity
  - OrbitController: Orbit camera with distance, yaw, pitch, target, zoom limits, rotation sensitivity, zoom sensitivity, pan sensitivity
- **Dual-Controller Architecture**: Both OrbitController and CameraController components are always present on the camera entity. Mode switching (via toggle_camera_mode) affects which controller responds to input, not component attachment.
- **Configuration**:
  - CameraConfig in genesis-core has fields: initial_position, initial_target, camera_mode (String), movement_speed, orbit_distance
  - CameraMode enum exists in genesis-render::camera but CameraConfig uses String for camera_mode field
  - genesis.toml has fields: initial_mode (String), orbit_distance (f64)
- **Configuration Field Mismatches**:
  - **camera_mode vs CameraMode enum**: CameraConfig.camera_mode is a String, and CameraMode enum exists. CameraState::from_config() in genesis-render/src/camera/mod.rs (line 60) correctly accesses `config.camera_mode` and converts it to CameraMode enum.
  - **initial_time_acceleration vs default_time_acceleration**: genesis.toml has `initial_time_acceleration` field, and TimeConfig struct also has `initial_time_acceleration` field (field names match)
- **Status**:
  - Camera setup (setup_camera system): Implemented - spawns 3D camera at orbit_distance looking at origin with OrbitController (distance: orbit_distance) and CameraController::default().
  - Camera movement controls: Implemented for both free-flight (update_free_flight_camera) and orbit (update_orbit_camera) modes
  - Camera mode switching: Implemented via toggle_camera_mode system (press 'O' key to toggle between FreeFlight and Orbit)
  - Orbit camera zoom: Implemented via handle_orbit_zoom system (scroll wheel controls zoom distance, clamped between min_distance and max_distance)
  - Orbit camera pan: Implemented via handle_orbit_pan system (middle or right mouse button to pan orbit target)
  - Camera interpolation: **NOT implemented** (interpolate_camera system does NOT exist in camera/mod.rs or CameraPlugin)

### 6. Input System Architecture
- **InputState Resource**: Tracks keyboard direction vector, mouse delta, and mouse button states
- **Keyboard Handling**: WASD key inputs mapped to directional vectors
- **Mouse Handling**: Mouse button states tracked using HashMap<MouseButton, bool>; Tracks mouse motion delta
- **Status**: InputPlugin fully implemented with handle_keyboard_input and handle_mouse_input systems (run in PreUpdate schedule)

### 7. Epoch Plugin Architecture (Planned for Future Phases)

**Current Implementation**:
Only the following epoch-related types are defined in genesis-core/epoch/:
- CameraMode enum (FreeFlight, Orbit) - defined for camera mode transitions
- EpochCameraConfig struct - configuration for camera transitions between epochs
- SingularityEpoch marker struct - epoch marker for Singularity phase

**Epoch Management Infrastructure - NOT YET IMPLEMENTED**:
The following infrastructure is planned for future phases and does NOT exist in the current codebase:
- EpochPlugin trait - NOT defined anywhere (planned)
- EpochManager resource - NOT defined anywhere (planned)
- EpochManagerPlugin - NOT defined anywhere (planned)
- EpochChangeEvent - NOT defined anywhere (planned)
- update_epoch_transition system - NOT defined anywhere (planned)
- handle_epoch_change_transition system - NOT defined anywhere (planned)

**Status**: Epoch management infrastructure is NOT implemented. The current implementation only includes basic epoch marker and camera configuration types that provide foundational infrastructure for future epoch transitions. The full epoch plugin architecture (EpochPlugin trait, EpochManager, and transition systems) is deferred to future phases.

### 8. Configuration Management
- **Format**: TOML for human-readable configuration
- **Status**: Configuration system implemented with file loading support
  - Config struct with WindowConfig, ParticleConfig, CameraConfig, TimeConfig, DisplayConfig fully defined
  - Default implementations provided for all config structs
  - ConfigResource wrapper for Bevy integration
  - Config::load() method implemented - reads from genesis.toml with file path search logic
  - Searches ./genesis.toml, ~/.config/genesis/config.toml, /etc/genesis/config.toml in order
- **Configuration Field Mismatches**:
  - **ParticleConfig**: Field names match correctly between genesis.toml and ParticleConfig struct (initial_count, max_count, base_size)
  - **CameraConfig**: genesis.toml has `initial_mode`, `orbit_distance` but CameraConfig struct has `initial_position`, `initial_target`, `camera_mode` (String), `movement_speed`, `orbit_distance`
  - **DisplayConfig**: genesis.toml has `show_epoch_info` and OverlayState struct also has this field
  - **TimeConfig**: genesis.toml has `initial_time_acceleration` but TimeConfig struct has `initial_time_acceleration`
- **Note**: Configuration loading infrastructure is implemented. Some field name mismatches remain (CameraConfig.initial_position, initial_target fields are not in genesis.toml)

## Phase 1 Scope (Current Implementation)

### Goal
A running Bevy application with a 3D particle system, camera controls, and a time slider.

### Current Implementation Status

**NOTE**: Only Phase 1 deliverables are currently implemented. Features from Phase 2-7 (epoch management, physics-based particle simulation, nucleosynthesis, recombination, N-body gravity, SPH, star formation, cinematic mode, etc.) are NOT implemented and are planned for future phases.

### Phase 1 Implementation Status
**Implemented:**
- Core infrastructure setup (workspace, Cargo.toml)
- Bevy 0.15+ application scaffold with window and event loop
- Basic input handling (keyboard, mouse) - InputPlugin with InputState, handle_keyboard_input, handle_mouse_input (runs in PreUpdate)
- Time integration system with f64 accumulator - TimeIntegrationPlugin, TimeAccumulator, update_cosmic_time system, pause/resume methods
- Particle rendering system with custom point sprite shader (PointSpriteMaterial, PointMesh) with GPU instancing
- Particle spawning system (spawn_particles) that creates test cluster at origin with configurable count
- Particle update system (update_particles) with basic outward expansion animation
- Energy-based particle color system (update_particle_energy_colors) with thermal gradient (white-hot core → red edges)
- Camera system with free-flight and orbit modes - CameraPlugin, CameraController, OrbitController, update_free_flight_camera, update_orbit_camera
- Camera mode switching via toggle_camera_mode (press 'O' key)
- Orbit camera zoom via scroll wheel (handle_orbit_zoom) with distance clamping
- Orbit camera pan via middle/right mouse button (handle_orbit_pan)
- Overlay UI with FPS and particle count panels - update_overlay_ui system
- Timeline UI with play/pause, logarithmic slider, and speed control - TimelinePlugin, CosmicTime resource with logarithmic mapping, timeline_panel_ui system (runs in PostUpdate)
- Time synchronization (sync_time_resources) between PlaybackState and TimeAccumulator including speed-to-acceleration mapping

**Partially Implemented (Infrastructure exists but not connected):**
- Epoch camera configuration (EpochCameraConfig struct defined, CameraMode enum defined) - infrastructure exists but not used
- SingularityEpoch - defined as marker struct but does NOT implement EpochPlugin trait (trait doesn't exist)
- Per-instance particle attributes (ATTRIBUTE_INSTANCE_SIZE, ATTRIBUTE_INSTANCE_COLOR) - mesh attributes defined but not synchronized with Particle component data
- Dual time system (TimeAccumulator.years + CosmicTime.cosmic_time) - both exist but timeline scrubbing doesn't sync back to TimeAccumulator.years

**Pending (Phase 1 Completion Items):**
- Config::load() method implemented for external TOML configuration (reads from genesis.toml with file path search logic)
- Full physics-based particle updates with simulation-level particle sync (update_particles has basic outward expansion, full physics sync pending)
- Per-instance particle color and size synchronization with GPU instance attributes (Particle component → GPU attributes)
- OverlayState.show_epoch_info field addition to struct and UI rendering
- Timeline scrubbing to TimeAccumulator synchronization with state restoration for reverse/replay (slider changes update both CosmicTime and TimeAccumulator.years)
- ParticleConfigResource definition in genesis-core (main.rs references it but ParticleConfig has Resource derive - should use ParticleConfig directly)
- Configuration field name reconciliation between genesis.toml and Config structs:
  - ParticleConfig: reconcile genesis.toml fields (initial_count, max_count, base_size) with struct fields (particle_count, particle_size_base, particle_size_variation, color_hot, color_cool)
  - CameraConfig: reconcile genesis.toml fields (initial_mode, orbit_distance) with struct fields (camera_mode, orbit_radius) and CameraMode enum usage
  - OverlayState.show_epoch_info field exists in struct and UI rendering (main.rs line 51 correctly sets this field)

**Deferred to Future Phases (Phase 2+):**
- Epoch management system implementation (EpochPlugin trait, EpochManager resource, EpochManagerPlugin, EpochChangeEvent, update_epoch_transition system, handle_epoch_change_transition system)
- SingularityEpoch implementation of EpochPlugin trait (trait doesn't exist - planned for Phase 2+)
- Friedmann equation integrator for scale factor a(t) (Phase 2)
- Density perturbations and Zel'dovich approximation (Phase 2)
- Nucleosynthesis reaction network solver (Phase 3)
- CMB surface projection and volumetric fog (Phase 4)
- N-body gravity simulation (Phase 5)
- SPH for baryonic gas dynamics (Phase 6)
- Cinematic mode with pre-authored camera paths (Phase 7)

## Dependency Graph

```
genesis-ui (bevy_egui)
    ↓
genesis-render (Bevy, wgpu)
    ↓
genesis-core (Bevy - for Resource trait)
```

**Resource Dependencies:**
- `TimeAccumulator` (genesis-core) is used by `sync_time_resources` (genesis-ui)
- `InputState` (genesis-render) is used by camera and input systems
- `CameraState` (genesis-render) is used by camera systems
- `CosmicTime` (genesis-ui) is used by timeline UI
- `PlaybackState` (genesis-ui) is synchronized with `TimeAccumulator` (genesis-core)
- `OverlayState` (genesis-ui) is used by overlay UI system (NOTE: missing show_epoch_info field)
- `ConfigResource` (main.rs) wraps Config for Bevy resource system
- `ParticleConfig` (genesis-core) can be used directly as Resource (has Resource derive), but main.rs references non-existent ParticleConfigResource
- `PointMesh` (genesis-render) is a shared resource for all particle entities
- **NOTE**: EpochManager does NOT exist (not defined in codebase)

## Development Guidelines

1. **Follow Bevy Conventions**: Use Bevy's ECS patterns for systems and resources
2. **Keep Crates Independent**: Avoid circular dependencies between crates
3. **Use Type Safety**: Leverage Rust's type system for physics calculations
4. **Document Public APIs**: Add inline documentation for all public functions
5. **Test Core Logic**: Unit tests for genesis-core physics and time calculations
6. **GPU First**: Offload particle rendering to GPU via instancing

## Performance Targets

- **Frame Rate**: 60 FPS minimum with 100K particles
- **Startup Time**: < 3 seconds to load and render
- **Memory Usage**: < 2GB with 1M particles
- **GPU Utilization**: Efficient batching with instanced rendering

## Future Considerations (Beyond Phase 1)

The following features are planned for future phases and are not currently implemented:

**Phase 2-7 Features (From PRD)**:
- WebAssembly support for browser deployment
- Save/load simulation state
- Multi-threaded physics calculations
- Compute shaders for advanced particle interactions
- Audio integration for cosmic events
- Epoch management system implementation (EpochPlugin trait, EpochManager resource, EpochManagerPlugin, update_epoch_transition system) - Planned for Phase 2+
- Friedmann equation integrator for scale factor a(t)
- Density perturbations and Zel'dovich approximation
- Nucleosynthesis reaction network solver
- CMB surface projection and volumetric fog
- N-body gravity simulation (direct-sum and Barnes-Hut)
- SPH for baryonic gas dynamics
- Star formation and reionization visualization
- Cinematic mode with pre-authored camera paths

**Phase 1 Completion Items (Pending)**:
- Per-instance particle attribute synchronization for individual particle colors and sizes
- Config::load() method implementation for external TOML configuration
- Timeline scrubbing to TimeAccumulator synchronization (reverse/replay with state restoration)
- OverlayState.show_epoch_info field addition and UI rendering
- Configuration field name reconciliation between genesis.toml and Config structs
- Particle scaling to 100K particles (currently at ~1000)

## Architectural Decisions Log

### [2026-02-09] Point Sprite Shader Asset Path Resolution

**Issue:** Application crashes on startup because Bevy cannot find `assets/point_sprite.wgsl`. The shader file exists at `genesis-render/src/particle/point_sprite.wgsl` but the standard `assets/` folder was deleted during a cleanup.

**Decision:** Recreate the `assets/` directory at the project root and copy `genesis-render/src/particle/point_sprite.wgsl` to `assets/point_sprite.wgsl`.

**Rationale:**
1. Follows Bevy's standard convention: assets are expected in an `assets/` folder at the project root
2. `PointSpriteMaterial::fragment_shader()` returns `"point_sprite.wgsl"` as a relative path, which Bevy resolves relative to `assets/`
3. This approach is simpler than embedding the shader directly in code or configuring custom asset paths
4. The shader file in `genesis-render/src/particle/` can be kept as the source of truth

**Implementation Required:**
- Create `assets/` directory if it doesn't exist
- Copy `genesis-render/src/particle/point_sprite.wgsl` to `assets/point_sprite.wgsl`
- Add `assets/` to `.gitignore` (optional - if assets are considered build artifacts)

**Impact:** Resolves the critical blocker preventing application startup. No code changes required.

### [2026-02-09] ViewUniform Shader Type Definition

**Issue:** Point sprite shader `point_sprite.wgsl` fails to compile because it references `ViewUniform` type (line 29) which is not defined. The shader uses `view.view_proj` to transform world positions to clip space, but `ViewUniform` struct is missing.

**Error Message:**
```
error: unknown type: 'ViewUniform'
   ┌─ point_sprite.wgsl:29:20
   │
29 │ var<uniform> view: ViewUniform;
   │                    ^^^^^^^^^^^ unknown type
```

**Root Cause:** The shader was written assuming Bevy's standard `ViewUniform` type would be available via import, but Bevy's Material trait does not automatically provide ViewUniform type definition in custom shaders. The type must be defined in the shader or imported from Bevy's shader library.

**Decision:** Define the `ViewUniform` struct directly in the shader file. This is the standard approach for Bevy custom materials that need view/projection data.

**Rationale:**
1. Bevy 0.15's Material trait provides automatic binding of the view uniform buffer at binding 1
2. The struct layout must match Bevy's internal ViewUniform definition for binding to work
3. Defining it inline is simpler than configuring custom imports
4. This pattern is used throughout Bevy's custom material examples

**ViewUniform Struct Definition:**
```wgsl
// Bevy's view uniform containing camera view-projection matrix
struct ViewUniform {
    view_proj: mat4x4<f32>,
    world_position: vec3<f32>,  // Camera world position (not currently used but provided by Bevy)
}
```

**Implementation Required:**
- Add `ViewUniform` struct definition to `genesis-render/src/particle/point_sprite.wgsl` before the `@group(0) @binding(1)` uniform binding
- The struct must be defined at the module level (not inside a function)
- Ensure the struct layout matches Bevy's internal ViewUniform: `view_proj: mat4x4<f32>` and `world_position: vec3<f32>`

**Impact:** Resolves the critical blocker that prevents the application from starting. This is a high-priority fix required before any particle rendering or visualization work can proceed. The fix is a single-line struct definition addition to the shader file.

---

## Gap Analysis

### Overview
This document provides a comprehensive gap analysis comparing the Product Requirements Document (PRD.md) against the current implementation status documented in TODO.md, BACKLOG.md, and the actual codebase. The analysis identifies missing requirements, implementation gaps, and provides recommendations for sprint planning.

### Methodology
The gap analysis was conducted by:
1. Reading and analyzing PRD.md to understand all requirements across all 7 phases
2. Reviewing TODO.md to understand the current sprint (Sprint 1 - Phase 1) focus areas
3. Reading BACKLOG.md to understand documented future work items
4. Examining actual implementation in src/, genesis-core/, genesis-render/, and genesis-ui/ crates
5. Cross-referencing requirements against both TODO and BACKLOG to identify gaps

### Phase 1 (The Singularity) - Gap Analysis

#### PRD Phase 1 Deliverables (Lines 104-123)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Bevy application scaffold with window, input handling, and basic 3D scene | ✅ Implemented | DefaultPlugins, WindowPlugin configured correctly |
| 2 | Instanced particle renderer capable of displaying 100K-1M point sprites with position, color, and size | ⚠️ Partially Implemented | Currently 1000 test particles. Infrastructure exists for GPU instancing, but scaling to 100K-1M not complete |
| 3 | Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation | ✅ Implemented | CameraController and OrbitController components work correctly. NOTE: Camera interpolation is Phase 7 feature per PRD but currently implemented |
| 4 | Cosmic time system: f64 time accumulator with adjustable acceleration (1x to 10¹²x), pause, and reset | ✅ Implemented | TimeAccumulator resource with all required methods |
| 5 | Logarithmic timeline scrubber UI (bevy_egui) spanning 13.8 billion years | ✅ Implemented | TimelinePanel with logarithmic mapping from_slider() and to_slider() |
| 6 | Procedural "singularity" visualization: particles spawned at origin with outward velocity, color-mapped by energy (white-hot core fading to red) | ✅ Implemented | spawn_particles() creates radial outward expansion with energy_to_color() mapping |
| 7 | FPS counter and particle count overlay | ✅ Implemented | update_overlay_ui() system displays FPS and particle count |

#### Phase 1 Gaps Identified

**Gaps Between PRD Requirements and TODO.md:**

1. **Particle Scaling to 100K-1M** (PRD line 113)
   - **Status:** Partially documented in TODO.md (item 20) and BACKLOG.md (lines 13-18)
   - **Gap:** Specific performance monitoring tasks missing:
     - No task to track FPS vs particle count scaling
     - No task for GPU memory management at high particle counts
     - No task for adaptive LOD system implementation details
   - **Impact:** Cannot verify 60 FPS target with 1M particles

2. **Per-Instance Particle Attribute Synchronization** (PRD requirement implicit in line 113)
   - **Status:** Infrastructure exists (ATTRIBUTE_INSTANCE_SIZE, ATTRIBUTE_INSTANCE_COLOR in mesh) but NOT synchronized
   - **Gap:** BACKLOG.md mentions per-instance data sync needs (lines 26-28), but specific implementation path missing:
     - No task for implementing custom instance buffer with dynamic updates
     - No task for Bevy instancing API exploration for per-instance attributes
     - No task for alternative custom render pipeline approach
   - **Impact:** Individual particle colors and sizes do not affect rendering despite being updated in Particle component

3. **Timeline Reverse/Replay Capability** (PRD line 121: "Scrub the timeline back and forth — the expansion reverses and replays")
   - **Status:** Partially documented in BACKLOG.md (lines 385-393)
   - **Gaps:** Missing critical implementation details:
     - No task for SimulationSnapshot resource structure implementation
     - No task for SnapshotHistory circular storage system
     - No task for state restoration from nearest snapshot logic
     - No task for edge case handling (scrubbing beyond snapshot history, unvisited time regions)
     - No task for TimelineScrubEvent event creation
   - **Impact:** Timeline scrubbing works forward but cannot replay backward smoothly

4. **Config::load() Implementation** (PRD line 113: "TOML configuration presets")
   - **Status:** NOT documented in TODO.md or BACKLOG.md
   - **Gap:** Configuration loading infrastructure is missing:
     - Config::load() method exists but needs external TOML file reading
     - No task to implement config file path resolution (./genesis.toml, user config, system config)
     - No task to implement CLI --config flag parsing with clap
     - No task for config validation and error handling
   - **Impact:** Cannot load external configuration files, only uses defaults

**Gaps Between PRD Requirements and BACKLOG.md:**

5. **Temperature Resource Module** (PRD Phase 2 requirement but needed for Phase 1 demo moment)
   - **Status:** Temperature module structure exists in BACKLOG.md (lines 413-461) but NOT implemented
   - **Gap:** Complete Temperature resource system missing:
     - No genesis-core/src/temperature.rs module creation task
     - No Temperature struct with value, min_temperature, max_temperature fields
     - No TemperaturePlugin for Bevy registration
     - No TemperatureEvolution trait implementation
     - No update_temperature() system for epoch-based temperature updates
     - No temperature history tracking for visualization
   - **Impact:** Cannot display temperature in epoch indicator or visualize thermal evolution

6. **Scale Factor Resource Module** (PRD Phase 2 requirement but needed for Phase 1 demo moment)
   - **Status:** Scale factor structure exists in BACKLOG.md (lines 462-515) but NOT implemented
   - **Gap:** Complete ScaleFactor resource system missing:
     - No genesis-core/src/scale_factor.rs module creation task
     - No ScaleFactor struct with value, hubble_parameter, epoch fields
     - No ScaleFactorPlugin for Bevy registration
     - No update_scale_factor() system for epoch-based scale factor updates
     - No scale factor history tracking for timeline visualization
     - No Friedmann equation solver integration
   - **Impact:** Cannot display scale factor in epoch indicator or visualize expansion

7. **Epoch Indicator UI** (PRD Phase 2 line 138 but mentioned in Phase 1 demo)
   - **Status:** Partially documented in BACKLOG.md (lines 403-412, 96-137)
   - **Gap:** Epoch indicator panel implementation incomplete:
     - No EpochIndicatorPanel struct creation task
     - No epoch_indicator_panel_ui() function implementation task
     - No task for formatting cosmic time display (format_cosmic_time())
     - No task for formatting temperature display (format_temperature()) with color mapping
     - No task for formatting scale factor display (format_scale_factor())
     - No task for epoch name display with color scheme (epoch_color_map())
     - No task for epoch description tooltip implementation
   - **Impact:** Cannot show current epoch information despite show_epoch_info flag existing

8. **Per-Instance Data Transfer System** (PRD Phase 1 requirement implicit)
   - **Status:** Partially documented in BACKLOG.md (lines 26-28 in Core Visualization, lines 518-522 in Particle State Synchronization)
   - **Gap:** Specific implementation path not defined:
     - No task for implementing system to copy Transform.translation to Particle.position
     - No task for implementing system to update Transform based on particle physics (velocity integration)
     - No task for ensuring particle physics update system writes to Transform, not just Particle component
   - **Impact:** Physics updates may not affect rendering correctly

9. **Easing Function Module** (Camera interpolation infrastructure)
   - **Status:** Well-documented in BACKLOG.md (lines 37-87)
   - **Gap:** Easing function creation tasks are detailed but missing:
     - No task for creating genesis-render/src/camera/easing.rs module file
     - No task for defining EasingFunction trait
     - No task for implementing all easing types (Linear, EaseInQuad, EaseOutQuad, EaseInOutCubic, EaseInCubic, EaseOutCubic, EaseInOutQuart, EaseOutQuart)
     - No task for implementing EasingType enum and From<EasingType> conversion
     - No task for implementing unit tests for each easing function
   - **Impact:** Camera interpolation uses smoothstep (hardcoded), cannot use other easing types

10. **Camera Interpolation on Epoch Change** (Phase 7 feature but currently implemented)
    - **Status:** Infrastructure exists (CameraState has interpolation fields)
    - **Gap:** No explicit task for triggering interpolation when epoch changes:
      - No task for creating camera tween trigger system that listens for EpochChangeEvent events
      - No task for extracting camera_config from target epoch
      - No task for calling CameraState::start_interpolation_to_target()
      - No task for registering this system in main.rs after epoch_manager plugin
    - **Impact:** Camera interpolation exists but not connected to epoch transitions

**Configuration Field Mismatches Identified:**

11. **CameraConfig.camera_mode vs CameraMode enum**
    - **Status:** CameraConfig uses String (genesis.toml field: "initial_mode") but CameraMode enum exists
    - **Gap:** No task to resolve this inconsistency:
      - No task for converting CameraConfig.camera_mode String to CameraMode enum consistently
      - No task for adding derive(Serialize) to CameraMode enum in genesis-core/epoch/camera_config.rs
      - No task for updating CameraConfig to use CameraMode instead of String
    - **Impact:** Requires string-to-enum conversion in CameraState::from_config()

12. **ParticleConfig field names**
    - **Status:** genesis.toml uses `initial_count`, `max_count`, `base_size` which match ParticleConfig struct
    - **Gap:** None - field names correctly match, Config::load() is implemented
    - **Impact:** None - configuration loading works correctly

13. **DisplayConfig.show_epoch_info vs OverlayState.show_epoch_info**
    - **Status:** Both DisplayConfig and OverlayState struct have `show_epoch_info` field
    - **Gap:** None - both structs have the field correctly
    - **Impact:** None - field exists and works correctly

### Phase 2 (Inflation & Quantum Seeds) - Gap Analysis

#### PRD Phase 2 Deliverables (Lines 126-145)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Friedmann equation integrator for scale factor a(t) with slow-roll inflaton potential V(φ) | ❌ Not in TODO or BACKLOG | Physics infrastructure missing |
| 2 | Particle positions scale with a(t) — exponential expansion during inflation, decelerating after | ❌ Not in TODO or BACKLOG | Particle-scale coupling not implemented |
| 3 | 3D Gaussian random field generator with nearly scale-invariant power spectrum P(k) ∝ k^(n_s – 1) | ❌ Not in TODO or BACKLOG | Density perturbation infrastructure missing |
| 4 | Density perturbations mapped to particle displacement (Zel'dovich approximation) and color intensity | ❌ Not in TODO or BACKLOG | Zel'dovich implementation missing |
| 5 | Epoch indicator in UI showing current cosmic era and key parameters (temperature, scale factor, time) | ⚠️ Partially documented | BACKLOG.md has structure but no implementation tasks |
| 6 | Parameter panel (bevy_egui sidebar): adjust n_s, inflation duration, initial energy scale; changes restart simulation | ❌ Not in TODO or BACKLOG | Parameter panel infrastructure missing |
| 7 | Procedural QGP visualization: particles rendered as glowing plasma blobs with temperature-mapped color ramp | ⚠️ Partially documented | Temperature color ramp documented but not implemented |

#### Phase 2 Gaps Identified

**Missing from TODO and BACKLOG:**

14. **Friedmann Equation Implementation**
    - **Status:** NOT documented in TODO.md or BACKLOG.md
    - **Gap:** Complete physics infrastructure missing:
      - No task for implementing FriedmannSolver struct with G, rho_matter, rho_radiation, rho_lambda fields
      - No task for implementing FriedmannSolver::compute_hubble_parameter() method
      - No task for implementing density evolution functions (rho_m(a), rho_r(a), rho_lambda(a))
      - No task for implementing FriedmannSolver::evolve_scale_factor() with RK4 integration
      - No task for implementing RK4 helper method rk4_step()
    - **Impact:** Cannot simulate metric expansion accurately

15. **Inflaton Field and Potential**
    - **Status:** NOT documented in TODO.md or BACKLOG.md
    - **Gap:** Inflaton physics missing:
      - No task for implementing InflationPhysics resource tracking inflaton field φ, potential V(φ), slow-roll parameters (ε, η)
      - No task for implementing slow-roll potential V(φ) = ½m²φ²
      - No task for implementing slow-roll parameter calculations (ε, η)
    - **Impact:** Cannot simulate inflation phase physics

16. **Gaussian Random Field Generation**
    - **Status:** NOT documented in TODO.md or BACKLOG.md
    - **Gap:** Density perturbation infrastructure missing:
      - No task for implementing Box-Muller transform for Gaussian random numbers
      - No task for implementing 3D Gaussian random field generator on regular grid
      - No task for implementing FFT to convert real-space to k-space
      - No task for implementing power spectrum generator P(k) ∝ k^(n_s – 1)
      - No task for implementing inverse FFT to convert k-space back to real-space
    - **Impact:** Cannot seed density perturbations

17. **Zel'dovich Approximation Implementation**
    - **Status:** NOT documented in TODO.md or BACKLOG.md
    - **Gap:** Displacement mapping missing:
      - No task for implementing density-to-displacement mapping (displacement = ∇ψ where ∇²ψ = -δ)
      - No task for implementing Poisson equation solver for displacement field
      - No task for mapping density perturbations to particle displacement
      - No task for mapping density perturbations to particle color intensity
    - **Impact:** Cannot visualize density variations from quantum seeds

18. **Parameter Panel UI**
    - **Status:** NOT documented in TODO.md or BACKLOG.md
    - **Gap:** Configuration UI missing:
      - No task for creating parameter panel layout in bevy_egui sidebar
      - No task for implementing n_s (spectral index) adjustment control
      - No task for implementing inflation duration adjustment control
      - No task for implementing initial energy scale adjustment control
      - No task for implementing simulation restart function
      - No task for connecting parameter panel controls to config update function
      - No task for updating Config struct to include Phase 2 parameters
      - No task for creating "Standard Model" preset
    - **Impact:** Cannot adjust cosmological parameters interactively

19. **QGP Visualization**
    - **Status:** Partially documented in BACKLOG.md (lines 619-626)
    - **Gap:** Visualization incomplete:
      - No task for implementing temperature-to-color ramp function with piecewise linear interpolation
      - No task for defining temperature color stops (1e15K → blue-white, etc.)
      - No task for adding unit tests for color transitions
    - **Impact:** Cannot visualize quark-gluon plasma phase accurately

### Phase 3 (Nucleosynthesis & First Elements) - Gap Analysis

#### PRD Phase 3 Deliverables (Lines 148-167)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Stiff ODE solver (implicit Rosenbrock method) for 12-species nuclear reaction network | ⚠️ Documented in BACKLOG.md | Lines 693-706 have structure but NOT implemented |
| 2 | Reaction rates from NACRE II compilation, temperature-dependent | ⚠️ Documented in BACKLOG.md | Lines 700-705 have structure but NOT implemented |
| 3 | Live composition pie/bar chart overlay showing element abundances | ⚠️ Documented in BACKLOG.md | Lines 726-728 have structure but NOT implemented |
| 4 | Particle color transitions: color-coded by dominant composition (H=blue, He=yellow, Li=pink) | ⚠️ Documented in BACKLOG.md | Line 728 mentioned but NOT implemented |
| 5 | Epoch transition: smooth visual crossfade from QGP plasma to element-colored particles | ⚠️ Documented in BACKLOG.md | Line 729 mentioned but NOT implemented |
| 6 | Validation overlay: comparison lines showing observed primordial abundances (Y_p ≈ 0.245 for ⁴He) | ⚠️ Documented in BACKLOG.md | Lines 735-783 have extensive structure but NOT implemented |
| 7 | TOML configuration presets: "Standard Model" (Planck 2018 best-fit) and "High Baryon Density" for comparison | ⚠️ Documented in BACKLOG.md | Lines 733-734 mentioned but NOT implemented |

#### Phase 3 Gaps Identified

**Gap:** Phase 3 items are well-documented in BACKLOG.md (Sprint 3 section, lines 690-785) with extensive subtasks, indicating good sprint planning.

**Notable Observation:** Phase 3 (Nucleosynthesis) appears to be the most thoroughly documented phase in BACKLOG.md, with:
- Detailed nuclear reaction network structure (12-species, ~50 reactions)
- Stiff ODE solver implementation with Rosenbrock method
- Comprehensive validation overlay design with abundance comparison charts
- Extensive UI component structure for validation panel

### Phase 4 (Recombination & CMB) - Gap Analysis

#### PRD Phase 4 Deliverables (Lines 170-188)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Saha equation solver tracking ionization fraction x_e as function of temperature | ⚠️ Documented in BACKLOG.md | Lines 536-546 have structure but NOT implemented |
| 2 | Volumetric fog renderer: space starts opaque, then clears as x_e drops | ⚠️ Documented in BACKLOG.md | Lines 555-567 have structure but NOT implemented |
| 3 | CMB surface projection: spherical shell at last-scattering surface showing temperature anisotropies | ⚠️ Documented in BACKLOG.md | Lines 558-568 have structure but NOT implemented |
| 4 | Smooth camera transition: as recombination completes, camera pulls back to reveal CMB sphere | ⚠️ Documented in BACKLOG.md | Lines 561-566 have structure but NOT implemented |
| 5 | Temperature readout drops through 3000 K (recombination) toward 2.725 K (present-day CMB) | ⚠️ Documented in BACKLOG.md | Lines 571 mentioned but NOT implemented |
| 6 | Toggle overlay: show/hide CMB angular power spectrum C_ℓ with qualitative comparison to Planck data | ⚠️ Documented in BACKLOG.md | Lines 576-580 have structure but NOT implemented |

#### Phase 4 Gaps Identified

**Gap:** Phase 4 items are well-documented in BACKLOG.md (Sprint 4 section, lines 533-838 and duplicate lines 790-838) with extensive subtasks.

**Issue:** Sprint 4 (Phase 4) appears to be duplicated in BACKLOG.md (appears twice with overlapping content), which could cause confusion during sprint planning.

### Phase 5 (Dark Ages & Structure Formation) - Gap Analysis

#### PRD Phase 5 Deliverables (Lines 191-211)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Direct-sum N-body gravity on GPU (wgpu compute shader) for up to 500K particles | ⚠️ Documented in BACKLOG.md | Lines 845-856 have structure but NOT implemented |
| 2 | Barnes-Hut octree (CPU build, GPU traversal) for scaling to 1M–10M particles | ⚠️ Documented in BACKLOG.md | Lines 848-851 have structure but NOT implemented |
| 3 | Dark matter particles seeded from Phase 2 perturbation field; baryonic particles coupled | ⚠️ Documented in BACKLOG.md | Lines 858-864 have structure but NOT implemented |
| 4 | Adaptive level-of-detail: particle splitting in high-density regions, merging in voids | ⚠️ Documented in BACKLOG.md | Lines 865-868 have structure but NOT implemented |
| 5 | Halo finder (Friends-of-Friends algorithm) identifying collapsed structures in real time | ⚠️ Documented in BACKLOG.md | Lines 869-877 have structure but NOT implemented |
| 6 | Cosmic web visualization: filaments rendered as line geometry connecting halos, voids rendered as transparent dark regions | ⚠️ Documented in BACKLOG.md | Lines 886-884 have structure but NOT implemented |
| 7 | Data export: HDF5 snapshot export (particle positions, velocities, masses, temperatures) and CSV timeline summary | ⚠️ Documented in BACKLOG.md | Lines 892-896 have structure but NOT implemented |

#### Phase 5 Gaps Identified

**Gap:** Phase 5 items are well-documented in BACKLOG.md (Sprint 5 section, lines 842-903) with extensive subtasks.

**Notable Observation:** Phase 5 (N-body gravity, cosmic web) is the most extensively documented complex phase, indicating awareness of technical difficulty and planning for subtasks.

### Phase 6 (Cosmic Dawn & Galaxy Formation) - Gap Analysis

#### PRD Phase 6 Deliverables (Lines 214-233)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Smoothed Particle Hydrodynamics (SPH) with Wendland C4 kernel for baryonic gas dynamics | ⚠️ Documented in BACKLOG.md | Lines 910-920 have structure but NOT implemented |
| 2 | Radiative cooling functions (Sutherland & Dopita 1993 tables) driving gas collapse | ⚠️ Documented in BACKLOG.md | Lines 916-917 have structure but NOT implemented |
| 3 | Sub-grid star formation: Kennicutt-Schmidt relation converts dense gas into star particles | ⚠️ Documented in BACKLOG.md | Lines 922-939 have structure but NOT implemented |
| 4 | Pop III star formation in early halos; first light sources appear as bright point lights | ⚠️ Documented in BACKLOG.md | Lines 932-939 have structure but NOT implemented |
| 5 | Reionization visualization: ionization fronts expand as signed-distance-field bubbles around star-forming halos | ⚠️ Documented in BACKLOG.md | Lines 941-1005 have structure but NOT implemented |
| 6 | Galaxy billboard sprites: halos above mass threshold render as composite galaxy sprites with morphology based on merger history | ⚠️ Documented in BACKLOG.md | Lines 947-991 have structure but NOT implemented |
| 7 | Procedural ambient audio: deep bass drones during dark ages, rising harmonic tones as first stars ignite, full cosmic soundscape | ⚠️ Documented in BACKLOG.md | Lines 1007-1012 have structure but NOT implemented |
| 8 | VTK mesh export for density and velocity fields on regular grid | ⚠️ Documented in BACKLOG.md | Lines 1014-1016 have structure but NOT implemented |

#### Phase 6 Gaps Identified

**Gap:** Phase 6 items are well-documented in BACKLOG.md (Sprint 6 section, lines 907-1019) with extensive subtasks.

**Notable Observation:** Phase 6 (SPH, galaxy formation, audio) is the most extensively documented phase, with very detailed task breakdowns for galaxy sprite textures, billboard systems, reionization bubbles, and audio.

### Phase 7 (Polish, Cinematic Mode & Release) - Gap Analysis

#### PRD Phase 7 Deliverables (Lines 237-258)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Performance optimization pass: GPU shader profiling, memory budget enforcement, particle LOD tuning to hit 60 FPS / 1M particles on GTX 1660 | ⚠️ Documented in BACKLOG.md | Lines 1026-1101 have extensive structure but NOT implemented |
| 2 | Cinematic mode: pre-authored camera paths with keyframes and easing curves, narrated text overlays explaining each epoch | ⚠️ Documented in BACKLOG.md | Lines 1103-1205 have extensive structure but NOT implemented |
| 3 | Expanded parameter panel: full cosmological parameter set (Ωₘ, ΩΛ, H₀, n_s, σ₈) with presets | ⚠️ Documented in BACKLOG.md | Lines 1207-1288 have extensive structure but NOT implemented |
| 4 | Data overlay suite: temperature map, density field, velocity streamlines, dark matter distribution, power spectrum P(k) with observational comparison lines | ⚠️ Documented in BACKLOG.md | Lines 1263-1284 have structure but NOT implemented |
| 5 | PNG/EXR high-resolution frame capture with HDR support | ⚠️ Documented in BACKLOG.md | Lines 1290-1328 have extensive structure but NOT implemented |
| 6 | Benchmarking harness with automated performance regression tests | ⚠️ Documented in BACKLOG.md | Lines 1330-1365 have extensive structure but NOT implemented |
| 7 | Cross-platform release builds: Linux, macOS (including Apple Silicon), Windows | ⚠️ Documented in BACKLOG.md | Lines 1368-1383 have structure but NOT implemented |
| 8 | User documentation, README, and tutorial walkthrough | ⚠️ Documented in BACKLOG.md | Lines 1369-1372 have structure but NOT implemented |
| 9 | Preset configuration sharing via TOML files | ⚠️ Documented in BACKLOG.md | Line 1372 mentioned but NOT implemented |

#### Phase 7 Gaps Identified

**Gap:** Phase 7 items are extremely well-documented in BACKLOG.md (Sprint 7 section, lines 1023-1387) with very detailed task breakdowns.

**Notable Observation:** Phase 7 has the most extensive documentation in BACKLOG.md, with granular subtasks for GPU profiling, memory tracking, cinematic mode, narration, and benchmarking.

### Summary of Gap Findings

#### Critical Missing Requirements (Not in TODO or BACKLOG)

1. **Phase 1 Temperature Resource Module** - Required for epoch indicator display
2. **Phase 1 Scale Factor Resource Module** - Required for epoch indicator display
3. **Phase 1 Per-Instance Attribute Synchronization Implementation Path** - Infrastructure exists but no implementation tasks
4. **Phase 1 Config::load() Implementation** - Required for external configuration
5. **Phase 2 Friedmann Equation Implementation** - Core physics for inflation
6. **Phase 2 Gaussian Random Field Generation** - Required for density perturbations
7. **Phase 2 Zel'dovich Approximation** - Required for quantum seeds
8. **Phase 2 Parameter Panel UI** - Required for interactive parameter adjustment
9. **Phase 1 Camera Interpolation Trigger on Epoch Change** - Infrastructure exists but no trigger system

#### Implementation Priority Recommendations

**Sprint 1 Completion (Phase 1):**
1. Implement Config::load() method with external TOML file reading
2. Implement Temperature resource module with epoch-based updates
3. Implement Scale Factor resource module with epoch-based updates
4. Create Epoch Indicator UI panel with era name, temperature, scale factor display
5. Implement per-instance particle attribute synchronization system
6. Add simulation snapshot and history system for reverse/replay
7. Resolve configuration field mismatches (CameraConfig.camera_mode string vs enum)
8. Add particle scaling tasks with performance monitoring for 100K-1M target

**Sprint 2 (Phase 2 - Inflation):**
- BACKLOG.md has extensive documentation for Phase 2 tasks (lines 592-686)
- Prioritize Friedmann equation implementation first as it's foundational for all subsequent physics
- Gaussian random field generation should be implemented second
- Zel'dovich approximation can use density field from GRF generation

**Architecture Observations:**

1. **Strong Foundation:** Phase 1 infrastructure is well-implemented with Bevy ECS, rendering pipeline, camera controls, and UI framework
2. **Comprehensive Planning:** BACKLOG.md shows excellent sprint planning with granular subtasks for all phases
3. **Missing Infrastructure Core:** Temperature and Scale Factor resources are foundational for all phases and should be implemented in Sprint 1
4. **Configuration System:** Config structures are well-defined but external loading is missing
5. **Per-Instance Rendering:** GPU attribute infrastructure exists but synchronization is incomplete

**Issues to Address:**

1. **Documentation Inconsistency:** Sprint 4 (Phase 4) is duplicated in BACKLOG.md (lines 533-838 and 790-838)
2. **Camera Interpolation Timing:** Camera interpolation is currently implemented but PRD designates it as Phase 7 feature
3. **Easing Function Module:** Easing functions are documented but no file creation or implementation tasks exist
