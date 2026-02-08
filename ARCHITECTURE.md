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
│       ├── physics/      # Particle physics and interactions
│       ├── epoch/        # Epoch management and transitions
│       └── lib.rs
├── genesis-render/   # Rendering systems and visuals
│   └── src/
│       ├── particle/     # Instanced particle rendering
│       ├── camera/       # Camera mode definitions and state
│       ├── input/        # Keyboard and mouse input handling
│       └── lib.rs
└── genesis-ui/        # User interface components
    └── src/
        ├── timeline/     # Timeline scrubber and time controls
        ├── overlay/      # FPS, particle count, epoch info panels
        └── lib.rs
```

### Application Structure
```
src/
└── main.rs              # Application entry point
```

The application registers the following plugins and resources:
- **DefaultPlugins**: Bevy's default set of plugins
- **TimeIntegrationPlugin**: Cosmic time accumulation with f64 precision
- **EpochManagerPlugin**: Epoch management and automatic transitions
- **InputPlugin**: Keyboard and mouse input handling
- **ParticlePlugin**: Particle system initialization and spawning (with PointSpriteMaterial and PointMesh resources)
- **setup_camera**: Camera setup system
- **TimeAccumulator**: Resource for tracking cosmic years (initialized)

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
- **Resources**: Global state (EpochManager, TimeAccumulator, CameraState, InputState, OverlayState, PlaybackState)
- **Systems**:
  - Core: update_epoch_transition, update_particles (stub)
  - Particle: init_point_mesh, spawn_particles, update_particles (stub)
  - Input: handle_keyboard_input, handle_mouse_input
  - Time: initialize_time_accumulator, update_cosmic_time
- **Plugins**:
  - EpochManagerPlugin (implemented): Epoch registration and transition management
  - TimeIntegrationPlugin (implemented): Cosmic time accumulation with Bevy integration
  - InputPlugin (implemented): Keyboard and mouse input processing

### 3. Instanced Particle Rendering
- **Design**: GPU instancing with custom PointSpriteMaterial using WGSL shaders
- **Capacity**: 100K - 1M particles (planned)
- **Attributes**: Position ([f32; 3]), Velocity ([f32; 3]), Color ([f32; 3]), Size (f32)
- **Status**:
  - Point sprite rendering with custom shader: **Implemented**
  - Particle spawning system (spawn_particles): **Implemented**
  - Shared mesh resource initialization: **Implemented**
  - Physics-based particle updates: **Pending (stub exists)**

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
Currently, the rendering-level Particle is directly populated in [`spawn_particles()`](genesis-render/src/particle/mod.rs:111) with test data. The planned architecture will:
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
- Uses additive blending (AlphaMode::Add) for a glowing effect
- Registered via `MaterialPlugin::<PointSpriteMaterial>::default()` in ParticlePlugin

**PointMesh**
- Resource containing a shared `Handle<Mesh>` for all particle entities
- Mesh topology: `PrimitiveTopology::PointList` with a single vertex at origin
- The `Transform` component on each entity provides actual position
- Initialized once at startup via `init_point_mesh` system
- Shared across all particles for efficient GPU instancing

### 4. Cosmic Time System
- **Type**: f64 accumulator for precision over 13.8B years
- **Acceleration**:
  - TimeAccumulator.acceleration handles the actual 1x-10¹²x scaling
  - PlaybackState.speed (f32, 0.0-1.0) is intended for UI logarithmic mapping (not yet implemented)
- **Status**:
  - TimeAccumulator resource fully implemented
  - TimeIntegrationPlugin integrates with Bevy's time system
  - Epoch tracking via update_epoch_transition system is implemented
  - UI controls not yet implemented

### 5. Camera System Design
- **Camera Modes**: FreeFlight and Orbit enum variants defined
- **State Tracking**: CameraState resource with mode and target fields
- **Status**: 
  - Camera setup (setup_camera system): Implemented - spawns 3D camera at z=50.0 looking at origin
  - Camera movement controls: Pending (input handling is separate and implemented via InputPlugin)

### 6. Input System Architecture
- **InputState Resource**: Tracks keyboard direction vector, mouse delta, and mouse button states
- **Keyboard Handling**: WASD key inputs mapped to directional vectors
- **Mouse Handling**: Mouse button states tracked using HashMap<MouseButton, bool>; Tracks mouse motion delta
- **Status**: InputPlugin fully implemented with handle_keyboard_input and handle_mouse_input systems

### 7. Epoch Plugin Architecture
- **Registration System**: EpochPlugin trait for defining epoch time ranges and building systems
- **EpochManager**: Resource that tracks registered epochs and manages transitions
- **update_epoch_transition**: System that automatically transitions epochs based on cosmic time
- **Benefit**: Extensible for adding new cosmic epochs

### 8. Configuration Management (Planned)
- **Format**: TOML for human-readable configuration
- **Override**: Command-line arguments can override config values
- **Defaults**: Embedded default configuration
- **Status**: Configuration system not yet implemented

## Phase 1 Scope (Current Sprint)

### Goal
A running Bevy application with a 3D particle system, camera controls, and a time slider.

### Implementation Status
**Implemented:**
- Core infrastructure setup (workspace, Cargo.toml)
- Bevy 0.15+ application scaffold with window and event loop
- Epoch manager plugin architecture (EpochManager, EpochPlugin trait)
- Basic input handling (keyboard, mouse) - InputPlugin with InputState, handle_keyboard_input, handle_mouse_input
- Time integration system with f64 accumulator - TimeIntegrationPlugin, TimeAccumulator, update_cosmic_time system
- Epoch tracking via update_epoch_transition system
- Particle rendering system with custom point sprite shader (PointSpriteMaterial, PointMesh)
- Particle spawning system (spawn_particles) that creates test cluster

**Pending:**
- Physics-based particle updates (update_particles system stub - TODO)
- Free-flight and orbit camera systems (CameraMode enum and CameraState resource, no movement systems)
- Time controls (PlaybackState resource defined, UI not implemented)
- Timeline scrubber UI (PlaybackState resource defined, UI not implemented)
- Basic overlays (OverlayState resource defined, UI not implemented)

## Dependency Graph

```
genesis-ui (bevy_egui)
    ↓
genesis-render (Bevy, wgpu)
    ↓
genesis-core (Bevy - for Resource trait)
```

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
