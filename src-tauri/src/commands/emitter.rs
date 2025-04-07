use std::sync::Arc;

use crate::{calculations::service::CalculationService, data_manager::service::DataService};
use log::info;
use tauri::{AppHandle, Emitter};
use tokio::time::Duration;

use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn send_column_data(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    number_plates: usize,
) -> Result<(), String> {
    info!("Initializing send_column_data...");
    let calculation_service = Arc::new(CalculationService::new());
    let data_service = DataService::new(calculation_service, app_state.modbus_channel.clone());

    {
        // initialize transmission state
        let transmission_guard = app_state.transmission_state.clone();
        transmission_guard.lock().await.set_is_running(true);
    }

    loop {
        let (speed, mut data_source) = {
            let transmission_guard = app_state.transmission_state.lock().await;
            print!("\n------------------------\n");
            println!("\nTransmission state: {:?}", transmission_guard.is_running);

            if !transmission_guard.is_running {
                return Ok(());
            }

            println!("Speed: {}", transmission_guard.speed);
            (
                transmission_guard.speed,
                transmission_guard.data_source.clone(),
            )
        };

        let entry = match data_service
            .get_next_entry(&mut data_source, number_plates)
            .await
        {
            Ok(entry) => entry,
            Err(e) => return Err(format!("Error getting data: {}", e)),
        };

        {
            let mut transmission_guard = app_state.transmission_state.lock().await;
            transmission_guard.data_source = data_source;
        }
        {
            let mut history_guard = app_state.history.lock().await;
            history_guard.history.push(entry.clone());
        }

        println!("\nSending: {:?}", entry);
        app_handle
            .emit("column_data", entry)
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(Duration::from_millis(speed)).await;
    }
}

#[tauri::command]
pub async fn cancel_column_data(app_state: State<'_, AppState>) -> Result<(), String> {
    println!("Canceling column data");
    let mut transmission_state = app_state.transmission_state.lock().await;
    transmission_state.reset();

    let mut history_guard = app_state.history.lock().await;
    history_guard.history.clear();

    Ok(())
}

#[tauri::command]
pub async fn handle_skip(app_state: State<'_, AppState>, skip_count: i64) -> Result<(), String> {
    info!("Handling skip {} seconds", skip_count);
    let calculation_service = Arc::new(CalculationService::new());
    let data_service = DataService::new(calculation_service, app_state.modbus_channel.clone());

    let mut data_source = {
        let guard = app_state.transmission_state.lock().await;
        guard.data_source.clone()
    };

    if let Err(e) = data_service.skip(&mut data_source, skip_count).await {
        return Err(format!("Error skipping data: {}", e));
    }

    {
        let mut guard = app_state.transmission_state.lock().await;
        guard.data_source = data_source;
    }

    Ok(())
}

#[tauri::command]
pub async fn set_speed(app_state: State<'_, AppState>, speed_factor: u64) -> Result<(), String> {
    info!("Setting speed");
    let mut transmission_state = app_state.transmission_state.lock().await;
    let new_speed = 1000 / speed_factor;

    transmission_state.set_speed(new_speed);

    Ok(())
}
