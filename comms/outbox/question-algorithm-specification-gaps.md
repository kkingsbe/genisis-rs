# Question: Algorithm Implementation Specification Gaps

**Context:** Task 4 - PRD Ambiguity Check

## Issue Identified

Several critical algorithms are mentioned at a high level but lack sufficient technical specification for implementation.

## Ambiguous Requirements

### 1. Adaptive Level-of-Detail (Phase 5, Line 202)
**Statement:** "Adaptive level-of-detail: particle splitting in high-density regions, merging in voids"

**Missing Specifications:**
- At what density threshold should particles split?
- What is the target particle count for dense regions?
- What merging criteria should be used (density threshold, distance, other)?
- What is the minimum/maximum particle count per region?
- How should particle properties (mass, velocity, temperature) be conserved during split/merge operations?

### 2. Sub-grid Star Formation (Phase 6, Line 224)
**Statement:** "Sub-grid star formation: Kennicutt-Schmidt relation converts dense gas into star particles"

**Missing Specifications:**
- Which specific formulation of the Kennicutt-Schmidt relation should be used?
- What is the density threshold for star formation?
- What is the star formation efficiency parameter (ε)?
- How should star particle properties (mass, metallicity, age) be initialized?
- Should there be a minimum gas mass for star formation?
- How should we track stellar evolution (Pop III vs. Pop I/II)?

### 3. Halo Finder (Phase 5, Line 203)
**Statement:** "Halo finder (Friends-of-Friends algorithm) identifying collapsed structures in real time"

**Missing Specifications:**
- What is the linking length parameter (b)?
- What is the minimum particle count for a halo?
- Should halo properties (mass, center of mass, velocity) be computed and displayed?
- Should halos be tracked over time (mergers, acquisitions)?

### 4. Reionization Visualization (Phase 6, Line 226)
**Statement:** "Ionization fronts expand as signed-distance-field bubbles around star-forming halos"

**Missing Specifications:**
- How does ionization front expansion rate depend on star formation rate?
- What is the initial bubble size around a new star?
- How do multiple bubbles merge/overlap?
- What visual properties should represent ionized vs. neutral gas?

### 5. Particle Coupling (Phase 5, Line 201)
**Statement:** "Dark matter particles seeded from Phase 2 perturbation field; baryonic particles coupled"

**Missing Specifications:**
- What is the coupling mechanism between dark matter and baryons?
- What mass ratio should be used (e.g., 5:1 DM to baryons)?
- Do baryons follow DM particles exactly, or use separate physics?

## Questions for Clarification

1. **For each algorithm listed above:**
   - Should we implement standard approaches from research literature?
   - Are there specific references/papers we should follow?
   - What are the acceptable accuracy vs. performance tradeoffs?

2. **For sub-grid models (star formation, reionization):**
   - How much physical accuracy is required?
   - Is visual fidelity sufficient, or do we need research-grade models?
   - This affects the scope significantly.

3. **For real-time constraints:**
   - All these algorithms must run at ≥60 FPS
   - Some are computationally expensive (e.g., halo finder, ionization bubbles)
   - Should we use approximations or simplified models?

## Impact

These ambiguities affect:
- Phase 5 implementation scope (N-body + structure formation)
- Phase 6 implementation scope (galaxy formation, reionization)
- Performance targets (can we hit 60 FPS with these algorithms?)
- Integration between physics systems

## Requested Response

Please clarify:
- Which algorithms need detailed specification vs. can use standard approaches
- Preferred references/papers for each algorithm
- Acceptable simplifications for real-time performance
- Any specific parameters or constraints to follow
