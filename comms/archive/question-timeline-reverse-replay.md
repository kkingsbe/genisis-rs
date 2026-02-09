# Question: Timeline Reverse/Replay Implementation Strategy

## Ambiguity Identified
**Phase 1** specifies: "Scrub the timeline back and forth — the expansion reverses and replays"

The PRD explicitly requires full timeline scrubbing with reverse capability, allowing users to "scrub the timeline back and forth" and see the expansion reverse and replay.

## Why This Is a Problem
This requirement is technically extremely challenging and potentially impossible to implement efficiently:

1. **Memory Requirements**: To support arbitrary reverse/replay, we would need to store the complete simulation state at every frame. With 100K–1M particles and later 50M–100M particles, storing full state history would require hundreds of GB to terabytes of memory.

2. **State Complexity**: Each particle has position, velocity, color, composition, temperature, and other properties. Storing all this for millions of particles across billions of simulation steps is not feasible in memory.

3. **Determinism Challenge**: Even with deterministic physics, floating-point precision differences can cause divergence when replaying, especially with GPU compute where precision varies by hardware.

4. **Real-time Constraint**: The performance targets (≥60 FPS) make it impossible to capture and store full state every frame without significant performance impact.

## Suggested Approaches

1. **Checkpoint-based Rewind (Recommended)**
   - Store simulation snapshots at intervals (e.g., every 1,000 frames or every major epoch transition)
   - When scrubbing, rewind to nearest checkpoint and fast-forward
   - Tradeoff: Not truly "instant" reverse, but memory-efficient and workable
   - Pros: Memory-efficient, technically feasible, preserves performance
   - Cons: Rewind has brief pause while fast-forwarding, not instant

2. **Limited State Recording**
   - Record only particle positions and colors (not full physics state)
   - Re-simulate physics when rewinding using recorded positions as target constraints
   - Tradeoff: May have visual discrepancies during rewind
   - Pros: Reduced memory footprint
   - Cons: Physics may not match exactly, rewind may look "off"

3. **Accept PRD Modification**
   - Modify PRD to remove full reverse capability
   - Replace with "pause, speed control, and forward-only timeline"
   - Tradeoff: Loses the described "reverse and replay" demo experience
   - Pros: Technically feasible, realistic implementation
   - Cons: Changes the intended user experience

4. **Deterministic Replay with Sparse Storage**
   - Use fixed-step deterministic integration
   - Store random seeds and initial conditions
   - Replay simulation from scratch when rewinding
   - Tradeoff: Rewind requires full re-simulation, can be slow
   - Pros: Memory-efficient, exact physics reproduction
   - Cons: Rewind latency scales with distance scrubbed

## Question for Product Owner
Which approach is acceptable for the timeline reverse/replay feature? If checkpoint-based rewind is acceptable, what checkpoint interval and rewind latency tolerance is acceptable? If the PRD modification approach is chosen, should the "Demo Moment" description in Phase 1 be updated?
