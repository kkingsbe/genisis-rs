# Task Decomposition: Scale particle system from 1000 to 100K-1M particles

## Issue/Discrepancy
- TODO.md mentions current count is 1000
- genesis.toml shows initial_count = 100000
- Need to verify actual current state

## Subtasks (4 total)

### Subtask 1: Verify current particle count and system behavior
- Build and run the application
- Count actual number of particles spawned
- Verify genesis.toml values are being used
- Check if any hard-coded limits exist in code
- Output: Report showing actual particle count and configuration

### Subtask 2: Update configuration for target particle count (100K-1M)
- Update genesis.toml to use target particle count
- Ensure ParticleConfig structure supports the target count
- Verify no hard-coded limits prevent target count
- Output: Configuration updated to 100K initial, 1M max

### Subtask 3: Test performance at target particle count
- Build and run with new configuration
- Measure FPS at 100K particles
- Measure FPS at 1M particles (if feasible)
- Check for memory issues or crashes
- Output: Performance report with FPS metrics

### Subtask 4: Verify visual quality and FPS
- Confirm particles render correctly at target count
- Verify FPS ≥ 60 (PRD requirement)
- Check for visual artifacts
- If FPS < 60, identify bottlenecks
- Output: Final verification report

## Dependencies
- genesis.toml configuration
- genesis-core/src/config.rs (ParticleConfig)
- genesis-render/src/particle/mod.rs (spawn_particles, rendering)
