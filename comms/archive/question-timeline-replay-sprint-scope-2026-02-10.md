# Question: Timeline Reverse/Replay Scope and Sprint Priority

**Date:** 2026-02-10
**Source:** Architect Session Gap Analysis

## Context

During the architect session gap analysis, I identified a potential discrepancy between PRD requirements and current sprint planning:

### PRD Phase 1 Demo Moment Requirement

From PRD.md Section 5, Phase 1 Deliverables:

> **▶ Demo Moment: The Primordial Spark**
>
> Launch the app. A dense, glowing white-hot cluster of particles sits at the center of a dark void. Press Play on the timeline. The particles explode outward in all directions, cooling from white to yellow to red as they expand. **Scrub the timeline back and forth — the expansion reverses and replays.** Fly the camera around the expanding cloud. This is the visual foundation for every subsequent phase.

The PRD explicitly requires timeline reverse/replay capability for Phase 1's Demo Moment.

### Current TODO.md Status

Current TODO.md (Sprint 2) includes the following timeline tasks:

```markdown
- [ ] feature: Implement basic timeline scrubbing to TimeAccumulator synchronization
  - [ ] Enable particles to move backward/forward when scrubbing the timeline
  - [ ] Basic synchronization with TimeAccumulator.years during timeline scrub
  - [ ] Note: Full snapshot-based reverse/replay system is future sprint priority
- [ ] feature: Timeline reverse/replay capability (PRD Phase 1 Demo Moment requires "Scrub the timeline back and forth")
  - Location: genesis-ui/src/timeline/mod.rs
  - Current: Timeline scrubbing updates cosmic_time but particles don't move backward
  - PRD reference: Section 5, Phase 1 Demo Moment - "Scrub the timeline back and forth — the expansion reverses and replays"
```

There's an internal contradiction: the tasks acknowledge the PRD requirement but also note that "Full snapshot-based reverse/replay system is future sprint priority."

### Implementation Status

- Timeline slider: **IMPLEMENTED** - updates CosmicTime.cosmic_time
- Timeline to TimeAccumulator sync: **IMPLEMENTED** - syncs to TimeAccumulator.years
- Particle reverse/replay based on scrub: **NOT IMPLEMENTED** - particles don't move backward when scrubbing

## Question

Should timeline reverse/replay capability be:

**Option A:** Implemented in Sprint 2 (current sprint) as required by PRD Phase 1 Demo Moment?
- This would satisfy the explicit PRD requirement: "Scrub the timeline back and forth — the expansion reverses and replays"
- Would require implementing full snapshot system (SimulationSnapshot, SnapshotHistory) in current sprint
- May increase Sprint 2 scope significantly

**Option B:** Deferred to a future sprint with a simplified Phase 1 solution?
- Could implement basic particle time scaling without full snapshot history
- Phase 1 Demo Moment would not fully match PRD specification
- Full reverse/replay moved to Sprint 3 or later

**Option C:** Other approach you specify?
- Please clarify your preference for timeline reverse/replay implementation timing and scope

## Impact

This decision affects:
1. Sprint 2 completion criteria (what defines "done" for Phase 1?)
2. Whether Sprint 2 can create `.sprint_complete` file
3. Phase 1 Demo Moment completeness - will it match PRD specification?

## Recommendation

Based on PRD language, **Option A** seems correct - Phase 1 Demo Moment explicitly requires reverse/replay capability. However, I recommend clarifying with you before making a decision that affects sprint scope.

Please respond with your preferred option (A, B, or C) so I can adjust TODO.md and sprint planning accordingly.
