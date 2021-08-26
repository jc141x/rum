use crate::config::{Config, TorrentClientConfig, TorrentConfig};
use crate::util::ChadError;
use chad_torrent::{DelugeBackend, QBittorrentBackend, Torrent, TorrentBackend};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelugeConfig {
    web_address: String,
    web_password: String,
    pub daemon_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QBittorrentConfig {
    host: String,
    username: String,
    password: String,
}

pub type Backend = Box<dyn TorrentBackend + Send + Sync>;

#[derive(Default)]
pub struct DownloadManager {
    clients: HashMap<String, Backend>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load_config(&mut self, config: &Config) -> Result<(), ChadError> {
        for (name, c) in &config.torrent.clients {
            self.load_client(&name, &c).await?;
        }

        Ok(())
    }

    pub async fn load_client(
        &mut self,
        name: &str,
        config: &TorrentClientConfig,
    ) -> Result<(), ChadError> {
        self.clients.insert(
            name.into(),
            match config.backend.as_str() {
                "deluge" => Ok(Box::new(
                    self.deluge_connect(&serde_json::from_value(config.options.clone())?)
                        .await?,
                ) as Backend),
                "qbittorrent" => Ok(Box::new(
                    self.qbittorrent_connect(&serde_json::from_value(config.options.clone())?)
                        .await?,
                ) as Backend),
                _ => Err(ChadError::Message("Invalid configuration".into())),
            }?,
        );
        Ok(())
    }

    pub fn add_client(&mut self, name: &str, client: Backend) {
        self.clients.insert(name.into(), client);
    }

    pub async fn deluge_connect(&self, config: &DelugeConfig) -> Result<DelugeBackend, ChadError> {
        let backend = DelugeBackend::new(&config.web_address, &config.web_password).await?;

        if let Some(daemon) = &config.daemon_id {
            backend.connect(daemon).await?;
        }

        Ok(backend)
    }

    pub async fn qbittorrent_connect(
        &self,
        config: &QBittorrentConfig,
    ) -> Result<QBittorrentBackend, ChadError> {
        Ok(QBittorrentBackend::new(&config.host, &config.username, &config.password).await?)
    }

    pub fn clients(&self) -> impl Iterator<Item = &String> {
        self.clients.keys()
    }

    pub fn client(&self, name: &str) -> Option<&Backend> {
        self.clients.get(name)
    }
}
