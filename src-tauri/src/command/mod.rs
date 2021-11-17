pub mod config;
pub mod library;

use gilrs::Gilrs;
use serde::Serialize;
use std::{thread, time::Duration};
use tauri::Window;

#[derive(Debug, Serialize)]
pub struct TauriRumError {
    message: String,
}

impl TauriRumError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<T: std::error::Error> From<T> for TauriRumError {
    fn from(error: T) -> TauriRumError {
        TauriRumError {
            message: format!("{}", error),
        }
    }
}

#[tauri::command]
pub async fn misc_get_reqs_markdown() -> Result<String, TauriRumError> {
    Ok(reqwest::get("https://rentry.co/johncena141-reqs/raw")
        .await?
        .text()
        .await?)
}

#[tauri::command]
pub async fn misc_get_wiki_page(page: String) -> Result<String, TauriRumError> {
    Ok(reqwest::get(format!(
        "https://notabug.org/johncena141/rum-wiki/raw/master/{}",
        page
    ))
    .await?
    .text()
    .await?)
}

#[tauri::command]
pub fn misc_init_bg_process(window: Window) {
    static mut STARTED: bool = false;
    unsafe {
        if STARTED {
            return;
        }
    }
    thread::spawn(move || {
        let mut gilrs = Gilrs::new().unwrap();
        unsafe {
            STARTED = true;
        }
        loop {
            while let Some(ev) = gilrs.next_event() {
                if let gilrs::ev::EventType::ButtonPressed(..) = ev.event {
                    window.emit("gamepad", ev.event).unwrap();
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
}
