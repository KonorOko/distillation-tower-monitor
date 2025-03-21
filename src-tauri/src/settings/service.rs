use crate::errors::{FileError, Result};
use crate::files::{read_json_file, write_json_file};
use crate::settings::Settings;
use log::info;
use tauri::{AppHandle, Manager};

const SETTINGS_FILE: &str = "settings.json";

pub struct SettingsService;

impl SettingsService {
    pub fn new() -> Self {
        SettingsService
    }

    pub fn get_settings(&self, path: &str) -> Result<Settings> {
        info!("Getting settings from {}", path);

        let settings: Settings = read_json_file(path)?;
        Ok(settings)
    }

    pub fn update_settings(&self, path: &str, settings: &Settings) -> Result<Settings> {
        info!("Updating settings at {}", path);

        write_json_file(path, settings).map_err(|e| FileError::WriteError(e.to_string()))?;
        Ok(settings.clone())
    }

    pub fn get_settings_path(&self, app_handle: &AppHandle) -> Result<String> {
        let path = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data directory")
            .join(SETTINGS_FILE);

        path.to_str().map(|s| s.to_string()).ok_or_else(|| {
            FileError::ReadError("Failed to convert path to string".to_string()).into()
        })
    }
}
