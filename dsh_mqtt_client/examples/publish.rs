use std::env;

use dsh_mqtt_client::{
    error::DshError,
    stream_client::{BaseMqttOptions, MqttClient, MqttCredentials, MqttEnv},
};
use fetch_token::get_mqtt_token;
use rumqttc::{AsyncClient, Event, Incoming, QoS};

mod fetch_token;

#[tokio::main]
async fn main() {
    let mqtt_env = MqttEnv::Dev;
    let mqtt_options = BaseMqttOptions::new(mqtt_env).await;

    let (mqtt_token, client_id) = get_mqtt_token().await;

    let mqtt_credentials: MqttCredentials = MqttCredentials {
        mqtt_token: mqtt_token.raw_token,
        client_id,
    };
    let mut mqtt_client = MqttClient::new(mqtt_options, mqtt_credentials).await;

    let mqtt_topic_name = env::var("MQTT_TOPIC").unwrap();

    let res = publish_to_topic(&mut mqtt_client, &mqtt_topic_name, "hello mqtt!")
        .await
        .expect("Publish - Connection Error");

    println!("Result: {:?}", res);
}
pub async fn publish_to_topic(
    mqtt_client: &mut MqttClient,
    topic_name: &str,
    message: &str,
) -> Result<String, DshError> {
    let topic_prefix = String::from("/tt/");
    publish_message(
        &mqtt_client.async_client,
        &(topic_prefix + topic_name),
        message,
    )
    .await?;

    loop {
        match mqtt_client.eventloop.poll().await {
            Ok(Event::Incoming(Incoming::PubAck(_))) => {
                return Ok("Message published".to_string());
            }
            Ok(_) => {}
            Err(e) => {
                return Err(DshError::PublishError(e.to_string()));
            }
        }
    }
}

async fn publish_message(client: &AsyncClient, topic: &str, message: &str) -> Result<(), DshError> {
    let sanitized_topic = topic.replace(['#', '+'], "");
    println!("Publishing message to topic: {}", sanitized_topic);

    client
        .publish(sanitized_topic, QoS::AtLeastOnce, true, message)
        .await
        .map_err(DshError::ClientError)?;

    Ok(())
}
