use std::sync::Arc;

use rodbus::client::Channel;
use tokio::sync::Mutex;

use crate::{
    calculations::service::CalculationService,
    modbus::{client::ModbusClient, service::ModbusService},
};

use super::{
    live::LiveDataProvider, playback::PlaybackDataProvider, provider::DataProvider,
    types::ColumnEntry,
};

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn new() -> Self {
        Self
    }

    pub fn create_live_provider(
        &self,
        calculation_service: Arc<CalculationService>,
        modbus_channel: Arc<Mutex<Option<Channel>>>,
    ) -> Box<dyn DataProvider + Send> {
        let modbus_client = ModbusClient::new();
        let modbus_service = Arc::new(ModbusService::new(modbus_client));
        Box::new(LiveDataProvider::new(
            modbus_channel,
            calculation_service,
            modbus_service,
        ))
    }

    pub fn create_playback_provider(
        &self,
        data: Vec<Arc<ColumnEntry>>,
        index: usize,
    ) -> Box<dyn DataProvider + Send> {
        Box::new(PlaybackDataProvider::with_index(data, index))
    }
}
