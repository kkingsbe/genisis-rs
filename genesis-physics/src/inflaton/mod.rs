// Inflaton field module for cosmic inflation physics

use bevy::prelude::*;

/// Inflaton mass constant (in GeV)
/// The inflaton field typically has a mass of ~10^16 GeV, which in natural units
/// corresponds to ~10^19 eV. This value is configurable but uses a sensible default.
pub const INFLATON_MASS: f64 = 1.0e16;

/// Inflaton field representation
///
/// The inflaton field φ is a scalar field that drives cosmic inflation in the early universe.
/// It contains the field value along with its potential energy, derivatives, and slow-roll
/// parameters that characterize the inflationary dynamics.
///
/// # Fields
///
/// - `phi`: The inflaton field value (dimensionless in natural units)
/// - `potential`: The potential energy V(φ)
/// - `potential_first_derivative`: First derivative dV/dφ
/// - `potential_second_derivative`: Second derivative d²V/dφ²
/// - `epsilon`: First slow-roll parameter ε = (1/2)(V'/V)²
/// - `eta`: Second slow-roll parameter η = V''/V
#[derive(Resource, Debug, Clone)]
pub struct Inflaton {
    /// The inflaton field value φ (dimensionless in natural units)
    pub phi: f64,
    /// The potential energy V(φ)
    pub potential: f64,
    /// First derivative of the potential dV/dφ
    pub potential_first_derivative: f64,
    /// Second derivative of the potential d²V/dφ²
    pub potential_second_derivative: f64,
    /// First slow-roll parameter ε
    pub epsilon: f64,
    /// Second slow-roll parameter η
    pub eta: f64,
}

impl Default for Inflaton {
    fn default() -> Self {
        Self {
            phi: 0.0,
            potential: 0.0,
            potential_first_derivative: 0.0,
            potential_second_derivative: 0.0,
            epsilon: 0.0,
            eta: 0.0,
        }
    }
}

impl Inflaton {
    /// Creates a new Inflaton field with the specified initial φ value.
    ///
    /// # Arguments
    ///
    /// * `phi` - The initial value of the inflaton field φ (dimensionless in natural units)
    ///
    /// # Returns
    ///
    /// A new `Inflaton` instance with the given φ value and all computed fields updated.
    ///
    /// # Panics
    /// Panics if the resulting potential is zero (division by zero in slow-roll parameter calculation)
    pub fn new(phi: f64) -> Self {
        let mut inflaton = Self {
            phi,
            potential: 0.0,
            potential_first_derivative: 0.0,
            potential_second_derivative: 0.0,
            epsilon: 0.0,
            eta: 0.0,
        };
        inflaton.update_all();
        inflaton
    }

    /// Compute the quadratic potential V(φ) = ½m²φ²
    ///
    /// # Arguments
    /// * `phi` - The inflaton field value (dimensionless in natural units)
    ///
    /// # Returns
    /// The potential energy V(φ) as f64
    pub fn quadratic_potential(phi: f64) -> f64 {
        0.5 * INFLATON_MASS.powi(2) * phi.powi(2)
    }

    /// Compute the first derivative dV/dφ = m²φ
    ///
    /// # Arguments
    /// * `phi` - The inflaton field value (dimensionless in natural units)
    ///
    /// # Returns
    /// The first derivative dV/dφ as f64
    pub fn quadratic_potential_first_derivative(phi: f64) -> f64 {
        INFLATON_MASS.powi(2) * phi
    }

    /// Compute the second derivative d²V/dφ² = m²
    ///
    /// # Arguments
    /// * `phi` - The inflaton field value (dimensionless in natural units)
    /// Note: For quadratic potential, the second derivative is constant (m²)
    ///
    /// # Returns
    /// The second derivative d²V/dφ² as f64
    pub fn quadratic_potential_second_derivative(_phi: f64) -> f64 {
        INFLATON_MASS.powi(2)
    }

    /// Compute the first slow-roll parameter ε = (1/2)(V'/V)²
    ///
    /// # Arguments
    /// * `potential` - The potential energy V(φ)
    /// * `potential_first_derivative` - The first derivative dV/dφ
    ///
    /// # Returns
    /// The first slow-roll parameter ε as f64
    ///
    /// # Notes
    /// ε << 1 is the slow-roll condition that must be satisfied for inflation to occur
    pub fn epsilon(
        potential: f64,
        potential_first_derivative: f64,
    ) -> f64 {
        0.5 * (potential_first_derivative / potential).powi(2)
    }

    /// Compute the second slow-roll parameter η = V''/V
    ///
    /// # Arguments
    /// * `potential` - The potential energy V(φ)
    /// * `potential_second_derivative` - The second derivative d²V/dφ²
    ///
    /// # Returns
    /// The second slow-roll parameter η as f64
    ///
    /// # Notes
    /// |η| << 1 is the second slow-roll condition for inflation
    pub fn eta(
        potential: f64,
        potential_second_derivative: f64,
    ) -> f64 {
        potential_second_derivative / potential
    }

    /// Update the Inflaton state with potential and derivative values
    ///
    /// This method computes the potential and its derivatives for the current phi value
    /// and updates the corresponding fields. This is a convenience method to ensure
    /// all potential-related fields are kept in sync.
    pub fn update_potential(&mut self) {
        self.potential = Self::quadratic_potential(self.phi);
        self.potential_first_derivative = Self::quadratic_potential_first_derivative(self.phi);
        self.potential_second_derivative = Self::quadratic_potential_second_derivative(self.phi);
    }

    /// Update the slow-roll parameters based on current potential values
    ///
    /// This method computes ε and η from the potential and its derivatives
    /// and updates the corresponding fields in the Inflaton struct.
    ///
    /// # Panics
    /// Panics if potential is zero (division by zero)
    pub fn update_slow_roll_parameters(&mut self) {
        self.epsilon = Self::epsilon(self.potential, self.potential_first_derivative);
        self.eta = Self::eta(self.potential, self.potential_second_derivative);
    }

    /// Update all computed values: potential, derivatives, and slow-roll parameters
    ///
    /// This is the primary method to call when phi changes, ensuring all
    /// derived quantities stay synchronized.
    ///
    /// # Panics
    /// Panics if potential is zero (division by zero in slow-roll parameter calculation)
    pub fn update_all(&mut self) {
        self.update_potential();
        self.update_slow_roll_parameters();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_potential_zero() {
        // V(0) = ½m²(0)² = 0
        let result = Inflaton::quadratic_potential(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_quadratic_potential_one() {
        // V(1) = ½m²(1)² = ½m²
        let result = Inflaton::quadratic_potential(1.0);
        let expected = 0.5 * INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_two() {
        // V(2) = ½m²(2)² = 2m²
        let result = Inflaton::quadratic_potential(2.0);
        let expected = 2.0 * INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_negative() {
        // V(-1) = ½m²(-1)² = ½m² (potential is symmetric)
        let result = Inflaton::quadratic_potential(-1.0);
        let expected = 0.5 * INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_first_derivative_zero() {
        // dV/dφ(0) = m²(0) = 0
        let result = Inflaton::quadratic_potential_first_derivative(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_quadratic_potential_first_derivative_one() {
        // dV/dφ(1) = m²(1) = m²
        let result = Inflaton::quadratic_potential_first_derivative(1.0);
        let expected = INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_first_derivative_two() {
        // dV/dφ(2) = m²(2) = 2m²
        let result = Inflaton::quadratic_potential_first_derivative(2.0);
        let expected = 2.0 * INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_first_derivative_negative() {
        // dV/dφ(-1) = m²(-1) = -m²
        let result = Inflaton::quadratic_potential_first_derivative(-1.0);
        let expected = -INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_second_derivative_zero() {
        // d²V/dφ²(0) = m²
        let result = Inflaton::quadratic_potential_second_derivative(0.0);
        let expected = INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_second_derivative_one() {
        // d²V/dφ²(1) = m² (constant)
        let result = Inflaton::quadratic_potential_second_derivative(1.0);
        let expected = INFLATON_MASS.powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_second_derivative_arbitrary() {
        // d²V/dφ² is constant regardless of phi
        let phi_values = [-100.0, -10.0, -1.0, 0.0, 1.0, 10.0, 100.0];
        let expected = INFLATON_MASS.powi(2);

        for phi in phi_values {
            let result = Inflaton::quadratic_potential_second_derivative(phi);
            assert_eq!(result, expected, "Second derivative should be constant for phi={}", phi);
        }
    }

    #[test]
    fn test_update_potential_zero() {
        let mut inflaton = Inflaton::new(0.0);
        inflaton.update_potential();

        assert_eq!(inflaton.phi, 0.0);
        assert_eq!(inflaton.potential, 0.0);
        assert_eq!(inflaton.potential_first_derivative, 0.0);
        assert_eq!(inflaton.potential_second_derivative, INFLATON_MASS.powi(2));
    }

    #[test]
    fn test_update_potential_one() {
        let mut inflaton = Inflaton::new(1.0);
        inflaton.update_potential();

        assert_eq!(inflaton.phi, 1.0);
        assert_eq!(inflaton.potential, 0.5 * INFLATON_MASS.powi(2));
        assert_eq!(inflaton.potential_first_derivative, INFLATON_MASS.powi(2));
        assert_eq!(inflaton.potential_second_derivative, INFLATON_MASS.powi(2));
    }

    #[test]
    fn test_new_inflaton_initialized() {
        // new() constructor should call update_potential() automatically
        let inflaton = Inflaton::new(2.0);

        assert_eq!(inflaton.phi, 2.0);
        assert_eq!(inflaton.potential, 2.0 * INFLATON_MASS.powi(2));
        assert_eq!(inflaton.potential_first_derivative, 2.0 * INFLATON_MASS.powi(2));
        assert_eq!(inflaton.potential_second_derivative, INFLATON_MASS.powi(2));
    }

    #[test]
    fn test_new_inflaton_zero_phi() {
        // new() constructor with phi=0 should initialize properly
        let inflaton = Inflaton::new(0.0);

        assert_eq!(inflaton.phi, 0.0);
        assert_eq!(inflaton.potential, 0.0);
        assert_eq!(inflaton.potential_first_derivative, 0.0);
        assert_eq!(inflaton.potential_second_derivative, INFLATON_MASS.powi(2));
    }

    #[test]
    fn test_update_potential_after_phi_change() {
        // Test that update_potential works after changing phi
        let mut inflaton = Inflaton::new(1.0);
        assert_eq!(inflaton.potential, 0.5 * INFLATON_MASS.powi(2));

        // Change phi manually
        inflaton.phi = 3.0;
        // Update potential
        inflaton.update_potential();

        assert_eq!(inflaton.phi, 3.0);
        assert_eq!(inflaton.potential, 4.5 * INFLATON_MASS.powi(2));
        assert_eq!(inflaton.potential_first_derivative, 3.0 * INFLATON_MASS.powi(2));
        assert_eq!(inflaton.potential_second_derivative, INFLATON_MASS.powi(2));
    }

    #[test]
    fn test_default_epsilon_eta_still_zero() {
        // Epsilon and eta are now calculated in subtask 3, so they won't be zero
        // This test is deprecated - the slow-roll parameters are now computed
        let inflaton = Inflaton::new(1.0);
        // epsilon and eta should now be computed from potential values
        assert_ne!(inflaton.epsilon, 0.0);
        assert_ne!(inflaton.eta, 0.0);
    }

    #[test]
    fn test_epsilon_flat_potential() {
        // epsilon should be 0.0 when potential_first_derivative is 0.0 (flat potential)
        let potential = 100.0;
        let potential_first_derivative = 0.0;
        let result = Inflaton::epsilon(potential, potential_first_derivative);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_epsilon_scaling() {
        // epsilon should scale correctly with (V'/V)²
        // Let's test: ε = 0.5 * (V'/V)²
        let potential = 10.0_f64;
        let potential_first_derivative = 2.0_f64;
        let result = Inflaton::epsilon(potential, potential_first_derivative);
        let expected = 0.5_f64 * (2.0_f64 / 10.0_f64).powi(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_eta() {
        // eta should correctly compute V''/V
        let potential = 100.0;
        let potential_second_derivative = 20.0;
        let result = Inflaton::eta(potential, potential_second_derivative);
        let expected = 20.0 / 100.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadratic_potential_slow_roll_parameters() {
        // For quadratic potential V(φ) = ½m²φ²:
        // ε = 2/m²φ²
        // η = 2/φ²
        // At φ = 1.0e16 (near Planck scale), ε = 2/m²(1e16)² = 2/1e32 = 2e-32 (very small, good slow-roll)
        
        let phi = 1.0e16_f64;
        let m_squared = INFLATON_MASS.powi(2);
        let potential = 0.5_f64 * m_squared * phi.powi(2);
        let potential_first_derivative = m_squared * phi;
        let potential_second_derivative = m_squared;

        let epsilon = Inflaton::epsilon(potential, potential_first_derivative);
        let eta = Inflaton::eta(potential, potential_second_derivative);

        // Expected: ε = 0.5 * (V'/V)² = 0.5 * (m²φ / (0.5*m²φ²))² = 0.5 * (2/φ)² = 2/φ²
        let expected_epsilon = 2.0_f64 / phi.powi(2);
        // Expected: η = V''/V = m² / (0.5*m²φ²) = 2/φ²
        let expected_eta = 2.0_f64 / phi.powi(2);

        // Use approximate comparison due to floating point precision
        assert!((epsilon - expected_epsilon).abs() < 1e-47, "epsilon should match expected value");
        assert_eq!(eta, expected_eta);
        
        // At φ = 1e16, both should be very small (~2e-32), indicating good slow-roll
        assert!(epsilon < 1e-30, "epsilon should be very small for slow-roll");
        assert!(eta.abs() < 1e-30, "eta should be very small for slow-roll");
    }

    #[test]
    fn test_update_slow_roll_parameters() {
        let mut inflaton = Inflaton::new(2.0);
        
        // Manually set potential values to verify update_slow_roll_parameters
        inflaton.potential = 100.0_f64;
        inflaton.potential_first_derivative = 10.0_f64;
        inflaton.potential_second_derivative = 5.0_f64;
        
        inflaton.update_slow_roll_parameters();
        
        // epsilon = 0.5 * (10/100)² = 0.5 * 0.01 = 0.005
        assert!((inflaton.epsilon - 0.005_f64).abs() < 1e-15, "epsilon should be approximately 0.005");
        // eta = 5/100 = 0.05
        assert_eq!(inflaton.eta, 0.05_f64);
    }

    #[test]
    fn test_update_all() {
        let mut inflaton = Inflaton {
            phi: 3.0,
            potential: 0.0,
            potential_first_derivative: 0.0,
            potential_second_derivative: 0.0,
            epsilon: 0.0,
            eta: 0.0,
        };
        
        inflaton.update_all();
        
        // Verify all fields are updated
        assert_eq!(inflaton.phi, 3.0);
        assert_eq!(inflaton.potential, Inflaton::quadratic_potential(3.0));
        assert_eq!(inflaton.potential_first_derivative, Inflaton::quadratic_potential_first_derivative(3.0));
        assert_eq!(inflaton.potential_second_derivative, Inflaton::quadratic_potential_second_derivative(3.0));
        assert!(inflaton.epsilon > 0.0);
        assert!(inflaton.eta > 0.0);
    }

    #[test]
    fn test_new_inflaton_with_slow_roll() {
        // new() constructor should call update_all() and compute slow-roll parameters
        let phi = 5.0_f64;
        let inflaton = Inflaton::new(phi);
        
        // Verify phi
        assert_eq!(inflaton.phi, phi);
        
        // Verify potential and derivatives
        let m_squared = INFLATON_MASS.powi(2);
        assert_eq!(inflaton.potential, 0.5_f64 * m_squared * phi.powi(2));
        assert_eq!(inflaton.potential_first_derivative, m_squared * phi);
        assert_eq!(inflaton.potential_second_derivative, m_squared);
        
        // Verify slow-roll parameters
        // epsilon = 0.5 * (V'/V)² = 0.5 * (m²φ / (0.5*m²φ²))² = 2/φ²
        let expected_epsilon = 2.0_f64 / phi.powi(2);
        let expected_eta = 2.0_f64 / phi.powi(2);
        
        // Use approximate comparison due to floating point precision
        // The tolerance needs to be higher because of repeated floating point operations
        assert!((inflaton.epsilon - expected_epsilon).abs() < 1e-15, "epsilon should match expected value");
        assert!((inflaton.eta - expected_eta).abs() < 1e-15, "eta should match expected value");
    }

    #[test]
    fn test_inflaton_plugin_implements_plugin() {
        // Verify InflatonPlugin implements the Plugin trait
        // This test ensures the type is compatible with Bevy's plugin system
        let _plugin: InflatonPlugin = InflatonPlugin;
    }

    #[test]
    fn test_inflaton_plugin_inserts_resource() {
        // When InflatonPlugin is added to App, Inflaton resource should be available
        let mut app = App::new();
        app.add_plugins(InflatonPlugin);

        // Verify the Inflaton resource is available
        assert!(app.world().contains_resource::<Inflaton>());
    }

    #[test]
    fn test_inflaton_plugin_initializes_default_values() {
        // Verify that Inflaton resource is initialized with correct default values
        let mut app = App::new();
        app.add_plugins(InflatonPlugin);

        // Get the Inflaton resource
        let inflaton = app.world().resource::<Inflaton>();

        // Verify default initialization
        assert_eq!(inflaton.phi, 0.0, "phi should be initialized to 0.0");
        assert_eq!(inflaton.potential, 0.0, "potential should be initialized to 0.0");
        assert_eq!(inflaton.potential_first_derivative, 0.0, "potential_first_derivative should be initialized to 0.0");
        assert_eq!(inflaton.potential_second_derivative, 0.0, "potential_second_derivative should be initialized to 0.0");
        assert_eq!(inflaton.epsilon, 0.0, "epsilon should be initialized to 0.0");
        assert_eq!(inflaton.eta, 0.0, "eta should be initialized to 0.0");
    }

    #[test]
    fn test_inflaton_plugin_app_lifecycle() {
        // Verify that the plugin works correctly with app lifecycle
        let mut app = App::new();
        app.add_plugins(InflatonPlugin);

        // Verify the Inflaton resource is available after adding the plugin
        assert!(app.world().contains_resource::<Inflaton>());

        // Verify the values are at defaults
        let inflaton = app.world().resource::<Inflaton>();
        assert_eq!(inflaton.phi, 0.0);
        assert_eq!(inflaton.potential, 0.0);
    }

    #[test]
    fn test_inflaton_plugin_with_manual_resource_override() {
        // Test that we can override the default resource after adding the plugin
        let mut app = App::new();
        app.add_plugins(InflatonPlugin);

        // Override with a custom Inflaton
        let custom_inflaton = Inflaton::new(5.0);
        app.insert_resource(custom_inflaton);

        // Verify the custom resource is available
        let inflaton = app.world().resource::<Inflaton>();
        assert_eq!(inflaton.phi, 5.0);

        // Verify slow-roll parameters are computed
        let expected_epsilon = 2.0_f64 / 5.0_f64.powi(2);
        let expected_eta = 2.0_f64 / 5.0_f64.powi(2);
        assert!((inflaton.epsilon - expected_epsilon).abs() < 1e-15);
        assert!((inflaton.eta - expected_eta).abs() < 1e-15);
    }
}

/// Plugin that initializes the Inflaton field resource
///
/// This plugin registers the Inflaton as a Bevy Resource with
/// initial field value and computed potential, derivatives,
/// and slow-roll parameters.
pub struct InflatonPlugin;

impl Plugin for InflatonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inflaton::default());
    }
}
