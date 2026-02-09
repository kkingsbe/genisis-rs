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
- [ ] Create easing function module with trait definition in genesis-render/src/camera/easing.rs
  - [ ] Define EasingFunction trait with fn ease(&self, t: f32) -> f32 method where t ∈ [0.0, 1.0]
  - [ ] Add derive(Debug, Clone, Copy, PartialEq) attributes for serde serialization support
  - [ ] Add trait bounds requiring 'static lifetime for Bevy resource registration
  - [ ] Define EasingType enum with variants: Linear, EaseInQuad, EaseOutQuad, EaseInOutCubic, EaseInCubic, EaseOutCubic, EaseInOutQuart, EaseOutQuart
  - [ ] Implement From<EasingType> for Box<dyn EasingFunction> to convert enum to concrete easing function
  - [ ] Add Default impl for EasingType set to Linear
  - [ ] Register module in genesis-render/src/camera/mod.rs via pub mod easing; and pub use easing::{EasingFunction, EasingType};
- [ ] Implement Linear easing function in genesis-render/src/camera/easing.rs
  - [ ] Create Linear struct implementing EasingFunction trait
  - [ ] Implement ease() method returning t unchanged (no acceleration or deceleration)
  - [ ] Add derive(Debug, Clone, Copy) attributes for Bevy compatibility
  - [ ] Document that Linear provides constant velocity transitions: f(t) = t for all t ∈ [0.0, 1.0]
  - [ ] Add unit test in genesis-render/src/camera/easing/tests.rs verifying ease(0.0) = 0.0, ease(0.5) = 0.5, ease(1.0) = 1.0
- [ ] Implement EaseInQuad easing function in genesis-render/src/camera/easing.rs
  - [ ] Create EaseInQuad struct implementing EasingFunction trait
  - [ ] Implement ease() method returning t * t (quadratic acceleration from rest)
  - [ ] Add derive(Debug, Clone, Copy) attributes for Bevy compatibility
  - [ ] Document that EaseInQuad starts slow and accelerates: f(t) = t² with steepness increasing as t → 1.0
  - [ ] Add unit test verifying ease(0.0) = 0.0, ease(0.5) = 0.25, ease(1.0) = 1.0
  - [ ] Use for camera movements that should build momentum from initial state
- [ ] Implement EaseOutQuad easing function in genesis-render/src/camera/easing.rs
  - [ ] Create EaseOutQuad struct implementing EasingFunction trait
  - [ ] Implement ease() method returning t * (2.0 - t) (quadratic deceleration to rest)
  - [ ] Add derive(Debug, Clone, Copy) attributes for Bevy compatibility
  - [ ] Document that EaseOutQuad starts fast and decelerates: f(t) = t(2-t) with gentler slope as t → 1.0
  - [ ] Add unit test verifying ease(0.0) = 0.0, ease(0.5) = 0.75, ease(1.0) = 1.0
  - [ ] Use for camera movements that should slow down smoothly at destination
- [ ] Implement EaseInOutCubic easing function in genesis-render/src/camera/easing.rs
  - [ ] Create EaseInOutCubic struct implementing EasingFunction trait
  - [ ] Implement ease() method with conditional logic: if t < 0.5 return 4.0 * t * t * t else return 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
  - [ ] Add derive(Debug, Clone, Copy) attributes for Bevy compatibility
  - [ ] Document that EaseInOutCubic accelerates then decelerates with smooth S-curve: cubic polynomial with zero derivative at endpoints
  - [ ] Add unit test verifying ease(0.0) = 0.0, ease(0.25) = 0.0625, ease(0.5) = 0.5, ease(0.75) = 0.9375, ease(1.0) = 1.0
  - [ ] Use for smooth cinematic camera transitions with no abrupt velocity changes
- [ ] Add EasingType field to CameraState in genesis-render/src/camera/mod.rs
  - [ ] Add easing_type: EasingType field to CameraState struct with default value Linear
  - [ ] Update CameraState::default() to initialize easing_type = EasingType::Linear
  - [ ] Add easing_type parameter to CameraState::new() constructor method
  - [ ] Add set_easing() method to CameraState that updates easing_type field and returns &mut Self for chaining
  - [ ] Add getter method CameraState::easing_type(&self) -> EasingType for UI binding
  - [ ] Ensure serde derives on CameraState include easing_type field for config persistence
- [ ] Integrate easing function into camera interpolation system in genesis-render/src/camera/mod.rs
  - [ ] Modify interpolate_camera() system to call easing_type.ease(interpolation_progress) instead of linear interpolation
  - [ ] Apply eased progress value to position interpolation: current_pos = start_pos + (end_pos - start_pos) * eased_progress
  - [ ] Apply eased progress value to rotation interpolation: slerp quaternion rotation by eased_progress instead of linear
  - [ ] Add system parameter Query<&mut CameraState> to access easing_type field during interpolation
  - [ ] Register interpolate_camera() system in PostUpdate schedule after input handling
  - [ ] Add UI slider in camera control panel to select EasingType from dropdown menu
  - [ ] Connect slider changes to CameraState.set_easing() via event system for runtime easing adjustment
- [ ] ~~Create CameraTween resource tracking active tween~~ (REPLACED BY: CameraState already tracks interpolation state with start_pos, end_pos, interpolation_speed, interpolation_progress)
- [ ] ~~Implement camera tween update system that interpolates camera position over time~~ (COMPLETED: See interpolate_camera() in genesis-render/src/camera/mod.rs)
- [ ] Add camera tween trigger system that initiates interpolation when epoch changes
  - [ ] Create system that listens for EpochChangeEvent events
  - [ ] Extract camera_config from target epoch (target_position, target_rotation, fade_duration)
  - [ ] Call CameraState::start_interpolation_to_target() with epoch camera config
  - [ ] Register this system in main.rs after epoch_manager plugin

### UI Implementation
- [ ] Create EpochIndicatorPanel UI component in genesis-ui/src/overlay/epoch_indicator.rs
  - [ ] Define EpochIndicatorPanel struct with visibility: bool, position: egui::Pos2, size: egui::Vec2 fields for UI window configuration
  - [ ] Implement EpochIndicatorPanel::new() constructor with default position at top-right corner (Pos2::new(screen_width - 350.0, 50.0))
  - [ ] Create epoch_indicator_panel_ui() function accepting egui::Ui, Res<CosmicTime>, Res<Temperature>, Res<ScaleFactor>, Res<EpochManager> parameters
  - [ ] Design panel layout using egui::Window::new("Epoch Information") with collapsible frame and alpha transparency (0.8)
  - [ ] Add display_row() helper function for formatted label-value pairs with monospace font for values
  - [ ] Register panel rendering system in GenesisUiPlugin::build() via add_systems(PostUpdate, epoch_indicator_panel_ui.run_if(show_epoch_info))
- [ ] Format and display cosmic time in epoch indicator panel in genesis-ui/src/overlay/epoch_indicator.rs
  - [ ] Query CosmicTime resource and extract current_years field
  - [ ] Implement format_cosmic_time(years: f64) -> String function returning appropriate units and precision
  - [ ] Use conditional formatting: if years < 1e-6 return format!("{:.2e} s", years * 3.15e7), if years < 1.0 return format!("{:.3} yr", years), if years < 1e6 return format!("{:.1} Kyr", years / 1e3), else return format!("{:.3} Gyr", years / 1e9)
  - [ ] Display formatted time in panel with label "Cosmic Time:" using egui::Label::new() with text color egui::Color32::LIGHT_BLUE
  - [ ] Add scientific notation toggle in panel settings for alternative display (e.g., "1.23e+11 yr")
- [ ] Format and display temperature in epoch indicator panel in genesis-ui/src/overlay/epoch_indicator.rs
  - [ ] Query Temperature resource and extract value field (in Kelvin)
  - [ ] Implement format_temperature(kelvin: f64) -> String function returning formatted temperature with units
  - [ ] Use conditional formatting: if kelvin > 1e15 return format!("{:.1e} K", kelvin) with 1 significant figure, if kelvin > 1e3 return format!("{:.0} K", kelvin) with integer display, else return format!("{:.2} K", kelvin)
  - [ ] Map temperature range to color gradient using color_from_temperature() function: T > 1e20 K → egui::Color32::WHITE, T > 1e10 K → egui::Color32::from_rgb(200, 200, 255) (blue-white), T > 1e5 K → egui::Color32::from_rgb(255, 255, 100) (yellow), T > 3000 K → egui::Color32::from_rgb(255, 100, 50) (orange), T ≤ 3000 K → egui::Color32::from_rgb(50, 100, 200) (dark blue)
  - [ ] Display formatted temperature in panel with label "Temperature:" using temperature-based color for value text
  - [ ] Add temperature bar visualization using egui::ProgressBar spanning 10⁻³ K to 10³⁰ K logarithmic range
- [ ] Format and display scale factor in epoch indicator panel in genesis-ui/src/overlay/epoch_indicator.rs
  - [ ] Query ScaleFactor resource and extract value field (dimensionless metric expansion factor)
  - [ ] Implement format_scale_factor(a: f64) -> String function returning formatted value with units
  - [ ] Use conditional formatting: if a >= 1e20 return format!("{:.1e}", a) with scientific notation, if a >= 1000.0 return format!("{:.0}", a) with integer display, else return format!("{:.6}", a) with 6 decimal places
  - [ ] Add prefix "a = " to display string for clarity: format!("a = {}", formatted_value)
  - [ ] Display formatted scale factor in panel with label "Scale Factor:" using egui::Color32::LIGHT_GREEN for value text
  - [ ] Add scale factor bar visualization using egui::ProgressBar spanning a = 0.001 to a = 1e25 logarithmic range
- [ ] Query and display current epoch name in epoch indicator panel in genesis-ui/src/overlay/epoch_indicator.rs
  - [ ] Query EpochManager resource and call current_epoch() method to get active epoch name
  - [ ] Query EpochManager::current_epoch_name() returning &str (e.g., "Singularity", "Inflation", "Quark-Gluon Plasma", "Nucleosynthesis", "Recombination", "Dark Ages", "Cosmic Dawn")
  - [ ] Display epoch name at top of panel with larger font (egui::FontId::proportional(18.0)) and bold styling
  - [ ] Map epoch names to color scheme using epoch_color_map() function: Singularity → egui::Color32::WHITE, Inflation → egui::Color32::from_rgb(100, 200, 255), QGP → egui::Color32::from_rgb(255, 200, 100), Nucleosynthesis → egui::Color32::from_rgb(255, 100, 255), Recombination → egui::Color32::from_rgb(100, 255, 100), Dark Ages → egui::Color32::from_rgb(50, 50, 100), Cosmic Dawn → egui::Color32::from_rgb(255, 255, 200)
  - [ ] Add epoch description tooltip using egui::RichText::new() with epoch-specific educational text
  - [ ] Display epoch time range below epoch name using format_time_range() helper from EpochManager
- [ ] Connect epoch indicator panel visibility to OverlayState toggle in genesis-ui/src/overlay/mod.rs
  - [ ] Query OverlayState resource and access show_epoch_info field (bool)
  - [ ] In epoch_indicator_panel_ui() system, add .run_if(|overlay: Res<OverlayState>| overlay.show_epoch_info) system condition to only render when enabled
  - [ ] Add toggle button in overlay settings panel: egui::Checkbox::new(&mut overlay.show_epoch_info, "Show Epoch Indicator")
  - [ ] Persist show_epoch_info preference to Config via ConfigResource when changed
  - [ ] Add keyboard shortcut (e.g., Ctrl+E) to toggle epoch indicator visibility via input event handling
  - [ ] Update OverlayState::default() to set show_epoch_info = true to display on startup
- [ ] ~~Build FPS counter overlay system using bevy_egui~~ (COMPLETED: See update_overlay_ui() in genesis-ui/src/overlay/mod.rs)
- [ ] ~~Create particle count overlay system~~ (COMPLETED: See update_overlay_ui() in genesis-ui/src/overlay/mod.rs)
- [ ] ~~Build time control UI (play/pause button, speed slider, reset button)~~ (COMPLETED: See timeline_panel_ui() in genesis-ui/src/timeline/mod.rs)
- [ ] ~~Implement logarithmic timeline scrubber using bevy_egui Slider widget~~ (COMPLETED: See timeline_panel_ui() in genesis-ui/src/timeline/mod.rs)
- [ ] ~~Update main.rs to initialize PlaybackState resource~~ (COMPLETED: TimelinePlugin already inserts PlaybackState)
- [ ] Create SimulationSnapshot data structure and resource in genesis-core/src/time/snapshot.rs
  - [ ] Define ParticleState struct with position: Vec3, velocity: Vec3, energy: f32 fields for individual particle state capture
  - [ ] Define SimulationSnapshot struct with cosmic_time: f64, particles: Vec<ParticleState>, particle_count: usize fields for complete state representation
  - [ ] Add derive(Deserialize, Serialize) traits to both structs for optional persistence (feature-gated)
  - [ ] Create impl SimulationSnapshot with new() constructor and from_entities() method to extract state from Bevy Query<(&Transform, &Particle)>
  - [ ] Register SimulationSnapshot as Bevy Resource via SimulationSnapshotPlugin with insert_resource(SimulationSnapshot::default()) in build()
  - [ ] Add apply_to_entities() method on SimulationSnapshot that updates entity Transform and Particle components from snapshot data
- [ ] Implement snapshot history buffer with circular storage in genesis-core/src/time/snapshot.rs
  - [ ] Define SnapshotHistory struct with capacity: usize (default 20), snapshots: VecDeque<SimulationSnapshot>, last_capture_time: f64 fields
  - [ ] Implement SnapshotHistory::push() method that adds new snapshots and evicts oldest when capacity exceeded using VecDeque::push_front()
  - [ ] Implement SnapshotHistory::find_nearest(cosmic_time: f64) -> Option<&SimulationSnapshot> returning snapshot with minimum |snapshot.time - requested_time|
  - [ ] Implement SnapshotHistory::clear() method for resetting buffer on simulation reset
  - [ ] Add get_snapshot_interval() method returning fixed time interval between captures (default: every 1e28 years on log scale)
  - [ ] Register SnapshotHistory as Bevy Resource via SimulationSnapshotPlugin with insert_resource(SnapshotHistory::with_capacity(20)) in build()
- [ ] Implement state capture system in genesis-render/src/particle/mod.rs
  - [ ] Create capture_particle_state() system function querying Query<(Entity, &Transform, &Particle)> and CosmicTime resource
  - [ ] Implement capture logic that checks if current_time - last_capture_time >= interval before creating new SimulationSnapshot
  - [ ] Use Query::iter() to collect all particle states into Vec<ParticleState> with position=Transform.translation, velocity=Particle.velocity, energy=Particle.energy
  - [ ] Call SimulationSnapshot::from_entities() with collected particle data and current cosmic_time
  - [ ] Push new snapshot to SnapshotHistory resource via history.push(snapshot)
  - [ ] Register capture_particle_state() system in PostUpdate schedule with .run_if(time_state_changed) condition to capture after time changes
- [ ] Implement state restoration system in genesis-render/src/particle/mod.rs
  - [ ] Create restore_particle_state() system function accepting target_time: f64 parameter
  - [ ] Query SnapshotHistory resource and call history.find_nearest(target_time) to locate best matching snapshot
  - [ ] If snapshot found, iterate through snapshot.particles and update corresponding entities via Transform::translation = state.position, Particle.velocity = state.velocity, Particle.energy = state.energy
  - [ ] Handle mismatched particle counts by truncating or padding with default states to prevent index out-of-bounds errors
  - [ ] Set CosmicTime resource to snapshot.cosmic_time to synchronize timeline UI with restored state
  - [ ] Register restore_particle_state() system in PostUpdate schedule with .run_if(scrubbing_active) condition
- [ ] Implement reverse playback mode in genesis-ui/src/timeline/mod.rs
  - [ ] Add is_reverse: bool field to PlaybackState struct (default: false)
  - [ ] Modify sync_time_resources() system to check playback_state.is_reverse and multiply acceleration by -1.0 when true
  - [ ] Add reverse_mode toggle button to timeline_panel_ui() using egui::Checkbox::new(&mut playback_state.is_reverse, "Reverse Playback")
  - [ ] Implement reverse playback indicator in UI showing "◀ REVERSE" with red color when active
  - [ ] Handle reverse playback edge case: when cosmic_time < MIN_TIME (10⁻³²s), pause simulation and set TimeAccumulator.paused = true
  - [ ] Add system that automatically sets is_reverse = false when playback_state.playing transitions from false to true (reset to forward on play)
- [ ] Connect timeline slider scrubbing to state restoration in genesis-ui/src/timeline/mod.rs
  - [ ] Modify timeline_panel_ui() slider to detect scrubbing events using Slider::changed() return value
  - [ ] When scrubbing detected, call TimelineScrubEvent { target_time: slider_value } event
  - [ ] Create TimelineScrubEvent struct in genesis-ui/src/timeline/events.rs with target_time: f64 field
  - [ ] Add handle_timeline_scrub() system in genesis-ui that listens for TimelineScrubEvent events
  - [ ] In handle_timeline_scrub(), call restore_particle_state() with event.target_time and set playback_state.playing = false (pause on scrub)
  - [ ] Handle edge case when scrubbing beyond snapshot history by clamping target_time to oldest/newest snapshot times via target_time.clamp(oldest, newest)
  - [ ] Handle unvisited time regions by creating interpolated snapshot between nearest stored snapshots using linear interpolation of particle states

### Configuration System
- [ ] ~~Create genesis-config module with Config struct~~ (COMPLETED: See genesis-core/src/config.rs)
- [ ] ~~Add serde dependencies to genesis-core/Cargo.toml~~ (COMPLETED: serde already present in Cargo.toml)
- [ ] ~~Implement TOML deserialization for Config struct~~ (COMPLETED: See Config::load_from_file() in genesis-core/src/config.rs)
- [ ] ~~Create default Config constants~~ (COMPLETED: See Default impl for Config struct)
- [ ] ~~Implement config file loader with path resolution~~ (COMPLETED: See Config::load_from_path() in genesis-core/src/config.rs)
- [ ] ~~Implement clap argument parser for --config flag~~ (COMPLETED: See CliArgs and Config::load() in genesis-core/src/config.rs)
- [ ] ~~Add ConfigResource and insert into main.rs~~ (COMPLETED: See ConfigResource wrapper in src/main.rs)
- [ ] Update spawn_particles() to use config.particle.initial_count instead of constant PARTICLE_COUNT=1000
- [ ] Refactor ParticlePlugin to read base_size from config.particle.base_size
- [ ] Add movement_speed field to CameraConfig in genesis-core/src/config.rs
- [ ] Add mouse_sensitivity field to CameraConfig in genesis-core/src/config.rs
- [ ] Refactor time acceleration to use config.time.initial_time_acceleration in TimeAccumulator

### Epoch Plugin System (DEFERRED - Planned for Phase 2+)
**NOTE: The following infrastructure is NOT currently implemented:**
- EpochPlugin trait - NOT defined (planned for Phase 2+)
- EpochManager resource - NOT defined (planned for Phase 2+)
- Epoch transition systems - NOT defined (planned for Phase 2+)

**Currently Available (Phase 1):**
- CameraMode enum (FreeFlight, Orbit) in genesis-core/epoch/camera_config.rs
- EpochCameraConfig struct in genesis-core/epoch/camera_config.rs
- SingularityEpoch marker struct in genesis-core/epoch/singularity.rs

The following items were previously marked as completed but are NOT implemented:
- [ ] Implement epoch plugin registration system (NOT YET IMPLEMENTED - EpochManager does not exist)
- [ ] Define EpochPlugin trait (NOT YET IMPLEMENTED)
- [ ] Create SingularityEpoch plugin implementation (marker struct exists but does not implement EpochPlugin trait)
- [ ] Implement EpochManager resource (NOT YET IMPLEMENTED)
- [ ] Register epoch plugins in main.rs (NOT YET IMPLEMENTED - requires EpochManager and EpochPlugin trait)

**NOTE:** The following epoch plugin creation tasks (lines 220-269) span all future phases (2-7). These umbrella tasks are well-broken down into subtasks but should be distributed to their respective sprint sections for better organization. Consider moving these tasks to: Sprint 2 (InflationEpoch, QGPEpoch), Sprint 3 (NucleosynthesisEpoch), Sprint 4 (RecombinationEpoch), Sprint 5 (DarkAgesEpoch), Sprint 6 (CosmicDawnEpoch).

- [ ] Implement future epoch plugins (InflationEpoch, QGPEpoch, NucleosynthesisEpoch, RecombinationEpoch, DarkAgesEpoch, CosmicDawnEpoch)
  - [ ] Create InflationEpoch plugin in genesis-core/src/epoch/inflation.rs (10⁻³²s to 10⁻⁶s)
    - [ ] Implement InflationEpoch struct with EpochPlugin trait
    - [ ] Define name() returning "Inflation"
    - [ ] Define start_year() returning 1e-32 (years after Big Bang)
    - [ ] Define end_year() returning 1e-6 (years)
    - [ ] Implement build() method registering inflation physics systems (scale factor, inflaton field)
    - [ ] Define camera_config() with optimal camera settings for inflation phase (position, target, distance)
  - [ ] Create QGPEpoch plugin in genesis-core/src/epoch/qgp.rs (10⁻⁶s to 3 min)
    - [ ] Implement QGPEpoch struct with EpochPlugin trait
    - [ ] Define name() returning "Quark-Gluon Plasma"
    - [ ] Define start_year() returning 1e-6 (years)
    - [ ] Define end_year() returning 3/60/24/365 (3 minutes in years ≈ 5.7e-6)
    - [ ] Implement build() method registering QGP visualization systems (temperature-based rendering)
    - [ ] Define camera_config() with optimal camera settings for QGP phase
  - [ ] Create NucleosynthesisEpoch plugin in genesis-core/src/epoch/nucleosynthesis.rs (3 min to 20 min)
    - [ ] Implement NucleosynthesisEpoch struct with EpochPlugin trait
    - [ ] Define name() returning "Nucleosynthesis"
    - [ ] Define start_year() returning 3/60/24/365 (3 minutes in years)
    - [ ] Define end_year() returning 20/60/24/365 (20 minutes in years ≈ 3.8e-5)
    - [ ] Implement build() method registering nuclear reaction network and composition tracking systems
    - [ ] Define camera_config() with optimal camera settings for nucleosynthesis phase
  - [ ] Create RecombinationEpoch plugin in genesis-core/src/epoch/recombination.rs (~380,000 yr)
    - [ ] Implement RecombinationEpoch struct with EpochPlugin trait
    - [ ] Define name() returning "Recombination"
    - [ ] Define start_year() returning 380000 (years)
    - [ ] Define end_year() returning 400000 (years)
    - [ ] Implement build() method registering Saha equation solver, fog rendering, and CMB sphere systems
    - [ ] Define camera_config() with optimal camera settings for recombination phase (including pull-back position)
  - [ ] Create DarkAgesEpoch plugin in genesis-core/src/epoch/dark_ages.rs (380 Kyr to 100 Myr)
    - [ ] Implement DarkAgesEpoch struct with EpochPlugin trait
    - [ ] Define name() returning "Dark Ages"
    - [ ] Define start_year() returning 380000 (years)
    - [ ] Define end_year() returning 100000000 (100 million years)
    - [ ] Implement build() method registering N-body gravity and halo finder systems
    - [ ] Define camera_config() with optimal camera settings for dark ages phase
  - [ ] Create CosmicDawnEpoch plugin in genesis-core/src/epoch/cosmic_dawn.rs (100 Myr to 1 Gyr)
    - [ ] Implement CosmicDawnEpoch struct with EpochPlugin trait
    - [ ] Define name() returning "Cosmic Dawn"
    - [ ] Define start_year() returning 100000000 (100 million years)
    - [ ] Define end_year() returning 1000000000 (1 billion years)
    - [ ] Implement build() method registering SPH hydrodynamics, star formation, and galaxy rendering systems
    - [ ] Define camera_config() with optimal camera settings for cosmic dawn phase
  - [ ] Register all epoch plugins in main.rs using EpochManager registration pattern
    - [ ] Add .add_plugin(InflationEpochPlugin) in main.rs
    - [ ] Add .add_plugin(QGPEpochPlugin) in main.rs
    - [ ] Add .add_plugin(NucleosynthesisEpochPlugin) in main.rs
    - [ ] Add .add_plugin(RecombinationEpochPlugin) in main.rs
    - [ ] Add .add_plugin(DarkAgesEpochPlugin) in main.rs
    - [ ] Add .add_plugin(CosmicDawnEpochPlugin) in main.rs

### Core System Integration
- [ ] ~~Implement pause() method in TimeAccumulator resource~~ (COMPLETED: See TimeAccumulator::pause() and resume() in genesis-core/src/time/mod.rs)
- [ ] ~~Implement smooth camera interpolation system~~ (COMPLETED: See CameraState interpolation infrastructure in genesis-render/src/camera/mod.rs)

### Documentation
- [ ] Update ARCHITECTURE.md with final crate structure and responsibilities (document genesis-core, genesis-render, genesis-ui responsibilities)
- [ ] Document epoch plugin architecture design patterns (trait-based plugin system, EpochManager registration, epoch transitions) - **DEFERRED to Phase 2+ when EpochPlugin trait is implemented**
- [ ] Add inline documentation for genesis-core public APIs (time::TimeAccumulator, epoch::CameraMode, epoch::EpochCameraConfig, physics::Particle) - **Note: EpochPlugin trait not implemented yet**
- [ ] Add inline documentation for genesis-render public APIs (camera::CameraMode/State, input::InputState, particle::Particle component)
- [ ] Add inline documentation for genesis-ui public APIs (overlay::OverlayState, timeline::PlaybackState)
- [ ] Document CosmicTime resource methods (from_slider, to_slider, set_time, get_time, reset)
- [ ] Document PointSpriteMaterial uniform parameters (color, base_size, attenuation_factor)
- [ ] Document OrbitController spherical coordinate system (distance, yaw, pitch, target)

### Build System
**NOTE:** Cross-platform build configuration tasks (lines 285-368) are listed in Sprint 1 but per PRD.md line 251, these should be part of Sprint 7 (Phase 7: Polish, Cinematic Mode & Release). These tasks are well-broken down into subtasks and should be moved to Sprint 7 section.

- [ ] Configure Cargo.toml for cross-platform builds in project root
  - [ ] Add [target.'cfg(all(target_os = "macos", target_arch = "aarch64")'.dependencies] section for Apple Silicon specific dependencies
  - [ ] Add [target.'cfg(target_os = "windows")'.dependencies] section for Windows-specific dependencies (e.g., winapi)
  - [ ] Add [target.'cfg(target_os = "linux")'.dependencies] section for Linux-specific dependencies (e.g., alsa-sys)
  - [ ] Configure wgpu backend selection via [target.'cfg(target_os = "macos")'.dependencies] forcing Metal backend: wgpu = { version = "0.20", default-features = false, features = ["metal"] }
  - [ ] Configure wgpu for Linux with Vulkan and Wayland support: wgpu = { version = "0.20", default-features = false, features = ["vulkan", "wayland"] }
  - [ ] Configure wgpu for Windows with DX12 and DX11 support: wgpu = { version = "0.20", default-features = false, features = ["dx12", "dx11"] }
  - [ ] Add conditional compilation flags in genesis-core/src/lib.rs using #[cfg(target_os = "...")] for platform-specific code paths
  - [ ] Add [package.metadata.bundle] section for macOS app bundle configuration (identifier, icon, info_plist settings)
- [ ] Set up cross-compilation build scripts in .github/workflows/build.yml
  - [ ] Create Linux build job with steps: actions/checkout@v3, actions-rs/toolchain@stable-x86_64-unknown-linux-gnu, cargo build --release --target x86_64-unknown-linux-gnu
  - [ ] Create macOS Intel build job with steps: actions-rs/toolchain@stable-x86_64-apple-darwin, cargo build --release --target x86_64-apple-darwin
  - [ ] Create macOS Apple Silicon build job with steps: actions-rs/toolchain@stable-aarch64-apple-darwin, cargo build --release --target aarch64-apple-darwin
  - [ ] Create Windows build job with steps: actions-rs/toolchain@stable-x86_64-pc-windows-msvc, cargo build --release --target x86_64-pc-windows-msvc
  - [ ] Add artifact upload steps for each platform using actions/upload-artifact@v3 with binary files
  - [ ] Add cache actions for cargo registry and target directory to speed up builds: actions/cache@v3 with paths: ~/.cargo/registry, ~/.cargo/git, target
  - [ ] Configure matrix strategy to run Linux, macOS Intel, macOS Apple Silicon, Windows builds in parallel
  - [ ] Add conditional job for creating universal macOS binary using lipo tool to combine Intel and Apple Silicon binaries
- [ ] Configure platform-specific packaging in Cargo.toml
  - [ ] Add [package.metadata.deb] section for Linux Debian package: depends = ["libvulkan1", "libwayland-client0", "libxkbcommon0"], assets = [["target/release/genesis", "usr/bin/"], ["README.md", "usr/share/doc/genesis/"]]
  - [ ] Add [package.metadata.rpm] section for Linux Red Hat package: summary = "Genesis cosmological simulation", license = "MIT"
  - [ ] Add [package.metadata.msi] section for Windows installer: product_code = "{GUID}", upgrade_code = "{GUID}"
  - [ ] Configure macOS app bundle via [package.metadata.bundle] with CFBundleExecutable = "genesis", CFBundleIconFile = "AppIcon.icns"
  - [ ] Add code signing configuration for macOS (hardened runtime, notarization): bundle.macos.codesign_identity = "Developer ID Application: Your Name"
  - [ ] Add Windows code signing via signtool.exe integration: [package.metadata.msi].sign_command = "signtool.exe sign /f cert.pfx /p password genesis.msi"
  - [ ] Add installer icon generation using embedded resources: icon.png for Linux, icon.ico for Windows, AppIcon.icns for macOS
- [ ] Create platform-specific build and release scripts in scripts/ directory
  - [ ] Create scripts/build_linux.sh shell script for Linux builds:
    - #!/bin/bash
    - cargo build --release --features "vulkan wayland"
    - strip target/release/genesis
    - cp target/release/genesis dist/genesis-linux-x64
    - tar -czf genesis-linux-x64.tar.gz -C dist genesis-linux-x64 README.md LICENSE
  - [ ] Create scripts/build_macos_intel.sh shell script for macOS Intel builds:
    - #!/bin/bash
    - cargo build --release --target x86_64-apple-darwin --features "metal"
    - strip target/x86_64-apple-darwin/release/genesis
    - mkdir -p dist/Genesis.app/Contents/MacOS
    - cp target/x86_64-apple-darwin/release/genesis dist/Genesis.app/Contents/MacOS/
    - cp Info.plist dist/Genesis.app/Contents/
    - hdiutil create -volname Genesis -srcfolder dist/Genesis.app -ov -format UDZO genesis-macos-intel.dmg
  - [ ] Create scripts/build_macos_silicon.sh shell script for macOS Apple Silicon builds:
    - #!/bin/bash
    - cargo build --release --target aarch64-apple-darwin --features "metal"
    - strip target/aarch64-apple-darwin/release/genesis
    - mkdir -p dist/GenesisSilicon.app/Contents/MacOS
    - cp target/aarch64-apple-darwin/release/genesis dist/GenesisSilicon.app/Contents/MacOS/
    - cp Info.plist dist/GenesisSilicon.app/Contents/
    - hdiutil create -volname GenesisSilicon -srcfolder dist/GenesisSilicon.app -ov -format UDZO genesis-macos-silicon.dmg
  - [ ] Create scripts/build_windows.ps1 PowerShell script for Windows builds:
    - cargo build --release --features "dx12 dx11"
    - $target = "dist/genesis-windows-x64"
    - Copy-Item target/release/genesis.exe $target
    - Compress-Archive -Path $target, README.md, LICENSE -DestinationPath genesis-windows-x64.zip
    - Remove-Item $target
  - [ ] Create scripts/build_universal_macos.sh shell script combining Intel and Apple Silicon:
    - #!/bin/bash
    - ./build_macos_intel.sh && ./build_macos_silicon.sh
    - lipo -create -output dist/GenesisUniversal.app/Contents/MacOS/genesis dist/Genesis.app/Contents/MacOS/genesis dist/GenesisSilicon.app/Contents/MacOS/genesis
    - cp Info.plist dist/GenesisUniversal.app/Contents/
    - hdiutil create -volname GenesisUniversal -srcfolder dist/GenesisUniversal.app -ov -format UDZO genesis-macos-universal.dmg
- [ ] Add platform-specific testing and validation in CI pipeline
  - [ ] Create scripts/test_linux.sh running Linux tests with Vulkan validation layers: VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation cargo test --release
  - [ ] Create scripts/test_macos.sh running macOS tests with Metal validation: MTL_DEBUG_LAYER=1 cargo test --release
  - [ ] Create scripts/test_windows.ps1 running Windows tests with DirectX Debug Layer: cargo test --release
  - [ ] Add performance benchmarking per platform: scripts/benchmark.sh running 60 FPS test with 1M particles for 60 seconds
  - [ ] Add UI rendering validation per platform: screenshot capture at specific cosmic_time points (singularity, inflation, QGP, nucleosynthesis, recombination)
  - [ ] Add smoke test for each platform: verify app launches without crash, renders particles, responds to user input for 30 seconds
  - [ ] Add regression testing: compare current build screenshots against reference screenshots for each epoch and platform
  - [ ] Configure GitHub Actions to run test scripts after successful build, uploading test results as artifacts
  - [ ] Add platform-specific known issues tracking in docs/KNOWN_ISSUES.md (e.g., "macOS: Metal renderer may have artifacts on AMD GPUs")
- [ ] Set up release workflow and distribution in .github/workflows/release.yml
  - [ ] Create release workflow triggered on git tag v* (e.g., v1.0.0)
  - [ ] Configure release workflow to run all platform build jobs (Linux, macOS Intel, macOS Apple Silicon, Windows, macOS Universal)
  - [ ] Add GitHub Release creation step using softprops/action-gh-release@v1 with generated release notes from CHANGELOG.md
  - [ ] Upload all platform binaries as release assets: genesis-linux-x64.tar.gz, genesis-macos-intel.dmg, genesis-macos-silicon.dmg, genesis-macos-universal.dmg, genesis-windows-x64.zip
  - [ ] Generate and attach SHA256 checksums.txt file for all binaries using sha256sum command
  - [ ] Attach source code archive genesis-VERSION.tar.gz created via git archive
  - [ ] Add auto-generated release notes from CHANGELOG.md entries since last release tag
  - [ ] Configure draft release flag to enable manual review before publishing (auto-draft = true)
  - [ ] Add Slack/Discord notification webhook on successful release deployment
  - [ ] Update documentation website (if applicable) with new release download links

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

### Additional PRD Requirements (Identified During Gap Analysis)

#### Timeline Scrubber - Logarithmic Scale
- [ ] Replace linear timeline slider with logarithmic scrubber spanning 13.8 billion years
  - [ ] Update CosmicTime::from_slider() to use logarithmic mapping instead of linear
  - [ ] Update CosmicTime::to_slider() to use logarithmic mapping instead of linear
  - [ ] Formula: log_slider = log10(years / min_years) / log10(max_years / min_years)
  - [ ] Map slider range [0.0, 1.0] to years [10⁻³², 13.8×10⁹]
  - [ ] Update timeline_panel_ui() slider widget to display logarithmic scale
  - [ ] Add decade tick marks to timeline (10⁻³²s, 10⁻²⁰s, 1s, 1yr, 1Myr, 1Gyr, 13.8Gyr)

#### Timeline Reverse/Replay Capability
- [ ] Implement timeline reverse playback when scrubbing backward
  - [ ] Add reverse playback mode flag to PlaybackState
  - [ ] When slider moves to previous position, pause simulation and restore particle state
  - [ ] Implement simulation snapshot system to save particle states at key intervals
  - [ ] Create snapshot history buffer (store last N snapshots)
  - [ ] Implement state restoration from nearest snapshot when scrubbing backward
  - [ ] Handle edge cases: scrubbing beyond snapshot history, scrubbing to unvisited regions
  - [ ] Sync reverse playback with timeline UI (reverse indicator, slider position)

#### Timeline Speed Integration
- [ ] Map PlaybackState.speed slider value to TimeAccumulator.acceleration
  - [ ] Implement logarithmic speed mapping: slider (0.1 to 10.0) → acceleration (1.0 to 1e12)
  - [ ] Formula: acceleration = 10^(slider_value * log10(1e12/1.0)) or similar logarithmic scale
  - [ ] Add system in sync_time_resources() to update acceleration when speed slider changes
  - [ ] Add visual feedback for current acceleration factor (display "10ⁿx" where n is exponent)
  - [ ] Document speed-to-acceleration mapping in timeline/mod.rs comments

#### Epoch Indicator UI - Temperature & Scale Factor Display
- [ ] Create epoch indicator UI panel showing era name, temperature (Kelvin), scale factor a(t), and cosmic time
  - [ ] Design epoch indicator panel layout (bevy_egui Window with title "Epoch Information")
  - [ ] Display current epoch name (e.g., "Singularity", "Inflation", "Quark-Gluon Plasma")
  - [ ] Display temperature in appropriate units (e.g., "10^27 K", "10^15 K", "3000 K")
  - [ ] Display scale factor a(t) with formatting (e.g., "a = 1.000", "a = 10^23")
  - [ ] Display cosmic time in appropriate units (seconds, minutes, years)
  - [ ] Add epoch indicator to GenesisUiPlugin registration
  - [ ] Render epoch indicator in PostUpdate schedule (same as other UI panels)

#### Temperature & Scale Factor Tracking
- [ ] Create Temperature resource struct and module in genesis-core/src/temperature.rs
  - [ ] Define Temperature struct with value: f64 (in Kelvin), min_temperature: f64 (1e-3 K absolute minimum), max_temperature: f64 (1e32 K Planck temperature) fields
  - [ ] Add derive(Debug, Clone, Copy, PartialEq, Resource) attributes for Bevy resource registration
  - [ ] Implement Default trait for Temperature with value = 1e27 K (Planck boundary initial condition)
  - [ ] Implement Temperature::new(kelvin: f64) -> Self constructor with validation clamping: kelvin.clamp(min_temperature, max_temperature)
  - [ ] Implement Temperature::set(kelvin: f64) method updating value field with clamping
  - [ ] Implement Temperature::get(&self) -> f64 getter method returning value field
  - [ ] Add TemperaturePlugin struct implementing Plugin trait with build() method that calls app.insert_resource(Temperature::default())
- [ ] Implement temperature evolution model for Singularity epoch in genesis-core/src/temperature.rs
  - [ ] Define TemperatureEvolution trait with fn update(&mut self, cosmic_time: f64, scale_factor: f64) method
  - [ ] Implement TemperatureEvolution for Temperature struct using adiabatic expansion formula: T(t) = T₀ / a(t)
  - [ ] Store initial_temperature: f64 field in Temperature struct as reference value T₀
  - [ ] Implement singularity_temperature() method returning constant T₀ for singularity epoch where a = 1
  - [ ] Add epoch parameter to update() method: fn update(&mut self, epoch: &str, scale_factor: f64) with conditional logic
  - [ ] For Singularity epoch: maintain constant temperature value = initial_temperature (no cooling before inflation)
  - [ ] For post-inflation epochs: calculate temperature = initial_temperature / scale_factor using T ∝ 1/a relationship
- [ ] Create temperature update system in genesis-core/src/temperature.rs
  - [ ] Define update_temperature() system function accepting ResMut<Temperature>, Res<CosmicTime>, Res<ScaleFactor>, Res<EpochManager> parameters
  - [ ] Query current epoch name from EpochManager via epoch_manager.current_epoch()
  - [ ] Query current scale_factor value from ScaleFactor resource
  - [ ] Call temperature.update(epoch, scale_factor) to apply appropriate evolution model
  - [ ] Handle epoch transitions by storing temperature at epoch transition point for smooth interpolation
  - [ ] Add clamp_to_physics_range() helper ensuring temperature never exceeds physical bounds (1e-3 K to 1e32 K)
  - [ ] Register update_temperature() system in CoreSchedule with .in_set(PhysicsSet::Temperature) system set
- [ ] Add temperature history tracking for visualization in genesis-core/src/temperature.rs
  - [ ] Add history: VecDeque<(f64, f64)> field to Temperature struct storing (cosmic_time, temperature) pairs
  - [ ] Set history_capacity: usize = 1000 constant for ring buffer size
  - [ ] Implement Temperature::record_state(cosmic_time: f64) method that pushes (time, value) to history and evicts oldest when full
  - [ ] Implement Temperature::get_history() -> &VecDeque<(f64, f64)> accessor for UI plotting
  - [ ] Implement Temperature::get_temperature_at_time(cosmic_time: f64) -> Option<f64> returning nearest historical temperature
  - [ ] Add recording to update_temperature() system after each update step
  - [ ] Export history data to CSV via Temperature::export_history(path: &Path) for analysis
- [ ] Implement temperature-dependent physics interactions in genesis-core/src/temperature.rs
  - [ ] Define TemperatureThresholds const struct with values: PLANCK_MAX = 1e32 K, INFLATION_START = 1e27 K, QGP = 1e15 K, NUCLEOSYNTHESIS = 1e9 K, RECOMBINATION = 3000 K, DARK_AGES_START = 300 K, DARK_AGES_END = 10 K, CMB_PRESENT = 2.725 K
  - [ ] Implement Temperature::phase(&self) -> Phase enum returning current epoch phase based on temperature thresholds
  - [ ] Define Phase enum variants: PlanckEra, Inflation, QuarkGluonPlasma, HadronEra, Nucleosynthesis, Recombination, DarkAges, CosmicDawn, PresentDay
  - [ ] Add Temperature::is_qgp(&self) -> bool helper checking temperature > 1e15 K
  - [ ] Add Temperature::is_recombining(&self) -> bool helper checking 3000 K <= temperature <= 3000.1 K
  - [ ] Add Temperature::cooling_rate(&self) -> f64 method calculating dT/dt from history data
  - [ ] Export TemperatureThresholds constants for use in other systems (particle colors, camera transitions)
- [ ] Integrate Temperature resource with epoch transitions in genesis-core/src/temperature.rs
  - [ ] Create EpochTransitionEvent struct in genesis-core/src/epoch/events.rs with old_epoch: String, new_epoch: String, transition_temperature: f64 fields
  - [ ] Add handle_epoch_transition() system in genesis-core/src/temperature.rs listening for EpochTransitionEvent
  - [ ] In handle_epoch_transition(), store snapshot of temperature at transition point for interpolation continuity
  - [ ] Implement temperature interpolation during transition: T_interpolated = lerp(T_old, T_new, progress) where progress ∈ [0, 1]
  - [ ] Add transition_duration: f64 field to Temperature struct for interpolation timing (default: 1e-6 years for inflation transition)
  - [ ] Query EpochManager for transition timing and sync temperature updates with epoch change
  - [ ] Register handle_epoch_transition() system in CoreSchedule after EpochManager transition logic
- [ ] Create ScaleFactor resource struct and module in genesis-core/src/scale_factor.rs
  - [ ] Define ScaleFactor struct with value: f64 (dimensionless expansion factor), hubble_parameter: f64 (H = ȧ/a), epoch: String fields
  - [ ] Add derive(Debug, Clone, Copy, PartialEq, Resource) attributes for Bevy resource registration
  - [ ] Implement Default trait for ScaleFactor with value = 1.0 (a=1 at Planck boundary), hubble_parameter = 0.0, epoch = "Singularity"
  - [ ] Implement ScaleFactor::new(a: f64) -> Self constructor with validation: a >= 0.001 (minimum before bounce)
  - [ ] Implement ScaleFactor::set(a: f64) method updating value field
  - [ ] Implement ScaleFactor::get(&self) -> f64 getter returning value field
  - [ ] Implement ScaleFactor::hubble(&self) -> f64 getter returning Hubble parameter
  - [ ] Add ScaleFactorPlugin struct implementing Plugin trait with build() method that calls app.insert_resource(ScaleFactor::default())
- [ ] Implement scale factor evolution models for different epochs in genesis-core/src/scale_factor.rs
  - [ ] Define ScaleFactorEvolution trait with fn update(&mut self, cosmic_time: f64, dt: f64) method
  - [ ] Implement ScaleFactorEvolution for ScaleFactor struct with epoch-specific update logic
  - [ ] Implement singularity_evolution() method: a(t) = 1.0 (constant before inflation starts)
  - [ ] Implement inflation_evolution() method: a(t) = a₀ * exp(H_inflation * t) with H_inflation ≈ 10¹⁴ GeV
  - [ ] Implement matter_dominated_evolution() method: a(t) ∝ t^(2/3) using scale_factor = scale_factor₀ * (t/t₀)^(2/3)
  - [ ] Implement radiation_dominated_evolution() method: a(t) ∝ t^(1/2) using scale_factor = scale_factor₀ * (t/t₀)^(1/2)
  - [ ] Add epoch parameter to update() method and match on epoch string to select evolution model
  - [ ] Calculate Hubble parameter H = ȧ/a via finite difference: (a_new - a_old) / (a_old * dt)
- [ ] Create scale factor update system in genesis-core/src/scale_factor.rs
  - [ ] Define update_scale_factor() system function accepting ResMut<ScaleFactor>, Res<CosmicTime>, Res<Time>, Res<EpochManager> parameters
  - [ ] Query current epoch name from EpochManager via epoch_manager.current_epoch()
  - [ ] Extract dt from Time resource (delta time in seconds converted to cosmic years via time_scale factor)
  - [ ] Call scale_factor.update(cosmic_time, dt) to apply appropriate evolution model
  - [ ] Handle epoch transitions by storing scale_factor at transition point for continuity
  - [ ] Add clamp_to_valid_range() helper ensuring scale_factor stays within physical bounds (0.001 to 1e30)
  - [ ] Register update_scale_factor() system in CoreSchedule with .in_set(PhysicsSet::Expansion) system set
- [ ] Implement Friedmann equation solver for scale factor in genesis-core/src/scale_factor.rs
  - [ ] Define FriedmannSolver struct with G: f64 (gravitational constant), rho_matter: f64, rho_radiation: f64, rho_lambda: f64 fields
  - [ ] Implement FriedmannSolver::compute_hubble_parameter(a: f64) -> f64 returning H = sqrt((8πG/3) * (ρ_m + ρ_r + ρ_Λ))
  - [ ] Define density evolution functions: ρ_m(a) = ρ_m0 / a³, ρ_r(a) = ρ_r0 / a⁴, ρ_Λ(a) = ρ_Λ0 (constant)
  - [ ] Implement FriedmannSolver::evolve_scale_factor(dt: f64) -> (f64, f64) returning (new_scale_factor, new_hubble)
  - [ ] Use Runge-Kutta 4 (RK4) integration for solving ȧ = H * a differential equation
  - [ ] Add RK4 helper method rk4_step(a: f64, dt: f64) -> f64 implementing 4th-order integration
  - [ ] Integrate FriedmannSolver into ScaleFactor struct via solver: Option<FriedmannSolver> field
  - [ ] Add enable_friedmann_solver() method to ScaleFactor for physics-accurate mode (vs simplified evolution models)
- [ ] Add scale factor history tracking for cosmological timeline in genesis-core/src/scale_factor.rs
  - [ ] Add history: VecDeque<(f64, f64)> field to ScaleFactor struct storing (cosmic_time, scale_factor) pairs
  - [ ] Set history_capacity: usize = 1000 constant for ring buffer size
  - [ ] Implement ScaleFactor::record_state(cosmic_time: f64) method pushing (time, value) to history
  - [ ] Implement ScaleFactor::get_history() -> &VecDeque<(f64, f64)> accessor for UI plotting
  - [ ] Implement ScaleFactor::get_scale_factor_at_time(cosmic_time: f64) -> Option<f64> returning nearest historical value
  - [ ] Implement ScaleFactor::calculate_cosmic_distance(a_start: f64, a_end: f64) -> f64 returning comoving distance via integral
  - [ ] Add recording to update_scale_factor() system after each update step
  - [ ] Export history data to CSV via ScaleFactor::export_history(path: &Path) for cosmological analysis
- [ ] Integrate ScaleFactor resource with epoch transitions and UI in genesis-core/src/scale_factor.rs
  - [ ] Create EpochTransitionEvent integration in handle_epoch_transition_scale_factor() system
  - [ ] In handle_epoch_transition_scale_factor(), store snapshot of scale_factor at transition point
  - [ ] Implement scale factor interpolation during transition: a_interpolated = lerp(a_old, a_new, progress)
  - [ ] Add transition_duration: f64 field to ScaleFactor struct for interpolation timing (default: 1e-32 years for inflation start)
  - [ ] Query EpochManager for transition timing and sync scale_factor updates with epoch change
  - [ ] Add ScaleFactor::format_display(&self) -> String returning formatted string for UI (e.g., "a = 1.000" or "a = 2.3e+15")
  - [ ] Implement ScaleFactor::to_scientific_notation(&self) -> (f64, i32) returning (mantissa, exponent) for UI display
  - [ ] Register handle_epoch_transition_scale_factor() system in CoreSchedule after EpochManager transition logic
  - [ ] Expose ScaleFactor resource to epoch indicator UI via Res<ScaleFactor> query in genesis-ui/src/overlay/epoch_indicator.rs

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

## Sprint 4 - Phase 4: Recombination & CMB

### Physics - Recombination
- [ ] Implement Saha equation solver for ionization fraction x_e(T) (solve for electron fraction given temperature)
  - [ ] Use hydrogen ionization equilibrium: n_e n_p / n_H = (2π m_e k T / h²)^(3/2) exp(-13.6 eV / kT)
  - [ ] Include helium ionization fraction for completeness
  - [ ] Iterate to find x_e that satisfies equilibrium
- [ ] Create IonizationState resource tracking ionization fraction x_e, free electron density, and recombination progress
  - [ ] Track recombination completion percentage (0% at T=3000K, 100% at T=2.725K)
  - [ ] Store current temperature and scale factor
- [ ] Implement photon mean free path calculation (λ_mfp = 1 / (n_e σ_T) where n_e is free electron density and σ_T is Thomson cross-section)
  - [ ] Calculate n_e from IonizationState.x_e and baryon density
  - [ ] Use Thomson cross-section σ_T ≈ 6.65×10⁻²⁹ m²
- [ ] Model temperature evolution through recombination (T ∝ 1/a for adiabatic expansion, from 3000 K to 2.725 K)
  - [ ] Implement smooth temperature transition during recombination epoch
  - [ ] Couple to ScaleFactor resource from Phase 2
- [ ] Add RecombinationEpoch plugin implementing epoch transition from Nucleosynthesis to Recombination
- [ ] Create CMB resource tracking temperature anisotropies and power spectrum
  - [ ] Store spherical harmonics coefficients a_lm up to ℓ_max ~ 1000
  - [ ] Generate from Phase 2 density perturbations via transfer function

### Visualization - Fog & CMB
- [ ] Implement volumetric fog renderer using Bevy fog or custom shader (global fog with density varying by ionization fraction)
  - [ ] Create fog density function mapping ionization fraction x_e to fog density (fog_density = x_e when x_e > 0.1, drops to 0 when x_e < 0.01)
  - [ ] Implement fog clearing system (gradually reduce fog density as x_e drops below threshold)
- [ ] Create CMB surface projection mesh (spherical shell at last-scattering surface radius ~46 billion light years)
- [ ] Generate CMB temperature anisotropy texture (2D spherical harmonics from Phase 2 density perturbations)
- [ ] Implement smooth camera transition: as recombination completes, pull camera back to reveal CMB sphere
  - [ ] Create camera pull-back animation triggered by recombination completion
  - [ ] Interpolate camera from center position to viewing position at distance ~50 billion light years
  - [ ] Apply smooth easing function (EaseInOutCubic) over 2-3 seconds transition duration
  - [ ] Orient camera to face CMB sphere center for full view
  - [ ] Add fog lift effect synchronized with camera movement (fog density decreases as camera pulls back)
  - [ ] Register camera pull-back system in PostUpdate schedule with .run_if(recombination_completed) condition
- [ ] Add CMB sphere material with temperature anisotropy mapping (color map from cold dark blue to hot bright red)
- [ ] Create LastScatteringSurface resource tracking CMB sphere parameters (radius, center position)

### UI & Analysis
- [ ] Update temperature readout to show 3000 K → 2.725 K range (display current temperature during recombination epoch)
- [ ] Create CMB angular power spectrum C_ℓ display chart (plot C_ℓ vs ℓ up to ℓ=1000)
  - [ ] Implement egui LinePlot or use external charting library
  - [ ] Scale y-axis logarithmically (C_ℓ ranges over orders of magnitude)
  - [ ] Highlight first acoustic peak at ℓ ~ 220
- [ ] Add qualitative Planck data comparison lines (overlay observational data points on simulated power spectrum)
  - [ ] Load Planck 2018 C_ℓ reference data
  - [ ] Plot as overlay line or markers on same chart
  - [ ] Add toggle to show/hide comparison data
- [ ] Implement toggle overlay for power spectrum (show/hide CMB power spectrum chart in corner)
- [ ] Add last-scattering surface indicator (display "Last Scattering Surface at ~46 Gly" label pointing to CMB sphere)
- [ ] Add CMB analysis panel with temperature readout and recombination progress
  - [ ] Display current ionization fraction x_e
  - [ ] Show recombination completion percentage
  - [ ] Link to power spectrum toggle

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 2 - Phase 2: Inflation & Quantum Seeds

### Infrastructure - genesis-physics Crate
- [ ] Implement genesis-physics crate
  - [ ] Create genesis-physics/Cargo.toml with dependencies: glam (for vector math), nalgebra (for scientific linear algebra), wgpu (for GPU compute), serde (for serialization)
  - [ ] Create genesis-physics/src/lib.rs with module declarations for physics systems (gravity, inflaton, perturbations, nucleosynthesis)
  - [ ] Add GenesisPhysicsPlugin struct implementing Plugin trait with build() method that registers physics systems
  - [ ] Add genesis-physics to workspace Cargo.toml members list: "genesis-physics"
  - [ ] Add genesis-physics dependency to main Cargo.toml: genesis-physics = { path = "genesis-physics" }

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
- [ ] Add QGPEpoch plugin implementing the Quark-Gluon Plasma epoch with temperature-dependent rendering
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

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 3 - Phase 3: Nucleosynthesis & First Elements

### Physics - Nuclear Reaction Network
- [ ] Define NuclearReaction struct with reactants (Vec<Element>), products (Vec<Element>), and reaction rate coefficient function k(T)
- [ ] Define Element enum for nuclear species (Neutron, Proton, Deuterium, Tritium, Helium3, Helium4, Lithium7, Beryllium7)
  - [ ] Implement Display trait for Element enum (convert to string representation for UI)
  - [ ] Implement FromStr trait for Element enum (parse from configuration or data files)
- [ ] Create 12-species nuclear reaction network data structure with ~50 reactions from BBN network
  - [ ] Define network as HashMap or Vec of NuclearReaction entries
  - [ ] Include reverse reactions for thermodynamic equilibrium
- [ ] Implement NACRE II reaction rate compilation lookup table (temperature-dependent rates λ(T) in log-log space)
  - [ ] Create rate lookup table as 2D array or Vec of (T, λ) pairs
  - [ ] Cover temperature range T = 10⁸ K to 10¹⁰ K
- [ ] Implement reaction rate interpolation function (linear interpolation in log space for T and λ: log(λ) = Lerp(log(T1), log(T2), log(λ1), log(λ2)))
  - [ ] Clamp temperature to table bounds before interpolation
  - [ ] Handle extrapolation for temperatures outside table range
- [ ] Implement stiff ODE solver using implicit Rosenbrock method (2nd order with adaptive step size)
  - [ ] Define Rosenbrock coefficients (a, b, c, gamma for 2nd order method)
  - [ ] Implement adaptive step size control (error estimation and step adjustment)
  - [ ] Handle failed steps with reduced step size
- [ ] Define Jacobian matrix for nuclear reaction network (∂f_i/∂Y_j where f_i = dY_i/dt)
  - [ ] Implement automatic Jacobian construction from reaction network
  - [ ] Optimize for sparse matrix structure (most species interact with few others)
- [ ] Implement nuclear reaction network update system (solve ODE system dY_i/dt = Σ (production - destruction) each frame)
  - [ ] Time-step the network each frame based on cosmic time acceleration
  - [ ] Scale reaction rates by temperature from Temperature resource
- [ ] Add NuclearComposition resource tracking element abundances Y_i for each species (mass fractions)
  - [ ] Initialize with primordial composition (n: p = 1:7, Y_p ≈ 0 at t=0)
  - [ ] Update Y_i values as network evolves
- [ ] Create NucleosynthesisEpoch plugin implementing the Nucleosynthesis epoch (3 min - 20 min)
- [ ] Add reaction rate validation against NACRE II reference values at T = 10⁹ K
  - [ ] Create unit test comparing simulated rates to reference values
  - [ ] Output deviation percentage for each reaction

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
- [ ] Create ValidationOverlayPanel UI component in genesis-ui/src/overlay/validation.rs
  - [ ] Define ValidationOverlayPanel struct with visibility: bool, position: egui::Pos2, size: egui::Vec2, is_collapsed: bool fields for UI configuration
  - [ ] Implement ValidationOverlayPanel::new() constructor with default position at top-right overlay (Pos2::new(screen_width - 380.0, 400.0))
  - [ ] Create validation_overlay_panel_ui() function accepting egui::Ui, Res<NuclearComposition>, Res<CosmologyConfig>, Res<ValidationData> parameters
  - [ ] Design panel layout using egui::Window::new("Validation: Simulation vs Observed") with collapsible frame and alpha transparency (0.9)
  - [ ] Add section headers using egui::CollapsingHeader::new("Nucleosynthesis Abundances") and egui::CollapsingHeader::new("CMB Power Spectrum")
  - [ ] Register panel rendering system in GenesisUiPlugin::build() via add_systems(PostUpdate, validation_overlay_panel_ui.run_if(show_validation))
- [ ] Implement abundance comparison chart in ValidationOverlayPanel in genesis-ui/src/overlay/validation.rs
  - [ ] Create BarChart struct in genesis-ui/src/overlay/validation.rs with data: Vec<BarEntry> where BarEntry has label: String, simulated: f64, observed: f64, deviation: f64 fields
  - [ ] Query NuclearComposition resource for simulated abundances (Y_p: Helium4, D/H: Deuterium, ³He/⁴He, ⁷Li/H)
  - [ ] Define OBSERVATIONAL_CONSTANTS const with Planck 2018 values: Y_p_obs = 0.245 ± 0.003, D/H_obs = (2.527 ± 0.030)×10⁻⁵, ³He/⁴He_obs = (1.04 ± 0.04)×10⁻⁴, ⁷Li/H_obs = (1.58 ± 0.07)×10⁻¹⁰
  - [ ] Calculate deviation percentage: deviation = (simulated - observed) / observed * 100.0
  - [ ] Render side-by-side bar chart using egui::plot::Plot::new("abundance_comparison") with simulated values (blue bars) and observed values (red bars)
  - [ ] Add Y_p ≈ 0.245 horizontal reference line using egui::plot::Line::new(plot::plot::PlotPoints::from_iter(vec![(0.0, 0.245), (4.0, 0.245)])) with dashed style and gray color
  - [ ] Display abundance values in scientific notation for small values (e.g., 2.527e-5 for D/H)
- [ ] Implement deviation visualization and accuracy display in ValidationOverlayPanel in genesis-ui/src/overlay/validation.rs
  - [ ] Create DeviationIndicator struct with parameter_name: String, simulated: f64, observed: f64, deviation_percent: f64, is_within_tolerance: bool fields
  - [ ] Define TOLERANCE_THRESHOLD const: f64 = 5.0 (5% deviation tolerance for "good" agreement)
  - [ ] Implement color-coding for deviations: green if |deviation| < 1%, yellow if 1% ≤ |deviation| < 5%, red if |deviation| ≥ 5%
  - [ ] Display deviation percentages using egui::Label::new().text_color(color) where color depends on tolerance
  - [ ] Add ✓ checkmark icon for within-tolerance parameters using egui::RichText::new("✓").color(egui::Color32::GREEN)
  - [ ] Add ✗ cross icon for out-of-tolerance parameters using egui::RichText::new("✗").color(egui::Color32::RED)
  - [ ] Implement overall accuracy score calculation: score = Σ (1 - |deviation| / 100) / N_parameters displayed as percentage (e.g., "92.3% agreement")
- [ ] Implement CMB power spectrum comparison in ValidationOverlayPanel in genesis-ui/src/overlay/validation.rs
  - [ ] Create PowerSpectrumChart struct in genesis-ui/src/overlay/validation.rs with simulated: Vec<(f64, f64)>, observed: Vec<(f64, f64)> fields storing (ℓ, C_ℓ) data points
  - [ ] Query CMB resource for simulated angular power spectrum C_ℓ_sim(ℓ) for ℓ = 2 to 2500
  - [ ] Load Planck 2018 observational data points C_ℓ_obs(ℓ) from embedded constants or CSV file in genesis-core/src/cmb/data.rs
  - [ ] Render dual-line plot using egui::plot::Plot::new("power_spectrum") with simulated line (blue) and observed points (red markers)
  - [ ] Add first acoustic peak marker at ℓ ≈ 220 with vertical dashed line and "ℓ=220 (1st peak)" label
  - [ ] Display chi-squared statistic: χ² = Σ [(C_ℓ_sim - C_ℓ_obs)² / σ_ℓ²] summed over ℓ range
  - [ ] Calculate p-value from chi-squared distribution to quantify statistical agreement
- [ ] Add toggleable overlay control and persistence in ValidationOverlayPanel in genesis-ui/src/overlay/validation.rs
  - [ ] Add show_validation: bool field to OverlayState struct in genesis-ui/src/overlay/mod.rs
  - [ ] Add toggle button in overlay settings panel using egui::Checkbox::new(&mut overlay.show_validation, "Show Validation Overlay")
  - [ ] Implement keyboard shortcut (e.g., Ctrl+V) to toggle validation visibility via input event handling
  - [ ] Persist show_validation preference to Config via ConfigResource when changed using Config::save_to_path()
  - [ ] Add validation visibility indicator in main UI header showing "VALIDATION: ON" with green color when active
  - [ ] Implement auto-hide logic when simulation resets (clear validation data until nucleosynthesis completes)
  - [ ] Add "Clear Validation Data" button in panel to manually reset comparison state
- [ ] Export validation data and integrate with simulation workflow in genesis-ui/src/overlay/validation.rs
  - [ ] Implement export_validation_data() function writing simulated vs observed comparisons to CSV file
  - [ ] Create CSV format with columns: Parameter, Simulated, Observed, Deviation_%, Within_Tolerance, Notes
  - [ ] Add "Export CSV" button in ValidationOverlayPanel using egui::Button::new("Export CSV")
  - [ ] Implement file dialog using egui_file crate for choosing export path
  - [ ] Add timestamp to export filename: validation_YYYY-MM-DD_HH-MM-SS.csv
  - [ ] Export CMB power spectrum data to separate CSV file with columns: l, C_l_simulated, C_l_observed, residual
  - [ ] Integrate with EpochManager to automatically capture validation snapshots at epoch transitions (e.g., at nucleosynthesis completion T ≈ 10⁹ K)
  - [ ] Add validation progress indicator showing "Validation data available at: Nucleosynthesis (T=10⁹ K), Recombination (T=3000 K), CMB (T=2.725 K)"

### Testing
- [ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date.

---

## Sprint 4 - Phase 4: Recombination & CMB

### Physics - Recombination
- [ ] Implement Saha equation solver for ionization fraction x_e(T) (solve for electron fraction given temperature)
  - [ ] Use hydrogen ionization equilibrium: n_e n_p / n_H = (2π m_e k T / h²)^(3/2) exp(-13.6 eV / kT)
  - [ ] Include helium ionization fraction for completeness
  - [ ] Iterate to find x_e that satisfies equilibrium
- [ ] Create IonizationState resource tracking ionization fraction x_e, free electron density, and recombination progress
  - [ ] Track recombination completion percentage (0% at T=3000K, 100% at T=2.725K)
  - [ ] Store current temperature and scale factor
- [ ] Implement photon mean free path calculation (λ_mfp = 1 / (n_e σ_T) where n_e is free electron density and σ_T is Thomson cross-section)
  - [ ] Calculate n_e from IonizationState.x_e and baryon density
  - [ ] Use Thomson cross-section σ_T ≈ 6.65×10⁻²⁹ m²
- [ ] Model temperature evolution through recombination (T ∝ 1/a for adiabatic expansion, from 3000 K to 2.725 K)
  - [ ] Implement smooth temperature transition during recombination epoch
  - [ ] Couple to ScaleFactor resource from Phase 2
- [ ] Add RecombinationEpoch plugin implementing epoch transition from Nucleosynthesis to Recombination
- [ ] Create CMB resource tracking temperature anisotropies and power spectrum
  - [ ] Store spherical harmonics coefficients a_lm up to ℓ_max ~ 1000
  - [ ] Generate from Phase 2 density perturbations via transfer function

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
  - [ ] Implement egui LinePlot or use external charting library
  - [ ] Scale y-axis logarithmically (C_ℓ ranges over orders of magnitude)
  - [ ] Highlight first acoustic peak at ℓ ~ 220
- [ ] Add qualitative Planck data comparison lines (overlay observational data points on simulated power spectrum)
  - [ ] Load Planck 2018 C_ℓ reference data
  - [ ] Plot as overlay line or markers on same chart
  - [ ] Add toggle to show/hide comparison data
- [ ] Implement toggle overlay for power spectrum (show/hide CMB power spectrum chart in corner)
- [ ] Add last-scattering surface indicator (display "Last Scattering Surface at ~46 Gly" label pointing to CMB sphere)
- [ ] Add CMB analysis panel with temperature readout and recombination progress
  - [ ] Display current ionization fraction x_e
  - [ ] Show recombination completion percentage
  - [ ] Link to power spectrum toggle

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
  - [ ] Sample dark matter particle positions from perturbation field
  - [ ] Set initial velocities from Zel'dovich displacement
  - [ ] Tag particles as dark_matter vs baryonic
- [ ] Implement baryonic particle coupling to dark matter
  - [ ] Apply gravitational forces from dark matter to baryons
  - [ ] Model drag/buffering effects during recombination
- [ ] Create adaptive level-of-detail system (particle splitting/merging)
  - [ ] Split particles in high-density regions (increase local resolution)
  - [ ] Merge particles in low-density voids (reduce computational cost)
  - [ ] Maintain total mass conservation during split/merge
- [ ] Implement Friends-of-Friends halo finder
  - [ ] Define linking length ℓ (fraction of mean particle separation)
  - [ ] Group particles into connected components using linking length
  - [ ] Optimize with union-find data structure for O(N) grouping
- [ ] Add halo property calculation (mass, center-of-mass, radius)
  - [ ] Calculate virial radius from particle velocities
  - [ ] Compute halo mass from particle count × particle mass
  - [ ] Determine center-of-mass as weighted average
- [ ] Create HaloCatalog resource tracking discovered halos and their properties
  - [ ] Store halos as Vec<HaloEntry> with unique IDs
  - [ ] Update catalog in real-time as halos merge/split
  - [ ] Track halo merger history tree for galaxy morphology
- [ ] Implement CosmicWeb resource tracking filament and void detection
  - [ ] Detect filaments using particle density thresholding
  - [ ] Identify voids as regions with particle density below threshold
  - [ ] Store filament and void boundary information for visualization

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
  - [ ] Define Kennicutt-Schmidt law: Σ_SFR = K × Σ_gas^N (typically N ≈ 1.4)
  - [ ] Calculate surface density Σ_gas from SPH gas particles in cell
  - [ ] Convert Σ_SFR to star particle formation probability per time step
  - [ ] Implement density threshold for star formation (n_H > critical value)
- [ ] Create dense gas → star particle conversion
  - [ ] Spawn star particles from gas particles in dense regions
  - [ ] Transfer mass from gas to stars
  - [ ] Preserve momentum during conversion
- [ ] Implement Pop III star formation in early halos
  - [ ] Identify halos with no metal content (first star formation)
  - [ ] Set Pop III star properties (higher mass, lower metallicity)
  - [ ] Model Pop III supernova feedback on surrounding gas
- [ ] Add first light sources as bright point lights
  - [ ] Attach PointLight components to new star particles
  - [ ] Set light intensity based on star mass and age
  - [ ] Enable shadows for early galaxy visualization

### Reionization
- [ ] Implement ionization front expansion (signed-distance-field bubbles)
- [ ] Create bubbles around star-forming halos
- [ ] Implement bubble overlap and merging
- [ ] Model neutral gas consumption

### Visualization - Galaxies
- [ ] Create galaxy billboard sprites
  - [ ] Design galaxy sprite textures (elliptical, spiral, irregular morphologies)
    - [ ] Generate elliptical galaxy texture with smooth radial gradient (bright center, dim edges)
    - [ ] Generate spiral galaxy texture with two-armed spiral structure using logarithmic spiral equation
    - [ ] Generate irregular galaxy texture with asymmetric clumpy distribution
    - [ ] Store textures in assets/galaxies/ directory: elliptical.png, spiral.png, irregular.png
    - [ ] Create galaxy_texture_atlas combining all three types for efficient GPU access
  - [ ] Create billboard entities facing camera for each galaxy
    - [ ] Define GalaxyBillboard component with sprite_type: GalaxyType, size: f32, brightness: f32 fields
    - [ ] Implement update_galaxy_billboards() system that rotates billboards to face camera each frame
    - [ ] Use camera.forward and camera.up vectors to compute billboard orientation quaternion
    - [ ] Register billboard system in PostUpdate schedule after camera update
  - [ ] Apply textures based on galaxy type
    - [ ] Define GalaxyType enum with variants: Elliptical, Spiral, Irregular
    - [ ] Create galaxy_billboard_material with texture uniform binding to sprite_atlas
    - [ ] Map GalaxyType variants to texture UV coordinates in material shader
    - [ ] Implement sprite_uv_offset calculation based on GalaxyType for atlas lookup
- [ ] Implement halo mass threshold for galaxy rendering
  - [ ] Define MIN_HALO_MASS: f64 = 1e8 (10⁸ M☉ minimum mass for galaxy visibility)
  - [ ] Create GalaxyVisibilityQuery system filtering HaloCatalog entries above threshold
  - [ ] For each halo with mass > MIN_HALO_MASS, spawn GalaxyBillboard entity at halo.center_of_mass
  - [ ] Scale galaxy brightness with halo mass using brightness = log10(halo_mass / MIN_HALO_MASS)
  - [ ] Set galaxy sprite size based on halo mass: size = base_size * (halo_mass / MIN_HALO_MASS)^(1/3)
  - [ ] Only render galaxies above threshold (performance optimization)
    - [ ] Add visibility_query component to GalaxyBillboard for frustum culling
    - [ ] Implement frustum culling using camera view matrix and bounding sphere test
    - [ ] Despawn galaxies when halo mass falls below threshold during merger destruction
- [ ] Generate composite galaxy sprites based on merger history
  - [ ] Access halo merger tree from HaloCatalog
    - [ ] Add merger_history field to HaloEntry storing Vec<MergeEvent> with merge_time, merger_mass fields
    - [ ] Update merger_history each time halos merge (track cumulative merger count)
    - [ ] Implement update_merger_tree() system called after Friends-of-Friends grouping each frame
  - [ ] Set galaxy morphology based on merger count:
    - [ ] Compute total_mergers = merger_history.len() for each halo
    - [ ] Map merger count to GalaxyType: 0-1 mergers → Elliptical, 2-5 mergers → Spiral, 5+ mergers → Irregular
    - [ ] Update GalaxyBillboard.sprite_type field when merger count crosses threshold
    - [ ] Add morphological_transition() system handling sprite type changes with visual crossfade
  - [ ] Add merger_count field to GalaxyBillboard component for morphology tracking
  - [ ] Apply color based on stellar population age
    - [ ] Add stellar_age: f64 field to HaloEntry (computed from first star formation time)
    - [ ] Define color_from_age(age: f64) -> Color function: age < 100Myr → blue, age < 1Gyr → yellow-white, age < 5Gyr → white, age >= 5Gyr → yellow-red
    - [ ] Apply stellar population color to galaxy sprite via uniform binding
    - [ ] Implement color transition when stellar_age crosses age thresholds using interpolation
- [ ] Add ionization bubble visualization (translucent spheres)
  - [ ] Create sphere mesh with transparent material
    - [ ] Use Bevy sphere primitive with high tessellation (32 segments) for smooth rendering
    - [ ] Create IonizationBubbleMaterial with alpha: f32, bubble_color: Color, radius: f32 uniform bindings
    - [ ] Configure material to be back-face culled (render only outer shell) and double-sided
    - [ ] Set blend mode to AdditiveBlend for translucent glow effect
  - [ ] Scale sphere to ionization front radius
    - [ ] Define ionization_radius calculation: r = (M_star / M_crit)^(1/3) * r_base where M_star is total star mass in halo
    - [ ] Implement update_ionization_bubbles() system recalculating radius each frame based on star particle mass
    - [ ] Animate radius expansion: r(t) = r_final * (1 - exp(-t/τ)) where τ is bubble growth timescale
  - [ ] Render around star-forming halos
    - [ ] Query halos with star particles (halo.star_particle_count > 0)
    - [ ] Spawn IonizationBubble entity at halo.center_of_mass with calculated radius
    - [ ] Despawn ionization bubbles when halo stops forming stars (no new star particles for 10 Myr)
    - [ ] Implement bubble overlap rendering using additive blending for merged reionization regions

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
- [ ] Implement GPU timer query system in genesis-render/src/profiling/gpu.rs
  - [ ] Define GpuTimer struct with name: String, query_sets: Vec<wgpu::QuerySet>, active_query: usize, timestamp_resolution: f32 fields
  - [ ] Create GpuProfiler resource with timers: HashMap<String, GpuTimer>, frame_time: f32, max_frame_time: f32 fields
  - [ ] Implement GpuTimer::new(device: &wgpu::Device, name: &str) -> Self constructor creating 2 query sets for ping-pong buffering
  - [ ] Implement GpuTimer::begin() method that begins timestamp query using command_encoder.write_timestamp()
  - [ ] Implement GpuTimer::end(device: &wgpu::Device) method that ends timestamp query and resolves timestamps to buffer
  - [ ] Implement GpuTimer::get_elapsed_ms(&self, queue: &wgpu::Queue) -> Option<f32> returning elapsed time in milliseconds or None if queries unavailable
  - [ ] Add derive(Debug, Resource) traits to GpuProfiler for Bevy resource registration
  - [ ] Register GpuProfiler as Bevy resource via GpuProfilingPlugin with insert_resource(GpuProfiler::default()) in build()
- [ ] Add compute shader profiling in genesis-render/src/particle/mod.rs
  - [ ] Create update_particle_system_profiling() system wrapping existing particle update system with GPU timer queries
  - [ ] Register compute shader timer as "particle_update" in GpuProfiler.timers HashMap
  - [ ] Insert GpuTimer::begin() call before dispatching particle compute shader
  - [ ] Insert GpuTimer::end() call after particle compute shader completes
  - [ ] Create density_field_compute_profiling() system profiling Gaussian random field generation FFT compute shaders
  - [ ] Register density field timer as "density_field" in GpuProfiler.timers HashMap
  - [ ] Create gravity_compute_profiling() system profiling N-body gravity compute shaders (when implemented)
  - [ ] Register gravity timer as "gravity_compute" in GpuProfiler.timers HashMap
  - [ ] Profile SPH hydrodynamics compute shaders (when implemented) as "sph_compute" timer
  - [ ] Add query_set recreation on adapter change to handle GPU query set limits (typically 64-256 timer queries per set)
- [ ] Profile render passes in genesis-render/src/profiling/gpu.rs
  - [ ] Create render_pass_profiling() system that wraps render graph passes with GPU timer queries
  - [ ] Register geometry pass timer as "render_geometry" covering opaque mesh rendering
  - [ ] Register particle rendering timer as "render_particles" covering instanced point sprite drawing
  - [ ] Register post-processing timer as "render_post_process" covering bloom, tone mapping, FXAA
  - [ ] Register UI rendering timer as "render_ui" covering egui overlay rendering
  - [ ] Implement nested timer scoping: each timer can have child timers for sub-pass profiling
  - [ ] Create timer tree structure where parent_timer.children: Vec<String> references child timer names
  - [ ] Calculate timer hierarchy percentages: child_percent = child_time / parent_time * 100
  - [ ] Register render_pass_profiling() system in Render schedule before RenderSet::PassMain
- [ ] Display GPU time metrics in performance overlay in genesis-ui/src/profiling/overlay.rs
  - [ ] Create GpuProfilingPanel struct with show_gpu_times: bool, show_timer_tree: bool, highlight_slow_timers: bool fields
  - [ ] Implement GpuProfilingPanel::new() constructor with default position at bottom-left overlay
  - [ ] Create gpu_profiling_panel_ui() function accepting egui::Ui, Res<GpuProfiler> parameters
  - [ ] Render total GPU frame time using egui::Label::new(format!("GPU Frame: {:.2} ms", profiler.frame_time))
  - [ ] Render per-timer breakdown using egui::Table::new("gpu_timers") with columns: Timer Name, Time (ms), % of Total, Delta (ms)
  - [ ] Color-code timer rows: green for <1ms, yellow for 1-5ms, red for >5ms using egui::Color32
  - [ ] Add bar chart visualization next to timer names using egui::ProgressBar showing relative time contribution
  - [ ] Implement timer tree display with indent-based hierarchy using egui::indent() for child timers
  - [ ] Add timer filtering via search box showing only timers matching query string
  - [ ] Add "Slow Timers Only" toggle filtering timers where time > 3ms
- [ ] Track GPU memory usage in genesis-render/src/profiling/memory.rs
  - [ ] Define GpuMemoryTracker struct with total_memory: u64, used_memory: u64, peak_memory: u64, allocations: Vec<GpuAllocation> fields
  - [ ] Define GpuAllocation struct with name: String, size: u64, buffer_id: Option<wgpu::Buffer>, texture_id: Option<wgpu::Texture> fields
  - [ ] Create GpuMemoryTracker::new(device: &wgpu::Device) -> Self constructor querying device limits via device.limits()
  - [ ] Implement GpuMemoryTracker::allocate_buffer(&mut self, name: &str, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer tracking buffer allocation
  - [ ] Implement GpuMemoryTracker::allocate_texture(&mut self, name: &str, size: u64, format: wgpu::TextureFormat) -> wgpu::Texture tracking texture allocation
  - [ ] Implement GpuMemoryTracker::deallocate(&mut self, id: wgpu::Id) removing allocation from list and updating used_memory
  - [ ] Implement GpuMemoryTracker::get_usage_percent(&self) -> f32 returning (used_memory / total_memory) * 100.0
  - [ ] Register GpuMemoryTracker as Bevy resource via GpuProfilingPlugin with insert_resource(GpuMemoryTracker::new(&device)) in build()
  - [ ] Add memory leak detection: log warning in drop() if allocations remain when tracker destroyed
- [ ] Implement memory budget enforcement in genesis-render/src/profiling/memory.rs
  - [ ] Define MEMORY_BUDGET_PERCENT: f32 = 0.85 (use 85% of total GPU memory, leave 15% buffer)
  - [ ] Define MEMORY_WARNING_PERCENT: f32 = 0.70 (warning at 70% usage)
  - [ ] Create enforce_memory_budget() system accepting ResMut<GpuMemoryTracker>, ResMut<ParticleConfig> parameters
  - [ ] Check if memory_usage_percent > MEMORY_BUDGET_PERCENT in enforce_memory_budget() system
  - [ ] If budget exceeded, reduce particle count by 20% using particle_config.initial_count = (current_count * 0.8) as usize
  - [ ] Implement adaptive LOD adjustment: set particle_config.lod_distance = particle_config.lod_distance * 1.5 to reduce rendering load
  - [ ] Log budget enforcement event: "Memory budget exceeded ({}%). Reducing particle count to {}. Using LOD distance {}"
  - [ ] Add memory_warning flag to GpuMemoryTracker triggering UI alert dialog when memory_usage_percent > MEMORY_WARNING_PERCENT
  - [ ] Create memory_warning_ui() system rendering egui::Window::with_title("⚠️ GPU Memory Warning") with message "VRAM usage at {:.0}%. Approaching limit. Particle count reduced to maintain performance."
  - [ ] Implement particle count restoration when memory available: if memory_usage_percent < 0.5, incrementally increase particle_count back to target
  - [ ] Register enforce_memory_budget() system in PostUpdate schedule with .run_if(detect_memory_pressure)
- [ ] Integrate GPU profiling with particle LOD system in genesis-render/src/particle/mod.rs
  - [ ] Create adapt_particle_lod() system accepting Res<GpuProfiler>, ResMut<ParticleConfig>, ResMut<CameraState> parameters
  - [ ] Query GPU profiler for current_frame_time using GpuProfiler::get_frame_time()
  - [ ] Define TARGET_FRAME_TIME: f32 = 16.67 ms (60 FPS target)
  - [ ] Define MIN_LOD_DISTANCE: f32 = 50.0, MAX_LOD_DISTANCE: f32 = 2000.0 for LOD adjustment bounds
  - [ ] Implement adaptive LOD logic: if gpu_frame_time > 20ms, increase lod_distance by 10% (reduce render load), if gpu_frame_time < 14ms, decrease lod_distance by 5% (increase quality)
  - [ ] Clamp lod_distance to [MIN_LOD_DISTANCE, MAX_LOD_DISTANCE] range
  - [ ] Update ParticleConfig.lod_distance field and trigger particle LOD system recalculation
  - [ ] Add LOD adjustment logging: "GPU frame time {:.2}ms. Adjusting LOD distance from {:.1} to {:.1}"
  - [ ] Implement particle count adjustment based on GPU time: if gpu_frame_time > 30ms, reduce particle_count by 10%, if gpu_frame_time < 12ms, increase particle_count by 5%
  - [ ] Register adapt_particle_lod() system in PostUpdate schedule with .run_if(interval_seconds(0.5)) for gradual adjustment every 0.5 seconds
  - [ ] Add GPU-based LOD mode toggle in UI using egui::Checkbox::new("Adaptive GPU LOD") enabling/disabling adaptive adjustments
  - [ ] Document GPU-based LOD thresholds for different hardware tiers: GTX 1660 (60FPS @ 1M particles), RTX 3060 (60FPS @ 3M particles), RTX 3080 (60FPS @ 5M particles)

### Cinematic Mode
- [ ] Create CameraKeyframe data structure in genesis-render/src/camera/cinematic.rs
  - [ ] Define CameraKeyframe struct with position: Vec3, rotation: Quat, cosmic_time: f64, duration: f64, easing: EasingType, epoch: String fields
  - [ ] Add derive(Debug, Clone, Deserialize, Serialize) traits for TOML loading and config persistence
  - [ ] Implement CameraKeyframe::new(position: Vec3, rotation: Quat, cosmic_time: f64) -> Self constructor with default easing = EasingType::EaseInOutCubic
  - [ ] Add CameraKeyframe::default() returning keyframe at origin looking forward: position = Vec3::ZERO, rotation = Quat::IDENTITY, cosmic_time = 0.0
  - [ ] Implement CameraKeyframe::with_duration(mut self, duration: f64) -> Self setter for keyframe transition duration
  - [ ] Implement CameraKeyframe::with_easing(mut self, easing: EasingType) -> Self setter for easing function selection
  - [ ] Add CameraKeyframe::is_at_epoch(&self, epoch_name: &str) -> bool helper for epoch-based keyframe lookup
- [ ] Implement CinematicMode resource in genesis-render/src/camera/cinematic.rs
  - [ ] Define CinematicMode struct with keyframes: Vec<CameraKeyframe>, current_keyframe: usize, playback_progress: f64, playback_state: CinematicPlaybackState fields
  - [ ] Define CinematicPlaybackState enum with variants: Stopped, Playing, Paused, Transitioning (for camera interpolation phases)
  - [ ] Add derive(Debug, Clone, Resource) traits for Bevy resource registration
  - [ ] Implement CinematicMode::new() constructor with empty keyframes Vec and playback_state = CinematicPlaybackState::Stopped
  - [ ] Implement CinematicMode::add_keyframe(&mut self, keyframe: CameraKeyframe) method inserting keyframe and sorting by cosmic_time
  - [ ] Implement CinematicMode::get_current_keyframe(&self) -> Option<&CameraKeyframe> returning current keyframe or None if stopped
  - [ ] Implement CinematicMode::get_next_keyframe(&self) -> Option<&CameraKeyframe> returning next keyframe for interpolation target
  - [ ] Register CinematicMode as Bevy resource via CinematicModePlugin with insert_resource(CinematicMode::new()) in build()
- [ ] Create camera path interpolation system in genesis-render/src/camera/cinematic.rs
  - [ ] Define interpolate_camera_path() system function accepting ResMut<CameraTransform>, ResMut<CinematicMode>, Res<Time>, Res<CosmicTime> parameters
  - [ ] Query current and next keyframes from CinematicMode using current_keyframe index and index+1
  - [ ] Calculate interpolation parameter t = playback_progress / duration where duration is from current_keyframe.duration
  - [ ] Apply easing function: eased_t = current_keyframe.easing.ease(t) using EasingFunction trait
  - [ ] Interpolate position: new_position = lerp(current_keyframe.position, next_keyframe.position, eased_t)
  - [ ] Interpolate rotation: new_rotation = slerp(current_keyframe.rotation, next_keyframe.rotation, eased_t)
  - [ ] Update CameraTransform with new_position and new_rotation via CameraTransform::set_position() and CameraTransform::set_rotation()
  - [ ] Increment playback_progress by dt from Time resource each frame during playback
  - [ ] Trigger keyframe transition when playback_progress >= current_keyframe.duration using CinematicMode::advance_to_next_keyframe()
  - [ ] Register interpolate_camera_path() system in PostUpdate schedule with .run_if(|mode: Res<CinematicMode>| mode.playback_state == CinematicPlaybackState::Playing)
- [ ] Implement epoch narration system in genesis-ui/src/cinematic/narration.rs
  - [ ] Define NarrationEntry struct with epoch: String, start_time: f64, duration: f64, title: String, body_text: String, fade_in: f64, fade_out: f64 fields
  - [ ] Define NarrationQueue struct with entries: Vec<NarrationEntry>, current_entry: Option<usize>, display_time: f64, opacity: f32 fields
  - [ ] Implement NarrationEntry::default() returning empty narration with default fade times (fade_in = 1.0s, fade_out = 2.0s)
  - [ ] Implement NarrationQueue::add_entry(&mut self, entry: NarrationEntry) method inserting narration sorted by start_time
  - [ ] Define epoch narration texts for each epoch in NARRATION_TEXTS const:
    - Singularity: "At the beginning of time itself, all matter, energy, and spacetime were compressed into an infinitely dense singularity. Temperature exceeds 10³² K."
    - Inflation: "In a fraction of a second, the universe undergoes exponential expansion, inflating by a factor of 10²⁶. This smooths out any irregularities."
    - QGP: "The universe cools to 10¹⁵ K, allowing quarks and gluons to form a primordial soup. Matter and antimatter annihilate, leaving a slight excess of matter."
    - Nucleosynthesis: "At 10⁹ K, atomic nuclei begin to form. The universe is now 3 minutes old, creating hydrogen, helium, and trace lithium."
    - Recombination: "After 380,000 years, electrons bind to nuclei forming neutral atoms. The universe becomes transparent, releasing the cosmic microwave background."
    - Dark Ages: "For 100 million years, the universe is shrouded in darkness. Gravity slowly pulls matter together, forming the first cosmic structures."
    - Cosmic Dawn: "The first stars ignite, bringing light to the cosmos. These brilliant stars heat and reionize the surrounding hydrogen gas."
  - [ ] Implement narration display system updating opacity based on fade curves: opacity = 1.0 during body display, fading in/out using easing functions
  - [ ] Register NarrationQueue as Bevy resource via NarrationPlugin with insert_resource(NarrationQueue::default()) in build()
- [ ] Create NarrationPanel UI component in genesis-ui/src/cinematic/narration.rs
  - [ ] Define NarrationPanel struct with position: egui::Pos2, width: f32, background_color: egui::Color32, title_color: egui::Color32, body_color: egui::Color32 fields
  - [ ] Implement NarrationPanel::new() constructor with default position at bottom-center (Pos2::new(screen_width/2 - 400.0, screen_height - 200.0)) and width = 800.0
  - [ ] Create narration_panel_ui() function accepting egui::Ui, Res<NarrationQueue>, Res<CosmicTime> parameters
  - [ ] Design panel layout using egui::Window::new() with no_frame = true, no_title = true, movable = false for cinematic overlay style
  - [ ] Render current narration entry with title using egui::RichText::new(entry.title).size(24.0).color(title_color).strong()
  - [ ] Render body text using egui::RichText::new(entry.body_text).size(18.0).color(body_color).wrap(true) with text wrapping enabled
  - [ ] Apply opacity to panel background using egui::Color32::from_rgba_unmultiplied(background_color.r, background_color.g, background_color.b, (opacity * 255.0) as u8)
  - [ ] Add smooth fade transitions using egui::ctx().animate_value() for opacity interpolation
  - [ ] Register narration_panel_ui() system in PostUpdate schedule with .run_if(|mode: Res<CinematicMode>| mode.playback_state == CinematicPlaybackState::Playing)
- [ ] Implement cinematic playback controls in genesis-ui/src/cinematic/controls.rs
  - [ ] Define CinematicControls struct with show_controls: bool, auto_play: bool, loop_playback: bool fields
  - [ ] Create cinematic_controls_ui() function accepting egui::Ui, ResMut<CinematicMode>, ResMut<NarrationQueue>, ResMut<PlaybackState>, Res<EpochManager> parameters
  - [ ] Add "Play Cinematic Mode" button using egui::Button::new("▶ Play Full Story").fill(egui::Color32::from_rgb(100, 200, 100)) in main UI header
  - [ ] Implement on_play_cinematic() handler calling CinematicMode::play(), resetting CosmicTime to t=0, resetting EpochManager to Singularity epoch
  - [ ] Add pause/resume button during playback using egui::Button::new("⏸ Pause") or "▶ Resume" based on playback_state
  - [ ] Add "Stop" button using egui::Button::new("⏹ Stop").fill(egui::Color32::from_rgb(200, 100, 100)) to exit cinematic mode and return to interactive camera
  - [ ] Add playback progress bar using egui::ProgressBar showing current_keyframe_index / (keyframes.len() - 1) progress through keyframe sequence
  - [ ] Add time display showing current cosmic_time in narration panel using format_cosmic_time() from CosmicTime resource
  - [ ] Add chapter/epoch indicator showing current narration epoch name in upper-left corner of narration panel
  - [ ] Implement auto-hide logic for controls during playback (fade out after 3 seconds of inactivity, reappear on mouse movement)
- [ ] Implement input blocking during cinematic playback in genesis-render/src/input/mod.rs
  - [ ] Create InputBlocker resource with is_blocking: bool field (default: false)
  - [ ] Implement InputBlocker::block(&mut self) method setting is_blocking = true during cinematic playback
  - [ ] Implement InputBlocker::unblock(&mut self) method setting is_blocking = false in interactive mode
  - [ ] Modify handle_keyboard_input() system to skip processing when InputBlocker.is_blocking = true using .run_if(|blocker: Res<InputBlocker>| !blocker.is_blocking)
  - [ ] Modify handle_mouse_input() system to skip processing when InputBlocker.is_blocking = true using .run_if(|blocker: Res<InputBlocker>| !blocker.is_blocking)
  - [ ] Allow specific override keys (Space for pause, Escape for stop) to pass through even during cinematic playback using conditional logic in input systems
  - [ ] Add "Exit Cinematic Mode" keyboard shortcut (Escape) that calls CinematicMode::stop() and re-enables user input
  - [ ] Register InputBlocker as Bevy resource via InputPlugin with insert_resource(InputBlocker::new()) in build()
  - [ ] Integrate InputBlocker with CinematicMode by blocking input when playback_state = CinematicPlaybackState::Playing or Transitioning
- [ ] Load camera keyframes from TOML configuration in genesis-render/src/camera/cinematic.rs
  - [ ] Create cinematic.toml configuration file in config/ directory with [[keyframes]] table structure
  - [ ] Define TOML schema for CameraKeyframe serialization with fields: position = [x, y, z], rotation = [w, x, y, z], cosmic_time = 1e-32, duration = 10.0, easing = "EaseInOutCubic", epoch = "Singularity"
  - [ ] Implement CameraKeyframe::load_from_file(path: &Path) -> Result<Vec<CameraKeyframe>, Box<dyn Error>> function using toml::from_str()
  - [ ] Implement CinematicMode::load_keyframes(&mut self, path: &Path) -> Result<(), Box<dyn Error>> method loading and replacing keyframes Vec
  - [ ] Add default keyframe sequence for "Full 13.8B Year Story" with 8 keyframes:
    1. Singularity: position = [0, 0, 0], rotation = identity, time = 1e-32, duration = 5.0
    2. Inflation start: position = [0, 0, 100], rotation = identity, time = 1e-32, duration = 8.0
    3. QGP: position = [0, 0, 500], rotation = identity, time = 1e-6, duration = 10.0
    4. Nucleosynthesis: position = [0, 0, 2000], rotation = identity, time = 3*60, duration = 12.0
    5. Recombination: position = [0, 0, 10000], rotation = identity, time = 380000*365*24*3600, duration = 15.0
    6. Dark Ages: position = [0, 0, 50000], rotation = identity, time = 1e8*365*24*3600, duration = 15.0
    7. Cosmic Dawn: position = [0, 0, 100000], rotation = identity, time = 1e9*365*24*3600, duration = 15.0
    8. Present Day: position = [0, 0, 200000], rotation = identity, time = 13.8e9, duration = 20.0
  - [ ] Add validation logic checking keyframes are sorted by cosmic_time and no duplicate times exist
  - [ ] Implement CinematicMode::save_keyframes(&self, path: &Path) -> Result<(), Box<dyn Error>> for exporting edited keyframe sequences
  - [ ] Add config path resolution using std::env::var("CARGO_MANIFEST_DIR") to locate cinematic.toml relative to project root
- [ ] Integrate cinematic mode with epoch transitions in genesis-render/src/camera/cinematic.rs
  - [ ] Create EpochTransitionEvent listener system handle_epoch_transition_cinematic() in genesis-render/src/camera/cinematic.rs
  - [ ] Query CinematicMode for keyframes matching transition epoch using keyframes.iter().find(|k| k.epoch == event.new_epoch)
  - [ ] When epoch transition occurs and keyframe exists for new_epoch, trigger camera interpolation to keyframe.position and keyframe.rotation
  - [ ] Call CameraState::start_interpolation_to_target() with target position/rotation from epoch keyframe
  - [ ] Set CinematicMode.playback_state = CinematicPlaybackState::Transitioning during epoch-triggered camera moves
  - [ ] After transition completes, return to CinematicPlaybackState::Playing and resume playback_progress tracking
  - [ ] Sync narration entry display with epoch transition by activating corresponding NarrationEntry based on epoch name
  - [ ] Add camera shake effect during epoch transitions using random position offsets with decaying amplitude over 2.0 seconds
  - [ ] Implement cinematic timing adjustment: slow down simulation time during dramatic moments (e.g., first star ignition) using TimeAccumulator::set_acceleration()
  - [ ] Register handle_epoch_transition_cinematic() system in PostUpdate schedule after epoch_manager transition system

### Expanded UI
- [ ] Create CosmologyPanel UI component in genesis-ui/src/overlay/cosmology_panel.rs
  - [ ] Define CosmologyPanel struct with visibility: bool, position: egui::Pos2, size: egui::Vec2, is_collapsed: bool fields for UI configuration
  - [ ] Implement CosmologyPanel::new() constructor with default position at left-center (Pos2::new(20.0, 300.0)) and size (egui::Vec2::new(350.0, 500.0))
  - [ ] Create cosmology_panel_ui() function accepting egui::Ui, ResMut<Config>, Res<EpochManager> parameters
  - [ ] Design panel layout using egui::Window::new("Cosmological Parameters") with collapsible frame and default_collapsed = false
  - [ ] Add section headers using egui::CollapsingHeader::new() for grouping related parameters (Density Parameters, Expansion Parameters, Spectral Parameters)
  - [ ] Register panel rendering system in GenesisUiPlugin::build() via add_systems(PostUpdate, cosmology_panel_ui.run_if(show_cosmology_panel))
- [ ] Add density parameter controls (Ωₘ, ΩΛ) to CosmologyPanel in genesis-ui/src/overlay/cosmology_panel.rs
  - [ ] Add omega_m (matter density) slider using egui::Slider::new(&mut config.omega_m, 0.0..=1.0).text("Ωₘ (Matter)")
  - [ ] Add omega_lambda (dark energy density) slider using egui::Slider::new(&mut config.omega_lambda, 0.0..=1.0).text("ΩΛ (Dark Energy)")
  - [ ] Add omega_radiation (radiation density) slider using egui::Slider::new(&mut config.omega_radiation, 0.0..=1.0).text("Ωᵣ (Radiation)")
  - [ ] Implement real-time validation checking omega_m + omega_lambda + omega_radiation ≤ 1.01 for flat universe constraint
  - [ ] Add validation warning text in red if total density > 1.01 using egui::Label::new().text_color(egui::Color32::RED)
  - [ ] Add auto-normalization checkbox that redistributes parameter values to enforce flat universe when checked
  - [ ] Display current total density: Ω_total = Ωₘ + ΩΛ + Ωᵣ in panel footer with green color if valid, red if invalid
- [ ] Add expansion parameter controls (H₀) to CosmologyPanel in genesis-ui/src/overlay/cosmology_panel.rs
  - [ ] Add hubble_constant (H₀) slider using egui::Slider::new(&mut config.hubble_constant, 50.0..=100.0).text("H₀ (km/s/Mpc)")
  - [ ] Implement logarithmic slider alternative using egui::DragValue::new(&mut config.hubble_constant).speed(0.1).clamp_range(50.0..=100.0)
  - [ ] Add H₀ preset buttons for common values: Planck 2018 (67.4), Hubble (73.24), WMAP (70.5) using egui::Button
  - [ ] Display H₀ in alternative units: h = H₀ / 100 km/s/Mpc dimensionless parameter
  - [ ] Add critical density calculation display: ρ_c = 3H₀² / 8πG using egui::Label::new()
  - [ ] Implement parameter coupling with H₀ affecting cosmological time scale (display derived age of universe t₀ ≈ 2/3H for matter-dominated)
- [ ] Add spectral parameter controls (n_s, σ₈) to CosmologyPanel in genesis-ui/src/overlay/cosmology_panel.rs
  - [ ] Add spectral_index (n_s) slider using egui::Slider::new(&mut config.spectral_index, 0.85..=1.15).text("nₛ (Spectral Index)")
  - [ ] Add running_of_index (α_s) slider using egui::Slider::new(&mut config.running_of_index, -0.01..=0.01).text("αₛ (Running of nₛ)")
  - [ ] Add sigma_8 (σ₈) slider using egui::Slider::new(&mut config.sigma_8, 0.5..=1.2).text("σ₈ (Matter Fluctuation Amplitude)")
  - [ ] Add nₛ preset buttons for common models: Scale-invariant (1.0), Planck 2018 (0.9649), Blue tilt (0.85), Red tilt (1.15)
  - [ ] Display derived spectral slope for CMB: n_eff = n_s - 1 using conditional formatting (blue for negative, red for positive)
  - [ ] Add parameter descriptions tooltips via egui::RichText::new() with egui::tooltip_text() on hover
- [ ] Add perturbation parameter controls to CosmologyPanel in genesis-ui/src/overlay/cosmology_panel.rs
  - [ ] Add inflaton_mass (m_φ) slider using egui::Slider::new(&mut config.inflaton_mass_log, 15.0..=17.0).text("m_φ (log₁₀ GeV)")
  - [ ] Add inflation_duration slider using egui::Slider::new(&mut config.inflation_duration_log, -35.0..=-30.0).text("Inflation Duration (log₁₀ s)")
  - [ ] Add initial_energy_scale (V_φ) slider using egui::Slider::new(&mut config.initial_energy_scale_log, 15.0..=17.0).text("V_φ (log₁₀ GeV)")
  - [ ] Add perturbation_amplitude (A_s) slider using egui::Slider::new(&mut config.perturbation_amplitude_log, -10.0..=-8.0).text("Aₛ (log₁₀)")
  - [ ] Add scalar_to_tensor_ratio (r) slider using egui::Slider::new(&mut config.scalar_to_tensor_ratio, 0.0..=0.2).text("r (Tensor Ratio)")
  - [ ] Implement logarithmic display for all perturbation parameters using format!("{:.1e}", 10_f64.powi(value as i32))
  - [ ] Add validation warning if tensor ratio r > 0.1 (exceeds observational constraints from BICEP/Keck)
- [ ] Add preset configuration system to CosmologyPanel in genesis-ui/src/overlay/cosmology_panel.rs
  - [ ] Define CosmologyPreset enum in genesis-core/src/config.rs with variants: StandardModel(Planck2018), EinsteinDeSitter, DeSitter, OpenUniverse, ClosedUniverse, HighSigma8, LowInflation
  - [ ] Embed preset parameters in enum variants using struct fields: omega_m, omega_lambda, hubble_constant, spectral_index, sigma_8
  - [ ] Implement CosmologyPreset::apply_to_config(&self, config: &mut Config) method updating all config fields
  - [ ] Add preset dropdown using egui::ComboBox::from_id_source("preset").selected_text(current_preset_name)
  - [ ] Implement preset selection handler calling preset.apply_to_config(&mut config) on selection
  - [ ] Add "Custom" preset variant that activates when user modifies any parameter away from preset values
  - [ ] Implement detect_preset_drift() system that checks if config matches any preset and updates dropdown selection
- [ ] Connect parameter panel controls to simulation restart in genesis-ui/src/overlay/cosmology_panel.rs
  - [ ] Add restart_simulation() function accepting Config, Commands parameters that resets simulation state
  - [ ] Implement restart logic: despawn all Particle entities, reset TimeAccumulator to t=0, reset ScaleFactor to a=1, reset Temperature to T=10²⁷ K
  - [ ] Respawn particles using new config parameters (particle count, initial conditions, perturbation seed)
  - [ ] Re-seed Gaussian random field for density perturbations using updated spectral parameters (n_s, A_s)
  - [ ] Add "Apply & Restart" button to panel using egui::Button::new("Apply & Restart").fill(egui::Color32::from_rgb(50, 150, 50))
  - [ ] Add "Apply Only" button that updates config without restarting simulation (useful for camera settings)
  - [ ] Add "Reset to Defaults" button restoring original config values from genesis.toml
  - [ ] Add unsaved changes indicator displaying "*" next to panel title if config differs from saved values
  - [ ] Implement auto-save to config file on restart via Config::save_to_path()
- [ ] Implement data overlay suite:
  - [ ] Temperature map
    - [ ] Create 2D texture slice or volume ray-march of temperature field
    - [ ] Display with heat map color scheme (blue=cold, red=hot)
  - [ ] Density field
    - [ ] Visualize particle density on 2D grid
    - [ ] Use particle count per cell for density value
    - [ ] Apply density-based color mapping
  - [ ] Velocity streamlines
    - [ ] Compute velocity field from particle velocities on grid
    - [ ] Generate streamlines using velocity field integration
    - [ ] Render as line geometry with arrowheads
  - [ ] Dark matter distribution
    - [ ] Filter particles by dark_matter tag
    - [ ] Render dark matter particles with different color/size
    - [ ] Allow toggling baryon vs dark matter view
  - [ ] Power spectrum P(k) with observational comparisons
    - [ ] Compute P(k) from density perturbations via FFT
    - [ ] Plot P(k) vs k in egui chart
    - [ ] Overlay SDSS/Planck power spectrum data points
    - [ ] Add toggle for observational data visibility
- [ ] Update Config struct to include full cosmological parameter set (Ωₘ, ΩΛ, H₀, n_s, σ₈)
  - [ ] Add CosmologyConfig struct with all parameters
  - [ ] Implement serde serialization/deserialization
  - [ ] Add validation logic for physically meaningful ranges
  - [ ] Update Config to include CosmologyConfig field

### Capture & Export
- [ ] Implement PNG high-resolution frame capture with HDR support
  - [ ] Create FrameCapture resource with is_recording: bool, capture_interval: f32, frame_count: usize fields
  - [ ] Implement capture_frame() system that extracts current render target as Image
  - [ ] Use RenderTarget::resolve() to copy backbuffer to CPU-accessible image
  - [ ] Convert Image to PNG format using image crate (png encoder)
  - [ ] Support super-resolution capture: scale render target to 2x, 4x before capture for high-quality screenshots
  - [ ] Add resolution setting: capture_width, capture_height fields to FrameCapture
  - [ ] Implement timestamp-based filename generation: frame_capture_YYYY-MM-DD_HH-MM-SS_<frame>.png
  - [ ] Register capture_frame() system in Render schedule after post-processing
- [ ] Add EXR HDR frame capture with HDR support
  - [ ] Create EXRCapture struct with format: EXRFormat, exposure: f32, gamma: f32 fields
  - [ ] Implement extract_hdr_frame() system reading HDR render target (16-bit or 32-bit floating point)
  - [ ] Use exr crate for writing OpenEXR format files with floating point channels
  - [ ] Support half-float (16-bit) and full-float (32-bit) EXR output modes
  - [ ] Implement tone mapping: apply Reinhard or ACES tone curve before EXR export for preview
  - [ ] Add HDR metadata: write camera exposure settings, scene linear color space flag to EXR file
  - [ ] Generate timestamped EXR filenames: hdr_capture_YYYY-MM-DD_HH-MM-SS_<frame>.exr
  - [ ] Add HDR validation: clamp values to prevent inf/nan in EXR output
- [ ] Create frame-by-frame export controls
  - [ ] Add FrameCapturePanel UI component with controls: "Capture Single Frame", "Start Recording", "Stop Recording", "Export All Frames"
  - [ ] Implement capture_single_frame() function triggered by button press, saves one PNG/EXR immediately
  - [ ] Implement start_recording() function with auto-capture mode
  - [ ] Implement stop_recording() function that ends auto-capture
  - [ ] Add recording indicator overlay showing "● REC" in red during recording with frame counter
  - [ ] Implement auto-capture at regular intervals (every N frames based on capture_interval)
  - [ ] Implement progress bar showing frames captured: "123 frames captured" during recording
- [ ] Add image export settings panel (resolution, format, HDR toggle)
  - [ ] Create ExportSettingsPanel UI component with sections for Resolution, Format, Quality
  - [ ] Add resolution presets: "Native (1080p)", "2K (1440p)", "4K (2160p)", "8K (4320p)" buttons
  - [ ] Add custom resolution inputs: width (pixels), height (pixels) with aspect ratio lock toggle
  - [ ] Add format selection radio buttons: "PNG (8-bit)", "PNG (16-bit)", "EXR HDR (16-bit float)", "EXR HDR (32-bit float)"
  - [ ] Add HDR toggle: "Enable HDR" checkbox that switches between PNG and EXR formats
  - [ ] Add quality slider for PNG compression (0-100, default 85) controlling compression level
  - [ ] Add color space selection: "sRGB", "Linear RGB", "Display P3" for output color space
  - [ ] Add batch export mode: "Export Range (frames N to M)" with frame range inputs
  - [ ] Implement export_queue system for processing multiple frames without blocking main thread
  - [ ] Add export destination folder selector using file dialog for choosing save path
  - [ ] Persist export settings to Config using Config::save_to_path() for next session

### Benchmarking
- [ ] Implement genesis-bench crate
  - [ ] Create genesis-bench/Cargo.toml with dependencies: criterion (for benchmarking), serde, serde_json (for result export)
  - [ ] Implement benchmark main function in genesis-bench/src/main.rs
  - [ ] Add dependency on genesis-core and genesis-render to access systems and resources
  - [ ] Create benchmark harness that initializes Bevy app with minimal plugins for headless rendering
- [ ] Create automated performance regression tests
  - [ ] Define BenchmarkConfig struct with particle_counts: Vec<usize>, duration_seconds: f64, output_format: BenchmarkFormat fields
  - [ ] Implement particle_scaling_benchmark() function testing 10K, 100K, 1M, 10M particle counts
  - [ ] Implement frame_rate_benchmark() function measuring FPS over 60 seconds for each particle count
  - [ ] Implement memory_usage_benchmark() function tracking GPU and CPU memory allocation during benchmark
  - [ ] Implement startup_time_benchmark() function measuring time from launch to first rendered frame
  - [ ] Implement epoch_transition_benchmark() function testing performance impact of each epoch transition
  - [ ] Use criterion crate for statistical analysis (mean, median, std dev, confidence intervals)
  - [ ] Add warmup phase (10 seconds) before collecting benchmark data to stabilize performance
- [ ] Add benchmark results export
  - [ ] Define BenchmarkResult struct with benchmark_name: String, timestamp: String, hardware_info: HardwareInfo, metrics: BenchmarkMetrics fields
  - [ ] Define HardwareInfo struct with gpu_name: String, cpu_name: String, ram_gb: usize, vram_gb: usize fields
  - [ ] Define BenchmarkMetrics struct with fps: f64, frame_time_ms: f64, particle_count: usize, gpu_memory_mb: usize, cpu_memory_mb: usize fields
  - [ ] Implement export_to_json() function writing results to JSON with formatting for human readability
  - [ ] Implement export_to_csv() function writing results to CSV with columns: Benchmark, Timestamp, GPU, CPU, FPS, FrameTime, ParticleCount, GPUMem, CPUMem
  - [ ] Generate timestamped filenames: benchmark_YYYY-MM-DD_HH-MM-SS.json and .csv
  - [ ] Add hardware detection to HardwareInfo using wgpu::Adapter::get_info() for GPU, sysinfo crate for CPU/RAM
  - [ ] Include system information in export output (OS version, driver version, Rust version)
  - [ ] Create results directory: benchmarks/ in project root for organized storage
- [ ] Set up CI integration for performance tests
  - [ ] Create .github/workflows/benchmark.yml workflow file
  - [ ] Configure workflow to run on schedule (e.g., weekly) and on pull requests targeting main branch
  - [ ] Add benchmark job step: cd genesis-bench && cargo run --release -- --output-dir ./benchmark_results
  - [ ] Upload benchmark results as workflow artifacts using actions/upload-artifact@v3
  - [ ] Create benchmark_history.json tracking file accumulating historical results
  - [ ] Implement performance regression detection: compare current results against historical baseline
  - [ ] Add failure condition: if FPS drops >5% from baseline, mark workflow as failed
  - [ ] Add comment action posting benchmark summary to PRs with performance comparison
  - [ ] Create dashboard visualization using benchmark_history.json (HTML page with charts showing FPS trends over time)
  - [ ] Add alerts configuration: notify team if performance regression exceeds threshold

### Release & Documentation
- [ ] Create cross-platform release builds (Linux, macOS including Apple Silicon, Windows)
- [ ] Write comprehensive user documentation
  - [ ] Create docs/USER_GUIDE.md with complete user manual
    - [ ] Document installation procedures for all platforms (Linux, macOS, Windows)
    - [ ] Document system requirements and supported hardware
    - [ ] Provide getting started walkthrough (first launch, basic controls)
    - [ ] Document camera controls (free-flight WASD+mouse, orbit click-drag)
    - [ ] Document timeline controls (play/pause, speed adjustment, scrubbing)
    - [ ] Document epoch transitions and what to expect in each phase
    - [ ] Document cosmological parameters and their effects
    - [ ] Document keyboard shortcuts and mouse controls
    - [ ] Document troubleshooting common issues
  - [ ] Create README.md with project overview and quick start
    - [ ] Write project description and feature summary
    - [ ] Add build instructions for developers (Cargo commands)
    - [ ] Add quick start guide for users (download, install, run)
    - [ ] Include screenshots from each epoch (singularity, inflation, nucleosynthesis, recombination, dark ages, cosmic dawn)
    - [ ] List keyboard shortcuts and controls
    - [ ] Add links to full documentation
    - [ ] Include license and credits
  - [ ] Write tutorial walkthrough for key features
    - [ ] Create docs/TUTORIALS.md with step-by-step guides
    - [ ] Write "Your First Simulation" tutorial (launch app, watch evolution, use timeline)
    - [ ] Write "Exploring the Cosmic Timeline" tutorial (scrub through epochs)
    - [ ] Write "Adjusting Cosmological Parameters" tutorial (modify Ωₘ, ΩΛ, H₀, n_s, σ₈)
    - [ ] Write "Using Cinematic Mode" tutorial (play full story, understand narration)
    - [ ] Write "Exporting Data" tutorial (HDF5 snapshots, CSV summaries, screenshots)
    - [ ] Write "Creating Custom Presets" tutorial (TOML configuration files)
  - [ ] Create API documentation for developers
    - [ ] Add rustdoc comments to all public APIs
    - [ ] Generate documentation with `cargo doc --open`
    - [ ] Document crate structure and module organization
    - [ ] Document epoch plugin system for extending the simulator
  - [ ] Create CONTRIBUTING.md with contribution guidelines
    - [ ] Document development environment setup
    - [ ] Document code style and conventions
    - [ ] Document pull request process
    - [ ] Document testing procedures
    - [ ] Add template for issue reporting
  - [ ] Create CHANGELOG.md tracking version history
    - [ ] Document unreleased changes
    - [ ] Document version 1.0.0 release notes
    - [ ] Follow Keep a Changelog format (Added, Changed, Deprecated, Removed, Fixed, Security)
- [ ] Implement preset configuration sharing via TOML files
- [ ] Add version information and changelog
- [ ] Create installation scripts or packages for each platform (deb, rpm, dmg, msi)
  - [ ] Generate Debian package (.deb) using cargo-deb plugin: cargo deb --target x86_64-unknown-linux-gnu
  - [ ] Generate Red Hat package (.rpm) using cargo-generate-rpm: cargo generate-rpm --target x86_64-unknown-linux-gnu
  - [ ] Generate macOS disk image (.dmg) using app bundle: bundle macos --create-app --features "metal"
  - [ ] Generate Windows installer (.msi) using wix toolset or Inno Setup: cargo wix --target x86_64-pc-windows-msvc
  - [ ] Sign macOS binaries with Developer ID certificate and submit for Apple notarization: xcrun notarytool submit genesis-macos-universal.dmg
  - [ ] Sign Windows binaries with code signing certificate using signtool.exe
  - [ ] Create Linux AppImage for universal Linux distribution: ./scripts/build_appimage.sh
  - [ ] Create Windows portable .zip archive without installer for advanced users
  - [ ] Add install.sh and uninstall.sh scripts for Linux manual installation

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
