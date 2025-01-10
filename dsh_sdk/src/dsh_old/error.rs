use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DshError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Env var error: {0}")]
    EnvVarError(&'static str),
    #[error("Convert bytes to utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Error calling: {url}, status code: {status_code}, error body: {error_body}")]
    DshCallError {
        url: String,
        status_code: reqwest::StatusCode,
        error_body: String,
    },
    #[error("Certificates are not set")]
    NoCertificates,
    #[error("Invalid PEM certificate: {0}")]
    PemError(#[from] pem::PemError),
    #[error("Reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde_json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Rcgen error: {0}")]
    PrivateKeyError(#[from] rcgen::Error),
    #[error("Error parsing: {0}")]
    ParseDnError(String),
    #[error("Error getting group id, index out of bounds for {0}")]
    IndexGroupIdError(super::datastream::GroupType),
    #[error("No tenant name found")]
    NoTenantName,
    #[error("Error getting topic name {0}, Topic not found in datastreams.")]
    NotFoundTopicError(String),
    #[error("Error in topic permissions: {0} does not have {1:?} permissions.")]
    TopicPermissionsError(String, super::datastream::ReadWriteAccess),
}
