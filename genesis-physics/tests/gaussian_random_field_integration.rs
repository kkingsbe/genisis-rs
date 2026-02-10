//! Integration test for GaussianRandomField
//!
//! This test verifies that the public API of GaussianRandomField works correctly
//! when imported through the genesis_physics crate.

use genesis_physics::GaussianRandomField;

#[test]
fn test_gaussian_random_field_integration() {
    // Test creating a field through the public API
    let field = GaussianRandomField::generate(128, 0.1, Some(42));
    
    // Verify dimensions
    assert_eq!(field.resolution, 128);
    assert_eq!(field.spacing, 0.1);
    assert_eq!(field.values.len(), 128);
    
    // Verify all values are finite and reasonable
    for z in 0..128 {
        assert_eq!(field.values[z].len(), 128);
        for y in 0..128 {
            assert_eq!(field.values[z][y].len(), 128);
            for x in 0..128 {
                let val = field.values[z][y][x];
                assert!(val.is_finite());
                // Gaussian distribution: 99.7% within ±3σ
                assert!(val > -10.0 && val < 10.0);
            }
        }
    }
    
    println!("Successfully generated 128³ Gaussian random field");
}

#[test]
fn test_reproducibility_with_seed() {
    // Same seed should produce identical results
    let field1 = GaussianRandomField::generate(32, 1.0, Some(12345));
    let field2 = GaussianRandomField::generate(32, 1.0, Some(12345));
    
    assert_eq!(field1.resolution, field2.resolution);
    assert_eq!(field1.spacing, field2.spacing);
    
    for z in 0..32 {
        for y in 0..32 {
            for x in 0..32 {
                assert_eq!(
                    field1.values[z][y][x],
                    field2.values[z][y][x],
                    "Values differ at ({}, {}, {})",
                    x, y, z
                );
            }
        }
    }
    
    println!("Reproducibility test passed - same seed produces identical results");
}

#[test]
fn test_different_seeds_produce_different_results() {
    // Different seeds should produce different results
    let field1 = GaussianRandomField::generate(16, 1.0, Some(1));
    let field2 = GaussianRandomField::generate(16, 1.0, Some(2));
    
    // Find at least one differing value
    let mut found_difference = false;
    for z in 0..16 {
        for y in 0..16 {
            for x in 0..16 {
                if field1.values[z][y][x] != field2.values[z][y][x] {
                    found_difference = true;
                    break;
                }
            }
            if found_difference {
                break;
            }
        }
        if found_difference {
            break;
        }
    }
    
    assert!(found_difference, "Different seeds should produce different results");
    
    println!("Different seeds produce different results - verified");
}
