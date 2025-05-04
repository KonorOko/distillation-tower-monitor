use crate::data_manager::import_export::excel::ExcelDataExporter;
use crate::data_manager::import_export::ExcelDataImporter;
use crate::AppState;
use crate::{calculations::service::CalculationService, data_manager::factory::ProviderFactory};
use log::info;
use tauri::{AppHandle, Emitter, State};

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
pub async fn export_data(app_state: State<'_, AppState>, path: String) -> Result<(), String> {
    info!("Export data to excel...");
    let calculation_service = CalculationService::new();
    let exporter = ExcelDataExporter::new(calculation_service);

    let history_guard = app_state.history.lock().await;
    let history = history_guard.history.clone();
    exporter
        .export_data(&history, 1000.0, 0.98, &path)
        .map_err(|e| e.to_string())?;

    Ok(())
}
