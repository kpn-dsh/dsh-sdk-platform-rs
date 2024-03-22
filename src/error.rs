use thiserror::Error;

#[derive(Error, Debug)]
pub enum DshError {
    #[error("Error calling: {url}, status code: {status_code}, error body: {error_body}")]
    DshCallError {
        url: String,
        status_code: reqwest::StatusCode,
        error_body: String,
    },
    #[error("Certificates are not set")]
    NoCertificates,
    #[error("Reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("IO error for file {0}: {1}")]
    IoErrorFile(&'static str, std::io::Error),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde_json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Env var error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Error generating private key: {0}")]
    PrivateKeyError(#[from] picky::key::KeyError),
    #[error("Error with Certificate Sign Request: {0}")]
    CsrError(#[from] picky::x509::csr::CsrError),
    #[error("Error parsing Distinguished Name: {0}")]
    ParseDnError(String),
    #[error("Error getting group id, index out of bounds for {0}")]
    IndexGroupIdError(crate::dsh::datastream::GroupType),
    #[error("Error getting topic name {0}, Topic not found in datastreams.")]
    NotFoundTopicError(String),
    #[cfg(feature = "metrics")]
    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheus::Error),
    #[error("Convert bytes to utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[cfg(feature = "metrics")]
    #[error("Hyper error: {0}")]
    HyperError(#[from] hyper::http::Error),
}

impl From<(&'static str, std::io::Error)> for DshError {
    fn from(error: (&'static str, std::io::Error)) -> Self {
        DshError::IoErrorFile(error.0, error.1)
    }
}
