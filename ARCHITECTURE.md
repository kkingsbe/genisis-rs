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
│       ├── time/         # Cosmic time accumulator and epoch management
│       ├── physics/      # Particle physics and interactions
│       └── lib.rs
├── genesis-render/   # Rendering systems and visuals
│   └── src/
│       ├── particle/     # Instanced particle rendering
│       ├── camera/       # Camera systems (free-flight, orbit)
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

Note: The application is currently minimal, with only the basic Bevy app setup
and EpochManagerPlugin registered. Most plugin files and resources are not yet
created.

## Core Architectural Decisions

### 1. Modular Crate Architecture
- **Rationale**: Separates concerns into core, render, and UI domains
- **Benefit**: Clear dependency boundaries, easier testing, parallel development
- **genesis-core**: Pure simulation logic (epoch, physics, time), depends on Bevy for Resource trait
- **genesis-render**: Rendering systems using Bevy ECS (camera, particle components)
- **genesis-ui**: UI state resources using Bevy ECS (timeline, overlay)

### 2. Bevy ECS Pattern
- **Components**: ParticleComponent (marker), CameraState (resource), EpochManager (resource), TimeAccumulator (resource)
- **Systems**: update_epoch_transition, update_particles (stub)
- **Resources**: Global state (EpochManager, TimeAccumulator, CameraState, OverlayState, PlaybackState)
- **Plugins**: EpochManagerPlugin (implemented)

### 3. Instanced Particle Rendering
- **Design**: GPU instancing with Bevy PBR materials
- **Capacity**: 100K - 1M particles (planned)
- **Attributes**: Position (Vec3), Color (Vec4), Size (f32)
- **Status**: Basic ParticleComponent marker and update_particles system stub defined; actual GPU rendering not yet implemented

### 4. Cosmic Time System
- **Type**: f64 accumulator for precision over 13.8B years
- **Acceleration**: 1x to 10¹²x time scaling (with bounds checking)
- **Status**: Basic TimeAccumulator resource defined; UI controls and epoch tracking not yet implemented

### 5. Camera System Design
- **Camera Modes**: FreeFlight and Orbit enum variants defined
- **State Tracking**: CameraState resource with mode and target fields
- **Status**: Basic structure defined; input handling and camera movement systems not yet implemented

### 6. Epoch Plugin Architecture
- **Registration System**: EpochPlugin trait for defining epoch time ranges and building systems
- **EpochManager**: Resource that tracks registered epochs and manages transitions
- **update_epoch_transition**: System that automatically transitions epochs based on cosmic time
- **Benefit**: Extensible for adding new cosmic epochs

### 7. Configuration Management (Planned)
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

**Pending:**
- Basic input handling (keyboard, mouse)
- Time integration system with f64 accumulator (TimeAccumulator defined, not integrated)
- Instanced particle renderer with GPU support (ParticleComponent stub, TODO in update_particles)
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
genesis-core (no external dependencies)
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
