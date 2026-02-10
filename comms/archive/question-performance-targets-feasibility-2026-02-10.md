# Question: Performance Targets Feasibility and Tradeoffs

**Date:** 2026-02-10
**Source:** Architect Session Communication Review

## Ambiguity Identified

The PRD specifies aggressive performance targets for Real-Time Mode:
- ≥60 FPS with 1M particles on GTX 1660 class hardware
- <4 GB VRAM for Real-Time Mode
- N-body gravity + SPH + volumetric rendering + particle systems simultaneously

These targets are ambitious and may require significant optimization or tradeoffs that are not explicitly addressed in the PRD.

## PRD References

### Performance Targets (Section 8)

| Metric | Real-Time Mode | High-Fidelity Mode |
|--------|---------------|-------------------|
| Particle Count | 1M – 10M | 50M – 100M |
| Frame Rate | ≥60 FPS | ≥30 FPS (offline OK) |
| GPU Memory | <4 GB VRAM | <12 GB VRAM |
| Startup Time | <5 seconds | <15 seconds |
| Snapshot Export | <2s for 10M particles | <30s for 100M particles |
| Min GPU | GTX 1660 / RX 5600 | RTX 3080 / RX 6800 XT |
| Min CPU | 4-core / 8-thread @ 3 GHz | 8-core / 16-thread @ 3.5 GHz |

### Success Metrics (Section 10)

> "Achieves ≥60 FPS with 1M particles on GTX 1660 class hardware"

### Phase 5 Deliverables (Heaviest Computational Phase)

> "Direct-sum N-body gravity on GPU (wgpu compute shader) for up to 500K particles as baseline"
> "Barnes-Hut octree (CPU build, GPU traversal) for scaling to 1M–10M particles"
> "Smoothed Particle Hydrodynamics (SPH) with Wendland C4 kernel for baryonic gas dynamics"

### Phase 6 Deliverables

> "Reionization visualization: ionization fronts expand as signed-distance-field bubbles around star-forming halos"
> "Galaxy billboard sprites: halos above mass threshold render as composite galaxy sprites"
> "Volumetric fog renderer: space starts opaque (photon mean free path ≪ horizon), then clears"

## Computational Complexity Analysis

### N-Body Gravity

**Direct-Sum (O(N²)):**
- 1M particles = 10¹² pair interactions per frame
- At 60 FPS = 6 × 10¹³ interactions/second
- Even on GPU, this is computationally prohibitive

**Barnes-Hut (O(N log N)):**
- 1M particles ≈ 1M × log₂(1M) ≈ 20M interactions per frame
- At 60 FPS = 1.2B interactions/second
- Still demanding but feasible with GPU acceleration

**Note:** PRD says "Direct-sum N-body gravity on GPU for up to 500K particles as baseline" but also "Barnes-Hut octree for scaling to 1M–10M particles." This suggests Direct-Sum may be used for ≤500K, Barnes-Hut for >500K.

### SPH (Smoothed Particle Hydrodynamics)

**Neighbor Search (O(N²) naive, O(N) with spatial hashing):**
- For 1M particles with 100 neighbors each: 100M interactions
- Wendland C4 kernel evaluation for each neighbor
- Radiative cooling functions lookup

### Additional Rendering Overheads

- 1M point sprites for particles
- Volumetric fog (raymarching or screen-space)
- CMB sphere rendering
- Galaxy sprites (billboards)
- Ionization bubbles (SDF or sprites)
- UI overlays (timeline, composition chart, validation)

### GTX 1660 Specifications

- CUDA cores: 1408
- Base clock: 1530 MHz, Boost: 1785 MHz
- VRAM: 6 GB (GDDR5, 192-bit)
- Memory bandwidth: 192 GB/s
- Compute performance: ~5 TFLOPS (FP32)

## Feasibility Assessment

### Realistic Expectations

Achieving ≥60 FPS with 1M particles on GTX 1660 with:
- **Barnes-Hut N-body**: Feasible with careful GPU optimization
- **SPH**: Challenging, may require simplified kernels or reduced neighbor counts
- **Volumetric fog**: Depends on implementation technique (see comms/archive/question-volumetric-fog-implementation.md)
- **Ionization bubbles**: Depends on implementation (see comms/archive/question-reionization-sdf-visualization.md)

### Potential Issues

1. **Memory Bandwidth Limitation**: GTX 1660 has 192 GB/s bandwidth. Reading/writing 1M particles × 6 floats × 60 FPS = 1.44 GB/s just for position data, plus velocity, acceleration, etc. Could saturate bandwidth.

2. **VRAM Budget**: 1M particles with position, velocity, mass, composition, etc. = ~20-30 MB per attribute. Full particle state may exceed 4 GB budget when including octree, SPH neighbor lists, rendering buffers.

3. **SPH + N-Body Combined**: Running both simultaneously may exceed 60 FPS unless aggressively optimized or simplified.

4. **Wgpu Portability**: Cross-platform GPU compute via wgpu may have performance overhead compared to CUDA/Vulkan/Compute Shader direct implementations.

## Tradeoff Options

### Option A: Aggressive Optimization, Full Fidelity
- Implement full Barnes-Hut + SPH + volumetric rendering
- Optimize heavily (GPU compute, memory pooling, texture compression)
- Accept that 1M particles may only achieve ~30-40 FPS on GTX 1660
- Higher-end hardware (RTX 3060+) can hit 60 FPS
- Pros: Maximum visual fidelity, scientifically accurate
- Cons: May miss performance target on minimum specified hardware

### Option B: Simplified Physics for 60 FPS
- Use simpler gravity (TreePM, particle-mesh)
- Use simplified SPH (fewer neighbors, cheaper kernel)
- Use simpler volumetric fog (depth-based or screen-space)
- Optimize for 60 FPS on GTX 1660, accept reduced accuracy
- Pros: Meets performance target, good user experience
- Cons: Less scientifically accurate, may conflict with "physically grounded" goal

### Option C: Adjustable Quality Presets
- **High Quality (Real-Time Mode):** Full fidelity, 30-40 FPS on GTX 1660
- **Balanced (Real-Time Mode):** Simplified physics, 60 FPS on GTX 1660
- **Performance:** Further simplified, 60+ FPS on older hardware
- User can select preset in UI
- Pros: Meets target for some preset, flexible, user choice
- Cons: More complex, validation must specify which preset for success metrics

### Option D: Reduce Particle Count Target
- Change target from 1M particles to 500K particles for 60 FPS
- 500K is more realistically achievable with full fidelity on GTX 1660
- Keep 1M+ particles for High-Fidelity Mode on RTX 3080+ hardware
- Pros: More realistic performance targets, still impressive visualization
- Cons: Reduces particle count from PRD specification

### Option E: Time-Sliced Rendering
- Compute physics at lower frequency (e.g., 30 Hz)
- Render at 60 FPS with interpolated positions
- Offload heavy computation to multiple frames
- Pros: Visual smoothness maintained, computational load distributed
- Cons: Physics less accurate, complex implementation, potential artifacts

### Option F: Async GPU Compute + Rendering
- Use separate GPU compute queue for N-body/SPH
- Render while compute for next frame executes
- Overlap computation and rendering
- Pros: Better GPU utilization, potentially 60 FPS
- Cons: Complex synchronization, may introduce latency, wgpu limitations

## Questions for Product Owner

1. **Which priority is higher: Frame rate (60 FPS) or Particle Count (1M)?**
   - Can we reduce particle count to 500K if needed for 60 FPS?
   - Or is 1M particles a hard requirement regardless of FPS?

2. **Can the performance target be hardware-specific?**
   - "60 FPS with 1M particles on RTX 3060" (higher than GTX 1660)?
   - Or accept ~40 FPS on GTX 1660, 60 FPS on RTX 3060+?

3. **Should quality presets be implemented?**
   - Allow users to choose between quality and performance?
   - Which preset defines the "success metric"?

4. **Are there acceptable simplifications?**
   - TreePM instead of full N-body?
   - Simplified SPH with fewer neighbors?
   - Lower-precision floating point (f16 instead of f32)?

5. **Should we validate performance incrementally?**
   - Phase 1-2: 60 FPS with 1M kinematic particles (baseline)
   - Phase 3-4: 60 FPS with 1M particles + additional visual effects
   - Phase 5-6: May drop to 30-40 FPS as physics is added
   - Phase 7: Performance optimization to restore 60 FPS

6. **Is the GTX 1660 a realistic minimum for the full feature set?**
   - Or should we target GTX 1660 for early phases, RTX 3060+ for full experience?
   - Update documentation to reflect realistic expectations?

## Recommendation

Given the computational complexity of N-body + SPH + volumetric rendering, **Option C (Adjustable Quality Presets)** with **Option D (Reduce Particle Count Target)** seems most pragmatic:

1. **Balanced Preset (Default):** 500K particles, simplified physics, 60 FPS on GTX 1660
2. **High Quality Preset:** 1M particles, full physics, 30-40 FPS on GTX 1660, 60 FPS on RTX 3060+
3. Update PRD success metric to reference the "Balanced" preset
4. Document realistic hardware expectations

This allows:
- Meeting the 60 FPS target for GTX 1660 users (with 500K particles)
- Supporting 1M+ particles for users with better hardware
- Clear documentation of what to expect
- User choice based on their hardware and priorities

## Impact

This decision affects:
1. Performance optimization priorities
2. Hardware requirements documentation
3. Phase 5-6 implementation complexity
4. User experience quality (FPS vs. particle count vs. physics accuracy)
5. Success criteria validation (which preset is tested?)
