# Question: Nucleosynthesis Validation Benchmark Data Sources

**Date:** 2026-02-10
**Source:** Architect Session Communication Review

## Ambiguity Identified

**Phase 3** specifies: "Validation overlay (toggle-able): comparison lines showing observed primordial abundances (Y_p ≈ 0.245 for ⁴He)"

The PRD mentions a specific value for ⁴He (helium-4 mass fraction) but does not specify:
- Which observational datasets to use for comparison
- Which elements should have validation overlays
- What level of precision is required for the validation to be considered "successful"
- How the validation should be presented to users

## PRD References

### Phase 3 Deliverables
> "Stiff ODE solver (implicit Rosenbrock method) for 12-species nuclear reaction network (n, p, D, T, ³He, ⁴He, ⁷Li, ⁷Be, intermediates)"
> "Reaction rates from NACRE II compilation, temperature-dependent"
> "Live composition pie/bar chart overlay showing element abundances evolving in real time"
> "Validation overlay (toggle-able): comparison lines showing observed primordial abundances (Y_p ≈ 0.245 for ⁴He)"
> "TOML configuration presets: 'Standard Model' (Planck 2018 best-fit) and 'High Baryon Density' for comparison"

### Phase 3 Demo Moment
> "After 20 minutes of cosmic time, the chart stabilizes — toggle the validation overlay and see your simulated abundances line up with observed values. Switch to 'High Baryon Density' preset and watch helium overshoot."

### Success Metrics
> "Primordial helium abundance within 5% of observed value (Y_p ≈ 0.245)"

## Questions

### 1. Which Observational Datasets to Use?

The standard primordial abundance values come from multiple observational sources. Which should Genesis use for validation?

**Helium-4 (Y_p):**
- Planck 2018: Y_p = 0.2471 ± 0.0002 (CMB-derived)
- WMAP: Y_p = 0.2485 ± 0.0006 (CMB-derived)
- Direct observation (metal-poor galaxies): Y_p ≈ 0.245 ± 0.003
- Particle Data Group: Y_p = 0.2467 ± 0.0002

**Deuterium/Hydrogen (D/H):**
- Planck 2018: D/H = 2.58 × 10⁻⁵ ± 0.07 × 10⁻⁵
- Quasar absorption systems: D/H = 2.527 × 10⁻⁵ ± 0.030 × 10⁻⁵
- CMB-derived: D/H = 2.57 × 10⁻⁵ ± 0.14 × 10⁻⁵

**Lithium-7 (⁷Li/H):**
- Standard BBN prediction: ⁷Li/H ≈ 5.0 × 10⁻¹⁰
- Observational (metal-poor stars): ⁷Li/H ≈ 1.6 × 10⁻¹⁰ (the "lithium problem")
- **Note:** There is a well-known ~3× discrepancy between predicted and observed lithium abundances. Genesis should clarify how to handle this.

**Helium-3 (³He/⁴He):**
- Galactic HII regions: ³He/⁴He ≈ 1.1 × 10⁻⁴
- Planetary nebulae: varies significantly

### 2. Which Elements Should Have Validation Overlays?

The 12-species network includes: n, p, D, T, ³He, ⁴He, ⁷Li, ⁷Be, plus intermediates.

Should validation overlays be shown for:
- **Primordial elements only** (p, D, ³He, ⁴He, ⁷Li)?
- **All stable isotopes** (exclude unstable T, ⁷Be)?
- **All species** including intermediates and unstable nuclei?
- **User-selectable** via configuration?

### 3. How to Present the Lithium Problem?

The "lithium problem" is a known discrepancy in Big Bang nucleosynthesis: predicted ⁷Li/H ≈ 5.0 × 10⁻¹⁰ vs observed ≈ 1.6 × 10⁻¹⁰.

Should Genesis:
- Show both predicted and observed values and highlight the discrepancy as an educational feature?
- Use only the predicted value (treat observed as "problem to be explained")?
- Adjust simulation parameters to match observed values (sacrifice scientific accuracy)?
- Not include ⁷Li in validation overlay at all?

### 4. What Precision Level is Required?

The success metric specifies: "Primordial helium abundance within 5% of observed value (Y_p ≈ 0.245)"

Should other elements have similar tolerance thresholds?
- Deuterium: Within X% of observed value?
- Lithium: Within Y% of observed value?
- Or should there be no explicit tolerance for these elements?

### 5. Should Multiple Datasets Be Supported?

Should users be able to:
- Compare against Planck 2018 values (recommended)
- Compare against WMAP values
- Compare against direct observation values
- Compare against custom user-provided values

This could be useful for educational purposes (showing how different methods yield different results).

### 6. Validation Visual Presentation

How should the validation overlay present the comparison data?

**Option A: Absolute Error Bars**
- Show simulated abundance with error bar
- Show observed value with error bar
- Visual overlap indicates agreement

**Option B: Percentage Difference**
- Show simulated value as primary
- Show percentage difference from observed value
- Color-coded: green (within tolerance), yellow (close), red (outside tolerance)

**Option C: Multiple Reference Lines**
- Show simulated abundance curve over time
- Overlay horizontal lines for multiple observational datasets
- User can see which datasets align and which don't

**Option D: Simple Toggle Text**
- Text overlay showing: "Y_p: simulated=0.245, observed=0.247 (within 5% ✓)"
- No visual graph, just status indicator

### 7. Success Criteria Beyond Helium-4

The PRD specifies: "Primordial helium abundance within 5% of observed value (Y_p ≈ 0.245)"

Are there success criteria for other elements? For example:
- Deuterium: Within X% of observed value?
- Lithium: Does the 3× discrepancy need to be reproduced, or should we try to "solve" the lithium problem in the simulation?

## Suggested Approach (Default Recommendation)

Unless otherwise specified, Genesis should use:

### Reference Values
- **Helium-4:** Y_p = 0.2471 (Planck 2018)
- **Deuterium:** D/H = 2.58 × 10⁻⁵ (Planck 2018)
- **Lithium-7:** ⁷Li/H = 5.0 × 10⁻¹⁰ (Standard BBN prediction, with note about observational discrepancy)
- **Helium-3:** ³He/H = 1.0 × 10⁻⁵ (Planck 2018)

### Validation Elements
- Primordial elements: p, D, ³He, ⁴He, ⁷Li (T and ⁷Be are unstable intermediates, not part of final validation)

### Tolerance Criteria
- **Helium-4:** Within 5% (explicit in PRD)
- **Deuterium:** Within 10% (reasonable given observational uncertainties)
- **Lithium-7:** Within 20% OR explicitly reproduce the ~3× discrepancy as an educational feature
- **Helium-3:** Within 15% (observational data less precise)

### Visual Presentation
- **Option B (Percentage Difference)** with color coding
- Real-time update as simulation progresses
- Toggle button to enable/disable validation overlay
- Optional: Show multiple reference datasets if user requests

### Educational Feature for Lithium
- Include a tooltip or info panel explaining the "lithium problem"
- Show both predicted and observed values
- Note that this is an open question in cosmology

## Impact

This decision affects:
1. Phase 3 validation overlay implementation (which values to display)
2. Success criteria for Phase 3 completion (which metrics must pass)
3. Educational value of the simulation (showing real scientific discrepancies)
4. User experience (how validation information is presented)
5. Configuration file format (which parameters to include for presets)
