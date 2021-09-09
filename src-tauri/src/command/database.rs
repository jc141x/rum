use super::TauriChadError;
use chad_rs::database::{self, table, DatabaseFetcher, GetGamesOpts};
use serde::de::DeserializeOwned;

#[tauri::command]
pub async fn database_get_games(
    opts: GetGamesOpts,
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<database::Game>, TauriChadError> {
    fetcher.get_games(&opts).await.map_err(|e| e.into())
}

async fn get_items<T: table::Table + Into<String> + DeserializeOwned>(
    fetcher: &DatabaseFetcher,
) -> Result<Vec<String>, TauriChadError> {
    Ok(fetcher
        .list_table::<T>()
        .await
        .map_err(|e| TauriChadError::from(e))?
        .into_iter()
        .map(|i| i.into())
        .collect())
}

#[tauri::command]
pub async fn database_get_genres(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    get_items::<table::ListGenres>(&*fetcher).await
}

#[tauri::command]
pub async fn database_get_languages(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    get_items::<table::ListLanguages>(&*fetcher).await
}

#[tauri::command]
pub async fn database_get_tags(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    get_items::<table::ListTags>(&*fetcher).await
}
