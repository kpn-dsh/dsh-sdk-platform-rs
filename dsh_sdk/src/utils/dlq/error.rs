/// Errors Dead Letter Queue client
#[derive(Debug, thiserror::Error)]
pub enum DlqErrror {
    #[error("Kafka Error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),
    #[error("DSH Error: {0}")]
    Dsh(#[from] crate::error::DshError),
    #[error("Utils Error: {0}")]
    Utils(#[from] crate::utils::UtilsError),
}
