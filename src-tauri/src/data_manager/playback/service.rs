use crate::data_manager::provider::DataProvider;
use crate::data_manager::types::ColumnEntry;
use crate::errors::{DataError, Result};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct PlaybackDataProvider {
    data: Vec<Arc<ColumnEntry>>,
    current_index: usize,
}

impl PlaybackDataProvider {
    pub fn new(data: Vec<Arc<ColumnEntry>>) -> Self {
        Self {
            data,
            current_index: 0,
        }
    }

    pub fn with_index(data: Vec<Arc<ColumnEntry>>, index: usize) -> Self {
        Self {
            data,
            current_index: index,
        }
    }
}

#[async_trait]
impl DataProvider for PlaybackDataProvider {
    async fn get_next_entry(
        &mut self,
        _number_plates: i32,
        _initial_mass: f32,
        _initial_concentration: f32,
    ) -> Result<Arc<ColumnEntry>> {
        if self.data.is_empty() {
            return Err(DataError::EmptyDataError.into());
        }

        if self.current_index >= self.data.len() {
            return Err(DataError::NoDataError.into());
        }

        let entry = self.data[self.current_index].clone();
        self.current_index += 1;
        Ok(entry)
    }

    fn skip(&mut self, count: i64) -> Result<()> {
        if self.data.is_empty() {
            return Err(DataError::EmptyDataError.into());
        }

        let new_index = if count.is_negative() {
            self.current_index
                .saturating_sub(count.unsigned_abs() as usize)
        } else {
            self.current_index
                .saturating_add(count as usize)
                .min(self.data.len() - 1)
        };

        self.current_index = new_index;
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        self.current_index = 0;
        Ok(())
    }

    fn get_current_index(&self) -> usize {
        self.current_index
    }

    async fn disconnect(&self) -> Result<()> {
        Ok(())
    }

    fn clone_provider(&self) -> Box<dyn DataProvider + Send> {
        Box::new(Self {
            data: self.data.clone(),
            current_index: self.current_index,
        })
    }
}
