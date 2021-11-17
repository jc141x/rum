use crate::command::TauriRumError;
use rumlibrs::config::Config;
use std::path::PathBuf;
use tauri::async_runtime::Mutex;

#[tauri::command]
pub async fn config_save(config: tauri::State<'_, Mutex<Config>>) -> Result<(), TauriRumError> {
    config.lock().await.save()?;
    Ok(())
}

#[tauri::command]
pub async fn config_get(config: tauri::State<'_, Mutex<Config>>) -> Result<Config, TauriRumError> {
    Ok(config.lock().await.clone())
}

#[tauri::command]
pub async fn config_get_data_path(
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<PathBuf, TauriRumError> {
    Ok(config.lock().await.data_path().to_owned())
}

#[tauri::command]
pub async fn config_get_library_paths(
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<Vec<PathBuf>, TauriRumError> {
    Ok(config
        .lock()
        .await
        .library_paths()
        .iter()
        .map(|p| p.to_owned())
        .collect())
}

#[tauri::command]
pub async fn config_get_terminal(
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<String, TauriRumError> {
    Ok(config.lock().await.terminal().into())
}

#[tauri::command]
pub async fn config_set(
    new_config: Config,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriRumError> {
    config.lock().await.set_config(new_config);
    Ok(())
}

#[tauri::command]
pub async fn config_set_data_path(
    data_path: PathBuf,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriRumError> {
    config.lock().await.set_data_path(&data_path);
    Ok(())
}

#[tauri::command]
pub async fn config_set_library_paths(
    library_paths: Vec<PathBuf>,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriRumError> {
    config.lock().await.set_library_paths(&library_paths);
    Ok(())
}

#[tauri::command]
pub async fn config_set_terminal(
    terminal: String,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriRumError> {
    config.lock().await.set_terminal(&terminal);
    Ok(())
}
