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
├── main.rs              # Application entry point
├── plugins/             # Bevy plugins
│   ├── epoch.rs         # Epoch manager and phase transitions
│   ├── camera.rs        # Camera system registration
│   ├── particle.rs      # Particle system registration
│   └── ui.rs            # UI system registration
└── resources/           # Global resources
    ├── config.toml      # Configuration file
    └── assets/          # Shaders, textures, etc.
```

## Core Architectural Decisions

### 1. Modular Crate Architecture
- **Rationale**: Separates concerns into core, render, and UI domains
- **Benefit**: Clear dependency boundaries, easier testing, parallel development
- **genesis-core**: Pure simulation logic, no rendering dependencies
- **genesis-render**: Rendering systems using Bevy ECS
- **genesis-ui**: UI components using bevy_egui

### 2. Bevy ECS Pattern
- **Components**: Particle, CameraState, Epoch, Time
- **Systems**: Update positions, render particles, handle input
- **Resources**: Global state (config, time accumulator)
- **Plugins**: Encapsulated systems for epoch, camera, particles, UI

### 3. Instanced Particle Rendering
- **Technique**: GPU instancing with Bevy PBR materials
- **Capacity**: 100K - 1M particles
- **Attributes**: Position (Vec3), Color (Vec4), Size (f32)
- **Benefit**: Efficient rendering of large particle counts

### 4. Cosmic Time System
- **Type**: f64 accumulator for precision over 13.8B years
- **Acceleration**: 1x to 10¹²x time scaling
- **UI Control**: Logarithmic timeline scrubber
- **Epoch Tracking**: Current era, temperature, scale factor

### 5. Camera System Design
- **Free-flight Camera**: WASD + mouse look for exploration
- **Orbit Camera**: Click-drag rotation around focal point
- **Transitions**: Smooth interpolation + crossfade for epoch changes
- **Controls**: Zoom, pan, smooth camera movement

### 6. Epoch Plugin Architecture
- **Registration System**: Plugins register for specific epoch ranges
- **Lifecycle**: Pre-epoch, during-epoch, post-epoch hooks
- **Phase Support**: 7 phases from Singularity to Dark Energy Era
- **Benefit**: Extensible for adding new cosmic epochs

### 7. Configuration Management
- **Format**: TOML for human-readable configuration
- **Override**: Command-line arguments can override config values
- **Defaults**: Embedded default configuration
- **Loading**: On startup, with validation

## Phase 1 Scope (Current Sprint)

### Goal
A running Bevy application with a 3D particle system, camera controls, and a time slider.

### Features
- Core infrastructure setup (workspace, Cargo.toml)
- Bevy 0.15+ application scaffold with window and event loop
- Basic input handling (keyboard, mouse)
- Epoch manager plugin architecture
- Time integration system with f64 accumulator
- Instanced particle renderer with GPU support
- Free-flight and orbit camera systems
- Time controls (play/pause, reset, speed adjustment)
- Timeline scrubber UI
- Basic overlays (FPS, particle count, epoch info)

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
