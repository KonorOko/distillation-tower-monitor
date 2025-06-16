use log::{debug, info};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::time::Duration;

use crate::data_manager::playback::PlaybackDataProvider;
use crate::data_manager::types::ColumnEntry;
use crate::errors::{DataError, Error};
use crate::AppState;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn send_column_data(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    number_plates: i32,
    initial_mass: f32,
    initial_concentration: f32,
) -> Result<(), String> {
    info!("Initializing send_column_data...");

    {
        let transmission_guard = app_state.transmission_state.clone();
        transmission_guard.lock().await.set_is_running(true);
    }

    loop {
        let (speed, entry) = {
            let mut transmission_guard = app_state.transmission_state.lock().await;

            if !transmission_guard.is_running {
                return Ok(());
            }

            if transmission_guard.is_paused {
                drop(transmission_guard);
                tokio::time::sleep(Duration::from_millis(200)).await;
                continue;
            }

            let entry_result = transmission_guard
                .data_provider
                .get_next_entry(number_plates, initial_mass, initial_concentration)
                .await;

            let entry = match entry_result {
                Err(Error::DataError(DataError::NoDataError)) => {
                    info!("No more data to send, pausing transmission.");
                    transmission_guard.set_is_paused(true);
                    continue;
                }
                Err(e) => {
                    return {
                        info!("Error getting next entry: {}, of type {:?}", e, e);
                        Err(e.to_string())
                    }
                }
                Ok(e) => e,
            };

            (transmission_guard.speed, entry)
        };

        debug!(
            "Data collected: Mass={}, Concentration={}, entry_timestamp={}",
            initial_mass, initial_concentration, entry.timestamp
        );

        let calculation_service = &app_state.calculation_service;
        calculation_service.calculate_distilled_mass(
            initial_concentration as f64,
            initial_mass as f64,
            &[entry.clone()],
        );

        app_handle
            .emit("column_data", entry)
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(Duration::from_millis(speed)).await;
    }
}

#[tauri::command]
#[specta::specta]
pub async fn load_history_data(
    app_state: State<'_, AppState>,
    history_id: String,
) -> Result<(), String> {
    info!("Loading history data for ID: {}", history_id);
    let path = app_state
        .calculation_history_service
        .base_dir
        .join(&history_id);

    info!("Loading history from path: {:?}", path);

    let history = app_state
        .calculation_history_service
        .load_history(&path)
        .map_err(|e| e.to_string())?;

    let entries = app_state
        .calculation_history_service
        .convert_history_to_entries(&history);

    let playback_provider = PlaybackDataProvider::new(entries);
    let mut transmission_guard = app_state.transmission_state.lock().await;
    transmission_guard.data_provider = Box::new(playback_provider);

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn toggle_column_data(app_state: State<'_, AppState>) -> Result<String, String> {
    info!("Toggling column data");
    let mut transmission_state = app_state.transmission_state.lock().await;

    if !transmission_state.data_provider.has_next() {
        return Ok("paused".to_string());
    }

    transmission_state.toggle();

    let is_paused = transmission_state.is_paused;

    if is_paused {
        Ok("paused".to_string())
    } else {
        Ok("running".to_string())
    }
}

#[tauri::command]
#[specta::specta]
pub async fn refresh_data(
    app_state: State<'_, AppState>,
    data_amount: i32,
) -> Result<Vec<Arc<ColumnEntry>>, String> {
    info!("Refreshing data with amount: {}", data_amount);
    let start = std::time::Instant::now();

    let history_data = app_state
        .calculation_history_service
        .get_current_session_data()
        .await
        .map_err(|e| e.to_string())?;

    if !history_data.is_empty() {
        let len = history_data.len();
        let start_idx = if data_amount as usize >= len {
            0
        } else {
            len - data_amount as usize
        };

        debug!(
            "Refresh completed in {:?} using calculation history ({} entries)",
            start.elapsed(),
            history_data[start_idx..].len()
        );
        return Ok(history_data[start_idx..].to_vec());
    }

    let mut transmission_state = app_state.transmission_state.lock().await;
    let index = transmission_state.data_provider.get_current_index();
    let initial_index = index.saturating_sub(data_amount as usize);

    transmission_state.data_provider.skip(-data_amount as i64)?;

    let mut new_data = Vec::with_capacity(data_amount as usize);

    for _ in initial_index..index {
        let entry = transmission_state
            .data_provider
            .get_next_entry(0, 0.0, 0.0)
            .await
            .map_err(|e| e.to_string())?;

        new_data.push(entry);
    }

    debug!(
        "Refresh completed in {:?} using data provider ({} entries)",
        start.elapsed(),
        new_data.len()
    );

    Ok(new_data)
}

#[tauri::command]
#[specta::specta]
pub async fn set_is_paused(app_state: State<'_, AppState>, is_paused: bool) -> Result<(), String> {
    info!("Setting is_paused to {}", is_paused);
    let mut transmission_state = app_state.transmission_state.lock().await;
    transmission_state.set_is_paused(is_paused);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn cancel_column_data(app_state: State<'_, AppState>) -> Result<(), String> {
    info!("Canceling column data");
    let mut transmission_state = app_state.transmission_state.lock().await;
    transmission_state.reset().await?;

    let _ = app_state
        .calculation_history_service
        .finish_current_history()
        .await
        .map_err(|e| debug!("Error finalizing history: {}", e));

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn handle_skip(app_state: State<'_, AppState>, skip_count: i32) -> Result<(), String> {
    info!("Handling skip operations: {}", skip_count);
    let mut transmission_guard = app_state.transmission_state.lock().await;
    transmission_guard.data_provider.skip(skip_count as i64)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn set_speed(app_state: State<'_, AppState>, speed_factor: u32) -> Result<(), String> {
    info!("Setting speed");
    let mut transmission_state = app_state.transmission_state.lock().await;
    let new_speed = 1000 / speed_factor;

    transmission_state.set_speed(new_speed as u64);

    Ok(())
}
