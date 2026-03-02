//! Minimal HTTP protocol adapter for DSH: GET retained message as `text/plain`.
//!
//! Authentication model (per DSH docs):
//! - The Messaging API requires an **MQTT token** in `Authorization: Bearer <token>`.
//! - The token itself is obtained elsewhere (SDK token fetcher / trusted service).
//! - Do **not** put API keys or REST tokens in this adapter.
//!
//! This module intentionally keeps a tiny public surface and isolates the `reqwest`
//! dependency to a single file for maintainability and clarity.

pub mod config;
pub mod client;

pub use config::{Accept, HttpConfig};
pub use client::{HttpClient, HttpError};

// # Quick start (pseudo; requires a valid MQTT token from the SDK)
//
// ```no_run
// use dsh_sdk::protocol_adapters::http_protocol::{HttpClient, HttpConfig};
//
// # #[tokio::main]
// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
// let cfg = HttpConfig::default()
//     .with_base_url("https://dsh.example.com/v1")
//     .with_tenant("GREENBOX-DEV")
//     // IMPORTANT: The MQTT token should be fetched by the SDK's token-fetcher in a trusted service
//     .with_mqtt_token("YOUR_MQTT_TOKEN");
//
// let client = HttpClient::new(&cfg)?;
// let payload = client.get_text_plain(&cfg, "my/topic").await?;
// println!("payload: {}", payload);
// # Ok(()) }
// ```