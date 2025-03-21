use crate::errors::{ModbusError, Result};
use crate::settings::SettingsService;
use crate::ModbusClient;
use crate::ModbusService;
use crate::{AppState, DataSource};
use log::error;
use tauri::State;

#[tauri::command]
pub async fn connect_modbus(app_state: State<'_, AppState>) -> Result<()> {
    let mut channel_guard = app_state.modbus_channel.lock().await;

    if channel_guard.is_some() {
        return Err(ModbusError::ConnectionError("Already connected".to_string()).into());
    }

    // Get Settings
    let settings_path = app_state.settings_path.clone();
    let settings_service = SettingsService::new();
    let settings = settings_service.get_settings(&settings_path)?;

    // Connect to Modbus
    let modbus_client = ModbusClient::new();
    let modbus_service = ModbusService::new(modbus_client);
    let new_channel = modbus_service.connect(&settings.modbus).await?;

    // Set the new channel
    *channel_guard = Some(new_channel);

    // Initialize transmission state
    let mut transmission_guard = app_state.transmission_state.lock().await;
    transmission_guard.set_data_source(DataSource::Live);
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
