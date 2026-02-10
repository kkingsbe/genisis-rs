# Question: Sprint Scope and Drift Prioritization

**Date:** 2026-02-10
**Source:** Architect Session - Sprint Management Review

## Context and Ambiguity Identified

The current TODO.md has conflicting signals about sprint scope:

1. **TODO.md Title (line 1):** "Sprint 2: Phase 2 - Inflation & Quantum Seeds"
   - Suggests we are currently working on Phase 2 features

2. **TODO.md "Drift Remediation" Section (lines 104-152):** Contains 48 tasks to remove/refactor Phase 2+ features that were implemented during Phase 1
   - Suggests we need to clean up drift before proceeding

3. **TODO.md "Drift Analysis" Section (lines 155-255):** Contains 43 additional refactor tasks
   - Further suggests cleanup is needed before Phase 2

4. **Critical Issues Section (lines 394-421):** Contains 3 blocking issues that must be resolved
   - Failing compilation in genesis-render tests
   - Timeline minimum range enhancement
   - Configuration validation

5. **SPRINT QA Task (line 258):** "[ ] SPRINT QA: Run full build and test suite. Fix ALL errors. If green, create/update '.sprint_complete' with the current date."

6. **`.sprint_complete` file:** Does NOT exist, indicating Sprint 1 (Phase 1) is not officially complete

## The Core Question

**What is the current sprint scope and priority ordering?**

### Current State Confusion

The TODO.md has THREE potentially conflicting priority groups:

| Group | Task Count | Purpose | Location |
|-------|-----------|---------|----------|
| **Phase 2 Features** | ~50+ | Implement Phase 2: Inflation & Quantum Seeds | Lines 28-101 |
| **Drift Remediation** | 48 | Remove/refactor unrequested Phase 2+ features | Lines 104-152 |
| **Drift Analysis** | 43 | Additional refactor tasks for Phase 2+ features | Lines 155-255 |
| **Critical Issues** | 3 | Blocking bugs that prevent SPRINT QA | Lines 394-421 |

### Understanding the Drift Issue

The genesis-physics crate contains Phase 2+ features that were implemented during Phase 1:

- `genesis-physics/src/cosmology/mod.rs` - Friedmann equations, scale factor, Hubble parameter (Phase 2)
- `genesis-physics/src/inflaton/mod.rs` - Inflaton field, slow-roll parameters (Phase 2)
- `genesis-physics/src/integrator/mod.rs` - Generic RK4 integrator (Phase 2/3)
- `genesis-physics/src/perturbations/fft/mod.rs` - FFT for density fields (Phase 5)

These are registered in `src/main.rs` via `CosmologyPlugin` (line 82) and `InflatonPlugin` (line 49 in genesis-physics/src/lib.rs).

## PRD Alignment Concerns

### PRD Phase 1 Requirements (from PRD.md lines 104-123)

Phase 1: The Singularity - Window, Particle Engine & Time

Deliverables:
- Bevy application scaffold with window, input handling, and basic 3D scene ✓
- Instanced particle renderer capable of displaying 100K–1M point sprites ✓
- Free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation ✓
- Cosmic time system: f64 time accumulator with adjustable acceleration (1x to 10¹²x) ✓
- Logarithmic timeline scrubber UI spanning 13.8 billion years ✓
- Procedural "singularity" visualization ✓
- FPS counter and particle count overlay ✓

### PRD Phase 2 Requirements (from PRD.md lines 126-145)

Phase 2: Inflation & Quantum Seeds - Metric Expansion & Density Perturbations

Deliverables:
- Friedmann equation integrator for scale factor a(t) with slow-roll inflaton potential V(φ) - **Already implemented**
- Particle positions scaling with a(t) - **Already implemented**
- 3D Gaussian random field generator - **Placeholder exists**
- Density perturbations mapped to particle displacement (Zel'dovich approximation) - **Placeholder exists**
- Epoch indicator in UI - **Placeholder exists**
- Parameter panel (bevy_egui sidebar) - **Placeholder exists**
- Procedural QGP visualization - **Not implemented**

### The Dilemma

**Phase 2+ features are ALREADY partially implemented:**
- Friedmann equations: Complete
- Scale factor coupling: Complete
- Inflaton field: Complete
- Temperature evolution: Complete

**Phase 1 is not marked complete:**
- `.sprint_complete` file does NOT exist
- 3 critical issues remain
- Test compilation errors exist

## Questions for Product Owner

### 1. Sprint Scope Definition

**What is the current sprint scope?**

- **Option A: Continue with Sprint 2 (Phase 2)**
  - Complete Phase 2 implementation (Gaussian random field, Zel'dovich, epoch UI, parameter panel)
  - Accept that Phase 2+ features were implemented early
  - Defer drift remediation to a future cleanup sprint
  - Pros: Continue forward momentum, leverage existing physics code
  - Cons: Violates incremental delivery principle, drift accumulation

- **Option B: Sprint 1.5 - Drift Remediation Sprint**
  - Focus exclusively on drift remediation tasks (91 tasks in Drift Remediation + Drift Analysis)
  - Remove/refactor Phase 2+ features back to placeholder state
  - Fix critical issues (3 tasks)
  - Run SPRINT QA and create `.sprint_complete` for Phase 1
  - Pros: Clean slate, restores PRD alignment, proper sprint boundaries
  - Cons: Discards work, loses momentum, may take significant time

- **Option C: Sprint 1.5 - Critical Issues Only**
  - Fix only the 3 critical issues blocking SPRINT QA
  - Accept Phase 2+ features as "early implementation"
  - Run SPRINT QA and create `.sprint_complete` for Phase 1
  - Move to Sprint 2 (Phase 2) with existing Phase 2+ physics code as baseline
  - Pros: Fastest path forward, minimal work discarded
  - Cons: Drift remains, violates strict phase boundaries

- **Option D: Sprint 2 - Hybrid Approach**
  - Implement Phase 2 features (leverage existing physics code)
  - Defer drift remediation to Phase 7 (Polish sprint)
  - Fix critical issues in parallel with Phase 2 implementation
  - Pros: Maximizes progress, cleanup happens during polish phase
  - Cons: Accumulates technical debt, may complicate later phases

### 2. Drift Strategy

**How should Phase 2+ features implemented during Phase 1 be handled?**

- **Accept as "Early Implementation":** The features are correct, just implemented early. Keep them and continue.
- **Remove and Reimplement:** Remove Phase 2+ code, restore placeholders, reimplement in proper sprint.
- **Document and Deprecate:** Mark early-implemented features as "premature" but keep them for now.
- **Create Retroactive Sprint 1.5:** Treat the Phase 2+ work as its own completed sprint, document retrospectively.

### 3. Sprint Completion Criteria

**When is Sprint 1 considered complete?**

- Is it complete when `.sprint_complete` is created, regardless of drift?
- Should drift remediation be a required criterion for Sprint 1 completion?
- Or is drift remediation a separate concern?

### 4. Critical Issues Priority

**How should critical issues be prioritized relative to Phase 2 features?**

- Fix critical issues BEFORE starting any Phase 2 features?
- Fix critical issues IN PARALLEL with Phase 2 features?
- Defer critical issues and address them during SPRINT QA?

### 5. Architecture Decision Authority

**Who has authority to decide whether to keep or remove early-implemented features?**

- Should I (Architect) make the decision based on PRD alignment?
- Or should this be escalated to the Product Owner (user)?
- Is there a formal decision-making process for architectural tradeoffs?

## Impact

This decision affects:

1. **Sprint Planning:** What tasks go into the current sprint vs future sprints
2. **Technical Debt:** Accumulation vs cleanup strategy
3. **Development Velocity:** Forward momentum vs corrective work
4. **PRD Alignment:** Strict adherence vs pragmatic adaptation
5. **Team Morale:** Discarding work vs leveraging existing code
6. **Release Timeline:** Delays from cleanup vs risks from accumulated drift

## Recommendation (for Architect Decision)

Given the complexity and volume of drift remediation tasks (91 tasks), I recommend:

**Option C: Sprint 1.5 - Critical Issues Only**

1. Fix the 3 critical issues (test compilation, timeline min range, config validation)
2. Run SPRINT QA
3. Create `.sprint_complete` file for Phase 1
4. Document the "early implementation" decision in ARCHITECTURE.md
5. Move to Sprint 2 (Phase 2) with existing Phase 2+ physics code as baseline
6. Defer drift remediation to Phase 7 (Polish sprint) when codebase is more mature

**Rationale:**
- Minimal work discarded
- Fastest path to sprint completion
- Leverages existing correct implementation
- Cleanup happens naturally during polish phase
- Reduces decision paralysis and keeps momentum

**Alternative:** If Product Owner prefers strict PRD alignment, use **Option B: Sprint 1.5 - Drift Remediation Sprint**, but this will take significant time and may delay Phase 2 by 2-3 weeks.
