use super::config::{Accept, HttpConfig};
use reqwest::{Client, ClientBuilder, StatusCode};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("HTTP status {code}")]
    Status { code: StatusCode, body: Option<String> },

    #[error("decode error: {0}")]
    Decode(String),
}

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    stream: String,
    token: String,
    accept: Accept,
}

impl HttpClient {
    pub fn new(cfg: HttpConfig) -> Result<Self, HttpError> {
        if cfg.base_url().trim().is_empty() {
            return Err(HttpError::InvalidConfig(
                "base_url must not be empty".into(),
            ));
        }
        if cfg.stream().trim().is_empty() {
            return Err(HttpError::InvalidConfig("stream must not be empty".into()));
        }

        let mut builder = ClientBuilder::new()
            .https_only(true)
            .timeout(Duration::from_secs(cfg.timeout()));

        if let Some(path) = cfg.ca_cert() {
            let cert = std::fs::read(path)
                .map_err(|e| HttpError::InvalidConfig(format!("cannot read CA: {e}")))?;
            let cert = reqwest::Certificate::from_pem(&cert)
                .map_err(|e| HttpError::InvalidConfig(format!("invalid CA: {e}")))?;
            builder = builder.add_root_certificate(cert);
        }

        let client = builder
            .build()
            .map_err(|e| HttpError::InvalidConfig(format!("failed to build client: {e}")))?;

        Ok(Self {
            client,
            base_url: cfg.base_url().to_string(),
            stream: cfg.stream().to_string(),
            token: cfg.token().to_string(),
            accept: cfg.accept(),
        })
    }

    /// GET retained message with Accept: text/plain
    pub async fn get_text_plain(&self, topic: &str) -> Result<String, HttpError> {
        self.get_with_accept(topic, Accept::TextPlain).await
    }

    pub async fn get_with_accept(
        &self,
        topic: &str,
        accept: Accept,
    ) -> Result<String, HttpError> {
        if topic.trim().is_empty() {
            return Err(HttpError::InvalidConfig("topic cannot be empty".into()));
        }

        let url = format!(
            "{}/data/v0/single/tt/{}/{}",
            self.base_url.trim_end_matches('/'),
            self.stream,
            topic
        );

        let resp = self
            .client
            .get(&url)
            .header(reqwest::header::ACCEPT, accept.as_str())
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.token))
            .send()
            .await
            .map_err(|e| HttpError::Network(e.to_string()))?;

        let status = resp.status();

        if status.is_success() {
            resp.text()
                .await
                .map_err(|e| HttpError::Decode(e.to_string()))
        } else {
            let body = resp.text().await.ok();
            Err(HttpError::Status { code: status, body })
        }
    }
}