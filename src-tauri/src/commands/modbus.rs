use std::sync::Arc;
use std::time::Duration;

use crate::calculations::service::CalculationService;
use crate::data_manager::factory::ProviderFactory;
use crate::errors::{Error, ModbusError, Result};
use crate::settings::SettingsService;
use crate::AppState;
use crate::ModbusClient;
use crate::ModbusService;
use log::error;
use rodbus::client::RequestParam;
use rodbus::{AddressRange, UnitId};
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
#[specta::specta]
pub async fn connect_modbus(app_state: State<'_, AppState>) -> Result<()> {
    // Get Settings
    let settings_path = app_state.settings_path.clone();
    let settings_service = SettingsService::new();
    let settings = settings_service.get_settings(&settings_path)?;

    // Connect to Modbus
    let modbus_client = ModbusClient::new();
    let modbus_service = ModbusService::new(modbus_client);
    let mut channel = modbus_service.connect(&settings.modbus).await?;

    let param = RequestParam {
        id: UnitId {
            value: settings.modbus.unit_id,
        },
        response_timeout: Duration::from_millis(settings.modbus.timeout as u64),
    };

    for attempt in 1..=3 {
        match channel
            .read_coils(param, AddressRange::try_from(1, 1).unwrap())
            .await
        {
            Ok(_) => break,
            Err(err) => {
                println!("Attempt {}/3 failed: {:?}", attempt, err);
                if attempt < 3 {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
                return Err(Error::ModbusError(ModbusError::ConnectionError(
                    "Failed to connect to Modbus channel".to_string(),
                )));
            }
        }
    }

    // Initialize transmission state
    let calculation_service = CalculationService::new();
    let provider_factory = ProviderFactory::new();
    let provider = provider_factory.create_live_provider(
        Arc::new(calculation_service),
        Arc::new(Mutex::new(Some(channel.clone()))),
    );

    let mut transmission_guard = app_state.transmission_state.lock().await;
    transmission_guard.set_data_provider(provider);
    transmission_guard.start();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn disconnect_modbus(app_state: State<'_, AppState>) -> Result<()> {
    let transmission_guard = app_state.transmission_state.lock().await;

    // Disconnect from Modbus
    transmission_guard
        .data_provider
        .disconnect()
        .await
        .map_err(|e| {
            error!("Failed to disconnect Modbus provider: {}", e);
            ModbusError::ConnectionError("Failed to disconnect Modbus provider".to_string())
        })?;

    Ok(())
}
