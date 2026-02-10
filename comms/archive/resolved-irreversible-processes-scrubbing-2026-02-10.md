# Resolution: Irreversible Physical Processes and Timeline Scrubbing

**Date:** 2026-02-10
**Status:** RESOLVED - Architectural Decision Made
**See:** ARCHITECTURE.md - Section "[2026-02-10] Timeline Scrubbing Strategy for Irreversible Processes"

---

## Original Question

The PRD requires both:
1. **Real-time physics simulation** with irreversible processes (nucleosynthesis, star formation, galaxy assembly)
2. **Timeline scrubbing** that allows users to "scrub the timeline back and forth — the expansion reverses and replays"

These requirements are fundamentally in conflict. Irreversible processes cannot be "reversed" by simply moving a timeline slider.

## Architectural Decision

A **hybrid timeline scrubbing approach** has been adopted that balances user experience, performance, and physical accuracy:

### Phase-by-Phase Scrubbing Strategy

1. **Phase 1-2 (Kinematic Expansion): Full Scrubbing Support**
   - Particles follow simple kinematic motion (position = initial_position + velocity × time)
   - Time-symmetric physics allow true reverse scrubbing
   - No state restoration needed - recompute from time parameter

2. **Phase 3 (Nucleosynthesis): Limited Scrubbing Within Window**
   - Allow scrubbing within the nucleosynthesis window (3-20 minutes)
   - Store periodic snapshots during nucleosynthesis evolution
   - Cannot scrub before nucleosynthesis starts (irreversible barrier)
   - Use snapshots to restore state within the window

3. **Phase 4 (Recombination): Forward-Only After CMB Release**
   - Allow scrubbing through recombination phase (380,000 years)
   - Forward-only playback after CMB is released (irreversible barrier)
   - User sees "Cannot scrub past recombination" message when attempting reverse

4. **Phase 5-6 (Structure Formation): Forward-Only Playback**
   - Disable reverse scrubbing for structure and galaxy formation
   - Timeline shows full range but only forward playback works
   - Clear user indication: "Forward-only playback for irreversible phases"
   - Optional: Implement limited "rewind 30 seconds" for presentation purposes

5. **Snapshot System for Partial Reversibility**
   - Store lightweight snapshots at configurable intervals (e.g., every 1 million years)
   - Snapshot includes: particle positions, velocities, composition, halo structures
   - Memory budget: Target < 500 MB VRAM for snapshot storage
   - Allow scrubbing backward to nearest snapshot, then replay forward

6. **User Experience Design**
   - Visual indicators on timeline showing scrubbing limits
   - Gradient on timeline slider: green (fully scrubbing), yellow (limited), red (forward-only)
   - Informative message when user hits scrubbing limit
   - "Demo Mode" option that hides limitations for presentations

### Rationale

1. **Memory Constraints**: Full snapshot system for all phases would require 2-5 GB VRAM, exceeding <4 GB budget
2. **Scientific Accuracy**: Irreversible processes (nuclear reactions, star formation) cannot be truly reversed
3. **User Experience**: Full scrubbing for early phases provides satisfying exploration; forward-only for later phases is scientifically honest
4. **Performance**: Lightweight snapshots (periodic) enable limited reversibility without memory overhead
5. **Demo Use Case**: "Demo Mode" can hide limitations for presentations while maintaining accuracy

### Impact

- Timeline scrubbing implementation: Hybrid approach (kinematic recompute + periodic snapshots)
- Memory budget: <500 MB VRAM for snapshot storage
- User experience: Clear limitations with visual indicators
- Physics accuracy: Irreversible processes remain irreversible
- Demo capability: "Demo Mode" hides limitations for presentations
