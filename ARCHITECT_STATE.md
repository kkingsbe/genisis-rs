# ARCHITECT_STATE.md
> Last Updated: 2026-02-09T00:44:00.000Z
> Status: IN_PROGRESS

## Completed This Session
- [x] Task 1: Gap Analysis & Sprint Planning
  - Added ~40 new requirements to BACKLOG.md
  - Refined ~15 items in BACKLOG.md (breaking down vague/large items into atomic tasks)
- [x] Task 2: Sprint Management
  - Verified .sprint_complete file does not exist (no sprint completion marker)
  - Did NOT move any tasks from BACKLOG.md to TODO.md (per Sprint Protocol)
  - Verified Sprint QA task exists in TODO.md (line 84)
  - Added Final Sprint QA section with Sprint QA task as the final item in TODO.md (line 131)
  - Ensured Sprint Protocol enforcement - no new tasks allowed without .sprint_complete
- [x] Task 3: Blocker Review
  - Reviewed BLOCKERS.md - no blockers currently reported
  - Total blockers reviewed: 0
  - Blockers resolved through architectural decisions: 0
  - Blockers remaining: 0
- [x] Task 4: Communication
  - Reviewed comms/outbox/ directory - no existing questions
  - Analyzed PRD.md for ambiguities and impossible requirements
  - Reviewed gap analysis results from Task 1 (ARCHITECT_STATE.md)
  - Created 10 specific question documents in comms/outbox/
  - Identified critical ambiguities requiring user attention

## Currently Working On
- [ ] (None - waiting for next task)

## Remaining Tasks
- [ ] Task 5: Cleanup

## Gap Analysis Summary

### Requirements Added to BACKLOG.md

#### Phase 1 (The Singularity)
1. Scroll wheel zoom controls for free-flight camera (move along forward vector)
2. Scroll wheel zoom controls for orbit camera with min/max bounds
3. Pan controls for both camera modes (WASD + modifier keys or middle mouse button)
4. Smooth camera interpolation system (camera tween resource with start/end positions, duration, easing function)
5. Timeline scrubbing sync with particle simulation (reverse/replay capability)
6. PlaybackState resource initialization in main.rs
7. TOML deserialization dependencies and implementation
8. Update existing systems to read from ConfigResource
9. Epoch plugin implementation (actual plugins with build() methods)
10. Epoch plugin registration in main.rs
11. Documentation for epoch plugin architecture and public APIs
12. Documentation for CosmicTime, PointSpriteMaterial, OrbitController

#### Phase 2 (Inflation & Quantum Seeds)
13. ScaleFactor resource tracking a(t), ȧ, and cosmic epoch
14. Temperature evolution model (T ∝ 1/a for adiabatic expansion)
15. DensityField resource tracking perturbation values and derivatives
16. QGPEpoch plugin
17. Config struct updates for Phase 2 parameters
18. "Standard Model" preset with Phase 2 cosmological parameters

#### Phase 3 (Nucleosynthesis & First Elements)
19. NuclearComposition resource tracking element abundances Y_i
20. NucleosynthesisEpoch plugin

#### Phase 4 (Recombination & CMB)
21. Thomson cross-section constant implementation
22. CMB resource tracking temperature anisotropies
23. LastScatteringSurface resource tracking CMB sphere parameters
24. CMB analysis panel with temperature readout

#### Phase 5 (Dark Ages & Structure Formation)
25. GravitySystem resource tracking gravitational constants
26. DarkAgesEpoch plugin
27. Velocity Verlet/leapfrog integrator for particle motion

#### Phase 6 (Cosmic Dawn & Galaxy Formation)
28. SPHSystem resource tracking SPH parameters
29. CosmicDawnEpoch plugin

#### Phase 7 (Polish, Cinematic Mode & Release)
30. PerformanceProfiler resource tracking FPS, GPU time, memory usage
31. CinematicMode resource tracking keyframes, playback state
32. Image export settings panel

### Items Refined in BACKLOG.md

#### Core Visualization
- Refined "energy-based color mapping" to specify gradient: E > 0.8 → white, 0.5 < E < 0.8 → yellow, E < 0.5 → red
- Refined "cooling model" to specify T ∝ 1/r for adiabatic expansion
- Added Energy component tracking and update system tasks

#### Camera Controls
- Refined "pan controls" to specify WASD + Shift or middle mouse button
- Added easing functions (Linear, EaseInQuad, EaseOutQuad, EaseInOutCubic)
- Added CameraTween resource tracking active tween details
- Added camera tween update and trigger system tasks

#### Physics Integration (Phase 2)
- Refined Friedmann equation to specify H² = (8πG/3)ρ - k/a²
- Refined RK4 solver details
- Refined inflaton potential to specify m ~ 10¹⁶ GeV
- Refined expansion formulas with specific H and T values
- Added InflationPhysics resource for tracking φ, V(φ), ε, η

#### Density Perturbations (Phase 2)
- Refined Gaussian random field generator to include Box-Muller transform details
- Added FFT and power spectrum application steps
- Refined displacement mapping with specific formula
- Refined color mapping with intensity = 1.0 + α*δ
- Added GaussianRandomField resource

#### Nuclear Reaction Network (Phase 3)
- Refined NuclearReaction struct details
- Added Element enum for species
- Specified ~50 reactions from BBN network
- Added NACRE II lookup and interpolation details
- Added Rosenbrock method and Jacobian matrix details
- Added reaction rate validation task

#### N-Body Gravity (Phase 5)
- Added gravitational constant G value
- Refined direct-sum force calculation formula
- Added softening parameter ε details
- Specified Barnes-Hut octree structure and traversal
- Added optimization techniques (spatial hashing, shared memory)
- Added velocity Verlet/leapfrog integrator task

#### SPH (Phase 6)
- Refined Wendland C4 kernel with full formula
- Added kernel gradient and Laplacian implementations
- Specified equation of state with γ = 5/3
- Refined pressure and viscosity force formulas
- Added SPH neighbor search task using spatial hashing/kd-tree

## Key Gaps Identified

1. **Configuration System**: No TOML configuration system exists - needs serde, clap integration, and Config struct
2. **Epoch Plugin Implementation**: Infrastructure exists but no actual epoch plugins are implemented
3. **Timeline Sync**: Timeline UI exists but particle simulation reverse/replay is not connected
4. **Procedural Singularity**: Current implementation uses random spawning, not PRD-specified energy-mapped visualization
5. **Smooth Camera Interpolation**: No interpolation system exists for epoch transitions
6. **Documentation**: Many public APIs lack detailed documentation
7. **Physics Resources**: Many physics systems need dedicated resources (ScaleFactor, NuclearComposition, etc.)

## Significant Concerns

1. **Particle Component Mismatch**: `genesis-core::physics::Particle` uses `[f32; 3]` while `genesis-render::particle::Particle` uses `Vec3` and `Color` - these are disconnected
2. **GPU Instance Data Sync**: Particle component data is not synchronized with GPU instance attributes (instance_size, instance_color)
3. **Resource Initialization**: PlaybackState is not initialized in main.rs
4. **Build System**: Cross-platform build configuration is not set up

## Next Steps

Wait for user responses to communication questions before proceeding with implementation.

## Task 4 Communication Summary

### Questions Written
Created 10 question documents in `comms/outbox/`:

1. **question-timeline-reverse-replay.md**
   - Issue: Timeline reverse/replay requires storing full simulation state (memory-intensive)
   - Impact: Affects Phase 1 timeline scrubbing functionality
   - Criticality: HIGH - fundamental feature requirement

2. **question-particle-coordinate-system.md**
   - Issue: Ambiguous whether to use comoving or physical coordinates for particles
   - Impact: Affects all phases (2-7) and entire simulation pipeline
   - Criticality: HIGH - fundamental architectural decision

3. **question-volumetric-fog-implementation.md**
   - Issue: No specification of which volumetric fog rendering technique to use
   - Impact: Affects Phase 4 recombination visualization
   - Criticality: MEDIUM - affects visual quality and performance

4. **question-barnes-hut-gpu-traversal.md**
   - Issue: CPU build + GPU traversal for Barnes-Hut is architecturally complex and potentially inefficient
   - Impact: Affects Phase 5 N-body gravity and performance targets
   - Criticality: HIGH - single hardest technical challenge per PRD

5. **question-zeldovich-nonlinear-limitations.md**
   - Issue: Zel'dovich approximation (linear regime) incompatible with nonlinear structure formation
   - Impact: Affects Phase 2-5 transition and scientific accuracy
   - Criticality: HIGH - fundamental physics incompatibility

6. **question-reionization-sdf-visualization.md**
   - Issue: "Signed-distance-field bubbles" specification lacks implementation details
   - Impact: Affects Phase 6 reionization visualization and performance
   - Criticality: MEDIUM - affects complex visual effect

7. **question-high-fidelity-performance-targets.md**
   - Issue: 50M-100M particles at ≥30 FPS on RTX 3080 appears unachievable with full visual fidelity
   - Impact: Affects Phase 7 and overall performance targets
   - Criticality: HIGH - marketing/expectations vs. technical feasibility

8. **question-camera-interpolation-epic-transitions.md**
   - Issue: Inconsistent camera requirements across phases (smooth interpolation, automatic transitions, cinematic mode)
   - Impact: Affects Phase 1, 4, 7 camera systems
   - Criticality: MEDIUM - UX/interaction design

9. **question-minimum-particle-count-per-phase.md**
   - Issue: No minimum particle count specified for each phase to be considered "complete"
   - Impact: Affects all phase completion criteria and development priorities
   - Criticality: MEDIUM - affects scope and validation

10. **question-minor-ambiguities.md**
    - Issue: 9 minor ambiguities covering audio timing, export format, benchmarking, presets, cross-platform, validation data, epoch plugins, timeline acceleration, singularity visualization
    - Impact: Various, lower individual criticality but should be resolved collectively
    - Criticality: LOW-MEDIUM - implementation details

### Critical Ambiguities Requiring Immediate Attention
1. **Timeline reverse/replay** - May require PRD modification or alternative approach
2. **Particle coordinate system** - Fundamental architectural decision blocking Phase 2
3. **Barnes-Hut implementation** - May require PRD modification (GPU build vs. CPU build)
4. **Zel'dovich vs. N-body transition** - Physics incompatibility requiring resolution
5. **High-Fidelity performance targets** - May require reducing particle count targets

### Total Questions: 10
- HIGH criticality: 5
- MEDIUM criticality: 3
- LOW-MEDIUM criticality: 2
