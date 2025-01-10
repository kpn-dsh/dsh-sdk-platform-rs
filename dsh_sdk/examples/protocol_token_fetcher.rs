use std::env;

use dsh_sdk::protocol_adapters::token_fetcher::*;

#[tokio::main]
async fn main() {
    let tenant_name = env::var("TENANT").unwrap().to_string();
    let api_key = env::var("API_KEY").unwrap().to_string();
    let mqtt_token_fetcher =
        ProtocolTokenFetcher::new(tenant_name, api_key, dsh_sdk::Platform::NpLz);
    let token: ProtocolToken = mqtt_token_fetcher
        .get_token("Client-id", None) //Claims = None fetches all possible claims
        .await
        .unwrap();
    println!("MQTT Token: {:?}", token);
}
