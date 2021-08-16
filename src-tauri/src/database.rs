use crate::util::ChadError;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::async_runtime::Mutex;

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

#[derive(Default)]
pub struct GameCache {
    games: HashMap<usize, Game>,
    pages: HashMap<usize, Vec<usize>>,
}

impl GameCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_games(&mut self, games: &[Game]) {
        for game in games {
            self.games.insert(game.id, game.clone());
        }
    }

    pub fn get_game(&self, id: usize) -> Option<&Game> {
        self.games.get(&id)
    }

    pub fn get_page(&self, page: usize) -> Option<Vec<Game>> {
        self.pages.get(&page).map(|games| {
            games
                .iter()
                .filter_map(move |id| self.get_game(*id))
                .cloned()
                .collect()
        })
    }

    pub fn set_page(&mut self, page: usize, games: &[Game]) {
        self.add_games(&games);
        self.pages
            .insert(page, games.iter().map(|game| game.id).collect());
    }
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
    cache: tauri::State<'_, Mutex<GameCache>>,
    fetcher: tauri::State<'_, DatabaseFetcher>,
) -> Result<Vec<Game>, ChadError> {
    // Don't use cache when using search or filters
    let use_cache = opts.search.is_none() && opts.filter_genre.is_none() && opts.filter_language.is_none() && opts.filter_tag.is_none();

    let mut cache = cache.lock().await;
    if let (true, Some(page)) = (use_cache, cache.get_page(opts.page_number)) {
        Ok(page)
    } else {
        let result = fetcher.get_games(&opts).await?;
        if use_cache {
            cache.set_page(opts.page_number, &result);
        }
        Ok(result)
    }
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
