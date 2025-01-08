//! # Dead Letter Queue
//! This optional module contains an implementation of pushing unprocessable/invalid messages towards a Dead Letter Queue (DLQ).
//! It is implemeted with [rdkafka] and [tokio].
//!
//! add feature `dlq` to your Cargo.toml to enable this module.
//!
//! ### NOTE:
//! This module is meant for pushing messages towards a dead/retry topic only, it does and WILL not handle any logic for retrying messages.
//! Reason is, it can differ per use case what strategy is needed to retry messages and handle the dead letters.
//!
//! It is up to the user to implement the strategy and logic for retrying messages.
//!
//! ### How it works
//! The DLQ struct can
//!
//! ## How to use
//! 1. Implement the [ErrorToDlq] trait on top your (custom) error type.
//! 2. Use the [Dlq::start] in your main or at start of your process logic. (this will start the DLQ in a separate tokio task)
//! 3. Get the dlq [DlqChannel] from the [Dlq::start] method and use this channel to communicate errored messages with the [Dlq] via the [ErrorToDlq::to_dlq] method.
//!
//! The topics are set via environment variables `DLQ_DEAD_TOPIC` and `DLQ_RETRY_TOPIC`.
//!
//! ### Example:
//! https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs

use std::collections::HashMap;
use std::str::from_utf8;

use log::{debug, error, info, warn};
use rdkafka::client::DefaultClientContext;
use rdkafka::error::KafkaError;
use rdkafka::message::{Header, Headers, Message, OwnedHeaders, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use tokio::sync::mpsc;

use crate::utils::get_env_var;
use crate::utils::graceful_shutdown::Shutdown;
use crate::DshKafkaConfig;

/// Channel to send messages to the dead letter queue
pub type DlqChannel = mpsc::Sender<SendToDlq>;

/// Trait to convert an error to a dlq message
/// This trait is implemented for all errors that can and should be converted to a dlq message
///
/// Example:
///```
/// use dsh_sdk::dlq;
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
    /// Convert error message to a dlq message
    fn to_dlq(&self, kafka_message: OwnedMessage) -> SendToDlq;
    /// Match error if the orignal message is able to be retried or not
    fn retryable(&self) -> Retryable;
}

/// Struct with required details to send a channel message to the dlq
/// Error needs to be send as string, as it is not possible to send a struct that implements Error trait
pub struct SendToDlq {
    kafka_message: OwnedMessage,
    retryable: Retryable,
    error: String,
    stack_trace: Option<String>,
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

    fn get_original_msg(&self) -> OwnedMessage {
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

/// The dead letter queue
///
/// ## How to use
/// 1. Implement the [ErrorToDlq] trait on top your (custom) error type.
/// 2. Use the [Dlq::start] in your main or at start of your process logic. (this will start the DLQ in a separate tokio task)
/// 3. Get the dlq [DlqChannel] from the [Dlq::start] method and use this channel to communicate errored messages with the [Dlq] via the [ErrorToDlq::to_dlq] method.
///
/// # Example
/// See full implementation example [here](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs)
pub struct Dlq {
    dlq_producer: FutureProducer,
    dlq_rx: mpsc::Receiver<SendToDlq>,
    dlq_dead_topic: String,
    dlq_retry_topic: String,
    _shutdown: Shutdown, // hold the shutdown alive until exit
}

impl Dlq {
    /// Start the dlq on a tokio task
    ///
    /// The DLQ will run until the return `Sender` is dropped.
    ///
    /// # Arguments
    /// * `shutdown` - The shutdown is required to keep the DLQ alive until the DLQ Sender is dropped
    ///
    /// # Returns
    /// * The [DlqChannel] to send messages to the DLQ
    ///
    /// # Note
    /// **NEVER** borrow the [DlqChannel] to your consumer, always use an owned [DlqChannel].
    /// This is required to stop the gracefull shutdown the DLQ as it depends on the [DlqChannel] to be dropped.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::utils::graceful_shutdown::Shutdown;
    /// use dsh_sdk::utils::dlq::{Dlq, DlqChannel, SendToDlq};
    ///
    /// async fn consume(dlq_channel: DlqChannel) {
    ///     // Your consumer logic together with error handling
    ///     loop {
    ///         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let shutdown = Shutdown::new();
    ///     let dlq_channel = Dlq::start(shutdown.clone()).unwrap();
    ///     
    ///     tokio::select! {
    ///        _ = async move {
    ///             // Your consumer logic together with the owned dlq_channel
    ///             dlq_channel
    ///       } => {}
    ///      _ = shutdown.signal_listener() => {
    ///        println!("Shutting down consumer");
    ///         }
    ///     }
    ///     // wait for graceful shutdown to complete
    ///     // NOTE that the `dlq_channel` will go out of scope when shutdown is called and the DLQ will stop
    ///     shutdown.complete().await;
    /// }
    /// ```
    pub fn start(shutdown: Shutdown) -> Result<DlqChannel, Box<dyn std::error::Error>> {
        let (dlq_tx, dlq_rx) = mpsc::channel(200);
        let dlq_producer: FutureProducer<DefaultClientContext, rdkafka::util::TokioRuntime> =
            ClientConfig::new().set_dsh_producer_config().create()?;
        let dlq_dead_topic = get_env_var("DLQ_DEAD_TOPIC")?;
        let dlq_retry_topic = get_env_var("DLQ_RETRY_TOPIC")?;
        let dlq = Self {
            dlq_producer,
            dlq_rx,
            dlq_dead_topic,
            dlq_retry_topic,
            _shutdown: shutdown,
        };
        tokio::spawn(dlq.run());
        Ok(dlq_tx)
    }

    /// Run the dlq. This will consume messages from the dlq channel and send them to the dlq topics
    /// This function will run until the shutdown channel is closed
    async fn run(mut self) {
        info!("DLQ started");
        loop {
            if let Some(mut dlq_message) = self.dlq_rx.recv().await {
                match self.send(&mut dlq_message).await {
                    Ok(_) => {}
                    Err(e) => error!("Error sending message to DLQ: {}", e),
                };
            } else {
                warn!("DLQ stopped as there is no active DLQ Channel");
                break;
            }
        }
    }
    /// Create and send message towards the dlq
    async fn send(&self, dlq_message: &mut SendToDlq) -> Result<(), KafkaError> {
        let orignal_kafka_msg: OwnedMessage = dlq_message.get_original_msg();
        let headers = orignal_kafka_msg
            .generate_dlq_headers(dlq_message)
            .to_owned_headers();
        let topic = self.dlq_topic(dlq_message.retryable);
        let key: &[u8] = orignal_kafka_msg.key().unwrap_or_default();
        let payload = orignal_kafka_msg.payload().unwrap_or_default();
        debug!("Sending message to DLQ topic: {}", topic);
        let record = FutureRecord::to(topic)
            .payload(payload)
            .key(key)
            .headers(headers);
        let send = self.dlq_producer.send(record, None).await;
        match send {
            Ok((p, o)) => warn!(
                "Message {:?} sent to DLQ topic: {}, partition: {}, offset: {}",
                from_utf8(key),
                topic,
                p,
                o
            ),
            Err((e, _)) => return Err(e),
        };
        Ok(())
    }

    fn dlq_topic(&self, retryable: Retryable) -> &str {
        match retryable {
            Retryable::Retryable => &self.dlq_retry_topic,
            Retryable::NonRetryable => &self.dlq_dead_topic,
            Retryable::Other => &self.dlq_dead_topic,
        }
    }
}

trait DlqHeaders {
    fn generate_dlq_headers<'a>(
        &'a self,
        dlq_message: &'a mut SendToDlq,
    ) -> HashMap<&'a str, Option<Vec<u8>>>;
}

impl DlqHeaders for OwnedMessage {
    fn generate_dlq_headers<'a>(
        &'a self,
        dlq_message: &'a mut SendToDlq,
    ) -> HashMap<&'a str, Option<Vec<u8>>> {
        let mut hashmap_headers: HashMap<&str, Option<Vec<u8>>> = HashMap::new();
        // Get original headers and add to hashmap
        if let Some(headers) = self.headers() {
            for header in headers.iter() {
                hashmap_headers.insert(header.key, header.value.map(|v| v.to_vec()));
            }
        }

        // Add dlq headers if not exist (we don't want to overwrite original dlq headers if message already failed earlier)
        let partition = self.partition().to_string().as_bytes().to_vec();
        let offset = self.offset().to_string().as_bytes().to_vec();
        let timestamp = self
            .timestamp()
            .to_millis()
            .unwrap_or(-1)
            .to_string()
            .as_bytes()
            .to_vec();
        hashmap_headers
            .entry("dlq_topic_origin")
            .or_insert_with(|| Some(self.topic().as_bytes().to_vec()));
        hashmap_headers
            .entry("dlq_partition_origin")
            .or_insert_with(move || Some(partition));
        hashmap_headers
            .entry("dlq_partition_offset_origin")
            .or_insert_with(move || Some(offset));
        hashmap_headers
            .entry("dlq_topic_origin")
            .or_insert_with(|| Some(self.topic().as_bytes().to_vec()));
        hashmap_headers
            .entry("dlq_timestamp_origin")
            .or_insert_with(move || Some(timestamp));
        // Overwrite if exist
        hashmap_headers.insert(
            "dlq_retryable",
            Some(dlq_message.retryable.to_string().as_bytes().to_vec()),
        );
        hashmap_headers.insert(
            "dlq_error",
            Some(dlq_message.error.to_string().as_bytes().to_vec()),
        );
        if let Some(stack_trace) = &dlq_message.stack_trace {
            hashmap_headers.insert("dlq_stack_trace", Some(stack_trace.as_bytes().to_vec()));
        }
        // update dlq_retries with +1 if exists, else add dlq_retries wiith 1
        let retries = hashmap_headers
            .get("dlq_retries")
            .map(|v| {
                let mut retries = [0; 4];
                retries.copy_from_slice(v.as_ref().unwrap());
                i32::from_be_bytes(retries)
            })
            .unwrap_or(0);
        hashmap_headers.insert("dlq_retries", Some((retries + 1).to_be_bytes().to_vec()));

        hashmap_headers
    }
}

trait HashMapToKafkaHeaders {
    fn to_owned_headers(&self) -> OwnedHeaders;
}

impl HashMapToKafkaHeaders for HashMap<&str, Option<Vec<u8>>> {
    fn to_owned_headers(&self) -> OwnedHeaders {
        // Convert to OwnedHeaders
        let mut owned_headers = OwnedHeaders::new_with_capacity(self.len());
        for header in self {
            let value = header.1.as_ref().map(|value| value.as_slice());
            owned_headers = owned_headers.insert(Header {
                key: header.0,
                value,
            });
        }
        owned_headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rdkafka::config::ClientConfig;
    use rdkafka::mocking::MockCluster;

    #[derive(Debug)]
    enum MockError {
        MockErrorRetryable(String),
        MockErrorDead(String),
    }
    impl MockError {
        fn to_string(&self) -> String {
            match self {
                MockError::MockErrorRetryable(e) => e.to_string(),
                MockError::MockErrorDead(e) => e.to_string(),
            }
        }
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

    #[test]
    fn test_dlq_get_original_msg() {
        let topic = "original_topic";
        let partition = 0;
        let offset = 123;
        let timestamp = 456;
        let mut original_headers: OwnedHeaders = OwnedHeaders::new();
        original_headers = original_headers.insert(Header {
            key: "some_key",
            value: Some("some_value".as_bytes()),
        });
        let owned_message = OwnedMessage::new(
            Some(vec![1, 2, 3]),
            Some(vec![4, 5, 6]),
            topic.to_string(),
            rdkafka::Timestamp::CreateTime(timestamp),
            partition,
            offset,
            Some(original_headers),
        );
        let dlq_message =
            MockError::MockErrorRetryable("some_error".to_string()).to_dlq(owned_message.clone());
        let result = dlq_message.get_original_msg();
        assert_eq!(
            result.payload(),
            dlq_message.kafka_message.payload(),
            "payoad does not match"
        );
        assert_eq!(
            result.key(),
            dlq_message.kafka_message.key(),
            "key does not match"
        );
        assert_eq!(
            result.topic(),
            dlq_message.kafka_message.topic(),
            "topic does not match"
        );
        assert_eq!(
            result.partition(),
            dlq_message.kafka_message.partition(),
            "partition does not match"
        );
        assert_eq!(
            result.offset(),
            dlq_message.kafka_message.offset(),
            "offset does not match"
        );
        assert_eq!(
            result.timestamp(),
            dlq_message.kafka_message.timestamp(),
            "timestamp does not match"
        );
    }

    #[test]
    fn test_dlq_hashmap_to_owned_headers() {
        let mut hashmap: HashMap<&str, Option<Vec<u8>>> = HashMap::new();
        hashmap.insert("some_key", Some(b"key_value".to_vec()));
        hashmap.insert("some_other_key", None);
        let result: Vec<(&str, Option<&[u8]>)> =
            vec![("some_key", Some(b"key_value")), ("some_other_key", None)];

        let owned_headers = hashmap.to_owned_headers();
        for header in owned_headers.iter() {
            assert!(result.contains(&(header.key, header.value)));
        }
    }

    #[test]
    fn test_dlq_topic() {
        let mock_cluster = MockCluster::new(1).unwrap();
        let mut producer = ClientConfig::new();
        producer.set("bootstrap.servers", mock_cluster.bootstrap_servers());
        let producer = producer.create().unwrap();
        let dlq = Dlq {
            dlq_producer: producer,
            dlq_rx: mpsc::channel(200).1,
            dlq_dead_topic: "dead_topic".to_string(),
            dlq_retry_topic: "retry_topic".to_string(),
            _shutdown: Shutdown::new(),
        };
        let error = MockError::MockErrorRetryable("some_error".to_string());
        let topic = dlq.dlq_topic(error.retryable());
        assert_eq!(topic, "retry_topic");
        let error = MockError::MockErrorDead("some_error".to_string());
        let topic = dlq.dlq_topic(error.retryable());
        assert_eq!(topic, "dead_topic");
    }

    #[test]
    fn test_dlq_generate_dlq_headers() {
        let topic = "original_topic";
        let partition = 0;
        let offset = 123;
        let timestamp = 456;
        let error = Box::new(MockError::MockErrorRetryable("some_error".to_string()));

        let mut original_headers: OwnedHeaders = OwnedHeaders::new();
        original_headers = original_headers.insert(Header {
            key: "some_key",
            value: Some("some_value".as_bytes()),
        });

        let owned_message = OwnedMessage::new(
            Some(vec![1, 2, 3]),
            Some(vec![4, 5, 6]),
            topic.to_string(),
            rdkafka::Timestamp::CreateTime(timestamp),
            partition,
            offset,
            Some(original_headers),
        );

        let mut dlq_message = error.to_dlq(owned_message.clone());

        let mut expected_headers: HashMap<&str, Option<Vec<u8>>> = HashMap::new();
        expected_headers.insert("some_key", Some(b"some_value".to_vec()));
        expected_headers.insert("dlq_topic_origin", Some(topic.as_bytes().to_vec()));
        expected_headers.insert(
            "dlq_partition_origin",
            Some(partition.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert(
            "dlq_partition_offset_origin",
            Some(offset.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert(
            "dlq_timestamp_origin",
            Some(timestamp.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert(
            "dlq_retryable",
            Some(Retryable::Retryable.to_string().as_bytes().to_vec()),
        );
        expected_headers.insert("dlq_retries", Some(1_i32.to_be_bytes().to_vec()));
        expected_headers.insert("dlq_error", Some(error.to_string().as_bytes().to_vec()));
        if let Some(stack_trace) = &dlq_message.stack_trace {
            expected_headers.insert("dlq_stack_trace", Some(stack_trace.as_bytes().to_vec()));
        }

        let result = owned_message.generate_dlq_headers(&mut dlq_message);
        for header in result.iter() {
            assert_eq!(
                header.1,
                expected_headers.get(header.0).unwrap_or(&None),
                "Header {} does not match",
                header.0
            );
        }

        // Test if dlq headers are correctly overwritten when to be retried message was already retried before
        let mut original_headers: OwnedHeaders = OwnedHeaders::new();
        original_headers = original_headers.insert(Header {
            key: "dlq_error",
            value: Some(
                "to_be_overwritten_error_as_this_was_the_original_error_from_1st_retry".as_bytes(),
            ),
        });
        original_headers = original_headers.insert(Header {
            key: "dlq_topic_origin",
            value: Some(topic.as_bytes()),
        });
        original_headers = original_headers.insert(Header {
            key: "dlq_retries",
            value: Some(&1_i32.to_be_bytes().to_vec()),
        });

        let owned_message = OwnedMessage::new(
            Some(vec![1, 2, 3]),
            Some(vec![4, 5, 6]),
            "retry_topic".to_string(),
            rdkafka::Timestamp::CreateTime(timestamp),
            partition,
            offset,
            Some(original_headers),
        );
        let result = owned_message.generate_dlq_headers(&mut dlq_message);
        assert_eq!(
            result.get("dlq_error").unwrap(),
            &Some(error.to_string().as_bytes().to_vec())
        );
        assert_eq!(
            result.get("dlq_topic_origin").unwrap(),
            &Some(topic.as_bytes().to_vec())
        );
        assert_eq!(
            result.get("dlq_retries").unwrap(),
            &Some(2_i32.to_be_bytes().to_vec())
        );
    }
}
