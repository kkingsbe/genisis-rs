# Question: Phase 5 Cosmic Web Visualization Technique

## Date
2026-02-09

## Context

Phase 5 implements Dark Ages & First Structures, including the cosmic web visualization specified in the PRD.

## Ambiguity Identified

**Phase 5 (Line 204):** "Cosmic web visualization: filaments rendered as line geometry connecting halos, voids rendered as transparent dark regions"

The PRD specifies that filaments should be rendered as "line geometry connecting halos" but provides no implementation details about:
- How to determine which halos are connected
- How to generate the line geometry
- How voids should be rendered as "transparent dark regions"
- How this visualization integrates with the particle system

## Why This Is a Problem

1. **Filament Detection Algorithm Not Specified:**
   - What algorithm determines which halos are connected by filaments?
   - Is it based on proximity, density, or the underlying particle distribution?
   - What's the distance threshold for considering halos connected?
   - Do we use the Friends-of-Friends algorithm (same as halo finder) with different linkage length?

2. **Line Geometry Generation Unclear:**
   - Are lines simple straight segments between halo centers?
   - Or curved paths following the density field?
   - What's the line width and opacity?
   - Should lines be colored by density or mass?

3. **Vold Rendering Not Defined:**
   - How do we render "transparent dark regions" for voids?
   - Are voids just regions without particles/lines?
   - Or do we explicitly render something (e.g., dark spheres, fog)?
   - How do we identify void boundaries?

4. **Interaction with Particle System:**
   - Do we hide particles in voids to emphasize emptiness?
   - Do particles participate in filament visualization, or are lines separate?
   - Should lines be rendered on top of particles, or below?

5. **Performance Concerns:**
   - With thousands of halos and millions of particles, line rendering could be expensive
   - O(N²) distance calculations for filament detection would be prohibitive
   - Need an efficient algorithm to determine connections

6. **Visualization Fidelity vs. Scientific Accuracy:**
   - Should visualization be visually impressive (dense web of lines)?
   - Or scientifically accurate (only true filament connections)?
   - The Friends-of-Friends algorithm may not correspond to visual filaments

## Suggested Approaches

1. **Density-Based Filament Tracing (Recommended for scientific accuracy)**
   - Use particle density field to identify filament paths
   - Trace high-density ridges connecting halos using gradient ascent
   - Generate smooth curved line geometry along density maxima
   - Voids are regions below density threshold (no explicit rendering needed)
   - Pros: Scientifically accurate, visually convincing, follows true structure
   - Cons: Complex to implement (requires density field calculation), computationally expensive

2. **Proximity-Based Line Connections (Simplest)**
   - Connect halos to N nearest neighbors within distance threshold
   - Use simple straight line segments
   - Voids are implicit (regions with no halos/lines)
   - Pros: Very simple to implement, fast, works with any halo count
   - Cons: Not scientifically accurate, visual "spaghetti" if too many connections, requires tuning thresholds

3. **Friends-of-Friends Filament Detection (Uses same algorithm as halo finder)**
   - Reuse Friends-of-Friends algorithm with different linkage length
   - Halos in same FOF group are connected
   - Render lines between group members
   - Pros: Consistent with halo finding, scientifically motivated
   - Cons: FOF groups are blob-like, not filament-like, lines may look clustered not web-like

4. **Minimum Spanning Tree (MST) Approach**
   - Build minimum spanning tree connecting all halos
   - Tree structure naturally selects important connections
   - Prune long edges to create forest (multiple connected components)
   - Pros: Guaranteed no cycles, simple algorithm (O(N log N)), produces clean structure
   - Cons: Not physically motivated, creates tree not web, requires pruning threshold

5. **Voxelized Grid Filament Detection**
   - Voxelize particle positions into 3D grid
   - Apply threshold to identify high-density voxels
   - Use connected components to identify filaments
   - Marching cubes or skeletonization to extract line geometry
   - Pros: Naturally identifies filaments and voids, works well with existing density calculations
   - Cons: Resolution-dependent, memory-intensive, complex post-processing

6. **Hybrid: Halo-Based + Density-Based**
   - Use proximity connections for local filament structure
   - Use density tracing for long-range filament connections
   - Combine both for visually pleasing and reasonably accurate result
   - Pros: Best of both worlds, tunable balance
   - Cons: Most complex, requires two algorithms

## Additional Questions

- **Line Styling:** What should filament lines look like?
  - Color: White? Density-mapped? Mass-mapped?
  - Width: Constant? Tapered at ends?
  - Opacity: Constant? Distance-faded?
  - Glow/blur: Add bloom for visual effect?

- **Vold Visualization:** How should voids be rendered?
  - Implicit (just empty space between filaments/halos)?
  - Explicit (dark spheres or fog)?
  - Labelled (void names, sizes)?

- **User Controls:** Should the cosmic web visualization be:
  - Always visible?
  - Toggleable via UI?
  - Adjustable (connection density threshold, line opacity)?
  - Distance-dependent (fade out when viewing from far)?

- **Performance Targets:** With 1M-10M particles, how many halos and filament lines are acceptable?
  - 100 halos / 500 lines?
  - 1000 halos / 5000 lines?
  - 10,000 halos / 50,000 lines?

## Reference: Related PRD Sections

**Phase 5 Demo Moment (Lines 208-210):**
> "Play through to ~500 million years. The uniform particle field from earlier phases begins to clump. Density perturbations grow under gravity — filaments of matter stretch between growing dark matter halos. By 1 billion years, a recognizable cosmic web has formed: bright nodes (proto-clusters) connected by filaments, separated by vast voids. Fly the camera through the filaments."

The demo moment explicitly mentions "filaments of matter stretch between growing dark matter halos" and "bright nodes (proto-clusters) connected by filaments, separated by vast voids" - this confirms that:
1. Filaments should visually connect halos (bright nodes)
2. Voids should be visible as empty space between filaments
3. The cosmic web should be "recognizable" and users should be able to "fly through the filaments"

## Question for Product Owner

Which cosmic web visualization approach should we use? Given the demo moment description of flying through filaments and the need for a "recognizable cosmic web," should we prioritize:

- Visual impressiveness and clarity (Approach 2: proximity-based lines, simple, produces dense web)
- Scientific accuracy (Approach 1: density-based tracing, accurate but complex)
- Efficiency and simplicity (Approach 4: minimum spanning tree, fast but less accurate)
- Hybrid approach balancing both (Approach 6: most complex but best of both worlds)

Also:
- How should filaments be styled (color, width, opacity)?
- Should voids be explicitly rendered or implicit?
- Should the visualization be always visible or toggleable?
- What are the acceptable performance characteristics (number of halos/lines to render)?
