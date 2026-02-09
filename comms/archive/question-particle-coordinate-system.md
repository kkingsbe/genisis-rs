# Question: Particle Coordinate System Strategy

## Ambiguity Identified
**Phase 2** states: "Particle positions now scale with a(t) — exponential expansion during inflation, decelerating after"

This specification is ambiguous about the coordinate system architecture:
- Should particles be stored in **comoving coordinates** (constant position, universe expands around them)?
- Or in **physical coordinates** (positions explicitly scaled by a(t))?

This is a fundamental architectural decision that affects all subsequent phases (3-7) and the entire simulation pipeline.

## Why This Is a Problem

1. **Physics Implementation**: The Friedmann equation works naturally with comoving coordinates, but visualization and user interaction expect physical (observable) coordinates.

2. **Density Perturbations**: The Zel'dovich approximation (Phase 2) and subsequent structure formation (Phase 5) have very different implementations depending on the coordinate system.

3. **Gravity Calculations**: N-body gravity (Phase 5) requires careful consideration of which coordinate system to use for force calculations.

4. **Timeline Scrubbing**: If using comoving coordinates, scrubbing requires tracking a(t) history. If physical coordinates, requires storing explicit position history.

5. **Rendering**: GPU shaders need to know whether positions are in comoving or physical space for correct rendering.

6. **Phase Transitions**: Each epoch transition may require coordinate transformations if the coordinate system changes.

## Suggested Approaches

1. **Comoving Coordinates with Physical Rendering (Recommended for scientific accuracy)**
   - Store particles in comoving coordinates internally
   - Apply a(t) transformation at render time
   - Physics calculations use comoving space (Friedmann, gravity, perturbations)
   - Pros: Scientifically accurate, matches cosmology literature, efficient for cosmic scales
   - Cons: Requires coordinate transformations for all rendering and interaction, more complex architecture

2. **Physical Coordinates with Comoving Metadata**
   - Store particles in physical coordinates (explicit positions scaled by a(t))
   - Track a(t) as metadata for physics calculations
   - Transform to comoving for physics operations when needed
   - Pros: Intuitive for rendering and user interaction, simpler visualization pipeline
   - Cons: Numerical precision issues at large scales, less scientifically standard, requires inverse transforms for physics

3. **Hybrid System (Phase-Dependent)**
   - Use comoving coordinates for Phase 2-4 (inflation through recombination)
   - Switch to physical coordinates for Phase 5-7 (structure formation)
   - Implement coordinate transformation at epoch boundaries
   - Pros: Optimized for each phase's dominant physics
   - Cons: Complex phase transitions, debugging difficulty, potential for data loss

4. **Dual Coordinate Storage**
   - Store both comoving and physical coordinates for all particles
   - Update both simultaneously with a(t)
   - Use appropriate coordinate system for each operation
   - Pros: Maximum flexibility, no runtime transformation overhead
   - Cons: 2x memory for particle positions, complex synchronization, potential for drift between coordinate systems

## Question for Product Owner
Which coordinate system approach should we use? Given the emphasis on "physically grounded" simulation in the goals, should we prioritize scientific accuracy (Approach 1) or implementation simplicity (Approach 2)? The decision affects multiple phases and should be made before Phase 2 implementation begins.

If a hybrid or dual system is chosen, at which phase boundary should the coordinate system change or how should we handle synchronization?
