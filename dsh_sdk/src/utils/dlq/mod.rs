//! # Dead Letter Queue
//! This optional module contains an implementation of pushing unprocessable/invalid messages towards a Dead Letter Queue (DLQ).
//! It is implemeted with [rdkafka] and [tokio].
//!
//! ## Feature flag
//! Add feature `dlq` to your Cargo.toml to enable this module.
//!
//! ### NOTE:
//! This module is meant for pushing messages towards a dead/retry topic only, it does and WILL not handle any logic for retrying messages.
//! Reason is, it can differ per use case what strategy is needed to retry messages and handle the dead letters.
//!
//! It is up to the user to implement the strategy and logic for retrying messages.
//!
//! ## How to use
//! 1. Implement the [ErrorToDlq] trait on top your (custom) error type.
//! 2. Use the [Dlq::start] in your main or at start of your process logic. (this will start the DLQ in a separate tokio task)
//! 3. Get the dlq [DlqChannel] from the [Dlq::start] method and use this channel to communicate errored messages with the [Dlq] via the [ErrorToDlq::to_dlq] method which is implemented on your Error.
//!
//! The topics are set via environment variables `DLQ_DEAD_TOPIC` and `DLQ_RETRY_TOPIC`.
//! 
//! ## Importance of `DlqChannel` in the graceful shutdown procedure
//! The [`Dlq::start`] will return a [`DlqChannel`]. The [`Dlq`] will keep running till the moment [`DlqChannel`] is dropped and finished processing all messages. 
//! This also means that the [`Shutdown`] procedure will wait for the [`Dlq`] to finish processing all messages before the application is shut down. 
//! This is to make sure that **all** messages are properly processed before the application is shut down.
//! 
//! **NEVER** borrow the [`DlqChannel`] but provide the channel as owned/cloned version to your processing logic and **NEVER** keep an owned version in main function, as this will result in a **deadlock** and your application will never shut down. 
//! It is fine to start the [`Dlq`] in the main function, but make sure the [`DlqChannel`] is moved to your processing logic.
//!
//! ### Example:
//! <https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs>
mod dlq;
mod error;
mod headers;
mod types;

#[doc(inline)]
pub use dlq::Dlq;
#[doc(inline)]
pub use error::DlqErrror;
#[doc(inline)]
pub use types::*;
/// Channel to send [SendToDlq] messages to the dead letter queue
pub type DlqChannel = tokio::sync::mpsc::Sender<SendToDlq>;

// Mock error avaialbnle in tests
#[cfg(test)]
mod tests {
    use super::*;
    use rdkafka::message::OwnedMessage;

    #[derive(Debug)]
    pub enum MockError {
        MockErrorRetryable(String),
        MockErrorDead(String),
    }

    impl std::fmt::Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MockError::MockErrorRetryable(e) => write!(f, "{}", e),
                MockError::MockErrorDead(e) => write!(f, "{}", e),
            }
        }
    }

    impl ErrorToDlq for MockError {
        fn to_dlq(&self, kafka_message: OwnedMessage) -> SendToDlq {
            let backtrace = "some_backtrace";
            SendToDlq::new(
                kafka_message,
                self.retryable(),
                self.to_string(),
                Some(backtrace.to_string()),
            )
        }

        fn retryable(&self) -> Retryable {
            match self {
                MockError::MockErrorRetryable(_) => Retryable::Retryable,
                MockError::MockErrorDead(_) => Retryable::NonRetryable,
            }
        }
    }
}
