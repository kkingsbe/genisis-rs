//! Integration tests for FFT density field transformations

use genesis_physics::perturbations::fft::DensityFft;
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
