use super::TauriChadError;
use chad_rs::database::{self, table, DatabaseFetcher, GetGamesOpts};

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
    fetcher
        .list_items::<table::ListGenres>()
        .await
        .map_err(|e| TauriChadError::from(e))
}

#[tauri::command]
pub async fn database_get_languages(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher
        .list_items::<table::ListLanguages>()
        .await
        .map_err(|e| TauriChadError::from(e))
}

#[tauri::command]
pub async fn database_get_tags(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher
        .list_items::<table::ListTags>()
        .await
        .map_err(|e| TauriChadError::from(e))
}
