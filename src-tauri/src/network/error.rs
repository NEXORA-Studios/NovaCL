use std::fmt;
use std::io;

/// 网络错误类型
#[derive(Debug)]
pub enum NetworkError {
    /// IO错误
    IoError(io::Error),
    /// HTTP请求错误
    HttpError(String),
    /// 无效的URL
    InvalidUrl(String),
    /// 请求超时
    Timeout,
    /// 连接错误
    ConnectionError(String),
    /// 响应解析错误
    ResponseParseError(String),
    /// 其他错误
    Other(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "IO错误: {}", err),
            Self::HttpError(err) => write!(f, "HTTP请求错误: {}", err),
            Self::InvalidUrl(url) => write!(f, "无效的URL: {}", url),
            Self::Timeout => write!(f, "请求超时"),
            Self::ConnectionError(err) => write!(f, "连接错误: {}", err),
            Self::ResponseParseError(err) => write!(f, "响应解析错误: {}", err),
            Self::Other(err) => write!(f, "其他错误: {}", err),
        }
    }
}

impl std::error::Error for NetworkError {}

impl From<io::Error> for NetworkError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<reqwest::Error> for NetworkError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::Timeout
        } else if err.is_connect() {
            Self::ConnectionError(err.to_string())
        } else if err.is_decode() {
            Self::ResponseParseError(err.to_string())
        } else if err.is_status() {
            Self::HttpError(format!("状态码错误: {}", err))
        } else {
            Self::Other(err.to_string())
        }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for NetworkError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Self::Other(format!("无效的请求头: {}", err))
    }
}

impl From<serde_json::Error> for NetworkError {
    fn from(err: serde_json::Error) -> Self {
        Self::ResponseParseError(err.to_string())
    }
}