# Question: Time Acceleration Baseline Definition

**Context:** Task 4 - PRD Ambiguity Check

## Issue Identified

The PRD specifies time acceleration as "1x to 10^12x" (Line 115) but does not define what "1x" represents in the context of cosmic time simulation.

## Problem Statement

The time system needs to span from approximately 10^-32 seconds (Inflation) to 13.8 billion years (present day) — a range of over 18 orders of magnitude. Without a clear definition of the "1x" baseline:

1. **We cannot design appropriate UI controls** - What does the time slider's range represent?
2. **We cannot determine appropriate timestep sizes** for numerical integration
3. **We cannot guarantee the user experience described in Demo Moments** - Will users be able to scrub through all epochs meaningfully?

## Questions for Clarification

1. **What does "1x" time acceleration mean?**
   - 1 second of real time = 1 second of cosmic time? (Too slow — would take 13.8 billion years to simulate)
   - 1 second of real time = 1 year of cosmic time? (Still impractical)
   - 1 second of real time = 1 million years of cosmic time?
   - Some other reference point?

2. **How should the time acceleration be distributed across epochs?**
   - Should early epochs (10^-32s to 10^-6s) use different acceleration than later epochs (millions to billions of years)?
   - Should the UI show a single acceleration control, or epoch-specific controls?

3. **What is the expected user experience for timeline scrubbing?**
   - Can users scrub continuously from inflation to present day?
   - Or should the timeline have "keyframes" at epoch boundaries?
   - The Demo Moment for Phase 1 mentions "Scrub the timeline back and forth" — does this imply a specific scrubbing granularity?

4. **What is the default acceleration for "Play" mode?**
   - Section 10.2 mentions "Full cinematic playback completes in under 10 minutes at default acceleration"
   - This implies 13.8 billion years / 10 minutes ≈ 1.38 million years per second
   - Is this the default "1x" or some other setting?

## Impact

This ambiguity affects:
- Phase 1 time system architecture
- UI design for the logarithmic timeline scrubber
- Cinematic mode timing (Phase 7 success metric)
- Numerical solver timestep selection for ODE integration

## Requested Response

Please clarify:
- The exact definition of "1x" time acceleration
- How acceleration should scale across the full time range
- The expected default acceleration for playback/cinematic mode
- Any epoch-specific acceleration considerations
