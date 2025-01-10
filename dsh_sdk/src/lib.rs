#![doc = include_str!("../README.md")]
#![allow(deprecated)]

// to be kept in v0.6.0
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
pub mod protocol_adapters;
pub mod utils;

#[cfg(feature = "schema-store")]
pub mod schema_store;

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
    note = "The `Properties` struct phased out. Use
    `dsh_sdk::Dsh` for an all-in-one struct, similar to the original `Properties`;
    `dsh_sdk::certificates` for all certificate related info;
    `dsh_sdk::datastream` for all datastream related info;
    "
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
    note = "`RestTokenFetcher` and `RestTokenFetcherBuilder` are renamed to `ManagementApiTokenFetcher` and `ManagementApiTokenFetcherBuilder`"
)]
mod rest_api_token_fetcher;
#[cfg(feature = "management-api-token-fetcher")]
pub use rest_api_token_fetcher::{RestTokenFetcher, RestTokenFetcherBuilder};

// Environment variables
const VAR_APP_ID: &str = "MARATHON_APP_ID";
const VAR_TASK_ID: &str = "MESOS_TASK_ID";
const VAR_DSH_CA_CERTIFICATE: &str = "DSH_CA_CERTIFICATE";
const VAR_DSH_SECRET_TOKEN: &str = "DSH_SECRET_TOKEN";
const VAR_DSH_SECRET_TOKEN_PATH: &str = "DSH_SECRET_TOKEN_PATH";
const VAR_DSH_TENANT_NAME: &str = "DSH_TENANT_NAME";
const VAR_KAFKA_CONFIG_HOST: &str = "KAFKA_CONFIG_HOST";

// kafka general
const VAR_KAFKA_BOOTSTRAP_SERVERS: &str = "KAFKA_BOOTSTRAP_SERVERS";
const VAR_SCHEMA_REGISTRY_HOST: &str = "SCHEMA_REGISTRY_HOST";

// Consumer
const VAR_KAFKA_AUTO_OFFSET_RESET: &str = "KAFKA_AUTO_OFFSET_RESET";
const VAR_KAFKA_CONSUMER_GROUP_TYPE: &str = "KAFKA_CONSUMER_GROUP_TYPE";
const VAR_KAFKA_ENABLE_AUTO_COMMIT: &str = "KAFKA_ENABLE_AUTO_COMMIT";
const VAR_KAFKA_GROUP_ID: &str = "KAFKA_GROUP_ID";
const VAR_KAFKA_CONSUMER_SESSION_TIMEOUT_MS: &str = "KAFKA_CONSUMER_SESSION_TIMEOUT_MS";
const VAR_KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES: &str =
    "KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES";

// Producer
const VAR_KAFKA_PRODUCER_BATCH_NUM_MESSAGES: &str = "KAFKA_PRODUCER_BATCH_NUM_MESSAGES";
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES: &str =
    "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES";
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES: &str =
    "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES";
const VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS: &str = "KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS";

const VAR_PKI_CONFIG_DIR: &str = "PKI_CONFIG_DIR";

const VAR_LOCAL_DATASTREAMS_JSON: &str = "LOCAL_DATASTREAMS_JSON";

const DEFAULT_CONFIG_HOST: &str = "https://pikachu.dsh.marathon.mesos:4443";
