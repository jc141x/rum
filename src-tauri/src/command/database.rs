use super::TauriChadError;
use chad_rs::database::{self, table, DatabaseFetcher, GetGamesOpts};
use serde::de::DeserializeOwned;
use serde::Serialize;

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

#[tauri::command]
pub async fn database_is_admin(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<bool, TauriChadError> {
    fetcher
        .is_admin()
        .await
        .map_err(|e| TauriChadError::from(e))
}

#[tauri::command]
pub async fn database_add_update_game(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game: database::Game,
) -> Result<(), TauriChadError> {
    fetcher
        .add_update_game(&game.game, &game.languages, &game.genres, &game.tags)
        .await
        .map_err(|e| TauriChadError::from(e))
}

#[tauri::command]
pub async fn database_remove_game(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game_id: database::table::GameId,
) -> Result<(), TauriChadError> {
    fetcher
        .remove_game(&game_id)
        .await
        .map_err(|e| TauriChadError::from(e))
}

async fn add_items<T: table::Item + Serialize>(
    fetcher: &DatabaseFetcher,
    game_id: &database::table::GameId,
    items: &[String],
) -> Result<(), TauriChadError> {
    fetcher
        .add_items::<T>(game_id, items)
        .await
        .map_err(|e| TauriChadError::from(e))
}

#[tauri::command]
pub async fn database_add_genres(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game_id: database::table::GameId,
    genres: Vec<String>,
) -> Result<(), TauriChadError> {
    add_items::<table::Genre>(&fetcher, &game_id, &genres).await
}

#[tauri::command]
pub async fn database_add_languages(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game_id: database::table::GameId,
    languages: Vec<String>,
) -> Result<(), TauriChadError> {
    add_items::<table::Language>(&fetcher, &game_id, &languages).await
}

#[tauri::command]
pub async fn database_add_tags(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game_id: database::table::GameId,
    tags: Vec<String>,
) -> Result<(), TauriChadError> {
    add_items::<table::Genre>(&fetcher, &game_id, &tags).await
}

async fn delete_items<T: table::Item + Serialize>(
    fetcher: &DatabaseFetcher,
    game_id: &database::table::GameId,
    items: &[String],
) -> Result<(), TauriChadError> {
    fetcher
        .delete_items::<T>(game_id, items)
        .await
        .map_err(|e| TauriChadError::from(e))
}

#[tauri::command]
pub async fn database_delete_genres(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game_id: database::table::GameId,
    genres: Vec<String>,
) -> Result<(), TauriChadError> {
    delete_items::<table::Genre>(&fetcher, &game_id, &genres).await
}

#[tauri::command]
pub async fn database_delete_languages(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game_id: database::table::GameId,
    languages: Vec<String>,
) -> Result<(), TauriChadError> {
    delete_items::<table::Language>(&fetcher, &game_id, &languages).await
}

#[tauri::command]
pub async fn database_delete_tags(
    fetcher: tauri::State<'_, DatabaseFetcher>,
    game_id: database::table::GameId,
    tags: Vec<String>,
) -> Result<(), TauriChadError> {
    delete_items::<table::Genre>(&fetcher, &game_id, &tags).await
}
