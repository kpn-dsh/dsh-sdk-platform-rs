use dsh_sdk::dsh::Properties;
use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};
use dsh_sdk::rdkafka::producer::{FutureProducer, FutureRecord};
use dsh_sdk::rdkafka::Message;

const TOTAL_MESSAGES: usize = 10;

async fn produce(producer: &mut FutureProducer, topic: &str) {
    for key in 0..TOTAL_MESSAGES {
        let payload = format!("hello world {}", key);
        let msg = producer
            .send(
                FutureRecord::to(topic)
                    .payload(payload.as_bytes())
                    .key(&key.to_be_bytes()),
                std::time::Duration::from_secs(0),
            )
            .await;
        match msg {
            Ok(_) => println!("Message {} sent", key),
            Err(e) => println!("Error sending message: {}", e.0),
        }
    }
}

async fn consume(consumer: &mut StreamConsumer, topic: &str) {
    consumer.subscribe(&[topic]).unwrap();
    let mut i = 0;
    while i < TOTAL_MESSAGES {
        let msg = consumer.recv().await.unwrap();
        let payload = String::from_utf8_lossy(msg.payload().unwrap());
        let key = usize::from_be_bytes(msg.key().unwrap().try_into().unwrap());
        println!("Received message: key: {}, payload: {}", key, payload);
        i += 1;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new DSH Properties instance (requires local_datastreams.json in root of project, as it runs in local mode)
    let dsh_properties = Properties::get()?;

    // Define your topic
    let topic = "scratch.local.local-tenant";

    // Create a new producer based on the properties default config
    let mut producer: FutureProducer = dsh_properties.producer_rdkafka_config()?.create()?;

    // Produce messages towards topic
    produce(&mut producer, topic).await;

    // Create a new consumer based on the properties default config
    let mut consumer: StreamConsumer = dsh_properties.consumer_rdkafka_config()?.create()?;

    consume(&mut consumer, topic).await;
    Ok(())
}
