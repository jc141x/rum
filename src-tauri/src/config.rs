
use std::path::PathBuf;

pub struct Config {
    pub data_path: PathBuf,
    pub library_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            data_path: dirs::data_dir().unwrap().join("chad_launcher"),
            library_path: dirs::home_dir().unwrap().join("Games/chad_launcher"),
        }
    }
}
