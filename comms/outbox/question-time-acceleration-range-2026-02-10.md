# Question: Time Acceleration Range Definition

**Date:** 2026-02-10
**Source:** PRD Review

## Ambiguity Identified

The PRD specifies two conflicting time acceleration requirements:

1. **From Performance Targets (Section 8):** Time acceleration range of "1x to 10¹²x"
2. **From Demo Requirements:** Simulate "13.8 billion years in 8 minutes"

These are mathematically inconsistent:
- 13.8 billion years = 13.8 × 10⁹ years ≈ 4.35 × 10¹⁷ seconds
- 8 minutes = 480 seconds
- Required acceleration = 4.35 × 10¹⁷ ÷ 480 ≈ 9.06 × 10¹⁴x (~10¹⁵x)

**The demo requirement demands ~10¹⁵x acceleration, but the specified maximum is only 10¹²x (1,000x lower).**

Additional ambiguities:
- What does "1x" mean in this context (real-time 1 second = 1 simulation second? or 1 frame = 1 simulation step)?
- Is acceleration linear, logarithmic, or adaptive?
- How does the acceleration range interact with the physical accuracy requirements?
- Can users select arbitrary values within the range, or are there presets?

## PRD References

### Performance Targets (Section 8)

| Metric | Real-Time Mode | High-Fidelity Mode |
|--------|---------------|-------------------|
| Frame Rate | ≥60 FPS | ≥30 FPS (offline OK) |
| Time Acceleration | 1x – 10¹²x | 1x – 10¹²x |

### Demo Requirements

> "Demo scenario: 13.8 billion years in 8 minutes (time acceleration)"

### Timeline & Time Control (Section 4.7)

> "Time control: Pause, play, scrub, acceleration (1x – 10¹²x)"

### Success Metrics (Section 10)

> "Accurately simulates key cosmological epochs (Inflation, Nucleosynthesis, Recombination, Structure Formation, Reionization) from t = 10⁻³⁶s to 13.8 Gyr"
> "Time acceleration enables rapid exploration of cosmic history"

## Mathematical Analysis

### Demo Scenario Requirements

To simulate 13.8 billion years in 8 minutes:

**Constant acceleration:**
- Acceleration = (13.8 × 10⁹ years) / (8 minutes)
- = (4.35 × 10¹⁷ seconds) / 480 seconds
- = 9.06 × 10¹⁴
- **Required: ~10¹⁵x acceleration**

**Linear mapping (variable acceleration):**
- If acceleration increases over time (slow early epochs, fast later), average still ~10¹⁵x
- Peak acceleration could be higher (10¹⁶x - 10¹⁸x) to account for slower early phases

### Current PRD Specification

- Maximum acceleration: 10¹²x
- **Gap: 1,000x short of demo requirement**

### Frame Rate Impact

At 60 FPS with 10¹²x acceleration:
- Each frame advances simulation by 10¹² ÷ 60 ≈ 1.67 × 10¹⁰ simulation seconds
- 1 year = 3.15 × 10⁷ seconds
- Each frame advances ~528 years
- 13.8 billion years requires 13.8 × 10⁹ ÷ 528 ≈ 26.1 million frames
- At 60 FPS = 26,100,000 ÷ 60 ≈ 435,000 seconds ≈ **7.25 hours**

**Conclusion: With 10¹²x acceleration at 60 FPS, the demo takes ~7.25 hours, not 8 minutes.**

## Questions for Product Owner

1. **What is the correct maximum time acceleration value?**
   - 10¹²x (as currently specified)?
   - 10¹⁵x (to meet the 8-minute demo requirement)?
   - Something else?
   - Should this be configurable or adaptive?

2. **What does "1x" mean in this context?**
   - 1 second real-time = 1 second simulation time?
   - 1 frame = 1 simulation time step (dt)?
   - Something else?
   - This is important for UI labeling and user understanding.

3. **Is time acceleration linear, logarithmic, or adaptive?**
   - **Linear:** Slider 1x → 10¹⁵x, each value equally spaced
   - **Logarithmic:** Slider in decades (1x, 10x, 100x, 1Kx, 10Kx, 100Kx, 1Mx, 10Mx, 100Mx, 1Bx, 10Bx, 100Bx, 1Tx, 10Tx, 100Tx, 1Px)
   - **Adaptive:** Acceleration varies by epoch (e.g., slow for Inflation, fast for Structure Formation)
   - Or user-configurable presets?

4. **Should there be different acceleration ranges for different modes?**
   - Real-Time: 1x – 10¹²x (interactivity focus)
   - High-Fidelity: 1x – 10¹⁵x (completeness focus)
   - Demo mode: Fixed 10¹⁵x (8-minute constraint)

5. **How does time acceleration interact with physical accuracy?**
   - At 10¹²x+ acceleration, can physics remain "physically grounded"?
   - Or do we need larger time steps with reduced accuracy?
   - Should there be warnings or visual indicators when accuracy is compromised?

6. **What are the epoch-specific acceleration requirements?**

   | Epoch | Time Range | Recommended Acceleration |
   |-------|------------|------------------------|
   | Inflation | 10⁻³⁶s – 10⁻³²s | ? |
   | Nucleosynthesis | 1s – 20 minutes | ? |
   | Recombination | 380,000 years | ? |
   | Structure Formation | 10M – 13.8B years | ? |
   | Reionization | 150M – 1B years | ? |

   - Should these be different? If so, what are the target values?

7. **Should the demo scenario use a pre-recorded path or real-time simulation?**
   - **Pre-recorded:** Pre-computed 8-minute animation, "simulation" is playback
   - **Real-time:** Actually simulates during the demo, but may be simplified or cached
   - This affects whether the acceleration requirement is truly "real-time"

8. **What happens when users scrub the timeline?**
   - Scrubbing speed independent of time acceleration?
   - Does scrubbing use the same acceleration mechanism?
   - Should there be "fast forward" and "jump to time" controls?

9. **Is the 8-minute demo requirement hard or soft?**
   - Hard: Must be exactly 8 minutes
   - Soft: Approximately 8 minutes is acceptable (e.g., 5-10 minutes)
   - Can be changed: 8 minutes is just an example

10. **How should time acceleration be exposed in the UI?**
    - Numeric input (e.g., "1000x")?
    - Preset buttons (1x, 10x, 100x, 1000x, ... 10¹⁵x)?
    - Slider with logarithmic scale?
    - Epoch-specific presets (e.g., "Fast forward to Structure Formation")?

## Potential Options

### Option A: Fix the inconsistency
- Update maximum acceleration to 10¹⁵x (or higher) to meet demo requirement
- Update all PRD references to be consistent
- Validate that physics remains accurate at this acceleration

### Option B: Change the demo requirement
- Change "13.8 billion years in 8 minutes" to something achievable with 10¹²x
- New target: "13.8 billion years in ~7 hours" (realistic with current spec)
- Or "1.38 billion years in 8 minutes"

### Option C: Use adaptive acceleration
- Variable acceleration by epoch (slow early epochs, fast later)
- Average acceleration meets 8-minute target
- Maximum acceleration still 10¹²x (but peak could be higher for brief periods)
- More complex implementation but scientifically more appropriate

### Option D: Separate demo mode
- Real-Time/High-Fidelity modes: 1x – 10¹²x (accurate, interactive)
- Demo mode: Fixed 10¹⁵x+ acceleration (may be pre-computed or simplified)
- Clear separation of "exploration" vs "demonstration" use cases

## Impact

This decision affects:
1. Physics simulation integration (dt selection, accuracy vs speed)
2. Timeline UI design (slider vs presets vs presets + slider)
3. Demo implementation (real-time vs pre-computed)
4. Performance validation (what acceleration to test at)
5. User documentation (setting expectations)
6. Epoch transition handling (smooth acceleration changes)
