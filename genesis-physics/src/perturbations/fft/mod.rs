//! Fast Fourier Transform utilities for density field analysis

use rustfft::{FftPlanner, num_complex::Complex};
use rand::Rng;
use rand::SeedableRng;
use crate::perturbations::PowerSpectrum;

/// FFT engine for transforming between real-space and k-space density fields
pub struct DensityFft {
    size: usize,
    planner: FftPlanner<f64>,
}

impl DensityFft {
    /// Create a new FFT engine for density fields of the given size
    pub fn new(size: usize) -> Self {
        let planner = FftPlanner::new();
        Self { size, planner }
    }

    /// Get the FFT size (number of grid cells per dimension)
    pub fn size(&self) -> usize {
        self.size
    }

    /// Transform a 3D real-space density field to k-space
    /// 
    /// # Arguments
    /// * `field` - Flat slice representing a 3D density field of size (size, size, size)
    /// 
    /// # Returns
    /// Vec of complex numbers representing the field in k-space
    /// 
    /// # Note
    /// This uses a naive approach by transforming along each dimension sequentially.
    /// For production use, consider a more efficient 3D FFT implementation.
    pub fn real_to_kspace(&mut self, field: &[f64]) -> Vec<Complex<f64>> {
        // Convert to complex numbers (imaginary part = 0)
        let mut buffer: Vec<Complex<f64>> = field.iter()
            .map(|&x| Complex::new(x, 0.0))
            .collect();
        
        // Get FFT plan for the size
        let fft = self.planner.plan_fft_forward(self.size);
        
        // Transform along x-dimension (strided access)
        for z in 0..self.size {
            for y in 0..self.size {
                let start = z * self.size * self.size + y * self.size;
                let row = &mut buffer[start..start + self.size];
                fft.process(row);
            }
        }
        
        // Transform along y-dimension
        for z in 0..self.size {
            let start = z * self.size * self.size;
            let slab = &mut buffer[start..start + self.size * self.size];
            for x in 0..self.size {
                let mut col: Vec<Complex<f64>> = (0..self.size)
                    .map(|y| slab[y * self.size + x])
                    .collect();
                fft.process(&mut col);
                for y in 0..self.size {
                    slab[y * self.size + x] = col[y];
                }
            }
        }
        
        // Transform along z-dimension
        for y in 0..self.size {
            for x in 0..self.size {
                let mut col: Vec<Complex<f64>> = (0..self.size)
                    .map(|z| buffer[z * self.size * self.size + y * self.size + x])
                    .collect();
                fft.process(&mut col);
                for z in 0..self.size {
                    buffer[z * self.size * self.size + y * self.size + x] = col[z];
                }
            }
        }
        
        buffer
    }
    
    /// Transform a k-space density field back to real-space (inverse FFT)
    /// 
    /// # Arguments
    /// * `field` - Vec of complex numbers representing the field in k-space
    /// 
    /// # Returns
    /// Vec of f64 values representing the field in real-space
    pub fn kspace_to_real(&mut self, field: Vec<Complex<f64>>) -> Vec<f64> {
        let mut buffer = field;
        
        // Get inverse FFT plan
        let ifft = self.planner.plan_fft_inverse(self.size);
        
        // Transform along z-dimension (inverse)
        for y in 0..self.size {
            for x in 0..self.size {
                let mut col: Vec<Complex<f64>> = (0..self.size)
                    .map(|z| buffer[z * self.size * self.size + y * self.size + x])
                    .collect();
                ifft.process(&mut col);
                for z in 0..self.size {
                    buffer[z * self.size * self.size + y * self.size + x] = col[z];
                }
            }
        }
        
        // Transform along y-dimension (inverse)
        for z in 0..self.size {
            let start = z * self.size * self.size;
            let slab = &mut buffer[start..start + self.size * self.size];
            for x in 0..self.size {
                let mut col: Vec<Complex<f64>> = (0..self.size)
                    .map(|y| slab[y * self.size + x])
                    .collect();
                ifft.process(&mut col);
                for y in 0..self.size {
                    slab[y * self.size + x] = col[y];
                }
            }
        }
        
        // Transform along x-dimension (inverse)
        for z in 0..self.size {
            for y in 0..self.size {
                let start = z * self.size * self.size + y * self.size;
                let row = &mut buffer[start..start + self.size];
                ifft.process(row);
            }
        }
        
        // Normalize and extract real parts
        let n = (self.size * self.size * self.size) as f64;
        buffer.into_iter()
            .map(|c| c.re / n)
            .collect()
    }

    /// Apply a power spectrum to k-space field data.
    ///
    /// This method multiplies each k-space frequency component by sqrt(P(k)) and a random phase,
    /// where P(k) is the power at wavenumber k. This is used to generate Gaussian random fields
    /// with specified power spectra.
    ///
    /// # Arguments
    /// * `field` - Mutable reference to k-space data (complex numbers) to be modified
    /// * `power_spectrum` - Reference to the power spectrum to apply
    /// * `seed` - Random seed for deterministic phase generation
    ///
    /// # Returns
    /// `Ok(())` on success, `Err(String)` on error
    ///
    /// # Notes
    /// - The DC component (k = 0) is typically set to zero to maintain zero mean
    /// - The random phase ensures the field is statistically isotropic
    /// - Uses deterministic random number generation for reproducibility
    pub fn apply_power_spectrum(
        &mut self,
        field: &mut Vec<Complex<f64>>,
        power_spectrum: &PowerSpectrum,
        seed: u64,
    ) -> Result<(), String> {
        let size = self.size;
        let total_elements = size * size * size;

        if field.len() != total_elements {
            return Err(format!(
                "Field size mismatch: expected {} elements ({}³), got {}",
                total_elements, size, field.len()
            ));
        }

        // Create deterministic random number generator
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        // Apply power spectrum to each k-space component
        for z in 0..size {
            for y in 0..size {
                for x in 0..size {
                    let idx = z * size * size + y * size + x;

                    // Calculate wave numbers (handle FFT frequency wrapping)
                    let kx = if x <= size / 2 {
                        x as f64
                    } else {
                        (x as f64) - (size as f64)
                    };
                    let ky = if y <= size / 2 {
                        y as f64
                    } else {
                        (y as f64) - (size as f64)
                    };
                    let kz = if z <= size / 2 {
                        z as f64
                    } else {
                        (z as f64) - (size as f64)
                    };

                    // Calculate total wave number k
                    let k = (kx * kx + ky * ky + kz * kz).sqrt();

                    // Skip DC component (k = 0) - set to zero for zero mean
                    if k > 0.0 {
                        // Compute power at this wavenumber
                        let p_k = power_spectrum.compute(k);

                        // Generate Gaussian random numbers using Box-Muller transform
                        let u1: f64 = rng.gen_range(0.0..1.0);
                        let u2: f64 = rng.gen_range(0.0..1.0);

                        // Avoid zero by ensuring values are in (0, 1)
                        let u1 = u1 * 0.9999 + 0.00005;
                        let u2 = u2 * 0.9999 + 0.00005;

                        // Apply Box-Muller transform to get Gaussian random numbers
                        let radius = (-2.0 * u1.ln()).sqrt();
                        let angle = 2.0 * std::f64::consts::PI * u2;

                        let z_real = radius * angle.cos();
                        let z_imag = radius * angle.sin();

                        // Generate complex random number with unit variance
                        let random_complex = Complex::new(z_real, z_imag);

                        // Multiply by sqrt(P(k))
                        let amplitude = p_k.sqrt();

                        // Apply to the field
                        field[idx] = field[idx] * random_complex * amplitude;
                    } else {
                        // Set DC component to zero for zero mean
                        field[idx] = Complex::new(0.0, 0.0);
                    }
                }
            }
        }

        Ok(())
    }
}
