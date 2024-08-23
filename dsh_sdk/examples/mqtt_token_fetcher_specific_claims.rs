use std::env;

use dsh_sdk::mqtt_token_fetcher::{Actions, Claims, MqttToken, MqttTokenFetcher, Resource};

#[tokio::main]
async fn main() {
    let tenant_name = env::var("TENANT").unwrap().to_string();
    let api_key = env::var("API_KEY").unwrap().to_string();
    let stream = env::var("STREAM").unwrap().to_string();
    let topic = "#".to_string();
    let resource = Resource {
        stream,
        prefix: "/tt".to_string(),
        topic, // check MQTT documentation for better understanding of wildcards
        type_: Some("topic".to_string()),
    };

    let claims_sub = Claims {
        resource: resource.clone(),
        action: Actions::Subscribe.to_string(),
    };

    let claims_pub = Claims {
        resource: resource,
        action: Actions::Publish.to_string(),
    };

    let claims_vector = vec![claims_pub, claims_sub];

    let mqtt_token_fetcher: MqttTokenFetcher = MqttTokenFetcher::new(
        tenant_name,
        api_key,
        Some(claims_vector),
        dsh_sdk::Platform::NpLz,
    )
    .await
    .unwrap();

    let token: MqttToken = mqtt_token_fetcher
        .get_token("Client-id", None)
        .await
        .unwrap();
    println!("MQTT Token: {:?}", token);
}
