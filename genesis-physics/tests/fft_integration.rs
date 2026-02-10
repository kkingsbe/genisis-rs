//! Integration tests for FFT density field transformations

use genesis_physics::perturbations::fft::DensityFft;
use genesis_physics::perturbations::PowerSpectrum;
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

#[test]
fn test_fft_construction() {
    let size = 16;
    let fft = DensityFft::new(size);
    assert_eq!(fft.size(), size);
}

#[test]
fn test_real_to_kspace_roundtrip() {
    let size = 16;
    let mut fft = DensityFft::new(size);

    // Create a simple test field (all ones)
    let original: Vec<f64> = vec![1.0; size * size * size];

    // Transform to k-space and back
    let kspace = fft.real_to_kspace(&original);
    let reconstructed = fft.kspace_to_real(kspace);

    // Verify round-trip preserves values (within numerical precision)
    for (orig, recon) in original.iter().zip(reconstructed.iter()) {
        assert!((orig - recon).abs() < 1e-10,
                "Round-trip failed: {} vs {}", orig, recon);
    }
}

#[test]
fn test_sine_wave_transform() {
    let size = 32;
    let mut fft = DensityFft::new(size);

    // Create a simple sine wave along x-direction
    let mut field = vec![0.0; size * size * size];
    for z in 0..size {
        for y in 0..size {
            for x in 0..size {
                let idx = z * size * size + y * size + x;
                field[idx] = (2.0 * PI * x as f64 / size as f64).sin();
            }
        }
    }

    // Transform to k-space
    let kspace = fft.real_to_kspace(&field);

    // Verify the DC component is near zero
    let dc_component = kspace[0].norm();
    assert!(dc_component < 1e-10,
            "DC component should be ~0 for sine wave, got {}", dc_component);
}

#[test]
fn test_dc_component() {
    let size = 16;
    let mut fft = DensityFft::new(size);

    // Create a constant field
    let constant_value = 5.0;
    let field: Vec<f64> = vec![constant_value; size * size * size];

    // Transform to k-space
    let kspace = fft.real_to_kspace(&field);

    // The DC component (index 0) should contain all the energy
    let dc_component = kspace[0];
    let expected_magnitude = (size * size * size) as f64 * constant_value;

    assert!((dc_component.norm() - expected_magnitude).abs() < 1e-8,
            "DC component magnitude: expected {}, got {}",
            expected_magnitude, dc_component.norm());
}

#[test]
fn test_empty_field() {
    let size = 16;
    let mut fft = DensityFft::new(size);

    // Create an empty field
    let field: Vec<f64> = vec![0.0; size * size * size];

    // Transform to k-space and back
    let kspace = fft.real_to_kspace(&field);
    let reconstructed = fft.kspace_to_real(kspace);

    // Verify everything is still zero
    for &val in &reconstructed {
        assert!(val.abs() < 1e-15, "Empty field should transform to zero, got {}", val);
    }
}

#[test]
fn test_power_spectrum_conservation() {
    let size = 16;
    let mut fft = DensityFft::new(size);

    // Create a random field
    let field: Vec<f64> = (0..size * size * size)
        .map(|_| rand::random::<f64>() * 2.0 - 1.0)
        .collect();

    // Calculate power in real space
    let real_power: f64 = field.iter().map(|&x| x * x).sum();

    // Transform to k-space
    let kspace = fft.real_to_kspace(&field);

    // Calculate power in k-space (Parseval's theorem)
    let kspace_power: f64 = kspace.iter()
        .map(|c| c.norm_sqr())
        .sum::<f64>() / ((size * size * size) as f64);

    // Power should be conserved (within numerical precision)
    let relative_error = (real_power - kspace_power).abs() / real_power.max(1e-10);
    assert!(relative_error < 1e-8,
            "Power conservation violated: real={}, kspace={}, relative_error={}",
            real_power, kspace_power, relative_error);
}

// Tests for apply_power_spectrum() method

/// Test Case 1: Basic functionality and non-mutation
/// - Create a small DensityFft instance with grid size 8
/// - Create a PowerSpectrum with reasonable parameters
/// - Create a k-space data array with known values (all ones)
/// - Call apply_power_spectrum() with a test seed
/// - Verify the method returns Ok(())
/// - Verify the k-space data has been modified (values changed from original)
/// - Verify DC component (index 0) is set to zero
#[test]
fn test_apply_power_spectrum_basic_functionality() {
    let size = 8;
    let mut fft = DensityFft::new(size);

    // Create a PowerSpectrum with standard cosmological parameters
    let power_spectrum = PowerSpectrum::new(0.96, 2.1e-9);

    // Create a k-space data array with all ones
    let total_elements = size * size * size;
    let mut kspace: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];

    // Store a copy of the original data
    let original_kspace = kspace.clone();

    // Apply power spectrum
    let result = fft.apply_power_spectrum(&mut kspace, &power_spectrum, 42);

    // Verify the method returns Ok(())
    assert!(result.is_ok(), "apply_power_spectrum should return Ok(())");

    // Verify the k-space data has been modified (at least one value changed)
    let mut modified = false;
    for (orig, new) in original_kspace.iter().zip(kspace.iter()) {
        if (orig - new).norm() > 1e-10 {
            modified = true;
            break;
        }
    }
    assert!(modified, "k-space data should have been modified");

    // Verify DC component (index 0) is set to zero
    assert!(
        kspace[0].norm() < 1e-15,
        "DC component should be zero, got {:?}",
        kspace[0]
    );
}

/// Test Case 2: Reproducibility with same seed
/// - Create two identical DensityFft instances and k-space arrays
/// - Apply power spectrum to both with the same seed
/// - Verify both resulting k-space arrays are identical (within floating-point tolerance)
#[test]
fn test_apply_power_spectrum_reproducibility_same_seed() {
    let size = 8;
    let mut fft1 = DensityFft::new(size);
    let mut fft2 = DensityFft::new(size);

    // Create a PowerSpectrum
    let power_spectrum = PowerSpectrum::new(0.96, 1.0);

    // Create two identical k-space data arrays
    let total_elements = size * size * size;
    let seed = 12345;

    let mut kspace1: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];
    let mut kspace2: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];

    // Apply power spectrum to both with the same seed
    let result1 = fft1.apply_power_spectrum(&mut kspace1, &power_spectrum, seed);
    let result2 = fft2.apply_power_spectrum(&mut kspace2, &power_spectrum, seed);

    // Both should succeed
    assert!(result1.is_ok(), "First apply_power_spectrum should succeed");
    assert!(result2.is_ok(), "Second apply_power_spectrum should succeed");

    // Verify both resulting k-space arrays are identical within floating-point tolerance
    for (idx, (val1, val2)) in kspace1.iter().zip(kspace2.iter()).enumerate() {
        let diff = (val1 - val2).norm();
        assert!(
            diff < 1e-10,
            "k-space values at index {} should be identical: {:?} vs {:?}",
            idx, val1, val2
        );
    }
}

/// Test Case 3: Different seeds produce different results
/// - Create two identical DensityFft instances and k-space arrays
/// - Apply power spectrum to each with different seeds
/// - Verify the resulting k-space arrays are different
#[test]
fn test_apply_power_spectrum_different_seeds_different_results() {
    let size = 8;
    let mut fft1 = DensityFft::new(size);
    let mut fft2 = DensityFft::new(size);

    // Create a PowerSpectrum
    let power_spectrum = PowerSpectrum::new(0.96, 1.0);

    // Create two identical k-space data arrays
    let total_elements = size * size * size;
    let seed1 = 11111;
    let seed2 = 22222;

    let mut kspace1: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];
    let mut kspace2: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];

    // Apply power spectrum with different seeds
    let result1 = fft1.apply_power_spectrum(&mut kspace1, &power_spectrum, seed1);
    let result2 = fft2.apply_power_spectrum(&mut kspace2, &power_spectrum, seed2);

    // Both should succeed
    assert!(result1.is_ok(), "First apply_power_spectrum should succeed");
    assert!(result2.is_ok(), "Second apply_power_spectrum should succeed");

    // Verify the resulting k-space arrays are different (at least some values should differ)
    let mut different = false;
    for (val1, val2) in kspace1.iter().zip(kspace2.iter()) {
        let diff = (val1 - val2).norm();
        if diff > 1e-10 {
            different = true;
            break;
        }
    }
    assert!(different, "Different seeds should produce different results");
}

/// Test Case 4: Power spectrum scaling behavior
/// - Create a DensityFft and k-space array
/// - Apply power spectrum twice with the same seed but different power spectrum parameters
/// - Verify higher amplitude produces larger variance in results
#[test]
fn test_apply_power_spectrum_scaling_behavior() {
    let size = 8;
    let mut fft1 = DensityFft::new(size);
    let mut fft2 = DensityFft::new(size);

    // Create two power spectra with different amplitudes
    let power_spectrum_low = PowerSpectrum::new(0.96, 1.0);
    let power_spectrum_high = PowerSpectrum::new(0.96, 10.0);

    // Create k-space data arrays
    let total_elements = size * size * size;
    let seed = 42;

    let mut kspace1: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];
    let mut kspace2: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];

    // Apply power spectrum with same seed but different amplitudes
    let result1 = fft1.apply_power_spectrum(&mut kspace1, &power_spectrum_low, seed);
    let result2 = fft2.apply_power_spectrum(&mut kspace2, &power_spectrum_high, seed);

    assert!(result1.is_ok(), "apply_power_spectrum with low amplitude should succeed");
    assert!(result2.is_ok(), "apply_power_spectrum with high amplitude should succeed");

    // Calculate variance for each result
    let mean1: f64 = kspace1.iter().map(|c| c.norm()).sum::<f64>() / total_elements as f64;
    let variance1: f64 = kspace1.iter()
        .map(|c| (c.norm() - mean1).powi(2))
        .sum::<f64>() / total_elements as f64;

    let mean2: f64 = kspace2.iter().map(|c| c.norm()).sum::<f64>() / total_elements as f64;
    let variance2: f64 = kspace2.iter()
        .map(|c| (c.norm() - mean2).powi(2))
        .sum::<f64>() / total_elements as f64;

    // Higher amplitude should produce larger variance
    assert!(
        variance2 > variance1,
        "Higher amplitude should produce larger variance: variance_low={}, variance_high={}",
        variance1, variance2
    );

    // Also check max magnitude (should be larger for higher amplitude)
    let max1: f64 = kspace1.iter().map(|c| c.norm()).fold(0.0_f64, |a, b| a.max(b));
    let max2: f64 = kspace2.iter().map(|c| c.norm()).fold(0.0_f64, |a, b| a.max(b));

    assert!(
        max2 > max1,
        "Higher amplitude should produce larger max magnitude: max_low={}, max_high={}",
        max1, max2
    );
}

/// Test Case 5: Field size validation
/// - Create k-space array with wrong size (not Nx*Ny*Nz)
/// - Verify method returns an error
/// - Verify error message indicates size mismatch
#[test]
fn test_apply_power_spectrum_field_size_validation() {
    let size = 8;
    let mut fft = DensityFft::new(size);

    // Create a PowerSpectrum
    let power_spectrum = PowerSpectrum::new(0.96, 1.0);

    // Create k-space data with wrong size (too small)
    let mut kspace_wrong_size: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 100];

    // Apply power spectrum - should return an error
    let result = fft.apply_power_spectrum(&mut kspace_wrong_size, &power_spectrum, 42);

    // Verify an error is returned
    assert!(result.is_err(), "apply_power_spectrum should return an error for wrong field size");

    // Verify error message indicates size mismatch
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("size mismatch") || error_msg.contains("Field size"),
        "Error message should indicate size mismatch, got: {}",
        error_msg
    );

    // Also test with wrong size (too large)
    let total_elements = size * size * size;
    let mut kspace_too_large: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements + 100];

    let result2 = fft.apply_power_spectrum(&mut kspace_too_large, &power_spectrum, 42);
    assert!(result2.is_err(), "apply_power_spectrum should return an error for oversized field");

    let error_msg2 = result2.unwrap_err();
    assert!(
        error_msg2.contains("size mismatch") || error_msg2.contains("Field size"),
        "Error message should indicate size mismatch, got: {}",
        error_msg2
    );
}

/// Test Case 6: All frequencies are finite
/// - Create a DensityFft and apply power spectrum with test seed
/// - Verify all resulting complex values are finite (no NaN or infinity)
#[test]
fn test_apply_power_spectrum_all_frequencies_finite() {
    let size = 16;
    let mut fft = DensityFft::new(size);

    // Create a PowerSpectrum
    let power_spectrum = PowerSpectrum::new(0.96, 1.0);

    // Create k-space data array
    let total_elements = size * size * size;
    let mut kspace: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); total_elements];

    // Apply power spectrum
    let result = fft.apply_power_spectrum(&mut kspace, &power_spectrum, 42);
    assert!(result.is_ok(), "apply_power_spectrum should succeed");

    // Verify all resulting complex values are finite (no NaN or infinity)
    for (idx, val) in kspace.iter().enumerate() {
        assert!(
            val.is_finite(),
            "k-space value at index {} should be finite, got re={}, im={}",
            idx, val.re, val.im
        );
        assert!(
            !val.re.is_nan(),
            "k-space real part at index {} should not be NaN",
            idx
        );
        assert!(
            !val.im.is_nan(),
            "k-space imaginary part at index {} should not be NaN",
            idx
        );
        assert!(
            !val.re.is_infinite(),
            "k-space real part at index {} should not be infinite",
            idx
        );
        assert!(
            !val.im.is_infinite(),
            "k-space imaginary part at index {} should not be infinite",
            idx
        );
    }
}
