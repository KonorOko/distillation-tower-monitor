use crate::calculations::types::CompositionResult;
use crate::data_manager::types::{ColumnEntry, DataSource};
use crate::AppState;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use log::info;
use rust_xlsxwriter::{Workbook, XlsxError};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub async fn import_data(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    info!("Importing data from {}", path);
    let mut imported_data: Vec<Arc<ColumnEntry>> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
    let worksheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or("Not valid sheets")
        .unwrap();
    let range = workbook
        .worksheet_range(&worksheet_name)
        .map_err(|_| "Can't read sheet")
        .unwrap();
    let total_rows = range.rows().count().saturating_sub(1);
    let number_plates = range
        .rows()
        .next()
        .ok_or("No data in sheet")
        .unwrap()
        .len()
        .saturating_sub(1)
        / 3;

    for (index, row) in range.rows().skip(1).enumerate() {
        info!("Processing row {}", index);
        let percentage_complete = (index as f64 + 1.0) / total_rows as f64 * 100.0;
        if row.is_empty() || row.len() < 4 {
            continue;
        }

        let timestamp = if let Some(ts) = row.get(0).and_then(|cell| cell.as_f64()) {
            ts as u64
        } else {
            continue;
        };

        let temperatures: Vec<f64> = row
            .iter()
            .skip(1)
            .take(number_plates)
            .filter_map(|cell| cell.as_f64())
            .collect();

        let comp_x: Vec<Option<f64>> = row
            .iter()
            .skip(1 + number_plates)
            .take(number_plates)
            .map(|cell| cell.as_f64())
            .collect();

        let comp_y: Vec<Option<f64>> = row
            .iter()
            .skip(1 + number_plates * 2)
            .take(number_plates)
            .map(|cell| cell.as_f64())
            .collect();

        let compositions: Vec<CompositionResult> = comp_x
            .into_iter()
            .zip(comp_y.into_iter())
            .map(|(x, y)| CompositionResult { x_1: x, y_1: y })
            .collect();

        println!("Timestamp {}", timestamp);
        println!("Temperatures {:?}", temperatures);
        println!("Compositions {:?}", compositions);
        print!("\n");

        imported_data.push(Arc::new(ColumnEntry {
            timestamp,
            temperatures,
            compositions,
            percentage_complete,
        }));
    }
    {
        let mut trasmission_guard = app_state.transmission_state.lock().await;
        trasmission_guard.set_data_source(DataSource::Playback {
            index: 0,
            data: imported_data,
        });
    }
    app_handle
        .emit("number_plates", number_plates)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn import_temperatures(app_state: State<'_, AppState>, path: &str) -> Result<(), String> {
    info!("Importing data from {}", path);
    let mut imported_data: Vec<Arc<ColumnEntry>> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
    let worksheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or("Not valid sheets")
        .unwrap();
    let range = workbook
        .worksheet_range(&worksheet_name)
        .map_err(|_| "Can't read sheet")
        .unwrap();
    let total_rows = range.rows().count().saturating_sub(1);

    for (index, row) in range.rows().skip(1).enumerate() {
        info!("Processing row {}", index);
        let percentage_complete = (index as f64 + 1.0) / total_rows as f64 * 100.0;
        if row.is_empty() || row.len() < 4 {
            continue;
        }

        let timestamp = if let Some(ts) = row.get(0).and_then(|cell| cell.as_f64()) {
            ts as u64
        } else {
            continue;
        };

        let number_plates = row.len() - 1;

        let temperatures: Vec<f64> = row
            .iter()
            .skip(1)
            .take(number_plates)
            .filter_map(|cell| cell.as_f64())
            .collect();

        println!("Timestamp {}", timestamp);
        println!("Temperatures {:?}", temperatures);
        print!("\n");

        imported_data.push(Arc::new(ColumnEntry {
            timestamp,
            temperatures,
            compositions: vec![CompositionResult {
                x_1: Some(0.0),
                y_1: Some(0.0),
            }],
            percentage_complete,
        }));
    }
    {
        let mut trasmission_guard = app_state.transmission_state.lock().await;
        trasmission_guard.set_data_source(DataSource::Playback {
            index: 0,
            data: imported_data,
        });
    }
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
