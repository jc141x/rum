#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
pub mod util;

use database::{DatabaseFetcher, get_games, get_genres, get_languages, get_tags};

fn main() {
    let client = DatabaseFetcher::new();

    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![get_games, get_genres, get_languages, get_tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
