use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// use futures::stream::FuturesUnordered;
use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, RANGE};
use tauri::async_runtime::{spawn, JoinHandle};
use tokio::sync::mpsc;
use url::Url;
use uuid::Uuid;

use crate::download::error::DownloadError;
use crate::download::task::{DownloadProgress, DownloadStatus, DownloadTask};

/// 下载事件类型
#[derive(Debug, Clone)]
pub enum DownloadEvent {
    /// 下载开始
    Started(String),
    /// 下载进度更新
    ProgressUpdated(String, DownloadProgress),
    /// 下载暂停
    Paused(String),
    /// 下载恢复
    Resumed(String),
    /// 下载完成
    Completed(String),
    /// 下载失败
    Failed(String, String),
    /// 下载取消
    Cancelled(String),
    // 注意：这些字段在编译时显示为未使用，但它们在事件系统中是必要的
    // 如果确实不需要，可以考虑将字段类型改为()
}

/// 下载管理器
#[derive(Debug)]
pub struct DownloadManager {
    /// 下载任务列表
    tasks: Arc<Mutex<HashMap<String, Arc<DownloadTask>>>>,
    /// 活跃任务的句柄
    active_tasks: Arc<Mutex<HashMap<String, Vec<JoinHandle<()>>>>>,
    /// 事件发送器
    event_sender: mpsc::Sender<DownloadEvent>,
    /// 事件接收器
    event_receiver: Arc<Mutex<Option<mpsc::Receiver<DownloadEvent>>>>,
    /// HTTP客户端
    client: reqwest::Client,
    /// 最大并发下载数
    max_concurrent_downloads: usize,
}

impl DownloadManager {
    /// 创建一个新的下载管理器
    pub fn new(max_concurrent_downloads: usize) -> Self {
        let (tx, rx) = mpsc::channel(100);

        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
            event_sender: tx,
            event_receiver: Arc::new(Mutex::new(Some(rx))),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            max_concurrent_downloads,
        }
    }

    /// 获取事件接收器
    pub fn take_event_receiver(&self) -> Option<mpsc::Receiver<DownloadEvent>> {
        let mut receiver = self.event_receiver.lock().unwrap();
        receiver.take()
    }

    /// 添加下载任务
    pub async fn add_task(
        &self,
        url: &str,
        save_path: PathBuf,
        filename: Option<String>,
        segments: usize,
    ) -> Result<String, DownloadError> {
        // 解析URL
        let parsed_url = Url::parse(url)?;

        // 如果没有提供文件名，从URL中提取
        let filename = match filename {
            Some(name) => name,
            None => {
                let path = parsed_url.path();
                let segments: Vec<&str> = path.split('/').collect();
                let last_segment = segments.last().unwrap_or(&"");
                if last_segment.is_empty() {
                    return Err(DownloadError::Other("无法从URL中提取文件名".to_string()));
                }
                last_segment.to_string()
            }
        };

        // 创建保存目录
        if !save_path.exists() {
            fs::create_dir_all(&save_path)?;
        }

        // 生成任务ID
        let task_id = Uuid::new_v4().to_string();

        // 创建下载任务
        let task = Arc::new(DownloadTask::new(
            task_id.clone(),
            url.to_string(),
            save_path,
            filename,
            segments,
        ));

        // 添加到任务列表
        {
            let mut tasks = self.tasks.lock().map_err(|_| DownloadError::LockError)?;
            if tasks.contains_key(&task_id) {
                return Err(DownloadError::TaskAlreadyExists(task_id));
            }
            tasks.insert(task_id.clone(), task.clone());
        }

        Ok(task_id)
    }

    /// 开始下载任务
    pub async fn start_task(&self, task_id: &str) -> Result<(), DownloadError> {
        let task = {
            let tasks = self.tasks.lock().map_err(|_| DownloadError::LockError)?;
            tasks
                .get(task_id)
                .cloned()
                .ok_or_else(|| DownloadError::TaskNotFound(task_id.to_string()))?
        };

        // 检查当前活跃任务数量
        {
            let active_tasks = self
                .active_tasks
                .lock()
                .map_err(|_| DownloadError::LockError)?;
            if active_tasks.len() >= self.max_concurrent_downloads {
                // 设置为等待状态
                task.set_status(DownloadStatus::Pending)?;
                return Ok(());
            }
        }

        // 获取文件大小
        let file_size = self.get_file_size(&task.url).await?;
        task.set_total_size(file_size)?;

        // 设置开始时间
        // 使用方法设置开始时间，而不是直接修改Arc中的数据
        task.set_start_time()?;

        // 设置状态为下载中
        task.set_status(DownloadStatus::Downloading)?;

        // 发送开始事件
        let _ = self
            .event_sender
            .send(DownloadEvent::Started(task_id.to_string()))
            .await;

        // 计算每个分段的大小
        let segment_size = file_size / task.segments as u64;
        let mut handles = Vec::new();

        // 创建临时文件
        let temp_path = task.full_path().with_extension("part");
        {
            let file = File::create(&temp_path)?;
            file.set_len(file_size)?;
        }

        // 创建进度更新通道
        let (progress_tx, mut progress_rx) = mpsc::channel(100);

        // 启动进度更新任务
        let task_clone = task.clone();
        let task_id_clone = task_id.to_string();
        let event_sender = self.event_sender.clone();
        let progress_handle = spawn(async move {
            let mut total_downloaded = 0;

            while let Some((_segment_id, bytes)) = progress_rx.recv().await {
                total_downloaded += bytes;

                // 更新总进度
                if let Err(e) = task_clone.update_progress(total_downloaded) {
                    eprintln!("更新进度失败: {}", e);
                    continue;
                }

                // 获取当前进度并发送事件
                if let Ok(progress) = task_clone.get_progress() {
                    let _ = event_sender
                        .send(DownloadEvent::ProgressUpdated(
                            task_id_clone.clone(),
                            progress,
                        ))
                        .await;
                }
            }
        });

        handles.push(progress_handle);

        // 启动分段下载任务
        for i in 0..task.segments {
            let start = i as u64 * segment_size;
            let end = if i == task.segments - 1 {
                file_size - 1
            } else {
                (i as u64 + 1) * segment_size - 1
            };

            let client = self.client.clone();
            let url = task.url.clone();
            let temp_path = temp_path.clone();
            let progress_tx = progress_tx.clone();
            let segment_id = i;

            let handle = spawn(async move {
                let mut headers = HeaderMap::new();
                headers.insert(
                    RANGE,
                    HeaderValue::from_str(&format!("bytes={}-{}", start, end)).unwrap(),
                );

                let mut retry_count = 0;
                let max_retries = 3;

                loop {
                    match Self::download_segment(
                        client.clone(),
                        &url,
                        &temp_path,
                        start,
                        end,
                        segment_id,
                        progress_tx.clone(),
                    )
                    .await
                    {
                        Ok(_) => break,
                        Err(e) => {
                            retry_count += 1;
                            if retry_count >= max_retries {
                                eprintln!("分段 {} 下载失败: {}", segment_id, e);
                                break;
                            }

                            // 等待一段时间后重试
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    }
                }
            });

            handles.push(handle);
        }

        // 添加完成处理任务
        let task_clone = task.clone();
        let task_id_clone = task_id.to_string();
        let event_sender = self.event_sender.clone();
        let temp_path_clone = temp_path.clone();
        let active_tasks = self.active_tasks.clone();

        let completion_handle = spawn(async move {
            // 等待所有分段下载完成
            for handle in handles {
                let _ = handle.await;
            }

            // 检查下载状态
            let status = match task_clone.get_progress() {
                Ok(progress) => progress.status,
                Err(_) => DownloadStatus::Failed,
            };

            match status {
                DownloadStatus::Downloading => {
                    // 重命名临时文件
                    if let Err(e) = fs::rename(&temp_path_clone, task_clone.full_path()) {
                        eprintln!("重命名文件失败: {}", e);
                        let _ = task_clone.set_status(DownloadStatus::Failed);
                        let _ = event_sender
                            .send(DownloadEvent::Failed(
                                task_id_clone.clone(),
                                format!("重命名文件失败: {}", e),
                            ))
                            .await;
                    } else {
                        // 设置状态为已完成
                        let _ = task_clone.set_status(DownloadStatus::Completed);
                        let _ = event_sender
                            .send(DownloadEvent::Completed(task_id_clone.clone()))
                            .await;
                    }
                }
                DownloadStatus::Cancelled => {
                    // 删除临时文件
                    let _ = fs::remove_file(&temp_path_clone);
                }
                DownloadStatus::Failed => {
                    // 删除临时文件
                    let _ = fs::remove_file(&temp_path_clone);
                    let _ = event_sender
                        .send(DownloadEvent::Failed(
                            task_id_clone.clone(),
                            "下载失败".to_string(),
                        ))
                        .await;
                }
                _ => {}
            }

            // 从活跃任务列表中移除
            let mut active = active_tasks.lock().unwrap();
            active.remove(&task_id_clone);

            // 检查是否有等待中的任务可以启动
            // 这里简化处理，实际应该通知管理器启动下一个任务
        });

        // 添加到活跃任务列表
        {
            let mut active_tasks = self
                .active_tasks
                .lock()
                .map_err(|_| DownloadError::LockError)?;
            active_tasks.insert(task_id.to_string(), vec![completion_handle]);
        }

        Ok(())
    }

    /// 暂停下载任务
    pub async fn pause_task(&self, task_id: &str) -> Result<(), DownloadError> {
        let task = {
            let tasks = self.tasks.lock().map_err(|_| DownloadError::LockError)?;
            tasks
                .get(task_id)
                .cloned()
                .ok_or_else(|| DownloadError::TaskNotFound(task_id.to_string()))?
        };

        // 设置状态为已暂停
        task.set_status(DownloadStatus::Paused)?;

        // 取消活跃任务
        {
            let mut active_tasks = self
                .active_tasks
                .lock()
                .map_err(|_| DownloadError::LockError)?;
            if let Some(handles) = active_tasks.remove(task_id) {
                for handle in handles {
                    handle.abort();
                }
            }
        }

        // 发送暂停事件
        let _ = self
            .event_sender
            .send(DownloadEvent::Paused(task_id.to_string()))
            .await;

        Ok(())
    }

    /// 恢复下载任务
    pub async fn resume_task(&self, task_id: &str) -> Result<(), DownloadError> {
        let task = {
            let tasks = self.tasks.lock().map_err(|_| DownloadError::LockError)?;
            tasks
                .get(task_id)
                .cloned()
                .ok_or_else(|| DownloadError::TaskNotFound(task_id.to_string()))?
        };

        // 检查当前状态
        let current_status = {
            let progress = task.get_progress()?;
            progress.status
        };

        if current_status != DownloadStatus::Paused {
            return Err(DownloadError::Other(format!(
                "任务状态不是已暂停: {:?}",
                current_status
            )));
        }

        // 发送恢复事件
        let _ = self
            .event_sender
            .send(DownloadEvent::Resumed(task_id.to_string()))
            .await;

        // 重新开始任务
        self.start_task(task_id).await
    }

    /// 取消下载任务
    pub async fn cancel_task(&self, task_id: &str) -> Result<(), DownloadError> {
        let task = {
            let tasks = self.tasks.lock().map_err(|_| DownloadError::LockError)?;
            tasks
                .get(task_id)
                .cloned()
                .ok_or_else(|| DownloadError::TaskNotFound(task_id.to_string()))?
        };

        // 设置状态为已取消
        task.set_status(DownloadStatus::Cancelled)?;

        // 取消活跃任务
        {
            let mut active_tasks = self
                .active_tasks
                .lock()
                .map_err(|_| DownloadError::LockError)?;
            if let Some(handles) = active_tasks.remove(task_id) {
                for handle in handles {
                    handle.abort();
                }
            }
        }

        // 删除临时文件
        let temp_path = task.full_path().with_extension("part");
        if temp_path.exists() {
            let _ = fs::remove_file(temp_path);
        }

        // 发送取消事件
        let _ = self
            .event_sender
            .send(DownloadEvent::Cancelled(task_id.to_string()))
            .await;

        Ok(())
    }

    /// 获取任务列表
    pub fn get_tasks(&self) -> Result<Vec<(String, DownloadProgress)>, DownloadError> {
        let tasks = self.tasks.lock().map_err(|_| DownloadError::LockError)?;

        let mut result = Vec::new();
        for (id, task) in tasks.iter() {
            if let Ok(progress) = task.get_progress() {
                result.push((id.clone(), progress));
            }
        }

        Ok(result)
    }

    /// 获取任务进度
    pub fn get_task_progress(&self, task_id: &str) -> Result<DownloadProgress, DownloadError> {
        let tasks = self.tasks.lock().map_err(|_| DownloadError::LockError)?;

        let task = tasks
            .get(task_id)
            .ok_or_else(|| DownloadError::TaskNotFound(task_id.to_string()))?;
        task.get_progress()
    }

    /// 获取文件大小
    async fn get_file_size(&self, url: &str) -> Result<u64, DownloadError> {
        let response = self.client.head(url).send().await?;

        if !response.status().is_success() {
            return Err(DownloadError::HttpError(format!(
                "HTTP错误: {}",
                response.status()
            )));
        }

        let content_length = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .ok_or(DownloadError::ContentLengthError)?;

        Ok(content_length)
    }

    /// 下载分段
    async fn download_segment(
        client: reqwest::Client,
        url: &str,
        file_path: &PathBuf,
        start: u64,
        end: u64,
        segment_id: usize,
        progress_tx: mpsc::Sender<(usize, u64)>,
    ) -> Result<(), DownloadError> {
        // 设置请求头
        let mut headers = HeaderMap::new();
        headers.insert(
            RANGE,
            HeaderValue::from_str(&format!("bytes={}-{}", start, end))?,
        );

        // 发送请求
        let response = client.get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(DownloadError::HttpError(format!(
                "HTTP错误: {}",
                response.status()
            )));
        }

        // 打开文件
        let mut file = fs::OpenOptions::new().write(true).open(file_path)?;

        // 设置文件指针位置
        file.seek(SeekFrom::Start(start))?;

        // 下载数据
        let mut stream = response.bytes_stream();
        let mut _downloaded = 0;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            file.write_all(&chunk)?;

            _downloaded += chunk.len() as u64;

            // 发送进度更新
            if let Err(e) = progress_tx.send((segment_id, chunk.len() as u64)).await {
                eprintln!("发送进度更新失败: {}", e);
            }
        }

        Ok(())
    }
}
