//! # Kafka Properties
//!
//! This module contains logic to connect to Kafka on DSH and get properties of your tenant. For example all available streams and topics.
//!
//! The implementation contains some high level functions to get the correct config to connect to Kafka and schema store.
//! For more low level functions, see
//!     - [datastream](datastream/index.html) module.
//!     - [certificates](certificates/index.html) module.
//!
//! # Example
//! ```
//! use dsh_sdk::dsh::Properties;
//! use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let dsh_properties = Properties::new().await?;
//!     
//!     let consumer_config = dsh_properties.consumer_rdkafka_config()?;
//!     let consumer: StreamConsumer = consumer_config.create()?;
//!
//!     Ok(())
//! }
//! ```
use log::warn;

use crate::error::DshError;
use std::env;

pub mod bootstrap;
pub mod certificates;
pub mod datastream;
#[cfg(feature = "local")]
pub mod local;

/// Get the configured topics from the environment variable TOPICS
/// Topics can be delimited by a comma
pub fn get_configured_topics() -> Result<Vec<String>, DshError> {
    let kafka_topic_string = env::var("TOPICS")?;
    Ok(kafka_topic_string
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect())
}

/// Kafka properties struct. Create new to initialize all related components to connect to the DSH kafka clusters
///  - Contains a struct similar to datastreams.json
///  - Metadata of running container/task
///  - Certificates for Kafka and DSH Schema Registry
///
/// # Example
/// ```
/// use dsh_sdk::dsh::Properties;
/// use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let dsh_properties = Properties::new().await?;
///     
///     let consumer_config = dsh_properties.consumer_rdkafka_config()?;
///     let consumer: StreamConsumer = consumer_config.create()?;
///
///     Ok(())
/// }
/// ```

#[derive(Debug, Clone)]
pub struct Properties {
    client_id: String,
    tenant_name: String,
    datastream: datastream::Datastream,
    certificates: Option<certificates::Cert>,
}

impl Properties {
    /// Create a new Properties struct that contains all information and certificates.
    /// needed to connect to Kafka and DSH.
    ///
    ///  - Contains a struct equal to datastreams.json
    ///  - Metadata of running container/task
    ///  - Certificates for Kafka and DSH
    ///
    /// If running locally, it will try to load the local_datastreams.json file
    /// and crate the properties struct based on this file
    ///
    /// # Error
    /// If running locally and local_datastreams.json file is not present in the root of the project
    ///
    /// # local_datastreams.json
    /// local_datastreams.json should be placed in the root of the project
    ///
    /// Example of local_datastreams.json.
    /// (important that read and write has correct topic names that are configured in your local kafka cluster)
    ///
    /// ```text
    /// {
    ///     "brokers": ["localhost:9092"],
    ///     "streams": {
    ///       "scratch.local": {
    ///         "name": "scratch.local",
    ///         "cluster": "/tt",
    ///         "read": "scratch.local.local-tenant",
    ///         "write": "scratch.local.local-tenant",
    ///         "partitions": 3,
    ///         "replication": 1,
    ///         "partitioner": "default-partitioner",
    ///         "partitioningDepth": 0,
    ///         "canRetain": false
    ///       },
    ///       "stream.test": {
    ///         "name": "scratch.dlq.local-tenant",
    ///         "cluster": "/tt",
    ///         "read": "scratch\\.dlq.\\[^.]*",
    ///         "write": "scratch.dlq.local-tenant",
    ///         "partitions": 1,
    ///         "replication": 1,
    ///         "partitioner": "default-partitioner",
    ///         "partitioningDepth": 0,
    ///         "canRetain": false
    ///       }
    ///     },
    ///     "private_consumer_groups": [
    ///       "local-app.7e93a513-6556-11eb-841e-f6ab8576620c_1",
    ///       "local-app.7e93a513-6556-11eb-841e-f6ab8576620c_2",
    ///       "local-app.7e93a513-6556-11eb-841e-f6ab8576620c_3",
    ///       "local-app.7e93a513-6556-11eb-841e-f6ab8576620c_4"
    ///     ],
    ///     "shared_consumer_groups": [
    ///       "local-app_1",
    ///       "local-app_2",
    ///       "local-app_3",
    ///       "local-app_4"
    ///     ],
    ///     "non_enveloped_streams": [],
    ///     "schema_store": "http://localhost:8081/apis/ccompat/v7"
    ///   }
    /// ```
    pub async fn new() -> Result<Self, DshError> {
        #[cfg(not(feature = "local"))]
        Self::new_dsh().await;
        #[cfg(feature = "local")]
        match Self::new_dsh().await {
            Ok(b) => Ok(b),
            Err(e) => {
                warn!("App does not seem to be running on DSH, due to: {}", e);
                warn!("Starting with local settings");
                Self::new_local()
            }
        }
    }

    pub fn new_blocking() -> Result<Self, DshError> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(Self::new())
    }

    /// Get default RDKafka Consumer config to connect to Kafka on DSH.
    /// If certificates are present, it will use SSL to connect to Kafka.
    /// If not, it will use plaintext so it can connect to local as well.
    ///
    /// Note: This config is set to auto commit to false. You need to manually commit offsets.
    /// You can overwrite this config by setting the enable.auto.commit and enable.auto.offset.store property to `true`.
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::rdkafka::config::RDKafkaLogLevel;
    /// use dsh_sdk::rdkafka::consumer::stream_consumer::StreamConsumer;
    /// use dsh_sdk::dsh::Properties;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let dsh_properties = Properties::new().await?;
    ///     let mut consumer_config = dsh_properties.consumer_rdkafka_config()?;
    ///     let consumer: StreamConsumer =  consumer_config.create().expect("Consumer creation failed");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Default configs
    /// See full list of configs properties in case you want to add/overwrite the config:
    /// <https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md>
    ///
    /// | **config**                | **Default value**                      | **Remark**                                                                                                                                                           |
    /// |---------------------------|----------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `bootstrap.servers`       | Brokers based on <br>datastreams.json  | Brokers from datastreams.json                                                                                                                                        |
    /// | `group.id`                | Group ID from <br>datastreams.json     | Set env variable KAFKA_CONSUMER_GROUP_TYPE to<br>"private" or "shared" to switch between group types.<br>Always selects first group id.<br>DEFAULT: private, if not set |
    /// | `client.id`               | task_id of service                     | Based on task_id of running service                                                                                                                                  |
    /// | `enable.auto.commit`      | false                                  | Autocommmit                                                                                                                                                          |
    /// | `enable.auto.offset.store`| false                                  | Store autocommit of last message provided                                                                                                                            |
    /// | `auto.offset.reset`       | earliest                               | Start consuming from the beginning.                                                                                                                                  |
    /// | `security.protocol`       | ssl (DSH)<br>plaintext (local)        | Security protocol                                                                                                                                                    |
    /// | `ssl.key.pem`             | private key                            | Generated when bootstrap is initiated                                                                                                                                |
    /// | `ssl.certificate.pem`     | dsh kafka certificate                  | Signed certificate to connect to kafka cluster <br>(signed when bootstrap is initiated)                                                                              |
    /// | `ssl.ca.pem`              | CA certifacte                          | Root certificate, provided by DSH.                                                                                                                                   |
    /// | `log_level`               | Info                                   | Log level of rdkafka                                                                                                                                                 |
    #[cfg(any(feature = "rdkafka-ssl", feature = "rdkafka-ssl-vendored"))]
    pub fn consumer_rdkafka_config(&self) -> Result<rdkafka::config::ClientConfig, DshError> {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.datastream().get_brokers())
            .set(
                "group.id",
                self.datastream()
                    .get_group_id(datastream::GroupType::from_env())?,
            )
            .set("client.id", self.client_id())
            .set("enable.auto.commit", "false")
            .set("enable.auto.offset.store", "false")
            .set("auto.offset.reset", "earliest")
            .set_log_level(rdkafka::config::RDKafkaLogLevel::Info);
        // Set SSL if certificates are present
        if let Some(certificates) = &self.certificates() {
            config
                .set("security.protocol", "ssl")
                .set("ssl.key.pem", certificates.private_key_pem())
                .set(
                    "ssl.certificate.pem",
                    certificates.dsh_kafka_certificate_pem(),
                )
                .set("ssl.ca.pem", certificates.dsh_ca_certificate_pem());
        } else {
            config.set("security.protocol", "plaintext");
        }
        Ok(config)
    }

    /// Get default RDKafka Producer config to connect to Kafka on DSH.
    /// If certificates are present, it will use SSL to connect to Kafka.
    /// If not, it will use plaintext so it can connect to local as well.
    ///
    /// Note: This config is set to auto commit to false. You need to manually commit offsets.
    /// You can overwrite this config by setting the enable.auto.commit and enable.auto.offset.store property to `true`.
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::rdkafka::config::RDKafkaLogLevel;
    /// use dsh_sdk::rdkafka::producer::FutureProducer;
    /// use dsh_sdk::dsh::Properties;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///     let dsh_properties = Properties::new().await?;
    ///     let mut producer_config = dsh_properties.producer_rdkafka_config();
    ///     let producer: FutureProducer =  producer_config.create().expect("Producer creation failed");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Default configs
    /// See full list of configs properties in case you want to add/overwrite the config:
    /// <https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md>
    ///
    /// | **config**          | **Default value**                      | **Remark**                                                                              |
    /// |---------------------|----------------------------------------|-----------------------------------------------------------------------------------------|
    /// | bootstrap.servers   | Brokers based on <br>datastreams.json  | Brokers from datastreams.json                                                           |
    /// | client.id           | task_id of service                     | Based on task_id of running service                                                     |
    /// | security.protocol   | ssl (DSH))<br>plaintext (local)        | Security protocol                                                                       |
    /// | ssl.key.pem         | private key                            | Generated when bootstrap is initiated                                                   |
    /// | ssl.certificate.pem | dsh kafka certificate                  | Signed certificate to connect to kafka cluster <br>(signed when bootstrap is initiated) |
    /// | ssl.ca.pem          | CA certifacte                          | Root certificate, provided by DSH.                                                      |
    /// | log_level           | Info                                   | Log level of rdkafka                                                                    |
    #[cfg(any(feature = "rdkafka-ssl", feature = "rdkafka-ssl-vendored"))]
    pub fn producer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.datastream().get_brokers())
            .set("client.id", self.client_id())
            .set_log_level(rdkafka::config::RDKafkaLogLevel::Info);

        // Set SSL if certificates are present
        if let Some(certificates) = &self.certificates() {
            config
                .set("security.protocol", "ssl")
                .set("ssl.key.pem", certificates.private_key_pem())
                .set(
                    "ssl.certificate.pem",
                    certificates.dsh_kafka_certificate_pem(),
                )
                .set("ssl.ca.pem", certificates.dsh_ca_certificate_pem());
        } else {
            config.set("security.protocol", "plaintext");
        }
        config
    }

    /// Get reqwest client config to connect to DSH Schema Registry.
    /// If certificates are present, it will use SSL to connect to Schema Registry.
    ///
    /// Use <https://crates.io/crates/schema_registry_converter> to connect to Schema Registry.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::dsh::Properties;
    /// # use reqwest::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let dsh_properties = Properties::new().await?;
    ///     let client = dsh_properties.reqwest_client_config()?.build()?;
    ///
    /// #    Ok(())
    /// # }
    /// ```
    pub fn reqwest_client_config(&self) -> Result<reqwest::ClientBuilder, DshError> {
        let mut client_builder = reqwest::Client::builder();
        if let Some(certificates) = &self.certificates() {
            client_builder = certificates.reqwest_client_config()?;
        }
        Ok(client_builder)
    }

    /// Get the certificates. If running local it returns None
    pub fn certificates(&self) -> Option<certificates::Cert> {
        self.certificates.clone()
    }

    /// Get the client id based on the task id.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get the tenant name.
    pub fn tenant_name(&self) -> &str {
        &self.tenant_name
    }

    /// Get the kafka properties provided by DSH (datastreams.json)
    pub fn datastream(&self) -> &datastream::Datastream {
        &self.datastream
    }

    /// Get schema host of DSH.
    ///
    /// Overwritable with environment variable SCHEMA_REGISTRY_HOST, if set
    pub fn schema_registry_host(&self) -> &str {
        self.datastream().schema_store()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_configured_topics() {
        std::env::set_var("TOPICS", "topic1, topic2, topic3");

        let topics = get_configured_topics().unwrap();

        assert_eq!(topics.len(), 3);
        assert_eq!(topics[0], "topic1");
        assert_eq!(topics[1], "topic2");
        assert_eq!(topics[2], "topic3");

        std::env::remove_var("TOPICS");

        let topics = get_configured_topics();
        assert!(topics.is_err());
    }
}
