use crate::util::ChadError;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf, Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    data_path: PathBuf,
    library_paths: Vec<PathBuf>,
    terminal: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_path: dirs::data_dir().unwrap().join("chad_launcher"),
            library_paths: vec![dirs::home_dir().unwrap().join("Games/chad_launcher")],
            terminal: "alacritty".into(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir().unwrap().join("chad_launcher");
        let _ = std::fs::create_dir_all(&config_dir);
        let config_file = config_dir.join("config.json");
        let config_data = std::fs::read_to_string(&config_file);
        if let Ok(config) = config_data.and_then(|data| {
            let json = serde_json::from_str(&data)?;
            Ok(json)
        }) {
            config
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), ChadError> {
        let config_dir = dirs::config_dir().unwrap().join("chad_launcher");
        let _ = std::fs::create_dir_all(&config_dir);
        std::fs::write(
            &config_dir.join("config.json"),
            serde_json::to_string_pretty(&self)?,
        )?;
        Ok(())
    }

    pub fn data_path(&self) -> &Path {
        &self.data_path
    }

    pub fn library_paths(&self) -> &[PathBuf] {
        &self.library_paths
    }

    pub fn terminal(&self) -> &str {
        &self.terminal
    }

    pub fn set_data_path(&mut self, data_path: &Path) {
        self.data_path = data_path.into();
    }

    pub fn set_library_paths(&mut self, library_paths: &[PathBuf]) {
        self.library_paths = library_paths.into();
    }

    pub fn set_terminal(&mut self, terminal: &str) {
        self.terminal = terminal.into()
    }

    pub fn set_config(&mut self, other: Config) {
        self.data_path = other.data_path;
        self.library_paths = other.library_paths;
        self.terminal = other.terminal;
    }
}
