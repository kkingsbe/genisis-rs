# PRD Ambiguity Check - Summary Report

**Task:** Task 4 - Communication - PRD Ambiguity Check
**Date:** 2026-02-08
**Reviewer:** AI Assistant (Code Mode)

---

## Executive Summary

A comprehensive review of [`PRD.md`](../PRD.md) has identified **5 major categories of ambiguities, contradictions, and potential technical feasibility issues** that require clarification before implementation can proceed effectively.

**Status:** ✅ Ambiguities detected and documented
**Questions Created:** 5 question files in `comms/outbox/`
**Action Required:** User review and response to each question file

---

## Summary of Findings

### 1. Particle Count Performance Target Ambiguity
**File:** [`question-particle-count-ambiguity.md`](./question-particle-count-ambiguity.md)

**Issue:** Inconsistent particle count specifications across PRD sections:
- Phase 1: 100K–1M particles
- Performance table: 1M–10M particles (Real-Time Mode)
- Phase 5: 500K particles baseline for N-body

**Impact:** Affects memory budgeting, GPU compute optimization, and Phase 7 success metrics.

---

### 2. Time Acceleration Baseline Definition
**File:** [`question-time-acceleration-definition.md`](./question-time-acceleration-definition.md)

**Issue:** Time acceleration specified as "1x to 10^12x" without defining what "1x" means in the context of cosmic time simulation.

**Impact:** Affects Phase 1 time system architecture, UI timeline design, and Cinematic Mode timing.

---

### 3. Algorithm Implementation Specification Gaps
**File:** [`question-algorithm-specification-gaps.md`](./question-algorithm-specification-gaps.md)

**Issue:** Critical algorithms mentioned at high level without sufficient technical specification:
- Adaptive Level-of-Details (particle splitting/merging criteria)
- Sub-grid star formation (Kennicutt-Schmidt parameters)
- Halo finder (linking length, minimum particle count)
- Reionization visualization (bubble expansion rate)
- Particle coupling (dark matter to baryons)

**Impact:** Affects Phases 5 and 6 implementation scope and performance targets.

---

### 4. Performance Mode Configuration and Selection
**File:** [`question-performance-mode-configuration.md`](./question-performance-mode-configuration.md)

**Issue:** "Real-Time Mode" vs "High-Fidelity Mode" referenced without explaining:
- How modes are selected (runtime toggle, config file, auto-detect)
- Whether modes can be switched at runtime
- Hardware compatibility behavior

**Impact:** Affects Phase 1 architecture, Phase 7 deliverables, and UX design.

---

### 5. Validation Criteria and Technical Feasibility Concerns
**File:** [`question-validation-and-feasibility.md`](./question-validation-and-feasibility.md)

**Issues Identified:**
- **Ambiguous validation:** "Qualitatively matches Planck data" - no tolerance specified
- **Memory constraints:** 4 GB VRAM target may be unrealistic for 10M particles
- **GPU portability:** wgpu support varies across platforms
- **Stiff ODE solver:** Complexity may conflict with 60 FPS real-time requirement
- **SPH performance:** At 1M+ particles, may struggle to hit 60 FPS
- **Core priority:** Contradiction between Non-Goals (not research-grade) and success metrics (scientific accuracy)

**Impact:** Affects all phases, especially Phase 3 (nucleosynthesis), Phase 5 (N-body), and Phase 6 (galaxy formation).

---

## Detailed Breakdown by Issue Type

### Ambiguous Requirements (9 items)
1. Particle count target (multiple conflicting values)
2. "1x" time acceleration definition
3. Particle splitting/merging thresholds
4. Sub-grid star formation parameters
5. Halo finder linking length and minimum count
6. Ionization bubble expansion rate
7. Dark matter to baryon coupling mechanism
8. Performance mode selection mechanism
9. "Qualitative match" validation criteria

### Contradictory Requirements (3 items)
1. Particle count inconsistency across sections
2. Non-goals vs. success metrics (educational tool vs. scientific accuracy)
3. Real-time 60 FPS vs. stiff ODE solver complexity

### Potential Technical Infeasibility (5 items)
1. 4 GB VRAM target for 10M particles with spatial indexing
2. 60 FPS with stiff ODE solver for nucleosynthesis
3. 60 FPS with SPH at 1M+ particles
4. GPU compute portability across all target platforms
5. Memory budget allows no margin for rendering overhead

---

## Critical Path Analysis

### Highest Priority (Block Implementation)
1. **Time acceleration definition** - Required for Phase 1
2. **Particle count target** - Required for Phase 1 architecture
3. **Core project priority** (educational vs. research-grade) - Affects all decisions

### High Priority (Affect Phase Scope)
4. **Algorithm specifications** - Affects Phase 5/6 scope and timelines
5. **Performance mode configuration** - Affects Phase 1 and 7

### Medium Priority (Can Be Deferred with Assumptions)
6. **Validation criteria tolerances** - Can define later, but affects Phase 7
7. **Technical feasibility concerns** - May require architecture adjustments

---

## Recommendations

### Before Starting Phase 1
1. **Clarify core project priority:** Is this primarily an educational visualization or a research-grade simulation? This determines all subsequent tradeoffs.

2. **Define time acceleration baseline:** What does "1x" mean? This is required for Phase 1 time system.

3. **Resolve particle count targets:** Select definitive targets for each phase.

### Before Starting Phase 3
4. **Define stiff ODE approach:** Simplified real-time vs. accurate offline solver?

### Before Starting Phase 5
5. **Provide algorithm specifications:** Especially for particle LOD and halo finding.

### Before Starting Phase 6
6. **Provide sub-grid model details:** Star formation and reionization parameters.

### Before Starting Phase 7
7. **Define validation tolerances:** Specific numeric thresholds for success metrics.

---

## Files Created

All question files are available in [`comms/outbox/`](./):

1. [`question-particle-count-ambiguity.md`](./question-particle-count-ambiguity.md)
2. [`question-time-acceleration-definition.md`](./question-time-acceleration-definition.md)
3. [`question-algorithm-specification-gaps.md`](./question-algorithm-specification-gaps.md)
4. [`question-performance-mode-configuration.md`](./question-performance-mode-configuration.md)
5. [`question-validation-and-feasibility.md`](./question-validation-and-feasibility.md)

---

## Conclusion

The PRD provides an excellent high-level vision with clear incremental delivery principles. However, several critical ambiguities require clarification before implementation can proceed confidently.

**The most critical issue is the fundamental project priority:** determining whether Genesis is primarily an educational visualization tool or a research-grade scientific simulation. This decision will inform acceptable tradeoffs for all technical challenges.

**Next Steps:**
1. Review each question file
2. Provide responses with specific requirements where needed
3. Update PRD with clarified requirements
4. Proceed to Task 5 (Technical Specification) once ambiguities are resolved

---

**Reviewed By:** AI Assistant
**Review Date:** 2026-02-08
**Review Method:** Comprehensive line-by-line analysis of PRD.md
