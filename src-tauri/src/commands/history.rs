use crate::calculations::types::{CalculationHistory, HistorySummary};
use crate::AppState;
use log::info;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn list_calculation_histories(
    app_state: State<'_, AppState>,
) -> Result<Vec<HistorySummary>, String> {
    info!("Listing calculation histories");

    let paths = app_state
        .calculation_history_service
        .list_history_files()
        .map_err(|e| e.to_string())?;

    let mut summaries = Vec::with_capacity(paths.len());

    for path in paths {
        match app_state
            .calculation_history_service
            .load_history_summary(&path)
        {
            Ok(summary) => summaries.push(summary),
            Err(e) => {
                log::error!("Failed to load history summary for {:?}: {}", path, e);
                continue;
            }
        }
    }

    info!("Found {} calculation histories", summaries.len());
    Ok(summaries)
}

#[tauri::command]
#[specta::specta]
pub async fn get_calculation_history(
    app_state: State<'_, AppState>,
    history_id: String,
) -> Result<CalculationHistory, String> {
    let path = app_state
        .calculation_history_service
        .base_dir
        .join(&history_id);

    info!("Loading history from path: {:?}", path);

    let history = app_state
        .calculation_history_service
        .load_history(&path)
        .map_err(|e| e.to_string())?;

    Ok(history)
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ExportResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn export_calculation_history(
    history_id: Option<String>,
    export_path: String,
) -> Result<ExportResult, String> {
    info!("Exporting calculation history to {}", export_path);

    if history_id.is_none() {
        return Ok(ExportResult {
            success: false,
            file_path: None,
            error: Some("Not implemented yet".into()),
        });
    }

    Ok(ExportResult {
        success: false,
        file_path: None,
        error: Some("Export functionality not implemented yet".into()),
    })
}

#[tauri::command]
#[specta::specta]
pub async fn delete_calculation_history(
    app_state: State<'_, AppState>,
    history_id: String,
) -> Result<bool, String> {
    info!("Deleting calculation history: {}", history_id);

    let path = app_state
        .calculation_history_service
        .base_dir
        .join(&history_id);

    match std::fs::remove_file(path) {
        Ok(_) => {
            info!("Successfully deleted history: {}", history_id);
            Ok(true)
        }
        Err(e) => {
            log::error!("Failed to delete history {}: {}", history_id, e);
            Ok(false)
        }
    }
}
