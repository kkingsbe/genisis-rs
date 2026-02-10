# Resolution: Particle Identity and State Persistence Across Phases

**Date:** 2026-02-10
**Status:** RESOLVED - Architectural Decision Made
**See:** ARCHITECTURE.md - Section "[2026-02-10] Particle Identity and Persistence Across Phases"

---

## Original Question

The PRD describes particles in multiple phases, but it is ambiguous whether these are the same particles persisting with identity across phases, or whether particles are regenerated for each phase.

## Architectural Decision

Implement **Persistent Particles with Evolving Attributes** (Option A from original analysis).

### Core Design

- Particles are created once in Phase 1 (Singularity) and persist through all phases
- Each particle has extensible attributes that evolve across epochs
- No particle regeneration at phase boundaries
- Particle count changes via splitting (high-density regions) and merging (voids)

### Particle Attribute Structure

```rust
pub struct Particle {
    // Core physics (Phase 1+)
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub energy: f32,
    
    // Composition (Phase 3+)
    pub abundances: ElementAbundances,
    
    // Ionization (Phase 4+)
    pub ionization_fraction: f32,
    
    // Structure (Phase 5+)
    pub is_dark_matter: bool,
    pub halo_id: Option<u32>,
    
    // Galaxy (Phase 6+)
    pub is_star: bool,
    pub galaxy_id: Option<u32>,
    pub stellar_age: f64,
}
```

**Phase-Specific Attributes:**
- **Core Attributes (Phase 1+)**: position, velocity, mass, energy
- **Composition Attributes (Phase 3+)**: element_abundances {H, D, ³He, ⁴He, ⁷Li, ⁷Be}
- **Ionization Attributes (Phase 4+)**: ionization_state, electron_fraction
- **Structure Attributes (Phase 5+)**: is_dark_matter, halo_id, density_neighbors
- **Galaxy Attributes (Phase 6+)**: is_star, galaxy_id, stellar_age

### Phase Transition Handling

- Smooth transitions between phases (no particle recreation)
- Attributes are added/modified as phases progress
- Visual crossfade handles appearance changes
- No data loss during transitions

### Particle Count Evolution

| Phase | Particle Count | Evolution Mechanism |
|---------|---------------|-------------------|
| Phase 1-2 | Fixed count (100K - 1M) | Initial spawning |
| Phase 3-4 | Same particles, composition changes | Attribute updates only |
| Phase 5 | 1M - 10M | Splitting (dense) / merging (voids) - adaptive LOD |
| Phase 6 | Additional star particles | New entities spawned from gas |

### Timeline Scrubbing Integration

- Particle persistence enables true timeline scrubbing
- Scrubbing works by recomputing particle states from time parameter
- No snapshot restoration needed (except for irreversible phases - see separate timeline scrubbing decision)
- Efficient memory usage (single particle pool)

### Rationale

1. **PRD Alignment**: Matches "continuous story" narrative and "physically grounded" simulation requirements
2. **Scientific Accuracy**: Real cosmological evolution tracks the same particles (same dark matter particles, same baryonic gas)
3. **Timeline Scrubbing**: Natural implementation without complex snapshot system
4. **User Experience**: Users watch "the same universe" evolve, creating emotional connection
5. **Performance**: Single particle pool avoids allocation/deallocation overhead

### Impact

- **Particle data structure design**: Extensible attribute system
- **Epoch plugin architecture**: Shared particle storage across all epoch plugins
- **Timeline scrubbing implementation**: Recompute from time parameter (efficient)
- **Phase transition handling**: Smooth attribute evolution, no recreation
- **Memory management**: Single shared particle pool with adaptive LOD for Phase 5+
- **Phase 6 star creation**: New star entities spawned from gas particles
