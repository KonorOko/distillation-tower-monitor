use serde::{Deserialize, Serialize};

pub struct EquationParams {
    pub a_1: f64,
    pub b_1: f64,
    pub c_1: f64,
    pub a_van_1: f64,

    pub a_2: f64,
    pub b_2: f64,
    pub c_2: f64,
    pub a_van_2: f64,

    pub p: f64,
}

impl Default for EquationParams {
    fn default() -> Self {
        EquationParams {
            a_1: 8.12875,
            b_1: 1660.8713,
            c_1: 238.131,
            a_van_1: 1.6798,

            a_2: 8.05573,
            b_2: 1723.6425,
            c_2: 233.08,
            a_van_2: 0.9227,

            p: 585.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositionResult {
    pub x_1: Option<f64>,
    pub y_1: Option<f64>,
}
