use dsh_sdk::graceful_shutdown::Shutdown;
use dsh_sdk::rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use dsh_sdk::rdkafka::message::{BorrowedMessage, Message};
use dsh_sdk::Properties;

use log::{error, info};

mod custom_metrics;

fn deserialize_and_print(msg: &BorrowedMessage) {
    let payload = String::from_utf8_lossy(msg.payload().unwrap_or(b""));
    let key = String::from_utf8_lossy(msg.key().unwrap_or(b""));

    info!(
        "Received message from topic {} partition {} offset {} with key {:?} and payload {}",
        msg.topic(),
        msg.partition(),
        msg.offset(),
        key,
        payload
    );
}

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
    dsh_sdk::metrics::start_http_server(8080);

    // Create a new properties instance (connects to the DSH server and fetches the datastream)
    let dsh_properties = Properties::get();

    // Get the configured topics from env variable TOPICS (comma separated)
    let topics_string = std::env::var("TOPICS").expect("TOPICS env variable not set");
    let topics = topics_string.split(",").map(|s| s).collect::<Vec<&str>>();

    // Validate your configured topic if it has read access (optional)
    dsh_properties
        .datastream()
        .verify_list_of_topics(&topics, dsh_sdk::dsh::datastream::ReadWriteAccess::Read)?;

    // Initialize the shutdown handler (This will handle SIGTERM and SIGINT signals, and you can act on them)
    let shutdown = Shutdown::new();

    // Get the consumer config from the Properties instance
    let mut consumer_client_config = dsh_properties.consumer_rdkafka_config();

    // Override some default values (optional)
    consumer_client_config.set("auto.offset.reset", "latest");

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

    // Wait till the shutdown is complete
    shutdown.complete().await;
    Ok(())
}
