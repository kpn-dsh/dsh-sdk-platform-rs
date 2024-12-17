//! # DSH
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

#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::dsh::certificates` is moved to `dsh_sdk::certificates`"
)]
pub mod certificates;
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::dsh::datastream` is moved to `dsh_sdk::datastream`"
)]
pub mod datastream;
#[deprecated(
    since = "0.5.0",
    note = "`dsh_sdk::dsh::properties` is moved to `dsh_sdk::dsh`"
)]
pub mod properties;

// Re-export the properties struct to avoid braking changes

#[deprecated(
    since = "0.5.0",
    note = "get_configured_topics is moved to `dsh_sdk::utils::get_configured_topics`"
)]
pub fn get_configured_topics() -> Result<Vec<String>, crate::error::DshError> {
    let kafka_topic_string = crate::utils::get_env_var("TOPICS")?;
    Ok(kafka_topic_string
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect())
}

pub use properties::Properties;
