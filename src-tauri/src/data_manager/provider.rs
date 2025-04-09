use crate::data_manager::types::ColumnEntry;
use crate::errors::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DataProvider {
    async fn get_next_entry(&mut self, number_plates: usize) -> Result<Arc<ColumnEntry>>;
    fn skip(&mut self, count: i64) -> Result<()>;
    fn reset(&mut self) -> Result<()>;
    fn get_current_index(&self) -> usize;
    async fn disconnect(&self) -> Result<()>;
    fn clone_provider(&self) -> Box<dyn DataProvider + Send>;
}
