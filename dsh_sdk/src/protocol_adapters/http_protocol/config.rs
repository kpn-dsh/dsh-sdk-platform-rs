//! HTTP adapter configuration (protocol-agnostic; no `reqwest` imports).
//!
//! This keeps configuration independent of any HTTP client crate so we can
//! easily swap implementations, and it mirrors the SDK's pattern for other
//! protocols. The Messaging API requires an MQTT token (not API key / REST token).

use std::path::PathBuf;

/// Minimal configuration for the HTTP adapter.
///
/// Keep this struct small and explicit for Rust beginners. All fields are public
/// to make experimentation easy; builder-style methods improve readability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpConfig {
    /// Base URL of the DSH Messaging API, typically including API version.
    /// Example: "https://api.<platform>/data/v0" or "https://dsh.example.com/v1"
    pub base_url: String,

    /// DSH tenant (e.g., "GREENBOX-DEV").
    pub tenant: String,

    /// MQTT token used for `Authorization: Bearer <token>`.
    /// This must be provided by the SDK's token fetcher (or your trusted service).
    pub mqtt_token: Option<String>,

    /// Optional custom CA certificate (PEM) path for TLS validation (internal environments).
    pub ca_cert_path: Option<PathBuf>,

    /// Network timeout, in seconds.
    pub timeout_secs: u64,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            tenant: String::new(),
            mqtt_token: None,
            ca_cert_path: None,
            timeout_secs: 10,
        }
    }
}

impl HttpConfig {
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn with_tenant(mut self, tenant: impl Into<String>) -> Self {
        self.tenant = tenant.into();
        self
    }

    pub fn with_mqtt_token(mut self, token: impl Into<String>) -> Self {
        self.mqtt_token = Some(token.into());
        self
    }

    pub fn with_ca_cert_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.ca_cert_path = Some(path.into());
        self
    }

    pub fn with_timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::HttpConfig;

    #[test]
    fn config_builds_minimally() {
        let cfg = HttpConfig::default()
            .with_base_url("https://host/v1")
            .with_tenant("GREENBOX-DEV")
            .with_timeout_secs(5);
        assert_eq!(cfg.base_url, "https://host/v1");
        assert_eq!(cfg.tenant, "GREENBOX-DEV");
        assert_eq!(cfg.timeout_secs, 5);
        assert!(cfg.mqtt_token.is_none());
    }

    #[test]
    fn config_sets_mqtt_token() {
        let cfg = HttpConfig::default().with_mqtt_token("abc");
        assert_eq!(cfg.mqtt_token.as_deref(), Some("abc"));
    }
}