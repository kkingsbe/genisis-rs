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
│       ├── events.rs     # Event types for inter-system communication (ScrubbingEvent)
│       └── lib.rs
├── genesis-physics/   # Physics simulation modules
│   └── src/
│       ├── integrator/    # Numerical integrators for differential equations
│       │   └── mod.rs   # RK4 step and RK4 integrate functions for ODE solving
│       ├── cosmology/   # Cosmological physics (Friedmann equations)
│       │   └── mod.rs   # Curvature, EnergyDensity, ScaleFactor, HubbleParameter, CosmicEpoch, Cosmology, CosmologyPlugin (data structures + RK4/Euler integration + epoch-aware update system)
│       ├── gravity/      # Gravity simulation (placeholder)
│       │   └── mod.rs   # Gravity-related structures (not implemented)
│       ├── inflaton/     # Inflaton field dynamics for cosmic inflation
│       │   └── mod.rs   # Inflaton struct (phi, potential, derivatives, slow-roll parameters), InflatonPlugin (initializes Inflaton resource), comprehensive unit tests
│       ├── perturbations/ # Density perturbations (placeholder)
│       │   └── mod.rs   # Perturbation-related structures (not implemented)
│       └── nucleosynthesis/ # Big Bang nucleosynthesis (placeholder)
│           └── mod.rs   # Nucleosynthesis structures (not implemented)
├── genesis-render/   # Rendering systems and visuals
│   └── src/
│       ├── particle/     # Instanced particle rendering
│       │   ├── mod.rs           # PointSpriteMaterial, PointSpriteMaterialHandle, PointMesh, Particle component, spawn/update systems
│       │   ├── instance_buffer.rs # Storage buffer for per-instance particle data (GPU synchronization)
│       │   └── point_sprite.wgsl  # WGSL shader for point sprite rendering
│       ├── camera/       # Camera mode definitions and state
│       │   └── mod.rs   # CameraMode enum, CameraState, CameraController, OrbitController, camera control systems (rotation and zoom, pan not implemented)
│       ├── input/        # Keyboard and mouse input handling
│       │   └── mod.rs   # InputState resource (with scroll_delta), handle_keyboard_input, handle_mouse_input
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
- **CameraPlugin** (genesis-render): Camera control systems (free-flight, orbit rotation, zoom, mode switching with smooth interpolation) with CameraState resource
- **GenesisUiPlugin** (genesis-ui): UI system with bevy_egui integration, overlay, and timeline controls (includes TimelinePlugin internally)
- **ConfigResource** (main.rs): Wrapper for Config as a Bevy Resource (NOTE: Config::load() IS implemented - reads from genesis.toml with file path search logic)
- **ParticleConfig** (genesis-core): Resource for particle spawning configuration (correctly used directly with Resource derive in main.rs line 88)
- **CameraState** (genesis-render): Resource for tracking camera mode and target (initialized from CameraConfig)
- **OverlayState** (genesis-ui): Resource for overlay visibility (initialized from DisplayConfig: show_fps, show_particle_count)
  - Note: show_epoch_info field does NOT exist in OverlayState or DisplayConfig (not implemented yet)
- **CosmicTime** (genesis-ui): Resource for timeline state management with logarithmic slider mapping (auto-initialized by TimelinePlugin)
- **PlaybackState** (genesis-ui): Resource for playback control (auto-initialized by TimelinePlugin)
- **TimeAccumulator** (genesis-core): Resource for tracking cosmic years (auto-initialized by TimeIntegrationPlugin)
- **setup_camera**: Camera setup system that spawns 3D camera at orbit_distance looking at origin with OrbitController and CameraController components (correctly uses config.camera.orbit_distance which matches CameraConfig struct)

**Epoch Infrastructure Status**: The following epoch-related types are defined:
- SingularityEpoch marker struct - epoch marker for Singularity phase (defined in genesis-core/epoch/singularity.rs)

**The following epoch management infrastructure is NOT implemented and is planned for future phases**:
- EpochManager resource - NOT defined
- EpochManagerPlugin - NOT defined
- EpochPlugin trait - NOT defined
- update_epoch_transition system - NOT defined
- EpochChangeEvent - NOT defined
- SingularityEpoch does NOT implement EpochPlugin trait (trait doesn't exist)

## Core Architectural Decisions

### 1. Modular Crate Architecture
- **Rationale**: Separates concerns into core, render, physics, and UI domains
- **Benefit**: Clear dependency boundaries, easier testing, parallel development
- **genesis-core**: Pure simulation logic (epoch, physics, time), depends on Bevy for Resource trait
  - Exports: Config, ParticleConfig, CameraConfig, TimeConfig, WindowConfig, DisplayConfig, TimeIntegrationPlugin, SingularityEpoch
  - **NOTE**: CameraMode is exported from genesis-render, not genesis-core. EpochCameraConfig, EpochManager, EpochManagerPlugin, EpochPlugin, and EpochChangeEvent are NOT exported (not defined in codebase)
- **genesis-physics**: Physics simulation modules with data structures and planned computation kernels
  - Exports: InflatonPlugin, ScaleFactor, CosmicEpoch
  - NOTE: gravity, inflaton, perturbations, nucleosynthesis modules exist as placeholders (data structures not implemented) except inflaton which is fully implemented
- **genesis-render**: Rendering systems using Bevy ECS (camera, particle components)
  - Exports: CameraMode, CameraPlugin, CameraState, InputPlugin, ParticlePlugin
- **genesis-ui**: UI state resources using Bevy ECS (timeline, overlay)
  - Exports: GenesisUiPlugin, TimelinePlugin, CosmicTime, PlaybackState, OverlayState

### 2. Bevy ECS Pattern
- **Components**: `Particle` (rendering component with position: Vec3, velocity: Vec3, initial_position: Vec3, initial_velocity: Vec3, color: Color, size: f32)
  - Particles are spawned with Mesh3d, PointSpriteMaterialHandle, Transform, and Particle components
  - Camera components: `CameraController` (free-flight), `OrbitController` (orbit)
- **Resources**: Global state organized by crate:
   - genesis-core: TimeAccumulator, DisplayConfig, ScrubbingEvent (Event)
   - genesis-render: CameraState, InputState, PointMesh, ScrubbingState
   - genesis-ui: CosmicTime, OverlayState, PlaybackState
- **Systems**:
   - Core: initialize_time_accumulator (Startup), update_cosmic_time (Update)
   - Particle: init_point_mesh (Startup), spawn_particles (Startup), update_particles (basic outward expansion animation), update_particle_energy_colors (thermal gradient coloring), sync_particle_position (Update), update_scrubbing_state (Update), update_particles_for_scrubbing (Update), extract_particle_instances (ExtractSchedule), prepare_particle_instance_buffers (Render)
   - Camera: update_free_flight_camera (Update), update_orbit_camera (Update), toggle_camera_mode (Update), interpolate_camera (Update), handle_orbit_zoom (Update), handle_free_flight_zoom (Update)
   - Input: handle_keyboard_input (PreUpdate), handle_mouse_input (PreUpdate)
   - UI: update_overlay_ui (Update), timeline_panel_ui (PostUpdate), sync_time_resources (Update)
- **Plugins**:
   - TimeIntegrationPlugin (implemented): Cosmic time accumulation with Bevy integration
   - InputPlugin (implemented): Keyboard and mouse input processing (including scroll wheel input)
   - CameraPlugin (implemented): Camera control systems for free-flight and orbit modes (rotation, zoom, mode switching with smooth interpolation)
     - Camera interpolation: Implemented via interpolate_camera() system with cubic ease-in-out easing
     - Orbit zoom: Implemented (handle_orbit_zoom system uses scroll wheel input, clamps distance [1.0, 200.0])
     - Free-flight zoom: Implemented (handle_free_flight_zoom system uses scroll wheel input, clamps distance [1.0, 200.0])
     - Orbit pan: **NOT implemented** (handle_orbit_pan system does NOT exist)
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
  - Per-instance particle size and color attributes: **Implemented** via storage buffer
    - Storage buffer synchronization system exists (`instance_buffer.rs` with extract_particle_instances and prepare_particle_instance_buffers)
    - These systems are registered in ParticlePlugin (ExtractSchedule and Render phases)
    - Shader uses storage buffer at @group(0)@binding(3) with @builtin(instance_index) for per-instance data access
    - Full synchronization from Particle component → GPU attributes is complete (extract system runs, shader reads from storage buffer)
  - Energy-based particle color mapping for thermal gradient: **Implemented** (white-hot core → red edges)
- **Documentation**: See [`genesis-render/src/particle/DESIGN.md`](genesis-render/src/particle/DESIGN.md) for detailed particle rendering design

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
  - `velocity: Vec3` - Velocity vector
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
- Custom Bevy material implementing `AsBindGroup` trait
- Uses a custom WGSL shader (`point_sprite.wgsl`) for vertex and fragment processing
- Uniform parameters:
  - `color: LinearRgba` - Base color for all particles
  - `base_size: f32` - Base size in pixels before distance attenuation
  - `attenuation_factor: f32` - Controls size attenuation with distance
- Uses additive blending (AlphaMode::Add) for a glowing effect
- Registered via `MaterialPlugin::<PointSpriteMaterial>::default()` in ParticlePlugin

**PointSpriteMaterialHandle**
- Component for attaching a point sprite material to particle entities
- Wraps `Handle<PointSpriteMaterial>` for Bevy asset system integration
- Allows particles to share a single material handle for efficient GPU instancing

**PointMesh**
- Resource containing a shared `Handle<Mesh>` for all particle entities
- Mesh topology: `PrimitiveTopology::PointList` with a single vertex at origin
- The `Transform` component on each entity provides actual position
- Custom vertex attributes:
  - `ATTRIBUTE_INSTANCE_SIZE`: Float32 at location(1) for per-instance particle size
  - `ATTRIBUTE_INSTANCE_COLOR`: Float32x4 at location(2) for per-instance particle color
- Initialized once at startup via `init_point_mesh` system
- Shared across all particles for efficient GPU instancing

### 3.3 Instance Buffer Data Synchronization

**Overview**
The `instance_buffer.rs` module implements a Storage Buffer approach for synchronizing per-instance particle attributes from CPU Particle components to GPU storage buffers. This enables efficient CPU-GPU data transfer for 10K-100K particles while maintaining Bevy 0.15's automatic GPU instancing benefits.

**Architecture**
```
Particle Components (Main World)
    ↓ [extract_particle_instances - ExtractSchedule]
ExtractedParticleInstances (Render World)
    ↓ [prepare_particle_instance_buffers - RenderSet::Prepare]
ParticleInstanceBuffer + ParticleInstanceBindGroup (GPU)
    ↓ [Vertex Shader]
Per-Instance Rendering
```

**Data Structures**
- **ParticleInstanceData**: GPU-compatible struct with size (f32) and color ([f32; 4]) fields, using #[repr(C)] layout (32 bytes total)
- **ExtractedParticleInstances**: Resource containing Vec<ParticleInstanceData> extracted from Main world's Particle components
- **ParticleInstanceBuffer**: GPU buffer wrapping Bevy StorageBuffer, dynamically resizable (power-of-two capacity)
- **ParticleInstanceBindGroup**: Bind group making the storage buffer accessible to shader at binding 3
- **ParticleInstanceBindGroupLayout**: Bind group layout defining storage buffer interface (FromWorld)

**Systems**
- **extract_particle_instances** (ExtractSchedule): Copies Particle component data from Main world to Render world, converting Color to linear RGBA and extracting size
- **prepare_particle_instance_buffers** (RenderSet::Prepare): Writes extracted data to GPU storage buffer, creates/updates bind group, handles dynamic buffer resizing

**Current Status**
- Extract and prepare systems are implemented and registered in ParticlePlugin
- Storage buffer infrastructure exists with proper GPU resource management
- Shader uses storage buffer at @group(0)@binding(3) with @builtin(instance_index) for per-instance data access
- Full CPU-GPU synchronization is complete (systems exist, shader integration implemented)

### 4. Cosmic Time System
- **Type**: f64 accumulator for precision over 13.8B years
- **Dual Time System**: The application uses two separate time resources:
  - **TimeAccumulator.years** (genesis-core): Tracks accumulated cosmic time in years, updated each frame via add_time() with acceleration factor
  - **CosmicTime.cosmic_time** (genesis-ui): Stores timeline position used by the slider UI, updated by timeline scrubbing
  - **Synchronization**: The sync_time_resources system synchronizes:
    - TimeAccumulator's paused state with PlaybackState.playing
    - PlaybackState.speed to TimeAccumulator.acceleration (direct pass-through)
  - **Timeline Scrubbing**: Timeline scrubbing updates CosmicTime.cosmic_time and syncs to TimeAccumulator.years via timeline_panel_ui system (line 180: `time_accumulator.years = cosmic_time.cosmic_time;`).
- **Acceleration**:
  - TimeAccumulator.acceleration handles the actual 1x-10¹²x scaling
  - TimeAccumulator provides pause() and resume() methods for playback control
  - PlaybackState.speed (f32, 1.0-1e12) controls time acceleration via speed control slider
- **UI Integration**:
  - CosmicTime resource provides logarithmic slider mapping via from_slider() and to_slider() methods
  - Timeline UI panel (timeline_panel_ui) renders play/pause button, timeline slider, and speed control (runs in PostUpdate schedule)
  - sync_time_resources system synchronizes TimeAccumulator with PlaybackState (runs in Update schedule)
- **Status**:
  - TimeAccumulator resource fully implemented with pause/resume/toggle/is_paused methods
  - TimeIntegrationPlugin integrates with Bevy's time system
  - Timeline UI controls fully implemented with logarithmic slider mapping
  - Time synchronization between UI playback controls and accumulator fully implemented
  - Speed-to-acceleration mapping: **Implemented** - PlaybackState.speed (1.0-1e12) maps to TimeAccumulator.acceleration (1.0-1e12) via direct pass-through (no logarithmic scaling)
  - Timeline scrubbing to TimeAccumulator synchronization: **Implemented** - slider changes update both CosmicTime resource and TimeAccumulator.years via timeline_panel_ui
- **Constants**:
  - `SECONDS_PER_YEAR`: Number of seconds in a cosmic year (365.25 days)
  - `SECONDS_PER_MINUTE`: Number of seconds in a minute
  - `SECONDS_PER_HOUR`: Number of seconds in an hour
  - `SECONDS_PER_DAY`: Number of seconds in a day
  - `MIN_YEARS`: Minimum representable cosmic time in years (~10⁻⁴⁰ years for Planck scale)
  - `INFLATION_START_YEARS`: Cosmic inflation epoch start time (~10⁻⁴⁴ years / 10⁻³⁶s)
  - `INFLATION_END_YEARS`: Cosmic inflation epoch end time (~10⁻³² years / 10⁻²⁴s)
  - `PLANCK_EPOCH_YEARS`: Planck epoch time (~10⁻³⁶ years / 10⁻²⁸s)
  - `YEARS_PER_SECOND`: Number of years in a second (1/SECONDS_PER_YEAR)
  - `YEARS_PER_MINUTE`: Number of years in a minute
- **Functions**:
  - `seconds_to_years(seconds: f64) -> f64`: Converts seconds to cosmic years
  - `minutes_to_years(minutes: f64) -> f64`: Converts minutes to cosmic years

### 5. Camera System Design
- **Camera Modes**: FreeFlight and Orbit enum variants defined (Orbit is default)
- **State Tracking**: CameraState resource with mode, target, and current_orbit_target fields
- **Components**:
  - CameraController: Free-flight camera with yaw, pitch, movement_speed, mouse_sensitivity, zoom_speed
  - OrbitController: Orbit camera with distance, yaw, pitch, rotation sensitivity
- **Dual-Controller Architecture**: Both OrbitController and CameraController components are always present on the camera entity. Mode switching (via toggle_camera_mode) affects which controller responds to input, not component attachment.
- **Configuration**:
  - CameraConfig in genesis-core has fields: initial_mode (String), orbit_distance (f64)
  - CameraMode enum exists in genesis-render::camera but CameraConfig uses String for initial_mode field
  - genesis.toml has fields: initial_mode (String), orbit_distance (f64)
- **Configuration Field Status**:
  - **initial_mode vs CameraMode enum**: CameraConfig.initial_mode is a String, and CameraMode enum exists. CameraState::from_config() in genesis-render/src/camera/mod.rs (line 117) correctly accesses `config.initial_mode` and converts it to CameraMode enum.
  - **All configuration fields match correctly** between genesis.toml and Config structs (ParticleConfig, CameraConfig, TimeConfig, WindowConfig, DisplayConfig)
- **Status**:
  - Camera setup (setup_camera system): Implemented - spawns 3D camera at orbit_distance looking at origin with OrbitController (distance: orbit_distance) and CameraController::default().
  - Camera movement controls: Implemented for both free-flight (update_free_flight_camera) and orbit (update_orbit_camera) modes
  - Camera mode switching: Implemented via toggle_camera_mode system (press 'O' key to toggle between FreeFlight and Orbit)
  - Orbit camera zoom: **Implemented** (handle_orbit_zoom system exists at camera/mod.rs:408-430) - scroll wheel zooms in/out with clamped distance [1.0, 200.0]
  - Free-flight camera zoom: **Implemented** (handle_free_flight_zoom system exists at camera/mod.rs:448-486) - scroll wheel zooms in/out with clamped distance [1.0, 200.0]
   - Orbit camera pan: **NOT implemented** (not required for Phase 1 PRD deliverables)
  - Camera interpolation: **Implemented** (interpolate_camera system exists at camera/mod.rs:642-686) - smooth cubic ease-in-out transitions during mode switches

### 6. Input System Architecture
- **InputState Resource**: Tracks keyboard direction vector, mouse delta, mouse button states (Left, Middle), and scroll wheel delta
- **Keyboard Handling**: WASD key inputs mapped to directional vectors
- **Mouse Handling**: Mouse button states tracked using HashMap<MouseButton, bool> (Left and Middle buttons); Tracks mouse motion delta; Tracks scroll wheel delta for orbit zoom and free-flight zoom
- **Status**: InputPlugin fully implemented with handle_keyboard_input and handle_mouse_input systems (run in PreUpdate schedule)

### 7. Epoch Plugin Architecture (Planned for Future Phases)

**Current Implementation**:
Only the following epoch-related types are defined in genesis-core/epoch/:
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

### 8. Physics Module Implementation Status

**Current Implementation**:
genesis-physics crate contains the following modules:

- **integrator/mod.rs** - Numerical integrators for ODE solving
  - **Implemented**: rk4_step() (4th-order Runge-Kutta step), rk4_integrate() (multi-step integration)
  - Generic functions supporting any state type with appropriate trait bounds
  - Well-documented with inline comments and examples

- **cosmology/mod.rs** - Cosmological physics data structures
  - **Implemented**: Curvature enum (Open, Flat, Closed), EnergyDensity struct (total, matter, radiation, dark_energy, inflaton), ScaleFactor struct (value, derivative, time, epoch), HubbleParameter struct (value, squared), CosmicEpoch enum (Planck, Inflation, QuarkGluonPlasma, Nucleosynthesis, Recombination, DarkAges, CosmicDawn, Structure), Cosmology struct (combines all parameters), CosmologyPlugin (Bevy plugin that registers all cosmology resources)
  - **Methods Implemented**: compute_hubble() (Friedmann equation), compute_scale_factor_derivative(), update_hubble(), update_scale_factor_derivative(), integrate_scale_factor_euler(), integrate_scale_factor_rk4() (RK4 integrator), integrate_scale_factor_inflation() (exponential expansion), update_scale_factor_by_epoch() (epoch-aware integration)
  - **Helper Functions**: compute_exponential_scale_factor() (a(t) = a₀e^(Ht)), years_to_gev_inv() (time unit conversion)
  - **Constants Module**: G (gravitational constant), C (speed of light), PLANCK_MASS, PLANCK_LENGTH, PLANCK_TIME, INFLATION_HUBBLE_GEV, GEV_TO_JOULES

- **inflaton/mod.rs** - Inflaton field dynamics for cosmic inflation
  - **Implemented**: Inflaton struct (phi, potential, potential_first_derivative, potential_second_derivative, epsilon, eta), InflatonPlugin (initializes Inflaton resource), quadratic potential V(φ) = ½m²φ², quadratic potential derivatives, slow-roll parameter calculations (ε = (1/2)(V'/V)², η = V''/V)
  - **Fully Documented**: Comprehensive inline comments and extensive unit tests

- **gravity/mod.rs** - Gravity simulation
  - **Status**: Placeholder module with no data structures or implementations

- **perturbations/mod.rs** - Density perturbations (Phase 2)
  - **Status**: Placeholder module with no data structures or implementations

- **nucleosynthesis/mod.rs** - Big Bang nucleosynthesis (Phase 3)
  - **Status**: Placeholder module with no data structures or implementations

**Gap Analysis**:
The cosmology module has comprehensive data structures and both Euler and RK4 integration methods implemented. The inflaton module is fully implemented with complete slow-roll parameter calculations. The other physics modules (gravity, perturbations, nucleosynthesis) are placeholders awaiting implementation.

### 9. Configuration Management
- **Format**: TOML for human-readable configuration
- **Status**: Configuration system implemented with file loading support
  - Config struct with WindowConfig, ParticleConfig, CameraConfig, TimeConfig, DisplayConfig fully defined
  - Default implementations provided for all config structs
  - ConfigResource wrapper for Bevy integration
  - Config::load() method implemented - reads from genesis.toml with file path search logic
  - Searches ./genesis.toml, ~/.config/genesis/config.toml, /etc/genesis/config.toml in order
- **Configuration Field Notes**:
  - **ParticleConfig**: Field names match correctly between genesis.toml and ParticleConfig struct (initial_count, max_count, base_size) - no mismatches
  - **CameraConfig**: genesis.toml has `initial_mode` and `orbit_distance` fields; CameraConfig struct has matching `initial_mode` (String) and `orbit_distance` (f64) fields - fields match correctly
  - **TimeConfig**: genesis.toml has `time_acceleration_min` and `time_acceleration_max` fields; TimeConfig struct has matching fields - all configuration fields match correctly
- **Note**: Configuration loading infrastructure is fully implemented. All struct fields with #[serde(default)] use default values when not present in genesis.toml

### 10. Testing Infrastructure
- Resource binding tests (`genesis-render/tests/resource_binding_tests.rs`): 1377 lines validating GPU resource setup
- Shader validation tests (`genesis-render/tests/shader_tests.rs`): 995 lines ensuring WGSL compatibility
- Unit tests for particle instance data (`instance_buffer.rs:298-320`): Validates particle instance data synchronization
- Unit tests for inflaton module (`inflaton/mod.rs:186-545`): Comprehensive tests for potential, derivatives, slow-roll parameters
- Unit tests for cosmology module (`cosmology/mod.rs:573-600`): Tests for curvature, energy density, and cosmological calculations

## Phase 1 Scope (Current Implementation)

### Goal
A running Bevy application with a 3D particle system, camera controls, and a time slider.

### Current Implementation Status

**NOTE**: Only Phase 1 deliverables are currently implemented. Features from Phase 2-7 (epoch management, physics-based particle simulation, nucleosynthesis, recombination, N-body gravity, SPH, star formation, cinematic mode, etc.) are NOT implemented and are planned for future phases.

### Phase 1 Implementation Status
**Implemented:**
- Core infrastructure setup (workspace, Cargo.toml)
- Bevy 0.15+ application scaffold with window and event loop
- Basic input handling (keyboard, mouse, scroll wheel) - InputPlugin with InputState (including scroll_delta), handle_keyboard_input, handle_mouse_input (runs in PreUpdate)
- Time integration system with f64 accumulator - TimeIntegrationPlugin, TimeAccumulator, update_cosmic_time system, pause/resume methods
- Particle rendering system with custom point sprite shader (PointSpriteMaterial, PointMesh) with GPU instancing
- Particle spawning system (spawn_particles) that creates test cluster at origin with configurable count
- Particle update system (update_particles) with basic outward expansion animation
- Energy-based particle color system (update_particle_energy_colors) with thermal gradient (white-hot core → red edges)
- Particle position synchronization system (sync_particle_position) to keep Particle.position and Transform.translation in sync
- Camera system with free-flight and orbit modes - CameraPlugin, CameraController, OrbitController, update_free_flight_camera, update_orbit_camera
- Camera mode switching via toggle_camera_mode (press 'O' key)
- Orbit camera zoom: **Implemented** (handle_orbit_zoom system uses scroll wheel input, clamps distance [1.0, 200.0])
  - Orbit camera pan: **NOT implemented** (not required for Phase 1 PRD deliverables)
- Free-flight camera zoom: **Implemented** (handle_free_flight_zoom system uses scroll wheel input, clamps distance [1.0, 200.0])
- Overlay UI with FPS and particle count panels - update_overlay_ui system
- Timeline UI with play/pause, logarithmic slider, and speed control - TimelinePlugin, CosmicTime resource with logarithmic mapping, timeline_panel_ui system (runs in PostUpdate)
- Time synchronization (sync_time_resources) between PlaybackState and TimeAccumulator including speed-to-acceleration mapping
- Per-instance particle data GPU synchronization via storage buffer - extract_particle_instances, prepare_particle_instance_buffers

**Partially Implemented (Infrastructure exists but not connected):**
- SingularityEpoch - defined as marker struct but does NOT implement EpochPlugin trait (trait doesn't exist)
- Dual time system (TimeAccumulator.years + CosmicTime.cosmic_time) - both exist, timeline scrubbing DOES sync to TimeAccumulator.years via timeline_panel_ui

**Pending (Phase 1 Completion Items):**
- Full physics-based particle updates with simulation-level particle sync (update_particles has basic outward expansion, full physics sync pending)
- Timeline scrubbing state restoration for reverse/replay with snapshot history (slider changes update both CosmicTime and TimeAccumulator.years, but reverse/replay capability pending)

**Deferred to Future Phases (Phase 2+):**
- Epoch management system implementation (EpochPlugin trait, EpochManager resource, EpochManagerPlugin, EpochChangeEvent, update_epoch_transition system, handle_epoch_change_transition system)
- SingularityEpoch implementation of EpochPlugin trait (trait doesn't exist - planned for Phase 2+)
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
genesis-physics (Bevy - for Resource trait)
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
- `ParticleConfig` (genesis-core) is used directly as Resource via `config.particle.clone()` in main.rs line 88
- `PointMesh` (genesis-render) is a shared resource for all particle entities
- **Physics Resources** (genesis-physics): Cosmology, ScaleFactor, HubbleParameter, EnergyDensity, Curvature, Inflaton (initialized by InflatonPlugin and CosmologyPlugin)
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
- Density perturbations and Zel'dovich approximation - NOTE: perturbations module exists as placeholder
- Nucleosynthesis reaction network solver - NOTE: nucleosynthesis module exists as placeholder
- CMB surface projection and volumetric fog
- N-body gravity simulation (direct-sum and Barnes-Hut) - NOTE: gravity module exists as placeholder
- SPH for baryonic gas dynamics
- Star formation and reionization visualization
- Cinematic mode with pre-authored camera paths

**Phase 1 Completion Items (Pending):**
- Timeline scrubbing to TimeAccumulator synchronization with reverse/replay capability (slider changes update both resources, but reverse/replay with snapshot history is pending)
- Full physics-based particle updates with simulation-level particle sync (update_particles has basic outward expansion, full physics sync pending)

## Architectural Decisions Log

### [2026-02-09] Point Sprite Shader Asset Path Resolution

**Issue:** Application crashes on startup because Bevy cannot find `assets/point_sprite.wgsl`. The shader file exists at `genesis-render/src/particle/point_sprite.wgsl` but the standard `assets/` folder was deleted during a cleanup.

**Decision:** Recreate the `assets/` directory at the project root and copy `genesis-render/src/particle/point_sprite.wgsl` to `assets/point_sprite.wgsl`.

**Rationale:**
1. Follows Bevy's standard convention: assets are expected in an `assets/` folder at the project root
2. `PointSpriteMaterial::fragment_shader()` returns `"point_sprite.wgsl"` as a relative path, which Bevy resolves relative to `assets/`
3. This approach is simpler than embedding the shader directly in code or configuring custom asset paths
4. The shader file in `genesis-render/src/particle/` can be kept as the source of truth
5. The shader in assets/ folder can be added to .gitignore (optional - if assets are considered build artifacts)

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

### [2026-02-10] GPU Integration Testing Strategy

**Issue:** Ten integration tests in `genesis-render/tests/resource_binding_tests.rs` are failing because they require GPU resources not available in headless CI/testing environments. Bevy 0.15's AssetServer and RenderPlugin require GPU initialization, which fails in environments without GPU access.

**Affected Tests:**
1. `test_materials_initialized_before_rendering` (line 399)
2. `test_camera_initialized_before_rendering` (line 443)
3. `test_system_ordering_point_mesh_before_spawn` (line 493)
4. `test_resources_created_at_startup` (line 549)
5. `test_resources_accessible_during_update` (line 599)
6. `test_resource_lifecycle_create_modify_access` (line 645)
7. `test_pipeline_cache_no_index_out_of_bounds` (line 682)
8. `test_particle_instance_bind_group_layout` (line 953)
9. `test_resource_reference_counting` (line 1000)
10. `test_complete_particle_rendering_setup` (line 1132)

**Decision:** Apply a dual-strategy approach for GPU-dependent integration tests:
1. **Use dummy handles where possible**: Modify tests to use `Handle::default()` instead of accessing `Assets<T>` to test the infrastructure that can be validated without GPU resources.
2. **Mark GPU-dependent tests as ignored**: For tests that truly require the asset system and GPU resources, mark them with `#[ignore]` and add explanatory comments.

**Rationale:**
1. Maintains test coverage for non-GPU-dependent infrastructure (resource creation, lifecycle, system ordering)
2. Preserves test code for future environments with GPU access (re-enable by removing `#[ignore]`)
3. Avoids blocking CI pipelines with tests that cannot run in headless environments
4. The passing test `test_point_mesh_initialized_before_particles_spawn` (line 364) demonstrates the working pattern using `Handle::default()`

**Implementation Pattern:**
```rust
// Example of working pattern (from test_point_mesh_initialized_before_particles_spawn)
let mesh_handle: Handle<Mesh> = Handle::default();  // Use dummy handle
// Test infrastructure without requiring actual GPU asset loading
```

**Future Considerations:**
- When GPU access is available in CI/testing environments (e.g., using wgpu-headless), the `#[ignore]` tests can be re-enabled
- Consider creating separate test suites for GPU-dependent tests with hardware requirements
- The dummy handle approach has limitations - tests requiring actual asset loading cannot be fully validated

**Impact:**
- Unblocks Sprint 1 test coverage goals for non-GPU infrastructure
- Allows CI pipelines to run integration tests for resource binding architecture
- GPU-dependent tests remain in codebase for future use when environment supports them

### [2026-02-10] Documentation Sync - ARCHITECTURE.md and Inline Comments

**Issue:** Several discrepancies exist between ARCHITECTURE.md and actual source code implementation. The documentation needs to be updated to reflect the current state of the codebase accurately.

**Decision:** Update ARCHITECTURE.md to accurately reflect the current implementation status across all modules, particularly:

1. **RK4 Integrator**: Update status from "NOT Implemented" to "Implemented" - The `integrate_scale_factor_rk4()` method exists in cosmology/mod.rs and is fully functional
2. **Inflaton Module**: Update from "placeholder with no data structures or implementations" to "Fully implemented with Inflaton struct (phi, potential, derivatives, slow-roll parameters), InflatonPlugin (initializes Inflaton resource), comprehensive unit tests"
3. **Integrator Module**: Add documentation for the integrator module which provides RK4 step and RK4 integrate functions for ODE solving
4. **CosmicEpoch Enum**: Add to exports list in cosmology module documentation
5. **Helper Functions**: Add documentation for `compute_exponential_scale_factor()` and `years_to_gev_inv()` helper functions
6. **Scale Factor Integration Methods**: Document `integrate_scale_factor_inflation()` and `update_scale_factor_by_epoch()` methods

**Rationale:**
1. Maintains accuracy of architectural documentation for developers
2. Reduces confusion between documented features and actual implementation
3. Ensures developers can correctly understand what infrastructure is available
4. Supports future development planning by accurately reflecting current capabilities

**Impact:** Updated ARCHITECTURE.md provides accurate architectural overview matching the current codebase state, enabling better developer onboarding and planning.

## Gap Analysis

### Overview
This document provides a comprehensive gap analysis comparing the Product Requirements Document (PRD.md) against the current implementation status documented in TODO.md, BACKLOG.md, and the actual codebase. The analysis identifies missing requirements, implementation gaps, and provides recommendations for sprint planning.

### Phase 1 (The Singularity) - Gap Analysis

#### PRD Phase 1 Deliverables (Lines 104-123)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Bevy application scaffold with window, input handling, and basic 3D scene | ✅ Implemented | DefaultPlugins, WindowPlugin configured correctly |
| 2 | Instanced particle renderer capable of displaying 100K-1M point sprites with position, color, and size | ⚠️ Partially Implemented | Currently 100,000 default particles. Infrastructure exists for GPU instancing, but scaling to 1M not complete |
| 3 | Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation | ✅ Implemented | CameraController and OrbitController components work correctly. Camera interpolation is implemented via interpolate_camera() system with cubic ease-in-out easing |
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
   - **Status:** Infrastructure exists (ATTRIBUTE_INSTANCE_SIZE, ATTRIBUTE_INSTANCE_COLOR in mesh) and IS synchronized via storage buffer
   - **Gap:** BACKLOG.md mentions per-instance data sync needs (lines 26-28), but specific implementation path documented in ARCHITECTURE.md
   - **Impact:** Individual particle colors and sizes DO affect rendering as they are updated in Particle component and synchronized via storage buffer to GPU

3. **Timeline Reverse/Replay Capability** (PRD line 121: "Scrub the timeline back and forth — the expansion reverses and replays")
   - **Status:** Partially documented in BACKLOG.md (lines 385-393)
   - **Gaps:** Missing critical implementation details:
     - No task for SimulationSnapshot resource structure implementation
     - No task for SnapshotHistory circular storage system
     - No task for state restoration from nearest snapshot logic
     - No task for edge case handling (scrubbing beyond snapshot history, unvisited time regions)
     - No task for TimelineScrubEvent event creation
   - **Impact:** Timeline scrubbing works forward but cannot replay backward smoothly without history

4. **Config::load() Implementation** (PRD line 113: "TOML configuration presets")
   - **Status:** Implemented
   - Config::load() method is fully implemented and reads from genesis.toml with file path search logic
   - Searches ./genesis.toml, ~/.config/genesis/config.toml, /etc/genesis/config.toml in order
   - Config struct with WindowConfig, ParticleConfig, CameraConfig, TimeConfig, DisplayConfig fully defined
   - All configuration fields match correctly between genesis.toml and Config structs
   - **Note:** CLI --config flag parsing with clap is not yet implemented (future enhancement)

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
   - **Status:** Scale factor structure EXISTS and is initialized by CosmologyPlugin
   - **Gap:** No separate resource module needed - ScaleFactor is part of cosmology module and fully integrated
   - **Impact:** Scale factor is available for future epoch-based updates via update_scale_factor_by_epoch() system

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
   - **Status:** Fully documented in ARCHITECTURE.md (lines 212-244) and fully implemented in source code
   - **Gap:** All systems are implemented and working:
     - Extract system (`extract_particle_instances`) transfers Particle component data to render world
     - Prepare system (`prepare_particle_instance_buffers`) creates GPU storage buffers
     - Bind group layout (`init_particle_instance_bind_group_layout`) initializes shader binding
     - Shader integration complete: storage buffer binding at @group(0)@binding(3) with instance index access
   - **Impact:** None - per-instance color and size synchronization is fully functional

9. **Easing Function Module** (Camera interpolation infrastructure)
   - **Status:** Documented in BACKLOG.md (lines 37-87)
   - **Gap:** Easing function creation tasks are detailed but inline implementation exists:
     - `ease_cubic()` function exists in camera/mod.rs (lines 176-182) for cubic ease-in-out interpolation
     - No separate module file needed as function is integrated into camera system
   - **Impact:** Camera interpolation uses cubic ease-in-out (hardcoded), other easing types not implemented

10. **Camera Interpolation on Epoch Change** (Mode switching interpolation implemented)
   - **Status:** Fully implemented - CameraState has interpolation fields and interpolate_camera() system is functional for mode switching
   - **Gap:** Interpolation is currently only triggered by toggle_camera_mode system ('O' key), not by epoch changes:
     - No task for creating camera tween trigger system that listens for EpochChangeEvent events
     - No task for extracting camera_config from target epoch
     - No task for calling CameraState::start_interpolation() on epoch transitions
     - No task for registering this system in main.rs after epoch_manager plugin
   - **Impact:** Camera interpolation works for manual mode switching but not connected to epoch transitions (EpochManagerPlugin not yet implemented)

**Configuration Field Mismatches Identified:**

11. **CameraConfig.initial_mode vs CameraMode enum**
   - **Status:** CameraConfig uses String (genesis.toml field: "initial_mode") but CameraMode enum exists
   - **Resolution:** String-to-enum conversion is implemented in CameraState::from_config() (genesis-render/src/camera/mod.rs line 117) correctly accesses `config.initial_mode` and converts it to CameraMode enum
   - **Impact:** None - configuration loading works correctly with proper string-to-enum conversion

12. **ParticleConfig field names**
   - **Status:** genesis.toml uses `initial_count`, `max_count`, `base_size` which match ParticleConfig struct
   - **Gap:** None - field names correctly match, Config::load() is implemented
   - **Impact:** None - configuration loading works correctly

13. **DisplayConfig.show_epoch_info vs OverlayState.show_epoch_info**
   - **Status:** NEITHER DisplayConfig NOR OverlayState struct have `show_epoch_info` field
   - **Gap:** Field is missing from both structs - not implemented yet
   - **Impact:** Epoch indicator display cannot be toggled via configuration

### Phase 2 (Inflation & Quantum Seeds) - Gap Analysis

#### PRD Phase 2 Deliverables (Lines 126-145)

| # | Requirement | Status | Implementation Note |
|---|-------------|--------|-------------------|
| 1 | Friedmann equation integrator for scale factor a(t) with slow-roll inflaton potential V(φ) | ⚠️ Partially Implemented | RK4 integrator IS implemented, slow-roll parameters computed, but density evolution functions not yet connected |
| 2 | Particle positions scale with a(t) — exponential expansion during inflation, decelerating after | ❌ Not in TODO or BACKLOG | Particle-scale coupling not implemented |
| 3 | 3D Gaussian random field generator with nearly scale-invariant power spectrum P(k) ∝ k^(n_s – 1) | ❌ Not in TODO or BACKLOG | Density perturbation infrastructure missing |
| 4 | Density perturbations mapped to particle displacement (Zel'dovich approximation) and color intensity | ❌ Not in TODO or BACKLOG | Zel'dovich implementation missing |
| 5 | Epoch indicator in UI showing current cosmic era and key parameters (temperature, scale factor, time) | ⚠️ Partially documented | BACKLOG.md has structure but no implementation tasks |
| 6 | Parameter panel (bevy_egui sidebar): adjust n_s, inflation duration, initial energy scale; changes restart simulation | ❌ Not in TODO or BACKLOG | Parameter panel infrastructure missing |
| 7 | Procedural QGP visualization: particles rendered as glowing plasma blobs with temperature-mapped color ramp | ⚠️ Partially documented | Temperature color ramp documented but not implemented |

#### Phase 2 Gaps Identified

**Missing from TODO and BACKLOG:**

14. **Density evolution functions** for Cosmology module
   - **Status:** Partially implemented - data structures exist but RK4 integrator not connected to density evolution
   - **Gap:** Complete physics infrastructure missing:
     - Density evolution functions (rho_m(a), rho_r(a), rho_lambda(a)) not implemented
     - FriedmannSolver struct with comprehensive state tracking not implemented
     - Higher accuracy integration for long-term evolution missing without density functions
   - **Impact:** Cannot simulate metric expansion with required accuracy for long timescales

15. **Inflaton Field and Potential**
   - **Status:** **FULLY IMPLEMENTED** - not a placeholder as stated in original ARCHITECTURE.md
   - **Gap:** None - Inflaton struct with all methods and InflatonPlugin exist
   - **Impact:** None - inflaton physics ready for integration into cosmology simulation

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
| 5 | Temperature readout drops through 3000 K (recombination) toward 2.725 K (present-day CMB) | ⚠️ Documented in BACKLOG.md | Line 571 mentioned but NOT implemented |
| 6 | Toggle overlay: show/hide CMB angular power spectrum C_ℓ with qualitative comparison to Planck data | ⚠️ Documented in BACKLOG.md | Lines 576-580 have structure but NOT implemented |

#### Phase 4 Gaps Identified

**Gap:** Phase 4 items are well-documented in BACKLOG.md (Sprint 4 section, lines 533-838 and duplicate lines 790-838) with extensive subtasks.

**Issue:** Sprint 4 (Phase 4) is duplicated in BACKLOG.md (appears twice with overlapping content), which could cause confusion during sprint planning.

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
| 3 | Expanded parameter panel: full cosmological parameter set (Ωₘ, ΩΛ, H₀, n_s, σ₈) with presets | ⚠️ Documented in BACKLOG.md | Lines 1207-1288 have structure but NOT implemented |
| 4 | Data overlay suite: temperature map, density field, velocity streamlines, dark matter distribution, power spectrum P(k) with observational comparison lines | ⚠️ Documented in BACKLOG.md | Lines 1263-1284 have structure but NOT implemented |
| 5 | PNG/EXR high-resolution frame capture with HDR support | ⚠️ Documented in BACKLOG.md | Lines 1290-1328 have extensive structure but NOT implemented |
| 6 | Benchmarking harness with automated performance regression tests | ⚠️ Documented in BACKLOG.md | Lines 1330-1365 have structure but NOT implemented |
| 7 | Cross-platform release builds: Linux, macOS (including Apple Silicon), Windows | ⚠️ Documented in BACKLOG.md | Lines 1368-1383 have structure but NOT implemented |
| 8 | User documentation, README, and tutorial walkthrough | ⚠️ Documented in BACKLOG.md | Lines 1369-1372 have structure but NOT implemented |
| 9 | Preset configuration sharing via TOML files | ⚠️ Documented in BACKLOG.md | Line 1372 mentioned but NOT implemented |

#### Phase 7 Gaps Identified

**Gap:** Phase 7 has the most extensive documentation in BACKLOG.md (Sprint 7 section, lines 1023-1387) with very detailed task breakdowns for GPU profiling, memory tracking, cinematic mode, narration, and benchmarking.

### Summary of Gap Findings

#### Critical Missing Requirements (Not in TODO or BACKLOG)

1. **Phase 1 Temperature Resource Module** - Required for epoch indicator display
2. **Phase 1 Scale Factor Resource Module** - Required for epoch indicator display
3. **Phase 1 Per-Instance Attribute Synchronization Implementation Path** - Infrastructure exists but implementation path now documented in ARCHITECTURE.md
4. **Phase 1 Config::load() Implementation** - Required for external configuration
5. **Phase 2 Density evolution functions** - Core physics for inflation integration
6. **Phase 2 Gaussian Random Field Generation** - Required for density perturbations
7. **Phase 2 Zel'dovich Approximation** - Required for quantum seeds
8. **Phase 2 Parameter Panel UI** - Required for interactive parameter adjustment
9. **Phase 1 Camera Interpolation Trigger on Epoch Change** - Infrastructure exists but no trigger system
10. **Phase 1 Epoch Marker Infrastructure** - SingularityEpoch exists but full epoch system not documented

#### Implementation Priority Recommendations

**Sprint 1 Completion (Phase 1):**
1. ~~Implement Config::load() method with external TOML file reading~~ (COMPLETED)
2. Implement Temperature resource module with epoch-based updates
3. Implement Scale Factor resource module with epoch-based updates (already exists in cosmology)
4. Create Epoch Indicator UI panel with era name, temperature, scale factor display
5. ~~Implement per-instance particle attribute synchronization system~~ (COMPLETED - storage buffer systems exist, shader integration pending)
   - Note: Full synchronization is now working - extract and prepare systems transfer data to GPU
6. Add simulation snapshot and history system for reverse/replay
7. ~~Resolve configuration field mismatches (CameraConfig.camera_mode string vs enum)~~ (RESOLVED - string-to-enum conversion in CameraState::from_config())
8. Add particle scaling tasks with performance monitoring for 100K-1M target
9. Document update_scale_factor_by_epoch() system (exists in cosmology/mod.rs)

**Sprint 2 (Phase 2 - Inflation):**
- BACKLOG.md has extensive documentation for Phase 2 tasks (lines 592-686)
- ~~genesis-physics crate exists with placeholder modules~~ (UPDATE: inflaton module is FULLY IMPLEMENTED)
- cosmology module has data structures and RK4 integrator - integrate density evolution functions for Friedmann solver
- Prioritize Gaussian random field generation should be implemented second
- Zel'dovich approximation can use density field from GRF generation

**Architecture Observations:**

1. **Strong Foundation:** Phase 1 infrastructure is well-implemented with Bevy ECS, rendering pipeline, camera controls, and UI framework
2. **Comprehensive Planning:** BACKLOG.md shows excellent sprint planning with granular subtasks for all phases
3. **Physics Module Status:** genesis-physics crate has:
   - integrator module with full RK4 implementation (NEW - not documented previously)
   - inflaton module with full Inflaton struct and InflatonPlugin implementation (UPDATE - was listed as placeholder in original documentation)
   - cosmology module with data structures, RK4/Euler integration, and epoch-aware update system
   - gravity, perturbations, nucleosynthesis modules exist as placeholders
4. **Missing Infrastructure Core:** Temperature and Scale Factor resources are foundational for all phases and should be implemented in Sprint 1
   - ScaleFactor is already implemented as part of cosmology module and initialized by CosmologyPlugin
   - Temperature module not yet implemented
5. **Configuration System:** Config structures are well-defined and external loading is fully implemented
6. **Per-Instance Rendering:** GPU attribute infrastructure exists and synchronization is complete (storage buffer systems exist, shader integration complete)
7. **Integration Module:** RK4 integrator with helper functions exists (NEW - not documented in original ARCHITECTURE.md)

**Issues to Address:**

1. ~~Documentation Inconsistency:~~ Sprint 4 (Phase 4) is duplicated in BACKLOG.md (lines 533-838 and 790-838) - ARCHITECTURE.md now correctly notes this issue in the Gap Analysis section
2. **Camera Interpolation Status:** Camera interpolation is fully implemented via interpolate_camera() system (update - documentation now reflects this correctly)
3. **Easing Function Module:** Easing functions are documented but inline implementation exists as ease_cubic() function in camera/mod.rs
4. **Inflaton Module Status:** Fully implemented with Inflaton struct, potential functions, slow-roll parameters, and InflatonPlugin (update - documentation now reflects this correctly)

### [2026-02-10] Timeline Scrubbing Strategy for Irreversible Processes

**Issue:** The PRD requires both real-time physics simulation with irreversible processes (nucleosynthesis, star formation, galaxy assembly) AND timeline scrubbing that allows users to "scrub the timeline back and forth — the expansion reverses and replays." These requirements are fundamentally in conflict.

**Decision:** Implement a **hybrid timeline scrubbing approach** that balances user experience, performance, and physical accuracy:

1. **Phase 1-2 (Kinematic Expansion): Full Scrubbing Support**
   - Particles follow simple kinematic motion (position = initial_position + velocity × time)
   - Time-symmetric physics allow true reverse scrubbing
   - No state restoration needed - recompute from time parameter

2. **Phase 3 (Nucleosynthesis): Limited Scrubbing Within Window**
   - Allow scrubbing within the nucleosynthesis window (3-20 minutes)
   - Store periodic snapshots during nucleosynthesis evolution
   - Cannot scrub before nucleosynthesis starts (irreversible barrier)
   - Use snapshots to restore state within the window

3. **Phase 4 (Recombination): Forward-Only After CMB Release**
   - Allow scrubbing through recombination phase (380,000 years)
   - Forward-only playback after CMB is released (irreversible barrier)
   - User sees "Cannot scrub past recombination" message when attempting reverse

4. **Phase 5-6 (Structure Formation): Forward-Only Playback**
   - Disable reverse scrubbing for structure and galaxy formation
   - Timeline shows full range but only forward playback works
   - Clear user indication: "Forward-only playback for irreversible phases"
   - Optional: Implement limited "rewind 30 seconds" for presentation purposes

5. **Snapshot System for Partial Reversibility**
   - Store lightweight snapshots at configurable intervals (e.g., every 1 million years)
   - Snapshot includes: particle positions, velocities, composition, halo structures
   - Memory budget: Target < 500 MB VRAM for snapshot storage
   - Allow scrubbing backward to nearest snapshot, then replay forward

6. **User Experience Design**
   - Visual indicators on timeline showing scrubbing limits
   - Gradient on timeline slider: green (fully scrubbing), yellow (limited), red (forward-only)
   - Informative message when user hits scrubbing limit
   - "Demo Mode" option that hides limitations for presentations

**Rationale:**
1. **Memory Constraints**: Full snapshot system for all phases would require 2-5 GB VRAM, exceeding <4 GB budget
2. **Scientific Accuracy**: Irreversible processes (nuclear reactions, star formation) cannot be truly reversed
3. **User Experience**: Full scrubbing for early phases provides satisfying exploration; forward-only for later phases is scientifically honest
4. **Performance**: Lightweight snapshots (periodic) enable limited reversibility without memory overhead
5. **Demo Use Case**: "Demo Mode" can hide limitations for presentations while maintaining accuracy

**Impact:**
- Timeline scrubbing implementation: Hybrid approach (kinematic recompute + periodic snapshots)
- Memory budget: <500 MB VRAM for snapshot storage
- User experience: Clear limitations with visual indicators
- Physics accuracy: Irreversible processes remain irreversible
- Demo capability: "Demo Mode" hides limitations for presentations

### [2026-02-10] Nucleosynthesis Validation Data Sources

**Issue:** Phase 3 specifies validation overlay showing observed primordial abundances, but does not specify which datasets, which elements, or how to handle the known "lithium problem."

**Decision:** Use the following validation data sources and approach:

**Reference Values (Primary: Planck 2018)**
- **Helium-4 (Y_p)**: 0.2471 ± 0.0002 (Planck 2018, CMB-derived)
- **Deuterium/Hydrogen (D/H)**: 2.58 × 10⁻⁵ ± 0.07 × 10⁻⁵ (Planck 2018)
- **Lithium-7 (⁷Li/H)**: 5.0 × 10⁻¹⁰ (Standard BBN prediction - NOT observed value)
- **Helium-3 (³He/H)**: 1.0 × 10⁻⁵ (Planck 2018)

**Validation Elements**
- Primordial stable elements only: **p (hydrogen), D (deuterium), ³He, ⁴He, ⁷Li**
- Exclude unstable intermediates: T (tritium), ⁷Be (beryllium-7)

**Tolerance Criteria**
- **Helium-4**: Within 5% (explicit PRD requirement)
- **Deuterium**: Within 10% (reasonable given observational uncertainties)
- **Lithium-7**: Within 20% OR reproduce ~3× discrepancy as educational feature
- **Helium-3**: Within 15% (observational data less precise)

**Lithium Problem Handling**
- Show **both predicted and observed values** in validation overlay
- Include a tooltip or info panel explaining the discrepancy
- Note that this is an open question in cosmology
- Educational value: demonstrates real scientific frontier

**Visual Presentation**
- **Primary Display**: Percentage difference with color coding (Green/Yellow/Red)
- **Secondary Display**: Multiple reference dataset lines (optional, user-toggleable)
- Real-time update as simulation progresses
- Toggle button to enable/disable validation overlay

**Alternative Datasets (Optional User-Selectable)**
- Planck 2018 (default, recommended)
- WMAP (legacy, for comparison)
- Direct observation values (metal-poor galaxies)
- User-provided custom values

**Rationale:**
1. **Scientific Authority**: Planck 2018 is most precise and recent CMB dataset
2. **Educational Value**: Showing lithium problem demonstrates real scientific frontier
3. **User Flexibility**: Alternative datasets support educational comparison
4. **Clear Success Criteria**: Explicit tolerances for each element
5. **Implementation Clarity**: Specific values avoid ambiguity

**Impact:**
- Phase 3 validation overlay implementation (Planck 2018 values as default)
- Success criteria for Phase 3 completion (tolerances defined)
- Educational value (lithium problem highlighted)
- User experience (clear visual indicators, optional dataset comparison)
- Configuration file format (validation parameters included for presets)

### [2026-02-10] Particle Identity and Persistence Across Phases

**Issue:** The PRD describes particles in multiple phases but is ambiguous whether these are the same particles persisting with identity across phases, or whether particles are regenerated for each phase.

**Decision:** Implement **Persistent Particles with Evolving Attributes** (Option A):

**Core Design**
- Particles are created once in Phase 1 (Singularity) and persist through all phases
- Each particle has extensible attributes that evolve across epochs
- No particle regeneration at phase boundaries
- Particle count changes via splitting (high-density regions) and merging (voids)

**Particle Attribute Structure**
- **Core Attributes (Phase 1)**: position, velocity, mass, energy
- **Composition Attributes (Phase 3)**: element_abundances {H, D, ³He, ⁴He, ⁷Li, ⁷Be}
- **Ionization Attributes (Phase 4)**: ionization_state, electron_fraction
- **Structure Attributes (Phase 5)**: is_dark_matter, halo_id, density_neighbors
- **Galaxy Attributes (Phase 6)**: is_star, galaxy_id, stellar_age

**Phase Transition Handling**
- Smooth transitions between phases (no particle recreation)
- Attributes are added/modified as phases progress
- Visual crossfade handles appearance changes
- No data loss during transitions

**Particle Count Evolution**
- **Phase 1-2**: Fixed count (100K - 1M particles)
- **Phase 3-4**: Same particles, composition changes
- **Phase 5**: Splitting in high-density regions, merging in voids (adaptive LOD)
- **Phase 6**: Star particles created from dense gas (new entities, not splitting)

**Timeline Scrubbing Integration**
- Particle persistence enables true timeline scrubbing
- Scrubbing works by recomputing particle states from time parameter
- Efficient memory usage (single particle pool)

**Rationale:**
1. **PRD Alignment**: Matches "continuous story" narrative and "physically grounded" simulation
2. **Scientific Accuracy**: Real cosmological evolution tracks the same particles
3. **Timeline Scrubbing**: Natural implementation without complex snapshot system
4. **User Experience**: Users watch "the same universe" evolve
5. **Performance**: Single particle pool avoids allocation/deallocation overhead

**Impact:**
- Particle data structure design: Extensible attribute system
- Epoch plugin architecture: Shared particle storage across all epoch plugins
- Timeline scrubbing implementation: Recompute from time parameter (efficient)
- Phase transition handling: Smooth attribute evolution, no recreation
- Memory management: Single shared particle pool with adaptive LOD for Phase 5+
