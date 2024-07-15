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
mod bootstrap;
pub mod certificates;
pub mod datastream;
mod pki_config_dir;
pub mod properties;

// Re-export the properties struct to avoid braking changes
pub use super::utils::get_configured_topics;
pub use properties::Properties;
