use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ManagementApiTokenError {
    #[error("Client ID is unknown")]
    UnknownClientId,
    #[error("Client secret not set")]
    UnknownClientSecret,
    #[error("Unexpected failure while fetching token from server: {0}")]
    FailureTokenFetch(reqwest::Error),
    #[error("Unexpected status code: {status_code}, error body: {error_body}")]
    StatusCode {
        status_code: reqwest::StatusCode,
        error_body: String,
    },
}
