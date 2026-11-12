use belegwerk_infrastructure::{migrate, open_db, DataPaths, FsBlobStore, BlobStore};
use serde::Serialize;
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
struct AppState {
    data_root: Mutex<Option<std::path::PathBuf>>,
}

#[derive(Serialize)]
struct HealthInfo {
    app: &'static str,
    version: &'static str,
    locale: &'static str,
    bank_payments: bool,
    telemetry_default: bool,
}

#[tauri::command]
fn health() -> HealthInfo {
    HealthInfo {
        app: "Belegwerk",
        version: env!("CARGO_PKG_VERSION"),
        locale: "de-DE",
        bank_payments: false,
        telemetry_default: false,
    }
}

#[tauri::command]
fn init_data_root(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("profile-default");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let paths = DataPaths::new(&dir);
    paths.ensure().map_err(|e| e.to_string())?;
    let conn = open_db(&paths.db_path()).map_err(|e| e.to_string())?;
    let version = migrate(&conn).map_err(|e| e.to_string())?;
    let _store = FsBlobStore::new(paths.clone()).map_err(|e| e.to_string())?;
    *state.data_root.lock().map_err(|e| e.to_string())? = Some(dir.clone());
    Ok(format!("data_root={} schema_version={version}", dir.display()))
}

#[tauri::command]
fn put_synthetic_blob(state: tauri::State<'_, AppState>, text: String) -> Result<String, String> {
    let root = state
        .data_root
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "data root not initialized".to_string())?;
    let store = FsBlobStore::new(DataPaths::new(root)).map_err(|e| e.to_string())?;
    let key = store.put_bytes(text.as_bytes()).map_err(|e| e.to_string())?;
    Ok(key.hash.0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![health, init_data_root, put_synthetic_blob])
        .run(tauri::generate_context!())
        .expect("error while running Belegwerk");
}
