//! 网络模块
//!
//! 这个模块提供了HTTP客户端功能，支持自定义UserAgent和常见的HTTP请求方法。

mod client;
mod error;

pub use client::HttpClient;
pub use error::NetworkError;
