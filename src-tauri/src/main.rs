#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};
use postgrest::Postgrest;
use std::error::Error;
use tauri::async_runtime::Mutex;
use std::collections::HashMap;

const API_KEY: &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A";

#[derive(Serialize, Deserialize, Clone)]
struct Game {
    id: usize,
    leetx_id: usize,
    name: String,
    version: String,
    #[serde(rename = "type")]
    type_: String,
    hash: String,
    description: String,
    nsfw: bool,
    banner_path: Option<String>,
    genres: Vec<String>,
    tags: Vec<String>,
    languages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GetGamesOpts {
    page_number: usize,
    page_size: usize,
}

#[derive(Debug, Serialize)]
struct ChadError {
    message: String,
}

impl<T: Error> From<T> for ChadError {
    fn from(error: T) -> ChadError {
        ChadError { message: format!("{}", error) }
    }
}

#[tauri::command]
async fn get_games(opts: GetGamesOpts, games: tauri::State<'_, Mutex<HashMap<usize, Vec<Game>>>>, client: tauri::State<'_, Postgrest>) -> Result<Vec<Game>, ChadError> {
    let mut games = games.lock().await;
    if let Some(page) = games.get(&opts.page_number) {
        println!("Using cache");
        Ok(page.to_vec())
    } else {
        println!("Fetching new games");
        let result = client
            .rpc("get_games", &serde_json::to_string(&opts)?)
            .execute()
            .await?
            .json::<Vec<Game>>()
            .await?;

        games.insert(opts.page_number, result.clone());

        println!("Done");
        Ok(result)
    }
}

#[tauri::command]
async fn get_genres(client: tauri::State<'_, Postgrest>) -> Result<Vec<String>, ChadError> {
    let result = client
        .rpc("get_genres", "")
        .execute()
        .await?
        .json::<Vec<String>>()
        .await?;

    Ok(result)
}

fn main() {
    let client = Postgrest::new("https://bkftwbhopivmrgzcagus.supabase.co/rest/v1/")
        .insert_header("apikey", API_KEY);

    let games: Mutex<HashMap<usize, Vec<Game>>> = Mutex::new(HashMap::new());

    tauri::Builder::default()
        .manage(client)
        .manage(games)
        .invoke_handler(tauri::generate_handler![get_games, get_genres])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
