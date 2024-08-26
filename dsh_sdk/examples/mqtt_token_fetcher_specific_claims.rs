use std::env;

use dsh_sdk::mqtt_token_fetcher::{Actions, Claims, MqttToken, MqttTokenFetcher, Resource};

#[tokio::main]
async fn main() {
    let tenant_name = env::var("TENANT").unwrap().to_string();
    let api_key = env::var("API_KEY").unwrap().to_string();
    let stream = env::var("STREAM").unwrap().to_string();
    let topic = "#".to_string(); // check MQTT documentation for better understanding of wildcards
    let resource = Resource::new(stream, "/tt".to_string(), topic, Some("topic".to_string()));

    let claims_sub = Claims::new(resource.clone(), Actions::Subscribe.to_string());

    let claims_pub = Claims::new(resource, Actions::Publish.to_string());

    let claims_vector = vec![claims_sub, claims_pub];

    let mqtt_token_fetcher: MqttTokenFetcher =
        MqttTokenFetcher::new(tenant_name, api_key, dsh_sdk::Platform::NpLz).unwrap();

    let token: MqttToken = mqtt_token_fetcher
        .get_token("Client-id", Some(claims_vector))
        .await
        .unwrap();
    println!("MQTT Token: {:?}", token);
}
