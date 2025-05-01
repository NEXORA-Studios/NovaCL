//! 下载模块
//! 
//! 这个模块提供了一个多线程下载管理器，支持暂停/恢复、多线程并行下载、进度报告和错误重试机制。

mod manager;
mod task;
mod error;

pub use manager::DownloadManager;
pub use task::DownloadProgress;