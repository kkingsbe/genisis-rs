# Question: Zel'dovich Approximation vs. Nonlinear Structure Formation

## Ambiguity Identified
**Phase 2** states: "Density perturbations mapped to particle displacement (Zel'dovich approximation)"

**Phase 5** states: "Direct-sum N-body gravity... grow dark matter halos from the density perturbations... halo finder (Friends-of-Friends algorithm) identifying collapsed structures"

There is a fundamental conflict: The Zel'dovich approximation used in Phase 2 is only valid in the **linear regime** (small density perturbations δ ≪ 1). However, Phase 5 expects **nonlinear structure formation** where halos collapse, filaments form, and δ ≫ 1.

## Why This Is a Problem

1. **Physics Incompatibility**: Zel'dovich approximation breaks down as density perturbations grow. Using it to seed particles that later undergo N-body collapse will produce incorrect initial conditions.

2. **Transition Point**: At what point does the simulation switch from Zel'dovich (linear) to N-body (nonlinear)? The PRD doesn't specify this critical transition condition.

3. **Seed Consistency**: If particles are initialized with Zel'dovich displacements, then N-body takes over, the initial conditions may not produce the expected halo structure.

4. **Scientific Accuracy**: The "physically grounded" goal requires correct transition from linear growth to nonlinear collapse.

## Suggested Approaches

1. **Zel'dovich for Linear Phase Only, Full N-Build from Inflation Start (Recommended)**
   - Use Zel'dovich approximation for visualization in Phase 2-4
   - For Phase 5, do NOT use Zel'dovich-seeded particles
   - Instead, re-seed particles with proper Gaussian random field
   - Run full N-body from the start of inflation, not from Zel'dovich output
   - Pros: Physically accurate, consistent physics throughout, correct halo formation
   - Cons: Can't "continue" from Phase 2-4 visualization; Phase 5 requires restart or precomputation

2. **Hybrid with Smooth Transition**
   - Use Zel'dovich for Phase 2-4 (linear regime, δ < 0.1)
   - At δ = 0.1 threshold, smoothly transition to N-body
   - Use Zel'dovich particle positions as initial conditions for N-body
   - Pros: Continuity between phases, scientifically valid for both regimes
   - Cons: Complex transition logic, potential for visual discontinuity, need to track δ everywhere

3. **Phase Restart Between 4 and 5**
   - Phase 1-4: Use Zel'dovich for visualization (not scientifically accurate structure)
   - Between Phase 4 and 5: Clear simulation and restart with proper N-body initial conditions
   - User sees seamless transition (visual crossfade), but simulation resets
   - Pros: Each phase optimized for its purpose, no physics incompatibility
   - Cons: Not truly "continuous" simulation, Phase 5 doesn't grow from Phase 2 seeds

4. **Zel'dovich with Nonlinear Extensions**
   - Use higher-order extensions like 2nd-order Lagrangian perturbation theory (2LPT)
   - 2LPT remains valid into mildly nonlinear regime (δ ~ 1)
   - Transition to full N-body when δ exceeds threshold
   - Pros: More accurate than Zel'dovich, extends linear regime validity
   - Cons: More complex to implement, still breaks down in fully nonlinear regime

5. **Accept Scientific Inaccuracy**
   - Use Zel'dovich seeds for Phase 5 despite physics incompatibility
   - N-body will still produce halos and filaments, but from incorrect initial conditions
   - Adjust parameters (σ₈, random seed) to produce visually plausible results
   - Pros: Simpler implementation, continuous simulation across all phases
   - Cons: Not scientifically accurate, violates "physically grounded" goal, halo properties will be wrong

## Additional Context

The PRD's success metric specifies: "CMB power spectrum shape qualitatively matches Planck data at ℓ < 1000" - this requires accurate linear perturbation growth. However, "Dark matter halos... collapsed structures" requires nonlinear physics. The two are fundamentally different regimes requiring different approaches.

## Question for Product Owner
Given the fundamental incompatibility between Zel'dovich (linear) and N-body (nonlinear), should we:

1. Restart simulation between Phase 4 and 5 with proper N-body initial conditions (Approach 3)?
2. Implement a hybrid system with smooth transition at δ threshold (Approach 2)?
3. Use higher-order perturbation theory (2LPT) to extend the linear regime (Approach 4)?
4. Accept scientific inaccuracy and use Zel'dovich seeds despite physics issues (Approach 5)?
5. Run full N-body from inflation start, using Zel'dovich only for Phase 2-4 visualization (Approach 1)?

The choice affects the "continuous simulation" narrative and the scientific accuracy of the results. The PRD emphasizes both "physically grounded" simulation AND "continuous story" through all phases.
