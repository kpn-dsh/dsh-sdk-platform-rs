//! HTTP adapter configuration (kept free of any `reqwest` imports).
//!
//! This config mirrors the HTTP Messaging API expectations:
//! * `base_url` like `https://api.<platform-url>`
//! * `stream` is the DSH stream name
//! * `mqtt_token` must be provided by the SDK's token fetcher / trusted service
//! * Optional custom CA for internal TLS chains
//! * Configurable timeout and `Accept` header

use std::path::PathBuf;

/// Content types the adapter can request.
///
/// The spec examples commonly show `application/json` and `text/plain`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accept {
    TextPlain,
    ApplicationJson,
}

impl Accept {
    pub fn as_str(&self) -> &'static str {
        match self {
            Accept::TextPlain => "text/plain",
            Accept::ApplicationJson => "application/json",
        }
    }
}

/// Minimal configuration for the HTTP adapter.
///
/// Keep this struct small and explicit for Rust beginners. All fields are public
/// to make experimentation easy; builder-style methods improve readability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpConfig {
    /// Base URL of the DSH HTTP Messaging API host **without** path.
    /// Example: `https://api.dsh-dev.np.aws.kpn.com`
    pub base_url: String,

    /// DSH stream name (e.g., "weather").
    pub stream: String,

    /// MQTT token used for `Authorization: Bearer <token>`.
    /// This must be provided by the SDK's token fetcher (or your trusted service).
    pub mqtt_token: Option<String>,

    /// Optional custom CA certificate (PEM) path for TLS validation (internal environments).
    pub ca_cert_path: Option<PathBuf>,

    /// Network timeout, in seconds.
    pub timeout_secs: u64,

    /// Preferred response type via `Accept` header.
    pub accept: Accept,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            stream: String::new(),
            mqtt_token: None,
            ca_cert_path: None,
            timeout_secs: 10,
            accept: Accept::TextPlain,
        }
    }
}

impl HttpConfig {
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn with_stream(mut self, stream: impl Into<String>) -> Self {
        self.stream = stream.into();
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

    pub fn with_accept(mut self, accept: Accept) -> Self {
        self.accept = accept;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Accept, HttpConfig};

    #[test]
    fn config_builds_minimally() {
        let cfg = HttpConfig::default()
            .with_base_url("https://api.host")
            .with_stream("weather")
            .with_timeout_secs(5)
            .with_accept(Accept::ApplicationJson);

        assert_eq!(cfg.base_url, "https://api.host");
        assert_eq!(cfg.stream, "weather");
        assert_eq!(cfg.timeout_secs, 5);
        assert_eq!(cfg.accept, Accept::ApplicationJson);
        assert!(cfg.mqtt_token.is_none());
    }

    #[test]
    fn config_sets_mqtt_token() {
        let cfg = HttpConfig::default().with_mqtt_token("abc");
        assert_eq!(cfg.mqtt_token.as_deref(), Some("abc"));
    }

    #[test]
    fn accept_to_header_value() {
        assert_eq!(Accept::TextPlain.as_str(), "text/plain");
        assert_eq!(Accept::ApplicationJson.as_str(), "application/json");
    }
}