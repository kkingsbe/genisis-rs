# Question: Reionization Visualization Technique - SDF Bubbles

## Ambiguity Identified
**Phase 6** specifies: "Reionization visualization: ionization fronts expand as signed-distance-field bubbles around star-forming halos"

The PRD specifies "signed-distance-field bubbles" for reionization visualization, but this is ambiguous about:

1. How the SDF is generated and updated
2. How multiple bubbles interact and merge
3. How the SDF is rendered
4. The computational feasibility for 1M-10M particles

## Why This Is a Problem

1. **SDF Complexity**: Computing a true signed distance field for dynamically expanding, merging, and overlapping ionization bubbles is computationally expensive.

2. **Dynamic Updates**: Each frame, as ionization fronts expand and new star sources ignite, the SDF must be recomputed. For 1M+ particles and thousands of ionization sources, this is a major performance challenge.

3. **Storage Requirements**: A 3D SDF at sufficient resolution would require significant GPU memory, competing with particle storage.

4. **GPU vs CPU Computing**: SDF generation is typically done on CPU for accuracy, but then requires GPU upload. Doing it on GPU is complex for signed distance calculations.

5. **Rendering Complexity**: Rendering from SDF requires raymarching or iso-surface extraction (marching cubes), both computationally expensive.

6. **Interaction with Particles**: How does the SDF visualization interact with the underlying particle system? Are particles hidden/colored based on their ionization state?

## Suggested Approaches

1. **Per-Particle Ionization State (Simplified "SDF" approach)**
   - Track ionization state for each particle: neutral, partially ionized, fully ionized
   - For each particle, compute distance to nearest ionization source
   - Render particles with color/intensity based on ionization state and distance
   - Use visual tricks (additive blending, glow sprites) to simulate expanding bubbles
   - Pros: Integrates with particle system, no separate SDF data structure, efficient
   - Cons: Not a true SDF, may not look like "bubbles", requires per-particle distance calculation

2. **GPU-Computed Distance Field with Approximation**
   - Store ionization source positions (halos with star formation) on GPU
   - In fragment shader, for each pixel, compute minimum distance to all ionization sources
   - Apply ionization radius threshold for each source
   - Combine using min() or other blending function
   - Pros: True distance calculation, supports expanding bubbles, GPU-optimized
   - Cons: O(N_sources) per pixel, limited number of sources (~100-1000), no offline data for analysis

3. **Voxel-Based SDF on GPU**
   - Create 3D voxel grid storing distance to nearest ionization source
   - Use jump flooding algorithm (JFA) or parallel BFS on GPU to compute SDF
   - Update incrementally as sources expand
   - Render using volume raymarching or iso-surface extraction
   - Pros: True SDF, supports merging bubbles, high visual quality
   - Cons: High GPU memory, complex GPU algorithms, performance impact, resolution limits

4. **Visual Simulation Using Sprite Bubbles**
   - Render large sprite quads centered on each ionization source
   - Size grows as ionization front expands
   - Use additive blending so overlapping regions appear brighter/more ionized
   - Not a true SDF, but achieves the visual effect of expanding bubbles
   - Pros: Very fast, simple implementation, good visual effect for presentation
   - Cons: Not scientifically accurate, no distance field data, artifacts at overlaps

5. **2D Slice Visualization**
   - Compute SDF only for 2D cross-sections or specific planes
   - Allow user to view reionization progress on slices through the volume
   - Use CPU computation, upload texture to GPU for display
   - Pros: Reduces computational complexity (2D vs 3D), easier to implement
   - Cons: Only shows slices, not full 3D volume, requires user interaction to see different views

6. **Static Pre-computed Reionization Maps**
   - Pre-compute ionization fraction at multiple time steps
   - Store as 3D textures and blend between them based on simulation time
   - No dynamic calculation at runtime
   - Pros: Zero runtime cost, smooth playback
   - Cons: Not interactive, can't respond to user parameter changes, requires pre-generation

## Additional Questions

- **Bubble merging**: When ionization bubbles overlap, should they:
  - Merge smoothly into one larger ionized region (union of SDFs)?
  - Remain distinct with visible boundaries?
  - Have interaction effects (fronts slow down in overlapping regions)?

- **Expansion rate**: Is the expansion rate:
  - Physically accurate (based on star formation rate and photon output)?
  - User-configurable parameter?
  - Fixed linear expansion?

- **Interaction with particles**: Should the ionization visualization:
  - Overlay on particles (particles still visible)?
  - Replace particle rendering in ionized regions?
  - Change particle appearance (neutral vs ionized particles)?

## Question for Product Owner
For the reionization "SDF bubbles" visualization, what level of scientific accuracy vs. visual effect is required?

- If visual presentation effect is priority: Use Approach 4 (sprite bubbles) or Approach 2 (per-particle state)
- If scientific accuracy with true SDF is required: Use Approach 3 (voxel-based SDF) but accept performance cost
- If interactive exploration is priority: Use Approach 1 (per-particle) or Approach 5 (2D slices)
- If pre-authored cinematic playback is priority: Use Approach 6 (pre-computed maps)

Also, how should bubbles interact when they overlap, and should the visualization show particles, replace particles, or modify particle appearance?
