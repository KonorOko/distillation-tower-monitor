use log::info;
use std::sync::Arc;
use std::time::Instant;
use tauri::{AppHandle, Emitter};
use tokio::time::Duration;

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
        // initialize transmission state
        let transmission_guard = app_state.transmission_state.clone();
        transmission_guard.lock().await.set_is_running(true);
    }

    loop {
        let start_time = Instant::now();

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
        {
            let mut history_guard = app_state.history.lock().await;
            history_guard.history.push(entry.clone());
        }

        let elapsed_time = start_time.elapsed();
        println!("Elapsed time: {:?}", elapsed_time);
        println!("\nSending: {:?}", entry);
        app_handle
            .emit("column_data", entry)
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(Duration::from_millis(speed)).await;
    }
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
    let mut transmission_state = app_state.transmission_state.lock().await;

    let index = transmission_state.data_provider.get_current_index();
    let initial_index = index.saturating_sub(data_amount as usize);

    transmission_state.data_provider.skip(-data_amount as i64)?;

    let mut new_data = Vec::new();

    for _ in initial_index..index {
        let entry = transmission_state
            .data_provider
            .get_next_entry(0, 0.0, 0.0)
            .await
            .map_err(|e| e.to_string())?;

        new_data.push(entry);
    }
    println!("{:?}", start.elapsed());
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

    let mut history_guard = app_state.history.lock().await;
    history_guard.history.clear();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn handle_skip(app_state: State<'_, AppState>, skip_count: i32) -> Result<(), String> {
    info!("Handling skip {} seconds", skip_count);
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
