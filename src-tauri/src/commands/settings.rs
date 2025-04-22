use crate::errors::{Result, SettingsError};
use crate::settings::{Settings, SettingsService};
use crate::AppState;
use log::debug;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn save_settings(app_state: State<'_, AppState>, settings: Settings) -> Result<Settings> {
    debug!("Saving settings");
    let settings_service = SettingsService::new();
    let app_data_dir = app_state.settings_path.clone();

    let new_setting = settings_service.update_settings(&app_data_dir, &settings)?;

    Ok(new_setting)
}

#[tauri::command]
#[specta::specta]
pub async fn get_settings(app_state: State<'_, AppState>) -> Result<Settings> {
    debug!("Getting settings");
    let settings_service = SettingsService::new();
    let app_data_dir = app_state.settings_path.clone();

    settings_service.get_settings(&app_data_dir)
}

#[tauri::command]
#[specta::specta]
pub async fn available_ports() -> Result<Vec<String>> {
    match serialport::available_ports() {
        Ok(ports) => {
            let port_names: Vec<String> = ports
                .into_iter()
                .filter_map(|port| match port.port_type {
                    serialport::SerialPortType::UsbPort(_) => {
                        if port.port_name.starts_with("/dev/tty.") {
                            None
                        } else {
                            Some(port.port_name)
                        }
                    }
                    _ => None,
                })
                .collect();
            Ok(port_names)
        }
        Err(err) => Err(SettingsError::GetUsbDevicesError(err.to_string()).into()),
    }
}
