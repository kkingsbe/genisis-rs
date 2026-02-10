# Task Decomposition: Zel'dovich Approximation

**Date:** 2026-02-10
**Sprint:** Sprint 2 (Phase 2: Inflation & Quantum Seeds)
**Parent TODO Item:** Line 33 - "Implement Zel'dovich approximation for density-to-displacement mapping (displacement = ∇ψ where ∇²ψ = -δ)"

---

## Overview

The Zel'dovich approximation is a method for mapping density perturbations to particle displacements in the early universe. It works as follows:

1. Start with a density perturbation field δ (generated from Gaussian random field + power spectrum)
2. Solve the Poisson equation ∇²ψ = -δ to obtain a potential field ψ
3. Compute the gradient of the potential: displacement = ∇ψ
4. Apply the displacement to particle positions

In Fourier space, the Poisson equation simplifies to algebraic operations:
- Real space: -∇²ψ = δ
- Fourier space: k²ψ̂ = δ̂ (where k² = |k|²)
- Solution: ψ̂ = δ̂ / k²

Displacement in Fourier space:
- ∇ψ = ik * ψ̂
- disp̂ = ik * ψ̂ = i * k * (δ̂ / k²) = -i * δ̂ / k

---

## Subtask 1: Create DensityField Resource

**File:** genesis-physics/src/perturbations/mod.rs

**Description:**
Add a `DensityField` struct to track the density perturbation values and their derivatives.

**Implementation:**
```rust
/// Represents a cosmological density perturbation field
///
/// Stores the density perturbation δ(x) at each grid point, along with
/// spatial derivatives ∇δ and the power spectrum P(k) used to generate it.
#[derive(Debug, Clone)]
pub struct DensityField {
    /// Grid resolution (N × N × N)
    pub resolution: usize,
    /// Density perturbation values δ at each grid point [z][y][x]
    pub delta: Vec<Vec<Vec<f64>>>,
    /// X-derivative of density (∂δ/∂x)
    pub grad_x: Vec<Vec<Vec<f64>>>,
    /// Y-derivative of density (∂δ/∂y)
    pub grad_y: Vec<Vec<Vec<f64>>>,
    /// Z-derivative of density (∂δ/∂z)
    pub grad_z: Vec<Vec<Vec<f64>>>,
    /// Grid spacing (physical units per grid cell)
    pub spacing: f64,
    /// Power spectrum used to generate this field
    pub power_spectrum: PowerSpectrum,
}

impl DensityField {
    /// Creates a new density field from a Gaussian random field
    ///
    /// Applies power spectrum P(k) to generate cosmologically-relevant
    /// density perturbations.
    pub fn from_gaussian_random_field(
        grf: &GaussianRandomField,
        power_spectrum: &PowerSpectrum,
        seed: u64,
    ) -> Result<Self, String> {
        // Implementation will use DensityFft to:
        // 1. Flatten GRF to 1D array
        // 2. Transform to k-space
        // 3. Apply power spectrum
        // 4. Transform back to real-space
        // 5. Store result in delta field
    }
}
```

---

## Subtask 2: Implement Potential Solver (Poisson Equation)

**File:** genesis-physics/src/perturbations/mod.rs

**Description:**
Implement a function to solve ∇²ψ = -δ for the potential field ψ using FFT.

**Mathematical Background:**
In Fourier space, the Poisson equation becomes:
```
∇²ψ = -δ
-k²ψ̂ = -δ̂
ψ̂ = δ̂ / k²
```

where k² = k_x² + k_y² + k_z².

**Implementation:**
```rust
/// Solves the Poisson equation ∇²ψ = -δ to obtain potential field ψ
///
/// Uses FFT to transform to k-space where the Poisson equation
/// becomes algebraic: ψ̂(k) = δ̂(k) / k²
///
/// # Arguments
///
/// * `density_field` - The density perturbation field δ
/// * `fft_engine` - FFT engine for transforming between spaces
///
/// # Returns
///
/// Potential field ψ as a 3D array [z][y][x]
///
/// # Note
///
/// The DC component (k=0) is set to zero since the mean density
/// is zero.
pub fn solve_poisson_equation(
    density_field: &DensityField,
    fft_engine: &mut DensityFft,
) -> Result<Vec<Vec<Vec<f64>>>, String> {
    // 1. Flatten delta field to 1D array
    // 2. Transform to k-space using fft_engine.real_to_kspace()
    // 3. For each k-space component:
    //    - Calculate k² = kx² + ky² + kz²
    //    - If k² > 0: ψ̂ = δ̂ / k²
    //    - If k² = 0: ψ̂ = 0 (DC component)
    // 4. Transform back to real-space using fft_engine.kspace_to_real()
    // 5. Reshape to 3D array and return
}
```

---

## Subtask 3: Implement Gradient Computation

**File:** genesis-physics/src/perturbations/mod.rs

**Description:**
Compute the gradient of the potential field in Fourier space: displacement = ∇ψ.

**Mathematical Background:**
In Fourier space, the gradient operator becomes multiplication by ik:
```
∂ψ/∂x → ik_x * ψ̂
∂ψ/∂y → ik_y * ψ̂
∂ψ/∂z → ik_z * ψ̂
```

**Implementation:**
```rust
/// Computes the displacement field from potential ψ via gradient: disp = ∇ψ
///
/// In Fourier space, the gradient becomes multiplication by the wave vector:
/// disp̂(k) = i * k * ψ̂(k)
///
/// # Arguments
///
/// * `potential_kspace` - Potential field ψ in k-space (complex)
/// * `size` - Grid resolution
///
/// # Returns
///
/// Tuple of (disp_x, disp_y, disp_z) as 3D arrays
pub fn compute_displacement_from_potential(
    potential_kspace: &[Complex<f64>],
    size: usize,
) -> Result<(Vec<Vec<Vec<f64>>), String> {
    // For each k-space component:
    // 1. Calculate wave vector (kx, ky, kz) with proper FFT ordering
    // 2. Compute ψ̂ from input
    // 3. Calculate displacement in k-space:
    //    - disp_x_kspace = i * kx * ψ̂
    //    - disp_y_kspace = i * ky * ψ̂
    //    - disp_z_kspace = i * kz * ψ̂
    // 4. Transform each displacement component back to real-space via inverse FFT
    // 5. Extract real parts and return as 3D arrays
}
```

---

## Subtask 4: Add Zel'dovich Integration Function

**File:** genesis-physics/src/perturbations/mod.rs

**Description:**
Create a high-level function that orchestrates the Zel'dovich approximation pipeline.

**Implementation:**
```rust
/// Applies the Zel'dovich approximation to convert density perturbations
/// to particle displacements
///
/// The Zel'dovich approximation maps density perturbations δ to particle
/// displacements by:
/// 1. Solving ∇²ψ = -δ for potential ψ
/// 2. Computing displacement = ∇ψ
/// 3. Applying displacement to particle positions
///
/// # Arguments
///
/// * `density_field` - Density perturbation field δ
/// * `particle_positions` - Mutable reference to particle positions to modify
/// * `spacing` - Physical spacing of the perturbation grid
///
/// # Returns
///
/// Ok(()) on success, Err(String) on failure
pub fn apply_zeldovich_approximation(
    density_field: &DensityField,
    particle_positions: &mut Vec<[f64; 3]>,
    spacing: f64,
) -> Result<(), String> {
    // 1. Create FFT engine
    // 2. Solve Poisson equation to get potential ψ
    // 3. Transform potential to k-space
    // 4. Compute displacement = ∇ψ in Fourier space
    // 5. Transform displacement components back to real-space
    // 6. For each particle:
    //    - Find grid cell containing the particle
    //    - Trilinear interpolation of displacement at particle position
    //    - Add displacement to particle position
}
```

---

## Dependencies and Existing Infrastructure

**Already Available:**
- `PowerSpectrum::compute(k)` - Computes P(k) at wavenumber k
- `GaussianRandomField::generate(resolution, spacing, seed)` - Generates 3D Gaussian field
- `DensityFft::real_to_kspace()` - Transforms real-space to k-space
- `DensityFft::kspace_to_real()` - Transforms k-space to real-space
- `DensityFft::apply_power_spectrum()` - Applies power spectrum in k-space

**Dependencies in Cargo.toml:**
- `rustfft` - Already present in genesis-physics/Cargo.toml

---

## Testing Strategy

**Unit Tests:**
1. Test solve_poisson_equation with known simple field
2. Test compute_displacement_from_potential with synthetic k-space data
3. Test apply_zeldovich_approximation end-to-end

**Integration Tests:**
4. Create test in genesis-physics/tests/zeldovich_integration.rs that:
   - Generates a small density field (8×8×8)
   - Applies Zel'dovich approximation
   - Verifies displacement field has expected statistical properties

---

## Verification Criteria

After implementation, verify:
1. `cargo test --package genesis-physics --lib` passes with new tests
2. Displacement field correlates with density field (high-density regions have inward displacement)
3. Mean displacement is approximately zero (momentum conservation)
4. Particles displaced in direction of potential gradient

---

## Next TODO Items After This Task

Once Zel'dovich approximation is complete:
- [ ] Map density perturbations to particle displacement (add displacement vectors to particle positions on spawn) - TODO line 34
- [ ] Map density perturbations to particle color intensity - TODO line 35
- [ ] Add DensityField resource tracking perturbation values δ, derivatives ∇δ, and power spectrum P(k) - TODO line 36
- [ ] Create GaussianRandomField resource tracking grid size, seed, and generated field data - TODO line 37
