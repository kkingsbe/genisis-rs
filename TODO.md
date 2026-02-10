# TODO - Current Sprint (Sprint 2: Phase 2 - Inflation & Quantum Seeds)

**Sprint Goal:** Implement physics-driven cosmic inflation, seed the universe with density fluctuations that will become the cosmic web, and add temperature visualization.

---

## Test Health

### Failing Tests

### Ignored Tests (8 in genesis-render/tests/resource_binding_tests.rs)


## Sprint 2 - Phase 2: Inflation & Quantum Seeds

### Physics Integration
- [x] Couple particle positions to scale factor a(t) (multiply positions by current a(t) in update system)
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
  - [ ] Create QGPMaterial with temperature_uniform binding point sprite material
  - [ ] Implement shader color lookup from temperature-to-color ramp texture
  - [ ] Update particle instance color uniforms from Temperature resource each frame
- [ ] Create temperature-to-color ramp function (map temperature T to color: T > 10¹⁵K → blue-white, 10¹⁴K → white, 10¹³K → yellow, 10¹²K → orange)
  - [ ] Implement color_from_temperature(T: f64) -> Color function using piecewise linear interpolation
  - [ ] Define temperature color stops: (1e15, Color::rgb(200, 200, 255)), (1e14, Color::WHITE), (1e13, Color::rgb(255, 255, 100)), (1e12, Color::rgb(255, 165, 0))
  - [ ] Add unit tests verifying color transitions at each temperature threshold
- [ ] Implement epoch transition crossfade system (handle epoch change events, trigger camera and visual transitions)
  - [ ] Define EpochTransitionEvent struct with old_epoch: String, new_epoch: String, transition_progress: f64 fields in genesis-core/src/epoch/events.rs
  - [ ] Implement visual crossfade system for epoch transitions using alpha blending between render layers
    - [ ] Create TransitionState resource with alpha: f32 (0.0 to 1.0), is_transitioning: bool, duration: f64 fields
    - [ ] Implement update_transition_alpha() system that increments alpha based on dt and transition duration
    - [ ] Apply alpha blending to epoch-specific visual materials (fog, particle colors, background)
    - [ ] Use separate render layers for old and new epoch visual effects during transition
  - [ ] Create camera fade effect during epoch transitions (camera fade to black on exit, fade in on enter)
    - [ ] Implement camera_fade_overlay() system using fullscreen UI quad with alpha transparency
    - [ ] Fade to black: alpha goes from 0.0 to 1.0 over first half of transition
    - [ ] Fade from black: alpha goes from 1.0 to 0.0 over second half of transition
    - [ ] Register fade overlay in Update schedule with .run_if(TransitionState.is_transitioning) condition
  - [ ] Implement parameter interpolation during transitions (smooth temperature, scale factor changes)
    - [ ] Define interpolated_temperature = lerp(T_old, T_new, transition_progress)
    - [ ] Define interpolated_scale_factor = lerp(a_old, a_new, transition_progress)
    - [ ] Apply interpolated values to Temperature and ScaleFactor resources during transition
    - [ ] Use cubic interpolation (EaseInOutCubic) for smoother parameter transitions
  - [ ] Add epoch transition event handling in GenesisUiPlugin to update UI
    - [ ] Create update_epoch_indicator_ui() system listening for EpochTransitionEvent
    - [ ] Animate epoch name text change with fade-out/fade-in effect
    - [ ] Update epoch color in UI panel with smooth color transition
  - [ ] Trigger epoch transitions in EpochManager based on cosmic_time thresholds
    - [ ] Implement EpochManager::check_transition() system called each frame
    - [ ] Compare cosmic_time against epoch start/end times
    - [ ] Send EpochTransitionEvent when crossing epoch boundary
    - [ ] Set TransitionState.is_transitioning = true with appropriate duration
- [ ] Visualize density variations as brightness clumps (scale particle size and brightness by local density)
  - [ ] Create density_at_position() function querying DensityField resource for particle position
  - [ ] Calculate particle size multiplier: size = base_size * (1.0 + density * contrast_factor)
  - [ ] Calculate particle brightness: brightness = base_brightness * (1.0 + density * brightness_factor)
  - [ ] Update particle instance uniforms with computed size and brightness each frame
- [ ] Add SingularityEpoch plugin implementing epoch transition from Planck Boundary to Inflation
  - [ ] Implement singularity_exit_transition() system handling transition to Inflation epoch
  - [ ] Set transition camera position at [0, 0, 100] facing origin for inflation start
  - [ ] Configure fade duration: 0.5 seconds for quick visual transition
- [ ] Add InflationEpoch plugin implementing epoch transition from Inflation to Quark-Gluon Plasma
  - [ ] Implement inflation_exit_transition() system handling transition to QGP epoch
  - [ ] Set transition camera position at [0, 0, 500] for wider view of expanded universe
  - [ ] Configure fade duration: 1.0 seconds for longer transition
  - [ ] Apply temperature color ramp transition from inflation white to QGP blue-white
- [ ] Add QGPEpoch plugin implementing Quark-Gluon Plasma epoch with temperature-dependent rendering
  - [ ] Implement qgp_exit_transition() system handling transition to Nucleosynthesis epoch
  - [ ] Set transition camera position at [0, 0, 1000] for full QGP phase visualization
  - [ ] Configure fade duration: 1.0 seconds for smooth epoch handoff
  - [ ] Apply particle color transition from temperature-based to composition-based colors

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

---

## Drift Remediation

### Documentation Updates
- [ ] docs: Update COMPLETED.md to accurately reflect Phase 2 status - Clarify that only Phase 1 is complete, Phase 2 has infrastructure (crate, module stubs) but no physics implementation
- [ ] docs: Update TODO.md title - Either implement Phase 2 features or update title to reflect actual current work
- [ ] docs: Mark Phase 2 features as blocked - If Phase 2 cannot proceed, document blockers and dependencies

### Phase 2 Implementation Items
- [ ] feat: Implement slow-roll inflaton potential V(φ) - Add quadratic potential model V(φ) = ½m²φ² with m ~ 10¹⁶ GeV in genesis-physics/src/inflaton/mod.rs
- [ ] feat: Implement 3D Gaussian random field generator - Create system in genesis-physics/src/perturbations/mod.rs using Box-Muller transform
- [ ] feat: Implement power spectrum P(k) ∝ k^(n_s – 1) - Add configurable spectral index n_s parameter (default 0.96)
- [ ] feat: Implement Zel'dovich approximation - Add density-to-displacement mapping: displacement = ∇ψ where ∇²ψ = -δ
- [ ] feat: Map density perturbations to particle colors - Scale particle intensity by local density: intensity = 1.0 + α*δ
- [ ] feat: Add epoch indicator UI - Display current cosmic era, temperature, scale factor, and time in overlay
- [ ] feat: Create parameter panel sidebar - Add bevy_egui panel for n_s, inflation duration, and initial energy scale controls
- [ ] feat: Implement QGP temperature-based colors - Map temperature to color ramp: blue-white (10¹⁵K) → white (10¹⁴K) → yellow (10¹³K) → orange (10¹²K)
- [ ] feat: Couple particle positions to scale factor a(t) - Update particle positions by multiplying with current a(t) value

### Fix Items (contradictions with PRD)
- [ ] fix: Align InflatonPlugin registration with GenesisPhysicsPlugin - InflatonPlugin is exported but never registered in the application
- [ ] fix: Clarify "smooth interpolation" implementation scope - PRD mentions simple interpolation but code implements complex cubic ease-in-out system
- [ ] fix: Resolve Timeline speed control implementation inconsistency - Comment says direct pass-through but UI uses logarithmic scaling

### PRD vs Implementation Drift (Janitor Analysis 2026-02-10)

#### Unrequested Features (Phase 2+ features implemented in genesis-physics crate)

- [ ] refactor: Remove unrequested Friedmann equation integrator - genesis-physics/src/cosmology/mod.rs implements complete Friedmann equation physics (Hubble parameter computation, energy density components, scale factor integration) which is a Phase 2 requirement, not Phase 1
- [ ] refactor: Remove unrequested inflaton field module - genesis-physics/src/inflaton/mod.rs implements complete inflaton field physics (quadratic potential V(φ), slow-roll parameters ε and η) which is a Phase 2 requirement, not Phase 1
- [ ] refactor: Remove unrequested generic RK4 integrator - genesis-physics/src/integrator/mod.rs implements generic RK4 solver which is not required until Phase 2 (for Friedmann equations) or Phase 3 (for nucleosynthesis ODE solver)
- [ ] refactor: Remove unrequested epoch-based scale factor system - genesis-physics/src/cosmology/mod.rs::update_scale_factor_by_epoch() implements cosmic epoch transitions and different expansion laws (exponential during inflation, matter-dominated post-inflation) which are Phase 2+ features
- [ ] refactor: Remove unrequested CosmologyPlugin - genesis-physics/src/cosmology/mod.rs::CosmologyPlugin registers resources and systems for cosmological physics (ScaleFactor, HubbleParameter, EnergyDensity) which are not needed in Phase 1
- [ ] refactor: Remove GenesisPhysicsPlugin or defer to Phase 2 - genesis-physics/src/lib.rs::GenesisPhysicsPlugin is exported but the entire physics crate should be deferred to Phase 2 when implementing inflation physics
- [ ] refactor: Fix nucleosynthesis module placeholder comment - genesis-physics/src/nucleosynthesis/mod.rs comment says "Phase 5" but nucleosynthesis is actually a Phase 3 requirement per PRD
- [ ] refactor: Fix perturbations module placeholder comment - genesis-physics/src/perturbations/mod.rs comment says "Phase 5" but density perturbations are actually a Phase 2 requirement per PRD

#### Existing Drift Items (previously identified)

- [ ] fix: Align timeline speed control with PRD logarithmic acceleration requirement - apply logarithmic mapping from PlaybackState.speed (1.0 to 1e12) to TimeAccumulator.acceleration in genesis-ui/src/timeline/mod.rs sync_time_resources() function

### Refactor Items (unrequested features)
- [ ] refactor: Remove or document `years_to_gev_inv()` if not needed for Phase 1 - GeV⁻¹ conversion not mentioned in Phase 1 requirements
- [ ] refactor: Timeline pre-1-year scaling extends beyond Phase 1 scope - PRD Phase 1 specifies 13.8 billion years, not pre-1-year timescales
- [ ] refactor: Evaluate particle scrubbing complexity vs Phase 1 scope - ScrubbingState is more advanced than Phase 1's basic play/pause
- [ ] refactor: Reconsider `sync_particle_position()` system necessity - May not be needed since camera manipulation shouldn't modify particle transforms
- [ ] refactor: Generic RK4 integrator may be over-engineering for Phase 2 - Specific solvers (Friedmann, Rosenbrock) come in later phases
- [ ] refactor: Scroll wheel zoom extends beyond Phase 1 camera requirements - PRD Phase 1 only specifies orbit camera (click-drag)
- [ ] refactor: ScrubbingEvent emission may exceed Phase 1 timeline requirements - Event system more complex than simple play/pause needed

---

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.
