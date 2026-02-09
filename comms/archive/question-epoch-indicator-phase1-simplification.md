# Question: Epoch Indicator UI for Phase 1 (Single Epoch)

**Date:** 2026-02-09

## Context

Phase 1 of the Genesis PRD implements only the **Singularity** epoch (t ≈ 10⁻⁴³s through initial expansion). The epoch management system (EpochManager, EpochPlugin trait) is scheduled for Phase 2+.

However:
- [`genesis.toml`](../genesis.toml) has `show_epoch_info = true` (line 32)
- [`DisplayConfig`](../genesis-core/src/config.rs:141-149) struct includes `show_epoch_info: bool` field
- [`OverlayState`](../genesis-ui/src/overlay/mod.rs:27-31) struct is **missing** the `show_epoch_info` field

## PRD Analysis

**Phase 1 Deliverables (lines 110-118):**
- Bevy application scaffold with window, input handling, and basic 3D scene ✅
- Instanced particle renderer capable of displaying 100K–1M point sprites ✅
- Free-flight camera and orbit camera with smooth interpolation ✅
- Cosmic time system with adjustable acceleration (1x to 10¹²x), pause, and reset ✅
- Logarithmic timeline scrubber UI (bevy_egui) spanning 13.8 billion years ✅
- A procedural "singularity" visualization ✅
- FPS counter and particle count overlay ✅

**Phase 1 Demo Moment (line 122):**
> "Launch the app. A dense, glowing white-hot cluster of particles sits at the center of a dark void. Press Play on the timeline. The particles explode outward in all directions, cooling from white to yellow to red as they expand. Scrub the timeline back and forth — the expansion reverses and replays. Fly the camera around the expanding cloud. This is the visual foundation for every subsequent phase."

**Phase 2 Deliverables (lines 132-140):**
- Friedmann equation integrator for scale factor a(t)
- Particle positions now scale with a(t)
- 3D Gaussian random field generator with density perturbations
- **Epoch indicator in UI showing current cosmic era and key parameters** (temperature, scale factor, time) ← FIRST MENTION
- Parameter panel (bevy_egui sidebar) to adjust n_s, inflation duration, and initial energy scale

## Key Observation

The PRD explicitly mentions "Epoch indicator in UI" as a deliverable for **Phase 2** (line 138), not Phase 1. Phase 1 only requires:
- FPS counter overlay ✅
- Particle count overlay ✅

## Current Implementation Status

**UI Overlay (genesis-ui/src/overlay/mod.rs):**
- ✅ FPS counter implemented (update_overlay_ui system)
- ✅ Particle count implemented (update_overlay_ui system)
- ❌ Epoch information display NOT implemented
- ❌ `show_epoch_info` field MISSING from OverlayState struct

## Implementation Gap

To implement epoch indicator for Phase 1, we would need to:
1. Add `show_epoch_info: bool` field to [`OverlayState`](../genesis-ui/src/overlay/mod.rs:27-31) struct
2. Create simple epoch display logic for single epoch (always show "Singularity")
3. Display static or computed epoch parameters (time, temperature if available)

## Question

**Should epoch indicator UI be implemented in Sprint 1 (Phase 1), or should it wait until Sprint 2 (Phase 2)?**

**Option A - Implement simplified epoch indicator for Sprint 1:**
- Pro: Resolves configuration field mismatch (show_epoch_info exists in TOML but not in OverlayState)
- Pro: Shows epoch name "Singularity" (single static value for Phase 1)
- Pro: Can display current cosmic time and temperature if computed
- Pro: Users can see epoch info even in Phase 1
- Con: Doesn't provide epoch transitions (only one epoch in Phase 1)
- Con: Requires temperature computation system which is also Phase 2 feature

**Option B - Defer epoch indicator to Sprint 2:**
- Pro: Aligns with PRD (epoch indicator explicitly listed as Phase 2 deliverable)
- Pro: Waits for epoch management system (EpochManager) to be implemented
- Pro: Full epoch indicator with transitions will be available immediately in Phase 2
- Con: Configuration field show_epoch_info cannot be used in Phase 1
- Con: genesis.toml show_epoch_info setting will be ignored until Phase 2
- Con: Requires removing or commenting show_epoch_info from genesis.toml for Phase 1

**Option C - Implement minimal epoch name display for Sprint 1:**
- Pro: Simple implementation (display "Epoch: Singularity" text only)
- Pro: Resolves OverlayState field mismatch
- Pro: No complex parameter display (temperature, scale factor deferred to Phase 2)
- Pro: Can be enhanced in Phase 2 with full parameters and transitions
- Con: Still requires temperature computation if we want to show temperature
- Con: May feel incomplete without transitions

## Recommendation

Given the PRD structure and alignment with phase-specific deliverables, **Option B (Defer epoch indicator to Sprint 2)** is recommended, with the following modifications:
1. Comment out or remove `show_epoch_info = true` from [`genesis.toml`](../genesis.toml:32) for Phase 1
2. Add `show_epoch_info` field to [`DisplayConfig`](../genesis-core/src/config.rs:141-149) struct (already exists)
3. Document that epoch indicator is Phase 2 feature
4. Sprint 2 will implement full epoch indicator with:
   - EpochManager integration
   - Temperature and scale factor display
   - Epoch name with transitions
   - Phase-specific parameter panels

However, if the show_epoch_info configuration field must be honored in Phase 1, Option C (minimal display) provides a reasonable compromise.

---

**Awaiting User Decision:** Please confirm which option to proceed with for Sprint 1 planning.
