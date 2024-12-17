//! # DSH
//!
//! Dsh properties struct. Create new to initialize all related components to connect to the DSH kafka clusters and get metadata of your tenant.
//! - Availablable datastreams info
//! - Metadata of running container/task
//! - Certificates for Kafka and DSH
//!
//! ## High level API
//!
//! The properties struct contains a high level API to interact with the DSH.
//! This includes generating RDKafka config for creating a consumer/producer and Reqwest config builder for Schema Registry.
//!
//! ### Example:
//! ```
//! use dsh_sdk::Properties;
//! use dsh_sdk::rdkafka::consumer::stream_consumer::StreamConsumer;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>>{
//! let dsh_properties = Properties::get();
//! let consumer: StreamConsumer = dsh_properties.consumer_rdkafka_config().create()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Low level API
//! It is also possible to get avaiable metadata or the certificates from the properties struct.
//!
//! ### Example:
//! ```no_run
//! # use dsh_sdk::Properties;
//! # use dsh_sdk::rdkafka::consumer::stream_consumer::StreamConsumer;
//! # fn main() -> Result<(), Box<dyn std::error::Error>>{
//! #    let dsh_properties = Properties::get();
//! // check for write access to topic
//! let write_access = dsh_properties.datastream().get_stream("scratch.local.local-tenant").expect("Topic not found").write_access();
//! // get the certificates, for example DSH_KAFKA_CERTIFICATE
//! let dsh_kafka_certificate = dsh_properties.certificates()?.dsh_kafka_certificate_pem();
//! #     Ok(())
//! # }
//! ```
//! ## Kafka Proxy / VPN / Local
//! Read [CONNECT_PROXY_VPN_LOCAL.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/CONNECT_PROXY_VPN_LOCAL.md) on how to connect to DSH with Kafka Proxy, VPN or to a local Kafka cluster.
//!
//! # Metrics
//! The metrics module provides a way to expose prometheus metrics. This module is a re-export of the `prometheus` crate. It also contains a function to start a http server to expose the metrics to DSH.
//!
//! See [metrics](metrics/index.html) for more information.
//!
//! # Graceful shutdown
//! To implement a graceful shutdown in your service, you can use the `Shutdown` struct. This struct has an implementation based on the best practices example of Tokio.
//!
//! This gives you the option to properly handle shutdown in your components/tasks.
//! It listens for SIGTERM requests and sends out shutdown requests to all shutdown handles.
//!
//! See [graceful_shutdown](graceful_shutdown/index.html) for more information.
//!
//! # DLQ (Dead Letter Queue)
//! `OPTIONAL feature: dlq`
//!
//! This is an experimental feature and is not yet finalized.
//!
//! This implementation only includes pushing messages towards a kafka topic. (Dead or Retry topic)
//!
//! ### NOTE:
//! This implementation does not (and will not) handle any other DLQ related tasks like:
//!     - Retrying messages
//!     - Handling messages in DLQ
//!     - Monitor the DLQ
//! Above tasks should be handled by a seperate component set up by the user, as these tasks are use case specific and can handle different strategies.
//!
//! The DLQ is implemented by running the `Dlq` struct to push messages towards the DLQ topics.
//! The `ErrorToDlq` trait can be implemented on your defined errors, to be able to send messages towards the DLQ Struct.

// to be kept in v0.6.0
pub mod certificates;
pub mod datastream;
#[cfg(feature = "management-api")]
pub mod management_api;

pub mod dsh;
pub mod error;
pub mod protocol_adapters;
pub mod utils;

#[doc(inline)]
pub use dsh::Dsh;

#[cfg(feature = "management-api")]
pub use management_api::token_fetcher::{
    ManagementApiTokenFetcher, ManagementApiTokenFetcherBuilder,
};

#[doc(inline)]
pub use utils::Platform;

// TODO: to be removed in v0.6.0
#[cfg(feature = "dlq")]
#[deprecated(since = "0.5.0", note = "The DLQ is moved to `dsh_sdk::utils::dlq`")]
pub mod dlq;

#[cfg(feature = "bootstrap")]
#[deprecated(
    since = "0.5.0",
    note = "The `dsh` as module is phased out. Use
    `dsh_sdk::Dsh` for all info about your running container;
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

#[cfg(any(feature = "rdkafka-ssl", feature = "rdkafka-ssl-vendored"))]
pub use rdkafka;
#[cfg(feature = "mqtt-token-fetcher")]
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::mqtt_token_fetcher` is moved to `dsh_sdk::protocol_adapters::token_fetcher`"
)]
pub mod mqtt_token_fetcher;
#[cfg(feature = "bootstrap")]
pub use dsh_old::Properties;

#[cfg(feature = "management-api")]
#[deprecated(
    since = "0.5.0",
    note = "`RestTokenFetcher` and `RestTokenFetcherBuilder` are renamed to `ManagementApiTokenFetcher` and `ManagementApiTokenFetcherBuilder`"
)]
mod rest_api_token_fetcher;
#[cfg(feature = "management-api")]
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
