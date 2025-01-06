use std::env;

use dsh_sdk::DshKafkaConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;

// Enter your details here
const KAFKA_BOOTSTRAP_SERVERS: &str = "kafkaproxy urls"; // example "broker-0.kafka.tenant.kpn-dsh.com:9091,broker-1.kafka.tenant.kpn-dsh.com:9091,broker-2.kafka.tenant.kpn-dsh.com:9091"
const PKI_CONFIG_DIR: &str = "path/to/pki/config/dir"; // example /Documents/pki_config_dir/tenant
const DSH_TENANT_NAME: &str = "tenant";  // enter your tenant name (required for creating group id)
const TOPIC: &str = "scratch.topic-name.tenant"; // enter your topic name

/// Simple consumer that consumes messages from a Kafka topic
async fn consume(consumer: StreamConsumer) {
    consumer.subscribe(&[TOPIC]).unwrap();
    loop {
        let msg = consumer.recv().await.unwrap();
        let payload = String::from_utf8_lossy(msg.payload().unwrap());
        println!("Received message: key: {:?}, payload: {}", msg.key(), payload);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set the environment variables (normally you would set them outside of the code)
    env::set_var("KAFKA_BOOTSTRAP_SERVERS", KAFKA_BOOTSTRAP_SERVERS);
    env::set_var("PKI_CONFIG_DIR", PKI_CONFIG_DIR);
    env::set_var("DSH_TENANT_NAME", DSH_TENANT_NAME);

    // Create a new consumer from the RDkafka Client Config together with dsh_consumer_config form DshKafkaConfig trait
    // The config will take over the info from the environment variables and load certificates from the PKI_CONFIG_DIR
    // This makes it easy to switch from Kafka Proxy to Normal usage without changing the code
    let consumer: StreamConsumer = ClientConfig::new().set_dsh_consumer_config().create()?;

    // start consuming messages from the topic
    consume(consumer).await;
    Ok(())
}