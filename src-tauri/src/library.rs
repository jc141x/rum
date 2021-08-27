use crate::{config::Config, database::DatabaseFetcher, util::ChadError};
use futures::future::join_all;
use serde::Serialize;
use std::os::unix::fs::PermissionsExt;
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use titlecase::titlecase;

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    id: usize,
    name: String,
    executable_dir: PathBuf,
    scripts: Vec<String>,
    banner_path: Option<PathBuf>,
    banner: Option<String>,
    data_path: PathBuf,
    log_file: PathBuf,
    config_file: PathBuf,
}

fn load_banner(banner_path: &Path) -> Option<String> {
    std::fs::read(banner_path)
        .ok()
        .map(|b| base64::encode(b))
        .map(|b64| format!("data:image/png;base64,{}", b64))
}

fn find_scripts(executable_dir: &Path) -> Result<Vec<String>, ChadError> {
    Ok(executable_dir
        // Try to read the directory
        .read_dir()?
        // Filter out errors
        .filter_map(|e| e.ok())
        // Only check files
        .filter(|e| {
            e.file_type()
                .map(|f| f.is_file() || f.is_symlink())
                .unwrap_or(false)
        })
        // Find executable files
        .filter(|e| {
            std::fs::metadata(e.path())
                .map(|m| m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
        // Map DirEntry to String
        .filter_map(|d| d.file_name().to_str().map(|s| s.into()))
        // Collect into a Vec
        .collect())
}

impl Game {
    pub fn new(config: &Config, id: usize, executable_dir: PathBuf) -> Self {
        let slug: String = executable_dir.file_name().unwrap().to_str().unwrap().into();
        let mut name = slug.clone();
        name = name.replace(".", " ");
        name = name.replace("_", " ");
        name = name.replace("-", " ");
        name = titlecase(&name).trim().into();

        let data_path = config.data_path().join("library").join(slug);
        let _ = std::fs::create_dir_all(&data_path);

        let banner_path = if data_path.join("banner.png").exists() {
            Some(data_path.join("banner.png"))
        } else {
            None // TODO Fetch banner
        };

        let banner = banner_path.as_ref().and_then(|p| load_banner(&p));

        let config_file = data_path.join("game.yaml");
        let log_file = executable_dir.join("chad.log");
        let scripts = find_scripts(&executable_dir).unwrap_or(Vec::new());

        Self {
            id,
            name,
            executable_dir,
            scripts,
            banner_path,
            banner,
            data_path,
            log_file,
            config_file,
        }
    }

    pub fn executable_dir(&self) -> &Path {
        &self.executable_dir
    }

    pub async fn get_banner(&mut self, fetcher: &DatabaseFetcher) -> Result<(), ChadError> {
        if let Ok(banner_path) = fetcher.find_banner(&self.name).await {
            let target = format!(
                "https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/{}",
                banner_path
            );
            let response = reqwest::get(target).await?;
            let content = response.text().await?;
            std::io::copy(
                &mut content.as_bytes(),
                &mut std::fs::File::create(self.data_path.join("banner.png"))?,
            )?;
            self.banner_path = Some(self.data_path.join("banner.png"));
            self.banner = self.banner_path.as_ref().and_then(|p| load_banner(&p));
        }

        Ok(())
    }

    pub fn launch(&self, script: &str) -> Result<Box<dyn Read>, ChadError> {
        let child = Command::new(&self.executable_dir.join(&script))
            .current_dir(&self.executable_dir)
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
        self.games = config
            // Iterate over all library paths
            .library_paths()
            .into_iter()
            // Read each library path
            .map(|lp| {
                if let Ok(dir) = lp.read_dir() {
                    Box::new(
                        dir
                            // Filter out any errors
                            .filter_map(|e| e.ok())
                            // Find all directories
                            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false)),
                    ) as Box<dyn Iterator<Item = std::fs::DirEntry>>
                } else {
                    Box::new(std::iter::empty())
                }
            })
            // Flatten those nested iterators into a single iterator
            .flatten()
            // Zip it with indices
            .zip(0..)
            // Create games
            .map(|(e, i)| Game::new(&config, i, e.path()))
            // Collect them into a vec
            .collect();
    }

    pub async fn download_banners(&mut self, fetcher: &DatabaseFetcher) {
        join_all(
            self.games
                .iter_mut()
                .filter(|g| g.banner == None)
                .map(|g| g.get_banner(&fetcher)),
        )
        .await;
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
