use crate::calculations::types::CompositionResult;
use serde::Serialize;

#[derive(Default, Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnEntry {
    pub timestamp: u64,
    pub temperatures: Vec<f64>,
    pub compositions: Vec<CompositionResult>,
    pub percentage_complete: f64,
    pub distilled_mass: f64,
}

pub struct ColumnStructure {
    pub number_plates: usize,
    pub has_compositions: bool,
    pub timestamp_column: usize,
    pub temperatures_start: usize,
    pub compositions_x_start: Option<usize>,
    pub compositions_y_start: Option<usize>,
}
