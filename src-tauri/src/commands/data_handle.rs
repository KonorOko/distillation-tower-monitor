use crate::data_manager::import_export::excel::ExcelDataExporter;
use crate::data_manager::import_export::ExcelDataImporter;
use crate::AppState;
use crate::{calculations::service::CalculationService, data_manager::factory::ProviderFactory};
use log::info;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Emitter, State};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ImportResult {
    pub success: bool,
    pub number_plates: i32,
    pub initial_mass: Option<f64>,
    pub initial_composition: Option<f64>,
    pub history_id: Option<String>,
    pub message: String,
}

#[tauri::command]
#[specta::specta]
pub async fn import_data(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    info!("Importing data from {}", path);
    let calculation_service = CalculationService::new();
    let importer = ExcelDataImporter::new(calculation_service);

    let (number_plates, imported_data, initial_mass, initial_composition) =
        importer.import(&path).await.map_err(|e| e.to_string())?;

    let provider_factory = ProviderFactory::new();
    let provider = provider_factory.create_playback_provider(imported_data, 0);

    {
        let mut transmission_guard = app_state.transmission_state.lock().await;
        transmission_guard.set_data_provider(provider);
    }

    app_handle
        .emit(
            "initial_data",
            (number_plates, initial_mass, initial_composition),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn import_to_history(
    app_state: State<'_, AppState>,
    path: String,
) -> Result<ImportResult, String> {
    info!("Importing Excel to history from {}", path);

    let calculation_service = CalculationService::new();
    let importer = ExcelDataImporter::with_history_service(
        calculation_service,
        app_state.calculation_history_service.clone(),
    );

    match importer.import_to_history(&path).await {
        Ok(history_id) => {
            let result = ImportResult {
                success: true,
                number_plates: 0,
                initial_mass: None,
                initial_composition: None,
                history_id: Some(history_id.clone()),
                message: format!("Excel imported: {}", history_id),
            };

            Ok(result)
        }
        Err(e) => {
            let error_msg = e.to_string();
            let result = ImportResult {
                success: false,
                number_plates: 0,
                initial_mass: None,
                initial_composition: None,
                history_id: None,
                message: format!("Import error: {}", error_msg),
            };

            Ok(result)
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn export_data(
    app_state: State<'_, AppState>,
    path: String,
    initial_mass: f32,
    initial_concentration: f32,
) -> Result<(), String> {
    info!("Export data to excel...");
    let exporter = ExcelDataExporter;

    let history = app_state
        .calculation_history_service
        .get_current_session_data()
        .await
        .map_err(|e| e.to_string())?;

    exporter
        .export_data(
            &history,
            initial_mass as f64,
            initial_concentration as f64,
            &path,
        )
        .map_err(|e| e.to_string())?;

    info!("Excel export completed successfully");
    Ok(())
}
