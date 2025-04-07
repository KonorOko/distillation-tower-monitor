use crate::calculations::types::CompositionResult;
use serde::Serialize;
use std::sync::Arc;

#[derive(Default, Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnEntry {
    pub timestamp: u64,
    pub temperatures: Vec<f64>,
    pub compositions: Vec<CompositionResult>,
    pub percentage_complete: f64,
    pub distilled_mass: f64,
}

#[derive(Debug, Clone)]
pub enum DataSource {
    Live,
    Playback {
        current_index: usize,
        data: Vec<Arc<ColumnEntry>>,
    },
    Temperatures {
        current_index: usize,
        data: Vec<Arc<ColumnEntry>>,
    },
}

impl DataSource {
    pub fn update_index(&mut self, index: usize) {
        match self {
            DataSource::Playback { current_index, .. } => *current_index = index,
            DataSource::Temperatures { current_index, .. } => *current_index = index,
            _ => {}
        }
    }
    pub fn get_current_entry(&self) -> Option<usize> {
        match self {
            DataSource::Playback { current_index, .. } => Some(*current_index),
            DataSource::Temperatures { current_index, .. } => Some(*current_index),
            _ => None,
        }
    }
}

pub struct ColumnStructure {
    pub number_plates: usize,
    pub has_compositions: bool,
    pub timestamp_column: usize,
    pub temperatures_start: usize,
    pub compositions_x_start: Option<usize>,
    pub compositions_y_start: Option<usize>,
}
