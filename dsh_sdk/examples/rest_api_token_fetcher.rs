//! This example demonstrates how to use the RestTokenFetcher to get a token and use it to get a list of topics
//! from the REST API.
//!
//! Run this example with:
//! ```sh
//! CLIENT_SECRET=your_client_secret TENANT=your_tenant cargo run --features rest-token-fetcher --example rest_api_token_fetcher
//! ```
use dsh_rest_api_client::Client;
use dsh_sdk::{Platform, RestTokenFetcherBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let platform = Platform::NpLz;
    let client_secret =
        env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set as environment variable");
    let tenant = env::var("TENANT").expect("TENANT must be set as environment variable");
    let client = Client::new(platform.endpoint_rest_api());
    let tf = RestTokenFetcherBuilder::new(platform)
        .tenant_name(tenant.clone())
        .client_secret(client_secret)
        .build()
        .unwrap();

    let response = client
        .topic_get_by_tenant_topic(&tenant, &tf.get_token().await.unwrap())
        .await;

    println!("Available topics: {:#?}", response);
}
