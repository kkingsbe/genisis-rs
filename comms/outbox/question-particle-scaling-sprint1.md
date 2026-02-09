# Question: Particle System Scaling to 100K-1M Particles

**Date:** 2026-02-09

## Context

The PRD Phase 1 Deliverable (line 113) requires:

> "Instanced particle renderer capable of displaying 100K–1M point sprites with position, color, and size"

The PRD Phase 1 Demo Moment (line 122) shows:
> "A dense, glowing white-hot cluster of particles sits at the center of a dark void."

## Current Implementation Status

**Infrastructure (Ready for scaling):**
- ✅ GPU instancing with shared mesh ([`PointMesh`](../genesis-render/src/particle/mod.rs:97-100) resource)
- ✅ Custom point sprite shader ([`PointSpriteMaterial`](../genesis-render/src/particle/mod.rs:61-95)) with WGSL
- ✅ Bevy ECS entity spawning system ([`spawn_particles`](../genesis-render/src/particle/mod.rs:209))
- ✅ Custom vertex attributes for per-instance size and color (ATTRIBUTE_INSTANCE_SIZE, ATTRIBUTE_INSTANCE_COLOR)

**Current Particle Count:**
- [`spawn_particles()`](../genesis-render/src/particle/mod.rs:209) currently spawns **1000** test particles
- [`Config::default()`](../genesis-core/src/config.rs:80) sets `particle_count = 10_000`
- [`genesis.toml`](../genesis.toml:13) sets `initial_count = 100000`

**Scaling Challenges:**
1. **Per-instance attribute synchronization:** GPU instance attributes (size, color) are not synchronized with [`Particle`](../genesis-render/src/particle/mod.rs) component data
   - Particles render with uniform size/color instead of individual values
   - This is a known issue documented in genesis-render/src/particle/mod.rs:9-29

2. **Performance monitoring:** No system to track FPS degradation as particle count increases
   - PRD requires 60 FPS minimum (Section 8, line 297)
   - Performance targets: 1M–10M particles with ≥60 FPS

3. **Adaptive spawning:** [`spawn_particles()`](../genesis-render/src/particle/mod.rs:209) currently uses fixed count
   - No dynamic adjustment based on config or performance
   - No fallback for lower-end hardware

## BACKLOG Reference

BACKLOG.md (lines 13-18) has detailed task breakdown for scaling:
- Implement adaptive particle spawning system that scales based on config.particle.initial_count
- Add performance monitoring to ensure target FPS with increasing particle counts
- Optimize spawn_particles() to handle 100K+ entities efficiently (use batch spawning)
- Implement particle LOD (Level of Detail) system
- Add GPU memory management for large particle systems (buffer reuse, streaming)

## Implementation Complexity

Scaling to 100K-1M particles involves:
1. **Performance optimization:** Batch spawning, efficient memory allocation
2. **LOD system:** Reduce rendering load for distant particles
3. **GPU memory management:** Buffer reuse, streaming for large counts
4. **Performance monitoring:** FPS tracking, automatic quality adjustment
5. **Per-instance attribute sync:** Connect Particle component to GPU attributes

Estimated effort: 1-3 weeks depending on LOD system complexity

## Question

**Should particle system scaling to 100K-1M be a Sprint 1 requirement, or can it be deferred to Sprint 2?**

**Option A - Implement full scaling in Sprint 1:**
- Pro: Meets PRD Phase 1 requirement completely (100K-1M capability)
- Pro: Enables impressive visual demo with dense particle cloud
- Pro: Validates performance target (60 FPS with 1M particles)
- Con: Significant implementation effort (1-3 weeks)
- Con: Delays other Sprint 1 completions (config loading, bug fixes)
- Con: Requires per-instance attribute sync system (large feature)

**Option B - Defer scaling to Sprint 2:**
- Pro: Sprint 1 focuses on core functionality and bug fixes
- Pro: More time to implement robust LOD and performance systems
- Pro: Per-instance attribute sync can be implemented alongside scaling
- Con: PRD requirement not fully met in Sprint 1
- Con: Demo Moment shows sparse particle cloud (10K instead of 100K-1M)
- Con: Performance targets cannot be validated in Sprint 1

**Option C - Implement moderate scaling in Sprint 1 (10K-50K):**
- Pro: Significant improvement over current 1K particles
- Pro: Validates basic scaling without full LOD system
- Pro: Allows Sprint 1 to include particle count adjustment via config
- Pro: Performance target can be partially validated
- Con: Doesn't meet PRD 100K-1M requirement
- Con: May hit performance limits on lower-end hardware
- Con: Requires basic per-instance attribute sync

**Option D - Keep 1K particles for Sprint 1:**
- Pro: Fastest path to Sprint 1 completion
- Pro: All focus on critical bug fixes and config system
- Pro: Clear separation of concerns: Sprint 1 = core features, Sprint 2 = performance optimization
- Con: PRD requirement explicitly requires 100K-1M capability
- Con: Demo Moment not representative of final visual quality
- Con: Cannot validate 60 FPS performance target

## Recommendation

Given the incremental delivery principle and Sprint 1 focus on Phase 1 core features, **Option C (Implement moderate scaling in Sprint 1)** is recommended, with the following plan:

**Sprint 1:**
- Implement per-instance attribute sync (enables individual particle colors/sizes)
- Scale to 10K-50K particles (configurable via particle_count field)
- Add basic performance monitoring (FPS counter exists, add particle count display)
- Validate performance target at 50K particles (should achieve ≥60 FPS on modern hardware)

**Sprint 2:**
- Implement full scaling to 100K-1M particles
- Add LOD system for performance optimization
- Implement GPU memory management (buffer reuse, streaming)
- Add automatic performance-based quality adjustment
- Validate 60 FPS at 1M particles target

This approach:
- Makes visible progress in Sprint 1 (10K+ particles)
- Validates architecture for larger scales
- Defers complex LOD optimization to Sprint 2
- Aligns with incremental delivery principle

However, if PRD Phase 1 requirement for 100K-1M is non-negotiable, Option A must be pursued.

---

**Awaiting User Decision:** Please confirm which option to proceed with for Sprint 1 planning.
