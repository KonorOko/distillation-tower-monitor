use crate::errors::{Error, Result, RootError};
pub fn newton_raphson<F>(f: F, x_0: f64, tol: f64, max_iter: u64) -> Result<f64>
where
    F: Fn(f64) -> f64,
{
    let mut x = x_0;
    let tol = tol;
    let max_iter = max_iter;

    for _ in 0..max_iter {
        let fx = f(x);
        let fx_prime = df(&f, x);

        if fx_prime.abs() < 1e-10 {
            return Err(Error::RootError(RootError::DivisionByZero));
        }

        let x_next = x - fx / fx_prime;

        if (x_next - x).abs() < tol {
            if x_next < 0.0 {
                return Err(Error::RootError(RootError::NegativeRootError));
            }
            return Ok(x_next);
        }

        x = x_next;
    }

    Err(Error::RootError(RootError::NotFoundedRoot))
}

pub fn df<F>(f: F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = 1e-6;
    (f(x + h) - f(x - h)) / (2.0 * h)
}

pub fn interpolate(x1: f64, y1: f64, x2: f64, y2: f64, x: f64) -> f64 {
    // Interpolate between two points using linear interpolation
    y1 + (y2 - y1) * (x - x1) / (x2 - x1)
}

pub fn round(value: f64, decimals: u32) -> f64 {
    // Round a floating-point number to a specified number of decimal places
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}
