use crate::util::ChadError;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

const API_KEY: &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGamesOpts {
    page_number: usize,
    page_size: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    filter_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
}

pub struct DatabaseFetcher {
    client: Postgrest,
}

impl DatabaseFetcher {
    pub fn new() -> Self {
        Self {
            client: Postgrest::new("https://bkftwbhopivmrgzcagus.supabase.co/rest/v1/")
                .insert_header("apikey", API_KEY),
        }
    }

    pub async fn get_games(&self, opts: &GetGamesOpts) -> Result<Vec<Game>, ChadError> {
        let result = self
            .client
            .rpc("get_games", &serde_json::to_string(&opts)?)
            .execute()
            .await?
            .json::<Vec<Game>>()
            .await?;

        Ok(result)
    }

    pub async fn get_items(&self, table_name: &str) -> Result<Vec<String>, ChadError> {
        let result = self
            .client
            .rpc(table_name, "")
            .execute()
            .await?
            .json::<Vec<String>>()
            .await?;

        Ok(result)
    }
}

#[tauri::command]
pub async fn get_games(
    opts: GetGamesOpts,
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<Game>, ChadError> {
    fetcher.get_games(&opts).await
}

#[tauri::command]
pub async fn get_genres(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, ChadError> {
    fetcher.get_items("get_genres").await
}

#[tauri::command]
pub async fn get_languages(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, ChadError> {
    fetcher.get_items("get_languages").await
}

#[tauri::command]
pub async fn get_tags(
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<String>, ChadError> {
    fetcher.get_items("get_tags").await
}
