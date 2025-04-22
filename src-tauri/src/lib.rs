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
use crate::commands::emitter::{
    cancel_column_data, handle_skip, send_column_data, set_speed, toggle_column_data,
};
use crate::commands::modbus::{connect_modbus, disconnect_modbus};
use crate::commands::settings::{available_ports, get_settings, save_settings};
use crate::data_manager::types::ColumnEntry;
use crate::modbus::client::ModbusClient;
use crate::modbus::service::ModbusService;
use data_manager::factory::ProviderFactory;
use data_manager::provider::DataProvider;
use log::info;
use rodbus::client::Channel;
use settings::types::Settings;
use settings::SettingsService;
use specta_typescript::Typescript;
use std::sync::Arc;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::Mutex;

#[derive(Clone)]
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

pub struct TransmissionState {
    pub data_provider: Box<dyn DataProvider + Send>,
    pub is_running: bool,
    pub is_paused: bool,
    pub speed: u64,
}

impl Clone for TransmissionState {
    fn clone(&self) -> Self {
        Self {
            data_provider: self.data_provider.clone_provider(),
            is_running: self.is_running,
            is_paused: self.is_paused,
            speed: self.speed,
        }
    }
}

impl TransmissionState {
    pub fn new(data_provider: Box<dyn DataProvider + Send>) -> Self {
        TransmissionState {
            data_provider,
            is_running: false,
            is_paused: false,
            speed: 1000,
        }
    }
    pub fn start(&mut self) {
        self.is_running = true;
        self.is_paused = false;
    }
    pub fn stop(&mut self) {
        self.is_running = false;
        self.is_paused = false;
    }
    pub fn toggle(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn set_data_provider(&mut self, data_provider: Box<dyn DataProvider + Send>) {
        self.data_provider = data_provider;
    }

    pub fn set_is_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }

    pub fn set_is_paused(&mut self, is_paused: bool) {
        self.is_paused = is_paused;
    }

    pub async fn reset(&mut self) -> Result<(), String> {
        self.data_provider.reset()?;
        self.is_running = false;
        self.is_paused = false;
        self.speed = 1000;
        Ok(())
    }

    pub fn set_speed(&mut self, speed: u64) {
        self.speed = speed;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
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
        import_temperatures,
        available_ports,
        toggle_column_data
    ]);
    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            builder.mount_events(app);
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

            let provider_factory = ProviderFactory::new();
            let provider = provider_factory.create_playback_provider(vec![], 0);
            // Initialize the app state
            let app_state = AppState {
                transmission_state: Arc::new(Mutex::new(TransmissionState::new(provider))),
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
