#[derive(Debug)]
pub enum DshError {
    RequestError(String),
    TokenError(String),
    SerdeJson(serde_json::Error),
    IoError(std::io::Error),
    Request(reqwest::Error),
}

impl From<std::io::Error> for DshError {
    fn from(err: std::io::Error) -> DshError {
        DshError::IoError(err)
    }
}
impl From<reqwest::Error> for DshError {
    fn from(e: reqwest::Error) -> Self {
        DshError::Request(e)
    }
}
impl From<serde_json::Error> for DshError {
    fn from(error: serde_json::Error) -> Self {
        DshError::SerdeJson(error)
    }
}
impl std::fmt::Display for DshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DshError::RequestError(e) => write!(f, "Reqwest error: {}", e),
            DshError::TokenError(e) => write!(f, "TokenError error: {}", e),
            DshError::SerdeJson(e) => write!(f, "SerdeJsonError: {}", e),
            DshError::IoError(e) => write!(f, "Io error: {}", e),
            DshError::Request(e) => write!(f, "Reqwest error: {}", e),
        }
    }
}
