//! # DSH Properties
//!
//! This module contains logic to connect to Kafka on DSH and retreive all properties of your tenant.
//!
//! From `Properties` there are level functions to get the correct config to connect to Kafka and schema store.
//! For more low level functions, see
//!     - [datastream](datastream/index.html) module.
//!     - [certificates](certificates/index.html) module.
//!
//! # Example
//! ```
//! use dsh_sdk::Properties;
//! use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let dsh_properties = Properties::get();
//! let consumer_config = dsh_properties.consumer_rdkafka_config();
//! let consumer: StreamConsumer = consumer_config.create()?;
//!
//! # Ok(())
//! # }
//! ```
pub mod bootstrap;
pub mod certificates;
pub mod datastream;
mod pki_config_dir;
mod properties;
mod utils;

// Re-export the properties struct to avoid braking changes
pub use properties::Properties;
pub use utils::get_configured_topics;

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
