#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chad_launcher::{
    config::Config,
    database::{self, get_magnet, DatabaseFetcher, GetGamesOpts},
    library::{self, LibraryFetcher},
};
use serde::Serialize;
use std::{
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    process::{Command, Stdio},
};
use tauri::{async_runtime::Mutex, Manager};

#[derive(Debug, Serialize)]
pub struct TauriChadError {
    message: String,
}

impl TauriChadError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<T: std::error::Error> From<T> for TauriChadError {
    fn from(error: T) -> TauriChadError {
        TauriChadError {
            message: format!("{}", error),
        }
    }
}

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
async fn run_game(
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
async fn get_local_games(
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<Vec<library::Game>, TauriChadError> {
    let fetcher = fetcher.lock().await;
    Ok(fetcher.get_games_cloned())
}

#[tauri::command]
async fn reload_local_games(
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
async fn open_terminal(
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

#[tauri::command]
async fn get_games(
    opts: GetGamesOpts,
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<database::Game>, TauriChadError> {
    fetcher.get_games(&opts).await.map_err(|e| e.into())
}

#[tauri::command]
async fn get_genres(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher.get_items("get_genres").await.map_err(|e| e.into())
}

#[tauri::command]
async fn get_languages(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher
        .get_items("get_languages")
        .await
        .map_err(|e| e.into())
}

#[tauri::command]
async fn get_tags(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, TauriChadError> {
    fetcher.get_items("get_tags").await.map_err(|e| e.into())
}

#[tauri::command]
async fn open_magnet(game: database::Game) -> Result<(), TauriChadError> {
    let magnet = get_magnet(&game);
    Command::new("xdg-open").arg(magnet).spawn()?;
    Ok(())
}

#[tauri::command]
async fn save_config(config: tauri::State<'_, Mutex<Config>>) -> Result<(), TauriChadError> {
    config.lock().await.save()?;
    Ok(())
}

#[tauri::command]
async fn set_config(
    new_config: Config,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriChadError> {
    config.lock().await.set_config(new_config);
    Ok(())
}

#[tauri::command]
async fn set_config_data_path(
    data_path: PathBuf,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriChadError> {
    config.lock().await.set_data_path(&data_path);
    Ok(())
}

#[tauri::command]
async fn set_config_library_paths(
    library_paths: Vec<PathBuf>,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriChadError> {
    config.lock().await.set_library_paths(&library_paths);
    Ok(())
}

#[tauri::command]
async fn set_config_terminal(
    terminal: String,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriChadError> {
    config.lock().await.set_terminal(&terminal);
    Ok(())
}

fn main() {
    let config = Config::new();
    let client = DatabaseFetcher::new();
    let library = LibraryFetcher::new();
    let _ = config.save();

    tauri::Builder::default()
        .manage(client)
        .manage(Mutex::new(library))
        .manage(Mutex::new(config))
        .invoke_handler(tauri::generate_handler![
            // Database
            get_games,
            get_genres,
            get_languages,
            get_tags,
            open_magnet,
            // Library
            get_local_games,
            reload_local_games,
            run_game,
            open_terminal,
            // Config
            save_config,
            set_config,
            set_config_data_path,
            set_config_library_paths,
            set_config_terminal,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
