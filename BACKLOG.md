# BACKLOG - Future Work

This document contains tasks for future sprints. Items here are not yet scheduled for implementation.

---

## Sprint 1 - Phase 1: The Singularity
#### Window, Particle Engine & Time

### Core Visualization
- [ ] ~~Implement procedural singularity visualization: spawn particles at origin with radial outward velocity vectors~~ (COMPLETED: See genesis-render/src/particle/mod.rs spawn_particles())
- [ ] ~~Replace random particle spawning in spawn_particles() with procedural singularity generation~~ (COMPLETED: Already implemented with deterministic pseudo-random distribution)
- [ ] Scale particle system from 1000 to 100K-1M particles
  - [ ] Implement adaptive particle spawning system that scales based on config.particle.initial_count
  - [ ] Add performance monitoring to ensure target FPS with increasing particle counts
  - [ ] Optimize spawn_particles() to handle 100K+ entities efficiently (use batch spawning)
  - [ ] Implement particle LOD (Level of Detail) system to reduce rendering load for distant particles
  - [ ] Add GPU memory management for large particle systems (buffer reuse, streaming)
- [ ] Add Energy component to Particle entities to track individual particle energy values (create separate genesis_core::physics::Energy component)
- [ ] Create energy update system that decreases particle energy as they expand outward (E = E₀ * exp(-d/λ) where λ is decay constant)
- [ ] ~~Calculate particle energy based on distance from origin~~ (COMPLETED: See update_particle_energy_colors())
- [ ] ~~Implement energy-based color mapping for singularity visualization~~ (COMPLETED: See energy_to_color() function)
- [ ] Create cooling model tied to particle distance from origin or elapsed time (T ∝ 1/r for adiabatic expansion, track Temperature resource)

### Camera Controls
- [ ] Implement scroll wheel zoom controls for free-flight camera (move along forward vector with adjustable zoom speed)
  - [ ] Add scroll wheel event handling to free-flight camera system (genesis-render/src/camera/mod.rs update_free_flight_camera)
  - [ ] Implement zoom speed parameter in CameraController (zoom_speed: f32)
  - [ ] Apply scroll delta to move camera along forward vector (translation += forward * scroll_delta * zoom_speed)
- [ ] ~~Implement scroll wheel zoom controls for orbit camera (adjust distance with clamping to min/max bounds: min_distance=5.0, max_distance=200.0)~~ (COMPLETED: See handle_orbit_zoom() in genesis-render/src/camera/mod.rs)
- [ ] Add pan controls for free-flight camera (use WASD + Shift keys or middle mouse button drag for lateral movement)
  - [ ] Add middle mouse button drag detection to InputState
  - [ ] Implement pan system for free-flight camera that moves camera laterally based on mouse drag
  - [ ] Add Shift key modifier detection for alternative pan mode
- [ ] ~~Add pan controls for orbit camera (use Shift + drag or middle mouse button to move target point)~~ (COMPLETED: See handle_orbit_pan() in genesis-render/src/camera/mod.rs)
- [ ] ~~Implement smooth camera interpolation system (camera tween resource with start/end positions, duration, easing function)~~ (COMPLETED: See CameraState interpolation infrastructure in genesis-render/src/camera/mod.rs)
- [ ] Define easing functions (Linear, EaseInQuad, EaseOutQuad, EaseInOutCubic) for camera transitions
  - [ ] Create easing function module in genesis-render/src/camera/easing.rs
  - [ ] Implement Linear easing: f(t) = t
  - [ ] Implement EaseInQuad easing: f(t) = t²
  - [ ] Implement EaseOutQuad easing: f(t) = t * (2 - t)
  - [ ] Implement EaseInOutCubic easing: f(t) = t < 0.5 ? 4t³ : 1 - (-2t + 2)³ / 2
  - [ ] Add EasingType enum to CameraState to select active easing function
  - [ ] Apply easing function in interpolate_camera() system
- [ ] ~~Create CameraTween resource tracking active tween~~ (REPLACED BY: CameraState already tracks interpolation state with start_pos, end_pos, interpolation_speed, interpolation_progress)
- [ ] ~~Implement camera tween update system that interpolates camera position over time~~ (COMPLETED: See interpolate_camera() in genesis-render/src/camera/mod.rs)
- [ ] Add camera tween trigger system that initiates interpolation when epoch changes
  - [ ] Create system that listens for EpochChangeEvent events
  - [ ] Extract camera_config from target epoch (target_position, target_rotation, fade_duration)
  - [ ] Call CameraState::start_interpolation_to_target() with epoch camera config
  - [ ] Register this system in main.rs after epoch_manager plugin

### UI Implementation
- [ ] Create epoch indicator UI panel showing era name, temperature (Kelvin), scale factor a(t), and cosmic time
  - [ ] Add Temperature resource to genesis-core tracking current cosmic temperature (initial: ~10²⁷ K at Planck boundary)
  - [ ] Add ScaleFactor resource to genesis-core tracking metric expansion a(t) (initial: a=1 at Planck boundary)
  - [ ] Update epoch indicator UI (genesis-ui/src/overlay/mod.rs) to display temperature and scale factor
  - [ ] Implement temperature evolution model for Singularity epoch (T ∝ 1/a for adiabatic expansion)
  - [ ] Connect epoch data to UI display: query current epoch's temperature and scale factor values
- [ ] ~~Build FPS counter overlay system using bevy_egui~~ (COMPLETED: See update_overlay_ui() in genesis-ui/src/overlay/mod.rs)
- [ ] ~~Create particle count overlay system~~ (COMPLETED: See update_overlay_ui() in genesis-ui/src/overlay/mod.rs)
- [ ] ~~Build time control UI (play/pause button, speed slider, reset button)~~ (COMPLETED: See timeline_panel_ui() in genesis-ui/src/timeline/mod.rs)
- [ ] ~~Implement logarithmic timeline scrubber using bevy_egui Slider widget~~ (COMPLETED: See timeline_panel_ui() in genesis-ui/src/timeline/mod.rs)
- [ ] ~~Update main.rs to initialize PlaybackState resource~~ (COMPLETED: TimelinePlugin already inserts PlaybackState)
- [ ] Implement timeline scrubbing sync with particle simulation (enable reverse/replay, save/restore particle states)
  - [ ] Create SimulationSnapshot resource tracking particle states at key timeline positions
  - [ ] Implement state capture system that saves particle positions, velocities, energies at current time
  - [ ] Add snapshot history buffer (store last N snapshots at fixed time intervals)
  - [ ] Implement state restoration system that restores particles from nearest snapshot when scrubbing
  - [ ] Add reverse playback mode (when playing and scrubbing backward, decrease cosmic time)
  - [ ] Connect timeline slider changes to state restoration (on scrub, restore particle state)
  - [ ] Handle edge cases: scrubbing beyond snapshot history, scrubbing to unvisited time regions

### Configuration System
- [ ] ~~Create genesis-config module with Config struct~~ (COMPLETED: See genesis-core/src/config.rs)
- [ ] ~~Add serde dependencies to genesis-core/Cargo.toml~~ (COMPLETED: serde already present in Cargo.toml)
- [ ] ~~Implement TOML deserialization for Config struct~~ (COMPLETED: See Config::load_from_file() in genesis-core/src/config.rs)
- [ ] ~~Create default Config constants~~ (COMPLETED: See Default impl for Config struct)
- [ ] ~~Implement config file loader with path resolution~~ (COMPLETED: See Config::load_from_path() in genesis-core/src/config.rs)
- [ ] ~~Implement clap argument parser for --config flag~~ (COMPLETED: See CliArgs and Config::load() in genesis-core/src/config.rs)
- [ ] ~~Add ConfigResource and insert into main.rs~~ (COMPLETED: See ConfigResource wrapper in src/main.rs)
- [ ] Update existing systems to read from ConfigResource instead of hardcoded values
  - [ ] Refactor spawn_particles() to use config.particle.initial_count instead of constant PARTICLE_COUNT=1000
  - [ ] Refactor ParticlePlugin to read base_size from config.particle.base_size
  - [ ] Refactor CameraController to read movement_speed from config (add to CameraConfig)
  - [ ] Refactor CameraController to read mouse_sensitivity from config (add to CameraConfig)
  - [ ] Refactor time acceleration to use config.time.initial_time_acceleration in TimeAccumulator

### Epoch Plugin System
- [ ] ~~Implement epoch plugin registration system~~ (COMPLETED: See EpochManager and EpochPlugin trait in genesis-core/src/epoch/mod.rs)
- [ ] ~~Define EpochPlugin trait~~ (COMPLETED: See EpochPlugin trait with name(), start_year(), end_year(), build(), camera_config() methods)
- [ ] ~~Create SingularityEpoch plugin~~ (COMPLETED: See genesis-core/src/epoch/singularity.rs)
- [ ] ~~Implement EpochManager resource~~ (COMPLETED: See EpochManager in genesis-core/src/epoch/mod.rs)
- [ ] Add epoch transition crossfade system (handle epoch change events, trigger camera and visual transitions)
  - [ ] Implement visual crossfade system for epoch transitions (alpha blend between epoch render layers)
  - [ ] Create camera fade effect during epoch transitions (camera fade to black on exit, fade in on enter)
  - [ ] Implement parameter interpolation during transitions (smooth temperature, scale factor changes)
  - [ ] Add epoch transition event handling in GenesisUiPlugin to update UI
- [ ] ~~Register epoch plugins in main.rs~~ (COMPLETED: See SingularityEpochPlugin in src/main.rs)
- [ ] Implement future epoch plugins (InflationEpoch, QGPEpoch, NucleosynthesisEpoch, RecombinationEpoch, DarkAgesEpoch, CosmicDawnEpoch)
  - [ ] Create InflationEpoch plugin in genesis-core/src/epoch/inflation.rs (10⁻³²s to 10⁻⁶s)
  - [ ] Create QGPEpoch plugin in genesis-core/src/epoch/qgp.rs (10⁻⁶s to 3 min)
  - [ ] Create NucleosynthesisEpoch plugin in genesis-core/src/epoch/nucleosynthesis.rs (3 min to 20 min)
  - [ ] Create RecombinationEpoch plugin in genesis-core/src/epoch/recombination.rs (~380,000 yr)
  - [ ] Create DarkAgesEpoch plugin in genesis-core/src/epoch/dark_ages.rs (380 Kyr to 100 Myr)
  - [ ] Create CosmicDawnEpoch plugin in genesis-core/src/epoch/cosmic_dawn.rs (100 Myr to 1 Gyr)
  - [ ] Each epoch plugin must implement build() method to register epoch-specific systems
  - [ ] Each epoch plugin must define camera_config() with optimal camera settings
  - [ ] Register all epoch plugins in main.rs using EpochManager registration pattern

### Core System Integration
- [ ] ~~Implement pause() method in TimeAccumulator resource~~ (COMPLETED: See TimeAccumulator::pause() and resume() in genesis-core/src/time/mod.rs)
- [ ] ~~Implement smooth camera interpolation system~~ (COMPLETED: See CameraState interpolation infrastructure in genesis-render/src/camera/mod.rs)

### Documentation
- [ ] Update ARCHITECTURE.md with final crate structure and responsibilities (document genesis-core, genesis-render, genesis-ui responsibilities)
- [ ] Document epoch plugin architecture design patterns (trait-based plugin system, EpochManager registration, epoch transitions)
- [ ] Add inline documentation for genesis-core public APIs (time::TimeAccumulator, epoch::EpochPlugin trait, physics::Particle)
- [ ] Add inline documentation for genesis-render public APIs (camera::CameraMode/State, input::InputState, particle::Particle component)
- [ ] Add inline documentation for genesis-ui public APIs (overlay::OverlayState, timeline::PlaybackState)
- [ ] Document CosmicTime resource methods (from_slider, to_slider, set_time, get_time, reset)
- [ ] Document PointSpriteMaterial uniform parameters (color, base_size, attenuation_factor)
- [ ] Document OrbitController spherical coordinate system (distance, yaw, pitch, target)

### Build System
- [ ] Set up cross-platform build configuration for Linux, macOS, Windows
- [ ] Configure Cargo.toml for platform-specific dependencies (e.g., Apple Silicon support)

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

### Additional PRD Requirements (Identified During Gap Analysis)

#### Timeline Speed Integration
- [ ] Map PlaybackState.speed slider value to TimeAccumulator.acceleration
  - [ ] Implement logarithmic speed mapping: slider (0.1 to 10.0) → acceleration (1.0 to 1e12)
  - [ ] Formula: acceleration = 10^(slider_value * log10(1e12/1.0)) or similar logarithmic scale
  - [ ] Add system in sync_time_resources() to update acceleration when speed slider changes
  - [ ] Add visual feedback for current acceleration factor (display "10ⁿx" where n is exponent)
  - [ ] Document speed-to-acceleration mapping in timeline/mod.rs comments

#### Temperature & Scale Factor Tracking
- [ ] Add Temperature resource to genesis-core for tracking cosmic temperature
  - [ ] Create Temperature struct in genesis-core/src/temperature.rs with value (f64 in Kelvin) and update_systems()
  - [ ] Implement temperature evolution model for Singularity epoch: T(t) = T₀ * (a(t))^(-1) where a is scale factor
  - [ ] Define initial temperature at Planck boundary (T₀ ≈ 10²⁷ K)
  - [ ] Add system to update Temperature based on cosmic time acceleration
  - [ ] Register Temperature as Bevy resource via TemperaturePlugin
- [ ] Add ScaleFactor resource to genesis-core for tracking metric expansion
  - [ ] Create ScaleFactor struct in genesis-core/src/scale_factor.rs with value (f64) and update_systems()
  - [ ] Implement scale factor evolution: for Singularity epoch, a(t) = 1 (constant before inflation)
  - [ ] Add system to update ScaleFactor based on cosmic time acceleration
  - [ ] Register ScaleFactor as Bevy resource via ScaleFactorPlugin
- [ ] Connect Temperature and ScaleFactor to UI display
  - [ ] Query Temperature resource in epoch indicator UI
  - [ ] Query ScaleFactor resource in epoch indicator UI
  - [ ] Format temperature display with appropriate units (e.g., "10^27 K", "10^15 K")
  - [ ] Format scale factor display (e.g., "a = 1.000", "a = 10^23")

#### Particle State Synchronization with Transform
- [ ] Sync Particle component data with entity Transform components
  - [ ] Add system to copy Transform.translation to Particle.position each frame
  - [ ] Add system to update Transform based on particle physics (velocity integration)
  - [ ] Ensure particle physics update system writes to Transform, not just Particle component
  - [ ] This ensures the rendering system (which reads Transform) reflects particle motion

#### Timeline Pause State Synchronization
- [ ] Fix timeline pause/play button state synchronization
  - [ ] Currently sync_time_resources() only handles play/pause state
  - [ ] Add two-way binding: when PlaybackState.playing changes, update TimeAccumulator.paused
  - [ ] When timeline UI pause button is clicked, update both PlaybackState.playing and TimeAccumulator.paused
  - [ ] Ensure button reflects correct state (Play vs Pause) at all times

---

## Sprint 2 - Phase 2: Inflation & Quantum Seeds

### Physics Integration
- [ ] Implement Friedmann equation: H² = (8πG/3)ρ - k/a² (where H = ȧ/a)
- [ ] Implement RK4 solver for scale factor a(t) differential equation (ȧ = H*a)
- [ ] Add slow-roll inflaton potential V(φ) model (quadratic potential: V(φ) = ½m²φ² with m ~ 10¹⁶ GeV)
- [ ] Implement metric expansion during inflation (exponential: a(t) = a₀e^(Ht) where H ≈ 10¹⁴ GeV)
- [ ] Implement decelerating expansion post-inflation (a(t) ∝ t^(2/3) for matter-dominated era)
- [ ] Couple particle positions to scale factor a(t) (multiply positions by current a(t) in update system)
- [ ] Add ScaleFactor resource tracking current a(t) value, ȧ, and cosmic epoch (inflation vs matter-dominated)
- [ ] Implement temperature evolution model (T ∝ 1/a for adiabatic expansion, with T₀ ≈ 10²⁷ K at inflation start)
- [ ] Create InflationPhysics resource tracking inflaton field φ, potential V(φ), and slow-roll parameters (ε, η)

### Density Perturbations
- [ ] Implement Box-Muller transform for generating Gaussian random numbers (u1, u2 → normal distribution)
- [ ] Create 3D Gaussian random field generator on regular grid (apply Box-Muller transform to each grid point)
- [ ] Implement Fourier transform (FFT) to convert real-space density field to k-space
- [ ] Create power spectrum generator P(k) ∝ k^(n_s – 1) with configurable n_s parameter (default 0.96)
- [ ] Apply power spectrum to k-space field (multiply by sqrt(P(k)) and random phase)
- [ ] Implement inverse FFT to convert k-space back to real-space density perturbations
- [ ] Implement Zel'dovich approximation for density-to-displacement mapping (displacement = ∇ψ where ∇²ψ = -δ)
- [ ] Map density perturbations to particle displacement (add displacement vectors to particle positions on spawn)
- [ ] Map density perturbations to particle color intensity (brighter = higher density: intensity = 1.0 + α*δ where α controls contrast)
- [ ] Add DensityField resource tracking perturbation values δ, derivatives ∇δ, and power spectrum P(k)
- [ ] Create GaussianRandomField resource tracking grid size, seed, and generated field data

### Visualization
- [ ] Implement procedural QGP visualization using glowing point sprite material with temperature-based color
- [ ] Create temperature-to-color ramp function (map temperature T to color: T > 10¹⁵K → blue-white, 10¹⁴K → white, 10¹³K → yellow, 10¹²K → orange)
- [ ] Implement epoch transition crossfade system (fade singularity → QGP using alpha blending over transition period)
- [ ] Visualize density variations as brightness clumps (scale particle size and brightness by local density)
- [ ] Add SingularityEpoch plugin implementing epoch transition from Planck Boundary to Inflation
- [ ] Add InflationEpoch plugin implementing epoch transition from Inflation to Quark-Gluon Plasma
- [ ] Add QGPEpoch plugin implementing the Quark-Gluon Plasma epoch with temperature-dependent rendering

### UI & Configuration
- [ ] Update epoch indicator display to show inflation → QGP transition (display epoch name, time range, current scale factor)
- [ ] Add temperature readout display (show temperature in Kelvin, update each frame based on cosmic time)
- [ ] Create parameter panel layout in bevy_egui sidebar (collapsible panel on right side of screen)
- [ ] Add n_s (spectral index) adjustment control (slider from 0.90 to 1.05, default 0.96)
- [ ] Add inflation duration adjustment control (slider from 10⁻³⁵s to 10⁻³⁰s in log scale)
- [ ] Add initial energy scale adjustment control (slider for V(φ)₀ parameter in GeV)
- [ ] Implement simulation restart function (reset TimeAccumulator, respawn particles, re-seed perturbations)
- [ ] Connect parameter panel controls to config update function (update config and trigger restart)
- [ ] Update Config struct to include Phase 2 parameters (n_s, inflation_duration, initial_energy_scale)
- [ ] Create "Standard Model" preset with Phase 2 cosmological parameters

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 3 - Phase 3: Nucleosynthesis & First Elements

### Physics - Nuclear Reaction Network
- [ ] Define NuclearReaction struct with reactants (Vec<Element>), products (Vec<Element>), and reaction rate coefficient function k(T)
- [ ] Define Element enum for nuclear species (Neutron, Proton, Deuterium, Tritium, Helium3, Helium4, Lithium7, Beryllium7)
- [ ] Create 12-species nuclear reaction network data structure with ~50 reactions from BBN network
- [ ] Implement NACRE II reaction rate compilation lookup table (temperature-dependent rates λ(T) in log-log space)
- [ ] Implement reaction rate interpolation function (linear interpolation in log space for T and λ: log(λ) = Lerp(log(T1), log(T2), log(λ1), log(λ2)))
- [ ] Implement stiff ODE solver using implicit Rosenbrock method (2nd order with adaptive step size)
- [ ] Define Jacobian matrix for nuclear reaction network (∂f_i/∂Y_j where f_i = dY_i/dt)
- [ ] Implement nuclear reaction network update system (solve ODE system dY_i/dt = Σ (production - destruction) each frame)
- [ ] Add NuclearComposition resource tracking element abundances Y_i for each species (mass fractions)
- [ ] Create NucleosynthesisEpoch plugin implementing the Nucleosynthesis epoch (3 min - 20 min)
- [ ] Add reaction rate validation against NACRE II reference values at T = 10⁹ K

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
- [ ] Implement Thomson cross-section constant (σ_T ≈ 6.65×10⁻²⁹ m²)
- [ ] Create CMB resource tracking temperature anisotropies and power spectrum

### Visualization - Fog & CMB
- [ ] Implement volumetric fog renderer using Bevy fog or custom shader (global fog with density varying by ionization fraction)
- [ ] Create fog density function mapping ionization fraction x_e to fog density (fog_density = x_e when x_e > 0.1, drops to 0 when x_e < 0.01)
- [ ] Implement fog clearing system (gradually reduce fog density as x_e drops below threshold)
- [ ] Create CMB surface projection mesh (spherical shell at last-scattering surface radius ~46 billion light years)
- [ ] Generate CMB temperature anisotropy texture (2D spherical harmonics from Phase 2 density perturbations)
- [ ] Implement camera transition system (pull camera back smoothly from center to view CMB sphere when recombination completes)
- [ ] Add CMB sphere material with temperature anisotropy mapping (color map from cold dark blue to hot bright red)
- [ ] Create LastScatteringSurface resource tracking CMB sphere parameters (radius, center position)

### UI & Analysis
- [ ] Update temperature readout to show 3000 K → 2.725 K range (display current temperature during recombination epoch)
- [ ] Create CMB angular power spectrum C_ℓ display chart (plot C_ℓ vs ℓ up to ℓ=1000)
- [ ] Add qualitative Planck data comparison lines (overlay observational data points on simulated power spectrum)
- [ ] Implement toggle overlay for power spectrum (show/hide CMB power spectrum chart in corner)
- [ ] Add last-scattering surface indicator (display "Last Scattering Surface at ~46 Gly" label pointing to CMB sphere)
- [ ] Add CMB analysis panel with temperature readout and recombination progress

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 5 - Phase 5: Dark Ages & Structure Formation

### Physics - N-Body Gravity
- [ ] Define gravitational constant G = 6.674×10⁻¹¹ m³/(kg·s²) for simulation units
- [ ] Implement direct-sum N-body gravity force calculation: F_i = G Σ (m_j * m_i / r_ij² * r_ij) for all j ≠ i
- [ ] Create wgpu compute shader for GPU gravity calculations (bind particle positions/masses, compute forces in parallel)
- [ ] Implement Barnes-Hut octree data structure (Octree with node center, mass, and child pointers)
- [ ] Implement CPU octree build system (insert particles, compute node masses and centers)
- [ ] Implement GPU octree traversal for force calculation (use opening angle θ to decide when to use node mass vs individual particles)
- [ ] Optimize for 1M–10M particle scaling (use spatial hashing, shared memory, warp-level reduction)
- [ ] Add softening parameter ε to prevent numerical singularities (F ∝ 1/(r² + ε²) instead of 1/r²)
- [ ] Create GravitySystem resource tracking gravitational constants (G, ε), time step Δt, and particle mass
- [ ] Add DarkAgesEpoch plugin implementing the Dark Ages epoch (380 Kyr - 100 Myr)
- [ ] Implement velocity Verlet or leapfrog integrator for particle motion (better energy conservation than Euler)

### Dark Matter & Structure
- [ ] Seed dark matter particles from Phase 2 perturbation field
- [ ] Implement baryonic particle coupling to dark matter
- [ ] Create adaptive level-of-detail system (particle splitting/merging)
- [ ] Implement Friends-of-Friends halo finder
- [ ] Add halo property calculation (mass, center-of-mass, radius)
- [ ] Create HaloCatalog resource tracking discovered halos and their properties
- [ ] Implement CosmicWeb resource tracking filament and void detection

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
- [ ] Implement Wendland C4 kernel: W(r, h) = (1/h)³ * (1 - r/h)⁶ * (35r²/h² + 18r/h + 3) for r < h
- [ ] Implement kernel gradient ∇W(r, h) and Laplacian ∇²W(r, h) for SPH force calculations
- [ ] Implement SPH density summation: ρ_i = Σ (m_j * W(r_ij, h)) for all neighbors j
- [ ] Implement pressure calculation using equation of state: P_i = k_B * (ρ_i / μ * m_p) * (γ - 1) (ideal gas with γ = 5/3)
- [ ] Implement SPH pressure force: F_pressure,i = -Σ (m_j * (P_i/ρ_i² + P_j/ρ_j²) * ∇W(r_ij, h))
- [ ] Implement SPH viscosity force: F_viscosity,i = Σ (m_j * Π_ij * ∇W(r_ij, h)) where Π_ij is artificial viscosity
- [ ] Implement Sutherland & Dopita 1993 radiative cooling functions (lookup table for Λ(T) in erg/s/cm³)
- [ ] Implement gas collapse through radiative cooling (reduce internal energy by Λ(T) * dt per volume element)
- [ ] Create SPHSystem resource tracking SPH parameters (smoothing length h, viscosity α, β, equation of state γ)
- [ ] Implement SPH neighbor search using spatial hashing or kd-tree for O(log N) neighbor finding
- [ ] Add CosmicDawnEpoch plugin implementing the Cosmic Dawn epoch (100 Myr - 1 Gyr)

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
- [ ] Create PerformanceProfiler resource tracking FPS, GPU time, memory usage

### Cinematic Mode
- [ ] Implement pre-authored camera path system
- [ ] Add keyframe system with easing curves
- [ ] Create epoch narration text overlays
- [ ] Implement automatic camera transitions for demo
- [ ] Add single-button "Play Full Story" mode
- [ ] Create CinematicMode resource tracking keyframes, current keyframe index, playback state

### Expanded UI
- [ ] Add full cosmological parameter panel (Ωₘ, ΩΛ, H₀, n_s, σ₈)
- [ ] Create preset configurations (Standard Model, Einstein-de Sitter, De Sitter, Open Universe)
- [ ] Implement data overlay suite:
  - [ ] Temperature map
  - [ ] Density field
  - [ ] Velocity streamlines
  - [ ] Dark matter distribution
  - [ ] Power spectrum P(k) with observational comparisons
- [ ] Update Config struct to include full cosmological parameter set (Ωₘ, ΩΛ, H₀, n_s, σ₈)

### Capture & Export
- [ ] Implement PNG high-resolution frame capture
- [ ] Add EXR HDR frame capture
- [ ] Create frame-by-frame export controls
- [ ] Add image export settings panel (resolution, format, HDR toggle)

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
- [ ] Create installation scripts or packages for each platform (deb, rpm, dmg, msi)

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
