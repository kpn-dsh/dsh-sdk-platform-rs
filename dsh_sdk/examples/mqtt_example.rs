//! This example demonstrates how to connect to the DSH MQTT broker and consume data using the DSH SDK and Rumqttc.
//!
//! Example is using the following crates:
//! - [`dsh_sdk`] with features = ["protocol-token"] for using tokens to authenticate to the DSH MQTT broker
//! - [`rumqttc`] for mqtt client
//! - [`tokio`] with features = ["full"] for async runtime
//! - [`env_logger`] for output logging to stdout to show what is happening
//!
//! NEVER distribute the API_KEY to an external client, this is only for demonstration purposes
//!
//! Run example with:
//! ```bash
//! API_KEY={your_api_key} TENANT={your_tenant} CLIENT_ID=sdk_example_client cargo run  --features protocol-token --example mqtt_example
//! ```
//!
//! The example will:
//! - Request a DataAccessToken
//! - Create a new MqttOptions based on the fetched token
//! - Create a new async client
//! - Subscribe to a topic
//! - Print received messages

use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
use dsh_sdk::protocol_adapters::token::data_access_token::{
    DataAccessToken, RequestDataAccessToken,
};

use rumqttc::{AsyncClient, MqttOptions, Transport};

/// The platform to fetch the token for.
const PLATFORM: dsh_sdk::Platform = dsh_sdk::Platform::Poc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tenant_name = std::env::var("TENANT").expect("TENANT is not set");
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID is not set");

    // Start logger to Stdout to show what is happening
    env_logger::builder()
        .filter(Some("dsh_sdk"), log::LevelFilter::Trace)
        .target(env_logger::Target::Stdout)
        .init();

    // Create a request for the Data Access Token (this request for full access)
    let request = RequestDataAccessToken::new(tenant_name, client_id);

    // Fetch token from your API Client Authentication service
    let token = ApiClientAuthenticationService::get_data_access_token(request).await;

    // Create a new MqttOptions based on info from  token
    let mut mqttoptions = MqttOptions::new(token.client_id(), token.endpoint(), token.port_mqtt());
    mqttoptions.set_credentials("", token.raw_token());
    mqttoptions.set_transport(Transport::tls_with_default_config());

    // Create a new async client
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // For demonstration purposes, we select the first topic that is available in token
    let topic = token
        .claims()
        .iter()
        .next()
        .expect("No avaialable topics")
        .full_qualified_topic_name();

    // Subscribe to a topic
    client.subscribe(topic, rumqttc::QoS::AtMostOnce).await?;

    loop {
        match eventloop.poll().await {
            Ok(v) => {
                println!("Received = {:?}", v);
            }
            Err(e) => {
                println!("Error = {e:?}");
                break;
            }
        }
    }

    Ok(())
}

/// NEVER use this in an external client, this is only for demonstration purposes.
/// This should be implemented in your API Client Authentication service.
///
/// See the following examples for a more complete example:
/// - `examples/protocol_authentication_partial_mediation.rs`
/// - `examples/protocol_authentication_full_mediation.rs`
struct ApiClientAuthenticationService;

impl ApiClientAuthenticationService {
    /// This should be properly implemented in your API Client Authentication service.
    async fn get_data_access_token(request: RequestDataAccessToken) -> DataAccessToken {
        let api_key = std::env::var("API_KEY").expect("API_KEY is not set");

        let token_fetcher = ApiClientTokenFetcher::new(api_key, PLATFORM);

        token_fetcher
            .fetch_data_access_token(request)
            .await
            .unwrap()
    }
}
