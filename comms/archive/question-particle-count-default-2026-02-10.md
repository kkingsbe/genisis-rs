# Question: genesis.toml Particle Count Default Value

**Date:** 2026-02-10
**Type:** Configuration Clarification Request
**Impact:** Sprint 2 - Configuration Alignment Task

## Issue

There is a discrepancy between the `genesis.toml` default particle count, the code default, and the PRD specification:

- **genesis.toml (current):** `initial_count = 1000`
- **Code default (ParticleConfig::default()):** `initial_count = 100_000`
- **PRD Phase 1:** "100K–1M point sprites" capability

## Context

From PRD Phase 1 Deliverables (line 113):
> "Instanced particle renderer capable of displaying 100K–1M point sprites with position, color, and size"

The PRD specifies a **capability** range of 100K–1M particles, not necessarily that the application should start with 100K particles.

From genesis-core/src/config.rs (lines 48-56):
```rust
impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            initial_count: 100_000,  // Code default: 100K particles
            max_count: 1_000_000,
            base_size: 2.0,
        }
    }
}
```

From genesis.toml (line 13):
```toml
[particle]
initial_count = 1000  # Current default: 1K particles
max_count = 1000000
base_size = 2.0
```

## Questions

1. **What should be the default particle count for Phase 1?**
   - Option A: Update `genesis.toml` to `initial_count = 100000` (100K) to match code default
   - Option B: Update code default to `initial_count = 1000` (1K) to match genesis.toml
   - Option C: Keep code default at 100K but use genesis.toml 1K for testing/development builds

2. **Should Phase 1 demo use 1K or 100K particles?**
   - The PRD specifies "capability" not "default", so either could be valid
   - However, for a "visible, demonstrable output", having more particles provides better visual impact

3. **Is the PRD requirement interpreted correctly?**
   - Should we interpret "100K–1M point sprites capability" as: "renderer must support up to 100K–1M particles" (default can be lower for performance)
   - Or as: "renderer should start with at least 100K particles" (default should be at minimum 100K)

## Recommendation

Given that:
- Phase 1 Demo Moment describes a "dense, glowing white-hot cluster of particles"
- Lower particle counts (1K) may not provide the visual impact described in the PRD
- Sprint 2 includes a "particle scaling" task that validates performance at 10K-50K particles

**Recommended Answer:** Update `genesis.toml` to `initial_count = 100000` to match code default and provide better visual demonstration of Phase 1 capabilities. This aligns with the PRD's emphasis on "visible, demonstrable output" and the "Demo Moment" specification.

## Decision Required

Please select one of the following options:

- [ ] **Option A:** Update genesis.toml to `initial_count = 100000`
- [ ] **Option B:** Update code default to `initial_count = 1000`
- [ ] **Option C:** Keep code default at 100K, use genesis.toml 1K for testing

Please also provide guidance on the interpretation of the PRD "100K–1M point sprites" capability requirement.
