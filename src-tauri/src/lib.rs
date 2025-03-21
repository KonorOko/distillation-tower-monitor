mod calculations;
mod commands;
mod data_manager;
mod errors;
mod files;
mod math;
mod modbus;
mod settings;

use crate::commands::data_handle::{export_data, import_data, import_temperatures};
use crate::commands::dialogs::{file_path, folder_path};
use crate::commands::emitter::{cancel_column_data, handle_skip, send_column_data, set_speed};
use crate::commands::modbus::{connect_modbus, disconnect_modbus};
use crate::commands::settings::{get_settings, save_settings};
use crate::data_manager::types::{ColumnEntry, DataSource};
use crate::modbus::client::ModbusClient;
use crate::modbus::service::ModbusService;
use log::info;
use rodbus::client::Channel;
use settings::types::Settings;
use settings::SettingsService;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    transmission_state: Arc<Mutex<TransmissionState>>,
    history: Arc<Mutex<History>>,
    modbus_channel: Arc<Mutex<Option<Channel>>>,
    settings_path: String,
}

#[derive(Default, Clone, Debug)]
pub struct History {
    pub history: Vec<Arc<ColumnEntry>>,
}

#[derive(Debug, Clone)]
pub struct TransmissionState {
    pub data_source: DataSource,
    pub is_running: bool,
    pub speed: u64,
}

impl TransmissionState {
    pub fn new(data_source: DataSource) -> Self {
        TransmissionState {
            data_source,
            is_running: false,
            speed: 500,
        }
    }
    pub fn start(&mut self) {
        self.is_running = true;
    }
    pub fn stop(&mut self) {
        self.is_running = false;
    }
    pub fn toggle(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn set_data_source(&mut self, data_source: DataSource) {
        self.data_source = data_source;
    }

    pub fn set_is_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }

    pub fn reset(&mut self) {
        self.data_source = DataSource::Live;
        self.is_running = false;
    }

    pub fn set_speed(&mut self, speed: u64) {
        self.speed = speed;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            connect_modbus,
            disconnect_modbus,
            export_data,
            import_data,
            file_path,
            folder_path,
            send_column_data,
            cancel_column_data,
            handle_skip,
            set_speed,
            import_temperatures
        ])
        .setup(|app| {
            let app_handle = app.handle();

            // Initialize settings service
            let settings_service = SettingsService::new();
            let settings_path = settings_service.get_settings_path(app_handle)?;

            // Get settings or initialize default settings
            let settings = settings_service
                .get_settings(&settings_path)
                .unwrap_or_else(|_| {
                    let new_settings =
                        settings_service.update_settings(&settings_path, &Settings::default());

                    if let Err(err) = new_settings {
                        eprintln!("Failed to update settings: {}", err);
                        panic!("Failed to update settings");
                    }

                    new_settings.unwrap()
                });
            info!("Initial settings: {:?}", settings);

            // Initialize the app state
            let app_state = AppState {
                transmission_state: Arc::new(Mutex::new(TransmissionState::new(DataSource::Live))),
                history: Arc::new(Mutex::new(History::default())),
                modbus_channel: Arc::new(Mutex::new(None)),
                settings_path,
            };

            app.manage(app_state.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
