#![doc(
    html_favicon_url = "https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/63989eb23690d26bcd4fe69b1ec1f4d0f8b8d5e0/doc/kpn.svg"
)]
#![doc(
    html_logo_url = "https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/63989eb23690d26bcd4fe69b1ec1f4d0f8b8d5e0/doc/kpn.svg"
)]
#![doc = include_str!("../README.md")]
#![deny(deprecated)]

#[cfg(feature = "bootstrap")]
pub mod certificates;
#[cfg(feature = "bootstrap")]
pub mod datastream;
#[cfg(feature = "bootstrap")]
pub mod dsh;
#[cfg(feature = "bootstrap")]
mod error;
#[cfg(feature = "management-api-token-fetcher")]
pub mod management_api;
// Protocol adapters and utilities
pub mod protocol_adapters;
#[cfg(feature = "schema-store")]
pub mod schema_store;
pub mod utils;

// Re-exports for convenience
#[cfg(feature = "management-api-token-fetcher")]
#[doc(inline)]
pub use management_api::{ManagementApiTokenFetcher, ManagementApiTokenFetcherBuilder};
#[cfg(feature = "kafka")]
#[doc(inline)]
pub use protocol_adapters::kafka_protocol::DshKafkaConfig;
#[doc(inline)]
pub use utils::Platform;
#[cfg(feature = "bootstrap")]
#[doc(inline)]
pub use {dsh::Dsh, error::DshError};

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
