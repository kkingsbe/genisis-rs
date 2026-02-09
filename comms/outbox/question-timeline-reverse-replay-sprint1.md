# Question: Timeline Reverse/Replay Implementation Timing

**Date:** 2026-02-09

## Context

The PRD Phase 1 Demo Moment (line 122) states:

> "Scrub the timeline back and forth — the expansion reverses and replays."

This implies that timeline scrubbing should allow users to:
- Move the timeline slider backward to see particle positions at earlier cosmic times
- Move the timeline slider forward to see particle positions at later cosmic times

## Current Implementation Status

**Implemented:**
- ✅ Timeline slider UI (genesis-ui/src/timeline/mod.rs)
- ✅ Logarithmic time mapping (CosmicTime::from_slider, CosmicTime::to_slider)
- ✅ Timeline scrubbing updates CosmicTime.cosmic_time resource

**Missing:**
- ❌ Timeline scrubbing does NOT sync to TimeAccumulator.years
- ❌ No snapshot system to save/restore particle states
- ❌ No history buffer for timeline navigation

## Implementation Complexity

Full timeline reverse/replay would require:
1. Simulation snapshot system to capture particle states (position, velocity, energy) at regular intervals
2. Circular history buffer to store snapshots (e.g., last 20-100 snapshots)
3. State restoration system to restore particle positions when timeline is scrubbed
4. Integration with existing time and particle systems

This is a significant implementation effort (estimated 1-2 weeks) that involves:
- New data structures (SimulationSnapshot, SnapshotHistory)
- New systems (capture_particle_state, restore_particle_state)
- Memory management for snapshot storage
- Handling edge cases (scrubbing beyond history, unvisited time regions)

## BACKLOG Reference

The feature is already documented in BACKLOG.md (lines 337-346) under "#### Timeline Reverse/Replay Capability" as part of future work.

## Question

**Should full timeline reverse/replay capability be implemented in Sprint 1 (Phase 1), or can it be deferred to Sprint 2?**

**Option A - Implement in Sprint 1:**
- Pro: Meets PRD Phase 1 Demo Moment requirement completely
- Pro: Provides full "scrub back and forth" experience as specified
- Con: Significant implementation effort (1-2 weeks)
- Con: Delays other Sprint 1 completions

**Option B - Defer to Sprint 2:**
- Pro: Allows Sprint 1 to focus on core Phase 1 features and critical bug fixes
- Pro: Aligns with incremental delivery principle - basic scrubbing exists, enhancement can wait
- Pro: More time to refine snapshot system design
- Con: Demo Moment not fully achievable in Sprint 1
- Con: Partial timeline scrubbing (UI works but doesn't affect simulation) may be confusing

**Option C - Implement simplified version for Sprint 1:**
- Pro: Basic reverse playback without full snapshot system
- Pro: Timeline scrubbing affects TimeAccumulator.years (particles move backward/forward)
- Con: Particles reset to origin when scrubbing (no state preservation)
- Con: May feel incomplete or buggy

## Recommendation

Given the incremental delivery principle and the current Sprint 1 focus on Phase 1 completion, **Option B (Defer to Sprint 2)** is recommended, with the caveat that:
- Sprint 1 should still implement timeline scrubbing to TimeAccumulator.years synchronization (basic forward/backward movement)
- Full snapshot-based reverse/replay system should be Sprint 2 priority #1

However, if the Demo Moment experience is critical for Sprint 1 completion, Option A should be pursued.

---

**Awaiting User Decision:** Please confirm which option to proceed with for Sprint 1 planning.
