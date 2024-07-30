// make sure to use the dlq feature in your Cargo.toml
// dsh_sdk = { version = "0.4", features = ["dlq"] }
//
// To run this example, run the following command:
// cargo run --features dlq --example dlq_implementation

use dsh_sdk::Properties;
use dsh_sdk::dlq::{self, ErrorToDlq};
use dsh_sdk::graceful_shutdown::Shutdown;
use dsh_sdk::rdkafka::consumer::{StreamConsumer, Consumer};
use dsh_sdk::rdkafka::Message;
use std::backtrace::Backtrace;
use thiserror::Error;
use tokio::sync::mpsc;

// Define your custom error type
#[derive(Error, Debug)]
enum ConsumerError {
    #[error("Deserialization error: {0}")]
    DeserializeError(#[from] std::string::FromUtf8Error),
}

// implement the ErrorToDlq trait for your custom error type (or exusting error types)
impl dlq::ErrorToDlq for ConsumerError {
    fn to_dlq(&self, kafka_message: rdkafka::message::OwnedMessage) -> dlq::SendToDlq {
        let backtrace = Backtrace::force_capture(); // this is optional as it is heavy on performance
        dlq::SendToDlq::new(
            kafka_message,
            self.retryable(),
            self.to_string(),
            Some(backtrace.to_string()),
        )
    }
    // Definition if error is retryable or not
    fn retryable(&self) -> dlq::Retryable {
        match self {
            ConsumerError::DeserializeError(_) => dlq::Retryable::NonRetryable,
        }
    }
}

// simple deserialization function, that returns a Result of string or defined ConsumerError
fn deserialize(msg: &dsh_sdk::rdkafka::message::OwnedMessage) -> Result<String, ConsumerError> {
    match msg.payload() {
        Some(payload) => Ok(String::from_utf8(payload.to_vec())?),
        None => Ok("".to_string()),
    }
}

// simple consumer function
async fn consume(consumer: StreamConsumer, dlq_tx: &mut mpsc::Sender<dlq::SendToDlq>) {
    consumer.subscribe(&["sub_to_your_topic"]).expect("Can't subscribe to topic");
    while let Ok(msg) = consumer.recv().await {
        let owned_msg = msg.detach();
        match deserialize(&owned_msg) {
            // send message to dlq if error occurs
            Err(e) => e.to_dlq(owned_msg).send(dlq_tx).await,
            // process message, in this case print payload
            Ok(payload) => {
                println!("Payload: {}", payload)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set the dlq topics
    std::env::set_var("DLQ_DEAD_TOPIC", "scratch.dlq.local-tenant");
    std::env::set_var("DLQ_RETRY_TOPIC", "scratch.dlq.local-tenant");
    let dsh = Properties::get();
    let shutdown = Shutdown::new();
    let consumer: StreamConsumer = dsh.consumer_rdkafka_config().create()?;

    let mut dlq = dlq::Dlq::new(dsh, shutdown.clone())?;
    // get the dlq channel sender to send messages to the dlq
    // for example in your consumer
    let mut dlq_tx = dlq.dlq_records_tx();
    let consumer_handle = tokio::spawn(async move {
        consume(consumer, &mut dlq_tx).await;
    });
    // run the dlq in a separate tokio task
    let dlq_handle = tokio::spawn(async move {
        dlq.run().await;
    });
    tokio::select! {
        _ = consumer_handle => {
            println!("Consumer finished");
        }
        _ = dlq_handle => {
            println!("DLQ finished");
        }
        _ = shutdown.signal_listener() => {
            println!("Shutting down");
        }
    }
    shutdown.complete().await; // wait for graceful shutdown to complete
    Ok(())
}

