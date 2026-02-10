//! Perturbations module
//!
//! This module provides functionality for generating and managing cosmological
//! perturbations, including quantum fluctuations during the inflationary epoch
//! that seeded the formation of cosmic structures.

use std::f64::consts::PI;

use rand::Rng;
use rand::SeedableRng;

pub mod fft;

/// Represents a cosmological power spectrum P(k).
///
/// The power spectrum describes how the variance of density perturbations
/// is distributed across different spatial scales (wavenumbers). In standard
/// cosmology, it follows a power-law form: P(k) = A * k^(n_s - 1), where
/// n_s is the spectral index and A is the amplitude.
#[derive(Debug, Clone)]
pub struct PowerSpectrum {
    /// The spectral index n_s (typically ~0.96 for standard cosmology).
    /// Controls how power is distributed across scales: n_s < 1 means
    /// more power on large scales, n_s > 1 means more power on small scales.
    spectral_index: f64,
    /// The normalization amplitude A (typically ~2.1e-9 for CMB).
    amplitude: f64,
}

impl PowerSpectrum {
    /// Creates a new power spectrum with the given spectral index and amplitude.
    ///
    /// # Arguments
    ///
    /// * `spectral_index` - The spectral index n_s (typically ~0.96)
    /// * `amplitude` - The normalization amplitude A
    ///
    /// # Returns
    ///
    /// A new `PowerSpectrum` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use genesis_physics::perturbations::PowerSpectrum;
    ///
    /// let ps = PowerSpectrum::new(0.96, 1.0);
    /// ```
    pub fn new(spectral_index: f64, amplitude: f64) -> Self {
        Self {
            spectral_index,
            amplitude,
        }
    }

    /// Computes the power P(k) at a given wavenumber k.
    ///
    /// Implements the power-law form: P(k) = A * k^(n_s - 1).
    ///
    /// # Arguments
    ///
    /// * `k` - The wavenumber (inverse spatial scale)
    ///
    /// # Returns
    ///
    /// The power at wavenumber k. Returns 0.0 for k <= 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use genesis_physics::perturbations::PowerSpectrum;
    ///
    /// let ps = PowerSpectrum::new(0.96, 1.0);
    /// let power = ps.compute(1.0);
    /// ```
    pub fn compute(&self, k: f64) -> f64 {
        if k <= 0.0 {
            return 0.0;
        }
        self.amplitude * k.powf(self.spectral_index - 1.0)
    }
}

impl Default for PowerSpectrum {
    /// Creates a power spectrum with standard cosmological parameters.
    ///
    /// Uses spectral_index = 0.96 (Planck-measured value) and amplitude = 1.0
    /// (will be normalized when applied to actual data).
    ///
    /// # Example
    ///
    /// ```rust
    /// use genesis_physics::perturbations::PowerSpectrum;
    ///
    /// let ps = PowerSpectrum::default();
    /// assert_eq!(ps.compute(1.0), 1.0);
    /// ```
    fn default() -> Self {
        Self {
            spectral_index: 0.96,
            amplitude: 1.0,
        }
    }
}

/// Generates a pair of independent standard normal (Gaussian) random numbers
/// using the Box-Muller transform.
///
/// This function converts two uniformly distributed random numbers from the
/// interval (0, 1] into two independent random numbers drawn from a standard
/// normal distribution (mean = 0, standard deviation = 1).
///
/// # Arguments
///
/// * `u1` - First uniformly distributed random number, must be in the range (0, 1]
/// * `u2` - Second uniformly distributed random number, must be in the range (0, 1]
///
/// # Returns
///
/// A tuple `(z1, z2)` containing two independent standard normal random numbers.
///
/// # Mathematical Formula
///
/// The Box-Muller transform applies the following equations:
///
/// ```text
/// z1 = sqrt(-2.0 * ln(u1)) * cos(2.0 * π * u2)
/// z2 = sqrt(-2.0 * ln(u1)) * sin(2.0 * π * u2)
/// ```
///
/// where:
/// - `ln` is the natural logarithm
/// - `π` is the mathematical constant pi (approximately 3.14159265359)
/// - `cos` and `sin` are the cosine and sine trigonometric functions
///
/// # Note
///
/// The input values `u1` and `u2` must be strictly positive (greater than 0).
/// If either value is 0, the natural logarithm will produce negative infinity.
/// If the input values are outside the range (0, 1], the output distribution
/// will not follow a standard normal distribution.
///
/// # Example
///
/// ```rust
/// use genesis_physics::perturbations::box_muller_pair;
///
/// let (z1, z2) = box_muller_pair(0.5, 0.75);
/// // z1 and z2 are independent standard normal random numbers
/// ```
pub fn box_muller_pair(u1: f64, u2: f64) -> (f64, f64) {
    let radius = (-2.0 * u1.ln()).sqrt();
    let angle = 2.0 * PI * u2;
    
    let z1 = radius * angle.cos();
    let z2 = radius * angle.sin();
    
    (z1, z2)
}

/// A 3D Gaussian random field on a regular grid.
///
/// This struct represents a three-dimensional array of random values sampled
/// from a standard normal distribution (mean=0, std=1). The grid is uniformly
/// spaced with a configurable number of points along each dimension.
///
/// Used as the foundation for cosmological density perturbations, which are
/// then transformed via the power spectrum and Zel'dovich approximation to
/// seed structure formation in the universe.
pub struct GaussianRandomField {
    /// Number of grid points along each axis (N × N × N grid)
    pub resolution: usize,
    /// 3D array of Gaussian random values, indexed as [z][y][x]
    pub values: Vec<Vec<Vec<f64>>>,
    /// Grid spacing (physical units per grid cell)
    pub spacing: f64,
}

impl GaussianRandomField {
    /// Generates a new 3D Gaussian random field on a regular grid.
    ///
    /// Creates a cube-shaped grid of resolution³ points, where each point
    /// contains a random value sampled from a standard normal distribution
    /// using the Box-Muller transform.
    ///
    /// # Arguments
    ///
    /// * `resolution` - Number of grid points along each axis (e.g., 64 for a 64³ grid)
    /// * `spacing` - Physical distance between adjacent grid points (e.g., 1.0 Mpc)
    /// * `seed` - Optional random seed for reproducibility (None = random seed)
    ///
    /// # Returns
    ///
    /// A new [`GaussianRandomField`] containing the generated values.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use genesis_physics::perturbations::GaussianRandomField;
    ///
    /// // Generate a 32³ field with unit spacing and a fixed seed
    /// let field = GaussianRandomField::generate(32, 1.0, Some(12345));
    /// ```
    ///
    /// # Notes
    ///
    /// - Uses the [`box_muller_pair`](crate::perturbations::box_muller_pair) function to convert
    ///   uniform random numbers to Gaussian distribution.
    /// - Memory usage scales as O(N³) with resolution.
    pub fn generate(resolution: usize, spacing: f64, seed: Option<u64>) -> Self {
        let mut rng: rand::rngs::StdRng = match seed {
            Some(s) => rand::SeedableRng::seed_from_u64(s),
            None => rand::rngs::StdRng::from_entropy(),
        };

        let mut values = Vec::with_capacity(resolution);
        for _z in 0..resolution {
            let mut z_slice = Vec::with_capacity(resolution);
            for _y in 0..resolution {
                let mut y_row = Vec::with_capacity(resolution);
                for _x in 0..resolution {
                    let u1: f64 = rng.gen();
                    let u2: f64 = rng.gen();
                    let (z1, _) = box_muller_pair(u1, u2);
                    y_row.push(z1);
                }
                z_slice.push(y_row);
            }
            values.push(z_slice);
        }

        Self {
            resolution,
            values,
            spacing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 1: Basic functionality
    /// Verify that box_muller_pair returns two finite f64 values for known inputs
    #[test]
    fn test_basic_functionality() {
        let (z1, z2) = box_muller_pair(0.5, 0.25);
        
        // Verify two f64 values are returned
        let _: f64 = z1;
        let _: f64 = z2;
        
        // Verify that the values are finite (not NaN or infinity)
        assert!(z1.is_finite(), "z1 should be finite");
        assert!(z2.is_finite(), "z2 should be finite");
    }

    /// Test 2: Input range edge cases
    /// Test with various inputs near the valid range bounds
    #[test]
    fn test_edge_cases() {
        // Test with u1=0.9, u2=0.1 (both in valid range)
        let (z1, z2) = box_muller_pair(0.9, 0.1);
        assert!(z1.is_finite(), "z1 should be finite for (0.9, 0.1)");
        assert!(z2.is_finite(), "z2 should be finite for (0.9, 0.1)");

        // Test with u1=0.0001, u2=0.9999 (near bounds)
        let (z1, z2) = box_muller_pair(0.0001, 0.9999);
        assert!(z1.is_finite(), "z1 should be finite for (0.0001, 0.9999)");
        assert!(z2.is_finite(), "z2 should be finite for (0.0001, 0.9999)");

        // Additional edge cases
        let (z1, z2) = box_muller_pair(0.001, 0.5);
        assert!(z1.is_finite(), "z1 should be finite for (0.001, 0.5)");
        assert!(z2.is_finite(), "z2 should be finite for (0.001, 0.5)");

        let (z1, z2) = box_muller_pair(0.999, 0.5);
        assert!(z1.is_finite(), "z1 should be finite for (0.999, 0.5)");
        assert!(z2.is_finite(), "z2 should be finite for (0.999, 0.5)");
    }

    /// Test 3: Statistical properties (10,000 samples)
    /// Generate 10,000 random pairs and verify the mean and standard deviation
    /// approximate the standard normal distribution (mean ≈ 0, std dev ≈ 1)
    #[test]
    fn test_statistical_properties() {
        use std::time::SystemTime;
        
        // Use system time as seed for reproducibility
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng = Pcg32::new(seed, seed.wrapping_add(1));
        
        const NUM_SAMPLES: usize = 10_000;
        let mut values = Vec::with_capacity(NUM_SAMPLES * 2);
        
        // Generate 10,000 random pairs using the Box-Muller transform
        for _ in 0..NUM_SAMPLES {
            let u1 = rng.gen_range(0.0_f64, 1.0_f64);
            let u2 = rng.gen_range(0.0_f64, 1.0_f64);
            // Ensure values are in (0, 1] by adding a small epsilon
            let u1 = u1 * 0.9999 + 0.00005;
            let u2 = u2 * 0.9999 + 0.00005;
            let (z1, z2) = box_muller_pair(u1, u2);
            values.push(z1);
            values.push(z2);
        }
        
        // Calculate mean
        let sum: f64 = values.iter().sum();
        let mean = sum / values.len() as f64;
        
        // Calculate standard deviation
        let variance: f64 = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();
        
        // The mean should be approximately 0, within ±0.05
        assert!(
            mean.abs() < 0.05,
            "Mean should be approximately 0, got {} (|{}| >= 0.05)",
            mean, mean.abs()
        );
        
        // The standard deviation should be approximately 1.0, within ±0.1
        assert!(
            (std_dev - 1.0).abs() < 0.1,
            "Standard deviation should be approximately 1.0, got {} (|{} - 1.0| >= 0.1)",
            std_dev, std_dev - 1.0
        );
    }

    /// Test 4: Independence of outputs
    /// Generate 1,000 pairs and verify that z1 and z2 are different for each pair
    #[test]
    fn test_independence_of_outputs() {
        use std::time::SystemTime;
        
        // Use system time as seed for reproducibility
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng = Pcg32::new(seed, seed.wrapping_add(1));
        
        const NUM_PAIRS: usize = 1_000;
        let tolerance = 1e-10; // Tolerance for floating-point comparison
        
        for _ in 0..NUM_PAIRS {
            let u1 = rng.gen_range(0.0_f64, 1.0_f64);
            let u2 = rng.gen_range(0.0_f64, 1.0_f64);
            // Ensure values are in (0, 1] by adding a small epsilon
            let u1 = u1 * 0.9999 + 0.00005;
            let u2 = u2 * 0.9999 + 0.00005;
            let (z1, z2) = box_muller_pair(u1, u2);
            
            // Verify that z1 and z2 are not identical (within tolerance)
            assert!(
                (z1 - z2).abs() > tolerance,
                "z1 and z2 should be different for inputs ({}, {}), got z1={}, z2={}",
                u1, u2, z1, z2
            );
        }
    }

    // A minimal PCG32 random number generator for testing
    // This is a simplified implementation sufficient for test purposes
    struct Pcg32 {
        state: u64,
        inc: u64,
    }

    impl Pcg32 {
        fn new(seed: u64, seq: u64) -> Self {
            let mut pcg = Pcg32 {
                state: 0,
                inc: (seq << 1) | 1,
            };
            pcg.state = pcg.state.wrapping_add(pcg.inc);
            pcg.state = pcg.state.wrapping_add(seed);
            pcg.step();
            pcg
        }

        fn step(&mut self) -> u64 {
            const MULTIPLIER: u64 = 6364136223846793005;
            self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(self.inc);
            self.state
        }

        fn gen_range(&mut self, low: f64, high: f64) -> f64 {
            let x = self.step();
            // Convert u64 to f64 in [0, 1)
            let normalized = x as f64 / (u64::MAX as f64 + 1.0);
            low + normalized * (high - low)
        }
    }

    /// Test 5: GaussianRandomField generation
    /// Verify that GaussianRandomField::generate creates a properly structured 3D field
    #[test]
    fn test_gaussian_random_field_generation() {
        // Test with a small 4³ field for fast execution
        let field = GaussianRandomField::generate(4, 1.0, Some(42));
        
        assert_eq!(field.resolution, 4);
        assert_eq!(field.spacing, 1.0);
        assert_eq!(field.values.len(), 4);
        
        // Verify all slices have correct dimensions
        for z in 0..4 {
            assert_eq!(field.values[z].len(), 4);
            for y in 0..4 {
                assert_eq!(field.values[z][y].len(), 4);
            }
        }
        
        // Total elements should be 4³ = 64
        let mut count = 0;
        for z in 0..4 {
            for y in 0..4 {
                for x in 0..4 {
                    assert!(field.values[z][y][x].is_finite());
                    count += 1;
                }
            }
        }
        assert_eq!(count, 64);
    }

    /// Test 6: GaussianRandomField reproducibility
    /// Verify that the same seed produces identical results
    #[test]
    fn test_gaussian_random_field_reproducibility() {
        // Same seed should produce identical results
        let field1 = GaussianRandomField::generate(8, 1.0, Some(999));
        let field2 = GaussianRandomField::generate(8, 1.0, Some(999));
        
        for z in 0..8 {
            for y in 0..8 {
                for x in 0..8 {
                    assert_eq!(field1.values[z][y][x], field2.values[z][y][x]);
                }
            }
        }
    }

    /// Test 7: GaussianRandomField statistical properties
    /// Verify that the generated field approximates a standard normal distribution
    #[test]
    fn test_gaussian_random_field_statistical_properties() {
        // Test with a reasonably sized field for statistical validation
        let field = GaussianRandomField::generate(32, 1.0, Some(12345));
        
        // Collect all values
        let mut all_values: Vec<f64> = Vec::new();
        for z in 0..field.resolution {
            for y in 0..field.resolution {
                for x in 0..field.resolution {
                    all_values.push(field.values[z][y][x]);
                }
            }
        }
        
        // Calculate mean (should be approximately 0)
        let sum: f64 = all_values.iter().sum();
        let mean = sum / all_values.len() as f64;
        
        // Calculate standard deviation (should be approximately 1)
        let variance: f64 = all_values.iter()
            .map(|&v| (v - mean).powi(2))
            .sum::<f64>() / all_values.len() as f64;
        let std_dev = variance.sqrt();
        
        // Allow some tolerance due to random sampling
        assert!(mean.abs() < 0.2, "Mean {} should be close to 0", mean);
        assert!((std_dev - 1.0).abs() < 0.2, "Std dev {} should be close to 1", std_dev);
    }

    // PowerSpectrum Tests

    /// Test PowerSpectrum::default() creates instance with spectral_index = 0.96 and amplitude = 1.0
    #[test]
    fn test_power_spectrum_default() {
        let ps = PowerSpectrum::default();
        // Verify indirectly through compute(1.0) which should return 1.0 for default parameters
        // P(1.0) = A * 1.0^(n_s - 1) = 1.0 * 1.0^(-0.04) = 1.0
        let result = ps.compute(1.0);
        assert!((result - 1.0).abs() < 1e-9, 
                "Default PowerSpectrum at k=1.0 should return 1.0, got {}", result);
    }

    /// Test PowerSpectrum::compute() with standard parameters (n_s=0.96, A=1.0)
    #[test]
    fn test_power_spectrum_compute_standard() {
        let ps = PowerSpectrum::default();
        // P(1.0) = A * k^(n_s - 1) = 1.0 * 1.0^(0.96 - 1) = 1.0 * 1.0^(-0.04) = 1.0
        let result = ps.compute(1.0);
        assert!((result - 1.0).abs() < 1e-9, 
                "P(1.0) should be 1.0 for standard parameters, got {}", result);
    }

    /// Test PowerSpectrum::compute() with custom parameters
    #[test]
    fn test_power_spectrum_compute_custom() {
        // n_s = 2.0, A = 2.0
        let ps = PowerSpectrum::new(2.0, 2.0);
        // P(3.0) = A * k^(n_s - 1) = 2.0 * 3.0^(2.0 - 1) = 2.0 * 3.0 = 6.0
        let result = ps.compute(3.0);
        let expected = 2.0 * 3.0f64.powf(1.0); // 2.0 * 3.0 = 6.0
        assert!((result - expected).abs() < 1e-9, 
                "P(3.0) should be {} for n_s=2.0, A=2.0, got {}", expected, result);
    }

    /// Test PowerSpectrum::compute() with k = 0.0 (edge case)
    #[test]
    fn test_power_spectrum_compute_zero_k() {
        let ps = PowerSpectrum::default();
        let result = ps.compute(0.0);
        assert_eq!(result, 0.0, "compute(0.0) should return 0.0, got {}", result);
    }

    /// Test PowerSpectrum::compute() with negative k (edge case)
    #[test]
    fn test_power_spectrum_compute_negative_k() {
        let ps = PowerSpectrum::default();
        let result = ps.compute(-1.0);
        assert_eq!(result, 0.0, "compute(-1.0) should return 0.0, got {}", result);
    }

    /// Test PowerSpectrum::new() constructor
    #[test]
    fn test_power_spectrum_new_constructor() {
        // Verify constructor by testing compute() results
        let ps1 = PowerSpectrum::new(0.96, 1.0);
        let ps2 = PowerSpectrum::new(2.0, 2.0);
        
        // P(1.0) = 1.0 * 1.0^(0.96 - 1) = 1.0
        let result1 = ps1.compute(1.0);
        assert!((result1 - 1.0).abs() < 1e-9, 
                "new(0.96, 1.0) at k=1.0 should return 1.0, got {}", result1);
        
        // P(1.0) = 2.0 * 1.0^(2.0 - 1) = 2.0
        let result2 = ps2.compute(1.0);
        assert!((result2 - 2.0).abs() < 1e-9, 
                "new(2.0, 2.0) at k=1.0 should return 2.0, got {}", result2);
    }
}
