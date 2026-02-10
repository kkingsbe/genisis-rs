# Question: Inflation Epoch Time Range Typo in PRD

**Date:** 2026-02-10
**Type:** Documentation Error
**Impact:** PRD Consistency

## Issue Identified

The PRD contains an obvious documentation error in the Cosmological Epochs Reference table (Section 7, Line 284):

| Epoch | Time Range | Key Physics | Implemented In |
|-------|-----------|-------------|---------------|
| Inflation | 10⁻³²s – 10⁻³²s | Exponential metric expansion | Phase 2 |

Both the start and end times are shown as **10⁻³²s**, making the inflation epoch zero duration. This is clearly a typo.

## Why This Is a Problem

1. **Physical Inaccuracy**: Cosmic inflation is theorized to have lasted from approximately 10⁻³⁶s to 10⁻³²s (or similar range depending on the inflation model). A zero-duration inflation epoch is physically incorrect.

2. **Implementation Confusion**: Developers implementing Phase 2 may be confused about whether to use the literal zero-duration value (which would be meaningless) or infer the correct range from other sources.

3. **Timeline Consistency**: The incorrect time range could affect timeline scrubbing logic, epoch transition detection, and the time system's handling of the inflation period.

4. **Phase 4 Reference**: The Quark-Gluon Plasma epoch is shown as starting at 10⁻³²s (the same time inflation "ends"), suggesting inflation should end before 10⁻³²s.

## Context from Related PRD Sections

**Phase 2 Demo Moment (Lines 142-144):**
> "Launch the app. The singularity is visible momentarily, then SNAP — the universe inflates exponentially... As inflation ends, the expansion decelerates."

This clearly describes inflation as having a duration, not being instantaneous.

**Cosmological Epochs Reference Table (Lines 281-290):**
- Planck Boundary: t < 10⁻³²s
- Inflation: 10⁻³²s – 10⁻³²s ← **ERROR**
- Quark-Gluon Plasma: 10⁻³²s – 10⁻⁶s

The Quark-Gluon Plasma epoch starts at 10⁻³²s, which should be when inflation ends. This confirms inflation should end at or before 10⁻³²s.

## Suggested Corrections

### Option A: Standard Inflationary Model (Recommended)
Change inflation time range to: **10⁻³⁶s – 10⁻³²s**
- This is the standard cosmological range found in most textbooks and papers
- Inflation lasts approximately 10⁻³⁴ seconds of proper time
- Matches the epoch transition sequence

### Option B: Alternative Inflationary Model
Change inflation time range to: **10⁻³⁴s – 10⁻³²s**
- Shorter inflation duration (~2 × 10⁻³³ seconds)
- Some models predict shorter inflation periods
- Still provides meaningful duration

### Option C: Late Inflation Model
Change inflation time range to: **10⁻³³s – 10⁻³²s**
- Very late inflation (~9 × 10⁻³⁴ seconds)
- Matches some reheating models
- Shorter but non-zero duration

### Option D: Use the Planck Boundary as Start
Change inflation time range to: **< 10⁻³²s – 10⁻³²s**
- Inflation begins before the Planck boundary
- Ends at 10⁻³²s as currently specified
- Ambiguous about exact start time

## Additional Considerations

1. **Timeline Display**: How should the timeline UI display inflation if the duration is only 10⁻³⁴ seconds? This is far shorter than the 3-minute nucleosynthesis or 380,000-year recombination epochs.
   - Should inflation be displayed as an "instant" event visually?
   - Should the timeline use logarithmic time scales to make inflation visible?

2. **Simulation vs. Visualization**: Should the simulation actually simulate the inflation period (10⁻³⁴ seconds of simulation time), or should it be handled as an instantaneous visual effect with appropriate parameter changes?

3. **Demo Moment Timing**: Phase 2 demo moment says "the singularity is visible momentarily" before inflation. How long is "momentarily" relative to a 10⁻³⁴ second inflation duration?

## Question for Product Owner

What is the correct time range for the inflation epoch?

- **Option A**: Use 10⁻³⁶s – 10⁻³²s (standard model)
- **Option B**: Use 10⁻³⁴s – 10⁻³²s (alternative model)
- **Option C**: Use 10⁻³³s – 10⁻³²s (late inflation model)
- **Option D**: Use < 10⁻³²s – 10⁻³²s (matches Planck boundary)

Also, please clarify:
- How should the ultra-short inflation duration be handled in the timeline UI and simulation?
- Should inflation be simulated as a discrete time period or an instantaneous transition?

---

**Awaiting Decision:** This is a documentation correction, not an implementation ambiguity. Any of the above options would resolve the typo. The standard model (Option A) is recommended for scientific accuracy.
