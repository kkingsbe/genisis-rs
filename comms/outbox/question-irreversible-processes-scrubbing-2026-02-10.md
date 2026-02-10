# Question: Irreversible Physical Processes and Timeline Scrubbing

**Date:** 2026-02-10
**Source:** Architect Session Communication Review

## Ambiguity Identified

The PRD requires both:
1. **Real-time physics simulation** with irreversible processes (nucleosynthesis, star formation, galaxy assembly)
2. **Timeline scrubbing** that allows users to "scrub the timeline back and forth — the expansion reverses and replays"

These requirements are fundamentally in conflict. Irreversible processes cannot be "reversed" by simply moving a timeline slider.

## PRD References

### Timeline Scrubbing Requirements

**Phase 1 Demo Moment:**
> "Scrub the timeline back and forth — the expansion reverses and replays."

**Phase 3 Demo Moment:**
> "After 20 minutes of cosmic time, the chart stabilizes — toggle the validation overlay and see your simulated abundances line up with observed values. Switch to 'High Baryon Density' preset and watch helium overshoot."

**Phase 4 Demo Moment:**
> "Play through to 380,000 years. The universe is an opaque glowing fog... Then the fog begins to clear... As recombination completes, the 'fog lifts' and the camera pulls back to reveal the CMB sphere"

**Phase 5 Demo Moment:**
> "Play through to ~500 million years. The uniform particle field from earlier phases begins to clump... By 1 billion years, a recognizable cosmic web has formed"

### Irreversible Processes in the Simulation

**Phase 3 (Nucleosynthesis):**
- 12-species nuclear reaction network (n, p, D, T, ³He, ⁴He, ⁷Li, ⁷Be)
- "Deuterium spikes briefly then burns away"
- Nuclear reactions are thermodynamically irreversible

**Phase 5 (Dark Ages):**
- "Halo finder (Friends-of-Friends algorithm) identifying collapsed structures in real time"
- "Filaments rendered as line geometry connecting halos, voids rendered as transparent dark regions"
- Structure formation involves gravitational collapse which is not easily reversible

**Phase 6 (Cosmic Dawn):**
- "Pop III star formation in early halos"
- "Reionization visualization: ionization fronts expand as signed-distance-field bubbles"
- "Galaxy billboard sprites: halos above mass threshold render as composite galaxy sprites with morphology based on merger history"
- Star formation, ionization, and galaxy mergers are irreversible processes

## The Problem

When a user scrubs the timeline backward from:
- Post-nucleosynthesis to pre-nucleosynthesis: How does deuterium "un-burn"?
- Post-structure formation to pre-collapse: Do halos "un-collapse" and filaments "un-form"?
- Post-reionization to neutral medium: Do ionization bubbles "shrink"?
- Post-galaxy formation to dark ages: Do galaxies "disassemble" back into dark matter halos?

## Suggested Approaches

### Option A: Snapshot-Based State Restoration (Most Comprehensive)
- Capture full simulation state snapshots at regular intervals
- When scrubbing backward, restore nearest snapshot and replay forward
- Requires storing: particle positions/velocities, composition, halo structures, ionization field, galaxy sprites, etc.
- Pros: Enables true timeline scrubbing, matches PRD requirement, preserves all visual states
- Cons: High memory overhead (millions of particles × many snapshots), complex state management, storage for complex structures (halos, filaments, galaxies)

### Option B: Kinematic Reversal for Simple Processes Only
- For Phase 1-2 (kinematic expansion): Reverse motion using initial positions/velocities
- For Phase 3+ (irreversible processes): Disable reverse scrubbing, show message "Cannot scrub past irreversible processes"
- Timeline still shows full range, but only forward playback works beyond certain points
- Pros: Simple, memory-efficient, clear limitation communicated to user
- Cons: Doesn't match PRD requirement ("scrub back and forth"), inconsistent user experience

### Option C: Visual Approximation (Best Visual Experience, Not Physically Accurate)
- For backward scrubbing, use visual approximations that look plausible:
  - Deuterium "grows back" by reversing the abundance curve mathematically
  - Halos "shrink" by reversing the density field evolution
  - Ionization bubbles "contract" by reversing radius calculation
  - Galaxy sprites "fade out" based on time parameter
- Physics not truly reversed, but visualization looks correct
- Pros: Smooth scrubbing experience, no memory overhead, good for demo/presentation
- Cons: Not scientifically accurate, violates "physically grounded" principle, could be confusing if scrutinized

### Option D: Continuous Simulation with Rewind Parameter (Most Physically Accurate, Most Complex)
- Simulate both forward and backward evolution simultaneously
- Use time-reversal-symmetric equations where possible (e.g., gravity is symmetric)
- For truly irreversible processes, use alternative models that support backward evolution:
  - Instead of reaction networks, use analytic abundance curves Y_p(t) that are mathematically invertible
  - Instead ofFriends-of-Friends halo finding, use density threshold that can be reversed
  - Instead of ionization bubbles, use analytic ionization fraction field
- Pros: Physically grounded, supports true scrubbing, consistent physics forward and backward
- Cons: Extremely complex to implement, requires alternative physics models for irreversible processes, significant development effort

### Option E: Restrict Scrubbing Scope (Simplifies User Experience)
- Allow scrubbing only within "safe" epochs where physics is reversible or approximately reversible
- Phase 1-2: Full scrubbing (kinematic)
- Phase 3: Scrubbing limited to nucleosynthesis window (3-20 min), cannot go before
- Phase 4: Scrubbing limited to recombination window, cannot go before
- Phase 5-6: Forward-only playback for structure/galaxy formation
- Visual indicator shows scrubbing limits on timeline
- Pros: Clear boundaries, prevents confusing behavior, realistic limitations
- Cons: Doesn't match PRD requirement for full "back and forth" scrubbing, frustrating if user wants to explore full timeline

## Additional Considerations

### Memory Requirements for Snapshot Approach (Option A)

If we implement full snapshots:
- 1M particles × 6 floats (position + velocity) × 100 snapshots ≈ 2.4 GB
- Plus composition data, halo structures, ionization fields
- This may exceed the <4 GB VRAM budget for Real-Time Mode

### Scientific Accuracy Tradeoffs

| Approach | Scientific Accuracy | Implementation Complexity | User Experience |
|----------|---------------------|---------------------------|-----------------|
| A: Snapshots | High (true state restoration) | High | Excellent (full scrubbing) |
| B: Partial Scrubbing | High (accurate forward, no reverse) | Low | Poor (limited scrubbing) |
| C: Visual Approx | Low (physics broken) | Medium | Good (smooth scrubbing) |
| D: Continuous Reversible | Highest (time-symmetric physics) | Very High | Excellent (full scrubbing) |
| E: Scoped Scrubbing | High (accurate where possible) | Low-Medium | Medium (partial scrubbing) |

## Questions for Product Owner

1. **What is the priority for timeline scrubbing capability?**
   - Is full "back and forth" scrubbing essential for the user experience?
   - Would partial scrubbing (with clear limitations) be acceptable?
   - Is smooth visual presentation more important than physical accuracy during reverse scrubbing?

2. **How should irreversible processes be handled?**
   - Should we store snapshots and restore state (memory intensive)?
   - Should we use visual approximations that look plausible (physically inaccurate)?
   - Should we disable reverse scrubbing past certain epochs (clear limitation)?

3. **What is the memory budget for timeline scrubbing?**
   - Can we allocate additional VRAM for snapshot storage?
   - Should scrubbing work only on high-fidelity hardware?
   - Is disk-based snapshot storage acceptable (slower but no memory limit)?

## Recommendation

Given the PRD's emphasis on "physically grounded" simulation AND "Demo Moment" experiences that involve full timeline exploration, **Option A (Snapshot-Based State Restoration)** or **Option E (Restrict Scrubbing Scope)** seem most appropriate:

- **Option A** if memory budget allows and full scrubbing is critical
- **Option E** if memory is constrained and clear limitations are acceptable

Option C (Visual Approximation) provides a good middle ground for demo/presentation use cases but should be documented as a visual effect rather than physically accurate physics.

## Impact

This decision affects:
1. Timeline scrubbing implementation (snapshot system vs. kinematic reversal vs. visual approximation)
2. Memory budget allocation (snapshots vs. particles vs. rendering)
3. Physics model complexity (reversible vs. irreversible models)
4. User experience design (full scrubbing vs. limited scrubbing vs. no reverse)
5. Demo Moment completeness (can user experience the full story as PRD describes?)
