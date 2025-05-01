use std::path::PathBuf;
use std::sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use crate::download::error::DownloadError;

/// 下载任务的状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    /// 等待中
    Pending,
    /// 正在下载
    Downloading,
    /// 已暂停
    Paused,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

/// 下载进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    /// 已下载的字节数
    pub downloaded: u64,
    /// 总字节数
    pub total: u64,
    /// 下载速度 (bytes/s)
    pub speed: u64,
    /// 预计剩余时间 (秒)
    pub eta: u64,
    /// 下载状态
    pub status: DownloadStatus,
}

impl DownloadProgress {
    /// 创建一个新的下载进度信息
    pub fn new(total: u64) -> Self {
        Self {
            downloaded: 0,
            total,
            speed: 0,
            eta: 0,
            status: DownloadStatus::Pending,
        }
    }

    /// 更新下载进度
    pub fn update(&mut self, downloaded: u64, elapsed: Duration) {
        self.downloaded = downloaded;
        
        // 计算下载速度
        if !elapsed.is_zero() {
            let elapsed_secs = elapsed.as_secs_f64();
            if elapsed_secs > 0.0 {
                self.speed = (downloaded as f64 / elapsed_secs) as u64;
            }
        }
        
        // 计算剩余时间
        if self.speed > 0 && self.downloaded < self.total {
            self.eta = (self.total - self.downloaded) / self.speed;
        } else {
            self.eta = 0;
        }
    }

    /// 获取下载百分比
    pub fn percentage(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        (self.downloaded as f32 / self.total as f32) * 100.0
    }
}

/// 下载任务
#[derive(Debug)]
pub struct DownloadTask {
    /// 任务ID
    pub id: String,
    /// 下载URL
    pub url: String,
    /// 保存路径
    pub save_path: PathBuf,
    /// 文件名
    pub filename: String,
    /// 下载进度
    pub progress: Arc<Mutex<DownloadProgress>>,
    /// 开始时间 - 使用AtomicU64存储UNIX时间戳（毫秒）
    pub start_time: AtomicU64,
    /// 分段数量
    pub segments: usize,
    /// 重试次数
    pub retry_count: usize,
    /// 最大重试次数
    pub max_retries: usize,
}

impl DownloadTask {
    /// 创建一个新的下载任务
    pub fn new(id: String, url: String, save_path: PathBuf, filename: String, segments: usize) -> Self {
        Self {
            id,
            url,
            save_path,
            filename,
            progress: Arc::new(Mutex::new(DownloadProgress::new(0))),
            start_time: AtomicU64::new(0), // 0表示未开始
            segments,
            retry_count: 0,
            max_retries: 3,
        }
    }

    /// 获取完整的保存路径
    pub fn full_path(&self) -> PathBuf {
        self.save_path.join(&self.filename)
    }

    /// 设置总大小
    pub fn set_total_size(&self, size: u64) -> Result<(), DownloadError> {
        let mut progress = self.progress.lock().map_err(|_| DownloadError::LockError)?;
        progress.total = size;
        Ok(())
    }

    /// 更新下载进度
    pub fn update_progress(&self, downloaded: u64) -> Result<(), DownloadError> {
        let mut progress = self.progress.lock().map_err(|_| DownloadError::LockError)?;
        
        let start_timestamp = self.start_time.load(Ordering::Relaxed);
        let elapsed = if start_timestamp > 0 {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_millis() as u64;
            Duration::from_millis(now.saturating_sub(start_timestamp))
        } else {
            Duration::from_secs(0)
        };
        
        progress.update(downloaded, elapsed);
        Ok(())
    }

    /// 设置下载状态
    pub fn set_status(&self, status: DownloadStatus) -> Result<(), DownloadError> {
        let mut progress = self.progress.lock().map_err(|_| DownloadError::LockError)?;
        progress.status = status;
        Ok(())
    }

    /// 获取当前下载进度
    pub fn get_progress(&self) -> Result<DownloadProgress, DownloadError> {
        let progress = self.progress.lock().map_err(|_| DownloadError::LockError)?;
        Ok(progress.clone())
    }
    
    /// 设置开始时间
    pub fn set_start_time(&self) -> Result<(), DownloadError> {
        // 使用当前时间的UNIX时间戳（毫秒）
        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| DownloadError::Other(format!("获取系统时间失败: {}", e)))?;
        self.start_time.store(now.as_millis() as u64, Ordering::Relaxed);
        Ok(())
    }
}