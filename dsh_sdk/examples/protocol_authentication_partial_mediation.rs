//! Demonstrates how an API Client Authentication service can fetch a REST token for a device.
//!
//! With this [RestToken], the device can fetch it's own DataAccessToken to connect to the protocol adapters.
//!
//! NEVER implement this logic in a device application/external clients!
//!
//! This logic is part of API Client role in the DSH architecture, where the API Client
//! delegates short lived tokens to devices with proper permissions. The API_KEY in this
//! code is the long lived REST token that the API Client uses to fetch short lived tokens
//! for devices and this API_KEY should never be distributed
use std::time::SystemTime;

use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
use dsh_sdk::protocol_adapters::token::data_access_token::RequestDataAccessToken;
use dsh_sdk::protocol_adapters::token::rest_token::{
    DatastreamsMqttTokenClaim, RequestRestToken, RestToken,
};

/// The platform to fetch the token for.
const PLATFORM: dsh_sdk::Platform = dsh_sdk::Platform::NpLz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tenant_name = std::env::var("TENANT").expect("TENANT env variable is not set");
    let api_key = std::env::var("API_KEY").expect("API_KEY env variable is not set");

    // Start logger to Stdout to show what is happening
    env_logger::builder()
        .filter(Some("dsh_sdk"), log::LevelFilter::Trace)
        .target(env_logger::Target::Stdout)
        .init();

    // Let's say that for example your API Authentication service receives a request from an external client
    // and you want to delegate a Rest token with the following partial permissions to the external client:
    // - The RestToken
    //      - Is valid for 10 minutes/
    //      - Can be used to fetch a DataAccessToken with the following permissions
    //          - Have a maximum expiration time of 5 minutes
    //          - Only be used by the external client with the id "External-client-id"

    println!("API Authetication service code:");

    // Create a token fetcher with the API key and platform
    let token_fetcher = ApiClientTokenFetcher::new(api_key, PLATFORM);

    // Create claim for MqttToken endpoint
    let claim = DatastreamsMqttTokenClaim::new()
        // Set the client id to the external client id (should be unique)
        .set_id("External-client-id")
        // Set the expiration time of DataAcessToken Token to expire in 5 minutes from now
        .set_relexp(60 * 5);
    // Create a token request with claim and specific expiration time
    let request = RequestRestToken::new(&tenant_name)
        // Set the expiration time of Rest Token to expire in 10 minutes from now
        .set_exp(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as i64
                + (60 * 10),
        )
        .set_claims(claim);

    let partial_token = token_fetcher.get_or_fetch_rest_token(request).await?;
    println!(
        "\nRest token with partial permission = {:?}\n",
        partial_token
    );

    // send the token as raw token to the external client
    let raw_token = partial_token.raw_token();

    // -------------------------------------------------------------------------------------
    // External Client code:
    //
    // When the external client receives the raw_token it can fetch it's own DataAccessToken
    // - Parse the raw token to a RestToken
    // - Prepare a request for a DataAccessToken with the external client id
    // - Fetch the DataAccessToken
    // -------------------------------------------------------------------------------------
    println!("\nExternal Client code:\n");

    // Parse the raw token to a RestToken
    let rest_token = RestToken::parse(raw_token)?;
    println!("Parsed rest token: {:?}\n", rest_token);

    // Prepare a request for a DataAccessToken with the external client id
    let request = RequestDataAccessToken::new(rest_token.tenant_id(), "External-client-id");
    let client = reqwest::Client::new();

    // Fetch the DataAccessToken
    let data_access_token = request.send(&client, rest_token).await?;
    println!("Data access token: {:#?}", data_access_token);

    Ok(())
}
