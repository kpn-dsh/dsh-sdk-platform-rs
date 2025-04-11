//! This example demonstrates how to use the dsh_sdk crate to produce and consume messages
//! to and from a Kafka topic using the rdkafka library.
//!
//! Example is using the following crates:
//! - [`dsh_sdk`] with features = ["rdkafka-config"] for DSH Kafka consumer config
//! - [`rdkafka`] with features = ["cmake-build", "ssl-vendored"] for kafka cosumer
//! - [`tokio`] with features = ["full"] for async runtime
//! - [`env_logger`] for output logging to stdout to show what is happening
//!
//! Run the example against a local kafka broker on localhost:9092
//! ```bash
//! cargo run --example kafka_example
//! ```
//!
//! To run this example against Kafka on DSH from your local environment,
//! check 'dsh_sdk/CONNECT_PROXY_VPN_LOCAL.md' for instructions on how to set up the connection.

use dsh_sdk::DshKafkaConfig;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use rdkafka::Message;

const TOTAL_MESSAGES: usize = 10;

async fn produce(producer: FutureProducer, topic: &str) {
    println!("Producing messages to topic: {}", topic);
    for key in 0..TOTAL_MESSAGES {
        let payload = format!("hello world {}", key);
        let msg = producer
            .send(
                FutureRecord::to(topic)
                    .payload(payload.as_bytes())
                    .key(&key.to_be_bytes()),
                std::time::Duration::from_secs(1),
            )
            .await;
        match msg {
            Ok(_) => println!("Message {} sent to {}", key, topic),
            Err(e) => println!("Error sending message: {}", e.0),
        }
    }
}

async fn consume(consumer: StreamConsumer, topic: &str) {
    println!("Consuming messages from topic: {}", topic);
    consumer.subscribe(&[topic]).unwrap();
    let mut i = 0;
    while i < TOTAL_MESSAGES {
        let msg = consumer.recv().await.unwrap();
        let payload = String::from_utf8_lossy(msg.payload().unwrap());
        let key = usize::from_be_bytes(msg.key().unwrap().try_into().unwrap());
        println!("Received message: key: {}, payload: {}", key, payload);
        consumer.commit_message(&msg, CommitMode::Async).unwrap();
        i += 1;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start logger to Stdout to show what is happening
    env_logger::builder()
        .filter(Some("dsh_sdk"), log::LevelFilter::Debug)
        .target(env_logger::Target::Stdout)
        .init();

    // Ask on stin for topic name
    println!("Enter topic to write and read from:");
    let mut topic = String::new();
    std::io::stdin()
        .read_line(&mut topic)
        .expect("Failed to read line");
    let topic = topic.trim();

    // Create a new producer from the RDkafka Client Config together with dsh_prodcer_config form DshKafkaConfig trait
    let producer: FutureProducer = ClientConfig::new().set_dsh_producer_config().create()?;

    // Produce messages towards topic
    produce(producer, topic).await;

    // Create a new consumer from the RDkafka Client Config together with dsh_consumer_config form DshKafkaConfig trait
    let consumer: StreamConsumer = ClientConfig::new().set_dsh_consumer_config().create()?;

    consume(consumer, topic).await;
    Ok(())
}
