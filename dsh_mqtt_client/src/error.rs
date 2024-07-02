use rumqttc::ClientError;

#[derive(Debug)]
pub enum DshError {
    RequestError(String),
    TokenError(String),
    SerdeJson(serde_json::Error),
    IoError(std::io::Error),
    Request(reqwest::Error),
    Utf8Error(String),
    PublishError(String),
    SubscribeError(String),
    FailedToDisconnect(String),
    StreamConnectionError(String),
    ClientError(ClientError),
    CertificateAddError(String),
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
impl From<rumqttc::ClientError> for DshError {
    fn from(error: rumqttc::ClientError) -> Self {
        DshError::ClientError(error)
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
            DshError::PublishError(e) => write!(f, "Error while publishing: {}", e),
            DshError::Utf8Error(e) => write!(f, "Error while receiving message: {}", e),
            DshError::SubscribeError(e) => write!(f, "Couldn't subscribe to topic: {}", e),
            DshError::FailedToDisconnect(e) => write!(f, "Failed to disconnect MQTT client: {}", e),
            DshError::StreamConnectionError(e) => write!(f, "Event loop disconnected: {}", e),
            DshError::ClientError(e) => write!(f, "Mqtt Client Error: {}", e),
            DshError::CertificateAddError(e) => write!(f, "Failed to load TLS certificates: {}", e),
        }
    }
}
