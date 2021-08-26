use crate::config::{Config, TorrentClientConfig, TorrentConfig};
use crate::util::ChadError;
use chad_torrent::{DelugeBackend, QBittorrentBackend, Torrent, TorrentBackend};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelugeConfig {
    web_address: String,
    web_password: String,
    daemon_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QBittorrentConfig {
    host: String,
    username: String,
    password: String,
}

#[derive(Default)]
pub struct DownloadManager {
    clients: HashMap<String, Box<dyn TorrentBackend + Send + Sync>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load_config(&mut self, config: &Config) -> Result<(), ChadError> {
        for c in &config.torrent.clients {
            self.clients
                .insert(c.name.clone(), Self::load_client(&c).await?);
        }

        Ok(())
    }

    pub async fn load_client(
        config: &TorrentClientConfig,
    ) -> Result<Box<dyn TorrentBackend + Send + Sync>, ChadError> {
        match config.backend.as_str() {
            "deluge" => Self::load_deluge(serde_json::from_value(config.options.clone())?).await,
            "qbittorrent" => {
                Self::load_qbittorrent(serde_json::from_value(config.options.clone())?).await
            }
            _ => Err(ChadError::Message("Invalid configuration".into())),
        }
    }

    pub async fn load_deluge(
        config: DelugeConfig,
    ) -> Result<Box<dyn TorrentBackend + Send + Sync>, ChadError> {
        let backend = DelugeBackend::new(&config.web_address, &config.web_password).await?;
        backend.connect(&config.daemon_id).await?;
        Ok(Box::new(backend))
    }

    pub async fn load_qbittorrent(
        config: QBittorrentConfig,
    ) -> Result<Box<dyn TorrentBackend + Send + Sync>, ChadError> {
        Ok(Box::new(
            QBittorrentBackend::new(&config.host, &config.username, &config.password).await?,
        ))
    }

    pub fn clients(&self) -> impl Iterator<Item = &String> {
        self.clients.keys()
    }

    pub fn client(&self, name: &str) -> Option<&Box<dyn TorrentBackend + Send + Sync>> {
        self.clients.get(name)
    }
}
