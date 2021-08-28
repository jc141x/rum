use crate::config::Config;
use crate::util::ChadError;
use chad_torrent::{DelugeBackend, QBittorrentBackend, TorrentClient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    pub client: String,

    #[serde(flatten)]
    pub torrent: chad_torrent::Torrent,
}

impl std::ops::Deref for Torrent {
    type Target = chad_torrent::Torrent;

    fn deref(&self) -> &Self::Target {
        &self.torrent
    }
}

impl std::ops::DerefMut for Torrent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.torrent
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "backend")]
pub enum TorrentClientConfig {
    #[serde(rename = "deluge")]
    Deluge(DelugeConfig),
    #[serde(rename = "qbittorrent")]
    QBittorrent(QBittorrentConfig),
}

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

#[derive(Default)]
pub struct DownloadManager {
    clients: HashMap<String, TorrentClient>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load_config(&mut self, config: &Config) -> Result<(), ChadError> {
        for (name, c) in &config.torrent.clients {
            self.load_client(&name, &c).await.ok();
        }

        Ok(())
    }

    pub async fn load_client(
        &mut self,
        name: &str,
        config: &TorrentClientConfig,
    ) -> Result<(), ChadError> {
        if !self.clients.contains_key(name) {
            println!("Connecting to {:#?}", &name);
            self.clients.insert(
                name.into(),
                match config {
                    TorrentClientConfig::Deluge(options) => {
                        self.deluge_connect(&options).await.map(|c| c.into())
                    }
                    TorrentClientConfig::QBittorrent(options) => {
                        self.qbittorrent_connect(&options).await.map(|c| c.into())
                    }
                }?,
            );
        }
        Ok(())
    }

    pub fn add_client(&mut self, name: &str, client: impl Into<TorrentClient>) {
        self.clients.insert(name.into(), client.into());
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

    pub fn client(&self, name: &str) -> Option<&TorrentClient> {
        self.clients.get(name)
    }
}
