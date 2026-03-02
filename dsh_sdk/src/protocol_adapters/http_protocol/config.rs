use std::path::PathBuf;

/// Accepted content types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accept {
    TextPlain,
    ApplicationJson,
}

impl Accept {
    pub fn as_str(self) -> &'static str {
        match self {
            Accept::TextPlain => "text/plain",
            Accept::ApplicationJson => "application/json",
        }
    }
}

#[derive(Clone, Debug)]
pub struct HttpConfig {
    base_url: String,
    stream: String,
    mqtt_token: String,
    ca_cert_path: Option<PathBuf>,
    timeout_secs: u64,
    accept: Accept,
}

impl HttpConfig {
    pub fn new(
        base_url: impl Into<String>,
        stream: impl Into<String>,
        mqtt_token: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            stream: stream.into(),
            mqtt_token: mqtt_token.into(),
            ca_cert_path: None,
            timeout_secs: 10,
            accept: Accept::TextPlain,
        }
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

    // Internal accessors
    pub(crate) fn base_url(&self) -> &str { &self.base_url }
    pub(crate) fn stream(&self) -> &str { &self.stream }
    pub(crate) fn token(&self) -> &str { &self.mqtt_token }
    pub(crate) fn timeout(&self) -> u64 { self.timeout_secs }
    pub(crate) fn accept(&self) -> Accept { self.accept }
    pub(crate) fn ca_cert(&self) -> Option<&PathBuf> { self.ca_cert_path.as_ref() }
}