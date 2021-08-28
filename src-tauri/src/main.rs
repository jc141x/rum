#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;

use crate::command::AppState;
use chad_launcher::{
    config::Config, database::DatabaseFetcher, download::DownloadManager, library::LibraryFetcher,
};
use tauri::async_runtime::Mutex;

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
            command::database::database_get_games,
            command::database::database_get_genres,
            command::database::database_get_languages,
            command::database::database_get_tags,
            // Library
            command::library::library_get_games,
            command::library::library_reload_games,
            command::library::library_run_game,
            command::library::library_open_terminal,
            // Config
            command::config::config_save,
            command::config::config_set,
            command::config::config_set_data_path,
            command::config::config_set_library_paths,
            command::config::config_set_terminal,
            command::config::config_get,
            command::config::config_get_data_path,
            command::config::config_get_library_paths,
            command::config::config_get_terminal,
            // Downloads
            command::download::download_init_clients,
            command::download::download_list_clients,
            command::download::download_list_downloads,
            command::download::download_list_all_downloads,
            command::download::download_add_magnet,
            command::download::download_add_game,
            command::download::download_pause,
            command::download::download_resume,
            command::download::download_remove,
            command::download::download_get_status,
            command::download::download_add_qbittorrent_client,
            command::download::download_create_deluge_client,
            command::download::download_deluge_connect_daemon,
            command::download::download_list_deluge_hosts,
            // Misc
            command::misc_get_reqs_markdown,
            command::misc_open_magnet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
