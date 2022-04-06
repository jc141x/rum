use super::TauriRumError;
use rumlibrs::{
    config::Config,
    library::{self, LibraryFetcher, Gameconfig},
};
use std::{
    fs::{copy, remove_file, read_to_string },
    io::{BufRead, BufReader, Read },
    process::{Command, Stdio},
};
use tauri::{async_runtime::Mutex, Manager};
use steamgriddb_api::Client;
use steamgriddb_api::query_parameters::{QueryType::Hero};
use dotenv;
use image;

fn get_sgdb_api_key() -> String {
    dotenv::dotenv().ok();
    std::env::var("SGDB_API_KEY").expect("SGDB_API_KEY not found")
}

fn handle_stdout(
    app_handle: tauri::AppHandle,
    stdout: Box<dyn Read>,
) -> Result<(), TauriRumError> {
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
pub async fn library_run_game(
    index: usize,
    script: String,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
    app_handle: tauri::AppHandle,
) -> Result<(), TauriRumError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            let stdout = game.launch(script)?;
            handle_stdout(app_handle, stdout)?;
            Ok(())
        })
        .unwrap_or(Err(TauriRumError::new("Game not found".into())))
}

#[tauri::command]
pub async fn library_get_games(
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<Vec<library::Game>, TauriRumError> {
    let fetcher = fetcher.lock().await;
    Ok(fetcher.get_games_cloned())
}

#[tauri::command]
pub async fn library_reload_games(
    config: tauri::State<'_, Mutex<Config>>,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<(), TauriRumError> {
    let mut fetcher = fetcher.lock().await;
    let config = config.lock().await;
    fetcher.load_games(&*config);
    // TODO: GUI would need a wait to manually
    // load banners, because we don't want to block the GUI while loading banners
    Ok(())
}

#[tauri::command]
pub async fn library_open_terminal(
    index: usize,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
    config: tauri::State<'_, Mutex<Config>>,
) -> Result<(), TauriRumError> {
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
        .unwrap_or(Err(TauriRumError::new("Game not found".into())))?;
    Ok(())
}

#[tauri::command]
pub async fn library_open_folder(
    index: usize,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
    app_handle: tauri::AppHandle,
) -> Result<(), TauriRumError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            let cmd = Command::new("xdg-open")
                .arg(game.executable_dir())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to open");
            let stdout = Box::new(cmd.stdout.unwrap());
            handle_stdout(app_handle, stdout)?;
            Ok(())
        })
        .unwrap_or(Err(TauriRumError::new("Game not found".into())))
}

#[tauri::command]
pub async fn library_set_banner(
    index: usize,
    path: String,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<(), TauriRumError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            copy(path, game.data_path.join("banner.png"))?;
            Ok(())
        })
        .unwrap_or(Err(TauriRumError::new("Game not found".into())))
}

#[tauri::command]
pub async fn library_remove_banner(
    index: usize,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<(), TauriRumError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            remove_file(game.data_path.join("banner.png"))?;
            Ok(())
        })
        .unwrap_or(Err(TauriRumError::new("Game not found".into())))
}
#[tauri::command]
pub async fn library_delete_all_banners(
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<(), TauriRumError> {
    for game in fetcher.lock().await.get_games().iter() {
        println!("{}", game.name);
        remove_file(game.data_path.join("banner.png")).unwrap_or(());
    }
    Ok(())
}

#[tauri::command]
pub async fn library_save_game_config(
    index: usize,
    wrapper: Option<String>,
    env: Option<Vec<String>>,
    args: Option<String>,
    sgdb: Option<usize>,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<(), TauriRumError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            game.save_config(wrapper, env, args, sgdb)?;
            Ok(())
        })
        .unwrap_or(Err(TauriRumError::new("Game not found".into())))
}

#[tauri::command]
pub async fn library_read_game_config(
    index: usize,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<String, TauriRumError> {
    fetcher
        .lock()
        .await
        .get_game(index)
        .map(|game| {
            Ok(read_to_string(game.config_file()).unwrap_or_default())
        }).unwrap_or(Err(TauriRumError::new("Game not found".into())))
}

#[tauri::command]
pub async fn library_sgdb_hero_fetch(
    index: usize,
    fetcher: tauri::State<'_, Mutex<LibraryFetcher>>,
) -> Result<String, TauriRumError> {
        match fetcher.lock().await.get_game(index) {
            Some(game) => {
                if game.banner_path.clone().unwrap_or_default().exists() {
                    Ok(game.banner.clone().unwrap())
                }
                else {
                    let mut conf: Gameconfig = serde_json::from_str(&read_to_string(game.config_file()).unwrap_or_default().to_string()).unwrap_or_default();
                    if conf.sgdb == None {
                        conf.sgdb = Some(_name_to_sgdb_id(game.name.clone()).await.unwrap_or_default());
                        game.save_config(conf.wrapper.clone(), conf.env.clone(), conf.args.clone(), conf.sgdb.clone()).unwrap_or_default();
                    }
                    if conf.sgdb != Some(0) {
                        Ok(_sgdb_hero_fetch(conf.sgdb.unwrap(), game.data_path.clone()).await.unwrap_or_default().to_string())
                    }
                    else {
                        Err(TauriRumError::new("No SGDB ID found".into()))
                    }
                }
            }
            None => Err(TauriRumError::new("Game not found".into())),
        }
}

async fn _name_to_sgdb_id(
    name: String
) -> Result<usize, TauriRumError> {
    let client = Client::new(get_sgdb_api_key());
    let games = client.search(&name).await.unwrap_or_default();
    let first_game = games.first().ok_or(TauriRumError::new("No game found".into()))?;
    Ok(first_game.id)
}

async fn _sgdb_hero_fetch(
    id: usize,
    path: std::path::PathBuf,
) -> Result<String, Box<TauriRumError>> {
    let client = Client::new(get_sgdb_api_key());
    let image = client.get_images_for_id(id, &Hero(None)).await.unwrap_or_default();
    let first_image = image.first().ok_or(TauriRumError::new("No image found".into()))?;
    let image_url = first_image.url.clone();
    let img_bytes = reqwest::get(&image_url).await.unwrap().bytes().await.unwrap();
    let image = image::load_from_memory(&img_bytes).unwrap();
    image.save(path.join("./banner.png")).unwrap();
    Ok(image_url)
}