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
│       ├── epoch/        # Epoch markers and camera configuration
│       │   ├── mod.rs        # Exports: CameraMode enum, EpochCameraConfig struct
│       │   ├── singularity.rs    # Singularity epoch marker struct (does NOT implement EpochPlugin trait)
│       │   └── camera_config.rs  # EpochCameraConfig, CameraMode enum for epoch camera transitions
│       ├── config.rs     # Configuration structures (WindowConfig, ParticleConfig, CameraConfig, etc.)
│       └── lib.rs
├── genesis-render/   # Rendering systems and visuals
│   └── src/
│       ├── particle/     # Instanced particle rendering
│       │   ├── mod.rs           # PointSpriteMaterial, PointMesh, Particle component, spawn/update systems
│       │   └── point_sprite.wgsl  # WGSL shader for point sprite rendering
│       ├── camera/       # Camera mode definitions and state
│       │   └── mod.rs   # CameraState, CameraController, OrbitController, camera control systems
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
- **ConfigResource** (main.rs): Wrapper for Config as a Bevy Resource (NOTE: Config::load() is NOT implemented - uses Config::default())
- **ParticleConfig** (genesis-core): Resource for particle spawning configuration (correctly used directly with Resource derive in main.rs line 48)
- **CameraState** (genesis-render): Resource for tracking camera mode, target, and interpolation state (initialized from CameraConfig)
- **OverlayState** (genesis-ui): Resource for overlay visibility (initialized from DisplayConfig: show_fps, show_particle_count; NOTE: show_epoch_info field is missing from OverlayState struct but present in DisplayConfig and genesis.toml)
- **CosmicTime** (genesis-ui): Resource for timeline state management with logarithmic slider mapping (auto-initialized by TimelinePlugin)
- **PlaybackState** (genesis-ui): Resource for playback control (auto-initialized by TimelinePlugin)
- **TimeAccumulator** (genesis-core): Resource for tracking cosmic years (auto-initialized by TimeIntegrationPlugin)
- **setup_camera**: Camera setup system that spawns 3D camera at orbit_distance looking at origin with OrbitController and CameraController components (correctly uses config.camera.orbit_distance which matches the CameraConfig struct)

**NOTE**: The following epoch management infrastructure does NOT exist in the codebase (though documented as if it does):
- EpochManager resource - NOT defined
- EpochManagerPlugin - NOT defined
- EpochPlugin trait - NOT defined
- update_epoch_transition system - NOT defined
- SingularityEpoch - defined as marker struct but does NOT implement EpochPlugin trait (trait doesn't exist)

Only EpochCameraConfig, CameraMode enum, and SingularityEpoch marker struct exist.

## Core Architectural Decisions

### 1. Modular Crate Architecture
- **Rationale**: Separates concerns into core, render, and UI domains
- **Benefit**: Clear dependency boundaries, easier testing, parallel development
- **genesis-core**: Pure simulation logic (epoch, physics, time), depends on Bevy for Resource trait
  - Exports: Config, ParticleConfig, CameraConfig, TimeConfig, WindowConfig, DisplayConfig, TimeIntegrationPlugin, CameraMode, EpochCameraConfig, SingularityEpoch
  - **NOTE**: EpochManager, EpochManagerPlugin, EpochPlugin, and EpochChangeEvent are NOT exported (not defined in codebase)
- **genesis-render**: Rendering systems using Bevy ECS (camera, particle components)
  - Exports: CameraPlugin, CameraState, InputPlugin, ParticlePlugin
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
   - Particle: init_point_mesh (Startup), spawn_particles (Startup), update_particles (basic outward expansion animation), update_particle_energy_colors (thermal gradient coloring)
   - Camera: update_free_flight_camera (Update), update_orbit_camera (Update), toggle_camera_mode (Update), handle_orbit_zoom (Update), handle_orbit_pan (Update), interpolate_camera (PostUpdate)
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
  - Particle spawning system (spawn_particles): **Implemented** (1000 test particles)
  - Shared mesh resource initialization: **Implemented**
  - Physics-based particle updates: **Implemented (basic outward expansion animation)**
  - Per-instance particle size and color attributes: **Infrastructure implemented ( ATTRIBUTE_INSTANCE_SIZE, ATTRIBUTE_INSTANCE_COLOR)** - synchronization with Particle component pending
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
Currently, the rendering-level Particle is directly populated in [`spawn_particles()`](genesis-render/src/particle/mod.rs:209) with test data. The planned architecture will:
1. Run physics calculations on simulation-level particles
2. Convert simulation state to render components
3. Update entity Transforms and Particle components with new positions

### 3.2 Point Sprite Rendering Resources

**PointSpriteMaterial**
- Custom Bevy material implementing the `Material` trait
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
  - **Synchronization**: The sync_time_resources system synchronizes TimeAccumulator's paused state with PlaybackState.playing and maps speed to acceleration
  - **Known Issue**: Timeline scrubbing updates CosmicTime.cosmic_time but does NOT sync back to TimeAccumulator.years. The two resources can become desynchronized when the user scrubs the timeline.
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
  - Timeline scrubbing to TimeAccumulator synchronization: **NOT IMPLEMENTED** - slider changes only affect CosmicTime resource

### 5. Camera System Design
- **Camera Modes**: FreeFlight and Orbit enum variants defined (Orbit is default)
- **State Tracking**: CameraState resource with mode, target, current_orbit_target, and interpolation state fields
- **Components**:
  - CameraController: Free-flight camera with yaw, pitch, movement_speed, mouse_sensitivity
  - OrbitController: Orbit camera with distance, yaw, pitch, target, zoom limits, rotation sensitivity, zoom sensitivity, pan sensitivity
- **Dual-Controller Architecture**: Both OrbitController and CameraController components are always present on the camera entity. Mode switching (via toggle_camera_mode) affects which controller responds to input, not component attachment.
- **Camera Interpolation**: CameraState includes interpolation support with start_interpolation_to_target(), start_interpolation_to_position_only() methods. The interpolate_camera system handles smooth transitions with smoothstep easing.
- **Configuration**:
  - CameraConfig in genesis-core has fields: initial_position, initial_target, camera_mode (String), movement_speed, orbit_distance
  - CameraMode enum exists in genesis-core::epoch but CameraConfig uses String for camera_mode field
  - genesis.toml has fields: initial_mode (String), orbit_distance (f64)
- **Configuration Field Mismatches**:
  - **camera_mode vs CameraMode enum**: CameraConfig.camera_mode is a String, and CameraMode enum exists. CameraState::from_config() in genesis-render/src/camera/mod.rs (line 144) correctly accesses `config.camera_mode` and converts it to the CameraMode enum.
  - **initial_time_acceleration vs default_time_acceleration**: genesis.toml has `initial_time_acceleration` field, but TimeConfig struct has `default_time_acceleration` field
- **Status**:
  - Camera setup (setup_camera system): Implemented - spawns 3D camera at orbit_distance looking at origin with OrbitController (distance: orbit_distance) and CameraController::default().
  - Camera movement controls: Implemented for both free-flight (update_free_flight_camera) and orbit (update_orbit_camera) modes
  - Camera mode switching: Implemented via toggle_camera_mode system (press 'O' key to toggle between FreeFlight and Orbit)
  - Orbit camera zoom: Implemented via handle_orbit_zoom system (scroll wheel controls zoom distance, clamped between min_distance and max_distance)
  - Orbit camera pan: Implemented via handle_orbit_pan system (middle or right mouse button to pan orbit target)
  - Camera interpolation: Implemented via interpolate_camera system with smoothstep easing
  - Epoch transition camera handling: EpochCameraConfig exists but EpochPlugin trait does NOT exist, so no system is implemented

### 6. Input System Architecture
- **InputState Resource**: Tracks keyboard direction vector, mouse delta, and mouse button states
- **Keyboard Handling**: WASD key inputs mapped to directional vectors
- **Mouse Handling**: Mouse button states tracked using HashMap<MouseButton, bool>; Tracks mouse motion delta
- **Status**: InputPlugin fully implemented with handle_keyboard_input and handle_mouse_input systems (run in PreUpdate schedule)

### 7. Epoch Plugin Architecture
- **What Exists**: Only the following epoch-related types are defined:
  - CameraMode enum (FreeFlight, Orbit) in genesis-core::epoch::camera_config
  - EpochCameraConfig struct in genesis-core::epoch::camera_config (camera transition configuration)
  - SingularityEpoch marker struct in genesis-core::epoch::singularity
- **What Does NOT Exist** (despite being documented as if it does):
  - EpochPlugin trait - NOT defined anywhere
  - EpochManager resource - NOT defined anywhere
  - EpochManagerPlugin - NOT defined anywhere
  - EpochChangeEvent - NOT defined anywhere
  - update_epoch_transition system - NOT defined anywhere
  - handle_epoch_change_transition system - NOT defined anywhere
- **SingularityEpoch**: Marker struct defined but does NOT implement EpochPlugin trait (trait doesn't exist)
- **Benefit**: EpochCameraConfig provides infrastructure for future epoch transitions
- **Status**: Epoch management infrastructure is NOT implemented. Only epoch marker and camera configuration types exist.

### 8. Configuration Management
- **Format**: TOML for human-readable configuration
- **Status**: Configuration system partially implemented with field name mismatches
  - Config struct with WindowConfig, ParticleConfig, CameraConfig, TimeConfig, DisplayConfig fully defined
  - Default implementations provided for all config structs
  - ConfigResource wrapper for Bevy integration
- **Missing**: Config::load() method is referenced in main.rs but not implemented in genesis-core/src/config.rs
  - TOML serialization/deserialization via serde is defined but load() method not implemented
  - CLI argument parsing via clap (--config flag) is defined but not connected
  - Default location search logic not implemented
- **Configuration Field Mismatches**:
  - **ParticleConfig**: genesis.toml has `initial_count`, `max_count`, `base_size` but ParticleConfig struct has `particle_count`, `particle_size_base`, `particle_size_variation`, `color_hot`, `color_cool`
  - **CameraConfig**: genesis.toml has `initial_mode`, `orbit_distance` but CameraConfig struct has `initial_position`, `initial_target`, `camera_mode` (String), `movement_speed`, `orbit_distance`
  - **DisplayConfig**: genesis.toml has `show_epoch_info` but OverlayState struct does not have this field
  - **TimeConfig**: genesis.toml has `initial_time_acceleration` but TimeConfig struct has `default_time_acceleration`
- **Note**: Configuration loading infrastructure needs to be implemented to enable external TOML configuration, and field names need to be reconciled between genesis.toml and Config structs

## Phase 1 Scope (Current Sprint)

### Goal
A running Bevy application with a 3D particle system, camera controls, and a time slider.

### Implementation Status
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
- Camera interpolation support with smoothstep easing (interpolate_camera)
- Overlay UI with FPS and particle count panels - update_overlay_ui system
- Timeline UI with play/pause, logarithmic slider, and speed control - TimelinePlugin, CosmicTime resource with logarithmic mapping, timeline_panel_ui system (runs in PostUpdate)
- Time synchronization (sync_time_resources) between PlaybackState and TimeAccumulator including speed-to-acceleration mapping

**Partially Implemented (Infrastructure exists but not connected):**
- Epoch camera configuration (EpochCameraConfig struct defined, CameraMode enum defined) - infrastructure exists but not used
- SingularityEpoch - defined as marker struct but does NOT implement EpochPlugin trait (trait doesn't exist)
- Per-instance particle attributes (ATTRIBUTE_INSTANCE_SIZE, ATTRIBUTE_INSTANCE_COLOR) - mesh attributes defined but not synchronized with Particle component data
- Dual time system (TimeAccumulator.years + CosmicTime.cosmic_time) - both exist but timeline scrubbing doesn't sync back to TimeAccumulator

**Pending:**
- Config::load() method implementation for external TOML configuration (main.rs calls Config::load() but method doesn't exist)
- SingularityEpoch implementation of EpochPlugin trait (trait doesn't exist - would need to create trait first)
- EpochManager, EpochManagerPlugin, EpochPlugin trait, EpochChangeEvent - entire epoch management system needs implementation
- Epoch transition systems (update_epoch_transition, handle_epoch_change_transition) - do not exist
- Full physics-based particle updates with simulation-level particle sync (update_particles has basic outward expansion, full physics sync pending)
- Per-instance particle color and size synchronization with GPU instance attributes (Particle component → GPU attributes)
- OverlayState.show_epoch_info field addition to struct and UI rendering
- Timeline scrubbing to TimeAccumulator synchronization (slider changes CosmicTime but not TimeAccumulator.years)
- ParticleConfigResource definition in genesis-core (main.rs references it but ParticleConfig has Resource derive - should use ParticleConfig directly)
- Configuration field name reconciliation between genesis.toml and Config structs:
  - ParticleConfig: reconcile genesis.toml fields (initial_count, max_count, base_size) with struct fields (particle_count, particle_size_base, particle_size_variation, color_hot, color_cool)
  - CameraConfig: reconcile genesis.toml fields (initial_mode, orbit_distance) with struct fields (camera_mode, orbit_radius) and CameraMode enum usage
- OverlayState.show_epoch_info field addition to struct and UI rendering (main.rs line 58 attempts to set this field but it doesn't exist in OverlayState struct)

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

## Future Considerations

- WebAssembly support for browser deployment
- Save/load simulation state
- Multi-threaded physics calculations
- Compute shaders for advanced particle interactions
- Audio integration for cosmic events
- Per-instance particle attribute synchronization for individual particle colors and sizes
- Epoch manager system implementation (create EpochPlugin trait, EpochManager resource, EpochManagerPlugin, update_epoch_transition system)
- Config::load() method implementation for external TOML configuration
- Timeline scrubbing to TimeAccumulator synchronization
- OverlayState.show_epoch_info field addition and UI rendering
- Configuration field name reconciliation between genesis.toml and Config structs

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

**Impact:** Resolves critical blocker preventing application startup. No code changes required.

### [2026-02-09] ViewUniform Shader Type Definition

**Issue:** Point sprite shader `point_sprite.wgsl` fails to compile because it references `ViewUniform` type (line 29) which is not defined. The shader uses `view.view_proj` to transform world positions to clip space, but the `ViewUniform` struct is missing.

**Error Message:**
```
error: unknown type: 'ViewUniform'
   ┌─ point_sprite.wgsl:29:20
   │
29 │ var<uniform> view: ViewUniform;
   │                    ^^^^^^^^^^^ unknown type
```

**Root Cause:** The shader was written assuming Bevy's standard `ViewUniform` type would be available via import, but Bevy's Material trait does not automatically provide the ViewUniform type definition in custom shaders. The type must be defined in the shader or imported from Bevy's shader library.

**Decision:** Define the `ViewUniform` struct directly in the shader file. This is the standard approach for Bevy custom materials that need view/projection data.

**Rationale:**
1. Bevy 0.15's Material trait provides automatic binding of the view uniform buffer at binding 1
2. The struct layout must match Bevy's internal ViewUniform definition for the binding to work
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
- Add the `ViewUniform` struct definition to `genesis-render/src/particle/point_sprite.wgsl` before the `@group(0) @binding(1)` uniform binding
- The struct must be defined at module level (not inside a function)
- Ensure the struct layout matches Bevy's internal ViewUniform: `view_proj: mat4x4<f32>` and `world_position: vec3<f32>`

**Impact:** Resolves the critical blocker that prevents the application from starting. This is a high-priority fix required before any particle rendering or visualization work can proceed. The fix is a single-line struct definition addition to the shader file.
