pub mod config;
pub mod library;

use serde::Serialize;

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

