# Question: Temperature Calculation and Display Across Phases 2-4

## Date
2026-02-09

## Context

The PRD references temperature calculations and displays across multiple phases without specifying the exact calculation method or how temperature evolves through the cosmic epochs.

## Ambiguity Identified

**Phase 2 (Line 140):** "Procedural QGP visualization: during quark-gluon plasma phase, particles rendered as glowing plasma blobs with temperature-mapped color ramp (blue-white at peak, cooling through yellow to orange)"

**Phase 2 Demo Moment (Line 144):** "...live temperature readout dropping from 10²⁷ K."

**Phase 3 (Line 156):** "Stiff ODE solver... for 12-species nuclear reaction network"

**Phase 4 (Line 178):** "Saha equation solver tracking ionization fraction x_e as a function of temperature"

**Phase 4 (Line 182):** "Temperature readout drops through 3000 K (recombination) toward 2.725 K (present-day CMB)"

## Why This Is a Problem

1. **Temperature Model Not Specified:** The PRD mentions temperature values (10²⁷ K at QGP, 3000 K at recombination, 2.725 K for CMB) but does not specify:
   - The thermodynamic model to calculate temperature over cosmic time
   - Whether temperature is derived from scale factor a(t) or calculated independently
   - The relationship between radiation temperature, matter temperature, and CMB temperature

2. **Temperature Phase Dependencies:**
   - Phase 2 requires temperature for QGP visualization and color ramp
   - Phase 3 requires temperature for reaction rate calculations (nuclear reaction network)
   - Phase 4 requires temperature for ionization fraction (Saha equation)
   - Each phase may have different temperature models, causing potential inconsistencies

3. **Temperature-Driven Visual Effects:**
   - Phase 2: Particle colors based on temperature
   - Phase 3: Reaction rates depend on temperature
   - Phase 4: Ionization fraction depends on temperature
   - Without a unified temperature model, these systems may produce inconsistent results

4. **Thermodynamic Regimes Not Defined:**
   - QGP phase (10²⁷ K): Radiation-dominated
   - Recombination (3000 K): Matter-radiation equality
   - CMB (2.725 K): Present-day temperature
   - The transitions between regimes are not specified

## Suggested Approaches

1. **Standard Friedmann Model (Recommended)**
   - Use standard cosmology: T(t) = T₀ / a(t) for radiation temperature
   - T₀ = 2.725 K (current CMB temperature)
   - a(t) is scale factor from Friedmann equation
   - Calculate T(t) directly from timeline cosmic time
   - Pros: Physically accurate, simple implementation, well-established
   - Cons: Doesn't capture all thermodynamic effects (e.g., reheating after recombination)

2. **Pre-Computed Temperature Table**
   - Create a lookup table of temperature vs. cosmic time
   - Interpolate between known points (e.g., 10⁻³²s, 3 min, 380Kyr, present)
   - Smooth interpolation for continuous temperature curve
   - Pros: Simple to implement, guaranteed smooth transitions, easy to tune
   - Cons: Less physically accurate, requires manual curation of temperature values

3. **Multi-Component Temperature Model**
   - Track separate temperatures: radiation Tᵣ, matter Tₘ, neutrino Tᵥ
   - Calculate evolution based on thermodynamic coupling/decoupling
   - Use appropriate temperature for each physical process (e.g., Tᵣ for CMB, Tₘ for nucleosynthesis)
   - Pros: Scientifically accurate, captures all temperature regimes
   - Cons: Very complex, requires coupling calculations, potentially unnecessary for visual simulation

4. **User-Configurable Temperature Function**
   - Allow temperature function to be specified in configuration (TOML)
   - Default: T(t) = T₀ / a(t)
   - Users can customize for "what-if" scenarios
   - Pros: Flexible, supports parameter exploration
   - Cons: More complex configuration system, user may create physically incorrect scenarios

## Additional Questions

- **Temperature Display Location:** Where should the temperature readout appear?
  - In the epoch indicator overlay (Phase 2)?
  - In a separate panel?
  - As part of the parameter sidebar?

- **Temperature Units:** What units should be displayed?
  - Kelvin (K) for all epochs?
  - Logarithmic scale for early universe (e.g., "log₁₀(T) = 27")?
  - Electron volts (eV) for particle physics regimes?

- **Temperature Interpolation:** How should temperature change during timeline scrubbing?
  - Recalculate from a(t) at each frame?
  - Interpolate pre-computed values?
  - Cache and reuse?

## Reference: Related PRD Sections

- **Phase 2, Line 138:** "Epoch indicator in UI showing current cosmic era and key parameters (temperature, scale factor, time)" - Temperature is a key parameter to display
- **Phase 3, Line 157:** "Reaction rates from NACRE II compilation, temperature-dependent" - Temperature directly affects nuclear reaction rates
- **Phase 4, Line 178:** "Saha equation solver tracking ionization fraction x_e as a function of temperature" - Temperature is the input to Saha equation

## Question for Product Owner

Which temperature calculation approach should we use? Given the emphasis on "physically grounded" simulation, should we use the standard Friedmann model (Approach 1), or would a simplified pre-computed table be sufficient (Approach 2)?

Also:
- Where should the temperature readout be displayed?
- What temperature units should be used for display?
- Should temperature be configurable for "what-if" scenarios (Approach 4)?

This decision affects Phases 2, 3, and 4, as all depend on temperature for calculations and visualization.
