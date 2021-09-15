pub mod config;
pub mod database;
pub mod download;
pub mod library;

use chad_rs::{
    database::{get_magnet, Game},
    download::TorrentClientConfig,
};
use serde::Serialize;
use std::process::Command;

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
pub struct AppState {
    pub current_client: Option<chad_torrent::TorrentClient>,
    pub current_config: Option<TorrentClientConfig>,
}

#[tauri::command]
pub async fn misc_get_reqs_markdown() -> Result<String, TauriChadError> {
    Ok(reqwest::get("https://rentry.co/johncena141-reqs/raw")
        .await?
        .text()
        .await?)
}

#[tauri::command]
pub async fn misc_get_wiki_page(page: String) -> Result<String, TauriChadError> {
    Ok(reqwest::get(format!(
        "https://gitlab.com/Gnurur/chad_launcher/-/wikis/{}.md",
        page
    ))
    .await?
    .text()
    .await?)
}

#[tauri::command]
pub async fn misc_open_magnet(game: Game) -> Result<(), TauriChadError> {
    let magnet = get_magnet(&game);
    Command::new("xdg-open").arg(magnet).spawn()?;
    Ok(())
}
