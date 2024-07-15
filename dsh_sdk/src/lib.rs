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

#[cfg(feature = "dlq")]
pub mod dlq;
#[cfg(feature = "bootstrap")]
pub mod dsh;
pub mod error;
#[cfg(feature = "graceful_shutdown")]
pub mod graceful_shutdown;
#[cfg(feature = "metrics")]
pub mod metrics;
#[cfg(any(feature = "rdkafka-ssl", feature = "rdkafka-ssl-vendored"))]
pub use rdkafka;
#[cfg(feature = "rest-token-fetcher")]
mod rest_api_token_fetcher;
mod utils;

#[cfg(feature = "bootstrap")]
pub use dsh::Properties;
#[cfg(feature = "rest-token-fetcher")]
pub use rest_api_token_fetcher::{RestTokenFetcher, RestTokenFetcherBuilder};
pub use utils::Platform;

// Environment variables
const VAR_APP_ID: &str = "MARATHON_APP_ID";
const VAR_TASK_ID: &str = "MESOS_TASK_ID";
const VAR_DSH_CA_CERTIFICATE: &str = "DSH_CA_CERTIFICATE";
const VAR_DSH_SECRET_TOKEN: &str = "DSH_SECRET_TOKEN";
const VAR_DSH_SECRET_TOKEN_PATH: &str = "DSH_SECRET_TOKEN_PATH";
const VAR_DSH_TENANT_NAME: &str = "DSH_TENANT_NAME";

const VAR_KAFKA_AUTO_OFFSET_RESET: &str = "KAFKA_AUTO_OFFSET_RESET";
const VAR_KAFKA_BOOTSTRAP_SERVERS: &str = "KAFKA_BOOTSTRAP_SERVERS";
const VAR_KAFKA_CONFIG_HOST: &str = "KAFKA_CONFIG_HOST";
const VAR_KAFKA_CONSUMER_GROUP_TYPE: &str = "KAFKA_CONSUMER_GROUP_TYPE";
const VAR_KAFKA_ENABLE_AUTO_COMMIT: &str = "KAFKA_ENABLE_AUTO_COMMIT";
const VAR_KAFKA_GROUP_ID: &str = "KAFKA_GROUP_ID";

const VAR_PKI_CONFIG_DIR: &str = "PKI_CONFIG_DIR";
const VAR_SCHEMA_REGISTRY_HOST: &str = "SCHEMA_REGISTRY_HOST";

const VAR_LOCAL_DATASTREAMS_JSON: &str = "LOCAL_DATASTREAMS_JSON";

const DEFAULT_CONFIG_HOST: &str = "https://pikachu.dsh.marathon.mesos:4443";
