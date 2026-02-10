---
**ARCHIVED NOTE (2026-02-10):** This RFI was written before `update_particles_for_scrubbing()` was implemented.

Since this RFI was created, the system now provides basic reverse scrubbing capability via a linear kinematic model:
- Implemented at `genesis-render/src/particle/mod.rs:429-448`
- Uses the formula: `particle.position = particle.initial_position + particle.initial_velocity * years`
- This enables particles to move backward when scrubbing the timeline
- However, this produces straight-line trajectories without physics fidelity

A full snapshot-based system (for accurate physics fidelity during reverse scrubbing) can be considered for Phase 2+.

---

# Decision Required: Timeline Reverse/Replay Implementation Scope

**Date:** 2026-02-10
**From:** Architect Mode
**Priority:** High (Blocks Sprint 2 completion)

---

## Issue Summary

Timeline scrubbing to TimeAccumulator synchronization is implemented, but **particles don't move backward when scrubbing**. The PRD Phase 1 Demo Moment explicitly requires:

> "Scrub the timeline back and forth — the expansion reverses and replays"

## Current Implementation Status

1. **Timeline UI:** Implemented - slider updates `CosmicTime.cosmic_time` and syncs to `TimeAccumulator.years`
2. **Reverse Playback:** NOT Implemented - particles continue moving forward even when timeline is scrubbed backward
3. **Snapshot System:** NOT Implemented - no particle state history buffer exists

## Implementation Options

### Option A: Implement Full Snapshot-Based Reverse/Replay (Sprint 2)
**Implementation:**
- Create `SimulationSnapshot` data structure storing particle states (position, velocity, energy)
- Create `SnapshotHistory` buffer with circular storage (e.g., 50-100 snapshots)
- Implement `capture_particle_state()` system that saves snapshots at regular intervals
- Implement `restore_particle_state()` system that restores nearest snapshot when scrubbing backward
- Implement interpolation between snapshots for smooth scrubbing

**Pros:**
- Fully satisfies PRD Phase 1 Demo Moment requirement
- Provides high-quality user experience (timeline works as expected)
- Foundation for future features (state save/load, replay analysis)

**Cons:**
- Significant development effort (estimated 1-2 weeks)
- Adds memory overhead (storing 100 snapshots of 100K particles ≈ 100MB)
- Complex edge cases (scrubbing outside snapshot history, unvisited regions)
- Delays other Sprint 2 work

### Option B: Defer Reverse/Replay to Future Sprint
**Implementation:**
- Timeline scrubbing continues to only update `CosmicTime` and `TimeAccumulator`
- Add UI indicator: "Reverse playback not yet implemented" when scrubbing backward
- Document as known limitation in Sprint 2 release
- Plan snapshot-based reverse/replay for Sprint 3 or 4

**Pros:**
- Faster Sprint 2 completion (focus on other Phase 1 deliverables)
- No memory overhead added in current sprint
- Allows more time to design optimal snapshot architecture

**Cons:**
- Does NOT satisfy PRD Phase 1 requirement (Demo Moment is explicit)
- Poor user experience (timeline doesn't work as advertised)
- Technical debt (will need to implement later anyway)
- May require regression testing when implemented in future sprint

### Option C: Minimal Implementation (Hybrid Approach)
**Implementation:**
- Implement simplified snapshot system with fewer snapshots (e.g., 10 keyframes)
- Only capture snapshots at epoch boundaries (Singularity, Inflation, QGP, etc.)
- When scrubbing backward, restore to nearest keyframe and replay forward
- Add UI showing "Replaying to [epoch name]..." during restoration

**Pros:**
- Balances effort and functionality
- Reasonable user experience (can scrub between epochs)
- Lower memory footprint (fewer snapshots)
- Foundation for future enhancement

**Cons:**
- More complex than full implementation (keyframe logic + replay logic)
- Still not true reverse playback (fast-forwards from keyframes)
- Epoch boundaries may not align with user's scrub target

## Recommended Decision

Based on PRD Phase 1 explicit requirement and the "Demo Moment" specification, **Option A (Full Snapshot-Based Reverse/Replay)** is recommended. The PRD states:

> "Launch the app... Scrub the timeline back and forth — the expansion reverses and replays."

This is not optional - it's a core feature of Phase 1. Implementing it now avoids technical debt and ensures Sprint 2 delivers a complete Phase 1 experience.

However, if the project timeline is constrained, **Option C (Minimal Hybrid Approach)** provides a middle ground that allows demonstration of the feature while controlling scope.

---

## Decision Requested

Please choose one of the following:

1. **[ ] Option A** - Implement full snapshot-based reverse/replay in Sprint 2
2. **[ ] Option B** - Defer reverse/replay to future sprint (document as limitation)
3. **[ ] Option C** - Implement minimal hybrid approach (keyframe-based restoration)

---

## Additional Context

The existing BACKLOG.md already contains detailed task breakdowns for full reverse/replay implementation (lines 203-245: Timeline Reverse/Replay Capability section). These tasks include:
- SimulationSnapshot data structure
- SnapshotHistory circular buffer
- State capture and restoration systems
- Timeline scrubbing to state restoration connection

Implementation can start immediately upon decision.

---

**Related:**
- BLOCKERS.md: "Timeline Reverse/Replay Scope" (lines 11-19)
- TODO.md: "feature: Timeline reverse/replay capability" (line 70)
- BACKLOG.md: "Timeline Reverse/Replay Capability" (lines 390-426)
- Archived: comms/archive/question-timeline-replay-sprint-scope-2026-02-10.md
