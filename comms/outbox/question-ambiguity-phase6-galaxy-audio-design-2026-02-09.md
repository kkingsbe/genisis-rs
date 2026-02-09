# Question: Phase 6 Galaxy and Audio Visual Design Specifications

## Date
2026-02-09

## Context

Phase 6 implements Cosmic Dawn and Galaxy Formation with several visual and audio features that lack detailed specifications.

## Ambiguity Identified

**Phase 6 (Line 227):** "Galaxy billboard sprites: halos above mass threshold render as composite galaxy sprites with morphology based on merger history"

**Phase 6 (Line 228):** "Procedural ambient audio: deep bass drones during dark ages, rising harmonic tones as first stars ignite, full cosmic soundscape by galaxy formation era"

## Why This Is a Problem

### Galaxy Sprite Rendering

1. **Sprite Asset Requirements Not Specified:**
   - How many galaxy sprite assets are needed?
   - What visual styles (spiral, elliptical, irregular, merger)?
   - What resolution and format (PNG with alpha, EXR with HDR)?
   - How are sprites generated or sourced?

2. **Morphology from Merger History Not Defined:**
   - What constitutes a "merger history"? How many past mergers to track?
   - How does merger history map to specific sprite assets?
   - How to visually represent intermediate morphologies (e.g., post-merger galaxy)?
   - What's the mass threshold for rendering galaxy sprites?

3. **Rendering Integration Unclear:**
   - Are galaxy sprites rendered as billboards (always facing camera)?
   - Do they rotate with the halo?
   - How do they interact with particle system and cosmic web visualization?
   - Should sprites have lighting/shading or be pre-lit?

### Procedural Audio Design

1. **Audio Generation Not Specified:**
   - What generates the "deep bass drones" and "rising harmonic tones"?
   - Are these procedural synthesis (oscillators, noise generators) or pre-recorded samples?
   - What audio library/API (kira, bevy_kira_audio) supports the required features?

2. **Audio State Machine Not Defined:**
   - How does the system transition between audio states?
   - What triggers the "first stars ignite" audio transition?
   - How many audio states exist (dark ages, cosmic dawn, galaxy formation)?
   - What's the duration and fade between states?

3. **Audio Parameters Not Specified:**
   - Frequency range for "deep bass drones" (20-60 Hz? Lower?)
   - Harmonic structure for "rising harmonic tones" (chords? overtone series?)
   - Volume levels and dynamics for each audio state
   - Should audio be spatialized (3D positional audio) or stereo ambient?

4. **Audio-Visual Synchronization:**
   - Should audio track visual state (e.g., first star appearance triggers tone)?
   - How to handle timeline scrubbing (audio rewinding, fast-forwarding)?
   - Should audio pause/stop when simulation is paused?
   - Audio latency requirements?

## Suggested Approaches for Galaxy Sprites

1. **Procedural Texture Generation (Recommended)**
   - Generate galaxy sprites procedurally using noise algorithms
   - Create morphological variants (spiral, elliptical, irregular) algorithmically
   - Apply color gradients and glow effects dynamically
   - Pros: No external assets, infinite variation, can generate any morphology
   - Cons: Procedural generation is complex, may not look as polished as hand-crafted art

2. **Pre-Rendered Sprite Assets**
   - Create 10-20 high-quality galaxy sprite images
   - Cover morphological spectrum (spiral, elliptical, irregular, merger)
   - Load from assets directory
   - Pros: High visual quality, artist-controlled, simpler implementation
   - Cons: Limited to pre-defined set, requires asset creation pipeline

3. **Hybrid: Base Assets + Procedural Variation**
   - Use 5-10 base galaxy sprite textures
   - Apply procedural modifications (color, rotation, scale, blend) based on merger history
   - Pros: Best of both worlds, manageable asset count, good variety
   - Cons: More complex than pure assets, more complex than pure procedural

4. **Particle-Based Galaxy Rendering (Alternative to sprites)**
   - Render galaxies as dense particle clusters instead of sprites
   - Use different particle colors/densities for morphology
   - Pros: Consistent with existing particle system, no sprite assets
   - Cons: Higher computational cost, more complex rendering pipeline

## Suggested Approaches for Procedural Audio

1. **Kira Procedural Synthesis (Recommended)**
   - Use kira (bevy_kira_audio) with oscillators and noise generators
   - Implement audio state machine with crossfade between states
   - Procedural generation: bass drones = low-frequency oscillators, harmonic tones = additive synthesis
   - Pros: No external audio files, dynamic audio, configurable
   - Cons: Audio programming is complex, may require trial-and-error for good sound

2. **Pre-Recorded Audio Layers**
   - Record or obtain 3-5 audio layers (dark ages drones, star ignition tones, galaxy soundscape)
   - Crossfade layers based on simulation state
   - Pros: Audio quality is controlled by recording, simpler programming
   - Cons: Limited to pre-recorded sounds, requires external assets, less dynamic

3. **Hybrid: Procedural Base + Recorded Effects**
   - Use procedural drones and harmonics for ambient layers
   - Add recorded sound effects for specific events (first star, galaxy formation)
   - Pros: Dynamic base audio with polished effects
   - Cons: More complex audio system, requires both procedural and recorded assets

4. **Minimal Audio (Alternative)**
   - Simple sine wave drones or no audio for initial implementation
   - Phase 7 adds full audio system as polish
   - Pros: Fastest implementation, defers audio complexity
   - Cons: PRD specifies Phase 6 audio, demo moment won't have audio

## Reference: Related PRD Sections

**Phase 6 Demo Moment (Lines 231-233):**
> "After the dark ages, tiny points of light flicker on inside the densest halos. These are the first stars. Ionization bubbles expand around them — translucent spheres eating into the dark neutral medium. The bubbles grow, overlap, and merge until the entire universe is reionized. Zoom into a massive halo and see a galaxy sprite forming. Zoom out and the cosmic web now glows with thousands of galaxies strung along its filaments. **Audio swells from a low rumble to a rich harmonic drone.**"

The demo moment explicitly references audio evolution, indicating audio is a key part of Phase 6 experience.

## Additional Questions

- **Audio Performance:** What's the acceptable CPU overhead for audio generation? Procedural synthesis may require significant CPU time.
- **Audio Quality Standards:** What level of audio fidelity is expected? Should audio be "good enough for presentation" or "professional quality"?
- **Galaxy Sprite Count:** What's the expected number of galaxy sprites visible simultaneously? 10? 100? 1000+?
- **Asset Format:** If using pre-rendered sprites, what file format and size limits?

## Question for Product Owner

For Phase 6:

**Galaxy Sprites:**
- Should we use procedural generation (Approach 1), pre-rendered assets (Approach 2), hybrid (Approach 3), or particle-based rendering (Approach 4)?
- If using assets or hybrid, how many sprite variations are needed?

**Procedural Audio:**
- Should we use full procedural synthesis (Approach 1), pre-recorded layers (Approach 2), hybrid (Approach 3), or defer to Phase 7 (Approach 4)?
- What audio quality level is acceptable? What CPU overhead is allowed?

**Timing:** Are both features required for Phase 6 demo moment, or can one be deferred to Phase 7 as polish?
