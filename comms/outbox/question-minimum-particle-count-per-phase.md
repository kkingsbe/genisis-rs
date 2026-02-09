# Question: Minimum Particle Count Requirements per Phase

## Ambiguity Identified
The PRD specifies target particle counts for the final application (1M-10M Real-Time, 50M-100M High-Fidelity), but does not specify **minimum particle count requirements** for each phase to be considered "complete".

This creates ambiguity for:
- Phase completion criteria
- Performance testing and validation
- Determining when a phase is "done"
- Resource allocation per phase

## Why This Is a Problem

1. **Phase Completion Definition**: Without a minimum particle count, Phase 1 could be considered "complete" with just 100 particles. The "Demo Moment" doesn't specify particle count.

2. **Incremental Validation**: Each phase's "success metrics" don't include particle count validation. We need to know what particle count to target during development.

3. **Performance Regression Testing**: Without per-phase particle count targets, we can't establish performance baselines for regression testing.

4. **Development Focus**: Developers need to know the particle count target for each phase to optimize appropriately.

5. **Demo Credibility**: A demo with too few particles won't be impressive. We need to balance impressiveness with development effort.

## Current Particle Count Mentions in PRD

- **Phase 1**: "Instanced particle renderer capable of displaying 100K–1M point sprites" (capability, not requirement)
- **Phase 1 Demo Moment**: "dense, glowing white-hot cluster of particles" (no count specified)
- **Phase 5**: "Direct-sum N-body... for up to 500K particles as baseline", "Barnes-Hut... for scaling to 1M–10M particles"
- **Performance Targets**: Final application: 1M-10M Real-Time, 50M-100M High-Fidelity

## Suggested Approaches

1. **Progressive Scaling Targets (Recommended)**
   - Phase 1: 10K-50K particles (prove rendering pipeline works)
   - Phase 2: 50K-100K particles (demonstrate inflation visualization)
   - Phase 3: 100K-200K particles (nucleosynthesis visualization)
   - Phase 4: 200K-500K particles (CMB visualization)
   - Phase 5: 500K-1M particles (structure formation baseline)
   - Phase 6: 1M-2M particles (galaxy formation visualization)
   - Phase 7: 2M-5M particles for Real-Time mode, optimize for higher counts
   - Pros: Gradual increase, allows optimization focus per phase, achievable
   - Cons: Lower than final targets, requires late-phase optimization work

2. **Final Target from Phase 1 (Aggressive)**
   - All phases target 1M+ particles from Phase 1 onwards
   - Phase 1: 1M particles (prove renderer can handle it)
   - Phase 2-7: 1M-5M particles (varying complexity)
   - Pros: Early validation of scalability, no late-phase surprises
   - Cons: Phase 1 becomes very complex, may slow early development

3. **Phase-Specific Practical Limits**
   - Phase 1: 100K particles (simple rendering, no physics)
   - Phase 2: 100K particles (inflation physics)
   - Phase 3: 50K particles (stiff ODE solver is computationally expensive)
   - Phase 4: 200K particles (volumetric fog overhead)
   - Phase 5: 500K particles (N-body baseline, add Barnes-Hut optimization)
   - Phase 6: 300K particles (SPH is expensive per particle)
   - Phase 7: Scale to 1M+ (optimization phase)
   - Pros: Realistic limits based on physics complexity, achievable
   - Cons: Lower particle counts in some phases, less impressive demos

4. **Demo-Mode vs. Real-Time Mode**
   - Demo/visualization mode: 10K-100K particles (optimized for visual demo)
   - Real-Time interactive mode: 1M+ particles (full physics)
   - Each phase implements both modes
   - Phase completion requires both modes working
   - Pros: Great demos for presentations, still meets Real-Time targets
   - Cons: Two code paths to maintain, more development work

5. **No Per-Phase Minimum, Final Only**
   - No minimum particle count per phase
   - Each phase completes when functional (any particle count works)
   - Particle count optimization happens in Phase 7
   - Pros: Maximum flexibility, phases can complete faster
   - Cons: Risk of late-phase performance issues, no early validation

## Additional Considerations

- **Testing**: Should we have automated performance tests at each phase?
- **Documentation**: Should we document "achievable particle count" for each phase on each hardware tier?
- **User Setting**: Should particle count be a user-configurable setting with recommended ranges per phase?

## Question for Product Owner
What should the minimum particle count be for each phase to consider it "complete"?

- Should we use progressive scaling (Approach 1) with gradually increasing targets?
- Or should we target 1M+ from Phase 1 (Approach 2)?
- Or should we use phase-specific limits based on physics complexity (Approach 3)?
- Or separate demo-mode from real-time mode (Approach 4)?

Also, should there be per-phase performance tests that validate minimum particle count and FPS?
