use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn folder_path(app: AppHandle) -> String {
    let file_path = app.dialog().file().blocking_pick_folder();
    if let Some(file_path) = file_path {
        return file_path.to_string();
    }
    return "".into();
}

#[tauri::command]
pub async fn file_path(app: AppHandle) -> String {
    let file_path = app
        .dialog()
        .file()
        .add_filter("My filert", &["xlsx"])
        .blocking_pick_file();
    if let Some(file_path) = file_path {
        return file_path.to_string();
    }
    return "".into();
}
