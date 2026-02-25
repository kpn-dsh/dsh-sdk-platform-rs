//! Reqwest-backed HTTP client (only this file depends on `reqwest`).
//!
//! What we implement (v0):
//! * `GET` a retained message for a stream/topic, using the documented HTTP endpoint:
//!   `{base_url}/data/v0/single/tt/{stream}/{topic}`
//! * Require an MQTT token in `Authorization: Bearer <token>`
//! * TLS enabled by default; optional custom CA. Minimal and well-commented.

use super::config::{Accept, HttpConfig};
use reqwest::{header, Client, ClientBuilder, StatusCode};
use std::{fs, time::Duration};

/// Minimal error type for the HTTP adapter.
#[derive(Debug)]
pub enum HttpError {
    /// Configuration missing required fields or inconsistent.
    InvalidConfig(String),
    /// Network or I/O error (includes TLS handshake failures surfaced by reqwest).
    Network(String),
    /// Non-success HTTP status returned by the server.
    Status(StatusCode),
    /// Response body decoding error.
    Decode(String),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::InvalidConfig(msg) => write!(f, "invalid configuration: {msg}"),
            HttpError::Network(msg) => write!(f, "network error: {msg}"),
            HttpError::Status(code) => write!(f, "http status: {code}"),
            HttpError::Decode(msg) => write!(f, "decode error: {msg}"),
        }
    }
}
impl std::error::Error for HttpError {}

/// Minimal reqwest-based client.
///
/// We keep it small and explicit for Rust beginners. Internally holds a `reqwest::Client`
/// configured with HTTPS-only and a timeout. Methods take `&HttpConfig` so the flow
/// remains explicit.
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// HEAD {base}/data/v0/single — proves TLS, hostname, token route work.
    pub async fn check_connectivity(&self, cfg: &HttpConfig) -> Result<(), HttpError> {
        let token = cfg.mqtt_token.as_ref().ok_or_else(|| {
            HttpError::InvalidConfig("Missing MQTT token".into())
        })?;
        let url = format!("{}/data/v0/single", cfg.base_url.trim_end_matches('/'));
        let resp = self
            .client
            .head(&url)
            .header(header::ACCEPT, cfg.accept.as_str())
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await

            .map_err(|e| HttpError::Network(e.to_string()))?;

        // Any HTTP status proves reachability; only network/TLS errors should fail.
        if resp.status().is_server_error() {
            return Err(HttpError::Status(resp.status()));
        }
        Ok(())
    }

    /// Build a `reqwest::Client` with HTTPS-only, timeout, and optional custom CA.
    pub fn new(cfg: &HttpConfig) -> Result<Self, HttpError> {
        if cfg.base_url.trim().is_empty() {
            return Err(HttpError::InvalidConfig(
                "base_url must be set (e.g., https://api.<platform-url>)".into(),
            ));
        }
        if cfg.stream.trim().is_empty() {
            return Err(HttpError::InvalidConfig(
                "stream must be set (e.g., weather)".into(),
            ));
        }

        let mut builder = ClientBuilder::new()
            .https_only(true)
            .timeout(Duration::from_secs(cfg.timeout_secs));

        // Optional: custom CA certificate (PEM) for internal environments.
        if let Some(ref ca_path) = cfg.ca_cert_path {
            let pem = fs::read(ca_path).map_err(|e| {
                HttpError::InvalidConfig(format!("failed to read CA cert at {:?}: {e}", ca_path))
            })?;
            let cert = reqwest::Certificate::from_pem(&pem).map_err(|e| {
                HttpError::InvalidConfig(format!("invalid CA PEM at {:?}: {e}", ca_path))
            })?;
            builder = builder.add_root_certificate(cert);
        }

        let client = builder
            .build()
            .map_err(|e| HttpError::InvalidConfig(format!("failed to build client: {e}")))?;
        Ok(Self { client })
    }

    /// Convenience: GET retained message as `text/plain`.
    ///
    /// Uses the documented endpoint and sets `Accept: text/plain`.
    pub async fn get_text_plain(
        &self,
        cfg: &HttpConfig,
        topic: &str,
    ) -> Result<String, HttpError> {
        self.get_with_accept(cfg, topic, Accept::TextPlain).await
    }

    /// GET retained message with the provided `Accept` value.
    ///
    /// Endpoint:
    /// `{base_url}/data/v0/single/tt/{stream}/{topic}`
    pub async fn get_with_accept(
        &self,
        cfg: &HttpConfig,
        topic: &str,
        accept: Accept,
    ) -> Result<String, HttpError> {
        if topic.trim().is_empty() {
            return Err(HttpError::InvalidConfig("topic must be non-empty".into()));
        }
        // Require an MQTT token provided by the SDK/trusted service.
        let token = cfg.mqtt_token.as_ref().ok_or_else(|| {
            HttpError::InvalidConfig(
                "Missing MQTT token. Fetch it using the SDK's token fetcher in a trusted service."
                    .into(),
            )
        })?;

        let url = build_url(&cfg.base_url, &cfg.stream, topic)?;

        let resp = self
            .client
            .get(&url)
            .header(header::ACCEPT, accept.as_str())
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| HttpError::Network(e.to_string()))?;

        let status = resp.status();
        if status.is_success() {
            resp.text()
                .await
                .map_err(|e| HttpError::Decode(e.to_string()))
        } else {
            Err(HttpError::Status(status))
        }
    }
}

/// Build the request URL by combining the base URL, stream and topic.
/// Keeps string handling in one place and easy to test.
///
/// Expected result:
/// `{base_url}/data/v0/single/tt/{stream}/{topic}`
fn build_url(base_url: &str, stream: &str, topic: &str) -> Result<String, HttpError> {
    if base_url.trim().is_empty() {
        return Err(HttpError::InvalidConfig("base_url must be set".into()));
    }
    if stream.trim().is_empty() {
        return Err(HttpError::InvalidConfig("stream must be set".into()));
    }
    if topic.trim().is_empty() {
        return Err(HttpError::InvalidConfig("topic must be set".into()));
    }
    let base = base_url.trim_end_matches('/');
    Ok(format!("{}/data/v0/single/tt/{}/{}", base, stream, topic))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_building_matches_spec() {
        let url = build_url(
            "https://api.example.com/",
            "weather",
            "house/kitchen/sensor",
        )
        .unwrap();
        assert_eq!(
            url,
            "https://api.example.com/data/v0/single/tt/weather/house/kitchen/sensor"
        );

        let url2 = build_url("https://api.example.com", "weather", "a").unwrap();
        assert_eq!(
            url2,
            "https://api.example.com/data/v0/single/tt/weather/a"
        );
    }
}
