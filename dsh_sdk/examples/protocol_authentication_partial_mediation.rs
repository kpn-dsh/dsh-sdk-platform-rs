//! Example: API Client Authentication service fetching a REST token for a device.
//!
//! The REST token enables a device to fetch its own DataAccessToken to connect to protocol adapters.
//!
//! ## Important Notes:
//! - **Do NOT implement this logic in device applications or external clients!**
//! - This logic is part of the **API Client role** in the DSH architecture.
//! - The API Client uses a long-lived API_KEY (REST token) to fetch short-lived tokens for devices.
//! - **The API_KEY must never be distributed.**

use dsh_sdk::protocol_adapters::token::{
    api_client_token_fetcher::ApiClientTokenFetcher, DatastreamsMqttTokenClaim,
    RequestDataAccessToken, RequestRestToken, RestToken,
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
    // We want to delegate a short-lived REST token with the following properties:
    // - REST token:
    //   - Valid for 10 minutes
    //   - Allows fetching a DataAccessToken with:
    //     - Maximum expiration of 5 minutes
    //     - Usage restricted to the external client ID "External-client-id"

    println!("API Authentication Service Code:\n");

    // Instantiate the API Client Token Fetcher
    let token_fetcher = ApiClientTokenFetcher::new(api_key, PLATFORM);

    // Define the claim for the DatastreamsMqttToken endpoint
    let claim = DatastreamsMqttTokenClaim::new()
        .set_id("External-client-id") // External client ID (should be unique)
        .set_relexp(300); // Relative expiration of 5 minutes (300 seconds)

    // Create a token request with the claim and expiration time
    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs() as i64
        + 600; // 10 minutes in seconds

    let rest_token_request = RequestRestToken::new(&tenant_name)
        .set_exp(expiration_time)
        .set_claims(claim);

    // Fetch the REST token
    let partial_token = token_fetcher
        .get_or_fetch_rest_token(rest_token_request)
        .await?;
    println!(
        "\nGenerated REST token with partial permissions: {:?}",
        partial_token
    );

    // Send the raw token to the external client
    let raw_token = partial_token.raw_token();
    println!("\nRaw token to send to external client: {}", raw_token);

    // -------------------------------------------------------------------------------------
    // External Client Code:
    //
    // When the external client receives the raw_token, it can fetch its own DataAccessToken:
    // 1. Parse the raw token into a RestToken.
    // 2. Prepare a request for a DataAccessToken with the external client ID.
    // 3. Fetch the DataAccessToken using the RestToken.
    // -------------------------------------------------------------------------------------
    println!("\nExternal Client Code:");

    // Parse the raw token into a RestToken
    let rest_token = RestToken::parse(raw_token)?;
    println!("\nParsed REST token: {:?}", rest_token);

    // Prepare a request for a DataAccessToken using the external client ID
    let data_access_request =
        RequestDataAccessToken::new(rest_token.tenant_id(), "External-client-id");

    // Use an HTTP client to send the request and fetch the DataAccessToken
    let http_client = reqwest::Client::new();
    let data_access_token = data_access_request.send(&http_client, rest_token).await?;
    println!("\nFetched DataAccessToken: {:#?}", data_access_token);

    Ok(())
}
