use crate::command::{AppState, TauriChadError};
use chad_launcher::{
    config::Config,
    download::{DownloadManager, Torrent, TorrentClientConfig},
};
use chad_torrent::TorrentBackend;
use tauri::async_runtime::Mutex;

#[tauri::command]
pub async fn download_init_clients(
    config: tauri::State<'_, Mutex<Config>>,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    download
        .lock()
        .await
        .load_config(&*config.lock().await)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn download_list_clients(
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<Vec<String>, TauriChadError> {
    Ok(download.lock().await.clients().cloned().collect())
}

fn get_backend(
    client: String,
    download: &DownloadManager,
) -> Result<&chad_torrent::TorrentClient, TauriChadError> {
    if let Some(backend) = download.client(&client) {
        Ok(backend)
    } else {
        Err(TauriChadError::new("Client not found".into()))
    }
}

#[tauri::command]
pub async fn download_list_downloads(
    client: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<Vec<Torrent>, TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client.clone(), &*download)?;
    let list = backend
        .list(Some("chad"))
        .await
        .map_err(|e| TauriChadError::from(&*e))?
        .into_iter()
        .map(|t| Torrent {
            client: client.clone(),
            torrent: t,
        })
        .collect();
    Ok(list)
}

#[tauri::command]
pub async fn download_list_all_downloads(
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<Vec<Torrent>, TauriChadError> {
    let download = download.lock().await;
    let mut result = Vec::new();

    for client in download.clients() {
        let backend = get_backend(client.clone(), &*download)?;
        let mut list = backend
            .list(Some("chad"))
            .await
            .map_err(|e| TauriChadError::from(&*e))?
            .into_iter()
            .map(|t| Torrent {
                client: client.clone(),
                torrent: t,
            })
            .collect();
        result.append(&mut list);
    }

    Ok(result)
}

#[tauri::command]
pub async fn download_add_magnet(
    client: String,
    magnet: String,
    options: chad_torrent::Options,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<String, TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .add_magnet(&magnet, options)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
pub async fn download_pause(
    client: String,
    torrent_id: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .pause(&torrent_id)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
pub async fn download_resume(
    client: String,
    torrent_id: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .resume(&torrent_id)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
pub async fn download_remove(
    client: String,
    torrent_id: String,
    remove_files: bool,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .remove_torrent(&torrent_id, remove_files)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
pub async fn download_get_status(
    client: String,
    torrent_id: String,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<chad_torrent::Torrent, TauriChadError> {
    let download = download.lock().await;
    let backend = get_backend(client, &*download)?;
    backend
        .torrent(&torrent_id)
        .await
        .map_err(|e| TauriChadError::from(&*e))
}

#[tauri::command]
pub async fn download_add_qbittorrent_client(
    name: String,
    options: chad_launcher::download::QBittorrentConfig,
    config: tauri::State<'_, Mutex<Config>>,
    download: tauri::State<'_, Mutex<DownloadManager>>,
) -> Result<(), TauriChadError> {
    let mut download = download.lock().await;
    let client = download.qbittorrent_connect(&options).await?;
    download.add_client(&name, client);
    let mut config = config.lock().await;
    config.insert_download_client(name, TorrentClientConfig::QBittorrent(options));
    Ok(config.save()?)
}

#[tauri::command]
pub async fn download_deluge_connect_daemon(
    name: String,
    daemon_id: String,
    config: tauri::State<'_, Mutex<Config>>,
    download: tauri::State<'_, Mutex<DownloadManager>>,
    app_state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), TauriChadError> {
    let mut app_state = app_state.lock().await;
    if let (
        Some(chad_torrent::TorrentClient::DelugeBackend(backend)),
        Some(TorrentClientConfig::Deluge(mut options)),
    ) = (
        app_state.current_client.take(),
        app_state.current_config.take(),
    ) {
        backend
            .connect(&daemon_id)
            .await
            .map_err(|e| TauriChadError::from(&*e))?;
        let mut download = download.lock().await;
        download.add_client(&name, backend);
        options.daemon_id = Some(daemon_id);
        let mut config = config.lock().await;
        config.insert_download_client(name, TorrentClientConfig::Deluge(options));
        Ok(config.save()?)
    } else {
        Err(TauriChadError::new(
            "Failed to connect to deluge daemon: no active Web UI connection".into(),
        ))
    }
}

#[tauri::command]
pub async fn download_create_deluge_client(
    options: chad_launcher::download::DelugeConfig,
    download: tauri::State<'_, Mutex<DownloadManager>>,
    app_state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), TauriChadError> {
    let download = download.lock().await;
    let client = download.deluge_connect(&options).await?;
    let mut app_state = app_state.lock().await;
    app_state.current_client = Some(client.into());
    app_state.current_config = Some(TorrentClientConfig::Deluge(options));
    Ok(())
}

#[tauri::command]
pub async fn download_list_deluge_hosts(
    app_state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<chad_torrent::backend::deluge::Host>, TauriChadError> {
    let app_state = app_state.lock().await;
    if let Some(chad_torrent::TorrentClient::DelugeBackend(backend)) = &app_state.current_client {
        backend
            .list_hosts()
            .await
            .map_err(|e| TauriChadError::from(&*e))
    } else {
        Err(TauriChadError::new(
            "Failed to list deluge hosts: no active Web UI connection".into(),
        ))
    }
}
