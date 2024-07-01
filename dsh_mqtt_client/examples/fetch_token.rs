use std::{env, sync::Arc};

use dsh_mqtt_client::{
    config::{DshConfig, DshEnv},
    model::{
        mqtt_model::{Claims, MqttToken, Resource},
        token_request_attr::RetrieveTokenRequest,
    },
    service::{AuthenticationService, DshAuthenticationServiceAdapter},
};

#[tokio::main]
async fn main() {
    let mqtt_token = get_mqtt_token().await;
    println!("mqtt -> {:?}", mqtt_token);
}

pub async fn get_mqtt_token() -> (MqttToken, String) {
    let dsh_conf = Arc::new(DshConfig::new(DshEnv::Dev));
    let client_id = uuid::Uuid::new_v4().to_string();
    let mqtt_topic_name = env::var("MQTT_TOPIC").unwrap();

    let retrieve_request = RetrieveTokenRequest {
        tenant: env::var("TENANT_NAME").unwrap().to_string(),
        api_key: env::var("API_KEY").unwrap().to_string(),
        claims: get_claims(mqtt_topic_name, "#".to_string()), // check MQTT documentation for better understanding of wildcards
        client_id: client_id.clone(),
    };
    let service: DshAuthenticationServiceAdapter = DshAuthenticationServiceAdapter::new(dsh_conf);
    let mqtt_token: MqttToken = service
        .retrieve_token(retrieve_request.clone())
        .await
        .unwrap();
    (mqtt_token, client_id)
}
pub fn get_claims(stream: String, topic: String) -> Option<Vec<Claims>> {
    let resource = Resource {
        stream,
        prefix: "/tt".to_string(),
        topic, // check MQTT documentation for better understanding of wildcards
        type_: Some("topic".to_string()),
    };

    let claims_sub = Claims {
        resource: resource.clone(),
        action: "subscribe".to_string(),
    };

    let claims_pub = Claims {
        resource: resource,
        action: "publish".to_string(),
    };

    let claims_vector = vec![claims_pub, claims_sub];
    Some(claims_vector)
}
