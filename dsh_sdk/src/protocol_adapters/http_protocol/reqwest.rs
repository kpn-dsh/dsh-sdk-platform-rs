//! Reqwest-backed HTTP client (only this file depends on `reqwest`).
//!
//! What we implement (v0):
//! - `GET` a retained message for a tenant/topic, with `Accept: text/plain`.
//! - Require an MQTT token in `Authorization: Bearer <token>`.
//! - TLS enabled by default; optional custom CA. Minimal and well-commented.

use super::config::HttpConfig;
use std::fs;
use std::time::Duration;

use reqwest::{header, Client, ClientBuilder, StatusCode};

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
    /// Build a `reqwest::Client` with HTTPS-only, timeout, and optional custom CA.
    pub fn new(cfg: &HttpConfig) -> Result<Self, HttpError> {
        if cfg.base_url.trim().is_empty() {
            return Err(HttpError::InvalidConfig(
                "base_url must be set (e.g., https://host/v1)".into(),
            ));
        }
        if cfg.tenant.trim().is_empty() {
            return Err(HttpError::InvalidConfig(
                "tenant must be set (e.g., GREENBOX-DEV)".into(),
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

    /// GET a retained message (as `text/plain`) from:
    /// `{base_url}/tenants/{tenant}/topics/{topic}/messages`
    ///
    /// Requirements:
    /// - `cfg.mqtt_token` must be set (MQTT token-only auth).
    /// - TLS is enforced by the client builder.
    pub async fn get_text_plain(
        &self,
        cfg: &HttpConfig,
        topic: &str,
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

        let url = build_url(&cfg.base_url, &cfg.tenant, topic)?;

        // Prepare GET with Accept: text/plain and Authorization: Bearer <token>.
        let resp = self
            .client
            .get(&url)
            .header(header::ACCEPT, "text/plain")
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

/// Build the request URL by combining the base URL, tenant and topic.
/// Keeps string handling in one place and easy to test.
fn build_url(base_url: &str, tenant: &str, topic: &str) -> Result<String, HttpError> {
    if base_url.trim().is_empty() {
        return Err(HttpError::InvalidConfig("base_url must be set".into()));
    }
    if tenant.trim().is_empty() {
        return Err(HttpError::InvalidConfig("tenant must be set".into()));
    }
    if topic.trim().is_empty() {
        return Err(HttpError::InvalidConfig("topic must be set".into()));
    }

    let base = base_url.trim_end_matches('/');
    Ok(format!("{}/tenants/{}/topics/{}/messages", base, tenant, topic))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_building_is_stable() {
        let url = build_url("https://host/v1/", "T1", "a/b").unwrap();
        assert_eq!(url, "https://host/v1/tenants/T1/topics/a/b/messages");

        let url2 = build_url("https://host/v1", "T1", "a").unwrap();
        assert_eq!(url2, "https://host/v1/tenants/T1/topics/a/messages");
    }
}