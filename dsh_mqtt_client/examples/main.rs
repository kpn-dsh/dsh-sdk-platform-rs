use std::{env, sync::Arc};

use dsh_mqtt_client::{
    config::{DshConfig, DshEnv},
    error::DshError,
    model::{
        mqtt_model::{Claims, MqttToken, Resource},
        token_request_attr::RetrieveTokenRequest,
    },
    service::{AuthenticationService, DshAuthenticationServiceAdapter},
    stream_client::{BaseMqttOptions, MqttClient, MqttCredentials, MqttEnv},
};
use log::{error, info};
use rumqttc::{AsyncClient, Event, Incoming, QoS};

#[tokio::main]
async fn main() {
    //FETCH TOKEN
    let dsh_conf = Arc::new(DshConfig::new(DshEnv::Dev));

    let retrieve_request = RetrieveTokenRequest {
        tenant: env::var("TENANT_NAME").unwrap().to_string(),
        api_key: env::var("API_KEY").unwrap().to_string(),
        claims: None,
        client_id: uuid::Uuid::new_v4().to_string(),
    };
    let service: DshAuthenticationServiceAdapter = DshAuthenticationServiceAdapter::new(dsh_conf);
    let mqtt_token: MqttToken = service
        .retrieve_token(retrieve_request.clone())
        .await
        .unwrap();

    print!("mqtt -> {:?}", mqtt_token);

    //STREAM
    let mqtt_env = MqttEnv::Dev;
    let mqtt_options = BaseMqttOptions::new(mqtt_env).await;

    let mqtt_credentials = MqttCredentials {
        mqtt_token: mqtt_token.raw_token,
        client_id: "018dc0fe-b37c-78a7-a84e-bd105e411d89".to_string(),
    };
    let mut mqtt_client = MqttClient::new(mqtt_options, mqtt_credentials).await;

    let message = subscribe_to_topic(&mut mqtt_client, "/tt/training-mobile-presence/")
        .await
        .expect("Subscribe - Connection error");
    print!("Message: {:?}", message);

    // let res = publish_to_topic(
    //     &mut mqtt_client,
    //     "/tt/training-mobile-presence/",
    //     "1 - mobile presence",
    // )
    // .await
    // .expect("Publish - Connection Error");
    // println!("Result: {:?}", res);
}

pub fn get_claims() -> Option<Vec<Claims>> {
    let resource = Resource {
        stream: "weather".to_string(),
        prefix: "/tt".to_string(),
        topic: "+/+/+/+/+/+/+/+/+/+/+/#".to_string(),
        type_: Some("topic".to_string()),
    };

    let claims = Claims {
        resource: resource,
        action: "subscribe".to_string(),
    };

    let claims_vector = vec![claims];
    Some(claims_vector)
}
pub async fn subscribe_to_topic(
    mqtt_client: &mut MqttClient,
    topic_name: &str,
) -> Result<String, DshError> {
    mqtt_client
        .async_client
        .subscribe(topic_name, QoS::AtLeastOnce)
        .await
        .map_err(|e| DshError::SubscribeError(e.to_string()))?;

    loop {
        match mqtt_client.eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                info!("Received message: {:?}", publish.payload);
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

pub async fn publish_to_topic(
    mqtt_client: &mut MqttClient,
    topic_name: &str,
    message: &str,
) -> Result<String, DshError> {
    publish_message(&mqtt_client.async_client, topic_name, message).await?;

    loop {
        match mqtt_client.eventloop.poll().await {
            Ok(Event::Incoming(Incoming::PubAck(_))) => {
                info!("Message published");
                return Ok("Message published".to_string());
            }
            Ok(_) => {}
            Err(e) => {
                error!("Error while polling for publish acknowledgement: {:?}", e);
                return Err(DshError::PublishError(e.to_string()));
            }
        }
    }
}

async fn publish_message(client: &AsyncClient, topic: &str, message: &str) -> Result<(), DshError> {
    let sanitized_topic = topic.replace(['#', '+'], "");
    info!("Publishing message to topic: {}", sanitized_topic);

    client
        .publish(sanitized_topic, QoS::AtLeastOnce, true, message)
        .await
        .map_err(DshError::ClientError)?;

    Ok(())
}
