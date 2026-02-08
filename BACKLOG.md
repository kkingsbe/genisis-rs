# BACKLOG - Future Work

This document contains tasks for future sprints. Items here are not yet scheduled for implementation.

---

## Sprint 1 - Phase 1: The Singularity
#### Window, Particle Engine & Time

### Core Visualization
- [ ] Implement procedural singularity visualization: spawn particles at origin with radial outward velocity vectors
- [ ] Implement energy-based color mapping for singularity visualization (map particle energy to white-hot → yellow → red gradient)
- [ ] Create cooling model tied to particle distance from origin or elapsed time
- [ ] Replace random particle spawning with procedural singularity generation

### Camera Controls
- [ ] Implement scroll wheel zoom controls for free-flight camera (move along forward vector)
- [ ] Implement scroll wheel zoom controls for orbit camera (adjust distance with clamping to min/max bounds)
- [ ] Add pan controls for both camera modes

### UI Implementation
- [ ] Create epoch indicator UI panel showing era name, temperature (Kelvin), scale factor a(t), and cosmic time
- [ ] Build FPS counter overlay system using bevy_egui (display in corner, update every frame using time diagnostics)
- [ ] Create particle count overlay system (query with<Particle> component, display count)
- [ ] Build time control UI (play/pause button, speed slider, reset button)
- [ ] Implement logarithmic timeline scrubber using bevy_egui Slider widget (span 0 to 13.8e9 years, map slider to cosmic time)

### Configuration System
- [ ] Create genesis-config module with Config struct defining Phase 1 parameters (particle_count, time_acceleration, camera_movement_speed, mouse_sensitivity)
- [ ] Implement TOML deserialization for Config struct using serde
- [ ] Create default Config constants for "Standard Model" preset (Planck 2018 best-fit cosmological parameters)
- [ ] Implement config file loader with path resolution (default: genesis.toml, fallback: embedded defaults)
- [ ] Implement clap argument parser for --config flag to override default config path
- [ ] Add ConfigResource and insert into main.rs via `.insert_resource(config)`

### Epoch Plugin System
- [ ] Implement epoch plugin registration system (actual plugin trait and registration, not just documentation)
- [ ] Define EpochPlugin trait with required methods: on_enter(), on_exit(), update_systems()
- [ ] Create SingularityEpoch plugin implementing the Singularity epoch from Planck Boundary to Inflation
- [ ] Implement EpochManager resource to track active epoch and handle transitions
- [ ] Add epoch transition crossfade system (handle epoch change events, trigger camera and visual transitions)

### Core System Integration
- [ ] Implement pause() method in TimeAccumulator resource (add `paused: bool` field and pause/play methods)
- [ ] Implement smooth camera interpolation system (camera_tween_resource with start/end positions, duration, easing function)

### Documentation
- [ ] Update ARCHITECTURE.md with final crate structure and responsibilities
- [ ] Document epoch plugin architecture design patterns (trait-based plugin system)
- [ ] Add inline documentation for genesis-core public APIs (time::TimeAccumulator, epoch::EpochPlugin trait, physics::Particle)
- [ ] Add inline documentation for genesis-render public APIs (camera::CameraMode/State, input::InputState, particle::Particle component)
- [ ] Add inline documentation for genesis-ui public APIs (overlay::OverlayState, timeline::PlaybackState)

### Build System
- [ ] Set up cross-platform build configuration for Linux, macOS, Windows
- [ ] Configure Cargo.toml for platform-specific dependencies (e.g., Apple Silicon support)

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 2 - Phase 2: Inflation & Quantum Seeds

### Physics Integration
- [ ] Implement Friedmann equation integrator for scale factor a(t) using RK4 solver
- [ ] Add slow-roll inflaton potential V(φ) model (quadratic potential: V(φ) = ½m²φ²)
- [ ] Implement metric expansion during inflation (exponential: a(t) = a₀e^(Ht))
- [ ] Implement decelerating expansion post-inflation (a(t) ∝ t^(2/3) for matter-dominated era)
- [ ] Couple particle positions to scale factor a(t) (multiply positions by current a(t))

### Density Perturbations
- [ ] Implement 3D Gaussian random field generator using Box-Muller transform
- [ ] Create power spectrum generator P(k) ∝ k^(n_s – 1) with configurable n_s parameter
- [ ] Implement Zel'dovich approximation for density-to-displacement mapping (displacement = ∇ψ where ∇²ψ = -δ)
- [ ] Map density perturbations to particle displacement (add displacement vectors to particle positions)
- [ ] Map density perturbations to particle color intensity (brighter = higher density)

### Visualization
- [ ] Implement procedural QGP visualization using glowing point sprite material with temperature-based color
- [ ] Create temperature-to-color ramp function (map temperature T to color: T > 10¹⁵K → blue-white, 10¹⁴K → white, 10¹³K → yellow, 10¹²K → orange)
- [ ] Implement epoch transition crossfade system (fade singularity → QGP using alpha blending over transition period)
- [ ] Visualize density variations as brightness clumps (scale particle size and brightness by local density)
- [ ] Add SingularityEpoch plugin implementing epoch transition from Planck Boundary to Inflation
- [ ] Add InflationEpoch plugin implementing epoch transition from Inflation to Quark-Gluon Plasma

### UI & Configuration
- [ ] Update epoch indicator display to show inflation → QGP transition (display epoch name, time range, current scale factor)
- [ ] Add temperature readout display (show temperature in Kelvin, update each frame based on cosmic time)
- [ ] Create parameter panel layout in bevy_egui sidebar (collapsible panel on right side of screen)
- [ ] Add n_s (spectral index) adjustment control (slider from 0.90 to 1.05, default 0.96)
- [ ] Add inflation duration adjustment control (slider from 10⁻³⁵s to 10⁻³⁰s in log scale)
- [ ] Add initial energy scale adjustment control (slider for V(φ)₀ parameter in GeV)
- [ ] Implement simulation restart function (reset TimeAccumulator, respawn particles, re-seed perturbations)
- [ ] Connect parameter panel controls to config update function (update config and trigger restart)

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 3 - Phase 3: Nucleosynthesis & First Elements

### Physics - Nuclear Reaction Network
- [ ] Implement stiff ODE solver using implicit Rosenbrock method for nuclear reaction network
- [ ] Define NuclearReaction struct with reactants, products, and reaction rate coefficient
- [ ] Create 12-species nuclear reaction network data structure (n, p, D, T, ³He, ⁴He, ⁷Li, ⁷Be, intermediates)
- [ ] Implement NACRE II reaction rate compilation lookup table (temperature-dependent rates)
- [ ] Implement reaction rate interpolation function (linear interpolation in log space for T and rates)
- [ ] Implement nuclear reaction network update system (solve ODE system dY_i/dt = Σ reactions)

### Visualization - Composition
- [ ] Create composition data structure tracking element abundances (Y_i for each species)
- [ ] Implement live composition bar chart overlay using bevy_egui (show H, He, Li abundances as percentages)
- [ ] Add real-time element abundance tracking system (update from nuclear reaction network each frame)
- [ ] Implement particle color-coding by dominant composition (map dominant element to color: H=blue, He=yellow, Li=pink)
- [ ] Add epoch transition crossfade system (fade QGP → element-colored particles over transition period)
- [ ] Add NucleosynthesisEpoch plugin implementing epoch transition from QGP to Nucleosynthesis

### Configuration & Validation
- [ ] Create ConfigPreset enum for "Standard Model" (Planck 2018 best-fit) and "High Baryon Density" presets
- [ ] Implement preset configuration loading (load from TOML or embedded defaults based on preset name)
- [ ] Create validation overlay panel using bevy_egui (show simulated vs observed abundances side-by-side)
- [ ] Add Y_p ≈ 0.245 comparison line for ⁴He (horizontal reference line in abundance chart)
- [ ] Implement toggleable validation overlay (show/hide validation comparison overlay)
- [ ] Add element abundance accuracy percentage display (show % deviation from observed values)

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 4 - Phase 4: Recombination & CMB

### Physics - Recombination
- [ ] Implement Saha equation solver for ionization fraction x_e(T) (solve for electron fraction given temperature)
- [ ] Create IonizationState resource tracking ionization fraction x_e, free electron density, and recombination progress
- [ ] Implement photon mean free path calculation (λ_mfp = 1 / (n_e σ_T) where n_e is free electron density and σ_T is Thomson cross-section)
- [ ] Model temperature evolution through recombination (T ∝ 1/a for adiabatic expansion, from 3000 K to 2.725 K)
- [ ] Add RecombinationEpoch plugin implementing epoch transition from Nucleosynthesis to Recombination

### Visualization - Fog & CMB
- [ ] Implement volumetric fog renderer using Bevy fog or custom shader (global fog with density varying by ionization fraction)
- [ ] Create fog density function mapping ionization fraction x_e to fog density (fog_density = x_e when x_e > 0.1, drops to 0 when x_e < 0.01)
- [ ] Implement fog clearing system (gradually reduce fog density as x_e drops below threshold)
- [ ] Create CMB surface projection mesh (spherical shell at last-scattering surface radius ~46 billion light years)
- [ ] Generate CMB temperature anisotropy texture (2D spherical harmonics from Phase 2 density perturbations)
- [ ] Implement camera transition system (pull camera back smoothly from center to view CMB sphere when recombination completes)
- [ ] Add CMB sphere material with temperature anisotropy mapping (color map from cold dark blue to hot bright red)

### UI & Analysis
- [ ] Update temperature readout to show 3000 K → 2.725 K range (display current temperature during recombination epoch)
- [ ] Create CMB angular power spectrum C_ℓ display chart (plot C_ℓ vs ℓ up to ℓ=1000)
- [ ] Add qualitative Planck data comparison lines (overlay observational data points on simulated power spectrum)
- [ ] Implement toggle overlay for power spectrum (show/hide CMB power spectrum chart in corner)
- [ ] Add last-scattering surface indicator (display "Last Scattering Surface at ~46 Gly" label pointing to CMB sphere)

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 5 - Phase 5: Dark Ages & Structure Formation

### Physics - N-Body Gravity
- [ ] Implement direct-sum N-body gravity (baseline, up to 500K particles)
- [ ] Create wgpu compute shader for GPU gravity calculations
- [ ] Implement Barnes-Hut octree (CPU build, GPU traversal)
- [ ] Optimize for 1M–10M particle scaling
- [ ] Add softening parameter to prevent numerical singularities

### Dark Matter & Structure
- [ ] Seed dark matter particles from Phase 2 perturbation field
- [ ] Implement baryonic particle coupling to dark matter
- [ ] Create adaptive level-of-detail system (particle splitting/merging)
- [ ] Implement Friends-of-Friends halo finder
- [ ] Add halo property calculation (mass, center-of-mass, radius)

### Visualization - Cosmic Web
- [ ] Render filaments as line geometry connecting halos
- [ ] Render voids as transparent dark regions
- [ ] Implement particle rendering with density-based coloring
- [ ] Add halo visualization (spheres or glow effects)

### Data Export
- [ ] Implement genesis-export crate
- [ ] Create HDF5 snapshot export (positions, velocities, masses, temperatures)
- [ ] Add CSV timeline summary export (scale factor, temperature, Hubble parameter)
- [ ] Implement export controls in UI

### Timeline Integration
- [ ] Add smooth transition from linear perturbation growth to nonlinear structure
- [ ] Update epoch indicator for Dark Ages era

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 6 - Phase 6: Cosmic Dawn & Galaxy Formation

### Physics - Baryonic Dynamics
- [ ] Implement Smoothed Particle Hydrodynamics (SPH)
- [ ] Create Wendland C4 kernel implementation
- [ ] Implement SPH density summation
- [ ] Add SPH force calculation (pressure, viscosity)
- [ ] Integrate Sutherland & Dopita 1993 radiative cooling functions
- [ ] Implement gas collapse through radiative cooling

### Star Formation
- [ ] Implement sub-grid star formation (Kennicutt-Schmidt relation)
- [ ] Create dense gas → star particle conversion
- [ ] Implement Pop III star formation in early halos
- [ ] Add first light sources as bright point lights

### Reionization
- [ ] Implement ionization front expansion (signed-distance-field bubbles)
- [ ] Create bubbles around star-forming halos
- [ ] Implement bubble overlap and merging
- [ ] Model neutral gas consumption

### Visualization - Galaxies
- [ ] Create galaxy billboard sprites
- [ ] Implement halo mass threshold for galaxy rendering
- [ ] Generate composite galaxy sprites based on merger history
- [ ] Add ionization bubble visualization (translucent spheres)

### Audio
- [ ] Implement genesis-audio crate
- [ ] Create procedural ambient audio system
- [ ] Generate deep bass drones for dark ages
- [ ] Add rising harmonic tones as stars ignite
- [ ] Implement full cosmic soundscape mixing

### Data Export
- [ ] Add VTK mesh export (density field, velocity field)
- [ ] Implement regular grid generation for field exports

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 7 - Phase 7: Polish, Cinematic Mode & Release

### Performance Optimization
- [ ] Implement GPU shader profiling tools
- [ ] Add memory budget enforcement
- [ ] Tune particle LOD for 60 FPS / 1M particles on GTX 1660
- [ ] Optimize Bevy scheduling and system ordering
- [ ] Add performance metrics overlay

### Cinematic Mode
- [ ] Implement pre-authored camera path system
- [ ] Add keyframe system with easing curves
- [ ] Create epoch narration text overlays
- [ ] Implement automatic camera transitions for demo
- [ ] Add single-button "Play Full Story" mode

### Expanded UI
- [ ] Add full cosmological parameter panel (Ωₘ, ΩΛ, H₀, n_s, σ₈)
- [ ] Create preset configurations (Standard Model, Einstein-de Sitter, De Sitter, Open Universe)
- [ ] Implement data overlay suite:
  - [ ] Temperature map
  - [ ] Density field
  - [ ] Velocity streamlines
  - [ ] Dark matter distribution
  - [ ] Power spectrum P(k) with observational comparisons

### Capture & Export
- [ ] Implement PNG high-resolution frame capture
- [ ] Add EXR HDR frame capture
- [ ] Create frame-by-frame export controls

### Benchmarking
- [ ] Implement genesis-bench crate
- [ ] Create automated performance regression tests
- [ ] Add benchmark results export
- [ ] Set up CI integration for performance tests

### Release & Documentation
- [ ] Create cross-platform release builds (Linux, macOS including Apple Silicon, Windows)
- [ ] Write comprehensive user documentation
- [ ] Create README with getting-started guide
- [ ] Write tutorial walkthrough for key features
- [ ] Implement preset configuration sharing via TOML files
- [ ] Add version information and changelog

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Future Enhancements (Post-Release)

### Additional Features
- [ ] Networked multiplayer mode
- [ ] VR support for immersive exploration
- [ ] User-defined custom epoch plugins
- [ ] Real-time collaboration features
- [ ] Cloud-based simulation state sharing

### Physics Extensions
- [ ] Full general-relativistic field equation solving
- [ ] Quantum chromodynamics simulation
- [ ] Sub-parsec stellar evolution resolution
- [ ] Research-grade precision mode

### Platform Support
- [ ] Mobile platforms (iOS, Android)
- [ ] WebAssembly deployment
- [ ] Console platforms (PlayStation, Xbox)
