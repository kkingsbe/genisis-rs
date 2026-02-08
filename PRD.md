# GENESIS

### Product Requirements Document — v2.0 (Incremental Delivery Edition)

*A Real-Time Big Bang & Cosmological Evolution Simulator*

**Version:** 2.0  
**Date:** February 2026  
**Author:** Kyle | Aegis AI  
**Status:** Draft

---

## 1. Executive Summary

Genesis is a high-performance Rust application that simulates the birth and evolution of the universe, from the initial singularity through cosmic inflation, nucleosynthesis, recombination, and large-scale structure formation. It combines N-body gravitational dynamics, thermodynamic modeling, and particle physics approximations to produce a visually compelling, physically grounded real-time experience.

> **Incremental delivery principle:** This plan is structured so that every development phase produces a fully runnable application with visible, demonstrable output. There is never a phase that ends with invisible backend work alone. Each phase builds on the last, and you can ship or demo at any milestone boundary.

---

## 2. Core Design Principles

### 2.1 Always Runnable

The application compiles and runs with visual output from the very first phase. New features are layered onto a working renderer, never developed in isolation. If a physics system isn't ready, a procedural placeholder visualization runs in its place.

### 2.2 Vertical Slices Over Horizontal Layers

Each phase delivers a thin vertical slice: input handling, simulation logic, rendering, and UI for one feature set. We do not build "all the physics first, then all the rendering." Every phase touches the full stack.

### 2.3 Progressive Enhancement

Early phases use simpler algorithms (direct-sum gravity, basic particles) that are replaced by optimized versions (Barnes-Hut, GPU compute) in later phases. The visual result is always present; only the fidelity changes.

### 2.4 Demo-Ready at Every Merge

Each phase ends with a defined "Demo Moment" — a specific visual outcome that proves the work is integrated and functioning. These are captured in callout boxes throughout the roadmap.

---

## 3. Goals & Non-Goals

### 3.1 Goals

- Simulate major epochs of universal evolution from t ≈ 10⁻³²s through 13.8 billion years
- Achieve real-time interactive frame rates (≥60 FPS) on modern consumer GPUs
- Provide physically motivated models for inflation, nucleosynthesis, recombination, dark matter halo formation, and galaxy assembly
- Deliver a visually compelling 3D rendering pipeline with volumetric effects and particle systems
- Support configurable cosmological parameters (Ωₘ, ΩΛ, H₀, σ₈) for what-if scenarios
- Export simulation snapshots in standard formats (HDF5, VTK) for external analysis
- Cross-platform: Linux, macOS, Windows

### 3.2 Non-Goals

- Full general-relativistic field equation solving (Friedmann approximations used instead)
- Quantum chromodynamics simulation at quark level
- Sub-parsec resolution for individual stellar evolution
- Competing with research-grade codes (Gadget-4, AREPO) for publication-quality results
- Multiplayer or networked modes (v1)

---

## 4. Technical Architecture

The architecture is designed so that the rendering and simulation layers are decoupled. This is critical for incremental delivery: a placeholder visualization can always stand in while the real physics is under development.

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| ECS Runtime | Bevy 0.15+ | Parallel system scheduling, asset pipeline, built-in rendering |
| GPU Compute | wgpu / WGSL shaders | Cross-platform GPU compute for N-body, SPH, particles |
| Math | glam + nalgebra | glam for transforms, nalgebra for scientific linear algebra |
| Spatial Index | Custom octree / BVH | Barnes-Hut O(N log N) gravity, rendering culling |
| ODE Solver | Custom RK4 / Dormand-Prince | Adaptive integration for nucleosynthesis networks |
| Rendering | Bevy PBR + custom shaders | Volumetric rendering, instanced particles, bloom, HDR |
| Serialization | serde + hdf5-rust | Snapshot I/O in scientific standard format |
| Audio | kira (bevy_kira_audio) | Procedural soundscape tied to simulation state |
| UI | bevy_egui | Immediate-mode GUI for controls, timeline, overlays |

### 4.1 Epoch Plugin Architecture

Each cosmological epoch is a Bevy plugin that registers its own systems, renderers, and UI panels with a central Epoch Manager. The manager handles transitions (crossfade blending, parameter interpolation) and ensures exactly one epoch's physics is active while all completed epochs' visual outputs remain available for timeline scrubbing. Critically, this means we can ship new epochs as drop-in additions without modifying existing code.

### 4.2 Crate Structure

| Crate | Responsibility | First Used |
|-------|---------------|------------|
| genesis-core | ECS setup, epoch manager, time integration, config loading | Phase 1 |
| genesis-render | Bevy rendering plugins, custom shaders, particle systems | Phase 1 |
| genesis-ui | egui panels, timeline, parameter controls, overlays | Phase 1 |
| genesis-physics | Gravity, SPH, nucleosynthesis, inflation, perturbations | Phase 2 |
| genesis-export | HDF5, VTK, PNG/EXR, CSV export pipelines | Phase 5 |
| genesis-audio | Procedural soundscape, epoch-aware audio mixing | Phase 6 |
| genesis-bench | Benchmarking harness, performance regression tests | Phase 7 |

---

## 5. Incremental Delivery Roadmap

Each phase below ends with a working, runnable application. The "Demo Moment" boxes describe exactly what a user sees when they launch the application after that phase is complete.

---

### Phase 1: The Singularity
#### Window, Particle Engine & Time

**Duration:** 2–3 weeks  
**Goal:** A running Bevy application with a 3D particle system, camera controls, and a time slider. This is the foundation everything else builds on.

**Deliverables:**

- Bevy application scaffold with window, input handling, and basic 3D scene
- Instanced particle renderer capable of displaying 100K–1M point sprites with position, color, and size
- Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation
- Cosmic time system: a f64 time accumulator with adjustable acceleration (1x to 10¹²x), pause, and reset
- Logarithmic timeline scrubber UI (bevy_egui) spanning 13.8 billion years
- A procedural "singularity" visualization: particles spawned at origin with outward velocity, color-mapped by energy (white-hot core fading to red)
- FPS counter and particle count overlay

> **▶ Demo Moment: The Primordial Spark**
>
> Launch the app. A dense, glowing white-hot cluster of particles sits at the center of a dark void. Press Play on the timeline. The particles explode outward in all directions, cooling from white to yellow to red as they expand. Scrub the timeline back and forth — the expansion reverses and replays. Fly the camera around the expanding cloud. This is the visual foundation for every subsequent phase.

---

### Phase 2: Inflation & Quantum Seeds
#### Metric Expansion & Density Perturbations

**Duration:** 3–4 weeks  
**Goal:** Replace the simple explosion with physics-driven cosmic inflation. Seed the universe with density fluctuations that will become the cosmic web.

**Deliverables:**

- Friedmann equation integrator for scale factor a(t) with slow-roll inflaton potential V(φ)
- Particle positions now scale with a(t) — exponential expansion during inflation, decelerating after
- 3D Gaussian random field generator with nearly scale-invariant power spectrum P(k) ∝ k^(n_s – 1)
- Density perturbations mapped to particle displacement (Zel'dovich approximation) and color intensity
- Epoch indicator in UI showing current cosmic era and key parameters (temperature, scale factor, time)
- Parameter panel (bevy_egui sidebar): adjust n_s, inflation duration, and initial energy scale; changes restart simulation
- Procedural QGP visualization: during quark-gluon plasma phase, particles rendered as glowing plasma blobs with temperature-mapped color ramp (blue-white at peak, cooling through yellow to orange)

> **▶ Demo Moment: Inflation in Action**
>
> Launch the app. The singularity is visible momentarily, then SNAP — the universe inflates exponentially, particles rocketing outward far faster than before. As inflation ends, the expansion decelerates. You can see faint density variations — slightly brighter clumps and dimmer voids — seeded by quantum fluctuations. The UI shows "Inflation" transitioning to "Quark-Gluon Plasma" with live temperature readout dropping from 10²⁷ K. Tweak n_s in the sidebar and watch the clumpiness change.

---

### Phase 3: Nucleosynthesis & the First Elements
#### BBN Reaction Network & Composition Visualization

**Duration:** 3–4 weeks  
**Goal:** Simulate Big Bang nucleosynthesis and show element formation in real time with a live composition chart overlay.

**Deliverables:**

- Stiff ODE solver (implicit Rosenbrock method) for 12-species nuclear reaction network (n, p, D, T, ³He, ⁴He, ⁷Li, ⁷Be, intermediates)
- Reaction rates from NACRE II compilation, temperature-dependent
- Live composition pie/bar chart overlay showing element abundances evolving in real time
- Particle color transitions: as nucleosynthesis proceeds, individual particles are color-coded by dominant composition (hydrogen = blue, helium = yellow, lithium = faint pink)
- Epoch transition: smooth visual crossfade from QGP plasma rendering to nucleosynthesis element-colored particles
- Validation overlay (toggle-able): comparison lines showing observed primordial abundances (Y_p ≈ 0.245 for ⁴He)
- TOML configuration presets: "Standard Model" (Planck 2018 best-fit) and "High Baryon Density" for comparison

> **▶ Demo Moment: Forging Helium**
>
> Launch the app and let it play through inflation. Around the 3-minute mark on the timeline, the nucleosynthesis epoch activates. Particles shift from plasma-white to element colors. A live bar chart in the corner shows hydrogen abundance dropping from 100% as helium climbs toward ~24.5%. Deuterium spikes briefly then burns away. After 20 minutes of cosmic time, the chart stabilizes — toggle the validation overlay and see your simulated abundances line up with observed values. Switch to "High Baryon Density" preset and watch helium overshoot.

---

### Phase 4: Recombination & the Cosmic Microwave Background
#### Photon Decoupling & CMB Surface

**Duration:** 3–4 weeks  
**Goal:** Model recombination, render the transition from opaque plasma to transparent space, and project the CMB.

**Deliverables:**

- Saha equation solver tracking ionization fraction x_e as a function of temperature
- Volumetric fog renderer: space starts opaque (photon mean free path ≪ horizon), then clears as x_e drops below threshold
- CMB surface projection: a spherical shell at the last-scattering surface showing temperature anisotropies generated from the Phase 2 density perturbations
- Smooth camera transition: as recombination completes, the "fog lifts" and the camera pulls back to reveal the CMB sphere surrounding the observable universe
- Temperature readout drops through 3000 K (recombination) toward 2.725 K (present-day CMB)
- Toggle overlay: show/hide CMB angular power spectrum C_ℓ with qualitative comparison to Planck data

> **▶ Demo Moment: First Light**
>
> Play through to 380,000 years. The universe is an opaque glowing fog — you can't see more than a short distance. Then the fog begins to clear. Over a few moments of cosmic time, the universe becomes transparent and you see it: the Cosmic Microwave Background, a mottled sphere of faint anisotropies surrounding you. Pull back the camera and the CMB sphere is visible as the edge of the observable universe. The density lumps you seeded in Phase 2 are now visible as hot and cold spots on the CMB.

---

### Phase 5: Dark Ages & First Structures
#### N-Body Gravity, Dark Matter Halos & Cosmic Web

**Duration:** 4–6 weeks  
**Goal:** This is the physics-heaviest phase. Implement gravitational N-body simulation, grow dark matter halos from the density perturbations, and render the cosmic web.

**Deliverables:**

- Direct-sum N-body gravity on GPU (wgpu compute shader) for up to 500K particles as baseline
- Barnes-Hut octree (CPU build, GPU traversal) for scaling to 1M–10M particles
- Dark matter particles seeded from Phase 2 perturbation field; baryonic particles coupled
- Adaptive level-of-detail: particle splitting in high-density regions, merging in voids
- Halo finder (Friends-of-Friends algorithm) identifying collapsed structures in real time
- Cosmic web visualization: filaments rendered as line geometry connecting halos, voids rendered as transparent dark regions
- Data export: HDF5 snapshot export (particle positions, velocities, masses, temperatures) and CSV timeline summary (scale factor, temperature, Hubble parameter over cosmic time)
- Timeline shows smooth transition from linear perturbation growth to nonlinear structure formation

> **▶ Demo Moment: The Cosmic Web Emerges**
>
> Play through to ~500 million years. The uniform particle field from earlier phases begins to clump. Density perturbations grow under gravity — filaments of matter stretch between growing dark matter halos. By 1 billion years, a recognizable cosmic web has formed: bright nodes (proto-clusters) connected by filaments, separated by vast voids. Fly the camera through the filaments. Export a snapshot to HDF5 and open it in ParaView to verify the 3D structure.

---

### Phase 6: Cosmic Dawn & Galaxy Formation
#### Star Formation, Reionization & Galaxy Assembly

**Duration:** 4–5 weeks  
**Goal:** Populate dark matter halos with baryonic physics: gas cooling, star formation, reionization, and the first galaxies.

**Deliverables:**

- Smoothed Particle Hydrodynamics (SPH) with Wendland C4 kernel for baryonic gas dynamics
- Radiative cooling functions (Sutherland & Dopita 1993 tables) driving gas collapse
- Sub-grid star formation: Kennicutt-Schmidt relation converts dense gas into star particles
- Pop III star formation in early halos; first light sources appear as bright point lights
- Reionization visualization: ionization fronts expand as signed-distance-field bubbles around star-forming halos, eating into the remaining neutral gas
- Galaxy billboard sprites: halos above mass threshold render as composite galaxy sprites with morphology based on merger history
- Procedural ambient audio: deep bass drones during dark ages, rising harmonic tones as first stars ignite, full cosmic soundscape by galaxy formation era
- VTK mesh export for density and velocity fields on regular grid

> **▶ Demo Moment: Let There Be Light**
>
> After the dark ages, tiny points of light flicker on inside the densest halos. These are the first stars. Ionization bubbles expand around them — translucent spheres eating into the dark neutral medium. The bubbles grow, overlap, and merge until the entire universe is reionized. Zoom into a massive halo and see a galaxy sprite forming. Zoom out and the cosmic web now glows with thousands of galaxies strung along its filaments. Audio swells from a low rumble to a rich harmonic drone.

---

### Phase 7: Polish, Cinematic Mode & Release
#### Performance, UX, Documentation & Packaging

**Duration:** 3–4 weeks  
**Goal:** Optimize performance, add cinematic mode, finalize UI/UX, write documentation, and produce cross-platform release builds.

**Deliverables:**

- Performance optimization pass: GPU shader profiling, memory budget enforcement, particle LOD tuning to hit 60 FPS / 1M particles on GTX 1660
- Cinematic mode: pre-authored camera paths with keyframes and easing curves, narrated text overlays explaining each epoch, suitable for museum installations and classroom presentations
- Expanded parameter panel: full cosmological parameter set (Ωₘ, ΩΛ, H₀, n_s, σ₈) with presets for Standard Model, Einstein-de Sitter, De Sitter, and Open Universe
- Data overlay suite: temperature map, density field, velocity streamlines, dark matter distribution, power spectrum P(k) with observational comparison lines
- PNG/EXR high-resolution frame capture with HDR support
- Benchmarking harness with automated performance regression tests
- Cross-platform release builds: Linux, macOS (including Apple Silicon), Windows
- User documentation, README, and tutorial walkthrough
- Preset configuration sharing via TOML files

> **▶ Demo Moment: The Full Story of Everything**
>
> Press a single button to enter Cinematic Mode. The camera pulls in to a white-hot singularity. Text fades in: "10⁻³² seconds after the beginning." Inflation snaps outward. The quark-gluon plasma glows and cools. Elements forge. The fog lifts to reveal the CMB. Gravity sculpts the cosmic web. First stars ignite. Galaxies assemble. The camera pulls back to reveal the full observable universe, 13.8 billion years of evolution in 8 minutes. The entire history plays without a single loading screen, stutter, or manual intervention.

---

## 6. Phase Dependency Map

Each phase builds strictly on the previous. There are no orphaned phases or parallel workstreams that converge later. The critical path is linear by design.

| Phase | Depends On | Outputs Consumed By | Can Ship Standalone? |
|-------|-----------|-------------------|---------------------|
| P1: Singularity | None | P2, P3, P4, P5, P6, P7 | Yes — particle toy / tech demo |
| P2: Inflation | P1 (renderer, timeline) | P3, P4, P5 | Yes — inflation visualizer |
| P3: Nucleosynthesis | P2 (temperature model) | P4 | Yes — BBN educational tool |
| P4: Recombination | P2 (perturbations), P3 (composition) | P5, P6 | Yes — CMB explorer |
| P5: Structure | P2 (perturbations), P4 (post-recomb state) | P6 | Yes — N-body cosmic web sim |
| P6: Cosmic Dawn | P5 (halos, cosmic web) | P7 | Yes — galaxy formation sim |
| P7: Polish | All prior phases | Release | Yes — final product |

---

## 7. Cosmological Epochs Reference

For reference, the following table maps physical epochs to the simulation phases that implement them.

| Epoch | Time Range | Key Physics | Implemented In |
|-------|-----------|-------------|---------------|
| Planck Boundary | t < 10⁻³²s | Quantum gravity (visual only) | Phase 1 |
| Inflation | 10⁻³²s – 10⁻³²s | Exponential metric expansion | Phase 2 |
| Quark-Gluon Plasma | 10⁻³²s – 10⁻⁶s | QGP cooling, confinement | Phase 2 |
| Nucleosynthesis | 3 min – 20 min | Light element formation | Phase 3 |
| Recombination | ~380,000 yr | Electron capture, CMB release | Phase 4 |
| Dark Ages | 380 Kyr – 100 Myr | Gravitational collapse | Phase 5 |
| Cosmic Dawn | 100 Myr – 1 Gyr | First stars, reionization | Phase 6 |
| Structure Formation | 1 Gyr – 13.8 Gyr | Galaxy assembly, cosmic web | Phase 5 + 6 |

---

## 8. Performance Targets

| Metric | Real-Time Mode | High-Fidelity Mode |
|--------|---------------|-------------------|
| Particle Count | 1M – 10M | 50M – 100M |
| Frame Rate | ≥60 FPS | ≥30 FPS (offline OK) |
| GPU Memory | <4 GB VRAM | <12 GB VRAM |
| Startup Time | <5 seconds | <15 seconds |
| Snapshot Export | <2s for 10M particles | <30s for 100M particles |
| Min GPU | GTX 1660 / RX 5600 | RTX 3080 / RX 6800 XT |
| Min CPU | 4-core / 8-thread @ 3 GHz | 8-core / 16-thread @ 3.5 GHz |

---

## 9. Risks & Mitigations

| Risk | Severity | Mitigation |
|------|---------|-----------|
| GPU compute shader portability | High | wgpu abstraction; test NVIDIA, AMD, Intel, Apple Silicon; CPU fallback for critical paths |
| Numerical instability in stiff ODE solvers | Medium | Implicit Rosenbrock methods; validate against PArthENoPE / PRIMAT |
| Memory pressure at high particle counts | High | Adaptive LOD with particle merge/split; disk streaming for offline mode |
| Bevy API churn between releases | Medium | Pin to stable release; isolate Bevy code behind abstraction layer |
| Scope creep into research-grade simulation | Medium | Strict non-goal boundaries; prioritize visual fidelity over numerical precision |
| Phase integration regressions | Medium | Automated visual regression tests at each phase boundary; CI pipeline with screenshot comparison |

---

## 10. Success Metrics

### 10.1 Per-Phase Gates

Each phase must pass these criteria before the next begins:

- Application compiles and runs on all three target platforms (Linux, macOS, Windows)
- Demo Moment is reproducible and visually matches the specification
- No performance regressions from previous phase (±5% FPS tolerance)
- All new UI controls are functional and documented in code comments

### 10.2 Final Release Metrics

- Achieves ≥60 FPS with 1M particles on GTX 1660 class hardware
- Primordial helium abundance within 5% of observed value (Y_p ≈ 0.245)
- CMB power spectrum shape qualitatively matches Planck data at ℓ < 1000
- Full cinematic playback completes in under 10 minutes at default acceleration
- Community adoption: 500+ GitHub stars within 6 months of release
- Educational validation: positive feedback from at least 3 university physics departments

---

## 11. Timeline Summary

| Phase | Duration | Cumulative | Visible Result |
|-------|---------|-----------|---------------|
| P1: Singularity | 2–3 weeks | Week 3 | Particle explosion with camera and timeline |
| P2: Inflation | 3–4 weeks | Week 7 | Physics-driven expansion with density seeds |
| P3: Nucleosynthesis | 3–4 weeks | Week 11 | Element formation with live composition chart |
| P4: Recombination | 3–4 weeks | Week 15 | Fog clearing to reveal CMB sphere |
| P5: Structure | 4–6 weeks | Week 21 | Cosmic web with N-body gravity |
| P6: Cosmic Dawn | 4–5 weeks | Week 26 | First stars, galaxies, reionization |
| P7: Polish | 3–4 weeks | Week 30 | Cinematic mode, full release |

**Total estimated duration: 22–30 weeks (~5–7 months).** The range accounts for the complexity variance in Phase 5 (N-body gravity is the single hardest technical challenge) and Phase 6 (SPH is the second hardest).
