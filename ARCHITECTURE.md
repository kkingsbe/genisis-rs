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
│       ├── physics/      # Particle physics data structures
│       ├── epoch/        # Epoch management and transitions
│       │   ├── mod.rs    # EpochPlugin trait, EpochManager resource, update_epoch_transition system
│       │   ├── singularity.rs  # Singularity epoch implementation
│       │   └── camera_config.rs  # Camera configuration for epoch transitions
│       ├── config.rs     # Configuration structures (WindowConfig, ParticleConfig, etc.)
│       └── lib.rs
├── genesis-render/   # Rendering systems and visuals
│   └── src/
│       ├── particle/     # Instanced particle rendering
│       │   └── mod.rs   # PointSpriteMaterial, PointMesh, Particle component, spawn/update systems
│       ├── camera/       # Camera mode definitions and state
│       │   ├── mod.rs   # CameraState, CameraController, OrbitController, CameraTarget, interpolation systems
│       │   └── epoch_transition.rs  # Epoch transition camera handling
│       ├── input/        # Keyboard and mouse input handling
│       │   └── mod.rs   # InputState resource, handle_keyboard_input, handle_mouse_input
│       └── lib.rs
└── genesis-ui/        # User interface components
    └── src/
        ├── timeline/     # Timeline scrubber and time controls
        │   └── mod.rs   # CosmicTime resource, PlaybackState resource, timeline_panel_ui system
        ├── overlay/      # FPS, particle count, epoch info panels
        │   ├── mod.rs   # OverlayState resource, update_overlay_ui system
        │   └── camera_fade.rs  # Camera fade effect for epoch transitions
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
- **EpochManagerPlugin** (genesis-core): Epoch management and automatic transitions
- **SingularityEpochPlugin** (main.rs): Local plugin that registers SingularityEpoch (from genesis-core) with EpochManager
- **InputPlugin** (genesis-render): Keyboard and mouse input handling with InputState resource
- **ParticlePlugin** (genesis-render): Particle system initialization and spawning (with PointSpriteMaterial and PointMesh resources)
- **CameraPlugin** (genesis-render): Camera control systems (free-flight, orbit, interpolation, epoch transitions) with CameraState resource
- **GenesisUiPlugin** (genesis-ui): UI system with bevy_egui integration, overlay, and timeline controls
- **OverlayState** (genesis-ui): Resource for overlay visibility (initialized with show_fps, show_particle_count, show_epoch_info = true)
- **CameraFadeState** (genesis-ui): Resource for camera fade effect during epoch transitions (auto-initialized)
- **setup_camera**: Camera setup system that spawns 3D camera at z=50.0 looking at origin with OrbitController and CameraController components
- **setup_test_camera_target**: Spawns a test CameraTarget entity for testing camera interpolation
- **TimeAccumulator** (genesis-core): Resource for tracking cosmic years (auto-initialized)
- **CameraState** (genesis-render): Resource for tracking camera mode, target, and interpolation state (auto-initialized, defaults to FreeFlight mode)
- **CosmicTime** (genesis-ui): Resource for timeline state management with logarithmic slider mapping (auto-initialized)
- **PlaybackState** (genesis-ui): Resource for playback control (auto-initialized, playing state and speed)

## Core Architectural Decisions

### 1. Modular Crate Architecture
- **Rationale**: Separates concerns into core, render, and UI domains
- **Benefit**: Clear dependency boundaries, easier testing, parallel development
- **genesis-core**: Pure simulation logic (epoch, physics, time), depends on Bevy for Resource trait
- **genesis-render**: Rendering systems using Bevy ECS (camera, particle components)
- **genesis-ui**: UI state resources using Bevy ECS (timeline, overlay)

### 2. Bevy ECS Pattern
- **Components**: `Particle` (rendering component with position: Vec3, color: Color, size: f32)
  - Particles are spawned with Mesh3d, MeshMaterial3d<PointSpriteMaterial>, Transform, and Particle components
  - Camera components: `CameraController` (free-flight), `OrbitController` (orbit), `CameraTarget` (marker for camera interpolation)
- **Resources**: Global state organized by crate:
   - genesis-core: EpochManager, TimeAccumulator
   - genesis-render: CameraState, InputState, PointMesh
   - genesis-ui: CosmicTime, OverlayState, PlaybackState, CameraFadeState
- **Systems**:
   - Core: update_epoch_transition
   - Particle: init_point_mesh, spawn_particles, update_particles (basic outward expansion animation), update_particle_energy_colors (thermal gradient coloring)
   - Camera: update_free_flight_camera, update_orbit_camera, toggle_camera_mode, handle_orbit_zoom, interpolate_camera, update_camera_targets, handle_epoch_change_transition
   - Input: handle_keyboard_input, handle_mouse_input
   - Time: initialize_time_accumulator, update_cosmic_time, sync_time_resources (syncs play/pause state only - speed mapping pending)
   - UI: update_overlay_ui (overlay rendering), timeline_panel_ui (timeline controls), setup_camera_fade_overlay, update_camera_fade
- **Plugins**:
   - EpochManagerPlugin (implemented): Epoch registration and transition management
   - TimeIntegrationPlugin (implemented): Cosmic time accumulation with Bevy integration
   - InputPlugin (implemented): Keyboard and mouse input processing
   - CameraPlugin (implemented): Camera control systems for free-flight, orbit, interpolation, and epoch transitions
   - ParticlePlugin (implemented): Particle spawning and rendering systems
   - GenesisUiPlugin (implemented): UI system with overlay, timeline, and camera fade panels

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
  - Epoch tracking via update_epoch_transition system is implemented
  - Timeline UI controls fully implemented with logarithmic slider mapping
  - Time synchronization between UI and accumulator fully implemented
  - Speed-to-acceleration mapping: **Pending** - PlaybackState.speed is not yet mapped to TimeAccumulator.acceleration

### 5. Camera System Design
- **Camera Modes**: FreeFlight and Orbit enum variants defined (Orbit is default)
- **State Tracking**: CameraState resource with mode, target, current_orbit_target, and interpolation state fields
- **Components**:
  - CameraController: Free-flight camera with yaw, pitch, movement_speed, mouse_sensitivity
  - OrbitController: Orbit camera with distance, yaw, pitch, target, zoom limits, rotation sensitivity, zoom sensitivity
  - CameraTarget: Marker component that identifies entities the camera should look at or move toward
- **Dual-Controller Architecture**: Both OrbitController and CameraController components are always present on the camera entity. Mode switching (via toggle_camera_mode) affects which controller responds to input, not component attachment.
- **Camera Interpolation**: CameraState includes interpolation support with start_interpolation_to_target(), start_interpolation_to_position_only() methods. The interpolate_camera system handles smooth transitions.
- **Epoch Camera Configuration**: EpochPlugin trait includes camera_config() method that returns EpochCameraConfig with optional target_position, target_rotation, and target_mode. The handle_epoch_change_transition system automatically transitions the camera when epochs change.
- **Status**:
  - Camera setup (setup_camera system): Implemented - spawns 3D camera at z=50.0 looking at origin with OrbitController::default() and CameraController::default()
  - Camera movement controls: Implemented for both free-flight (update_free_flight_camera) and orbit (update_orbit_camera) modes
  - Camera mode switching: Implemented via toggle_camera_mode system (press 'O' key to toggle between FreeFlight and Orbit)
  - Orbit camera zoom: Implemented via handle_orbit_zoom system (scroll wheel controls zoom distance)
  - Camera interpolation: Implemented via interpolate_camera system with smoothstep easing
  - Camera targets: Implemented via CameraTarget component and update_camera_targets system
  - Epoch transition camera handling: Implemented via handle_epoch_change_transition system

### 6. Input System Architecture
- **InputState Resource**: Tracks keyboard direction vector, mouse delta, and mouse button states
- **Keyboard Handling**: WASD key inputs mapped to directional vectors
- **Mouse Handling**: Mouse button states tracked using HashMap<MouseButton, bool>; Tracks mouse motion delta
- **Status**: InputPlugin fully implemented with handle_keyboard_input and handle_mouse_input systems (run in PreUpdate schedule)

### 7. Epoch Plugin Architecture
- **Registration System**: EpochPlugin trait for defining epoch time ranges, camera configuration, and building systems
- **EpochManager**: Resource that tracks registered epochs and manages transitions
- **EpochChangeEvent**: Event emitted when simulation transitions between epochs
- **update_epoch_transition**: System that automatically transitions epochs based on cosmic time
- **EpochCameraConfig**: Configuration for camera transitions when entering an epoch (target_position, target_rotation, target_mode)
- **handle_epoch_change_transition**: System that responds to EpochChangeEvent and triggers camera interpolation to the new epoch's camera configuration
- **Benefit**: Extensible for adding new cosmic epochs with automatic camera transitions

### 8. Configuration Management
- **Format**: TOML for human-readable configuration
- **Override**: Command-line arguments can override config values
- **Defaults**: Embedded default configuration for "Standard Model" preset
- **Status**: Configuration system fully implemented
  - Config struct with WindowConfig, ParticleConfig, CameraConfig, TimeConfig, DisplayConfig
  - TOML serialization/deserialization via serde
  - CLI argument parsing via clap (--config flag)
  - Default location search (./genesis.toml, ~/.config/genesis/config.toml, /etc/genesis/config.toml)
  - ConfigResource wrapper for Bevy integration

### 9. Camera Fade System
- **Purpose**: Provides visual crossfade effect during epoch transitions
- **Components**: CameraFade (marker component identifying the fade overlay entity)
- **Resources**: CameraFadeState tracks fade opacity, state (None/FadingOut/FadingIn), and duration
- **Systems**:
  - setup_camera_fade_overlay: Creates full-screen white overlay that is initially transparent
  - update_camera_fade: Responds to EpochChangeEvent, fades out to white, then fades back in using smoothstep easing
- **Status**: Fully implemented for smooth epoch transition visualization

## Phase 1 Scope (Current Sprint)

### Goal
A running Bevy application with a 3D particle system, camera controls, and a time slider.

### Implementation Status
**Implemented:**
- Core infrastructure setup (workspace, Cargo.toml)
- Bevy 0.15+ application scaffold with window and event loop
- Epoch manager plugin architecture (EpochManager, EpochPlugin trait, EpochChangeEvent)
- Epoch camera configuration (EpochCameraConfig, camera_config() method on EpochPlugin)
- Epoch transition camera handling (handle_epoch_change_transition system)
- Camera fade effect for epoch transitions (CameraFadeState, setup_camera_fade_overlay, update_camera_fade)
- Basic input handling (keyboard, mouse) - InputPlugin with InputState, handle_keyboard_input, handle_mouse_input (runs in PreUpdate)
- Time integration system with f64 accumulator - TimeIntegrationPlugin, TimeAccumulator, update_cosmic_time system, pause/resume methods
- Epoch tracking via update_epoch_transition system
- Particle rendering system with custom point sprite shader (PointSpriteMaterial, PointMesh)
- Particle spawning system (spawn_particles) that creates test cluster at origin
- Particle update system (update_particles) with basic outward expansion animation
- Energy-based particle color system (update_particle_energy_colors) with thermal gradient (white-hot core → red edges)
- Camera system with free-flight and orbit modes - CameraPlugin, CameraController, OrbitController, update_free_flight_camera, update_orbit_camera
- Camera mode switching via toggle_camera_mode (press 'O' key)
- Orbit camera zoom via scroll wheel (handle_orbit_zoom)
- Camera interpolation support with smoothstep easing (interpolate_camera)
- Camera target support (CameraTarget component, update_camera_targets system)
- Overlay UI with FPS, particle count, and epoch info panels - update_overlay_ui system
- Timeline UI with play/pause, logarithmic slider, and speed control - TimelinePlugin, CosmicTime resource with logarithmic mapping, timeline_panel_ui system (runs in PostUpdate)
- Time synchronization (sync_time_resources) between PlaybackState and TimeAccumulator

**Pending:**
- Full physics-based particle updates with simulation-level particle sync (update_particles has basic outward expansion, full physics sync pending)
- Per-instance particle color and size synchronization with GPU instance attributes (Particle component → GPU attributes)
- PlaybackState.speed to TimeAccumulator.acceleration mapping (speed control exists but not connected)

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
- `EpochManager` (genesis-core) is used by epoch transition systems in genesis-render
- `OverlayState` (genesis-ui) is used by overlay UI system
- `CameraFadeState` (genesis-ui) is used by camera fade system

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
- Speed-to-acceleration mapping for timeline speed control

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
