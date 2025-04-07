use super::playback::PlaybackDataProvider;
use super::types::{ColumnEntry, DataSource};
use crate::calculations::service::CalculationService;
use crate::data_manager::live::LiveDataProvider;
use crate::data_manager::provider::DataProvider;
use crate::errors::Result;
use crate::modbus::{client::ModbusClient, service::ModbusService};
use rodbus::client::Channel;
use std::sync::Arc;
use tokio::sync::Mutex;

pub trait DataProviderFactory: Send + Sync {
    async fn create_provider(&self, data_source: &DataSource) -> Result<DataProviderEnum>;
}

#[derive(Debug)]
pub enum DataProviderEnum {
    Playback(PlaybackDataProvider),
    Live(LiveDataProvider),
    Temperature(PlaybackDataProvider),
}

impl DataProviderEnum {
    pub async fn get_next_entry(&mut self, number_plates: usize) -> Result<Arc<ColumnEntry>> {
        match self {
            Self::Playback(provider) => provider.get_next_entry(number_plates).await,
            Self::Live(provider) => provider.get_next_entry(number_plates).await,
            Self::Temperature(provider) => provider.get_next_entry(number_plates).await,
        }
    }

    pub async fn skip(&mut self, count: i64) -> Result<()> {
        match self {
            Self::Playback(provider) => provider.skip(count).await,
            Self::Live(provider) => provider.skip(count).await,
            Self::Temperature(provider) => provider.skip(count).await,
        }
    }

    pub async fn reset(&mut self) -> Result<()> {
        match self {
            Self::Playback(provider) => provider.reset().await,
            Self::Live(provider) => provider.reset().await,
            Self::Temperature(provider) => provider.reset().await,
        }
    }

    pub fn get_current_index(&self) -> usize {
        match self {
            Self::Playback(provider) => provider.get_current_index(),
            Self::Live(provider) => provider.get_current_index(),
            Self::Temperature(provider) => provider.get_current_index(),
        }
    }
}

pub struct AppDataProviderFactory {
    calculation_service: Arc<CalculationService>,
    modbus_channel: Arc<Mutex<Option<Channel>>>,
}

impl AppDataProviderFactory {
    pub fn new(
        calculation_service: Arc<CalculationService>,
        modbus_channel: Arc<Mutex<Option<Channel>>>,
    ) -> Self {
        Self {
            calculation_service,
            modbus_channel,
        }
    }
}

impl DataProviderFactory for AppDataProviderFactory {
    async fn create_provider(&self, data_source: &DataSource) -> Result<DataProviderEnum> {
        match data_source {
            DataSource::Live => {
                let modbus_client = ModbusClient::new();
                let modbus_service = Arc::new(ModbusService::new(modbus_client));
                Ok(DataProviderEnum::Live(LiveDataProvider::new(
                    self.modbus_channel.clone(),
                    self.calculation_service.clone(),
                    modbus_service,
                )))
            }
            DataSource::Playback {
                current_index,
                data,
            } => Ok(DataProviderEnum::Playback(
                PlaybackDataProvider::with_index(data.clone(), *current_index as usize),
            )),
            DataSource::Temperatures {
                current_index,
                data,
            } => Ok(DataProviderEnum::Temperature(
                PlaybackDataProvider::with_index(data.clone(), *current_index as usize),
            )),
        }
    }
}

pub struct DataService {
    calculation_service: Arc<CalculationService>,
    modbus_channel: Arc<Mutex<Option<Channel>>>,
}

impl DataService {
    pub fn new(
        calculation_service: Arc<CalculationService>,
        modbus_channel: Arc<Mutex<Option<Channel>>>,
    ) -> Self {
        Self {
            calculation_service,
            modbus_channel,
        }
    }

    async fn create_provider(&self, data_source: &mut DataSource) -> Result<DataProviderEnum> {
        match data_source {
            DataSource::Live => {
                let modbus_client = ModbusClient::new();
                let modbus_service = Arc::new(ModbusService::new(modbus_client));

                Ok(DataProviderEnum::Live(LiveDataProvider::new(
                    self.modbus_channel.clone(),
                    self.calculation_service.clone(),
                    modbus_service,
                )))
            }
            DataSource::Playback {
                current_index,
                data,
            } => Ok(DataProviderEnum::Playback(
                PlaybackDataProvider::with_index(data.clone(), *current_index as usize),
            )),
            DataSource::Temperatures {
                current_index,
                data,
            } => Ok(DataProviderEnum::Temperature(
                PlaybackDataProvider::with_index(data.clone(), *current_index as usize),
            )),
        }
    }

    pub async fn get_next_entry(
        &self,
        data_source: &mut DataSource,
        number_plates: usize,
    ) -> Result<Arc<ColumnEntry>> {
        let mut provider = self.create_provider(data_source).await?;
        let entry = provider.get_next_entry(number_plates).await?;

        // Actualizar el índice en la fuente de datos
        match data_source {
            DataSource::Playback { current_index, .. } => {
                *current_index = provider.get_current_index()
            }
            DataSource::Temperatures { current_index, .. } => {
                *current_index = provider.get_current_index()
            }
            _ => {}
        }

        Ok(entry)
    }

    pub async fn skip(&self, data_source: &mut DataSource, skip_count: i64) -> Result<()> {
        let mut provider = self.create_provider(data_source).await?;
        provider.skip(skip_count).await?;

        match data_source {
            DataSource::Playback { current_index, .. } => {
                *current_index = provider.get_current_index()
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn reset(&self, data_source: &mut DataSource) -> Result<()> {
        let mut provider = self.create_provider(data_source).await?;
        provider.reset().await?;

        match data_source {
            DataSource::Playback { current_index, .. } => *current_index = 0,
            DataSource::Temperatures { current_index, .. } => *current_index = 0,
            _ => {}
        }

        Ok(())
    }
}
