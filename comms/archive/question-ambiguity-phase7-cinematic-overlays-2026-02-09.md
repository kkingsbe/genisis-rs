# Question: Phase 7 Cinematic Mode and Data Overlay Specifications

## Date
2026-02-09

## Context

Phase 7 implements Polish, Cinematic Mode & Release with several features that lack detailed implementation specifications.

## Ambiguity Identified

**Phase 7 (Line 246):** "Cinematic mode: pre-authored camera paths with keyframes and easing curves, narrated text overlays explaining each epoch, suitable for museum installations and classroom presentations"

**Phase 7 (Line 247):** "Expanded parameter panel: full cosmological parameter set (Ωₘ, ΩΛ, H₀, n_s, σ₈) with presets for Standard Model, Einstein-de Sitter, De Sitter, and Open Universe"

**Phase 7 (Line 248):** "Data overlay suite: temperature map, density field, velocity streamlines, dark matter distribution, power spectrum P(k) with observational comparison lines"

**Phase 7 (Line 251):** "Benchmarking harness with automated performance regression tests"

**Phase 7 (Line 253):** "Preset configuration sharing via TOML files"

## Why This Is a Problem

### Cinematic Mode Specifications

1. **Camera Path Format Not Specified:**
   - What format stores pre-authored camera paths?
   - How many keyframes per epoch?
   - What easing functions (linear, ease-in, ease-out, bezier)?
   - Are camera paths saved to file (JSON/TOML) or hardcoded?

2. **Narrated Text Overlays Not Defined:**
   - What text content for each epoch?
   - Who authors the narration?
   - How long does each text overlay display?
   - How are overlays triggered (time-based, epoch transition, user action)?
   - Text styling (font, size, color, position, background)?

3. **Museum/Classroom Mode Requirements:**
   - Should cinematic mode be non-interactive (press play and watch)?
   - Or interactive (user can pause, scrub, explore)?
   - How to handle loop/replay?
   - Should there be multiple cinematic sequences (short demo, full history, educational modules)?

### Data Overlay Suite

4. **Temperature Map Rendering:**
   - How to visualize temperature field across 3D space?
   - Color mapping (heat gradient)?
   - Slice-based or volumetric rendering?
   - Integration with existing particle system?

5. **Density Field Visualization:**
   - Rendering technique (volumetric fog, isosurface, particles)?
   - Which particles' density (dark matter only, all particles)?
   - Resolution and sampling?

6. **Velocity Streamlines:**
   - How to generate streamlines from particle velocities?
   - Number of streamlines, length, seeding strategy?
   - Visual style (lines, arrows, tubes)?

7. **Dark Matter Distribution:**
   - How to visualize if both dark matter and baryonic particles are rendered?
   - Color coding, opacity, spatial distribution?
   - Is this a toggleable mode or always-on overlay?

8. **Power Spectrum P(k) with Comparison:**
   - How to render power spectrum graph (2D plot, 3D visualization)?
   - What observational data to compare (Planck, SDSS, others)?
   - Data source for comparison values (bundled or loaded externally)?
   - Real-time calculation or pre-computed?

9. **Performance Impact:**
   - With all overlays active, what's acceptable performance impact?
   - Are overlays GPU-intensive (requiring optimization)?
   - Should overlays be toggleable individually or as a suite?

### Parameter Panel

10. **Full Parameter Set:**
    - What's the "full" set beyond Ωₘ, ΩΛ, H₀, n_s, σ₈?
    - Ω_b (baryon density), Ω_ν (neutrino density)?
    - Redshift z₀?
    - Others?

11. **Preset Configuration:**
    - What parameter values for "Standard Model," "Einstein-de Sitter," "De Sitter," "Open Universe"?
    - Are these hardcoded or stored in TOML?
    - Can users create custom presets?

12. **Preset Sharing:**
    - What's in a shared TOML preset?
    - Format specification?
    - How to import/export presets?
    - Validation of user presets?

### Benchmarking Harness

13. **Performance Metrics:**
    - What metrics to track (FPS, frame time, GPU memory, CPU usage, draw calls)?
    - What's the baseline for regression detection?
    - How to handle hardware differences (different GPUs produce different FPS)?

14. **Test Scenarios:**
    - What test cases (particle counts, epochs, rendering modes)?
    - How many tests to run?
    - Duration of each test?

15. **Regression Thresholds:**
    - What constitutes a regression (e.g., FPS drop >10%)?
    - Should we track absolute performance or relative change?
    - How to handle acceptable performance changes (e.g., optimization improvements)?

16. **Test Storage and Reporting:**
    - Where to store benchmark results (database, JSON files)?
    - How to report failures?
    - Should benchmarks run automatically in CI/CD?

## Suggested Approaches

### Cinematic Mode

1. **JSON Camera Paths + Hardcoded Narration (Recommended)**
   - Camera paths stored as JSON with keyframes (position, rotation, time, easing)
   - Narration text hardcoded in Rust (array of epoch descriptions)
   - Text overlays triggered by timeline position (time-based)
   - Pros: Simple, flexible, easy to modify
   - Cons: Narration changes require code recompilation

2. **TOML Configuration File (Everything externalized)**
   - Camera paths, narration, timing in single TOML file
   - External file editing for any changes
   - Pros: Fully externalized, no code changes for modifications
   - Cons: More complex parsing, multiple files for multiple sequences

3. **Multiple Cinematic Sequences**
   - Short demo (2-3 minutes), full history (8 minutes), educational modules (each epoch detailed)
   - Each sequence has its own configuration file
   - User selects which to play
   - Pros: Flexible, suitable for different use cases (museum vs classroom)
   - Cons: More content to create and maintain

### Data Overlays

4. **Toggleable Individual Overlays (Recommended)**
   - Each overlay (temperature, density, velocity, etc.) can be toggled independently
   - Multiple overlays can be active simultaneously
   - UI panel with checkboxes for each overlay
   - Pros: Flexible, user can choose what to see, performance control
   - Cons: Complex UI, potential for cluttered view

5. **Pre-Defined Overlay Modes**
   - Pre-configured combinations (e.g., "Physics View," "Visualization View," "Educational View")
   - Each mode enables specific overlays
   - Users select mode, not individual overlays
   - Pros: Simpler UI, curated experience
   - Cons: Less flexible, may not include desired combinations

6. **Simplified Overlays (Minimum viable)**
   - Only implement critical overlays (temperature map, power spectrum)
   - Others deferred or omitted
   - Pros: Faster implementation, lower complexity
   - Cons: Doesn't meet PRD "data overlay suite" requirement

### Benchmarking

7. **JSON-Based Benchmark Results with Local Comparison (Recommended)**
   - Run test suite, save results to JSON
   - Compare current run to previous run stored in same file
   - Report regressions in console/CI
   - Pros: Simple, no external dependencies, works in CI
   - Cons: Results not stored centrally, no historical tracking

8. **Database + Automated CI Integration**
   - Store results in database (SQLite, PostgreSQL)
   - CI runs benchmarks and pushes results
   - Regression alerts via notifications
   - Pros: Full history, automated monitoring, professional setup
   - Cons: Complex, requires database, more infrastructure

9. **No Automated Benchmarks (Defer)**
   - Manual performance testing only
   - Benchmark harness implementation deferred
   - Pros: Fastest path to release
   - Cons: Doesn't meet PRD requirement, no regression protection

## Additional Questions

- **Narration Length:** How much text per epoch? 1 sentence? 1 paragraph? Detailed explanation?
- **Cinematic Interactivity:** Can users pause, scrub, or exit cinematic mode mid-playback?
- **Overlay Integration:** Do overlays integrate with epoch transitions (e.g., temperature map active during nucleosynthesis)?
- **Preset Validation:** Should imported user presets be validated against physics constraints (e.g., Ωₘ + ΩΛ ≈ 1)?
- **Benchmark Frequency:** When do benchmarks run (every commit, nightly, on-demand)?
- **Performance Budget:** What's acceptable frame rate with all overlays active (e.g., ≥30 FPS acceptable, or must be ≥60 FPS)?

## Reference: Related PRD Sections

**Phase 7 Demo Moment (Lines 255-257):**
> "Press a single button to enter Cinematic Mode. The camera pulls in to a white-hot singularity. Text fades in: '10⁻³² seconds after the beginning.' Inflation snaps outward. The quark-gluon plasma glows and cools. Elements forge. The fog lifts to reveal the CMB. Gravity sculpts the cosmic web. First stars ignite. Galaxies assemble. The camera pulls back to reveal the full observable universe, 13.8 billion years of evolution in 8 minutes. The entire history plays without a single loading screen, stutter, or manual intervention."

This demo moment specifies:
- Single button to enter Cinematic Mode
- Narrated text overlays ("10⁻³² seconds after the beginning")
- 8-minute full history playback
- No loading screens, stutters, or manual intervention (pre-recorded sequence)

## Question for Product Owner

For Phase 7:

**Cinematic Mode:**
- Should narration be hardcoded (Approach 1) or externalized in TOML (Approach 2)?
- Should we have multiple cinematic sequences (short demo, full history, educational) or just the 8-minute full history?
- Can users interrupt cinematic mode (pause, scrub, exit), or is it a non-interactive playback?

**Data Overlays:**
- Should overlays be individually toggleable (Approach 4) or organized into pre-defined modes (Approach 5)?
- Are all 6 overlays required, or can some be simplified or deferred?

**Parameter Panel:**
- What's the full parameter set beyond Ωₘ, ΩΛ, H₀, n_s, σ₈?
- What parameter values should be used for the 4 presets (Standard Model, Einstein-de Sitter, De Sitter, Open Universe)?

**Benchmarking:**
- Should benchmarks be local JSON-based (Approach 7) or database-backed with CI integration (Approach 8)?
- What metrics to track and what regression threshold is acceptable?

Given the target use cases (museum installations, classroom presentations), should we prioritize ease of content modification (externalized narration) or simplicity (hardcoded)?
