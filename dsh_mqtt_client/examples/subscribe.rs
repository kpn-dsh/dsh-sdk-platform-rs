use std::env;

use dsh_mqtt_client::{
    error::DshError,
    stream_client::{BaseMqttOptions, MqttClient, MqttCredentials, MqttEnv},
};
use fetch_token::get_mqtt_token;
use rumqttc::{Event, Incoming, QoS};

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

    let mqtt_topic_name = env::var("MQTT_STREAM").unwrap();

    let message = subscribe_to_topic(&mut mqtt_client, &mqtt_topic_name)
        .await
        .expect("Subscribe - Connection error");

    println!("Recieved message: {:?}", message);
}
pub async fn subscribe_to_topic(
    mqtt_client: &mut MqttClient,
    topic_name: &str,
) -> Result<String, DshError> {
    let topic_prefix = String::from("/tt/");
    mqtt_client
        .async_client
        .subscribe(topic_prefix + topic_name, QoS::AtLeastOnce)
        .await
        .map_err(|e| DshError::SubscribeError(e.to_string()))?;

    loop {
        match mqtt_client.eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                let message = String::from_utf8(publish.payload.to_vec())
                    .map_err(|e| DshError::Utf8Error(e.to_string()))?;
                mqtt_client
                    .async_client
                    .disconnect()
                    .await
                    .map_err(|e| DshError::FailedToDisconnect(e.to_string()))?;
                return Ok(message);
            }
            Ok(_) => {}
            Err(e) => return Err(DshError::StreamConnectionError(e.to_string())),
        }
    }
}
