use crate::calculations::service::CalculationService;
use crate::data_manager::import_export::ExcelDataImporter;
use crate::data_manager::types::DataSource;
use crate::AppState;
use calamine::Reader;
use log::info;
use rust_xlsxwriter::{Workbook, XlsxError};
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub async fn import_data(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    info!("Importing data from {}", path);
    let calculation_service = CalculationService::new();
    let importer = ExcelDataImporter::new(calculation_service);

    let (number_plates, imported_data) = importer.import(&path).await.map_err(|e| e.to_string())?;

    {
        let mut transmission_guard = app_state.transmission_state.lock().await;
        transmission_guard.set_data_source(DataSource::Playback {
            current_index: 0,
            data: imported_data,
        });
    }

    app_handle
        .emit("number_plates", number_plates)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn import_temperatures(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    path: &str,
) -> Result<(), String> {
    info!("Importing data from {}", path);
    let calculation_service = CalculationService::new();
    let importer = ExcelDataImporter::new(calculation_service);

    let (number_plates, imported_data) = importer.import(path).await.map_err(|e| e.to_string())?;

    {
        let mut trasmission_guard = app_state.transmission_state.lock().await;
        trasmission_guard.set_data_source(DataSource::Playback {
            current_index: 0,
            data: imported_data,
        });
    }

    app_handle
        .emit("number_plates", number_plates)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn export_data(app_state: State<'_, AppState>, path: String) -> Result<(), String> {
    info!("Export data to excel...");
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let column_data = {
        let data_column = app_state.history.lock().await;
        data_column.history.clone()
    };

    info!("Writing headers...");

    let Some(first) = column_data.first() else {
        return Err("No current data".into());
    };

    // write headers
    worksheet
        .write(0, 0, "Timestamp")
        .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;
    let num_values = first.temperatures.len();
    for i in 0..num_values {
        worksheet
            .write(0, (i + 1) as u16, format!("Temperature {}", i + 1))
            .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;
        worksheet
            .write(
                0,
                (num_values + i + 1) as u16,
                format!("Composition x_1 {}", i + 1),
            )
            .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;
        worksheet
            .write(
                0,
                (num_values * 2 + i + 1) as u16,
                format!("Composition y_1 {}", i + 1),
            )
            .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;
    }

    // write data
    info!("Writing data");
    for (row, value) in column_data.iter().enumerate() {
        let row = (row + 1) as u32;
        worksheet
            .write(row, 0, value.timestamp)
            .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;

        for (i, &temp) in value.temperatures.iter().enumerate() {
            worksheet
                .write(row, (i + 1) as u16, temp)
                .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;
        }
        for (i, comp) in value.compositions.iter().enumerate() {
            worksheet
                .write(row, (num_values + i + 1) as u16, comp.x_1)
                .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;

            worksheet
                .write(row, (num_values * 2 + i + 1) as u16, comp.y_1)
                .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;
        }
    }

    info!("Saving excel...");
    workbook
        .save(path)
        .map_err(|e: XlsxError| format!("Xlsx error: {}", e))?;
    info!("Excel saved");

    Ok(())
}
