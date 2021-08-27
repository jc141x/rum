use super::TauriChadError;
use chad_launcher::config::Config;
use chad_launcher::database::DatabaseFetcher;
use chad_launcher::library::{self, LibraryFetcher};
use std::{
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio},
};
use tauri::{async_runtime::Mutex, Manager};

fn handle_stdout(
    app_handle: tauri::AppHandle,
    stdout: Box<dyn Read>,
) -> Result<(), TauriChadError> {
    let mut reader = BufReader::new(stdout);

    loop {
        let mut line_buf = String::new();

        if let Ok(status) = reader.read_line(&mut line_buf) {
            if status == 0 {
                break;
            }

            app_handle.emit_all("game_log", &line_buf)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn library_run_game(
    index: usize,
    script: String,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
    app_handle: tauri::AppHandle,
) -> Result<(), TauriChadError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            let stdout = game.launch(&script)?;
            handle_stdout(app_handle, stdout)?;
            Ok(())
        })
        .unwrap_or(Err(TauriChadError::new("Game not found".into())))
}

#[tauri::command]
pub async fn library_get_games(
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<Vec<library::Game>, TauriChadError> {
    let fetcher = fetcher.lock().await;
    Ok(fetcher.get_games_cloned())
}

#[tauri::command]
pub async fn library_reload_games(
    config: tauri::State<'_, Mutex<Config>>,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
    _database_fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<(), TauriChadError> {
    let mut fetcher = fetcher.lock().await;
    let config = config.lock().await;
    fetcher.load_games(&*config);
    //fetcher.download_banners(&*database_fetcher).await;
    // TODO: GUI would need a wait to manually
    // load banners, because we don't want to block the GUI while loading banners
    Ok(())
}

#[tauri::command]
pub async fn library_open_terminal(
    index: usize,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriChadError> {
    let fetcher = fetcher.lock().await;
    let config = config.lock().await;

    fetcher
        .get_game(index)
        .map(|game| {
            Command::new(&config.terminal())
                .current_dir(game.executable_dir())
                .stdout(Stdio::piped())
                .spawn()?;
            Ok(())
        })
        .unwrap_or(Err(TauriChadError::new("Game not found".into())))?;
    Ok(())
}
