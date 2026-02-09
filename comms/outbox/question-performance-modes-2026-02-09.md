# Question: Performance Mode Switching (Real-Time vs High-Fidelity)

## Date
2026-02-09

## Context

The PRD specifies performance targets for two different modes but provides no guidance on how users interact with or select between these modes.

## Ambiguity Identified

**PRD Section 8 (Lines 296-303):** Performance Targets table with two columns: "Real-Time Mode" and "High-Fidelity Mode"

| Metric | Real-Time Mode | High-Fidelity Mode |
|--------|---------------|-------------------|
| Particle Count | 1M – 10M | 50M – 100M |
| Frame Rate | ≥60 FPS | ≥30 FPS (offline OK) |
| GPU Memory | <4 GB VRAM | <12 GB VRAM |
| Startup Time | <5 seconds | <15 seconds |
| Snapshot Export | <2s for 10M particles | <30s for 100M particles |
| Min GPU | GTX 1660 / RX 5600 | RTX 3080 / RX 6800 XT |
| Min CPU | 4-core / 8-thread @ 3 GHz | 8-core / 16-thread @ 3.5 GHz |

The PRD defines two distinct performance modes with different requirements and capabilities but does not specify:
- How users switch between Real-Time and High-Fidelity modes
- Whether mode switching is automatic, manual, or hybrid
- User interface for mode selection
- Whether switching modes requires simulation restart
- How the application detects hardware capabilities
- What happens if user's hardware doesn't meet minimum requirements for selected mode

## Why This Is a Problem

1. **No Mode Selection UX Defined:**
   - Should there be a settings menu for mode selection?
   - A simple dropdown in the UI?
   - Command-line argument or config file setting?
   - A launcher that auto-detects hardware?

2. **Mode Switching Behavior Not Specified:**
   - Can users switch modes during live simulation?
   - Does mode switching require restarting the application?
   - Does switching modes reset simulation state?
   - Are there any warnings before switching to High-Fidelity mode on lower-end hardware?

3. **Hardware Detection Ambiguity:**
   - Should the application auto-detect GPU/CPU capabilities?
   - Should it recommend the appropriate mode based on detected hardware?
   - Should it prevent selecting High-Fidelity mode on incompatible hardware?
   - How to handle systems that fall between the two categories?

4. **Performance Budget Enforcement:**
   - Does Real-Time Mode enforce particle count limits (1M-10M)?
   - Does High-Fidelity Mode enforce frame rate ceiling or allow higher FPS?
   - Are GPU memory budgets enforced or just targets?
   - How to handle snapshot export timing targets in practice?

5. **User Expectations:**
   - What's the default mode when first launching the app?
   - Does the mode persist between sessions?
   - Is mode selection prominently exposed to users, or hidden in settings?
   - Should the mode be displayed in the UI (e.g., overlay)?

## Suggested Approaches

### 1. Automatic Hardware Detection (Recommended)
- Application detects GPU/CPU at startup
- Automatically selects appropriate mode based on hardware specs
- Shows "Auto" in UI with detected mode in parentheses
- Users can manually override if desired
- Pros: Best user experience, no configuration required, automatic optimization
- Cons: Hardware detection can be tricky, may misclassify edge cases

### 2. Manual Mode Selection in Settings
- Mode selection in configuration (genesis.toml) and/or UI settings panel
- Default to Real-Time Mode (safest option)
- Users must manually switch to High-Fidelity Mode
- Pros: Simple to implement, user has full control, predictable behavior
- Cons: Users may not know which mode is appropriate, requires user education

### 3. Launcher with Mode Selection
- Separate launcher window before main application
- User selects mode from simple dialog
- Launcher displays hardware detection results and recommendations
- Pros: Clear mode selection, can educate users about differences, prevents wrong mode selection
- Cons: Extra step before application starts, more complex UX

### 4. Runtime Mode Switching with No Restart
- Toggle or slider in main UI to switch modes
- Dynamically adjusts particle count, resolution, or other settings
- May need to reload or reinitialize some systems
- Pros: Flexible, users can experiment with different quality settings
- Cons: Complex to implement, may cause jarring visual changes, could cause instability

### 5. Single Mode with Dynamic Quality Scaling
- No explicit mode selection
- Application continuously adjusts quality based on performance
- Drops particle count or resolution if FPS falls below target
- Pros: Seamless experience, adapts to varying hardware conditions
- Cons: Harder to predict behavior, may not achieve either target reliably

### 6. Hybrid: Auto-Detect with Manual Override
- Automatically select mode based on hardware detection
- Show mode selection in UI panel allowing manual override
- Provide "Recommended: High-Fidelity Mode" or similar indicators
- Pros: Best of both worlds, automatic for most users, override available
- Cons: Most complex implementation, requires both detection and UI

## Additional Questions

- **Mode Persistence:** Should the selected mode persist between application launches? (stored in genesis.toml?)
- **Performance Warnings:** Should users be warned if selecting High-Fidelity mode on hardware that doesn't meet minimum requirements?
- **Frame Rate Targeting:** How strictly should the application enforce the ≥60 FPS (Real-Time) or ≥30 FPS (High-Fidelity) targets? Should it drop particle count if not meeting target, or just display warning?
- **Particle Count Adjustment:** If switching from High-Fidelity to Real-Time mode during simulation, how should we reduce particle count? Randomly cull, smart culling (keep important regions), or require restart?
- **Hardware Classification:** How do we classify hardware between "mid-range" (meets Real-Time specs) and "high-end" (meets High-Fidelity specs)? Is there a grey area?

## Reference: Related PRD Sections

**PRD Section 8 - Performance Targets (Lines 296-304):**
The performance targets explicitly define two distinct modes with different capabilities and requirements. This suggests mode selection is a user-facing feature, not just an internal implementation detail.

**PRD Section 10 - Success Metrics (Lines 321-340):**
Line 334: "Achieves ≥60 FPS with 1M particles on GTX 1660 class hardware"
This is a final release metric specifically for Real-Time Mode, confirming that mode selection impacts release criteria.

## Question for Product Owner

How should users interact with the two performance modes (Real-Time vs High-Fidelity)?

- Should mode selection be automatic (hardware detection), manual (settings), or hybrid?
- Where should mode selection appear in the UI (launcher, settings panel, always-visible overlay)?
- Can users switch modes during live simulation, or does it require restart?
- What should happen if a user tries to select High-Fidelity mode on hardware that doesn't meet minimum requirements (RTX 3080 / RX 6800 XT class)?

Also:
- Should the selected mode persist between sessions?
- Should the application warn users about performance implications of mode selection?
- How strictly should performance targets be enforced (e.g., drop particle count to maintain 60 FPS)?
- Is there a middle-ground mode for users between GTX 1660 and RTX 3080 class hardware?
