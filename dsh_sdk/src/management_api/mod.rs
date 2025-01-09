//! Fetch and store tokens for the DSH Management Rest API client
//!
//! This module is meant to be used together with the [dsh_rest_api_client].
//!
//! The TokenFetcher will fetch and store access tokens to be used in the DSH Rest API client.
//!
//! ## Example
//! Recommended usage is to use the [ManagementApiTokenFetcherBuilder] to create a new instance of the token fetcher.
//! However, you can also create a new instance of the token fetcher directly.
//! ```no_run
//! use dsh_sdk::{ManagementApiTokenFetcherBuilder, Platform};
//! use dsh_rest_api_client::Client;
//!
//! const CLIENT_SECRET: &str = "";
//! const TENANT: &str = "tenant-name";
//!
//! #[tokio::main]
//! async fn main() {
//!     let platform = Platform::NpLz;
//!     let client = Client::new(platform.endpoint_rest_api());
//!
//!     let tf = ManagementApiTokenFetcherBuilder::new(platform)
//!         .tenant_name(TENANT.to_string())
//!         .client_secret(CLIENT_SECRET.to_string())
//!         .build()
//!         .unwrap();
//!
//!     let response = client
//!         .topic_get_by_tenant_topic(TENANT, &tf.get_token().await.unwrap())
//!         .await;
//!     println!("Available topics: {:#?}", response);
//! }
//! ```
mod error;
mod token_fetcher;

#[doc(inline)]
pub use error::ManagementApiTokenError;

#[doc(inline)]
pub use token_fetcher::{ManagementApiTokenFetcher, ManagementApiTokenFetcherBuilder};
