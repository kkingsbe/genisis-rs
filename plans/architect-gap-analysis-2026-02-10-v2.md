# Gap Analysis & Sprint Planning
**Date:** 2026-02-10
**Architect Session:** 2026-02-10
**Purpose:** Compare PRD requirements to TODO/BACKLOG and identify gaps

---

## Executive Summary

**Phase 1 Status:**
- ✅ All Phase 1 PRD deliverables are implemented
- ⏳ Sprint QA pending (test suite completion required)
- ⚠️ No `.sprint_complete` marker exists

**Sprint Alignment Issue:**
- TODO.md currently shows "Sprint 2: Singularity Refinement" (Phase 1)
- BACKLOG.md contains "Sprint 2 - Phase 2: Inflation & Quantum Seeds"
- **This is a misalignment** - current sprint should be Phase 1 completion, not Phase 2 start

**Critical Issues Identified:**
1. Timeline minimum range cannot represent sub-year epochs (Planck boundary at 10⁻³²s, inflation at 10⁻³⁶s-10⁻³²s)
2. No configuration validation at load time (invalid config values can cause runtime issues)
3. Sprint appears stuck - no `.sprint_complete` marker means next sprint cannot start

---

## Phase-by-Phase Coverage Analysis

### Phase 1: The Singularity (Current)

**PRD Requirements Status:**

| Requirement | Status | Location |
|-------------|----------|----------|
| Bevy application scaffold with window, input handling | ✅ Implemented | src/main.rs |
| Instanced particle renderer (100K-1M point sprites) | ✅ Implemented | genesis-render/src/particle/mod.rs |
| Free-flight camera (WASD + mouse) | ✅ Implemented | genesis-render/src/camera/mod.rs |
| Orbit camera (click-drag) | ✅ Implemented | genesis-render/src/camera/mod.rs |
| Cosmic time system (1x to 10¹²x, pause, reset) | ✅ Implemented | genesis-core/src/time/mod.rs |
| Logarithmic timeline scrubber UI (13.8B years) | ✅ Implemented | genesis-ui/src/timeline/mod.rs |
| Procedural singularity visualization | ✅ Implemented | genesis-render/src/particle/mod.rs |
| FPS counter and particle count overlay | ✅ Implemented | genesis-ui/src/overlay/mod.rs |
| Camera smooth interpolation | ✅ Implemented | genesis-render/src/camera/mod.rs |
| Q/E key vertical movement | ✅ Implemented | genesis-render/src/input/mod.rs |
| Scroll wheel zoom (orbit & free-flight) | ✅ Implemented | genesis-render/src/camera/mod.rs |
| Timeline reverse/replay (Demo Moment) | ✅ Implemented | genesis-render/src/particle/mod.rs |

**Coverage: 100%** - All Phase 1 requirements implemented

### Phase 2: Inflation & Quantum Seeds

**PRD Requirements → BACKLOG.md Coverage:**

| PRD Requirement | BACKLOG Coverage | Lines |
|----------------|------------------|--------|
| Friedmann equation integrator | ✅ Included | 645-652 |
| Scale factor a(t) evolution | ✅ Included | 504-557 |
| Temperature evolution T ∝ 1/a | ✅ Included | 456-487 |
| Gaussian random field generator | ✅ Included | 656-666 |
| Power spectrum P(k) ∝ k^(n_s-1) | ✅ Included | 659 |
| Zel'dovich approximation | ✅ Included | 662 |
| Density perturbation mapping | ✅ Included | 663-664 |
| QGP visualization (temperature-based) | ✅ Included | 669-676 |
| Epoch indicator UI (temperature, scale factor) | ✅ Included | 445-454 |
| Parameter panel (n_s, inflation duration) | ✅ Included | 724-734 |

**Coverage: 100%** - All Phase 2 requirements captured

### Phase 3: Nucleosynthesis & First Elements

**PRD Requirements → BACKLOG.md Coverage:**

| PRD Requirement | BACKLOG Coverage | Lines |
|----------------|------------------|--------|
| 12-species nuclear reaction network | ✅ Included | 743-771 |
| NACRE II reaction rate compilation | ✅ Included | 750 |
| Stiff ODE solver (implicit Rosenbrock) | ✅ Included | 756-759 |
| Live composition chart overlay | ✅ Included | 776-778 |
| Particle color-coding by element | ✅ Included | 778 |
| TOML configuration presets | ✅ Included | 783-784 |
| Validation overlay (observed vs simulated) | ✅ Included | 785-843 |

**Coverage: 100%** - All Phase 3 requirements captured

### Phase 4: Recombination & CMB

**PRD Requirements → BACKLOG.md Coverage:**

| PRD Requirement | BACKLOG Coverage | Lines |
|----------------|------------------|--------|
| Saha equation solver | ✅ Included | 578-581 |
| IonizationState resource | ✅ Included | 582-584 |
| Photon mean free path calculation | ✅ Included | 585-587 |
| Temperature evolution (T ∝ 1/a) | ✅ Included | 588-591 |
| Volumetric fog renderer | ✅ Included | 597-600 |
| CMB surface projection | ✅ Included | 600-610 |
| Camera pull-back transition | ✅ Included | 602-608 |
| Temperature readout (3000K → 2.725K) | ✅ Included | 613 |
| CMB angular power spectrum display | ✅ Included | 614-622 |

**Coverage: 100%** - All Phase 4 requirements captured

### Phase 5: Dark Ages & First Structures

**PRD Requirements → BACKLOG.md Coverage:**

| PRD Requirement | BACKLOG Coverage | Lines |
|----------------|------------------|--------|
| Direct-sum N-body gravity (GPU) | ✅ Included | 199 |
| Barnes-Hut octree | ✅ Included | 200 |
| Dark matter particles | ✅ Included | 201 |
| Adaptive particle LOD | ✅ Included | 202 |
| Friends-of-Friends halo finder | ✅ Included | 203 |
| Cosmic web visualization | ✅ Included | 204 |
| HDF5 snapshot export | ✅ Included | 205 |

**Coverage: 100%** - All Phase 5 requirements captured

### Phase 6: Cosmic Dawn & Galaxy Formation

**PRD Requirements → BACKLOG.md Coverage:**

| PRD Requirement | BACKLOG Coverage | Lines |
|----------------|------------------|--------|
| SPH with Wendland C4 kernel | ✅ Included | 222 |
| Radiative cooling functions | ✅ Included | 223 |
| Kennicutt-Schmidt star formation | ✅ Included | 224 |
| Pop III star formation | ✅ Included | 225 |
| Reionization visualization (SDF bubbles) | ✅ Included | 226 |
| Galaxy billboard sprites | ✅ Included | 227 |
| Procedural ambient audio | ✅ Included | 228 |
| VTK mesh export | ✅ Included | 229 |

**Coverage: 100%** - All Phase 6 requirements captured

### Phase 7: Polish, Cinematic Mode & Release

**PRD Requirements → BACKLOG.md Coverage:**

| PRD Requirement | BACKLOG Coverage | Lines |
|----------------|------------------|--------|
| Performance optimization pass | ✅ Included | 245 |
| Cinematic mode with camera paths | ✅ Included | 246, 1165-1266 |
| Expanded parameter panel (Ωₘ, ΩΛ, H₀, n_s, σ₈) | ✅ Included | 247, 1269-1349 |
| Data overlay suite | ✅ Included | 248, 1324-1348 |
| PNG/EXR frame capture | ✅ Included | 249, 1352-1389 |
| Benchmarking harness | ✅ Included | 250, 1392-1460 |
| Cross-platform release builds | ✅ Included | 251 |
| User documentation | ✅ Included | 252 |
| Preset configuration sharing | ✅ Included | 253, 1306-1323 |

**Coverage: 100%** - All Phase 7 requirements captured

---

## Gap Analysis Results

### ✅ No Missing Requirements Found

**All PRD requirements (Phase 1-7) are captured in BACKLOG.md**

The BACKLOG.md file provides comprehensive coverage of:
- All 7 phases from PRD.md
- Critical Issues section (lines 7-34) for immediate fixes
- Sprint organization (Sprint 1-7) aligned with phases
- Well-decomposed atomic tasks with sub-bullet points

### Vague Items Analysis

**Most tasks are well-decomposed**. The BACKLOG.md contains detailed sub-tasks for each major deliverable, such as:
- Sub-bullet points for implementation steps
- Specific file locations and line numbers
- Clear completion criteria

**Potential areas for refinement:**

1. **Scale particle system to 100K-1M particles** (line 49-55 in BACKLOG.md)
   - Currently has milestones but no specific performance targets per milestone
   - Could benefit from: explicit FPS targets (60 FPS at 10K, 100K, 500K, 1M)

2. **Epoch indicator UI tasks** (lines 167-207)
   - These are Phase 2 features but currently in Sprint 1 section
   - Should be moved to Sprint 2 for better organization

3. **Temperature & Scale Factor Tracking** (lines 456-558)
   - Large task block could be split into separate Temperature and ScaleFactor tasks
   - Currently mixed together in one section

---

## Sprint State Analysis

### Current Sprint Status

**File:** `.sprint_complete`
- **Status:** NOT FOUND
- **Implication:** Current sprint (Phase 1) is not marked complete
- **Effect:** Next sprint (Phase 2) cannot start per gatekeeper rules

**File:** `TODO.md`
- **Current header:** "Sprint 2: Singularity Refinement"
- **Actual content:** Timeline reverse/replay task (already implemented) + SPRINT QA
- **Misalignment:** This appears to be Phase 1 completion work, not Phase 2

**Recommendation:**
- Complete SPRINT QA to verify all Phase 1 deliverables
- Create `.sprint_complete` marker to signal Phase 1 completion
- Update TODO.md to reflect Phase 2 tasks from BACKLOG.md Sprint 2 section

---

## Critical Issues Requiring Attention

### 1. Timeline Minimum Range Enhancement (BACKLOG.md line 15-21)

**Issue:** Timeline cannot represent pre-1-year timescales (Planck boundary at 10⁻³²s)

**Impact:** Early universe epochs (Planck boundary, inflation at 10⁻³⁶s-10⁻³²s) cannot be accessed via timeline UI

**Recommended Action:** Move to TODO.md as immediate fix before Phase 1 completion

### 2. Configuration Validation at Load Time (BACKLOG.md line 22-33)

**Issue:** No validation of genesis.toml values when loaded

**Impact:** Invalid config values can cause runtime issues or undefined behavior

**Recommended Action:** Move to TODO.md as immediate fix before Phase 1 completion

---

## Backlog Organization Recommendations

### Task Distribution Improvements

**Issue:** Some Phase 2-4 tasks are scattered in Sprint 1 section

**Specific Tasks to Relocate:**
1. Lines 167-207: Epoch indicator UI (Phase 2 feature) → Move to Sprint 2
2. Lines 213-256: Timeline snapshot/reverse capability → Phase 1 completion (already implemented)
3. Lines 456-558: Temperature & Scale Factor tracking → Move to Sprint 2 (Phase 2)

---

## Sprint Transition Plan

### Phase 1 Completion Path

1. **Immediate Actions (TODO.md):**
   - [ ] Complete SPRINT QA: Run full build and test suite
   - [ ] Fix timeline minimum range issue (BACKLOG.md line 15)
   - [ ] Add configuration validation (BACKLOG.md line 22)

2. **Completion Signal:**
   - [ ] Create `.sprint_complete` file with current date
   - [ ] Archive completed tasks to COMPLETED.md

3. **Sprint 2 Start (Phase 2):**
   - Populate TODO.md with BACKLOG.md Sprint 2 tasks (Phase 2: Inflation)
   - Ensure final TODO.md item is SPRINT QA task

---

## Conclusions

1. **Coverage:** ✅ 100% - All PRD requirements are captured in BACKLOG.md
2. **Decomposition:** ✅ Good - Tasks are well-decomposed with specific sub-steps
3. **Sprint State:** ⚠️ Stuck - No `.sprint_complete` marker, Phase 1 not formally complete
4. **Action Required:** Complete Phase 1 SPRINT QA to enable sprint transition
5. **Gaps:** None - No PRD requirements missing from TODO/BACKLOG

---

**Next Steps:**
1. Proceed to Task 2: Sprint Management (The Gatekeeper)
2. Address sprint completion status
3. Move appropriate tasks from BACKLOG to TODO for next sprint
