use dsh_sdk::utils::graceful_shutdown::Shutdown;
use dsh_sdk::DshKafkaConfig;

use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::ClientConfig;

use log::{error, info};

mod custom_metrics;

/// Deserialize and print the message
fn deserialize_and_print(msg: &BorrowedMessage) {
    let payload = String::from_utf8_lossy(msg.payload().unwrap_or(b""));
    let key = String::from_utf8_lossy(msg.key().unwrap_or(b""));

    println!(
        "Received message from topic: {}, partition: {}, offset: {}, key: {}, and payload:\n{}",
        msg.topic(),
        msg.partition(),
        msg.offset(),
        key,
        payload
    );
}

/// Simple consumer that consumes messages from Kafka and prints them
async fn consume(consumer: StreamConsumer, shutdown: Shutdown) {
    loop {
        tokio::select! {
            Ok(msg) = consumer.recv() => {
                    // Increment the counter that is defined in src/metrics.rs
                    custom_metrics::CONSUMED_MESSAGES.inc();
                    // Deserialize and print the message
                    deserialize_and_print(&msg);
                    // Commit the message
                    if let Err(e) = consumer.commit_message(&msg, CommitMode::Sync) {
                        error!("Error while committing message: {:?}", e);
                    }
            },
            _ = shutdown.recv() => {
                info!("Shutdown requested, breaking out of consumer");
                consumer.unsubscribe();
                break;
             }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start logger to Stdout
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    // Start http server for exposing prometheus metrics, note that in Dockerfile we expose port 8080 as well
    dsh_sdk::utils::metrics::start_http_server(8080);

    // Get the configured topics from env variable TOPICS (comma separated)
    let topics_string = std::env::var("TOPICS").expect("TOPICS env variable not set");
    let topics = topics_string.split(',').collect::<Vec<&str>>();

    // Initialize the shutdown handler (This will handle SIGTERM and SIGINT signals, and you can act on them)
    let shutdown = Shutdown::new();

    // Create RDKafka Client config
    let mut consumer_client_config = ClientConfig::new();

    // Load the Kafka configuration from the SDK (this method comes from the `DshKafkaConfig` trait)
    consumer_client_config.set_dsh_consumer_config();


    // Create a new consumer instance
    let consumer: StreamConsumer = consumer_client_config.create()?;

    // Subscribe to the configured topics
    consumer.subscribe(&topics)?;

    // Create handle for consuming messages,
    let shutdown_clone = shutdown.clone();
    let consumer_handle = tokio::spawn(async move {
        consume(consumer, shutdown_clone).await;
    });

    // Wait for shutdown signal or that the consumer has stopped
    tokio::select! {
        _ = shutdown.signal_listener() => {
            info!("Shutdown signal received");
        }
        _ = consumer_handle => {
            info!("Consumer stopped");
            shutdown.start(); // Start the shutdown process (this will stop other potential running tasks that implemented the shutdown listener)
        }
    }

    // Wait till the graceful shutdown is finished
    shutdown.complete().await;
    Ok(())
}
