# Question: Particle Count Performance Target Ambiguity

**Context:** Task 4 - PRD Ambiguity Check

## Issue Identified

The PRD contains inconsistent particle count specifications across different sections:

1. **Phase 1 (Line 113):** "Instanced particle renderer capable of displaying **100K–1M** point sprites"
2. **Performance Table (Line 298):** "Real-Time Mode: **1M – 10M** particles"
3. **Phase 5 (Line 199):** "Direct-sum N-body gravity on GPU for up to **500K** particles as baseline"
4. **Phase 5 (Line 200):** "Barnes-Hut octree... for scaling to **1M–10M** particles"

## Questions for Clarification

1. **What is the definitive target particle count for real-time performance?**
   - Should Phase 1 target 100K-1M or match the performance table's 1M-10M?
   - What particle count should be considered the "baseline" for N-body gravity?

2. **What is the expected particle count progression across phases?**
   - Should particle counts increase with each phase, or should we target 1M+ from Phase 1?
   - How should we handle performance when particle counts exceed N-body capabilities?

3. **How should the different performance modes be selected?**
   - Is "Real-Time Mode" vs "High-Fidelity Mode" a runtime toggle or build-time configuration?
   - Can users switch between modes, or are they separate application modes?

## Impact

This ambiguity affects:
- Memory budget planning
- GPU compute shader optimization targets
- Benchmarking criteria for Phase 7
- Success metrics validation (Section 10.2 requires ≥60 FPS with 1M particles)

## Requested Response

Please clarify:
- The expected particle count target for each phase
- How performance modes should be implemented and controlled
- Which specification takes precedence if conflicts exist
