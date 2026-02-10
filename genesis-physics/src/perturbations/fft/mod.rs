//! Fast Fourier Transform utilities for density field analysis

use rustfft::{FftPlanner, num_complex::Complex};

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
}
