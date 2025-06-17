use super::types::{CalculationStep, CompositionConfig, CompositionResult, EquationParams};
use crate::data_manager::types::ColumnEntry;
use crate::errors::Result;
use crate::math::{interpolate, newton_raphson, round};
use std::f64::consts::E;
use std::sync::Arc;

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
        temp: f64,
        config: Option<CompositionConfig>,
    ) -> Result<CompositionResult> {
        let config = config.unwrap_or_default();
        let x_0 = config.x_0.unwrap_or(0.5);
        let tol = config.tol.unwrap_or(1e-6);
        let max_iter = config.max_iter.unwrap_or(1000) as u64;

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

    pub fn calculate_distilled_mass(
        &self,
        w_0: f64,
        m_0: f64,
        column_entries: &[Arc<ColumnEntry>],
    ) -> Vec<CalculationStep> {
        if column_entries.is_empty() {
            return Vec::new();
        }

        let net = w_0 / 46.07;
        let nh2o = (1.0 - w_0) / 18.02;
        let x_0 = net / (net + nh2o);

        let mut steps = Vec::with_capacity(column_entries.len());
        let mut inte = 0.0;

        let calculate_f = |x_b: f64, x_d: f64| -> f64 {
            if (x_d - x_b).abs() < 1e-6 {
                return 0.0;
            }
            1.0 / (x_d - x_b)
        };

        for i in 0..column_entries.len() {
            let current = &column_entries[i];

            let mut delta_x = None;
            let mut f_0 = None;
            let mut f_1 = None;
            let mut partial_integral = None;

            let can_calculate = i < column_entries.len() - 1
                && !current.compositions.is_empty()
                && !column_entries[i + 1].compositions.is_empty();

            if can_calculate {
                let x_b0 = current.compositions.first().and_then(|c| c.x_1);
                let x_d0 = current.compositions.last().and_then(|c| c.y_1);
                let x_bf = column_entries[i + 1]
                    .compositions
                    .first()
                    .and_then(|c| c.x_1);
                let x_df = column_entries[i + 1]
                    .compositions
                    .last()
                    .and_then(|c| c.y_1);

                if let (Some(x_b0), Some(x_d0), Some(x_bf), Some(x_df)) = (x_b0, x_d0, x_bf, x_df) {
                    if x_b0 <= x_0 {
                        let dx = x_b0 - x_bf;
                        let f1 = calculate_f(x_b0, x_d0);
                        let f0 = calculate_f(x_bf, x_df);
                        let partial = 0.5 * (f0 + f1) * dx;

                        delta_x = Some(dx as f32);
                        f_0 = Some(f0 as f32);
                        f_1 = Some(f1 as f32);
                        partial_integral = Some(partial as f32);

                        inte += partial;
                    }
                }
            }

            let remaining_mass = (-inte).exp() * m_0;
            let distilled_mass = m_0 - remaining_mass;

            let step = CalculationStep {
                timestamp: current.timestamp as u32,
                step_index: i as u32,
                temperatures: current.temperatures.clone(),
                compositions: current.compositions.clone(),
                delta_x,
                f_0,
                f_1,
                partial_integral,
                accumulated_integral: Some(inte as f32),
                remaining_mass: Some(remaining_mass as f32),
                distilled_mass: Some(distilled_mass as f32),
            };

            steps.push(step);
        }

        steps
    }

    pub fn interpolate_temps(&self, num_plates: i32, t_1: f64, t_n: f64) -> Vec<f64> {
        if num_plates <= 2 {
            return vec![round(t_1, 3), round(t_n, 3)];
        }

        let mut interpolated_temps = Vec::with_capacity(num_plates as usize);
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
