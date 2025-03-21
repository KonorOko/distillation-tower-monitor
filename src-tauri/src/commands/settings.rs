use crate::errors::Result;
use crate::settings::{Settings, SettingsService};
use crate::AppState;
use log::debug;
use tauri::State;

#[tauri::command]
pub async fn save_settings(app_state: State<'_, AppState>, settings: Settings) -> Result<Settings> {
    debug!("Saving settings");
    let settings_service = SettingsService::new();
    let app_data_dir = app_state.settings_path.clone();

    let new_setting = settings_service.update_settings(&app_data_dir, &settings)?;

    Ok(new_setting)
}

#[tauri::command]
pub async fn get_settings(app_state: State<'_, AppState>) -> Result<Settings> {
    debug!("Getting settings");
    let settings_service = SettingsService::new();
    let app_data_dir = app_state.settings_path.clone();

    settings_service.get_settings(&app_data_dir)
}
