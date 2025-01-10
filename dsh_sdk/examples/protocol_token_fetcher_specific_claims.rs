use std::env;

use dsh_sdk::protocol_adapters::token_fetcher::*;

#[tokio::main]
async fn main() {
    // Get the config and secret from the environment
    let tenant_name = env::var("TENANT").unwrap().to_string();
    let api_key = env::var("API_KEY").unwrap().to_string();
    let stream = env::var("STREAM").unwrap().to_string();

    let topic = "#".to_string(); // check MQTT documentation for better understanding of wildcards
    let resource = Resource::new(stream, "/tt".to_string(), topic, Some("topic".to_string()));

    let claims_sub = Claims::new(resource.clone(), Actions::Subscribe);

    let claims_pub = Claims::new(resource, Actions::Publish);

    let claims = vec![claims_sub, claims_pub];

    let mqtt_token_fetcher =
        ProtocolTokenFetcher::new(tenant_name, api_key, dsh_sdk::Platform::NpLz);

    let token: ProtocolToken = mqtt_token_fetcher
        .get_token("Client-id", Some(claims))
        .await
        .unwrap();
    println!("MQTT Token: {:?}", token);
}
