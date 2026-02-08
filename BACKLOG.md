# BACKLOG - Future Work

This document contains tasks for future sprints. Items here are not yet scheduled for implementation.

---

## Sprint 2 - Phase 2: Inflation & Quantum Seeds

### Physics Integration
- [ ] Implement Friedmann equation integrator for scale factor a(t)
- [ ] Add slow-roll inflaton potential V(φ) model
- [ ] Implement metric expansion during inflation (exponential)
- [ ] Implement decelerating expansion post-inflation
- [ ] Couple particle positions to scale factor a(t)

### Density Perturbations
- [ ] Implement 3D Gaussian random field generator
- [ ] Create nearly scale-invariant power spectrum P(k) ∝ k^(n_s – 1)
- [ ] Implement Zel'dovich approximation for density-to-displacement mapping
- [ ] Map density perturbations to particle displacement
- [ ] Map density perturbations to particle color intensity

### Visualization
- [ ] Implement procedural QGP visualization (glowing plasma blobs)
- [ ] Create temperature-mapped color ramp (blue-white → yellow → orange)
- [ ] Add epoch transition crossfade (singularity → QGP)
- [ ] Visualize density variations as brightness clumps

### UI & Configuration
- [ ] Update epoch indicator with inflation → QGP transition
- [ ] Add temperature readout (10²⁷ K through cooling)
- [ ] Create parameter panel (bevy_egui sidebar)
- [ ] Add n_s (spectral index) adjustment control
- [ ] Add inflation duration adjustment control
- [ ] Add initial energy scale adjustment control
- [ ] Implement simulation restart on parameter changes

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 3 - Phase 3: Nucleosynthesis & First Elements

### Physics - Nuclear Reaction Network
- [ ] Implement stiff ODE solver (implicit Rosenbrock method)
- [ ] Create 12-species nuclear reaction network (n, p, D, T, ³He, ⁴He, ⁷Li, ⁷Be, intermediates)
- [ ] Integrate NACRE II reaction rate compilation (temperature-dependent)
- [ ] Implement reaction rate interpolation tables

### Visualization - Composition
- [ ] Create live composition pie/bar chart overlay
- [ ] Add real-time element abundance tracking
- [ ] Implement particle color-coding by dominant composition (H=blue, He=yellow, Li=pink)
- [ ] Add epoch transition crossfade (QGP → element-colored particles)

### Configuration & Validation
- [ ] Create TOML configuration presets ("Standard Model", "High Baryon Density")
- [ ] Implement validation overlay with observed primordial abundances
- [ ] Add Y_p ≈ 0.245 comparison line for ⁴He
- [ ] Add toggle-able validation overlay

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 4 - Phase 4: Recombination & CMB

### Physics - Recombination
- [ ] Implement Saha equation solver
- [ ] Track ionization fraction x_e as function of temperature
- [ ] Implement photon mean free path calculation
- [ ] Model temperature evolution through recombination (3000 K → 2.725 K)

### Visualization - Fog & CMB
- [ ] Implement volumetric fog renderer
- [ ] Create fog density based on ionization fraction
- [ ] Implement fog clearing as x_e drops below threshold
- [ ] Create CMB surface projection (spherical shell at last-scattering surface)
- [ ] Generate CMB temperature anisotropies from Phase 2 density perturbations
- [ ] Implement camera transition (fog lifts, camera pulls back)

### UI & Analysis
- [ ] Add temperature readout (3000 K → 2.725 K)
- [ ] Create CMB angular power spectrum C_ℓ display
- [ ] Add qualitative Planck data comparison lines
- [ ] Implement toggle overlay for power spectrum

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
