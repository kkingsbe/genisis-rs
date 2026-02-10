//! Numerical integrators for differential equations
//!
//! This module provides generic numerical integration methods for solving
//! ordinary differential equations (ODEs) of the form dy/dt = f(t, y).
//!
//! The Runge-Kutta 4th order (RK4) method is a widely used numerical
//! integration technique that provides good accuracy for many scientific
//! and engineering applications.
//!
//! # Time Units
//!
//! All time units are in natural units (GeV⁻¹) for consistency with
//! the GENESIS cosmological simulation.

/// Generic 4th-order Runge-Kutta integrator
///
/// Performs a single RK4 integration step from time `t` to `t + dt`.
///
/// The RK4 algorithm computes four slope estimates and combines them
/// to achieve 4th-order accuracy:
///
/// ```text
/// k1 = f(t, y)
/// k2 = f(t + dt/2, y + k1*dt/2)
/// k3 = f(t + dt/2, y + k2*dt/2)
/// k4 = f(t + dt, y + k3*dt)
///
/// y(t + dt) = y(t) + (k1 + 2*k2 + 2*k3 + k4) * dt / 6
/// ```
///
/// # Arguments
/// * `y` - Current state vector
/// * `t` - Current time
/// * `dt` - Time step
/// * `f` - Derivative function dy/dt = f(t, y)
///
/// # Returns
/// New state vector after one RK4 step
///
/// # Type Parameters
/// * `F` - Function type implementing the derivative: Fn(f64, &[T]) -> Vec<T>
/// * `T` - State type that supports arithmetic operations with f64
///
/// # Example
///
/// ```rust
/// use genesis_physics::integrator::rk4_step;
///
/// // Simple harmonic oscillator: dy/dt = v, dv/dt = -omega^2 * y
/// let omega = 1.0;
/// let y = &[1.0, 0.0]; // [position, velocity]
/// let t = 0.0;
/// let dt = 0.1;
///
/// let derivative = |t: f64, state: &[f64]| -> Vec<f64> {
///     vec![state[1], -omega * omega * state[0]]
/// };
///
/// let y_new = rk4_step(y, t, dt, derivative);
/// ```
pub fn rk4_step<F, T>(
    y: &[T],
    t: f64,
    dt: f64,
    f: F,
) -> Vec<T>
where
    F: Fn(f64, &[T]) -> Vec<T>,
    T: Copy + std::ops::Add<Output = T> + std::ops::Mul<f64, Output = T> + std::ops::Div<f64, Output = T>,
{
    let n = y.len();

    // k1 = f(t, y)
    let k1 = f(t, y);

    // Compute y + k1 * dt/2 for k2 calculation
    let mut y_temp: Vec<T> = Vec::with_capacity(n);
    let half_dt = dt / 2.0;
    for i in 0..n {
        y_temp.push(y[i] + k1[i] * half_dt);
    }

    // k2 = f(t + dt/2, y + k1*dt/2)
    let k2 = f(t + half_dt, &y_temp);

    // Compute y + k2 * dt/2 for k3 calculation
    for i in 0..n {
        y_temp[i] = y[i] + k2[i] * half_dt;
    }

    // k3 = f(t + dt/2, y + k2*dt/2)
    let k3 = f(t + half_dt, &y_temp);

    // Compute y + k3 * dt for k4 calculation
    for i in 0..n {
        y_temp[i] = y[i] + k3[i] * dt;
    }

    // k4 = f(t + dt, y + k3*dt)
    let k4 = f(t + dt, &y_temp);

    // Combine all slopes: y_new = y + (k1 + 2*k2 + 2*k3 + k4) * dt / 6
    let mut y_new: Vec<T> = Vec::with_capacity(n);
    let sixth_dt = dt / 6.0;
    for i in 0..n {
        // (k1 + 2*k2 + 2*k3 + k4) * dt / 6
        y_new.push(y[i] + (k1[i] + k2[i] * 2.0 + k3[i] * 2.0 + k4[i]) * sixth_dt);
    }

    y_new
}

/// Integrate using RK4 from t0 to t1 with fixed time steps
///
/// Performs repeated RK4 steps from initial time `t0` to final time `t1`
/// with a fixed time step `dt`.
///
/// The total number of steps is calculated as `n_steps = ceil((t1 - t0) / dt)`,
/// and the actual time step used may be slightly smaller than `dt` to ensure
/// exactly reaching `t1`.
///
/// # Arguments
/// * `y0` - Initial state vector at time t0
/// * `t0` - Initial time
/// * `t1` - Final time
/// * `dt` - Nominal time step
/// * `f` - Derivative function dy/dt = f(t, y)
///
/// # Returns
/// Tuple containing:
/// - Final state vector at time t1
/// - Final time (should equal t1)
///
/// # Type Parameters
/// * `F` - Function type implementing the derivative: Fn(f64, &[T]) -> Vec<T>
/// * `T` - State type that supports arithmetic operations with f64
///
/// # Example
///
/// ```rust
/// use genesis_physics::integrator::rk4_integrate;
///
/// // Simple exponential decay: dy/dt = -lambda * y
/// let lambda = 1.0;
/// let y0 = &[1.0];
/// let t0 = 0.0;
/// let t1 = 1.0;
/// let dt = 0.1;
///
/// let derivative = |t: f64, state: &[f64]| -> Vec<f64> {
///     vec![-lambda * state[0]]
/// };
///
/// let (y_final, t_final) = rk4_integrate(y0, t0, t1, dt, derivative);
/// ```
pub fn rk4_integrate<F, T>(
    y0: &[T],
    t0: f64,
    t1: f64,
    dt: f64,
    f: F,
) -> (Vec<T>, f64)
where
    F: Fn(f64, &[T]) -> Vec<T> + Clone,
    T: Copy + std::ops::Add<Output = T> + std::ops::Mul<f64, Output = T> + std::ops::Div<f64, Output = T>,
{
    // Calculate the actual time step to exactly reach t1
    let total_time = t1 - t0;
    
    // Handle edge cases
    if dt <= 0.0 {
        // Invalid dt, return initial state
        return (y0.to_vec(), t0);
    }
    
    if total_time <= 0.0 {
        // No time to advance
        return (y0.to_vec(), t0);
    }
    
    // Calculate number of steps
    let n_steps = (total_time / dt).ceil() as usize;
    if n_steps == 0 {
        return (y0.to_vec(), t0);
    }
    
    // Adjust dt to exactly reach t1
    let actual_dt = total_time / n_steps as f64;
    
    // Perform integration
    let mut y = y0.to_vec();
    let mut t = t0;
    
    for _ in 0..n_steps {
        y = rk4_step(&y, t, actual_dt, f.clone());
        t += actual_dt;
    }
    
    (y, t)
}
