# Question: High-Fidelity Mode Performance Targets Feasibility

## Ambiguity Identified
**Performance Targets table** specifies:
- **High-Fidelity Mode**: 50M–100M particles, ≥30 FPS, <12 GB VRAM, Min GPU: RTX 3080 / RX 6800 XT
- **Real-Time Mode**: 1M–10M particles, ≥60 FPS, <4 GB VRAM, Min GPU: GTX 1660 / RX 5600

The High-Fidelity Mode targets (50-100M particles) with full PBR rendering, volumetric effects, particle systems, and complex physics may be unachievable on the specified hardware.

## Why This Is a Problem

1. **Memory Budget Calculation** (rough estimate for 100M particles):
   - Position data: 100M × 12 bytes (vec3) = 1.2 GB
   - Velocity data: 100M × 12 bytes = 1.2 GB
   - Color/energy: 100M × 4 bytes = 400 MB
   - Mass/temperature: 100M × 8 bytes = 800 MB
   - GPU instance buffers: 100M × 32 bytes = 3.2 GB
   - **Particle physics data alone: ~6.8 GB**

2. **Rendering overhead** (additional to particle data):
   - PBR lighting data and textures: 1-2 GB
   - Volumetric fog/reionization SDF: 1-3 GB (depending on resolution)
   - Frame buffers (HDR, depth, etc.): 500 MB - 1 GB
   - Octree/spatial index for Barnes-Hut: 2-4 GB
   - **Rendering overhead: ~4.5-10 GB**

3. **Total estimated VRAM: 11.3-16.8 GB for 100M particles**, exceeding the 12 GB limit on RTX 3080.

4. **Compute cost**:
   - Direct-sum N-body: O(N²) = 10¹⁶ operations per frame at 100M particles
   - Even with Barnes-Hut O(N log N) = ~100M × 30 = 3×10⁹ operations
   - 30 FPS = 33 ms budget per frame
   - Need ~10¹¹ operations/ms = impossible on current GPUs

5. **Historical context**: Research codes (Gadget-4, AREPO) typically simulate ~10M particles on GPU clusters, not 100M on single GPU.

## Suggested Approaches

1. **Reduce High-Fidelity Particle Count (Recommended)**
   - Change High-Fidelity mode to 10M-20M particles instead of 50M-100M
   - Keep ≥30 FPS target and RTX 3080 minimum
   - Update Performance Targets table accordingly
   - Pros: Feasible, still significant improvement over Real-Time mode, realistic
   - Cons: Less impressive "wow factor" in marketing, reduces maximum achievable fidelity

2. **Accept Reduced Frame Rate or Quality**
   - Keep 50M-100M particle target
   - Accept 10-15 FPS instead of ≥30 FPS in High-Fidelity mode
   - Or reduce visual quality (disable PBR, volumetric effects) at high particle counts
   - Pros: Achieves particle count target, maintains visual quality option
   - Cons: "≥30 FPS" specification violated, poor user experience at high counts

3. **Increase Minimum GPU Requirement**
   - Keep 50M-100M particles and ≥30 FPS target
   - Change minimum GPU to RTX 4090 / RX 7900 XTX (24 GB VRAM)
   - Update Performance Targets table
   - Pros: Achieves all targets, uses cutting-edge hardware
   - Cons: Reduces accessibility, narrows user base, higher cost barrier

4. **Quality Scaling System**
   - Implement adaptive quality that degrades gracefully as particle count increases
   - At 10M: Full PBR, volumetric fog, reionization SDF
   - At 30M: Reduced PBR quality, simplified volumetric fog
   - At 50M+: Basic lighting, no volumetric effects, particle-only rendering
   - Pros: Maximizes particle count while maintaining usability
   - Cons: Visual quality varies significantly, no "single" high-fidelity experience

5. **CPU Offloading for Physics**
   - Use GPU for rendering only, CPU for physics computation
   - Requires massive CPU (32+ cores) for 50M+ particles
   - Physics computed slower than real-time, but rendering uses cached states
   - Pros: Reduces GPU compute pressure, focuses GPU on rendering
   - Cons: CPU bottleneck, expensive hardware requirement, real-time constraint violated

6. **Accept Non-Real-Time High-Fidelity Mode**
   - Rename "High-Fidelity Mode" to "Offline/Cinematic Mode"
   - Target 10-15 FPS for 50-100M particles
   - Frame playback (not interactive) for high-quality rendering
   - Pros: Achieves particle count target, maintains visual quality
   - Cons: Not real-time, different from PRD spec, reduces interactivity

## Reference: Real-Time Mode Analysis
Real-Time mode (1M-10M particles, ≥60 FPS, <4 GB VRAM) is achievable with current approaches:
- 10M particles: ~680 MB for physics data
- Rendering overhead: ~1-2 GB
- Total: ~1.7-2.7 GB (well under 4 GB)
- Barnes-Hut: 10M × 30 = 3×10⁸ operations at 60 FPS = 5×10⁶ ops/ms (feasible on modern GPU)

## Question for Product Owner
The High-Fidelity mode targets (50-100M particles at ≥30 FPS on RTX 3080 with 12 GB VRAM) appear unachievable with full visual fidelity. Should we:

1. Reduce particle count target to 10M-20M (Approach 1)?
2. Accept 10-15 FPS or reduced quality at high particle counts (Approach 2)?
3. Increase minimum GPU requirement to RTX 4090 / RX 7900 XTX (Approach 3)?
4. Implement adaptive quality scaling system (Approach 4)?
5. Change High-Fidelity to "Offline/Cinematic Mode" with non-real-time rendering (Approach 6)?

This decision affects the marketing positioning of Genesis and what we promise users. The PRD emphasizes "real-time interactive frame rates" which conflicts with 50-100M particle targets.
