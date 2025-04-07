use super::types::{CompositionResult, EquationParams};
use crate::errors::Result;
use crate::math::{integrate_trapezoidal, interpolate, newton_raphson, round};
use std::f64::consts::E;

#[derive(Debug)]
pub struct CalculationService {
    params: EquationParams,
}

impl CalculationService {
    pub fn new() -> Self {
        CalculationService {
            params: EquationParams::default(),
        }
    }

    pub fn calculate_composition(
        &self,
        x_0: Option<f64>,
        temp: f64,
        tol: Option<f64>,
        max_iter: Option<u64>,
    ) -> Result<CompositionResult> {
        let x_0 = x_0.unwrap_or(0.5);
        let tol = tol.unwrap_or(1e-6);
        let max_iter = max_iter.unwrap_or(1000);

        let params = &self.params;

        let residual_fn = move |x_1: f64| calculate_residual(x_1, temp, params);

        let x_1 = newton_raphson(residual_fn, x_0, tol, max_iter)?;
        let x_2 = 1.0 - x_1;
        let gamma_1 = calculate_gammas(params.a_van_1, params.a_van_2, x_1, x_2).0;
        let ps_1 = calculate_ps(temp, params.a_1, params.b_1, params.c_1);
        let k_1 = calculate_ks(gamma_1, ps_1, params.p);
        let y_1 = calculate_y(k_1, x_1);
        let result = CompositionResult {
            x_1: Some(round(x_1, 3)),
            y_1: Some(round(y_1, 3)),
        };

        Ok(result)
    }

    pub fn calculate_distilled_mass(&self, m_b0: f64, x_b0: f64, x_bf: f64, x_d: f64) -> f64 {
        let trap_num: usize = 1000;
        let f = |x_b: f64| 1.0 / (x_d - x_b);
        let integral_value = integrate_trapezoidal(f, x_b0, x_bf, trap_num);
        let mass_final = m_b0 * (integral_value).exp();

        mass_final
    }

    pub fn interpolate_temps(&self, num_plates: usize, t_1: f64, t_n: f64) -> Vec<f64> {
        if num_plates <= 2 {
            return vec![round(t_1, 3), round(t_n, 3)];
        }

        let mut interpolated_temps = Vec::with_capacity(num_plates);
        for i in 0..num_plates {
            let temp = interpolate(1.0, t_1, num_plates as f64, t_n, i as f64 + 1.0);
            interpolated_temps.push(round(temp, 3));
        }

        interpolated_temps
    }
}

fn calculate_residual(x_1: f64, temp: f64, params: &EquationParams) -> f64 {
    let x_2 = 1.0 - x_1;

    let (gamma_1, gamma_2) = calculate_gammas(params.a_van_1, params.a_van_2, x_1, x_2);

    let ps_1 = calculate_ps(temp, params.a_1, params.b_1, params.c_1);
    let ps_2 = calculate_ps(temp, params.a_2, params.b_2, params.c_2);

    let k_1 = calculate_ks(gamma_1, ps_1, params.p);
    let k_2 = calculate_ks(gamma_2, ps_2, params.p);

    let y_1 = calculate_y(k_1, x_1);
    let y_2 = calculate_y(k_2, x_2);

    return y_1 + y_2 - 1.0;
}

fn calculate_ps(temp: f64, a: f64, b: f64, c: f64) -> f64 {
    let log10_p: f64 = a - b / (c + temp);

    let p = 10.0f64.powf(log10_p);
    return p;
}

fn calculate_gammas(a_12: f64, a_21: f64, x_1: f64, x_2: f64) -> (f64, f64) {
    let denominator = a_12 * x_1 + a_21 * x_2;
    let gamma1 = E.powf(a_12 * (a_21 * x_2 / denominator).powf(2.0));
    let gamma2 = E.powf(a_21 * (a_12 * x_1 / denominator).powf(2.0));
    (gamma1, gamma2)
}

fn calculate_ks(gamma: f64, ps: f64, p: f64) -> f64 {
    return gamma * ps / p;
}

fn calculate_y(k: f64, x: f64) -> f64 {
    return k * x;
}
