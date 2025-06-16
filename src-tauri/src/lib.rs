mod calculations;
mod commands;
mod data_manager;
mod errors;
mod files;
mod math;
mod modbus;
mod settings;
mod storage;

use crate::calculations::service::CalculationService;
use crate::commands::data_handle::{export_data, import_data, import_to_history};
use crate::commands::dialogs::{file_path, folder_path};
use crate::commands::emitter::{
    cancel_column_data, handle_skip, load_history_data, refresh_data, send_column_data,
    set_is_paused, set_speed, toggle_column_data,
};
use crate::commands::history::{
    delete_calculation_history, export_calculation_history, get_calculation_history,
    list_calculation_histories,
};
use crate::commands::modbus::{connect_modbus, disconnect_modbus};
use crate::commands::settings::{available_ports, get_settings, save_settings};
use crate::modbus::client::ModbusClient;
use crate::modbus::service::ModbusService;
use crate::storage::service::CalculationHistoryService;
use data_manager::factory::ProviderFactory;
use data_manager::provider::DataProvider;
use log::{error, info};
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
    settings_path: String,
    calculation_history_service: Arc<CalculationHistoryService>,
    calculation_service: Arc<CalculationService>,
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
        import_to_history,
        file_path,
        folder_path,
        send_column_data,
        cancel_column_data,
        handle_skip,
        set_speed,
        available_ports,
        toggle_column_data,
        set_is_paused,
        refresh_data,
        list_calculation_histories,
        get_calculation_history,
        export_calculation_history,
        delete_calculation_history,
        load_history_data
    ]);

    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
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

            // Inicializar el servicio de historial de cálculos
            let calculation_history_dir = app_handle
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("./data"))
                .join("calculation_history");

            let calculation_history_service =
                match CalculationHistoryService::new(calculation_history_dir) {
                    Ok(service) => service,
                    Err(err) => {
                        error!(
                            "No se pudo inicializar el servicio de historial de cálculos: {}",
                            err
                        );
                        panic!("Failed to initialize calculation history service");
                    }
                };

            // Inicializar servicio de cálculos
            let calculation_service = CalculationService::new();

            let app_state = AppState {
                transmission_state: Arc::new(Mutex::new(TransmissionState::new(provider))),
                settings_path,
                calculation_history_service: Arc::new(calculation_history_service),
                calculation_service: Arc::new(calculation_service),
            };

            app.manage(app_state.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
