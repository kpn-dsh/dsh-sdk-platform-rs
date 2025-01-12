#![doc = include_str!("../README.md")]
#![allow(deprecated)]

//! This crate provides core functionalities, environment variable references, and feature-gated
//! modules for working with DSH (Data Services Hub). It also includes deprecated items from
//! older versions of the API that remain available for backward compatibility until v0.6.0.
//!
//! # Crate Overview
//! - **Feature-gated modules**: Certain modules like [`certificates`], [`datastream`], [`dsh`] etc.
//!   are only compiled if the `bootstrap` feature is enabled.  
//! - **Environment variables**: Constants representing environment variables are declared for
//!   configuring Kafka, schema registry, and other components.  
//! - **Deprecated modules**: Modules such as [`dlq`], [`dsh_old`], [`graceful_shutdown`], and
//!   [`metrics`], among others, are slated for removal in v0.6.0 or have been moved to new
//!   namespaces.  
//!
//! Refer to the included `README.md` for more information on usage, setup, and features.

// ----------------------------------------------------------------------------
// Bootstrap-related modules (enabled via the "bootstrap" feature)
// ----------------------------------------------------------------------------

/// The `certificates` module provides types and functions for handling
/// certificate-based security within the DSH platform.
#[cfg(feature = "bootstrap")]
pub mod certificates;

/// The `datastream` module contains logic for creating, managing, and retrieving
/// datastream configurations.
#[cfg(feature = "bootstrap")]
pub mod datastream;

/// The `dsh` module provides an all-in-one struct (`Dsh`) that encapsulates
/// the primary Data Services Hub functionality.
#[cfg(feature = "bootstrap")]
pub mod dsh;

/// Internal error handling for DSH-related operations.
#[cfg(feature = "bootstrap")]
mod error;

// ----------------------------------------------------------------------------
// Management API token fetcher (enabled via the "management-api-token-fetcher" feature)
// ----------------------------------------------------------------------------

/// The `management_api` module allows you to fetch tokens from a management API
/// to authorize your requests to DSH or other external services.
#[cfg(feature = "management-api-token-fetcher")]
pub mod management_api;

// ----------------------------------------------------------------------------
// Protocol adapters
// ----------------------------------------------------------------------------

/// The `protocol_adapters` module houses protocol-specific adapters (Kafka, MQTT, etc.)
/// that integrate external services with DSH.
pub mod protocol_adapters;

/// The `utils` module provides utility functions and helpers for the DSH SDK.
pub mod utils;

// ----------------------------------------------------------------------------
// Schema Store (enabled via the "schema-store" feature)
// ----------------------------------------------------------------------------

/// The `schema_store` module wraps interactions with an external schema registry,
/// providing convenience methods for schema validation and retrieval.
#[cfg(feature = "schema-store")]
pub mod schema_store;

// ----------------------------------------------------------------------------
// Re-exports for convenience
// ----------------------------------------------------------------------------

#[cfg(feature = "bootstrap")]
#[doc(inline)]
pub use {dsh::Dsh, error::DshError};

#[cfg(feature = "kafka")]
#[doc(inline)]
pub use protocol_adapters::kafka_protocol::DshKafkaConfig;

#[cfg(feature = "management-api-token-fetcher")]
#[doc(inline)]
pub use management_api::{ManagementApiTokenFetcher, ManagementApiTokenFetcherBuilder};

#[doc(inline)]
pub use utils::Platform;

// ----------------------------------------------------------------------------
// Deprecated modules (to be removed or relocated in a future release)
// ----------------------------------------------------------------------------

/// The `dlq` module is deprecated and will be removed in v0.6.0.
/// It has been moved to `dsh_sdk::utils::dlq`.
#[cfg(feature = "dlq")]
#[deprecated(since = "0.5.0", note = "The DLQ is moved to `dsh_sdk::utils::dlq`")]
pub mod dlq;

/// The old DSH struct (`Properties`) is deprecated as of v0.5.0.  
/// This module will be removed in v0.6.0. Refer to [`Dsh`](crate::dsh::Dsh)
/// or [`certificates`](crate::certificates) and [`datastream`](crate::datastream)
/// for updated functionality.
#[cfg(feature = "bootstrap")]
#[deprecated(
    since = "0.5.0",
    note = "The `Properties` struct is phased out. Use `dsh_sdk::Dsh` for an all-in-one struct; \
            `dsh_sdk::certificates` for certificate management; `dsh_sdk::datastream` for \
            datastream handling."
)]
pub mod dsh_old;

/// Graceful shutdown functionality is deprecated as of v0.5.0 and will be removed in v0.6.0.
/// Use `dsh_sdk::utils::graceful_shutdown` instead.
#[cfg(feature = "graceful-shutdown")]
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::graceful_shutdown` is moved to `dsh_sdk::utils::graceful_shutdown`"
)]
pub mod graceful_shutdown;

/// Metrics functionality is deprecated as of v0.5.0 and will be removed in v0.6.0.
/// Use `dsh_sdk::utils::metrics` instead.
#[cfg(feature = "metrics")]
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::metrics` is moved to `dsh_sdk::utils::metrics`"
)]
pub mod metrics;

/// Mqtt token fetching has been relocated as of v0.5.0.  
/// Use `dsh_sdk::protocol_adapters::token_fetcher` for equivalent functionality.
#[cfg(all(feature = "protocol-token-fetcher", feature = "bootstrap"))]
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::mqtt_token_fetcher` is moved to `dsh_sdk::protocol_adapters::token_fetcher`"
)]
pub mod mqtt_token_fetcher;

/// For backward compatibility, re-export the old `Properties` struct.
/// Will be removed in v0.6.0.
#[cfg(feature = "bootstrap")]
pub use dsh_old::Properties;

// ----------------------------------------------------------------------------
// Management API token fetcher (old naming, deprecated)
// ----------------------------------------------------------------------------

/// Deprecated in favor of [`ManagementApiTokenFetcher`](crate::management_api::ManagementApiTokenFetcher).
#[cfg(feature = "management-api-token-fetcher")]
#[deprecated(
    since = "0.5.0",
    note = "`RestTokenFetcher` and `RestTokenFetcherBuilder` are renamed to \
            `ManagementApiTokenFetcher` and `ManagementApiTokenFetcherBuilder`"
)]
mod rest_api_token_fetcher;

#[cfg(feature = "management-api-token-fetcher")]
pub use rest_api_token_fetcher::{RestTokenFetcher, RestTokenFetcherBuilder};

// ----------------------------------------------------------------------------
// Environment variables
// ----------------------------------------------------------------------------
// These constants define the names of environment variables used throughout the DSH SDK.
// They are separated into distinct categories for clarity (general, Kafka consumer/producer, etc.).

// -------------------- General environment variables --------------------

/// Environment variable for retrieving the Marathon application ID.
const VAR_APP_ID: &str = "MARATHON_APP_ID";

/// Environment variable for retrieving the Mesos task ID.
const VAR_TASK_ID: &str = "MESOS_TASK_ID";

/// Environment variable specifying the path to a CA certificate for the DSH platform.
const VAR_DSH_CA_CERTIFICATE: &str = "DSH_CA_CERTIFICATE";

/// Environment variable for an inline secret token used to authorize requests to DSH.
const VAR_DSH_SECRET_TOKEN: &str = "DSH_SECRET_TOKEN";

/// Environment variable specifying the filesystem path to the DSH secret token.
const VAR_DSH_SECRET_TOKEN_PATH: &str = "DSH_SECRET_TOKEN_PATH";

/// Environment variable indicating the tenant name for DSH.
const VAR_DSH_TENANT_NAME: &str = "DSH_TENANT_NAME";

/// Environment variable for configuring the DSH config host, typically referencing an internal endpoint.
const VAR_KAFKA_CONFIG_HOST: &str = "KAFKA_CONFIG_HOST";

/// Environment variable for specifying a PKI configuration directory.
const VAR_PKI_CONFIG_DIR: &str = "PKI_CONFIG_DIR";

/// Environment variable for specifying local datastream configurations in JSON format (for testing or local usage).
const VAR_LOCAL_DATASTREAMS_JSON: &str = "LOCAL_DATASTREAMS_JSON";

// -------------------- Kafka general environment variables --------------------

/// Environment variable listing Kafka bootstrap servers.
const VAR_KAFKA_BOOTSTRAP_SERVERS: &str = "KAFKA_BOOTSTRAP_SERVERS";

/// Environment variable specifying the schema registry host for Kafka.
const VAR_SCHEMA_REGISTRY_HOST: &str = "SCHEMA_REGISTRY_HOST";

// -------------------- Kafka consumer environment variables --------------------

/// Environment variable controlling how offsets are handled when no offset is available
/// (e.g., "earliest" or "latest").
const VAR_KAFKA_AUTO_OFFSET_RESET: &str = "KAFKA_AUTO_OFFSET_RESET";

/// Environment variable indicating the consumer group type.
const VAR_KAFKA_CONSUMER_GROUP_TYPE: &str = "KAFKA_CONSUMER_GROUP_TYPE";

/// Environment variable specifying whether consumer auto-commit is enabled.
const VAR_KAFKA_ENABLE_AUTO_COMMIT: &str = "KAFKA_ENABLE_AUTO_COMMIT";

/// Environment variable defining the group ID for Kafka consumers.
const VAR_KAFKA_GROUP_ID: &str = "KAFKA_GROUP_ID";

/// Environment variable for the Kafka consumer session timeout in milliseconds.
const VAR_KAFKA_CONSUMER_SESSION_TIMEOUT_MS: &str = "KAFKA_CONSUMER_SESSION_TIMEOUT_MS";

/// Environment variable for the Kafka consumer's maximum queued buffering in kilobytes.
const VAR_KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES: &str =
    "KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES";

// -------------------- Kafka producer environment variables --------------------

/// Environment variable for the Kafka producer's number of messages per batch.
const VAR_KAFKA_PRODUCER_BATCH_NUM_MESSAGES: &str = "KAFKA_PRODUCER_BATCH_NUM_MESSAGES";

/// Environment variable for the Kafka producer's maximum queue buffering for messages.
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES: &str =
    "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES";

/// Environment variable for the Kafka producer's maximum queue buffering in kilobytes.
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES: &str =
    "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES";

/// Environment variable specifying the Kafka producer's maximum queue buffering duration in milliseconds.
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS: &str = "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS";

// ----------------------------------------------------------------------------
// Default configuration
// ----------------------------------------------------------------------------

/// Default configuration host for DSH, used if no environment variable overrides are provided.
const DEFAULT_CONFIG_HOST: &str = "https://pikachu.dsh.marathon.mesos:4443";
