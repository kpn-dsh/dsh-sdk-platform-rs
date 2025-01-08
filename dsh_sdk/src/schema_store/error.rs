use thiserror::Error;

/// Error type for the SchemaStore
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SchemaStoreError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Could not parse raw schema to a valid schema {:?}", .0)]
    FailedToParseSchema(Option<super::types::SchemaType>),
    #[error("Invalid status code: {status_code} for {url} ({error})")]
    InvalidStatusCode {
        status_code: u16,
        url: String,
        error: String,
    },
    #[error("Invalid subject name: {0}")]
    InvalidSubjectName(String),
}
