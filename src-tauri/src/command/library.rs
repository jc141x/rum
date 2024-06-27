use rumlibrs::{
    config::Config,
    library::{self, LibraryFetcher},
};
use std::{
    fs::{copy, remove_file, read_to_string},
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio},
};
use tokio::sync::Mutex;
use std::sync::Arc;

/// Custom error type for handling errors in the library module.
#[derive(Debug)]
pub enum RumError {
    /// Represents IO errors.
    IoError(std::io::Error),
    /// Represents other errors with a custom message.
    Other(String),
}

impl From<std::io::Error> for RumError {
    /// Converts a `std::io::Error` into a `RumError::IoError`.
    fn from(err: std::io::Error) -> Self {
        RumError::IoError(err)
    }
}

/// Handles the stdout of a process and invokes a callback for each line read.
///
/// # Arguments
///
/// * `stdout` - A boxed trait object that implements the `Read` trait.
/// * `callback` - A callback function that takes a string slice and returns nothing.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
fn handle_stdout(
    stdout: Box<dyn Read>,
    callback: impl Fn(&str) + Send + Sync + 'static,
) -> Result<(), RumError> {
    let mut reader = BufReader::new(stdout);

    loop {
        let mut line_buf = String::new();

        if let Ok(status) = reader.read_line(&mut line_buf) {
            if status == 0 {
                break;
            }

            callback(&line_buf);
        }
    }

    Ok(())
}

/// Runs a game by its index and executes a script.
///
/// # Arguments
///
/// * `index` - The index of the game in the library.
/// * `script` - The script to execute for the game.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
/// * `callback` - A callback function to handle the stdout of the game.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
pub async fn library_run_game(
    index: usize,
    script: String,
    fetcher: Arc<Mutex<LibraryFetcher>>,
    callback: impl Fn(&str) + Send + Sync + 'static,
) -> Result<(), RumError> {
    let mut fetcher = fetcher.lock().await;
    fetcher
        .get_game(index)
        .map(|game| {
            let stdout = game.launch(script)?;
            handle_stdout(stdout, callback)?;
            Ok(())
        })
        .unwrap_or(Err(RumError::Other("Game not found".into())))
}

/// Retrieves a list of games from the library.
///
/// # Arguments
///
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
///
/// # Returns
///
/// * `Result<Vec<library::Game>, RumError>` - Returns a vector of games if successful, otherwise returns an error.
pub async fn library_get_games(
    fetcher: Arc<Mutex<LibraryFetcher>>,
) -> Result<Vec<library::Game>, RumError> {
    let fetcher = fetcher.lock().await;
    Ok(fetcher.get_games_cloned())
}

/// Reloads the games in the library using the current configuration.
///
/// # Arguments
///
/// * `config` - An `Arc<Mutex<Config>>` to manage the state of the configuration.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
pub async fn library_reload_games(
    config: Arc<Mutex<Config>>,
    fetcher: Arc<Mutex<LibraryFetcher>>,
) -> Result<(), RumError> {
    let mut fetcher = fetcher.lock().await;
    let config = config.lock().await;
    fetcher.load_games(&*config);
    Ok(())
}

/// Opens a terminal in the directory of the game specified by the index.
///
/// # Arguments
///
/// * `index` - The index of the game in the library.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
/// * `config` - An `Arc<Mutex<Config>>` to manage the state of the configuration.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
pub async fn library_open_terminal(
    index: usize,
    fetcher: Arc<Mutex<LibraryFetcher>>,
    config: Arc<Mutex<Config>>,
) -> Result<(), RumError> {
    let fetcher = fetcher.lock().await;
    let config = config.lock().await;

    fetcher
        .get_game(index)
        .map(|game| {
            Command::new(&config.terminal())
                .current_dir(game.executable_dir())
                .stdout(Stdio::piped())
                .spawn()?;
            Ok(())
        })
        .unwrap_or(Err(RumError::Other("Game not found".into())))?;
    Ok(())
}

/// Opens the folder of the game specified by the index.
///
/// # Arguments
///
/// * `index` - The index of the game in the library.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
/// * `callback` - A callback function to handle the stdout of the folder opening process.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
pub async fn library_open_folder(
    index: usize,
    fetcher: Arc<Mutex<LibraryFetcher>>,
    callback: impl Fn(&str) + Send + Sync + 'static,
) -> Result<(), RumError> {
    let mut fetcher = fetcher.lock().await;
    fetcher
        .get_game(index)
        .map(|game| {
            let cmd = Command::new("xdg-open")
                .arg(game.executable_dir())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to open");
            let stdout = Box::new(cmd.stdout.unwrap());
            handle_stdout(stdout, callback)?;
            Ok(())
        })
        .unwrap_or(Err(RumError::Other("Game not found".into())))
}

/// Sets the banner for the game specified by the index.
///
/// # Arguments
///
/// * `index` - The index of the game in the library.
/// * `path` - The path to the banner image file.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
pub async fn library_set_banner(
    index: usize,
    path: String,
    fetcher: Arc<Mutex<LibraryFetcher>>,
) -> Result<(), RumError> {
    let mut fetcher = fetcher.lock().await;
    fetcher
        .get_game(index)
        .map(|game| {
            copy(path, game.data_path.join("banner.png"))?;
            Ok(())
        })
        .unwrap_or(Err(RumError::Other("Game not found".into())))
}

/// Removes the banner for the game specified by the index.
///
/// # Arguments
///
/// * `index` - The index of the game in the library.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
pub async fn library_remove_banner(
    index: usize,
    fetcher: Arc<Mutex<LibraryFetcher>>,
) -> Result<(), RumError> {
    let mut fetcher = fetcher.lock().await;
    fetcher
        .get_game(index)
        .map(|game| {
            remove_file(game.data_path.join("banner.png"))?;
            Ok(())
        })
        .unwrap_or(Err(RumError::Other("Game not found".into())))
}

/// Saves the configuration for the game specified by the index.
///
/// # Arguments
///
/// * `index` - The index of the game in the library.
/// * `wrapper` - An optional wrapper script for the game.
/// * `env` - An optional list of environment variables for the game.
/// * `args` - An optional string of arguments for the game.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
///
/// # Returns
///
/// * `Result<(), RumError>` - Returns `Ok(())` if successful, otherwise returns an error.
pub async fn library_save_game_config(
    index: usize,
    wrapper: Option<String>,
    env: Option<Vec<String>>,
    args: Option<String>,
    fetcher: Arc<Mutex<LibraryFetcher>>,
) -> Result<(), RumError> {
    let mut fetcher = fetcher.lock().await;
    fetcher
        .get_game(index)
        .map(|game| {
            game.save_config(wrapper, env, args)?;
            Ok(())
        })
        .unwrap_or(Err(RumError::Other("Game not found".into())))
}

/// Reads the configuration for the game specified by the index.
///
/// # Arguments
///
/// * `index` - The index of the game in the library.
/// * `fetcher` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
///
/// # Returns
///
/// * `Result<String, RumError>` - Returns the configuration as a string if successful, otherwise returns an error.
pub async fn library_read_game_config(
    index: usize,
    fetcher: Arc<Mutex<LibraryFetcher>>,
) -> Result<String, RumError> {
    let mut fetcher = fetcher.lock().await;
    fetcher
        .get_game(index)
        .map(|game| {
            Ok(read_to_string(game.config_file()).unwrap_or_default())
        })
        .unwrap_or(Err(RumError::Other("Game not found".into())))
}
