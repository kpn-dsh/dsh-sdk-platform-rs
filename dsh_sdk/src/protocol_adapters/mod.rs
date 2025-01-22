//! The DSH Protocol adapters (HTTP, Kafka, MQTT)

//#[cfg(feature = "http-protocol-adapter")]
//pub mod http_protocol;
#[cfg(feature = "kafka")]
pub mod kafka_protocol;
// #[cfg(feature = "mqtt-protocol-adapter")]
// pub mod mqtt_protocol;
#[cfg(feature = "protocol-token")]
pub mod token;
