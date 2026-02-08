# TODO - Current Sprint (Phase 1: The Singularity)

**Sprint Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider.

---

## Sprint 1 - Phase 1: The Singularity

### Core Infrastructure
- [ ] Set up Rust project workspace with Cargo.toml for genesis-core, genesis-render, genesis-ui crates
- [ ] Initialize Bevy 0.15+ application scaffold with window creation and event loop
- [ ] Implement basic input handling system (keyboard, mouse)
- [ ] Create epoch manager plugin architecture with registration system
- [ ] Set up time integration system with f64 accumulator

### Particle Rendering
- [ ] Implement instanced particle renderer using Bevy PBR materials
- [ ] Create particle component with position, color, size attributes
- [ ] Implement particle spawner system for 100K-1M particles
- [ ] Add GPU instancing support for efficient rendering
- [ ] Implement point sprite rendering with size attenuation

### Camera System
- [ ] Implement free-flight camera (WASD + mouse look) system
- [ ] Implement orbit camera (click-drag rotation) system
- [ ] Add smooth camera interpolation between positions
- [ ] Implement camera transition crossfade for epoch changes
- [ ] Add zoom and pan controls

### Time & Timeline
- [ ] Create cosmic time accumulator (f64) with adjustable acceleration
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
- [ ] Create TOML configuration system
- [ ] Implement config file loading and default values
- [ ] Add command-line argument parsing for config override

### Architecture & Documentation
- [ ] Write ARCHITECTURE.md with crate structure and responsibilities
- [ ] Document epoch plugin architecture
- [ ] Add inline code documentation for all public APIs

### Sprint QA
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.
