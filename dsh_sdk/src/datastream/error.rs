// Errors relataed to datastreams 
#[derive(Debug, thiserror::Error)]
pub enum DatastreamError {
    #[error("Error getting group id, index out of bounds for {0}")]
    IndexGroupIdError(crate::datastream::GroupType),
    #[error("Error getting topic name {0}, Topic not found in datastreams.")]
    NotFoundTopicError(String),
    #[error("Error in topic permissions: {0} does not have {1:?} permissions.")]
    TopicPermissionsError(String, crate::datastream::ReadWriteAccess),
    #[error("Error calling: {url}, status code: {status_code}, error body: {error_body}")]
    DshCallError {
        url: String,
        status_code: reqwest::StatusCode,
        error_body: String,
    },
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde_json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}
