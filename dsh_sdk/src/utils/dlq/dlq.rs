//! Dead Letter Queue client

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

/// The dead letter queue
///
/// # How to use
/// 1. Implement the [`ErrorToDlq`](super::ErrorToDlq) trait on top your (custom) error type.
/// 2. Use the [`Dlq::start`] in your main or at start of your process logic. (this will start the DLQ in a separate tokio task)
/// 3. Get the dlq [`DlqChannel`] from the [`Dlq::start`] method and use this channel to communicate errored messages with the [`Dlq`] via the [`ErrorToDlq::to_dlq`](super::ErrorToDlq::to_dlq) method.
///
/// # Importance of `DlqChannel` in the graceful shutdown procedure
/// The [`Dlq::start`] will return a [`DlqChannel`]. The [`Dlq`] will keep running till the moment [`DlqChannel`] is dropped and finished processing all messages. 
/// This also means that the [`Shutdown`] procedure will wait for the [`Dlq`] to finish processing all messages before the application is shut down. 
/// This is to make sure that **all** messages are properly processed before the application is shut down.
/// 
/// **NEVER** borrow the [`DlqChannel`] but provide the channel as owned/cloned version to your processing logic and **NEVER** keep an owned version in main function, as this will result in a **deadlock**  and your application will never shut down. 
/// It is fine to start the [`Dlq`] in the main function, but make sure the [`DlqChannel`] is moved to your processing logic.
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
    /// * `shutdown` - The [`Shutdown`] is required to keep the DLQ alive until the [`DlqChannel`] is dropped
    ///
    /// # Returns
    /// * The [DlqChannel] to send messages to the DLQ
    ///
    /// # Importance of `DlqChannel` in the graceful shutdown procedure
    /// The [`Dlq::start`] will return a [`DlqChannel`]. The [`Dlq`] will keep running till the moment [`DlqChannel`] is dropped and finished processing all messages. 
    /// This also means that the [`Shutdown`] procedure will wait for the [`Dlq`] to finish processing all messages before the application is shut down. 
    /// This is to make sure that **all** messages are properly processed before the application is shut down.
    /// 
    /// **NEVER** borrow the [`DlqChannel`] but provide the channel as owned/cloned version to your processing logic and **NEVER** keep an owned version in main function, as this will result in a **deadlock**  and your application will never shut down. 
    /// It is fine to start the [`Dlq`] in the main function, but make sure the [`DlqChannel`] is moved to your processing logic.
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
    pub fn start(shutdown: Shutdown) -> Result<DlqChannel, DlqErrror> {
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
        let error = MockError::MockErrorRetryable("some_error".to_string());
        let topic = dlq.dlq_topic(error.retryable());
        assert_eq!(topic, "retry_topic");
        let error = MockError::MockErrorDead("some_error".to_string());
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
}
