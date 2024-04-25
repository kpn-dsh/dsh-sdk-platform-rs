use thiserror::Error;

use crate::dsh::datastream::ReadWriteAccess;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DshError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Env var error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[cfg(feature = "bootstrap")]
    #[error("Error calling: {url}, status code: {status_code}, error body: {error_body}")]
    DshCallError {
        url: String,
        status_code: reqwest::StatusCode,
        error_body: String,
    },
    #[cfg(feature = "bootstrap")]
    #[error("Certificates are not set")]
    NoCertificates,
    #[cfg(feature = "bootstrap")]
    #[error("Reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "bootstrap")]
    #[error("Serde_json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "bootstrap")]
    #[error("Error generating private key: {0}")]
    PrivateKeyError(#[from] picky::key::KeyError),
    #[cfg(feature = "bootstrap")]
    #[error("Error with Certificate Sign Request: {0}")]
    CsrError(#[from] picky::x509::csr::CsrError),
    #[error("Error parsing Distinguished Name: {0}")]
    ParseDnError(String),
    #[cfg(feature = "bootstrap")]
    #[error("Error getting group id, index out of bounds for {0}")]
    IndexGroupIdError(crate::dsh::datastream::GroupType),
    #[error("Error getting topic name {0}, Topic not found in datastreams.")]
    NotFoundTopicError(String),
    #[error("Error in topic permissions: {0} does not have {1:?} permissions.")]
    TopicPermissionsError(String, ReadWriteAccess),
    #[error("Convert bytes to utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[cfg(feature = "metrics")]
    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheus::Error),
    #[cfg(feature = "metrics")]
    #[error("Hyper error: {0}")]
    HyperError(#[from] hyper::http::Error),
}
