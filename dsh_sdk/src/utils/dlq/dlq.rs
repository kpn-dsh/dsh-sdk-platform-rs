//! Dead Letter Queue (DLQ) client for handling messages that cannot be processed successfully.

use std::str::from_utf8;

use log::{debug, error, info, warn};
use rdkafka::client::DefaultClientContext;
use rdkafka::error::KafkaError;
use rdkafka::message::{Message, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use tokio::sync::mpsc;

use super::headers::{DlqHeaders, HashMapToKafkaHeaders};
use super::{DlqChannel, DlqErrror, Retryable, SendToDlq};
use crate::utils::get_env_var;
use crate::utils::graceful_shutdown::Shutdown;
use crate::DshKafkaConfig;

/// Dead Letter Queue (DLQ) struct that runs asynchronously and processes error messages.
///
/// Once started via [`Dlq::start`], it listens on an `mpsc` channel for [`SendToDlq`] items.
/// Each received item is routed to the configured “dead” or “retry” Kafka topics,
/// depending on whether it is [`Retryable::Retryable`] or not.
///
/// A full implementation can be found in the [DLQ example]((https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs).
pub struct Dlq {
    dlq_producer: FutureProducer,
    dlq_rx: mpsc::Receiver<SendToDlq>,
    dlq_dead_topic: String,
    dlq_retry_topic: String,
    // Holding the shutdown handle ensures it remains valid until the DLQ stops.
    _shutdown: Shutdown,
}

impl Dlq {
    /// Spawns the DLQ in a dedicated Tokio task, returning a [`DlqChannel`] for sending error messages.
    ///
    /// - Internally creates a Kafka producer with [`set_dsh_producer_config`](DshKafkaConfig::set_dsh_producer_config).
    /// - Reads environment variables for `DLQ_DEAD_TOPIC` and `DLQ_RETRY_TOPIC` to determine the topics
    ///   used for permanently-dead and retryable messages.
    ///
    /// # Returns
    /// A `DlqChannel` (an `mpsc::Sender<SendToDlq>`) used by your worker logic to push errored messages.
    ///
    /// # Shutdown Procedure
    /// The DLQ runs until its channel is dropped. Once the channel closes, the DLQ finishes pending
    /// messages and then stops. The [`Shutdown`](crate::utils::graceful_shutdown::Shutdown) handle
    /// ensures that the main application waits for the DLQ to finish.
    ///
    /// # Errors
    /// Returns a [`DlqErrror`] if the producer could not be created or if environment variables
    /// (`DLQ_DEAD_TOPIC`, `DLQ_RETRY_TOPIC`) are missing.
    ///
    /// # Example
    /// A full implementation can be found in the [DLQ example](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs),
    /// ```
    /// use dsh_sdk::utils::graceful_shutdown::Shutdown;
    /// use dsh_sdk::utils::dlq::Dlq;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let shutdown = Shutdown::new();
    ///     let dlq_channel = Dlq::start(shutdown.clone()).expect("Failed to start DLQ");
    ///
    ///     // Spawn your worker logic, pass `dlq_channel` to handle errors...
    /// }
    /// ```
    pub fn start(shutdown: Shutdown) -> Result<DlqChannel, DlqErrror> {
        let (dlq_tx, dlq_rx) = mpsc::channel(200);

        // Build a Kafka producer with standard DSH config
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

        // Spawn the main DLQ processing loop
        tokio::spawn(dlq.run());
        Ok(dlq_tx)
    }

    /// Core loop for receiving `SendToDlq` messages and forwarding them to the correct Kafka topic.
    ///
    /// Runs until the `mpsc::Receiver` is closed (no more references to the channel exist).
    async fn run(mut self) {
        info!("DLQ started and awaiting messages...");
        while let Some(mut dlq_message) = self.dlq_rx.recv().await {
            match self.send(&mut dlq_message).await {
                Ok(_) => {}
                Err(e) => error!("Error sending message to DLQ: {}", e),
            };
        }
        warn!("DLQ stopped — channel closed, no further messages.");
    }

    /// Sends an individual message to either the “dead” or “retry” topic based on its [`Retryable`] status.
    ///
    /// # Errors
    /// Returns a [`KafkaError`] if the underlying producer fails to publish the message.
    async fn send(&self, dlq_message: &mut SendToDlq) -> Result<(), KafkaError> {
        let original_kafka_msg: OwnedMessage = dlq_message.get_original_msg();

        // Create Kafka headers with error details.
        let headers = original_kafka_msg
            .generate_dlq_headers(dlq_message)
            .to_owned_headers();

        let topic = self.dlq_topic(dlq_message.retryable);
        let key = original_kafka_msg.key().unwrap_or_default();
        let payload = original_kafka_msg.payload().unwrap_or_default();

        debug!("Sending DLQ message to topic: {}", topic);

        let record = FutureRecord::to(topic)
            .payload(payload)
            .key(key)
            .headers(headers);

        let result = self.dlq_producer.send(record, None).await;
        match result {
            Ok((partition, offset)) => warn!(
                "DLQ message [{:?}] -> topic: {}, partition: {}, offset: {}",
                from_utf8(key),
                topic,
                partition,
                offset
            ),
            Err((err, _)) => return Err(err),
        };
        Ok(())
    }

    /// Returns the appropriate DLQ topic, based on whether the message is [`Retryable::Retryable`] or not.
    fn dlq_topic(&self, retryable: Retryable) -> &str {
        match retryable {
            Retryable::Retryable => &self.dlq_retry_topic,
            Retryable::NonRetryable | Retryable::Other => &self.dlq_dead_topic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::dlq::tests::MockError;
    use crate::utils::dlq::types::*;
    use rdkafka::config::ClientConfig;
    use rdkafka::message::{Header, OwnedHeaders};
    use rdkafka::mocking::MockCluster;

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

        // Retryable => "retry_topic"
        let error = MockError::MockErrorRetryable("some_error".into());
        let topic = dlq.dlq_topic(error.retryable());
        assert_eq!(topic, "retry_topic");

        // Non-retryable => "dead_topic"
        let error = MockError::MockErrorDead("some_error".into());
        let topic = dlq.dlq_topic(error.retryable());
        assert_eq!(topic, "dead_topic");
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
            value: Some(b"some_value"),
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

        assert_eq!(result.payload(), dlq_message.kafka_message.payload());
        assert_eq!(result.key(), dlq_message.kafka_message.key());
        assert_eq!(result.topic(), dlq_message.kafka_message.topic());
        assert_eq!(result.partition(), dlq_message.kafka_message.partition());
        assert_eq!(result.offset(), dlq_message.kafka_message.offset());
        assert_eq!(result.timestamp(), dlq_message.kafka_message.timestamp());
    }
}
