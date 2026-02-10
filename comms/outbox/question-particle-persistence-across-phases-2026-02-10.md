# Question: Particle Identity and State Persistence Across Phases

**Date:** 2026-02-10
**Source:** Architect Session Communication Review

## Context

The PRD describes particles in multiple phases, but it is ambiguous whether these are the same particles persisting with identity across phases, or whether particles are regenerated for each phase.

### PRD References by Phase

**Phase 1 (Singularity):**
> "A procedural 'singularity' visualization: particles spawned at origin with outward velocity, color-mapped by energy (white-hot core fading to red)"

**Phase 2 (Inflation):**
> "Particle positions now scale with a(t) — exponential expansion during inflation, decelerating after"

**Phase 3 (Nucleosynthesis):**
> "individual particles are color-coded by dominant composition (hydrogen = blue, helium = yellow, lithium = faint pink)"

**Phase 4 (Recombination):**
> (No explicit particle changes mentioned; focuses on fog clearing and CMB)

**Phase 5 (Structure):**
> "Dark matter particles seeded from Phase 2 perturbation field; baryonic particles coupled"

**Phase 6 (Cosmic Dawn):**
> "Sub-grid star formation: Kennicutt-Schmidt relation converts dense gas into star particles"
> "Pop III star formation in early halos; first light sources appear as bright point lights"

### Architectural Concerns

This ambiguity affects multiple design decisions:

1. **State Persistence:** If particles persist, what state is preserved during phase transitions? Position? Velocity? Composition? Mass?

2. **Particle Count Changes:** Phase 1 mentions 100K–1M particles, Phase 5 mentions up to 10M particles. When does the particle count increase? Are particles split/merged?

3. **Phase Transitions:** The Epoch Plugin Architecture (Section 4.1) says "exactly one epoch's physics is active while all completed epochs' visual outputs remain available for timeline scrubbing." This suggests some visualization persistence, but does it apply to particles?

4. **Cross-Epoch Visualization:** The PRD mentions timeline scrubbing between phases. If particles are regenerated, how does scrubbing work? Do we store snapshots?

5. **Particle Identity:** If particles are the same entities throughout, they need attributes that can evolve (energy, composition, ionization state, etc.). If regenerated, each phase can define its own particle attributes.

## Suggested Approaches

### Option A: Persistent Particles with Evolving Attributes
- Particles are created once in Phase 1 and persist through all phases
- Each particle has extensible attributes: position, velocity, mass, energy, composition, ionization state, etc.
- Phase transitions add/modify attributes but don't recreate particles
- Particle count increases via splitting (high-density regions) and decreases via merging (voids)
- Pros: True continuous simulation, timeline scrubbing works naturally, matches "continuous story" narrative
- Cons: Complex attribute management, must handle attribute addition/removal, memory overhead for unused attributes

### Option B: Regenerate Particles Per Phase
- Each phase creates its own particle system optimized for that epoch's visualization
- Phase 1-2: Kinematic particles for singularity/inflation
- Phase 3: Composition-coded particles for nucleosynthesis
- Phase 5: Dark matter + baryonic particles for structure formation
- Pros: Each phase optimized, simpler code, no backward compatibility concerns
- Cons: Scrubbing requires snapshot storage, no true continuity, "restarts" between phases

### Option C: Hybrid with Key Snapshot Points
- Particles persist within "physics eras" but regenerate between major transitions
- Era 1 (Phases 1-2): Kinematic + scale factor visualization
- Era 2 (Phase 3-4): Composition + recombination visualization
- Era 3 (Phases 5-6): N-body + SPH visualization
- At era boundaries, regenerate particles with initial conditions from previous era's state
- Pros: Balances continuity and flexibility, manageable complexity
- Cons: Requires era transition logic, some "resets" at boundaries

### Option D: Full Regeneration with Visual Continuity
- Each phase generates its own particle system
- Use crossfade and parameter interpolation at transitions to simulate continuity
- Timeline scrubbing uses pre-saved snapshots from each phase's particle system
- Pros: Maximum flexibility, each phase can be completely independent
- Cons: No true particle identity, requires snapshot system, visually continuous but not physically

## Questions for Product Owner

1. **Which approach aligns with your vision for the simulation?** The choice affects the architecture and user experience significantly.

2. **Is particle identity important for the narrative?** Should users feel they're watching "the same universe" evolve, or is visual continuity sufficient?

3. **How should timeline scrubbing work?**
   - If particles persist: Scrubbing works by recomputing particle states from time parameter
   - If particles regenerate: Scrubbing requires snapshot storage and restoration
   - If hybrid: Scrubbing works within eras, but era boundaries require special handling

4. **What is the priority for scientific accuracy vs. implementation simplicity?**
   - Persistent particles (Option A) is most accurate but most complex
   - Regeneration (Option B) is simplest but least accurate
   - Hybrid approaches (C, D) offer tradeoffs

## Recommendation

Based on the PRD's emphasis on "physically grounded" simulation, "continuous story" through all phases, and timeline scrubbing capability, **Option A (Persistent Particles with Evolving Attributes)** seems most aligned with the vision. However, this requires careful architectural planning to manage the complexity.

If Option A is too complex, **Option C (Hybrid with Key Snapshot Points)** provides a reasonable middle ground: continuity within physics eras, with clear transition points where regeneration is acceptable.

## Impact

This decision affects:
1. Particle data structure design (persistent vs. per-phase)
2. Epoch plugin architecture (shared particle storage vs. independent systems)
3. Timeline scrubbing implementation (recompute vs. restore)
4. Phase transition handling (smooth vs. restart)
5. Memory management (shared particle pool vs. per-phase allocation)
