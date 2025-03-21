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
}

#[derive(Debug, Clone)]
pub enum DataSource {
    Live,
    Playback {
        index: u64,
        data: Vec<Arc<ColumnEntry>>,
    },
    Temperatures {
        index: u64,
        data: Vec<Arc<ColumnEntry>>,
    },
}
