use thiserror::Error;

#[derive(Error, Debug)]
pub enum DshError {
    #[error("Error calling: {url}, status code: {status_code}, error body: {error_body}")]
    DshCallError {
        url: String,
        status_code: reqwest::StatusCode,
        error_body: String,
    },
    #[error("Reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
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
    IndexGroupIdError(crate::bootstrap::GroupType),
}
