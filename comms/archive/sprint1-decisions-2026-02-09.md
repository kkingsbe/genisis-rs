# Sprint 1 Decisions

**Date:** 2026-02-09
**Project:** Genesis - Phase 1 (The Singularity)
**Sprint:** Sprint 1

---

## Decision 1: Epoch Indicator UI for Phase 1

**Choice:** Option B - Defer epoch indicator to Sprint 2

### Rationale
- PRD explicitly lists epoch indicator as Phase 2 deliverable (line 138)
- Phase 1 only has one epoch (Singularity), so transitions cannot be demonstrated
- Sprint 1 should focus on core features; complex UI can wait

### Required Actions
1. Comment out `show_epoch_info = true` in genesis.toml (line 32)
2. Keep DisplayConfig.show_epoch_info field for future use (Phase 2)
3. Document that epoch indicator is deferred to Phase 2

### Notes
- This aligns with the PRD's phased approach
- The DisplayConfig field should remain in code to avoid breaking changes when Phase 2 begins

---

## Decision 2: Particle System Scaling

**Choice:** Option C - Implement moderate scaling in Sprint 1 (10K-50K)

### Rationale
- PRD requires 100K-1M capability but Sprint 1 is already behind schedule
- 10K-50K particles provides significant visual improvement and validates basic scaling
- Enables performance target validation at moderate scale
- Full 100K-1M scaling with LOD system is Sprint 2 work

### Sprint 1 Scope
- Implement per-instance attribute sync (enables individual colors/sizes)
- Scale to 10K-50K particles (configurable via particle_count field)
- Add basic performance monitoring
- Validate performance target at 50K particles (≥60 FPS)

### Notes
- This provides a realistic baseline that demonstrates the concept
- Performance validation at 50K gives confidence for scaling to 100K+ in Sprint 2
- LOD system is a Sprint 2 feature for the full 100K-1M target

---

## Decision 3: Timeline Reverse/Replay

**Choice:** Option B - Defer to Sprint 2 with caveat

### Rationale
- Full snapshot system requires 1-2 weeks of implementation
- Sprint 1 should focus on core features and critical bug fixes
- Basic timeline scrubbing can be enhanced in Sprint 2

### Sprint 1 Scope
- Implement basic TimeAccumulator.years synchronization (particles move backward/forward when scrubbing)

### Sprint 2 Scope
- Full snapshot-based reverse/replay system (priority #1)

### Notes
- Basic TimeAccumulator sync is already implemented in current code
- The snapshot system will enable true reverse/replay with state preservation
- This is a Sprint 2 priority feature, as it's critical for showcasing the project

---

## Summary

These decisions help focus Sprint 1 on delivering core functionality while deferring complex features to Sprint 2. The approach:

1. Aligns with PRD phase boundaries (epoch indicator is Phase 2)
2. Provides realistic scaling targets (10K-50K instead of 100K-1M)
3. Delivers basic timeline functionality while planning full reverse/replay for Sprint 2

All deferred features remain in the backlog with clear rationale and Sprint 2 priorities defined.
