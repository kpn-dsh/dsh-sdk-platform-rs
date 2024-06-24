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
    let dsh_conf = Arc::new(DshConfig::new(DshEnv::Dev));

    let retrieve_request = RetrieveTokenRequest {
        tenant: env::var("TENANT_NAME").unwrap().to_string(),
        api_key: env::var("API_KEY").unwrap().to_string(),
        claims: get_claims(),
        client_id: uuid::Uuid::new_v4().to_string(),
    };
    let service: DshAuthenticationServiceAdapter = DshAuthenticationServiceAdapter::new(dsh_conf);
    let mqtt_token: MqttToken = service
        .retrieve_token(retrieve_request.clone())
        .await
        .unwrap();

    print!("mqtt -> {:?}", mqtt_token);
}

fn get_claims() -> Option<Vec<Claims>> {
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
