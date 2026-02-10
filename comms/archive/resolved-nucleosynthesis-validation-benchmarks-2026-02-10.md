# Resolution: Nucleosynthesis Validation Benchmark Data Sources

**Date:** 2026-02-10
**Status:** RESOLVED - Architectural Decision Made
**See:** ARCHITECTURE.md - Section "[2026-02-10] Nucleosynthesis Validation Data Sources"

---

## Original Question

Phase 3 specifies validation overlay showing observed primordial abundances, but does not specify:
- Which observational datasets to use for comparison
- Which elements should have validation overlays
- What level of precision is required for the validation to be considered "successful"
- How the validation should be presented to users

## Architectural Decision

### Reference Values (Primary: Planck 2018)

| Element | Planck 2018 Value | Source |
|---------|---------------------|--------|
| Helium-4 (Y_p) | 0.2471 ± 0.0002 | CMB-derived |
| Deuterium/Hydrogen (D/H) | 2.58 × 10⁻⁵ ± 0.07 × 10⁻⁵ | CMB-derived |
| Lithium-7 (⁷Li/H) | 5.0 × 10⁻¹⁰ | Standard BBN prediction |
| Helium-3 (³He/H) | 1.0 × 10⁻⁵ | CMB-derived |

### Validation Elements

- **Primordial stable elements only**: p (hydrogen), D (deuterium), ³He, ⁴He, ⁷Li
- **Exclude unstable intermediates**: T (tritium), ⁷Be (beryllium-7)

### Tolerance Criteria

| Element | Tolerance | Rationale |
|---------|-----------|-----------|
| Helium-4 | Within 5% | Explicit PRD requirement |
| Deuterium | Within 10% | Reasonable given observational uncertainties |
| Lithium-7 | Within 20% OR reproduce ~3× discrepancy | Educational feature (lithium problem) |
| Helium-3 | Within 15% | Observational data less precise |

### Lithium Problem Handling

- Show **both predicted and observed values** in validation overlay
- Include a tooltip or info panel explaining the ~3× discrepancy
- Note that this is an **open question in cosmology**
- Educational value: demonstrates real scientific frontier

### Visual Presentation

- **Primary Display**: Percentage difference with color coding
  - Green: Within tolerance
  - Yellow: Close to tolerance (80-99%)
  - Red: Outside tolerance
- **Secondary Display**: Multiple reference dataset lines (optional, user-toggleable)
- Real-time update as simulation progresses
- Toggle button to enable/disable validation overlay

### Alternative Datasets (Optional User-Selectable)

- Planck 2018 (default, recommended)
- WMAP (legacy, for comparison)
- Direct observation values (metal-poor galaxies)
- User-provided custom values

### Rationale

1. **Scientific Authority**: Planck 2018 is the most precise and recent CMB dataset
2. **Educational Value**: Showing the lithium problem demonstrates the real scientific frontier
3. **User Flexibility**: Alternative datasets support educational comparison
4. **Clear Success Criteria**: Explicit tolerances for each element avoid ambiguity
5. **Implementation Clarity**: Specific values provide clear implementation guidance

### Impact

- Phase 3 validation overlay implementation (Planck 2018 values as default)
- Success criteria for Phase 3 completion (tolerances defined)
- Educational value (lithium problem highlighted for learning)
- User experience (clear visual indicators, optional dataset comparison)
- Configuration file format (validation parameters included for presets)
