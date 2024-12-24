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
    #[error("Empty payload")]
    EmptyPayload,
    #[error("Failed to decode payload: {0}")]
    FailedToDecode(String),
    #[error("Failed to parse value onto struct")]
    FailedParseToStruct,

    #[error("Protobuf to struct not (yet) implemented")]
    NotImplementedProtobufDeserialize,
}
