use serde::{Deserialize, Serialize};
use specta::Type;
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub max_iter: Option<u32>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CalculationStep {
    pub timestamp: u32,
    pub step_index: u32,
    pub temperatures: Vec<f64>,
    pub compositions: Vec<CompositionResult>,
    pub delta_x: Option<f32>,
    pub f_0: Option<f32>,
    pub f_1: Option<f32>,
    pub partial_integral: Option<f32>,
    pub accumulated_integral: Option<f32>,
    pub remaining_mass: Option<f32>,
    pub distilled_mass: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CalculationHistory {
    pub id: String,
    pub initial_mass: f64,
    pub initial_concentration: f64,
    pub number_plates: u32,
    pub steps: Vec<CalculationStep>,
    pub start_time: u32,
    pub end_time: u32,
    pub file_size: Option<u32>,
    pub description: Option<String>,
}

impl CalculationHistory {
    pub fn new(
        start_time: Option<u32>,
        number_plates: usize,
        initial_mass: f64,
        initial_concentration: f64,
    ) -> Self {
        let timestamp = start_time.unwrap_or(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as u32,
        );

        Self {
            id: format!("distillation_history_{}", timestamp),
            initial_mass,
            initial_concentration,
            number_plates: number_plates as u32,
            steps: Vec::new(),
            start_time: timestamp,
            end_time: timestamp,
            file_size: Some(0),
            description: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct HistorySummary {
    pub id: String,
    pub initial_mass: f64,
    pub initial_concentration: f64,
    pub number_plates: u32,
    pub start_time: u32,
    pub end_time: u32,
    pub steps_count: u32,
    pub file_size: Option<u32>,
}
