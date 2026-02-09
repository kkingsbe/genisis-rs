# Question: Phase 3 Nucleosynthesis Configuration Presets

## Date
2026-02-09

## Context

Phase 3 implements Nucleosynthesis & the First Elements with configuration presets for comparing different cosmological models.

## Ambiguity Identified

**Phase 3 (Line 162):** "TOML configuration presets: 'Standard Model' (Planck 2018 best-fit) and 'High Baryon Density' for comparison"

**Phase 3 (Line 163):** "Validation overlay (toggle-able): comparison lines showing observed primordial abundances (Y_p ≈ 0.245 for ⁴He)"

The PRD mentions specific presets and validation values but does not provide:
1. The exact parameter values for each preset
2. Which cosmological parameters are included in the presets
3. How presets affect the nucleosynthesis reaction network
4. What other observational comparison values to include (besides Y_p ≈ 0.245)

## Why This Is a Problem

1. **Preset Parameter Values Not Specified:**
   - What are the exact values for "Standard Model (Planck 2018 best-fit)"?
   - What parameters should be included (Ω_b, h, n_s, Y_p initial, neutron lifetime, etc.)?
   - What are the values for "High Baryon Density" preset?
   - What constitutes "high" baryon density (how much higher than standard)?

2. **Nucleosynthesis Parameter Set Unclear:**
   - Which parameters affect the nuclear reaction network?
   - Is it just baryon density (Ω_b h²)?
   - Does it include neutron lifetime, number of neutrino species, etc.?
   - Are there parameters not related to nucleosynthesis (e.g., σ₈, n_s)?

3. **Validation Comparison Values Incomplete:**
   - PRD only mentions Y_p ≈ 0.245 for ⁴He
   - What about other primordial abundances (deuterium D/H, helium-3 ³He, lithium-7 ⁷Li)?
   - What are the observed values for these isotopes?
   - Which data sources (Planck, WMAP, Big Bang Nucleosynthesis calculations)?

4. **Preset Storage Format Not Defined:**
   - Are presets stored in separate TOML files (e.g., presets/standard-model.toml)?
   - Or are they embedded in genesis.toml as [presets.standard-model]?
   - How does user select which preset to use?
   - Can users create custom presets?

5. **Preset Switching Behavior:**
   - Does switching presets restart the simulation?
   - Can presets be switched during live playback?
   - Do presets affect only nucleosynthesis phase, or entire simulation?

## Suggested Approaches

1. **Standard Cosmological Parameters in TOML (Recommended)**
   - Define presets with well-known cosmological parameter sets
   - Standard Model: Ω_b h² = 0.0224, n_s = 0.965, Y_p initial = 0.247, τ_n = 880 s
   - High Baryon Density: Ω_b h² = 0.030 (approx. 34% higher)
   - Presets stored in genesis.toml under [presets.*] sections
   - Pros: Uses standard values from literature, scientifically accurate, simple implementation
   - Cons: Limited to known presets, users can't easily create custom ones

2. **User-Creatable Presets with Template Files**
   - Provide example preset files (e.g., presets/standard-model.toml, presets/high-density.toml)
   - Users can copy and modify to create custom presets
   - Presets loaded from presets/ directory
   - Pros: Flexible, users can explore parameter space, easy to share
   - Cons: More complex file handling, potential for invalid user presets

3. **Parameter Panel Instead of Presets**
   - No preset system
   - Users adjust parameters directly via UI panel
   - UI shows "reset to standard" buttons for common configurations
   - Pros: Maximum flexibility, no file management, simpler code
   - Cons: No preset storage/sharing, users must remember values, no presets for PRD requirement

4. **Both Presets and Manual Adjustment**
   - Provide presets (Approach 1) as quick-start configurations
   - Allow manual parameter adjustment via UI panel
   - Users can load preset then fine-tune parameters
   - Pros: Best of both worlds, scientific + exploratory
   - Cons: More complex UI and preset system

## Reference: Standard Cosmological Parameters

The following values are from Planck 2018 and Big Bang Nucleosynthesis literature:

### Standard Model (Planck 2018):
- Ω_b h² = 0.0224 ± 0.0001 (baryon density)
- h = 0.674 ± 0.005 (Hubble parameter)
- n_s = 0.965 ± 0.004 (scalar spectral index)
- Y_p = 0.2450 ± 0.0001 (primordial helium-4 mass fraction)
- D/H = (2.57 ± 0.05) × 10⁻⁵ (deuterium/hydrogen ratio)
- ³He/H = (1.0 ± 0.2) × 10⁻⁵ (helium-3/hydrogen ratio)
- ⁷Li/H = (1.6 ± 0.3) × 10⁻¹⁰ (lithium-7/hydrogen ratio)
- τ_n = 880.2 ± 1.1 s (neutron lifetime)
- N_eff = 3.046 (effective number of neutrino species)

### High Baryon Density (Example):
- Ω_b h² = 0.030 (33.6% higher than Standard Model)
- Other parameters same as Standard Model
- Result: Y_p ≈ 0.28-0.30 (helium-4 overshoots observed value)

### Parameters Affecting Nucleosynthesis:
- Ω_b h²: Baryon density (primary parameter - directly affects reaction rates)
- τ_n: Neutron lifetime (affects n/p ratio)
- N_eff: Number of neutrino species (affects expansion rate)
- G: Gravitational constant (usually fixed)
- Other parameters (n_s, σ₈, Ω_m, Ω_Λ) do NOT directly affect BBN

## Suggested Preset Implementation

### Option A: Embedded in genesis.toml
```toml
# genesis.toml

[presets.standard-model]
name = "Standard Model (Planck 2018)"
description = "Best-fit parameters from Planck 2018 cosmology"
omega_b_h2 = 0.0224
n_s = 0.965
neutron_lifetime = 880.2
n_eff = 3.046

[presets.high-density]
name = "High Baryon Density"
description = "33% higher baryon density than standard model"
omega_b_h2 = 0.030
n_s = 0.965
neutron_lifetime = 880.2
n_eff = 3.046
```

### Option B: Separate Preset Files
```toml
# presets/standard-model.toml
name = "Standard Model (Planck 2018)"
description = "Best-fit parameters from Planck 2018 cosmology"
omega_b_h2 = 0.0224
n_s = 0.965
neutron_lifetime = 880.2
n_eff = 3.046
```

```toml
# presets/high-density.toml
name = "High Baryon Density"
description = "33% higher baryon density than standard model"
omega_b_h2 = 0.030
n_s = 0.965
neutron_lifetime = 880.2
n_eff = 3.046
```

### Option C: Configuration System with Preset Selection
```toml
# genesis.toml

[cosmology]
preset = "standard-model"  # or "high-density", "custom"
omega_b_h2 = 0.0224       # used only if preset = "custom"
n_s = 0.965
neutron_lifetime = 880.2
n_eff = 3.046

[validation]
show_comparison = true
y_p_observed = 0.2450
d_h_observed = 2.57e-5
he3_h_observed = 1.0e-5
li7_h_observed = 1.6e-10
```

## Validation Overlay Specifications

### Observed Primordial Abundances (from literature):
| Isotope | Observed Value | Uncertainty | Source |
|---------|---------------|-------------|--------|
| Y_p (helium-4) | 0.2450 | ±0.0001 | Planck 2018 |
| D/H (deuterium) | 2.57 × 10⁻⁵ | ±0.05 × 10⁻⁵ | Cooke et al. 2018 |
| ³He/H (helium-3) | 1.0 × 10⁻⁵ | ±0.2 × 10⁻⁵ | Bania et al. 2002 |
| ⁷Li/H (lithium-7) | 1.6 × 10⁻¹⁰ | ±0.3 × 10⁻¹⁰ | Sbordone et al. 2010 |

### Suggested Validation Overlay Design:
- Live bar chart showing evolving abundances (as specified in PRD Line 158)
- Horizontal comparison lines showing observed values (as specified in PRD Line 163)
- Color coding: green for within 1σ, yellow for 1-2σ, red for >2σ
- Toggle button to show/hide comparison lines
- Display numerical values alongside bars

## Additional Questions

- **Preset Scope:** Should presets affect only nucleosynthesis parameters, or all cosmological parameters (Ω_m, Ω_Λ, H₀, n_s, σ₈)?
- **Preset Persistence:** When a preset is selected, does it overwrite current parameters, or can parameters be modified after loading a preset?
- **Validation Data Source:** Should observed abundance values be hardcoded, loaded from external data file, or computed from observational references?
- **Custom Presets:** Should users be able to save their own parameter sets as presets for later recall?
- **Reset Behavior:** Does selecting a preset reset the simulation to t=0, or apply new parameters to current simulation state?

## Question for Product Owner

Which preset implementation approach should we use?
- Embedded presets in genesis.toml (Option A)?
- [x] Separate preset files (Option B)?
- Configuration system with preset selection (Option C)?

Also:
- Should we provide only the two PRD-specified presets (Standard Model, High Baryon Density), or add additional presets?
- Which parameters should be included in presets (nucleosynthesis-only or full cosmological parameter set)?
- How should the validation overlay display observational comparison values for all four isotopes (Y_p, D/H, ³He/H, ⁷Li/H)?
- Should users be able to create and save custom presets?
