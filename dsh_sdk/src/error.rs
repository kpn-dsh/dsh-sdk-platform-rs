use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DshError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Env var error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Convert bytes to utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
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
    #[error("Invalid PEM certificate: {0}")]
    PemError(#[from] pem::PemError),
    #[cfg(feature = "bootstrap")]
    #[error("Reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "bootstrap")]
    #[error("Serde_json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "bootstrap")]
    #[error("Rcgen error: {0}")]
    PrivateKeyError(#[from] rcgen::Error),
    #[cfg(feature = "bootstrap")]
    #[error("Error parsing Distinguished Name: {0}")]
    ParseDnError(String),
    #[cfg(feature = "bootstrap")]
    #[error("Error getting group id, index out of bounds for {0}")]
    IndexGroupIdError(crate::dsh::datastream::GroupType),
    #[error("No tenant name found")]
    NoTenantName,
    #[cfg(feature = "bootstrap")]
    #[error("Error getting topic name {0}, Topic not found in datastreams.")]
    NotFoundTopicError(String),
    #[cfg(feature = "bootstrap")]
    #[error("Error in topic permissions: {0} does not have {1:?} permissions.")]
    TopicPermissionsError(String, crate::dsh::datastream::ReadWriteAccess),
    #[cfg(feature = "metrics")]
    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheus::Error),
    #[cfg(feature = "metrics")]
    #[error("Hyper error: {0}")]
    HyperError(#[from] hyper::http::Error),
}

#[cfg(feature = "rest-token-fetcher")]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DshRestTokenError {
    #[error("Client ID is unknown")]
    UnknownClientId,
    #[error("Client secret not set")]
    UnknownClientSecret,
    #[error("Unexpected failure while fetching token from server: {0}")]
    FailureTokenFetch(reqwest::Error),
    #[error("Unexpected status code: {status_code}, error body: {error_body:#?}")]
    StatusCode {
        status_code: reqwest::StatusCode,
        error_body: reqwest::Response,
    },
}
