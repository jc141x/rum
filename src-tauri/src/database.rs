use crate::util::ChadError;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

const API_KEY: &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A";

const TRACKERS: &[&'static str] = &[
    "udp://tracker.leechers-paradise.org:6969/announce",
    "udp://tracker.opentrackr.org:1337/announce",
    "udp://tracker.zer0day.to:1337/announce",
    "udp://eddie4.nl:6969/announce",
    "udp://46.148.18.250:2710",
    "udp://opentor.org:2710",
    "http://tracker.dler.org:6969/announce",
    "udp://9.rarbg.me:2730/announce",
    "udp://9.rarbg.to:2770/announce",
    "udp://tracker.pirateparty.gr:6969/announce",
    "http://retracker.local/announce",
    "http://retracker.ip.ncnet.ru/announce",
    "udp://exodus.desync.com:6969/announce",
    "udp://ipv4.tracker.harry.lu:80/announce",
    "udp://open.stealth.si:80/announce",
    "udp://coppersurfer.tk:6969/announce"
];

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

    pub async fn find_banner(&self, game_name: &str) -> Result<String, ChadError> {
        let result = self
            .client
            .rpc("get_games", format!("{{ \"search\": \"{}\" }}", game_name))
            .execute()
            .await?
            .json::<Vec<Game>>()
            .await?;

        if let Some(game) = result.get(0) {
            if let Some(path) = &game.banner_path {
                Ok(path.into())
            } else {
                Err(ChadError::message("No banner found"))
            }
        } else {
            Err(ChadError::message("No game found"))
        }
    }
}

pub fn get_magnet(game: &Game) -> String {
    let mut magnet = format!("magnet:?xt=urn:btih:{}&dn={}", game.hash, game.name);
    for tracker in TRACKERS {
        magnet.push_str(&format!("&tr={}", tracker));
    }
    magnet
}

