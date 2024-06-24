//! Implementation of MQTT library for DSH
//!
//! The goals of dsh_mqtt_client is:
//! * To facilitate fetching MQTT Token
//! * To provide a robust MQTT Client to stream data
//!
//! # Config
//!
//! DshConfig is to prepare URLs for Rest and MQTT token api depending on DSH environment. Create DshConfig with only passing DshEnv.
//!
//! ### Example:
//! ```
//! use std::sync::Arc;
//! use dsh_mqtt_client::{config::{DshConfig, DshEnv}};
//!
//! let dsh_conf = Arc::new(DshConfig::new(DshEnv::Dev));
//! ```
//!
//! # Model
//!
//! RetrieveTokenRequest is the model user should use to pass tenant, api_key, claims and client_id information. Suggested way of setting tenant and api_key is setting via environment variables.
//!
//! # Fetch MQTT token
//!
//! Create `DshAuthenticationServiceAdapter` and call `retrieve_token`` function passing `RetrieveTokenRequest` you created.
//!
//! ### Example:
//! ```
//! use std::sync::Arc;
//! use std::env;
//! use dsh_mqtt_client::{config::{DshConfig, DshEnv}};
//! use dsh_mqtt_client::service::{DshAuthenticationServiceAdapter,AuthenticationService};
//! use dsh_mqtt_client::model::mqtt_model::MqttToken;
//! use dsh_mqtt_client::model::token_request_attr::RetrieveTokenRequest;
//!
//! #[tokio::main]
//! # async fn main() {
//! let retrieve_request = RetrieveTokenRequest {
//! tenant: env::var("TENANT_NAME").unwrap().to_string(),
//! api_key: env::var("API_KEY").unwrap().to_string(),
//! claims: None,
//! client_id: uuid::Uuid::new_v4().to_string(),
//! };
//!
//! let dsh_conf = Arc::new(DshConfig::new(DshEnv::Dev));
//! let service: DshAuthenticationServiceAdapter = DshAuthenticationServiceAdapter::new(dsh_conf);
//! let mqtt_token: MqttToken = service.retrieve_token(retrieve_request.clone()).await.unwrap();
//! }
//! ```
/// Includes all the logic and steps to fetch rest and mqtt tokens.
mod authentication_client;
/// Prepares api endpoints depending on DSH environment.
pub mod config;
/// Implements errors
pub mod error;
/// Rest, Mqtt and RetrieveTokenRequest models
pub mod model;
/// Includes async service which uses authentication_client to fetch mqtt token.
pub mod service;
