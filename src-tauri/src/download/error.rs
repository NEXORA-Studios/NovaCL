use std::fmt;
use std::io;

/// 下载错误类型
#[derive(Debug)]
pub enum DownloadError {
    /// IO错误
    IoError(io::Error),
    /// HTTP请求错误
    HttpError(String),
    /// 无效的URL
    InvalidUrl(String),
    /// 无法获取文件大小
    ContentLengthError,
    /// 不支持断点续传
    RangeNotSupported,
    /// 任务已存在
    TaskAlreadyExists(String),
    /// 任务不存在
    TaskNotFound(String),
    /// 锁定错误
    LockError,
    /// 文件写入错误
    WriteError(String),
    /// 其他错误
    Other(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "IO错误: {}", err),
            Self::HttpError(err) => write!(f, "HTTP请求错误: {}", err),
            Self::InvalidUrl(url) => write!(f, "无效的URL: {}", url),
            Self::ContentLengthError => write!(f, "无法获取文件大小"),
            Self::RangeNotSupported => write!(f, "服务器不支持断点续传"),
            Self::TaskAlreadyExists(id) => write!(f, "任务已存在: {}", id),
            Self::TaskNotFound(id) => write!(f, "任务不存在: {}", id),
            Self::LockError => write!(f, "锁定错误"),
            Self::WriteError(err) => write!(f, "文件写入错误: {}", err),
            Self::Other(err) => write!(f, "其他错误: {}", err),
        }
    }
}

impl std::error::Error for DownloadError {}

impl From<io::Error> for DownloadError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpError(err.to_string())
    }
}

impl From<url::ParseError> for DownloadError {
    fn from(err: url::ParseError) -> Self {
        Self::InvalidUrl(err.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for DownloadError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Self::HttpError(err.to_string())
    }
}
