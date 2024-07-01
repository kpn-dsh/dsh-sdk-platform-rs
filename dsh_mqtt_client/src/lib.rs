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
//! # Model
//!
//! RetrieveTokenRequest is the model user should use to pass tenant, api_key, claims and client_id information. Suggested way of setting tenant and api_key is setting via environment variables.
//!
//! # Fetch MQTT token
//!
//! Create `DshAuthenticationServiceAdapter` and call `retrieve_token`` function passing `RetrieveTokenRequest` you created.
//!

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
/// Module for constructing MQTT Client
pub mod stream_client;
