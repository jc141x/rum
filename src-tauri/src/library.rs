use crate::config::Config;
use crate::util::ChadError;
use serde::Serialize;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tauri::async_runtime::Mutex;
use titlecase::titlecase;
use std::io::{Read, BufRead, BufReader};
use tauri::Manager;

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    id: usize,
    name: String,
    executable_path: PathBuf,
    banner: Option<PathBuf>,
    data_path: PathBuf,
    log_file: PathBuf,
    config_file: PathBuf,
}

impl Game {
    pub fn new(config: &Config, id: usize, executable_path: PathBuf) -> Self {
        let slug: String = executable_path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .into();
        let mut name = slug.clone();
        name = name.replace(".", " ");
        name = name.replace("_", " ");
        name = name.replace("-", " ");
        name = titlecase(&name).trim().into();

        let data_path = config.data_path.join("library").join(slug);
        let _ = std::fs::create_dir_all(&data_path);

        let banner = if data_path.join("banner.png").exists() {
            Some(data_path.join("banner.png"))
        } else {
            None // TODO Fetch banner
        };

        let config_file = data_path.join("game.yaml");
        let log_file = executable_path.parent().unwrap().join("chad.log");

        Self {
            id,
            name,
            executable_path,
            banner,
            data_path,
            log_file,
            config_file,
        }
    }

    pub fn launch(&self) -> Result<Box<dyn Read>, ChadError> {
        let child = Command::new(&self.executable_path)
            .current_dir(self.executable_path.parent().unwrap())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(Box::new(child.stdout.unwrap()))
    }
}

#[derive(Debug, Default)]
pub struct LibraryFetcher {
    games: Vec<Game>,
}

impl LibraryFetcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_games(&mut self, config: &Config) {
        self.games = Vec::new();
        let mut id = 0;

        if let Ok(entries) = config.library_path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if entry.path().join("start.sh").exists() {
                                self.games
                                    .push(Game::new(&config, id, entry.path().join("start.sh")));
                                id += 1;
                            } else if entry.path().join("start").exists() {
                                self.games
                                    .push(Game::new(&config, id, entry.path().join("start")));
                                id += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn iter_games<'a>(&'a self) -> impl Iterator<Item = &'a Game> {
        self.games.iter()
    }

    pub fn get_games<'a>(&'a self) -> Vec<&'a Game> {
        self.iter_games().collect()
    }

    pub fn get_games_cloned(&self) -> Vec<Game> {
        self.iter_games().cloned().collect()
    }

    pub fn get_game<'a>(&'a self, index: usize) -> Option<&'a Game> {
        self.games.get(index)
    }
}

fn handle_stdout(app_handle: tauri::AppHandle, stdout: Box<dyn Read>) -> Result<(), ChadError> {
    let mut reader = BufReader::new(stdout);

    loop {
        let mut line_buf = String::new();

        if let Ok(status) = reader.read_line(&mut line_buf) {
            if status == 0 {
                break;
            }

            app_handle.emit_all("game_log", &line_buf)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn run_game(
    index: usize,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
    app_handle: tauri::AppHandle,
) -> Result<(), ChadError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            let stdout = game.launch()?;
            handle_stdout(app_handle, stdout)?;
            Ok(())
        })
        .unwrap_or(Err(ChadError::new("Game not found".into())))
}

#[tauri::command]
pub async fn get_local_games(
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<Vec<Game>, ChadError> {
    let fetcher = fetcher.lock().await;
    Ok(fetcher.get_games_cloned())
}

#[tauri::command]
pub async fn reload_local_games(
    config: tauri::State<'_, Mutex<Config>>,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<(), ChadError> {
    let mut fetcher = fetcher.lock().await;
    let config = config.lock().await;
    fetcher.load_games(&*config);
    Ok(())
}

#[tauri::command]
pub async fn open_terminal(index: usize, fetcher: tauri::State<'_, Mutex<LibraryFetcher>>, config: tauri::State<'_, Mutex<Config>>) -> Result<(), ChadError> {
    let fetcher = fetcher.lock().await;
    let config = config.lock().await;
    
    fetcher.get_game(index).map(|game| {
        Command::new(&config.terminal)
            .current_dir(game.executable_path.parent().unwrap())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(())
    }).unwrap_or(Err(ChadError::new("Game not found".into())))?;
    Ok(())
}

