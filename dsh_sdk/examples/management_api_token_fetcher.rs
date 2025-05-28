//! This example demonstrates how to use the `RestTokenFetcher` to get a token and use it to get a list of topics
//! from the REST API.
//!
//! Example is using the following crates:
//! - [`dsh_sdk`] with features = ["management-api-token-fetcher"] for fetching token to authenticate to the DSH Management API
//! - [`dsh_rest_api_client`] for the DSH Management API client
//! - [`tokio`] with features = ["full"] for async runtime
//!
//! Run this example with:
//! ```sh
//! CLIENT_SECRET={your_client_secret} TENANT={your_tenant} cargo run --features rest-token-fetcher --example rest_api_token_fetcher
//! ```
use dsh_rest_api_client::Client;
use dsh_sdk::{ManagementApiTokenFetcherBuilder, Platform};
use std::env;

#[tokio::main]
async fn main() {
    let platform = Platform::Poc;
    let client_secret =
        env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set as environment variable");
    let tenant = env::var("TENANT").expect("TENANT must be set as environment variable");
    let client = Client::new(platform.endpoint_management_api());
    let tf = ManagementApiTokenFetcherBuilder::new(platform)
        .tenant_name(&tenant)
        .client_secret(client_secret)
        .build()
        .unwrap();

    let response = client
        .topic_get_by_tenant_topic(&tenant, &tf.get_token().await.unwrap())
        .await;

    println!("Available topics: {:#?}", response);
}
