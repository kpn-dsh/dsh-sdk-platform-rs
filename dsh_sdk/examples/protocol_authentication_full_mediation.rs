//! Example: API Client Authentication service fetching a DataAccessToken for a device.
//!
//! The DataAccessToken allows a device to connect to protocol adapters with specific permissions.
//!
//! ## Important Notes:
//! - **Do NOT implement this logic in device applications or external clients!**
//! - This logic is exclusive to the **API Client role** in the DSH architecture.
//! - The API Client uses a long-lived API_KEY to fetch short-lived tokens for devices.
//! - **The API_KEY must never be distributed.**

use dsh_sdk::protocol_adapters::token::{
    api_client_token_fetcher::ApiClientTokenFetcher, Action, RequestDataAccessToken,
    TopicPermission,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// Target platform for fetching the token.
const PLATFORM: dsh_sdk::Platform = dsh_sdk::Platform::NpLz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve environment variables
    let tenant_name = std::env::var("TENANT").expect("TENANT environment variable is not set");
    let api_key = std::env::var("API_KEY").expect("API_KEY environment variable is not set");

    // Initialize logger to display detailed SDK activity in stdout
    env_logger::builder()
        .filter(Some("dsh_sdk"), log::LevelFilter::Trace)
        .target(env_logger::Target::Stdout)
        .init();

    // Example Scenario:
    // Assume the API Authentication service receives a request from an external client.
    // We want to delegate a DataAccessToken with the following properties:
    // - Valid for 10 minutes
    // - Allows subscribing to the topic "state/app/{tenant_name}" in the "amp" stream

    // Instantiate the API Client Token Fetcher
    let token_fetcher = ApiClientTokenFetcher::new(api_key, PLATFORM);

    // Define the permissions for the DataAccessToken
    let permissions = vec![TopicPermission::new(
        Action::Subscribe,
        "amp",
        "/tt",
        format!("state/app/{}/#", tenant_name),
    )];

    // Create the token request
    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs() as i64
        + 600; // 10 minutes in seconds

    let token_request = RequestDataAccessToken::new(&tenant_name, "External-client-id")
        .set_exp(expiration_time)
        .set_claims(permissions);

    // Fetch the DataAccessToken
    let token = token_fetcher
        .get_or_fetch_data_access_token(token_request)
        .await?;

    println!(
        "\nGenerated DataAccessToken with partial permissions: {:?}\n",
        token
    );

    // Extract and send the raw token to the external client
    let raw_token = token.raw_token();
    println!("Raw token to send to external client: {}", raw_token);

    Ok(())
}
