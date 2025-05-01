use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

use crate::network::error::NetworkError;

/// HTTP客户端，支持自定义UserAgent
pub struct HttpClient {
    /// 内部reqwest客户端
    client: reqwest::Client,
    /// 默认超时时间（秒）
    timeout: u64,
}

impl HttpClient {
    /// 创建一个新的HTTP客户端，使用默认配置
    pub fn new() -> Self {
        Self::with_user_agent("NovaCL/1.0")
    }

    /// 创建一个新的HTTP客户端，使用自定义UserAgent
    pub fn with_user_agent(user_agent: &str) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            client,
            timeout: 30,
        }
    }

    /// 创建一个新的HTTP客户端，使用自定义配置
    pub fn with_config(user_agent: &str, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap_or_default();

        Self {
            client,
            timeout,
        }
    }

    /// 设置超时时间
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
        // 重新创建客户端以应用新的超时设置
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap_or_default();
        self.client = client;
    }

    /// 发送GET请求
    pub async fn get<T>(&self, url: &str) -> Result<T, NetworkError>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(NetworkError::HttpError(format!("HTTP错误: {}", status)));
        }

        let result = response.json::<T>().await?;
        Ok(result)
    }

    /// 发送GET请求并返回文本
    pub async fn get_text(&self, url: &str) -> Result<String, NetworkError> {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(NetworkError::HttpError(format!("HTTP错误: {}", status)));
        }

        let result = response.text().await?;
        Ok(result)
    }

    /// 发送GET请求并返回字节数据
    pub async fn get_bytes(&self, url: &str) -> Result<bytes::Bytes, NetworkError> {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(NetworkError::HttpError(format!("HTTP错误: {}", status)));
        }

        let result = response.bytes().await?;
        Ok(result)
    }

    /// 发送POST请求，带JSON数据
    pub async fn post_json<T, R>(&self, url: &str, data: &T) -> Result<R, NetworkError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let response = self.client.post(url).json(data).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(NetworkError::HttpError(format!("HTTP错误: {}", status)));
        }

        let result = response.json::<R>().await?;
        Ok(result)
    }

    /// 发送POST请求，带表单数据
    pub async fn post_form<T, R>(&self, url: &str, form: &T) -> Result<R, NetworkError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let response = self.client.post(url).form(form).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(NetworkError::HttpError(format!("HTTP错误: {}", status)));
        }

        let result = response.json::<R>().await?;
        Ok(result)
    }

    /// 发送自定义请求
    pub async fn request<R>(
        &self,
        method: reqwest::Method,
        url: &str,
        headers: Option<HeaderMap>,
        body: Option<Vec<u8>>,
    ) -> Result<R, NetworkError>
    where
        R: DeserializeOwned,
    {
        let mut request_builder = self.client.request(method, url);

        if let Some(headers) = headers {
            request_builder = request_builder.headers(headers);
        }

        if let Some(body) = body {
            request_builder = request_builder.body(body);
        }

        let response = request_builder.send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(NetworkError::HttpError(format!("HTTP错误: {}", status)));
        }

        let result = response.json::<R>().await?;
        Ok(result)
    }

    /// 创建一个HeaderMap，包含自定义的UserAgent
    pub fn create_headers(&self, user_agent: Option<&str>) -> Result<HeaderMap, NetworkError> {
        let mut headers = HeaderMap::new();
        
        if let Some(ua) = user_agent {
            headers.insert(USER_AGENT, HeaderValue::from_str(ua)?);
        }
        
        Ok(headers)
    }
}