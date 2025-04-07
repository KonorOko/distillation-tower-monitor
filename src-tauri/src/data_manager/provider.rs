use crate::data_manager::types::ColumnEntry;
use crate::errors::Result;
use std::sync::Arc;

pub trait DataProvider: Send + Sync {
    async fn get_next_entry(&mut self, number_plates: usize) -> Result<Arc<ColumnEntry>>;
    async fn skip(&mut self, count: i64) -> Result<()>;
    async fn reset(&mut self) -> Result<()>;
    fn get_current_index(&self) -> usize;
}
