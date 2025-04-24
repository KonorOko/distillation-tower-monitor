use crate::calculations::service::CalculationService;
use crate::calculations::types::CompositionResult;
use crate::data_manager::provider::DataProvider;
use crate::data_manager::types::ColumnEntry;
use crate::errors::{DataError, Result};
use crate::modbus::client::ModbusClient;
use crate::modbus::service::ModbusService;
use async_trait::async_trait;
use rodbus::client::Channel;
use rodbus::{AddressRange, UnitId};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct LiveDataProvider {
    modbus_channel: Arc<Mutex<Option<Channel>>>,
    calculation_service: Arc<CalculationService>,
    modbus_service: Arc<ModbusService<ModbusClient>>,
    history: Vec<Arc<ColumnEntry>>,
}

impl LiveDataProvider {
    pub fn new(
        modbus_channel: Arc<Mutex<Option<Channel>>>,
        calculation_service: Arc<CalculationService>,
        modbus_service: Arc<ModbusService<ModbusClient>>,
    ) -> Self {
        Self {
            modbus_service,
            calculation_service,
            modbus_channel,
            history: Vec::new(),
        }
    }
}

#[async_trait]
impl DataProvider for LiveDataProvider {
    async fn get_next_entry(&mut self, number_plates: i32) -> Result<Arc<ColumnEntry>> {
        let initial_mass = 1000.0;

        let mut channel_guard = self.modbus_channel.lock().await;
        let channel = channel_guard
            .as_mut()
            .ok_or_else(|| DataError::CustomError("No Modbus channel available".into()))?;

        // Modbus parameters
        let param = rodbus::client::RequestParam {
            id: UnitId::new(10),
            response_timeout: Duration::from_millis(1000),
        };
        let address = AddressRange::try_from(100, 2).unwrap();

        let temperatures = self
            .modbus_service
            .read_holding_registers(channel, param, address)
            .await?;

        let inter_temps = self.calculation_service.interpolate_temps(
            number_plates,
            temperatures[0].value as f64 / 100.0,
            temperatures[1].value as f64 / 100.0,
        );

        let mut compositions = Vec::with_capacity(number_plates as usize);
        for &temp in &inter_temps {
            let composition = self
                .calculation_service
                .calculate_composition(None, temp, None, None)
                .unwrap_or_else(|_| CompositionResult {
                    x_1: None,
                    y_1: None,
                });
            compositions.push(composition);
        }

        let mut distilled_mass = 0.0;

        let entry = Arc::new(ColumnEntry {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            temperatures: inter_temps,
            compositions,
            percentage_complete: 0.0,
            distilled_mass,
        });

        self.history.push(entry.clone());

        Ok(entry)
    }

    fn skip(&mut self, _count: i64) -> Result<()> {
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        self.history.clear();
        Ok(())
    }

    fn get_current_index(&self) -> usize {
        0
    }

    async fn disconnect(&self) -> Result<()> {
        let mut channel_guard = self.modbus_channel.lock().await;
        if let Some(channel) = channel_guard.take() {
            self.modbus_service.disconnect(channel).await?;
        }
        Ok(())
    }

    fn clone_provider(&self) -> Box<dyn DataProvider + Send> {
        Box::new(Self {
            calculation_service: self.calculation_service.clone(),
            history: self.history.clone(),
            modbus_channel: self.modbus_channel.clone(),
            modbus_service: self.modbus_service.clone(),
        })
    }
}
