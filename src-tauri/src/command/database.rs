use super::TauriChadError;
use chad_launcher::database::{self, get_magnet, DatabaseFetcher, GetGamesOpts};
use std::process::Command;

#[tauri::command]
pub async fn database_get_games(
    opts: GetGamesOpts,
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<database::Game>, TauriChadError> {
    fetcher.get_games(&opts).await.map_err(|e| e.into())
}

#[tauri::command]
pub async fn database_get_genres(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher.get_items("get_genres").await.map_err(|e| e.into())
}

#[tauri::command]
pub async fn database_get_languages(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher
        .get_items("get_languages")
        .await
        .map_err(|e| e.into())
}

#[tauri::command]
pub async fn database_get_tags(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher.get_items("get_tags").await.map_err(|e| e.into())
}
