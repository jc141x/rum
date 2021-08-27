#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chad_launcher::{
    config::Config,
    database::{self, get_magnet, DatabaseFetcher, GetGamesOpts},
    download::{DownloadManager, TorrentClientConfig},
    library::{self, LibraryFetcher},
};
use chad_torrent::TorrentBackend;
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

#[derive(Default)]
struct AppState {
    current_client: Option<chad_torrent::TorrentClient>,
    current_config: Option<TorrentClientConfig>,
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
async fn get_config(config: tauri::State<'_, Mutex<Config>>) -> Result<Config, TauriChadError> {
    Ok(config.lock().await.clone())
}

#[tauri::command]
async fn get_config_data_path(
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<PathBuf, TauriChadError> {
    Ok(config.lock().await.data_path().to_owned())
}

#[tauri::command]
async fn get_config_library_paths(
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<Vec<PathBuf>, TauriChadError> {
    Ok(config
        .lock()
        .await
        .library_paths()
        .iter()
        .map(|p| p.to_owned())
        .collect())
}

#[tauri::command]
async fn get_config_terminal(
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<String, TauriChadError> {
    Ok(config.lock().await.terminal().into())
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

#[tauri::command]
async fn init_download_clients(
    config: tauri::State<'_, Mutex<Config>>,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    download
        .lock()
        .await
        .load_config(&*config.lock().await)
        .await?;
    Ok(())
}

#[tauri::command]
async fn list_clients(
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<Vec<String>, TauriChadError> {
    Ok(download.lock().await.clients().cloned().collect())
}

fn get_backend(
    client: String,
    download: &DownloadManager,
) -> Result<&chad_torrent::TorrentClient, TauriChadError> {
    if let Some(backend) = download.client(&client) {
        Ok(backend)
    } else {
        Err(TauriChadError::new("Client not found".into()))
    }
}

#[tauri::command]
async fn list_downloads(
    client: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<Vec<chad_torrent::Torrent>, TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    let list = backend
        .list(Some("chad"))
        .await
        .map_err(|e| TauriChadError::from(&*e))?;
    Ok(list)
}

#[tauri::command]
async fn add_magnet(
    client: String,
    magnet: String,
    options: chad_torrent::Options,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<String, TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .add_magnet(&magnet, options)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
async fn pause_download(
    client: String,
    torrent_id: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .pause(&torrent_id)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
async fn resume_download(
    client: String,
    torrent_id: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .resume(&torrent_id)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
async fn remove_download(
    client: String,
    torrent_id: String,
    remove_files: bool,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .remove_torrent(&torrent_id, remove_files)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
async fn get_download_status(
    client: String,
    torrent_id: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<chad_torrent::Torrent, TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .torrent(&torrent_id)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
async fn add_qbittorrent_client(
    name: String,
    options: chad_launcher::download::QBittorrentConfig,
    config: tauri::State<'_, Mutex<Config>>,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let mut download = download.lock().await;
    let client = download.qbittorrent_connect(&options).await?;
    download.add_client(&name, client);
    let mut config = config.lock().await;
    config.insert_download_client(name, TorrentClientConfig::QBittorrent(options));
    Ok(config.save()?)
}

#[tauri::command]
async fn deluge_connect_daemon(
    name: String,
    daemon_id: String,
    config: tauri::State<'_, Mutex<Config>>,
    download: tauri::State<'_, Mutex<DownloadManager>>,
    app_state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), TauriChadError> {
    let mut app_state = app_state.lock().await;
    if let (
        Some(chad_torrent::TorrentClient::DelugeBackend(backend)),
        Some(TorrentClientConfig::Deluge(mut options)),
    ) = (
        app_state.current_client.take(),
        app_state.current_config.take(),
    ) {
        backend
            .connect(&daemon_id)
            .await
            .map_err(|e| TauriChadError::from(&*e))?;
        let mut download = download.lock().await;
        download.add_client(&name, backend);
        options.daemon_id = Some(daemon_id);
        let mut config = config.lock().await;
        config.insert_download_client(name, TorrentClientConfig::Deluge(options));
        Ok(config.save()?)
    } else {
        Err(TauriChadError::new(
            "Failed to connect to deluge daemon: no active Web UI connection".into(),
        ))
    }
}

#[tauri::command]
async fn create_deluge_client(
    options: chad_launcher::download::DelugeConfig,
    download: tauri::State<'_, Mutex<DownloadManager>>,
    app_state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let client = download.deluge_connect(&options).await?;
    let mut app_state = app_state.lock().await;
    app_state.current_client = Some(client.into());
    app_state.current_config = Some(TorrentClientConfig::Deluge(options));
    Ok(())
}

#[tauri::command]
async fn list_deluge_hosts(
    app_state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<chad_torrent::backend::deluge::Host>, TauriChadError> {
    let app_state = app_state.lock().await;
    if let Some(chad_torrent::TorrentClient::DelugeBackend(backend)) = &app_state.current_client {
        backend
            .list_hosts()
            .await
            .map_err(|e| TauriChadError::from(&*e))
    } else {
        Err(TauriChadError::new(
            "Failed to list deluge hosts: no active Web UI connection".into(),
        ))
    }
}

#[tauri::command]
async fn get_reqs_markdown() -> Result<String, TauriChadError> {
    Ok(reqwest::get("https://rentry.co/johncena141-reqs/raw")
        .await?
        .text()
        .await?)
}

fn main() {
    // Should improve performance
    std::env::set_var("WEBKIT_FORCE_COMPOSITING_MODE", "1");

    let config = Config::new();
    let client = DatabaseFetcher::new();
    let library = LibraryFetcher::new();
    let download = DownloadManager::new();
    let state = AppState::default();
    let _ = config.save();

    tauri::Builder::default()
        .manage(client)
        .manage(Mutex::new(download))
        .manage(Mutex::new(library))
        .manage(Mutex::new(config))
        .manage(Mutex::new(state))
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
            get_config,
            get_config_data_path,
            get_config_library_paths,
            get_config_terminal,
            // Downloads
            init_download_clients,
            list_clients,
            list_downloads,
            add_magnet,
            pause_download,
            resume_download,
            remove_download,
            get_download_status,
            add_qbittorrent_client,
            create_deluge_client,
            deluge_connect_daemon,
            list_deluge_hosts,
            // Misc
            get_reqs_markdown,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
