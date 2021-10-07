#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;

use chad_rs::{
    config::Config, library::LibraryFetcher,
};
use tauri::async_runtime::Mutex;

fn main() {
    // Should improve performance
    std::env::set_var("WEBKIT_FORCE_COMPOSITING_MODE", "1");
    let config = Config::new();

    let library = LibraryFetcher::new();
    let _ = config.save();

    tauri::Builder::default()
        .manage(Mutex::new(library))
        .manage(Mutex::new(config))
        .invoke_handler(tauri::generate_handler![
            // Library
            command::library::library_get_games,
            command::library::library_reload_games,
            command::library::library_run_game,
            command::library::library_open_terminal,
            command::library::library_open_folder,
            command::library::library_set_banner,
            command::library::library_remove_banner,
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
            // Misc
            command::misc_get_reqs_markdown,
            command::misc_get_wiki_page,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
