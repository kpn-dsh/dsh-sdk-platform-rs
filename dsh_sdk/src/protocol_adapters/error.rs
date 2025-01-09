/// Error type for the protocol adapter token fetcher
#[derive(Debug, thiserror::Error)]
pub enum ProtocolTokenError {
    #[error("Error calling: {url}, status code: {status_code}, error body: {error_body}")]
    DshCall {
        url: String,
        status_code: reqwest::StatusCode,
        error_body: String,
    },
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde_json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JWT Parse error: {0}")]
    Jwt(String),
}
