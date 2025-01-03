use dsh_sdk::DshKafkaConfig;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use rdkafka::Message;

const TOTAL_MESSAGES: usize = 10;

async fn produce(producer: FutureProducer, topic: &str) {
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

async fn consume(consumer: StreamConsumer, topic: &str) {
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
    // Define your topic
    let topic = "test";

    // Create a new producer from the RDkafka Client Config together with dsh_prodcer_config form DshKafkaConfig trait
    let producer: FutureProducer = ClientConfig::new().dsh_producer_config().create()?;

    // Produce messages towards topic
    produce(producer, topic).await;

    // Create a new consumer from the RDkafka Client Config together with dsh_consumer_config form DshKafkaConfig trait
    let consumer: StreamConsumer = ClientConfig::new().dsh_consumer_config().create()?;

    consume(consumer, topic).await;
    Ok(())
}
