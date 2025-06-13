use serde::{Deserialize, Serialize};
use specta::Type;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CalculationStep {
    pub timestamp: u32,
    pub step_index: usize,
    pub x_re: f32,
    pub x_do: f32,
    pub temp_re: f32,
    pub temp_do: f32,
    pub delta_x: f32,
    pub f_0: f32,
    pub f_1: f32,
    pub partial_integral: f32,
    pub accumulated_integral: f32,
    pub remaining_mass: f32,
    pub distilled_mass: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CalculationHistory {
    pub initial_mass: f32,
    pub initial_concentration: f32,
    pub steps: Vec<CalculationStep>,
    pub start_time: u32,
    pub end_time: u32,
}

impl CalculationHistory {
    pub fn new(initial_mass: f32, initial_concentration: f32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32;

        Self {
            initial_mass,
            initial_concentration,
            steps: Vec::new(),
            start_time: timestamp,
            end_time: timestamp,
        }
    }
}
