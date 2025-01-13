#![doc = include_str!("../README.md")]
#![allow(deprecated)]

// Keep in v0.6.0 for backward compatibility
#[cfg(feature = "bootstrap")]
pub mod certificates;
#[cfg(feature = "bootstrap")]
pub mod datastream;
#[cfg(feature = "bootstrap")]
pub mod dsh;
#[cfg(feature = "bootstrap")]
mod error;

// Management API token fetcher feature
#[cfg(feature = "management-api-token-fetcher")]
pub mod management_api;

// Protocol adapters and utilities
pub mod protocol_adapters;
pub mod utils;

// Schema Store feature
#[cfg(feature = "schema-store")]
pub mod schema_store;

// Re-exports for convenience
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

// TODO: to be removed in v0.6.0
#[cfg(feature = "dlq")]
#[deprecated(since = "0.5.0", note = "The DLQ is moved to `dsh_sdk::utils::dlq`")]
pub mod dlq;

#[cfg(feature = "bootstrap")]
#[deprecated(
    since = "0.5.0",
    note = "The `Properties` struct is phased out. Use `dsh_sdk::Dsh` for an all-in-one struct; \
            `dsh_sdk::certificates` for certificate management; `dsh_sdk::datastream` for \
            datastream handling."
)]
pub mod dsh_old;

#[cfg(feature = "graceful-shutdown")]
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::graceful_shutdown` is moved to `dsh_sdk::utils::graceful_shutdown`"
)]
pub mod graceful_shutdown;

#[cfg(feature = "metrics")]
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::metrics` is moved to `dsh_sdk::utils::metrics`"
)]
pub mod metrics;

#[cfg(all(feature = "protocol-token-fetcher", feature = "bootstrap"))]
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::mqtt_token_fetcher` is moved to `dsh_sdk::protocol_adapters::token_fetcher`"
)]
pub mod mqtt_token_fetcher;

#[cfg(feature = "bootstrap")]
pub use dsh_old::Properties;

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
// They are grouped into logical sections for clarity.

// -------------------- General environment variables --------------------

/// Environment variable for retrieving the Marathon application ID.
const VAR_APP_ID: &str = "MARATHON_APP_ID";

/// Environment variable for retrieving the Mesos task ID.
const VAR_TASK_ID: &str = "MESOS_TASK_ID";

/// Environment variable for retrieving the DSH CA certificate
const VAR_DSH_CA_CERTIFICATE: &str = "DSH_CA_CERTIFICATE";

/// Inline secret token used to authorize requests to DSH.
const VAR_DSH_SECRET_TOKEN: &str = "DSH_SECRET_TOKEN";

/// Filesystem path to the DSH secret token.
const VAR_DSH_SECRET_TOKEN_PATH: &str = "DSH_SECRET_TOKEN_PATH";

/// Tenant name for DSH.
const VAR_DSH_TENANT_NAME: &str = "DSH_TENANT_NAME";

/// DSH config host, typically pointing to an internal endpoint.
const VAR_KAFKA_CONFIG_HOST: &str = "KAFKA_CONFIG_HOST";

/// PKI configuration directory.
const VAR_PKI_CONFIG_DIR: &str = "PKI_CONFIG_DIR";

/// Local datastream configurations in JSON format (e.g., for Kafka Proxy or local testing).
const VAR_LOCAL_DATASTREAMS_JSON: &str = "LOCAL_DATASTREAMS_JSON";

// -------------------- Kafka general environment variables --------------------

/// Lists Kafka bootstrap servers.
const VAR_KAFKA_BOOTSTRAP_SERVERS: &str = "KAFKA_BOOTSTRAP_SERVERS";

/// Specifies the schema registry host for Kafka.
const VAR_SCHEMA_REGISTRY_HOST: &str = "SCHEMA_REGISTRY_HOST";

// -------------------- Kafka consumer environment variables --------------------

/// Controls how offsets are handled when no offset is available (e.g., "earliest" or "latest").
const VAR_KAFKA_AUTO_OFFSET_RESET: &str = "KAFKA_AUTO_OFFSET_RESET";

/// Indicates the Kafka consumer group type.
const VAR_KAFKA_CONSUMER_GROUP_TYPE: &str = "KAFKA_CONSUMER_GROUP_TYPE";

/// Specifies whether consumer auto-commit is enabled.
const VAR_KAFKA_ENABLE_AUTO_COMMIT: &str = "KAFKA_ENABLE_AUTO_COMMIT";

/// Defines the group ID for Kafka consumers.
const VAR_KAFKA_GROUP_ID: &str = "KAFKA_GROUP_ID";

/// Kafka consumer session timeout in milliseconds.
const VAR_KAFKA_CONSUMER_SESSION_TIMEOUT_MS: &str = "KAFKA_CONSUMER_SESSION_TIMEOUT_MS";

/// Kafka consumer's maximum queued buffering in kilobytes.
const VAR_KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES: &str =
    "KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES";

// -------------------- Kafka producer environment variables --------------------

/// Kafka producer's number of messages per batch.
const VAR_KAFKA_PRODUCER_BATCH_NUM_MESSAGES: &str = "KAFKA_PRODUCER_BATCH_NUM_MESSAGES";

/// Kafka producer's maximum queue buffering for messages.
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES: &str =
    "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES";

/// Kafka producer's maximum queue buffering in kilobytes.
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES: &str =
    "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES";

/// Kafka producer's maximum queue buffering duration in milliseconds.
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS: &str = "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS";

// ----------------------------------------------------------------------------
// Default configuration
// ----------------------------------------------------------------------------

/// Default configuration host for DSH, used if no environment variable overrides are provided.
const DEFAULT_CONFIG_HOST: &str = "https://pikachu.dsh.marathon.mesos:4443";
