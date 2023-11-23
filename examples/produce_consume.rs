use dsh_sdk::bootstrap::{Bootstrap, GroupType};
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use rdkafka::Message;

const TOTAL_MESSAGES: usize = 10;

async fn produce(producer: &mut FutureProducer, topic: &str) {
    for key in 0..TOTAL_MESSAGES {
        let payload = b"hello world";
        let msg = producer
            .send(
                rdkafka::producer::FutureRecord::to(topic)
                    .payload(payload)
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
async fn main() {
    // Create a new bootstrap instance (requires local_datastreams.json in root of project, as it runs in local mode)
    let bootstrap = Bootstrap::new().await.unwrap();

    // Define your topic
    let topic = "scratch.local.local-tenant";

    // Create a new producer based on the bootstrap default config
    let mut producer: FutureProducer = bootstrap.producer_rdkafka_config().create().unwrap();

    // Produce messages towards topic
    produce(&mut producer, topic).await;

    // Create a new consumer based on the bootstrap default config
    let mut consumer: StreamConsumer = bootstrap
        .consumer_rdkafka_config()
        // make optional adjustments to default config
        // e.g. get specific group id from the kafka properties
        .set(
            "group.id",
            bootstrap
                .kafka_properties()
                .get_group_id(GroupType::Shared(1))
                .unwrap(),
        )
        .create()
        .unwrap();

    consume(&mut consumer, topic).await;
}
