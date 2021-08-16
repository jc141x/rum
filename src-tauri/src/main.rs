#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
mod library;
pub mod config;
pub mod util;

use tauri::async_runtime::Mutex;
use database::{get_games, get_genres, get_languages, get_tags, DatabaseFetcher};
use library::{get_local_games, reload_local_games, run_game, LibraryFetcher};
use config::Config;

fn main() {
    let config = Config::new();
    let client = DatabaseFetcher::new();
    let mut library = LibraryFetcher::new();
    library.load_games(&config); 

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

            // Library
            get_local_games,
            reload_local_games,
            run_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
