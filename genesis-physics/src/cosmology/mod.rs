//! Cosmological physics implementation for cosmic evolution
//!
//! Implements the Friedmann equations governing cosmic expansion:
//! H² = (8πG/3)ρ - k/a²
//!
//! Where:
//! - H = Hubble parameter (ȧ/a)
//! - G = gravitational constant
//! - ρ = energy density
//! - k = curvature parameter
//! - a = scale factor
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use genesis_physics::cosmology::{Cosmology, EnergyDensity, Curvature};
//!
//! fn update_cosmology(mut cosmology: ResMut<Cosmology>) {
//!     // Set energy density for inflation epoch
//!     cosmology.energy_density = EnergyDensity::inflaton_dominated(1e64);
//!
//!     // Update Hubble parameter using Friedmann equation
//!     cosmology.update_hubble();
//!
//!     // Integrate scale factor forward
//!     cosmology.integrate_scale_factor_euler(1e-35);
//! }
//! ```
//!
//! # Mathematical Foundation
//!
//! The Friedmann equation describes how the Hubble parameter (expansion rate)
//! depends on the energy density of the universe:
//!
//! - **H** = Hubble parameter = ȧ/a (rate of expansion)
//! - **G** = Gravitational constant
//! - **ρ** = Energy density
//! - **k** = Curvature parameter (-1, 0, +1)
//! - **a** = Scale factor
//!
//! In natural units (ℏ = c = 1), the equation becomes:
//! H² = (8π/3) * M_pl² * ρ - k/a²
//!
//! where M_pl is the reduced Planck mass ~ 2.435 × 10¹⁸ GeV.

use bevy::prelude::*;
use bevy::time::Time;

use crate::integrator::{rk4_step};
use genesis_core::time::{TimeAccumulator, INFLATION_START_YEARS, INFLATION_END_YEARS, SECONDS_PER_YEAR};

/// Cosmological constants used in Friedmann equation calculations
pub mod constants {
    /// Gravitational constant in SI units: G ≈ 6.674 × 10⁻¹¹ m³/kg/s²
    pub const G: f64 = 6.674e-11;
    
    /// Speed of light in SI units: c = 299,792,458 m/s
    pub const C: f64 = 299_792_458.0;
    
    /// Planck mass in kg: mₚ = √(ħc/G) ≈ 2.176 × 10⁻⁸ kg
    pub const PLANCK_MASS: f64 = 2.176e-8;
    
    /// Planck length in meters: ℓₚ = √(ħG/c³) ≈ 1.616 × 10⁻³⁵ m
    pub const PLANCK_LENGTH: f64 = 1.616e-35;
    
    /// Planck time in seconds: tₚ = ℓₚ/c ≈ 5.391 × 10⁻⁴⁴ s
    pub const PLANCK_TIME: f64 = 5.391e-44;
    
    /// Inflationary Hubble parameter in GeV: H ≈ 10¹⁴ GeV
    /// Converted to SI: 1 GeV ≈ 1.602 × 10⁻¹⁰ J, so H ≈ 1.6 × 10⁴ J
    pub const INFLATION_HUBBLE_GEV: f64 = 1e14;
    
    /// 1 GeV in Joules
    pub const GEV_TO_JOULES: f64 = 1.602e-10;
}

/// Curvature parameter of the universe
/// 
/// -1: Open universe (hyperbolic geometry)
///  0: Flat universe (Euclidean geometry) - matches CMB observations
///  1: Closed universe (spherical geometry)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum Curvature {
    Open = -1,
    Flat = 0,
    Closed = 1,
}

impl Curvature {
    pub fn to_f64(self) -> f64 {
        self as i32 as f64
    }
}

/// Energy density of the universe
/// 
/// Energy density can come from different components:
/// - Matter density (baryonic + dark matter)
/// - Radiation density (photons, neutrinos)
/// - Dark energy density (cosmological constant)
/// - Inflaton field energy density (during inflation)
#[derive(Debug, Clone, Copy, Resource)]
pub struct EnergyDensity {
    /// Total energy density in GeV⁴ (natural units)
    pub total: f64,
    
    /// Matter density in GeV⁴
    pub matter: f64,
    
    /// Radiation density in GeV⁴
    pub radiation: f64,
    
    /// Dark energy density in GeV⁴
    pub dark_energy: f64,
    
    /// Inflaton field energy density in GeV⁴
    pub inflaton: f64,
}

impl Default for EnergyDensity {
    fn default() -> Self {
        Self {
            total: 0.0,
            matter: 0.0,
            radiation: 0.0,
            dark_energy: 0.0,
            inflaton: 0.0,
        }
    }
}

impl EnergyDensity {
    /// Create energy density with only matter component
    pub fn matter_dominated(matter_density: f64) -> Self {
        Self {
            total: matter_density,
            matter: matter_density,
            ..Default::default()
        }
    }
    
    /// Create energy density with only radiation component
    pub fn radiation_dominated(radiation_density: f64) -> Self {
        Self {
            total: radiation_density,
            radiation: radiation_density,
            ..Default::default()
        }
    }
    
    /// Create energy density with only inflaton component (inflation epoch)
    pub fn inflaton_dominated(inflaton_density: f64) -> Self {
        Self {
            total: inflaton_density,
            inflaton: inflaton_density,
            ..Default::default()
        }
    }
}

/// Represents a cosmic epoch in the evolution of the universe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum CosmicEpoch {
    /// Planck boundary epoch: t < 10⁻³²s (quantum gravity dominated)
    Planck,
    /// Inflation epoch: 10⁻³⁶s – 10⁻³²s (exponential metric expansion)
    Inflation,
    /// Quark-Gluon Plasma: 10⁻³²s – 10⁻⁶s (QGP cooling, confinement)
    QuarkGluonPlasma,
    /// Big Bang Nucleosynthesis: 3 min – 20 min (light element formation)
    Nucleosynthesis,
    /// Recombination: ~380,000 yr (electron capture, CMB release)
    Recombination,
    /// Dark Ages: 380 Kyr – 100 Myr (gravitational collapse)
    DarkAges,
    /// Cosmic Dawn: 100 Myr – 1 Gyr (first stars, reionization)
    CosmicDawn,
    /// Structure Formation: 1 Gyr – 13.8 Gyr (galaxy assembly, cosmic web)
    Structure,
}

/// Scale factor a(t) of the universe
/// 
/// The scale factor describes how the size of the universe changes over time.
/// - a(t) = 1 at the present day (convention)
/// - a(t) → 0 as t → 0 (Big Bang)
/// - During inflation: a(t) = a₀e^(Ht) (exponential expansion)
#[derive(Debug, Clone, Copy, Resource)]
pub struct ScaleFactor {
    /// Current scale factor value
    pub value: f64,
    
    /// Time derivative of scale factor: ȧ = d(a)/dt
    pub derivative: f64,
    
    /// Cosmic time in seconds
    pub time: f64,
    
    /// Current cosmic epoch
    pub epoch: CosmicEpoch,
}

impl Default for ScaleFactor {
    fn default() -> Self {
        Self {
            value: 1.0,
            derivative: 0.0,
            time: 0.0,
            epoch: CosmicEpoch::Planck,
        }
    }
}

/// Compute exponential scale factor during inflation: a(t) = a₀e^(Ht)
///
/// # Arguments
/// * `a0` - Initial scale factor
/// * `t_elapsed` - Elapsed time during inflation (in GeV⁻¹)
/// * `h` - Hubble parameter H during inflation (in GeV)
///
/// # Returns
/// Scale factor value at time t
///
/// # Example
/// ```
/// use genesis_physics::cosmology::compute_exponential_scale_factor;
/// let a = compute_exponential_scale_factor(1.0, 1e-35, 1e14);
/// // a ≈ 1.0 + small expansion
/// ```
pub fn compute_exponential_scale_factor(a0: f64, t_elapsed: f64, h: f64) -> f64 {
    a0 * (h * t_elapsed).exp()
}

/// Hubble parameter H(t) of the universe
/// 
/// The Hubble parameter describes the rate of cosmic expansion:
/// H = ȧ/a
/// 
/// It is related to the age and composition of the universe through the Friedmann equation.
#[derive(Debug, Clone, Copy, Resource)]
pub struct HubbleParameter {
    /// Current Hubble parameter value H = ȧ/a in GeV
    pub value: f64,
    
    /// Hubble parameter squared H² in GeV²
    pub squared: f64,
}

impl Default for HubbleParameter {
    fn default() -> Self {
        Self {
            value: 0.0,
            squared: 0.0,
        }
    }
}

/// Main cosmology resource combining all Friedmann equation parameters
/// 
/// This resource contains all the state needed to compute the Friedmann equation
/// and track cosmic expansion.
#[derive(Debug, Clone, Resource)]
pub struct Cosmology {
    /// Scale factor of the universe
    pub scale_factor: ScaleFactor,
    
    /// Hubble parameter of the universe
    pub hubble: HubbleParameter,
    
    /// Energy density components
    pub energy_density: EnergyDensity,
    
    /// Curvature parameter (k = -1, 0, +1)
    pub curvature: Curvature,
}

impl Default for Cosmology {
    fn default() -> Self {
        Self {
            scale_factor: ScaleFactor::default(),
            hubble: HubbleParameter::default(),
            energy_density: EnergyDensity::default(),
            curvature: Curvature::Flat,
        }
    }
}

impl Cosmology {
    /// Create a new cosmology state with flat curvature
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new cosmology state with specified curvature
    pub fn with_curvature(curvature: Curvature) -> Self {
        Self {
            curvature,
            ..Default::default()
        }
    }

    /// Compute the Hubble parameter H using the Friedmann equation
    ///
    /// Friedmann equation: H² = (8πG/3)ρ - k/a²
    ///
    /// In natural units (ℏ = c = 1), this becomes:
    /// H² = (8π/3) * M_pl² * ρ - k/a²
    /// where M_pl is the reduced Planck mass ~ 2.435 × 10¹⁸ GeV
    ///
    /// # Arguments
    /// * `energy_density` - Total energy density ρ in GeV⁴
    /// * `scale_factor` - Scale factor a
    /// * `curvature` - Curvature parameter k (-1, 0, +1)
    ///
    /// # Returns
    /// The Hubble parameter H in GeV
    ///
    /// # Notes
    /// Uses natural units where ℏ = c = 1. The Planck mass M_pl appears
    /// because G = 1/M_pl² in natural units.
    pub fn compute_hubble(
        energy_density: f64,
        scale_factor: f64,
        curvature: Curvature,
    ) -> f64 {
        // Planck mass squared in GeV²: (2.435 × 10^18)^2 = 5.929225 × 10^36
        const M_PL_SQUARED: f64 = 5.929225e36;

        // (8π/3) * M_pl² factor
        const PREFACTOR: f64 = (8.0 * std::f64::consts::PI / 3.0) * M_PL_SQUARED;

        // Curvature term: k/a²
        let curvature_term = curvature.to_f64() / (scale_factor * scale_factor);

        // Friedmann equation: H² = (8π/3)M_pl²ρ - k/a²
        let h_squared = PREFACTOR * energy_density - curvature_term;

        // Ensure H² is non-negative (physical requirement)
        let h_squared = h_squared.max(0.0);

        // H = sqrt(H²)
        h_squared.sqrt()
    }

    /// Compute the time derivative of the scale factor: ȧ = H * a
    ///
    /// # Arguments
    /// * `hubble` - Hubble parameter H in GeV
    /// * `scale_factor` - Scale factor a
    ///
    /// # Returns
    /// The time derivative ȧ in GeV units
    ///
    /// # Notes
    /// In natural units, time has units of GeV⁻¹, so ȧ has units of GeV
    pub fn compute_scale_factor_derivative(hubble: f64, scale_factor: f64) -> f64 {
        hubble * scale_factor
    }

    /// Update the Hubble parameter based on current cosmological state
    ///
    /// This function updates the HubbleParameter resource in-place using
    /// the Friedmann equation with the current energy density, scale factor,
    /// and curvature.
    pub fn update_hubble(&mut self) {
        let h = Self::compute_hubble(
            self.energy_density.total,
            self.scale_factor.value,
            self.curvature,
        );

        self.hubble.value = h;
        self.hubble.squared = h * h;
    }

    /// Update the scale factor derivative based on current Hubble parameter
    ///
    /// This function updates the ScaleFactor resource's derivative field using
    /// the relationship ȧ = H * a
    pub fn update_scale_factor_derivative(&mut self) {
        self.scale_factor.derivative = Self::compute_scale_factor_derivative(
            self.hubble.value,
            self.scale_factor.value,
        );
    }

    /// Perform a single integration step for the scale factor using Euler method
    ///
    /// This advances the scale factor forward in time by dt using:
    /// a(t + dt) = a(t) + ȧ * dt
    ///
    /// # Arguments
    /// * `dt` - Time step in GeV⁻¹ (natural units)
    ///
    /// # Notes
    /// This is a simple Euler integration step. For higher accuracy, use
    /// the RK4 solver (to be implemented in a separate subtask).
    pub fn integrate_scale_factor_euler(&mut self, dt: f64) {
        // Update derivative first
        self.update_scale_factor_derivative();

        // Euler step: a(t + dt) = a(t) + ȧ * dt
        self.scale_factor.value += self.scale_factor.derivative * dt;

        // Update time
        self.scale_factor.time += dt;

        // Recompute Hubble for the new state
        self.update_hubble();
    }

    /// Perform a single integration step for the scale factor using RK4 method
    ///
    /// This advances the scale factor forward in time by dt using the
    /// 4th-order Runge-Kutta method, which provides higher accuracy than
    /// the simple Euler method.
    ///
    /// The differential equations being solved are:
    /// - da/dt = H * a
    /// - dȧ/dt = H * ȧ (ignoring the a*d(H)/dt term for simplicity)
    ///
    /// The state vector is [a, ȧ] where:
    /// - a = scale_factor.value (the scale factor)
    /// - ȧ = scale_factor.derivative (the time derivative of scale factor)
    ///
    /// # Arguments
    /// * `dt` - Time step in GeV⁻¹ (natural units)
    ///
    /// # Notes
    /// The RK4 method uses four slope estimates to achieve 4th-order accuracy:
    /// - k1 = f(t, y)
    /// - k2 = f(t + dt/2, y + k1*dt/2)
    /// - k3 = f(t + dt/2, y + k2*dt/2)
    /// - k4 = f(t + dt, y + k3*dt)
    ///
    /// The derivative calculation for dȧ/dt simplifies to H*ȧ, which ignores
    /// the a*d(H)/dt term that would require computing time derivatives of
    /// density parameters. This is a reasonable approximation when the energy
    /// density changes slowly compared to the expansion timescale.
    pub fn integrate_scale_factor_rk4(&mut self, dt: f64) {
        // Update derivative first to ensure ȧ is consistent with current H
        self.update_scale_factor_derivative();

        // Capture current H for use in the derivative function
        // During the RK4 step, we assume H is constant (i.e., we ignore d(H)/dt)
        let h = self.hubble.value;

        // Define the derivative function for the state vector [a, ȧ]
        // Returns [da/dt, dȧ/dt] = [ȧ, H*ȧ]
        let derivative = |_t: f64, state: &[f64]| -> Vec<f64> {
            let _a = state[0];      // scale factor value (not used in simplified form)
            let a_dot = state[1];  // scale factor derivative

            // da/dt = ȧ
            let da_dt = a_dot;

            // dȧ/dt = d(H*a)/dt = H*d(a)/dt + a*d(H)/dt
            // For simplicity, we assume H is constant during the step, so d(H)/dt ≈ 0
            // Therefore: dȧ/dt ≈ H*ȧ
            let da_dot_dt = h * a_dot;

            vec![da_dt, da_dot_dt]
        };

        // Current state vector: [a, ȧ]
        let y = &[self.scale_factor.value, self.scale_factor.derivative];
        let t = self.scale_factor.time;

        // Perform RK4 step
        let y_new = rk4_step(y, t, dt, derivative);

        // Update scale factor from the new state
        self.scale_factor.value = y_new[0];
        self.scale_factor.derivative = y_new[1];

        // Update time
        self.scale_factor.time += dt;

        // Recompute Hubble for the new state
        self.update_hubble();
    }

    /// Update scale factor using exponential expansion during inflation epoch
    ///
    /// This method applies a(t) = a₀e^(Ht) where H is constant during inflation.
    /// During inflation, the Hubble parameter is approximately constant at
    /// H ≈ 10¹⁴ GeV.
    ///
    /// # Arguments
    /// * `dt` - Time step in GeV⁻¹
    ///
    /// # Side Effects
    /// - Updates `self.scale_factor.value` to the new exponential value
    /// - Updates `self.scale_factor.time` by adding `dt`
    /// - Updates `self.scale_factor.derivative` to H*a (consistent with ȧ = H*a for exponential expansion)
    pub fn integrate_scale_factor_inflation(&mut self, dt: f64) {
        let a0 = self.scale_factor.value;
        let h = constants::INFLATION_HUBBLE_GEV;
        let new_a = compute_exponential_scale_factor(a0, dt, h);

        // Update scale factor state
        self.scale_factor.value = new_a;
        self.scale_factor.time += dt;

        // For exponential expansion: ȧ = H * a
        self.scale_factor.derivative = h * new_a;
    }
}

/// Converts years to GeV⁻¹ (natural time units).
///
/// # Arguments
/// * `years` - Time in years
///
/// # Returns
/// Equivalent time in GeV⁻¹
///
/// # Formula
/// In natural units, 1 GeV⁻¹ = ħ / 1 GeV ≈ 6.582 × 10⁻²⁵ s
/// So 1 year = 31,557,600 s / (6.582 × 10⁻²⁵ s/GeV⁻¹) ≈ 4.79 × 10³¹ GeV⁻¹
fn years_to_gev_inv(years: f64) -> f64 {
    const H_BAR: f64 = 6.582e-25; // ħ in GeV·s
    years * SECONDS_PER_YEAR / H_BAR
}

/// System that updates scale factor using the appropriate expansion law
/// based on the current cosmic epoch.
///
/// # Epoch Behavior
/// - **Inflation (1e-44 to 1e-32 years)**: Exponential expansion a(t) = a₀e^(Ht)
/// - **Post-inflation (> 1e-32 years)**: RK4 integration using Friedmann equation
///
/// # Arguments
/// * `cosmology` - Mutable reference to cosmology state
/// * `time_accumulator` - Time tracking resource (cosmic time in years)
/// * `time` - Bevy's time resource for delta time calculation
#[allow(dead_code)]
pub fn update_scale_factor_by_epoch(
    mut cosmology: ResMut<Cosmology>,
    time_accumulator: Res<TimeAccumulator>,
    time: Res<Time>,
) {
    // Get delta time in years
    let delta_seconds = time.delta_secs() as f64;
    let delta_years = delta_seconds * time_accumulator.acceleration / SECONDS_PER_YEAR;

    // Convert delta time from years to GeV⁻¹ (natural units)
    let dt_gev_inv = years_to_gev_inv(delta_years);

    // Determine the current cosmic epoch and use the appropriate integration method
    if time_accumulator.years < INFLATION_END_YEARS {
        // During inflation: use exponential expansion
        cosmology.integrate_scale_factor_inflation(dt_gev_inv);
    } else {
        // After inflation: use RK4 integration
        cosmology.integrate_scale_factor_rk4(dt_gev_inv);
    }
}

pub struct CosmologyPlugin;

impl Plugin for CosmologyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cosmology>()
            .init_resource::<ScaleFactor>()
            .init_resource::<HubbleParameter>()
            .init_resource::<EnergyDensity>()
            .add_systems(PostUpdate, update_scale_factor_by_epoch);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_curvature_conversion() {
        assert_eq!(Curvature::Open.to_f64(), -1.0);
        assert_eq!(Curvature::Flat.to_f64(), 0.0);
        assert_eq!(Curvature::Closed.to_f64(), 1.0);
    }
    
    #[test]
    fn test_energy_density_default() {
        let density = EnergyDensity::default();
        assert_eq!(density.total, 0.0);
        assert_eq!(density.matter, 0.0);
        assert_eq!(density.radiation, 0.0);
        assert_eq!(density.dark_energy, 0.0);
        assert_eq!(density.inflaton, 0.0);
    }
    
    #[test]
    fn test_matter_dominated() {
        let density = EnergyDensity::matter_dominated(1.0);
        assert_eq!(density.total, 1.0);
        assert_eq!(density.matter, 1.0);
        assert_eq!(density.radiation, 0.0);
        assert_eq!(density.dark_energy, 0.0);
        assert_eq!(density.inflaton, 0.0);
    }
    
    #[test]
    fn test_radiation_dominated() {
        let density = EnergyDensity::radiation_dominated(1.0);
        assert_eq!(density.total, 1.0);
        assert_eq!(density.radiation, 1.0);
        assert_eq!(density.matter, 0.0);
        assert_eq!(density.dark_energy, 0.0);
        assert_eq!(density.inflaton, 0.0);
    }
    
    #[test]
    fn test_inflaton_dominated() {
        let density = EnergyDensity::inflaton_dominated(1.0);
        assert_eq!(density.total, 1.0);
        assert_eq!(density.inflaton, 1.0);
        assert_eq!(density.matter, 0.0);
        assert_eq!(density.radiation, 0.0);
        assert_eq!(density.dark_energy, 0.0);
    }
    
    #[test]
    fn test_scale_factor_default() {
        let a = ScaleFactor::default();
        assert_eq!(a.value, 1.0);
        assert_eq!(a.derivative, 0.0);
        assert_eq!(a.time, 0.0);
    }
    
    #[test]
    fn test_hubble_parameter_default() {
        let h = HubbleParameter::default();
        assert_eq!(h.value, 0.0);
        assert_eq!(h.squared, 0.0);
    }
    
    #[test]
    fn test_cosmology_default() {
        let c = Cosmology::default();
        assert_eq!(c.scale_factor.value, 1.0);
        assert_eq!(c.hubble.value, 0.0);
        assert_eq!(c.energy_density.total, 0.0);
        assert_eq!(c.curvature, Curvature::Flat);
    }
    
    #[test]
    fn test_cosmology_new() {
        let c = Cosmology::new();
        assert_eq!(c.curvature, Curvature::Flat);
    }
    
    #[test]
    fn test_cosmology_with_curvature() {
        let c = Cosmology::with_curvature(Curvature::Open);
        assert_eq!(c.curvature, Curvature::Open);
    }
    
    #[test]
    fn test_compute_hubble_flat_universe() {
        // For a flat universe (k=0) with high energy density (inflation era)
        let rho = 1e64; // GeV^4 - typical inflation energy density
        let a = 1.0;
        let curvature = Curvature::Flat;
        
        let h = Cosmology::compute_hubble(rho, a, curvature);
        
        // H should be very large for high energy density
        // H² = (8π/3) * M_pl² * ρ ≈ 4.96e37 * 1e64 = 4.96e101
        // H ≈ 7e50 GeV (above Planck scale as expected at very early times)
        assert!(h > 1e40, "Hubble parameter should be very large during early universe");
        
        // H should be positive
        assert!(h > 0.0, "Hubble parameter must be positive");
    }
    
    #[test]
    fn test_compute_hubble_open_universe() {
        // Open universe (k=-1) with moderate density
        let rho = 1e40; // GeV^4
        let a = 10.0;
        let curvature = Curvature::Open;
        
        let h = Cosmology::compute_hubble(rho, a, curvature);
        
        // H should be positive (energy density dominates at early times)
        assert!(h > 0.0, "Hubble parameter must be positive");
    }
    
    #[test]
    fn test_compute_hubble_closed_universe() {
        // Closed universe (k=+1) with moderate density
        let rho = 1e40; // GeV^4
        let a = 10.0;
        let curvature = Curvature::Closed;
        
        let h = Cosmology::compute_hubble(rho, a, curvature);
        
        // H should be positive (energy density dominates at early times)
        assert!(h > 0.0, "Hubble parameter must be positive");
    }
    
    #[test]
    fn test_compute_hubble_curvature_term() {
        // Test that curvature affects the result
        let rho = 1e40;
        let a = 10.0;
        
        let h_flat = Cosmology::compute_hubble(rho, a, Curvature::Flat);
        let h_open = Cosmology::compute_hubble(rho, a, Curvature::Open);
        let h_closed = Cosmology::compute_hubble(rho, a, Curvature::Closed);
        
        // For same density and scale factor:
        // - Open universe (negative curvature) should have larger H
        // - Closed universe (positive curvature) should have smaller H
        assert!(h_open >= h_flat, "Open universe should have H >= flat universe");
        assert!(h_closed <= h_flat, "Closed universe should have H <= flat universe");
    }
    
    #[test]
    fn test_compute_hubble_zero_density() {
        // Edge case: zero energy density with flat curvature
        let rho = 0.0;
        let a = 1.0;
        let curvature = Curvature::Flat;
        
        let h = Cosmology::compute_hubble(rho, a, curvature);
        
        // H should be zero for zero density in flat universe
        assert_eq!(h, 0.0, "Hubble should be zero for zero density");
    }
    
    #[test]
    fn test_compute_hubble_scale_factor_dependence() {
        // Test that H decreases as a increases (for fixed density)
        let rho = 1e40;
        let curvature = Curvature::Flat;
        
        let h_a1 = Cosmology::compute_hubble(rho, 1.0, curvature);
        let h_a10 = Cosmology::compute_hubble(rho, 10.0, curvature);
        
        // For fixed density, H is independent of a in flat universe
        // (in the simple Friedmann model without curvature term)
        assert_eq!(h_a1, h_a10, "H should be independent of scale factor in flat universe");
    }
    
    #[test]
    fn test_compute_scale_factor_derivative() {
        // Test ȧ = H * a
        let h = 1e14; // GeV - typical inflation Hubble
        let a = 1.0;
        
        let a_dot = Cosmology::compute_scale_factor_derivative(h, a);
        
        assert_eq!(a_dot, h, "For a=1, ȧ should equal H");
    }
    
    #[test]
    fn test_compute_scale_factor_derivative_with_a() {
        // Test ȧ = H * a with a != 1
        let h = 1e14; // GeV
        let a = 5.0;
        
        let a_dot = Cosmology::compute_scale_factor_derivative(h, a);
        
        assert_eq!(a_dot, h * a, "ȧ should equal H * a");
    }
    
    #[test]
    fn test_update_hubble() {
        let mut c = Cosmology::new();
        c.energy_density = EnergyDensity::inflaton_dominated(1e64);
        c.scale_factor.value = 1.0;
        c.curvature = Curvature::Flat;
        
        c.update_hubble();
        
        assert!(c.hubble.value > 0.0, "Hubble should be positive after update");
        assert_eq!(c.hubble.squared, c.hubble.value * c.hubble.value, 
                   "H² should equal H²");
    }
    
    #[test]
    fn test_update_scale_factor_derivative() {
        let mut c = Cosmology::new();
        c.hubble.value = 1e14;
        c.scale_factor.value = 1.0;
        
        c.update_scale_factor_derivative();
        
        assert_eq!(c.scale_factor.derivative, c.hubble.value * c.scale_factor.value,
                   "ȧ should equal H * a");
    }
    
    #[test]
    fn test_integrate_scale_factor_euler() {
        let mut c = Cosmology::new();
        c.energy_density = EnergyDensity::inflaton_dominated(1e64);
        c.scale_factor.value = 1.0;
        c.curvature = Curvature::Flat;
        
        // Initial state
        let initial_a = c.scale_factor.value;
        let initial_time = c.scale_factor.time;
        
        // Update H first
        c.update_hubble();
        
        // Integrate one step
        let dt = 1e-35; // Small time step
        c.integrate_scale_factor_euler(dt);
        
        // Scale factor should increase
        assert!(c.scale_factor.value > initial_a, 
                "Scale factor should increase during inflation");
        
        // Time should advance
        assert_eq!(c.scale_factor.time, initial_time + dt, 
                   "Time should advance by dt");
        
        // Derivative should be set
        assert!(c.scale_factor.derivative > 0.0, 
                "Scale factor derivative should be positive");
    }
    
    #[test]
    fn test_integrate_scale_factor_euler_multiple_steps() {
        let mut c = Cosmology::new();
        c.energy_density = EnergyDensity::inflaton_dominated(1e64);
        c.scale_factor.value = 1.0;
        c.curvature = Curvature::Flat;
        
        let dt = 1e-35;
        let steps = 10;
        
        let initial_a = c.scale_factor.value;
        
        // Integrate multiple steps
        for _ in 0..steps {
            c.integrate_scale_factor_euler(dt);
        }
        
        // Scale factor should have increased more than single step
        let mut single_step_c = Cosmology::new();
        single_step_c.energy_density = c.energy_density;
        single_step_c.scale_factor.value = initial_a;
        single_step_c.curvature = c.curvature;
        single_step_c.update_hubble();
        single_step_c.integrate_scale_factor_euler(dt);
        
        // Multiple steps should produce larger scale factor (approximately)
        // Note: this is approximate because H changes with a in general
        assert!(c.scale_factor.value > single_step_c.scale_factor.value,
                "Multiple integration steps should increase scale factor more");
    }
    
    #[test]
    fn test_cosmology_consistency() {
        // Test that the relationship H = ȧ/a holds after updates
        let mut c = Cosmology::new();
        c.energy_density = EnergyDensity::inflaton_dominated(1e64);
        c.scale_factor.value = 1.0;
        c.curvature = Curvature::Flat;
        
        // Update H and derivative
        c.update_hubble();
        c.update_scale_factor_derivative();
        
        // Check consistency: ȧ = H * a
        let expected_derivative = c.hubble.value * c.scale_factor.value;
        assert!((c.scale_factor.derivative - expected_derivative).abs() < 1e-10,
                "Scale factor derivative should equal H * a");
    }
    
    #[test]
    fn test_compute_hubble_negative_density() {
        // Edge case: negative energy density should be handled (clamped to 0)
        let rho = -1.0;
        let a = 1.0;
        let curvature = Curvature::Flat;
        
        let h = Cosmology::compute_hubble(rho, a, curvature);
        
        // H should be 0 for negative density (clamped by max(0.0, h_squared))
        assert_eq!(h, 0.0, "Hubble should be zero for negative density");
    }
    
    #[test]
    fn test_compute_hubble_very_small_scale_factor() {
        // Edge case: very small scale factor (early universe)
        let rho = 1e40;
        let a = 1e-10; // Very small scale factor
        let curvature = Curvature::Closed; // Positive curvature becomes significant
        
        let h = Cosmology::compute_hubble(rho, a, curvature);
        
        // H should be non-negative (may be reduced by large curvature term)
        assert!(h >= 0.0, "Hubble parameter should be non-negative");
    }
}

#[cfg(test)]
mod rk4_tests {
    use super::*;

    #[test]
    fn test_rk4_step_basic() {
        // Test that rk4_step produces a valid result
        // Simple test: dy/dt = y (exponential growth), y(0) = 1
        let y = vec![1.0f64];
        let t = 0.0;
        let dt = 0.1;
        
        let f = |_t: f64, y: &[f64]| -> Vec<f64> {
            vec![y[0]]
        };
        
        let y_new = rk4_step(&y, t, dt, f);
        
        // Result should be approximately e^0.1 ≈ 1.10517
        assert!((y_new[0] - 1.10517).abs() < 0.001);
    }

    #[test]
    fn test_rk4_step_multiple_variables() {
        // Test with 2D system: dy1/dt = y2, dy2/dt = -y1 (simple harmonic oscillator)
        let y = vec![1.0f64, 0.0f64]; // y1=1, y2=0
        let t = 0.0;
        let dt = 0.1;
        
        let f = |_t: f64, y: &[f64]| -> Vec<f64> {
            vec![y[1], -y[0]]
        };
        
        let y_new = rk4_step(&y, t, dt, f);
        
        // After small dt, should be approximately: y1 ≈ cos(dt), y2 ≈ -sin(dt)
        assert!((y_new[0] - 0.995004).abs() < 0.001); // cos(0.1)
        assert!((y_new[1] - (-0.0998334)).abs() < 0.001); // -sin(0.1)
    }

    #[test]
    fn test_integrate_scale_factor_rk4_increases_time() {
        let mut cosmology = Cosmology::default();
        let initial_time = cosmology.scale_factor.time;
        let dt = 1.0;
        
        cosmology.integrate_scale_factor_rk4(dt);
        
        assert_eq!(cosmology.scale_factor.time, initial_time + dt);
    }

    #[test]
    fn test_integrate_scale_factor_rk4_updates_hubble() {
        let mut cosmology = Cosmology::default();
        cosmology.energy_density = EnergyDensity::inflaton_dominated(1e64);
        cosmology.update_hubble();
        
        cosmology.integrate_scale_factor_rk4(0.1);
        
        // Hubble parameter should be updated (update_hubble() was called)
        assert!(cosmology.hubble.value > 0.0, "Hubble parameter should be positive after integration");
    }

    #[test]
    fn test_rk4_vs_euler_convergence() {
        // For small time steps, both RK4 and Euler should converge to similar results
        let mut cosmology_rk4 = Cosmology::default();
        let mut cosmology_euler = Cosmology::default();
        
        let dt = 0.001; // Small time step
        cosmology_rk4.integrate_scale_factor_rk4(dt);
        cosmology_euler.integrate_scale_factor_euler(dt);
        
        // Results should be close for small dt
        let diff = (cosmology_rk4.scale_factor.value - cosmology_euler.scale_factor.value).abs();
        assert!(diff < 0.01, "RK4 and Euler differ too much for small dt: {}", diff);
    }
}

#[cfg(test)]
mod exponential_tests {
    use super::*;

    #[test]
    fn test_compute_exponential_scale_factor_basic() {
        // Test that a(t) = a₀e^(Ht) produces expected values
        let a0 = 1.0;
        let h = 1e14;
        let t = 0.0;
        let result = compute_exponential_scale_factor(a0, t, h);
        // At t=0, a(t) = a₀
        assert!((result - a0).abs() < 1e-10);
    }

    #[test]
    fn test_compute_exponential_scale_factor_positive_time() {
        // Test that exponential growth works for positive time
        let a0 = 1.0;
        let h = 1.0; // Use H=1 for easier calculation
        let t = 1.0;
        let result = compute_exponential_scale_factor(a0, t, h);
        // a(t) = 1 * e^1 ≈ 2.718
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_exponential_derivative_consistency() {
        // Test that ȧ = H*a for exponential expansion
        let a0 = 1.0;
        let h = 1e14;
        // Use very small time step for accurate finite difference: h*t = 0.001
        let dt = 1e-17;

        // Compute a(t)
        let a1 = compute_exponential_scale_factor(a0, dt, h);

        // Compute derivative: ȧ = H*a (should equal (a1 - a0)/dt for small dt)
        let expected_derivative = h * a0;
        let computed_derivative = (a1 - a0) / dt;

        // Relative tolerance due to floating point arithmetic
        let relative_diff = (computed_derivative - expected_derivative).abs() / expected_derivative.abs();
        assert!(relative_diff < 1e-3, "Relative difference: {}", relative_diff);
    }

    #[test]
    fn test_exponential_growth_monotonic() {
        // Test that a(t) is monotonically increasing during inflation
        let a0 = 1.0;
        let h = 1e14;
        // Use larger time step to see meaningful changes: h*t = 0.1
        let dt = 1e-15;

        let a1 = compute_exponential_scale_factor(a0, dt, h);
        let a2 = compute_exponential_scale_factor(a1, dt, h);
        let a3 = compute_exponential_scale_factor(a2, dt, h);

        assert!(a1 > a0);
        assert!(a2 > a1);
        assert!(a3 > a2);
    }

    #[test]
    fn test_integrate_scale_factor_inflation_basic() {
        // Test the integrate_scale_factor_inflation method on Cosmology
        let mut c = Cosmology::new();
        c.scale_factor.value = 1.0;
        let initial_a = c.scale_factor.value;
        let initial_time = c.scale_factor.time;

        // Use larger time step to see meaningful change: h*t = 0.1
        let dt = 1e-15;
        c.integrate_scale_factor_inflation(dt);

        // Scale factor should increase
        assert!(c.scale_factor.value > initial_a,
                "Scale factor should increase during inflation");

        // Time should advance
        assert_eq!(c.scale_factor.time, initial_time + dt,
                   "Time should advance by dt");

        // Derivative should be H * a
        let expected_derivative = constants::INFLATION_HUBBLE_GEV * c.scale_factor.value;
        assert!((c.scale_factor.derivative - expected_derivative).abs() < 1e-10,
                "Derivative should equal H * a");
    }

    #[test]
    fn test_integrate_scale_factor_inflation_exponential() {
        // Test that integrate_scale_factor_inflation produces exponential growth
        let mut c = Cosmology::new();
        c.scale_factor.value = 1.0;
        let h = constants::INFLATION_HUBBLE_GEV;

        // Use larger time step to see meaningful change
        let dt = 1e-15;
        c.integrate_scale_factor_inflation(dt);

        // Check that the result matches compute_exponential_scale_factor
        let a0 = 1.0;
        let expected = compute_exponential_scale_factor(a0, dt, h);

        assert!((c.scale_factor.value - expected).abs() < 1e-10,
                "integrate_scale_factor_inflation should match compute_exponential_scale_factor");
    }

    #[test]
    fn test_integrate_scale_factor_inflation_multiple_steps() {
        // Test that multiple integration steps produce consistent exponential growth
        let mut c = Cosmology::new();
        c.scale_factor.value = 1.0;
        let h = constants::INFLATION_HUBBLE_GEV;

        // Use larger time step to see meaningful change
        let dt = 1e-15;
        let steps = 5;

        // Integrate multiple steps
        for _ in 0..steps {
            c.integrate_scale_factor_inflation(dt);
        }

        // Compute expected value: a = a0 * e^(H * total_time)
        let total_time = dt * steps as f64;
        let expected = compute_exponential_scale_factor(1.0, total_time, h);

        assert!((c.scale_factor.value - expected).abs() < 1e-10,
                "Multiple steps should produce exponential growth");
    }

    #[test]
    fn test_exponential_formula_consistency() {
        // Test that the formula a(t) = a₀e^(Ht) is consistently applied
        let a0_values = vec![0.5, 1.0, 2.0];
        let h = 1e14;
        let t = 1e-35;

        for a0 in a0_values {
            let result = compute_exponential_scale_factor(a0, t, h);
            let expected = a0 * (h * t).exp();
            assert!((result - expected).abs() < 1e-10,
                    "Exponential formula should be consistent for a0={}", a0);
        }
    }

    #[test]
    fn test_exponential_with_different_hubble() {
        // Test exponential growth with different Hubble parameters
        let a0 = 1.0;
        // Use time such that h*t = 0.1 for h=1e14
        let t = 1e-15;
        let h_values = vec![1e13, 1e14, 1e15];

        for h in h_values {
            let result = compute_exponential_scale_factor(a0, t, h);
            let expected = a0 * (h * t).exp();
            assert!((result - expected).abs() < 1e-10,
                    "Exponential formula should work for H={}", h);
            // Larger H should produce larger scale factor
            assert!(result > a0, "Scale factor should increase with positive H");
        }
    }
}
