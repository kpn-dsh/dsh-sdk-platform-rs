use dsh_sdk::utils::dlq::{self, ErrorToDlq, DlqChannel};
use dsh_sdk::utils::graceful_shutdown::Shutdown;
use dsh_sdk::DshKafkaConfig;

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message, OwnedMessage};
use rdkafka::ClientConfig;
use std::backtrace::Backtrace;
use thiserror::Error;
use tokio::sync::mpsc;

// Define your custom error type
#[derive(Error, Debug)]
enum ConsumerError {
    #[error("Deserialization error: {0}")]
    DeserializeError(#[from] std::string::FromUtf8Error),
}

// implement the `ErrorToDlq` trait for your custom error type (or existing error types)
impl ErrorToDlq for ConsumerError {
    fn to_dlq(&self, kafka_message: OwnedMessage) -> dlq::SendToDlq {
        let backtrace = Backtrace::force_capture(); // this is optional as it is heavy on performance
        dlq::SendToDlq::new(
            kafka_message,
            self.retryable(),
            self.to_string(),
            Some(backtrace.to_string()),
        )
    }
    // Define if error is retryable or not
    fn retryable(&self) -> dlq::Retryable {
        match self {
            ConsumerError::DeserializeError(_) => dlq::Retryable::NonRetryable,
        }
    }
}

// simple deserialization function, that returns a Result of string or defined ConsumerError
fn deserialize(msg: &BorrowedMessage) -> Result<String, ConsumerError> {
    match msg.payload() {
        Some(payload) => Ok(String::from_utf8(payload.to_vec())?),
        None => Ok("".to_string()),
    }
}

// simple consumer function with shutdown function
async fn consume(
    consumer: StreamConsumer,
    topic: &str,
    mut dlq_channel: DlqChannel,
    shutdown: Shutdown,
) {
    consumer
        .subscribe(&[topic])
        .expect("Can't subscribe to topic");

    loop {
        tokio::select! {
        msg = consumer.recv() => match msg {
            Ok(msg) => {
                match deserialize(&msg) {
                    // send message to dlq if error occurs
                    Err(e) => e.to_dlq(msg.detach()).send(&mut dlq_channel).await,
                    // process message, in this case print payload
                    Ok(payload) => {
                        println!("Payload: {}", payload)
                    }
                }
            }
            Err(e) => {
                eprintln!("Error while receiving message: {}", e);
            }
        },
        _ = shutdown.signal_listener() => {
            println!("Shutting down consumer");
            break;
        }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set the dlq topics (required)
    std::env::set_var("DLQ_DEAD_TOPIC", "scratch.dlq.local-tenant");
    std::env::set_var("DLQ_RETRY_TOPIC", "scratch.dlq.local-tenant");

    // Topic to subscribe to (change to your topic)
    let topic = "your_topic_name";

    let shutdown = Shutdown::new();
    let consumer: StreamConsumer = ClientConfig::new().dsh_consumer_config().create()?;

    // Start the `dlq` service, returns a sender to send messages to the dlq
    let dlq_channel = dlq::Dlq::start(shutdown.clone())?;

    // run the `consumer` in a separate tokio task
    let shutdown_clone = shutdown.clone();
    let consumer_handle = tokio::spawn(async move {
        consume(consumer, topic, dlq_channel, shutdown_clone).await;
    });

    // wait for `consumer` or `dlq` to finish or for shutdown signal
    tokio::select! {
        _ = consumer_handle => {
            println!("Consumer finished");
        }
        _ = shutdown.signal_listener() => {
            println!("Shutting down");
        }
    }

    // wait for graceful shutdown to complete
    shutdown.complete().await;
    Ok(())
}
