# Blockers

## Blockers Removed - 2026-02-10

The following blockers were removed based on verification that Phase 1 features are implemented:
- "Phase 1 Sprint Completeness Criteria" - Removed because both PRD deliverables (logarithmic timeline scrubber, energy color-mapping) are already implemented
- See comms/archive/question-phase1-sprint-completeness-2026-02-10.md (archived) and comms/outbox/clarification-phase1-implementation-status-2026-02-10.md for details

### Timeline Reverse/Replay Implementation Status - 2026-02-10

The "Timeline Reverse/Replay Scope" blocker has been removed. The system now provides basic reverse scrubbing capability:

**Current Implementation:**
- Implemented at `genesis-render/src/particle/mod.rs:429-448` in `update_particles_for_scrubbing()`
- Uses linear kinematic model: `particle.position = particle.initial_position + particle.initial_velocity * years`
- Particles now move backward when scrubbing the timeline, satisfying the basic PRD requirement
- Produces straight-line trajectories (sufficient for Phase 1)

**Status:**
- The Phase 1 Demo Moment requirement "Scrub the timeline back and forth — the expansion reverses and replays" is now partially met
- RFI `comms/archive/question-timeline-replay-sprint2-decision-2026-02-10.md` has been archived with implementation context
- Full snapshot-based system (for high-fidelity physics during reverse scrubbing) can be considered for Phase 2+

## Blockers - 2026-02-10

**No active blockers.**

## Resolved Blockers

### [2026-02-09] - Time Acceleration Starting Value Uncertainty

**Status:** Resolved

**Resolution:** Set to midpoint value of 1000000000.0 (1.0×10⁹) per RFI response. genesis.toml updated on 2026-02-09.

### [2026-02-09] - Point Sprite Shader Path Not Found

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-09)

**Resolution:** Architectural decision made to recreate `assets/` directory and copy shader file to standard Bevy location. Implementation task added to TODO.md (Sprint 1, Sprint QA section).

### [2026-02-09] - Point Sprite Shader Compilation Error

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-09)

**Resolution:** Solution documented in ARCHITECTURE.md. Task added to TODO.md as priority fix: "fix: Resolve ViewUniform shader compilation error". The ViewUniform struct must be defined in the shader file.

### [2026-02-10] - Failing integration tests require GPU access

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-10)

**Resolution:** Architectural decision made to use dummy handles (`Handle::default()`) for tests that can be validated without GPU resources, and mark GPU-dependent tests with `#[ignore]`. Tests have been modified accordingly. When GPU access becomes available in CI, the ignored tests can be re-enabled.

### [2026-02-10] - Build Error: bind_group_layout trait method in ParticleMaterial

**Status:** Resolved

**Resolution:** Removed incorrect `Material` trait implementation from `PointSpriteMaterial` and updated dependent code to use custom rendering approach. Full workspace build now succeeds.

### [2026-02-10] - Missing Asset Resource Registration - RESOLVED

**Status:** Resolved

**Original Issue:**
The spawn_particles system cannot access ResMut<Assets<PointSpriteMaterial>> because the resource was not registered. This caused a panic during application startup.

**Root Cause:**
ParticlePlugin::build() in genesis-render/src/particle/mod.rs was missing the asset registration call for PointSpriteMaterial.

**Fix Applied:**
Added `app.init_asset::<PointSpriteMaterial>();` at line 488 in genesis-render/src/particle/mod.rs within the ParticlePlugin::build() method.

**Verification:**
cargo run compiled successfully and reached window creation stage without asset-related panics. The application now progresses past asset initialization.

**Resolved By:** Orchestrator session 2026-02-10

## Format for New Blockers

When reporting a blocker, use the following format:

```markdown
### [Date] - Blocker Title

**Severity:** High/Medium/Low

**Description:**
[Detailed description of the blocker]

**Impact:**
[What tasks/features are blocked by this issue]

**Possible Approaches:**
- [ ] Approach 1
- [ ] Approach 2
- [ ] Approach 3

**Status:** Open/In Review/Resolved
```
