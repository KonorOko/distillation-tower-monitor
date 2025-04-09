use std::sync::Arc;

use crate::calculations::service::CalculationService;
use crate::data_manager::factory::ProviderFactory;
use crate::errors::{ModbusError, Result};
use crate::settings::SettingsService;
use crate::ModbusClient;
use crate::ModbusService;
use crate::{AppState, DataSource};
use log::error;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn connect_modbus(app_state: State<'_, AppState>) -> Result<()> {
    // Get Settings
    let settings_path = app_state.settings_path.clone();
    let settings_service = SettingsService::new();
    let settings = settings_service.get_settings(&settings_path)?;

    // Connect to Modbus
    let modbus_client = ModbusClient::new();
    let modbus_service = ModbusService::new(modbus_client);
    let new_channel = modbus_service.connect(&settings.modbus).await?;

    // Initialize transmission state
    let calculation_service = CalculationService::new();
    let provider_factory = ProviderFactory::new();
    let provider = provider_factory.create_live_provider(
        Arc::new(calculation_service),
        Arc::new(Mutex::new(Some(new_channel.clone()))),
    );

    let mut transmission_guard = app_state.transmission_state.lock().await;
    transmission_guard.set_data_source(DataSource { provider });
    transmission_guard.start();

    Ok(())
}

#[tauri::command]
pub async fn disconnect_modbus(app_state: State<'_, AppState>) -> Result<()> {
    let mut channel_guard = app_state.modbus_channel.lock().await;

    // Disconnect from Modbus
    if let Some(channel) = channel_guard.as_mut() {
        channel.disable().await.map_err(|e| {
            error!("Failed to disable Modbus channel: {}", e);
            ModbusError::ConnectionError("Failed to disable Modbus channel".to_string())
        })?;
    }

    // Clear the channel
    *channel_guard = None;

    Ok(())
}
