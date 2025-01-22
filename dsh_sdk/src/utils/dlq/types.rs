use log::{debug, error};
use rdkafka::message::OwnedMessage;

use super::DlqChannel;

/// Trait to convert an error to a dlq message
/// This trait is implemented for all errors that can and should be converted to a dlq message
///
/// Example:
///```
/// use dsh_sdk::utils::dlq;
/// use std::backtrace::Backtrace;
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// enum ConsumerError {
///     #[error("Deserialization error: {0}")]
///     DeserializeError(String),
/// }
///
/// impl dlq::ErrorToDlq for ConsumerError {
///     fn to_dlq(&self, kafka_message: rdkafka::message::OwnedMessage) ->  dlq::SendToDlq {
///         dlq::SendToDlq::new(kafka_message, self.retryable(), self.to_string(), None)
///     }
///     fn retryable(&self) -> dlq::Retryable {
///         match self {
///             ConsumerError::DeserializeError(e) => dlq::Retryable::NonRetryable,
///         }
///     }
/// }
/// ```
pub trait ErrorToDlq {
    /// Convert Error message to a dlq message
    fn to_dlq(&self, kafka_message: OwnedMessage) -> SendToDlq;
    /// Match Error if the orignal message is able to be retried
    fn retryable(&self) -> Retryable;
}

/// DLQ Message that can be send to the [DlqChannel]
pub struct SendToDlq {
    pub kafka_message: OwnedMessage,
    pub retryable: Retryable,
    pub error: String,
    pub stack_trace: Option<String>,
}

impl SendToDlq {
    /// Create new SendToDlq message
    pub fn new(
        kafka_message: OwnedMessage,
        retryable: Retryable,
        error: String,
        stack_trace: Option<String>,
    ) -> Self {
        Self {
            kafka_message,
            retryable,
            error,
            stack_trace,
        }
    }
    /// Send message to dlq channel
    pub async fn send(self, dlq_tx: &mut DlqChannel) {
        match dlq_tx.send(self).await {
            Ok(_) => debug!("Message sent to DLQ channel"),
            Err(e) => error!("Error sending message to DLQ: {}", e),
        }
    }

    pub(crate) fn get_original_msg(&self) -> OwnedMessage {
        self.kafka_message.clone()
    }
}

/// Helper enum to decide to which topic the message should be sent to.
#[derive(Debug, Clone, Copy)]
pub enum Retryable {
    Retryable,
    NonRetryable,
    Other,
}

impl std::fmt::Display for Retryable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Retryable::Retryable => write!(f, "Retryable"),
            Retryable::NonRetryable => write!(f, "NonRetryable"),
            Retryable::Other => write!(f, "Other"),
        }
    }
}
