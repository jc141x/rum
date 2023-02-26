use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub title: String,
    pub window: Window,
}

#[derive(Deserialize)]
pub struct Window {
    pub width: i32,
    pub height: i32,
    pub url: String,
}
