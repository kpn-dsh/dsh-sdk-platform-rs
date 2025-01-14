//! Dead Letter Queue (DLQ) client for handling messages that cannot be processed successfully.
//!
//! The [`Dlq`] provides an asynchronous mechanism to route unprocessable messages to special
//! “dead” or “retry” kafka topics. It coordinates with [`Shutdown`](crate::utils::graceful_shutdown::Shutdown)
//! to ensure messages are handled before the application exits.
//!
//! # Overview
//!  
//! | **Component**        | **Description**                                                                       |
//! |----------------------|---------------------------------------------------------------------------------------|
//! | [`Dlq`]             | Main struct managing the producer, dead/retry topics, and queue of failed messages.   |
//! | [`DlqChannel`]      | An `mpsc` sender returned by [`Dlq::start`], used by tasks to submit errored messages.|
//! | [`SendToDlq`]       | Wrapper carrying both the original Kafka message and error details.                   |
//! | [`Retryable`]       | Enum indicating whether a message is retryable or should be permanently “dead.”       |
//!
//! # Usage Flow
//! 1. **Implement** the [`ErrorToDlq`] trait on your custom error type.  
//! 2. **Start** the DLQ by calling [`Dlq::start`], which returns a [`DlqChannel`].  
//! 3. **Own** the [`DlqChannel`] in your processing logic (do **not** hold it in `main`!), and
//!    call [`ErrorToDlq::to_dlq`] when you need to push a message/error into the queue.  
//! 4. **Graceful Shutdown**: The [`DlqChannel`] should naturally drop during shutdown, letting
//!    the `Dlq` finish processing any remaining messages before the application fully closes.  
//!
//! The topics are set via environment variables `DLQ_DEAD_TOPIC` and `DLQ_RETRY_TOPIC`.
//!
//! # Important Graceful Shutdown Notes
//! - The [`Dlq`] remains active until the [`DlqChannel`] is dropped and all messages are processed.  
//! - Keep the [`DlqChannel`] **in your worker logic** and not in `main`, preventing deadlocks.  
//! - The [`Shutdown`](crate::utils::graceful_shutdown::Shutdown) will wait for the DLQ to finish once
//!   all channels have closed, ensuring no messages are lost.  
//!
//! # Example:
//! A detailed implementation example can be found in the [DLQ example](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs)
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
