use rodbus::{AddressRange, UnitId};
use std::{sync::Arc, time::Duration};
use tauri::State;

use super::types::{ColumnEntry, DataSource};
use crate::calculations::service::CalculationService;
use crate::calculations::types::CompositionResult;
use crate::modbus::{client::ModbusClient, service::ModbusService};
use crate::AppState;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DataService;

impl DataService {
    pub fn new() -> Self {
        DataService
    }

    pub async fn get_data(
        &mut self,
        app_state: &State<'_, AppState>,
        number_plates: usize,
        initial_mass: f64,
    ) -> Result<Arc<ColumnEntry>, String> {
        let transmission_state = &app_state.transmission_state;
        let transmission_guard = &mut transmission_state.lock().await;
        let data_source = &mut transmission_guard.data_source;
        match &mut *data_source {
            DataSource::Playback { index, data } => {
                if let Some(entry) = data.get(*index as usize) {
                    let current_entry = entry.clone();
                    *index += 1;
                    Ok(current_entry)
                } else {
                    return Err("No more data".to_string());
                }
            }
            DataSource::Live => {
                // Initialize services
                let modbus_client = ModbusClient::new();
                let modbus_service = ModbusService::new(modbus_client);
                let calculation_service = CalculationService::new();

                // Get channel
                let mut channel = app_state.modbus_channel.lock().await;

                // Modbus parameters
                let param = rodbus::client::RequestParam {
                    id: UnitId::new(10),
                    response_timeout: Duration::from_millis(1000),
                };
                let address_range = AddressRange::try_from(100, 2).unwrap();

                let mut channel = channel
                    .as_mut()
                    .ok_or("Failed to acquire modbus channel lock")?;

                // Read modbus data
                let temperatures = modbus_service
                    .read_holding_registers(&mut channel, param, address_range)
                    .await
                    .map_err(|e| e.to_string())?;

                // Interpolate temperatures
                let inter_temps = &calculation_service.interpolate_temps(
                    number_plates,
                    temperatures[0].value as f64 / 100.0,
                    temperatures[1].value as f64 / 100.0,
                );

                // Calculate compositions
                let mut compositions = Vec::with_capacity(number_plates);
                for temp in inter_temps {
                    let composition = calculation_service
                        .calculate_composition(None, *temp, None, None)
                        .unwrap_or_else(|e| {
                            println!("{:?}", e);
                            CompositionResult {
                                x_1: None,
                                y_1: None,
                            }
                        });
                    compositions.push(composition);
                }

                let history_guard = app_state.history.lock().await;
                let history = &history_guard.history;
                let mut distilled_mass = 0.0;

                if history.len() > 1 {
                    let x_b0 = history
                        .first()
                        .unwrap()
                        .compositions
                        .first()
                        .unwrap()
                        .x_1
                        .unwrap_or_else(|| 0.0);
                    let last_compositions = &history.last().unwrap().compositions;
                    let x_bf = last_compositions
                        .first()
                        .unwrap()
                        .x_1
                        .unwrap_or_else(|| 0.0);
                    let x_d = last_compositions.last().unwrap().x_1.unwrap_or_else(|| 0.0);

                    distilled_mass =
                        calculation_service.calculate_distilled_mass(initial_mass, x_b0, x_bf, x_d);
                }

                // Create column entry
                let entry = ColumnEntry {
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    temperatures: inter_temps.clone(),
                    compositions,
                    percentage_complete: 0.0,
                    distilled_mass,
                };

                return Ok(Arc::new(entry));
            }
            DataSource::Temperatures { index, data } => {
                if let Some(entry) = data.get(*index as usize) {
                    let current_entry = entry.clone();
                    let calculation_service = CalculationService::new();
                    let temps = &current_entry.temperatures;
                    let mut compositions = Vec::new();
                    for temp in temps {
                        let composition = calculation_service
                            .calculate_composition(None, *temp, None, None)
                            .unwrap_or_else(|e| {
                                println!("{:?}", e);
                                CompositionResult {
                                    x_1: None,
                                    y_1: None,
                                }
                            });
                        compositions.push(composition);
                    }

                    let new_entry = ColumnEntry {
                        timestamp: current_entry.timestamp,
                        temperatures: current_entry.temperatures.clone(),
                        compositions,
                        percentage_complete: current_entry.percentage_complete,
                        distilled_mass: 0.0,
                    };
                    *index += 1;
                    Ok(Arc::new(new_entry))
                } else {
                    return Err("No more data".to_string());
                }
            }
        }
    }

    pub async fn skip_data(
        &self,
        app_state: &State<'_, AppState>,
        skip_count: i64,
    ) -> Result<(), String> {
        let transmission_state = &app_state.transmission_state;
        let trasmission_guard = &mut transmission_state.lock().await;
        let data_source = &mut trasmission_guard.data_source;

        match &mut *data_source {
            DataSource::Live => {}
            DataSource::Temperatures { index, data } => {}
            DataSource::Playback { index, data } => {
                if data.is_empty() {
                    return Err("No data available".to_string());
                }

                let new_index = if skip_count.is_negative() {
                    index.saturating_sub(skip_count.unsigned_abs())
                } else {
                    index
                        .saturating_add(skip_count as u64)
                        .min(data.len() as u64 - 1)
                };

                *index = new_index;
            }
        }

        Ok(())
    }
}
