use dsh_rest_api_client::Client;
use dsh_sdk::{Platform, RestTokenFetcherBuilder};

const CLIENT_SECRET: &str = "";
const TENANT: &str = "tenant-name";

#[tokio::main]
async fn main() {
    let platform = Platform::NpLz;
    let client = Client::new(platform.endpoint_rest_api());

    let tf = RestTokenFetcherBuilder::new(platform)
        .tenant_name(TENANT.to_string())
        .client_secret(CLIENT_SECRET.to_string())
        .build()
        .unwrap();

    let response = client
        .get_allocation_by_tenant_topic(TENANT, &tf.get_token().await.unwrap())
        .await;

    println!("Available topics of my tenant: {:#?}", response);
}
