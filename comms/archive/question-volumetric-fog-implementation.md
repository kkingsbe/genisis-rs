# Question: Volumetric Fog Implementation Technique

## Ambiguity Identified
**Phase 4** specifies: "Volumetric fog renderer: space starts opaque (photon mean free path ≪ horizon), then clears as x_e drops below threshold"

The PRD requires a volumetric fog that starts opaque and clears as recombination completes, but does not specify which rendering technique to use.

## Why This Is a Problem

1. **Technical Complexity**: Volumetric fog is a sophisticated graphics technique with multiple implementation options, each with different performance characteristics and visual quality.

2. **Performance Impact**: With 1M-100M particles already rendering, adding volumetric fog could severely impact the ≥60 FPS target.

3. **Integration Difficulty**: The fog must interact with existing particle systems, CMB sphere rendering, and camera systems.

4. **Cross-Platform Concerns**: Not all volumetric fog techniques work equally well across all GPU architectures (NVIDIA, AMD, Apple Silicon, Intel).

5. **Memory Budget**: Some techniques require additional textures or buffers that compete with particle GPU memory.

## Suggested Approaches

1. **Raymarching with Distance Field (Recommended for quality)**
   - Use 3D texture storing fog density
   - Raymarch through volume in fragment shader
   - Density controlled by x_e (ionization fraction)
   - Pros: High visual quality, physically accurate light transport
   - Cons: Computationally expensive, may impact performance significantly

2. **Depth-Based Fog (Simplest, fastest)**
   - Use depth buffer to calculate distance from camera
   - Apply exponential fog based on distance and x_e
   - Standard approach: fog_color * (1 - exp(-distance * density))
   - Pros: Very fast, minimal GPU cost, widely supported
   - Cons: Not truly volumetric, limited visual effect, no light scattering

3. **Screen-Space Volumetric Fog (Balanced approach)**
   - Use screen-space techniques to approximate volumetric scattering
   - Apply in post-processing pass using depth buffer and noise
   - Pros: Good visual quality, moderate performance cost, well-supported
   - Cons: Not physically accurate, can have artifacts at screen edges

4. **Particle-Based Atmospheric Scattering**
   - Add additional particles representing atmospheric medium
   - Use billboards with soft blending
   - Density controlled by x_e and particle count
   - Pros: Integrates with existing particle system, visually interesting
   - Cons: Not true volumetric fog, may look like "dust" rather than atmospheric fog

5. **Simplified Visual Transition (Alternative to true volumetric fog)**
   - Use full-screen fade with animated noise texture
   - Fade from white/opaque to clear/transparent
   - Not true volumetric fog but achieves the visual effect
   - Pros: Very fast, easy to implement, minimal performance impact
   - Cons: Not volumetric, limited depth perception

## Question for Product Owner
What level of visual fidelity is required for the volumetric fog effect? Is the physically accurate light transport and depth perception essential (justifying Approach 1), or would the visual effect of a screen-fade be sufficient (Approach 5)? The middle options (2-4) offer various tradeoffs between quality and performance.

Also, what is the acceptable performance budget for the fog system? Should it be:
- Free: Use Approach 5 (screen fade)
- Low: Use Approach 2 (depth-based fog)
- Medium: Use Approach 3 or 4 (screen-space or particle-based)
- High: Use Approach 1 (raymarching), accepting potential FPS impact
