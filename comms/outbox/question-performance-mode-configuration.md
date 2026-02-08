# Question: Performance Mode Configuration and Selection

**Context:** Task 4 - PRD Ambiguity Check

## Issue Identified

The PRD references "Real-Time Mode" and "High-Fidelity Mode" (Section 8, Performance Targets) without explaining:
1. How these modes are configured or selected
2. What technical differences exist between them
3. Whether this is a runtime toggle or build-time configuration

## Problem Statement

The performance table specifies different targets for each mode:

| Metric | Real-Time Mode | High-Fidelity Mode |
|--------|---------------|-------------------|
| Particle Count | 1M – 10M | 50M – 100M |
| Frame Rate | ≥60 FPS | ≥30 FPS (offline OK) |
| GPU Memory | <4 GB VRAM | <12 GB VRAM |
| Startup Time | <5 seconds | <15 seconds |
| Snapshot Export | <2s for 10M particles | <30s for 100M particles |
| Min GPU | GTX 1660 / RX 5600 | RTX 3080 / RX 6800 XT |

However, the PRD provides no guidance on:
- How users select or configure these modes
- Whether the application should detect hardware and auto-select
- If modes can be switched at runtime
- What happens when hardware doesn't meet minimum requirements for either mode

## Questions for Clarification

### 1. Mode Selection Mechanism
**How should users access different performance modes?**
- Runtime UI toggle (e.g., "Performance" vs "Quality" setting)?
- Command-line flag or config file setting?
- Hardware auto-detection with manual override?
- Separate application builds/executables?
- Something else?

### 2. Mode Switching Behavior
**If modes are switchable at runtime:**
- Can users switch between Real-Time and High-Fidelity while the simulation is running?
- Should the simulation pause, reload, or adapt dynamically?
- Should particle counts change when switching modes?
- How should we handle state continuity (e.g., epoch progress)?

### 3. Configuration Granularity
**Should users have fine-grained control over performance settings?**
- Individual sliders for particle count, LOD, visual effects?
- Or coarse-grained presets (e.g., "Low", "Medium", "High", "Ultra")?
- Should we expose advanced settings for power users?

### 4. Hardware Compatibility
**What should happen on hardware that doesn't meet minimum requirements?**
- Should the application refuse to launch?
- Should it automatically downgrade particle counts/visuals to match available resources?
- Should we provide a warning but allow the user to proceed anyway?

### 5. Success Metric Implications
**Section 10.2 states:** "Achieves ≥60 FPS with 1M particles on GTX 1660 class hardware"

This references a specific hardware target (GTX 1660) that aligns with Real-Time Mode. However:
- Is this the only hardware configuration we must guarantee 60 FPS for?
- Should we also guarantee 30 FPS on RTX 3080 with 100M particles for High-Fidelity Mode?
- How does this interact with the "cross-platform" goal (macOS, Windows, Linux)?

## Related Ambiguities

This question intersects with:
- **Particle Count Ambiguity:** Which particle count targets are definitive?
- **Time Acceleration:** Do different modes use different timestep sizes or acceleration?
- **Algorithm Specification:** Can we use approximations in Real-Time Mode that aren't allowed in High-Fidelity Mode?

## Impact

This ambiguity affects:
- Phase 1 architecture (how we design the initial performance system)
- Phase 7 deliverables (cinematic mode, polish)
- User experience design (UI/UX for performance settings)
- Testing and validation (what hardware configurations to test)

## Requested Response

Please clarify:
- How performance modes should be implemented (runtime vs build-time)
- How users should select/configure modes
- Expected behavior when switching modes
- How to handle hardware that doesn't meet minimum specs
- Any specific UI/UX requirements for performance configuration
