use serde::{Deserialize, Serialize};
use specta::Type;


#[derive(Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct CompositionResult {
    pub x_1: Option<f64>,
    pub y_1: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct CompositionConfig {
    pub x_0: Option<f64>,
    pub tol: Option<f64>,
    pub max_iter: Option<u64>,
}

impl Default for CompositionConfig {
    fn default() -> Self {
        Self {
            x_0: Some(0.5),
            tol: Some(1e-6),
            max_iter: Some(1000),
        }
    }
}

impl CompositionConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_x_0(mut self, x_0: f64) -> Self {
        self.x_0 = Some(x_0);
        self
    }
    
    pub fn with_tol(mut self, tol: f64) -> Self {
        self.tol = Some(tol);
        self
    }
    
    pub fn with_max_iter(mut self, max_iter: u64) -> Self {
        self.max_iter = Some(max_iter);
        self
    }
}
