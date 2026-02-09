# Question: Minor PRD Ambiguities Requiring Clarification

## Overview
This document consolidates several minor ambiguities in the PRD that don't warrant individual questions but should be clarified before implementation begins.

---

## 1. Audio Implementation Timing

**Ambiguity**: Phase 6 specifies "Procedural ambient audio" but Phase 7 mentions "Cinematic mode... narrated text overlays".

**Question**: Should audio implementation begin in Phase 6 as specified, or should it wait until Phase 7 when cinematic mode is implemented? The current timeline has audio in Phase 6 but no specified audio system architecture.

**Options**:
- Implement basic audio system in Phase 6, enhance in Phase 7
- Move all audio to Phase 7 with cinematic mode
- Implement audio progressively: ambient in Phase 6, narration in Phase 7

---

## 2. HDF5/VTK Export Timing

**Ambiguity**: Phase 5 specifies "Data export: HDF5 snapshot export" but Phase 7 includes "PNG/EXR high-resolution frame capture with HDR support".

**Question**: Should the export pipeline (genesis-export crate) be implemented in Phase 5 as specified, or should all export functionality be consolidated in Phase 7?

**Options**:
- Implement genesis-export in Phase 5, add PNG/EXR in Phase 7 (as PRD specifies)
- Move all export to Phase 7 for unified implementation
- Implement basic HDF5 in Phase 5 for validation, full export system in Phase 7

---

## 3. Benchmarking Harness Placement

**Ambiguity**: Phase 7 specifies "Benchmarking harness with automated performance regression tests" but performance regression testing is mentioned as a risk mitigation earlier.

**Question**: Should the benchmarking system be implemented incrementally (beginning in Phase 1 with basic FPS tracking) or all at once in Phase 7?

**Options**:
- Basic profiling in Phase 1, full harness in Phase 7
- All benchmarking in Phase 7 as PRD specifies
- Progressive: add profiling each phase, integrate into harness in Phase 7

---

## 4. Preset Configuration Format

**Ambiguity**: Multiple phases mention "TOML configuration presets" (Phase 3, Phase 7) but the specific schema and format are not defined.

**Question**: What should be in the TOML configuration presets? Should each phase add to the schema, or should we define the full schema upfront?

**Options**:
- Define full TOML schema upfront covering all phases (comprehensive upfront design)
- Define schema incrementally as each phase adds configurable parameters
- Use a flexible schema with optional parameters for future phases

---

## 5. Cross-Platform Build Priority

**Ambiguity**: Goals specify "Cross-platform: Linux, macOS, Windows" but Phase 1-6 don't mention cross-platform testing. Only Phase 7 mentions "Cross-platform release builds".

**Question**: Should cross-platform compatibility be validated at each phase (Linux, macOS, Windows) or only tested in Phase 7 before release?

**Options**:
- Test on all three platforms at each phase completion (slower but catches issues early)
- Develop on primary platform (Linux), test others only in Phase 7 (faster development)
- Test on two platforms (Linux + macOS or Linux + Windows) in early phases, all three in Phase 7

---

## 6. Validation Data Sources

**Ambiguity**: Multiple phases mention comparison to observational data:
- Phase 3: "validation overlay... showing observed primordial abundances (Y_p ≈ 0.245 for ⁴He)"
- Phase 4: "comparison to Planck data" for CMB power spectrum
- Phase 7: "observational comparison lines" for various metrics

**Question**: Should we use actual observational data files from public databases (Planck, NACRE II, etc.) or hardcoded representative values? The PRD mentions specific values but doesn't specify data sources.

**Options**:
- Include actual observational data files in the project (Planck FITS files, NACRE II reaction rates)
- Use hardcoded representative values from PRD (Y_p = 0.245, etc.)
- Make it configurable: bundle default values but allow loading external data files

---

## 7. Epoch Plugin Activation Order

**Ambiguity**: The Epoch Plugin Architecture is specified but the exact order and timing of plugin activation is not defined.

**Question**: Should all epoch plugins be registered at application startup with the Epoch Manager handling transitions, or should plugins be loaded/unloaded dynamically as the timeline progresses?

**Options**:
- Register all plugins at startup, activate/deactivate based on timeline (simpler, predictable)
- Dynamically load/unload plugins as epochs begin/end (lower memory, more complex)
- Hybrid: load all physics plugins, unload completed visualization plugins (balance)

---

## 8. Timeline Acceleration Ranges

**Ambiguity**: Phase 1 specifies "adjustable acceleration (1x to 10¹²x)" but doesn't specify:
- What "1x" means (real-time? one second = one second of cosmic time?)
- What acceleration values are recommended for each phase
- Whether acceleration range changes between phases

**Question**: How should the timeline acceleration scale work? What does "1x" mean and what are recommended acceleration values for different phases?

**Options**:
- 1x = real-time (1 second simulation = 1 second cosmic time), not useful for cosmological scales
- 1x = 1 second simulation = 1 million years cosmic time, 10¹²x = 1 second = 1 trillion years
- Use logarithmic scale with preset buttons for each phase (Inflation: 10³⁰x, Nucleosynthesis: 10⁶x, etc.)

---

## 9. Procedural Singularity vs. Explosion Visualization

**Ambiguity**: Phase 1 specifies "A procedural 'singularity' visualization: particles spawned at origin with outward velocity" but current implementation uses random spawning.

**Question**: Should Phase 1 implement the PRD-specified "singularity" visualization with outward explosion, or is a different visualization acceptable?

**Options**:
- Implement PRD specification exactly (particles at origin, outward velocity, cooling colors)
- Use simplified visualization (random field, no specific singularity point)
- Make it configurable: both options, user selectable

---

## Summary Question
These minor ambiguities represent low-level implementation details that should be resolved early to prevent development friction. Would you like to:
- Answer each individually (as separate issues)?
- Provide general guidance (e.g., "prefer pragmatic, incremental approaches")?
- Review and answer in a single consolidated response?

For context, these questions are primarily about timing (when to implement features), data management (hardcoded vs. external files), and cross-platform strategy.
