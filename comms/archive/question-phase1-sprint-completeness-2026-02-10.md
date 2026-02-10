# Question: Phase 1 Sprint 2 Completeness Criteria

**Date:** 2026-02-10
**Source:** Architect Session Gap Analysis

## Context

During the architect session gap analysis, I identified several PRD Phase 1 requirements that are currently in BACKLOG.md rather than the active Sprint 2 TODO.md. This creates a question about what constitutes "Phase 1 complete."

## PRD Phase 1 Requirements Status

From PRD.md Section 5, Phase 1 Deliverables:

1. ✅ Bevy application scaffold with window, input handling, basic 3D scene - DONE
2. ✅ Instanced particle renderer capable of displaying 100K–1M point sprites - DONE
3. ✅ Free-flight camera (WASD + mouse) and orbit camera (click-drag) - DONE
4. ✅ Cosmic time system with adjustable acceleration (1x to 10¹²x) - DONE
5. ⚠️ **Logarithmic timeline scrubber UI spanning 13.8 billion years** - IN BACKLOG (not in TODO)
6. ⚠️ **Procedural singularity visualization with energy color-mapping (white-hot → red)** - IN BACKLOG (not in TODO)
7. ✅ FPS counter and particle count overlay - DONE

## Current Sprint 2 TODO.md

The active Sprint 2 tasks are:
- Timeline scrubbing to TimeAccumulator synchronization
- Timeline reverse/replay capability
- Drift Remediation (removing unrequested features)
- SPRINT QA

## The Question

For Phase 1 to be considered complete (and for Sprint 2 to create `.sprint_complete`), which requirements must be satisfied?

**Option A: Sprint 2 completes the active TODO items only**
- Items 5 and 6 above (logarithmic scrubber, energy cooling) move to Sprint 3
- Phase 1 Demo Moment would not fully match PRD specification
- Sprint 2 can create `.sprint_complete` after completing current TODO items

**Option B: Sprint 2 must complete all Phase 1 PRD requirements**
- Move items 5 and 6 from BACKLOG to TODO for Sprint 2
- Sprint 2 scope increases significantly
- Phase 1 Demo Moment fully matches PRD specification

**Option C: Clarify the definition of "Phase 1 complete"**
- Provide specific criteria for what defines Phase 1 completion
- Adjust sprint planning based on your criteria

## Additional Context

This question is separate from the existing outbox question about timeline replay scope. The replay scope question asks *how* to implement reverse/replay (snapshot system vs simplified approach), while this question asks *which* Phase 1 requirements must be in Sprint 2 to declare Phase 1 complete.

## Impact

This decision affects:
1. Sprint 2 completion criteria
2. When Sprint 2 can create `.sprint_complete` file
3. Phase 1 Demo Moment completeness
4. Sprint 3 scope (if items are deferred)

## Recommendation

Based on PRD language, **Option B** seems correct - the PRD lists specific deliverables for Phase 1, and items 5 and 6 are explicitly listed as Phase 1 deliverables. However, I recommend clarifying with you before making a decision that affects sprint scope.

Please respond with your preferred option (A, B, or C) so I can adjust TODO.md and sprint planning accordingly.
