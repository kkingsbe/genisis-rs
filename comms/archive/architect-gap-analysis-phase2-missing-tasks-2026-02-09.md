# Architect Communication: Phase 2 Missing Implementation Tasks

## Date
2026-02-09

## Context

During gap analysis, significant gaps were identified in Phase 2 (Inflation & Quantum Seeds) requirements. While BACKLOG.md has extensive documentation for phases 3-7, Phase 2 is missing several critical implementation tasks.

## Gap Identified: Phase 2 Core Physics Infrastructure Missing

### Phase 2 PRD Deliverables (PRD.md Lines 126-145):

| # | Requirement | Status in BACKLOG.md |
|---|-------------|---------------------|
| 1 | Friedmann equation integrator for scale factor a(t) with slow-roll inflaton potential V(φ) | ❌ NOT documented |
| 2 | Particle positions scale with a(t) — exponential expansion during inflation, decelerating after | ❌ NOT documented |
| 3 | 3D Gaussian random field generator with nearly scale-invariant power spectrum P(k) ∝ k^(n_s – 1) | ❌ NOT documented |
| 4 | Density perturbations mapped to particle displacement (Zel'dovich approximation) and color intensity | ❌ NOT documented |
| 5 | Epoch indicator in UI showing current cosmic era and key parameters (temperature, scale factor, time) | ⚠️ Partially documented |
| 6 | Parameter panel (bevy_egui sidebar): adjust n_s, inflation duration, initial energy scale | ❌ NOT documented |
| 7 | Procedural QGP visualization: particles rendered as glowing plasma blobs with temperature-mapped color ramp | ⚠️ Partially documented |

### What IS in BACKLOG.md for Phase 2:

Lines 220-269 contain epoch plugin creation tasks, but these are structural/scaffold tasks, not the core physics implementation:

- Epoch plugin structure definitions (InflationEpoch, QGPEpoch)
- Plugin registration in main.rs
- Camera configuration for each epoch

**Missing: The actual physics implementation tasks.**

## Questions for Resolution

### Question 1: Add Phase 2 Core Physics Tasks to BACKLOG?

**Issue**: Phase 2 core physics infrastructure tasks are completely missing from BACKLOG.md.

**Missing Tasks**:
1. Friedmann equation implementation (solver, density evolution functions, RK4 integration)
2. Inflaton field and slow-roll potential implementation
3. Gaussian random field generation (Box-Muller, FFT, power spectrum)
4. Zel'dovich approximation implementation (Poisson solver, displacement mapping)
5. Parameter panel UI implementation (n_s, inflation duration, energy scale controls)

**Analysis**:
- These are foundational physics for all subsequent phases
- Without these, Phase 2 demo moment (Inflation in Action) is impossible
- BACKLOG.md has detailed subtasks for phases 3-7 but not Phase 2

**Selected Option**:
1. **Add all Phase 2 core physics tasks to BACKLOG.md** - Recommended approach

---

### Question 2: BACKLOG.md Sprint 4 Duplication

**Issue**: BACKLOG.md has Sprint 4 (Phase 4) duplicated with overlapping content.

**Evidence**:
- Sprint 4 appears first at lines 533-838
- Sprint 4 appears again at lines 790-838 with nearly identical content
- This creates confusion during sprint planning

**Analysis**:
- Duplicate content makes it unclear which version is authoritative
- May lead to duplicate task work
- Violates single-source-of-truth principle

**Selected Option**:
**Consolidate into single Sprint 4 section** - Merge any unique content from both versions
