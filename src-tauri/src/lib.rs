// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod download;
mod network;

use std::path::PathBuf;
use std::sync::Arc;
use tauri::async_runtime::Mutex;
use tauri::State;

use download::{DownloadManager, DownloadProgress};
use network::HttpClient;

// 全局下载管理器状态
struct DownloadManagerState {
    manager: Arc<Mutex<DownloadManager>>,
}

// 全局HTTP客户端状态
struct HttpClientState {
    client: Arc<Mutex<HttpClient>>,
}

#[tauri::command]
async fn start_download(
    url: String,
    save_path: String,
    filename: Option<String>,
    segments: Option<usize>,
    state: State<'_, DownloadManagerState>,
) -> Result<String, String> {
    let manager = state.inner().manager.lock().await;
    let save_path = PathBuf::from(save_path);
    let segments = segments.unwrap_or(4); // 默认4个分段

    manager
        .add_task(&url, save_path, filename, segments)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn pause_download(
    task_id: String,
    state: State<'_, DownloadManagerState>,
) -> Result<(), String> {
    let manager = state.inner().manager.lock().await;
    manager
        .pause_task(&task_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn resume_download(
    task_id: String,
    state: State<'_, DownloadManagerState>,
) -> Result<(), String> {
    let manager = state.inner().manager.lock().await;
    manager
        .resume_task(&task_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn cancel_download(
    task_id: String,
    state: State<'_, DownloadManagerState>,
) -> Result<(), String> {
    let manager = state.inner().manager.lock().await;
    manager
        .cancel_task(&task_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_download_progress(
    task_id: String,
    state: State<'_, DownloadManagerState>,
) -> Result<DownloadProgress, String> {
    let manager = state.inner().manager.lock().await;
    manager
        .get_task_progress(&task_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_downloads(
    state: State<'_, DownloadManagerState>,
) -> Result<Vec<(String, DownloadProgress)>, String> {
    let manager = state.inner().manager.lock().await;
    manager.get_tasks().map_err(|e| e.to_string())
}

// HTTP客户端命令
#[tauri::command]
async fn http_get(
    url: String,
    user_agent: Option<String>,
    state: State<'_, HttpClientState>,
) -> Result<String, String> {
    let mut client = state.inner().client.lock().await;

    // 如果提供了自定义UserAgent，则重新创建客户端
    if let Some(ua) = &user_agent {
        *client = HttpClient::with_user_agent(ua);
    }

    client.get_text(&url).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn http_post_json<T: serde::Serialize>(
    url: String,
    data: T,
    user_agent: Option<String>,
    state: State<'_, HttpClientState>,
) -> Result<String, String> {
    let mut client = state.inner().client.lock().await;

    // 如果提供了自定义UserAgent，则重新创建客户端
    if let Some(ua) = &user_agent {
        *client = HttpClient::with_user_agent(ua);
    }

    client
        .post_json::<T, serde_json::Value>(&url, &data)
        .await
        .map(|json| json.to_string())
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建下载管理器
    let download_manager = DownloadManager::new(5); // 最大5个并发下载
    let download_manager_state = DownloadManagerState {
        manager: Arc::new(Mutex::new(download_manager)),
    };

    // 创建HTTP客户端
    let http_client = HttpClient::new();
    let http_client_state = HttpClientState {
        client: Arc::new(Mutex::new(http_client)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(download_manager_state)
        .manage(http_client_state)
        .invoke_handler(tauri::generate_handler![
            start_download,
            pause_download,
            resume_download,
            cancel_download,
            get_download_progress,
            get_all_downloads,
            http_get,
            http_post_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
