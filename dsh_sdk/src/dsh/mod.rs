//! # Kafka Properties
//!
//! This module contains logic to connect to Kafka on DSH and get properties of your tenant.
//! For example all available streams and topics.
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
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let dsh_properties = Properties::get();
//! let consumer_config = dsh_properties.consumer_rdkafka_config()?;
//! let consumer: StreamConsumer = consumer_config.create()?;
//!
//! # Ok(())
//! # }
//! ```
use log::{info, warn};
use std::env;
use std::sync::OnceLock;

use crate::error::DshError;

pub mod bootstrap;
pub mod certificates;
pub mod datastream;

static PROPERTIES: OnceLock<Properties> = OnceLock::new();

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
///     let dsh_properties = Properties::get();
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
    task_id: String,
    tenant_name: String,
    datastream: datastream::Datastream,
    certificates: Option<certificates::Cert>,
}

impl Properties {
    /// Get the DSH Properties on a lazy way. If not already initialized, it will initialize the properties
    /// and bootstrap to DSH.
    ///
    /// This struct contains all configuration and certificates needed to connect to Kafka and DSH.
    ///
    ///  - Contains a struct equal to datastreams.json
    ///  - Metadata of running container/task
    ///  - Certificates for Kafka and DSH
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::dsh::Properties;
    /// use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh_properties = Properties::get();
    /// let consumer: StreamConsumer = dsh_properties.consumer_rdkafka_config()?.create()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required environment variables
    /// The following environment variables are required to be set. If not set, it will default to local settings.
    /// When starting a container in DSH, these variable are automatically set.
    ///
    /// - `MESOS_TASK_ID` - The task id of the running container
    /// - `MARATHON_APP_ID` - Includes the tenant name of the running container
    /// - `DSH_SECRET_TOKEN` - The secret token to authenticate to DSH
    /// - `DSH_CA_CERTIFICATE` - The CA certificate of DSH
    ///
    /// ### Optional
    /// - `DSH_SECRET_TOKEN_PATH` - The path to the secret token file. (useful when running in system space)
    ///
    /// # Running on local machine
    /// When running on a local machine, it can connect to a local Kafka cluster and Schema Registry. By
    /// default it connects `localhost:9092` for kafka and `localhost:8081/apis/ccompat/v7` for the schema
    /// registry. If you want to connect to a different Kafka cluster, or manipulate the datastream configuration,
    /// you can create a [local_datastreams.json](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/local_datastreams.json)
    /// file in the root of the project.

    pub fn get() -> &'static Self {
        PROPERTIES.get_or_init(Self::init)
    }

    /// Initialize the properties and bootstrap to DSH
    fn init() -> Self {
        match Self::new_dsh() {
            Ok(properties) => {
                info!("Successfully connected to DSH");
                properties
            }
            Err(e) => {
                warn!("DSH_SDK was not able to connect to DSH, due to: {}", e);
                warn!("Using local configuration instead");
                Properties::default()
            }
        }
    }

    /// Get default RDKafka Consumer config to connect to Kafka on DSH.
    ///
    /// Note: This config is set to auto commit to false. You need to manually commit offsets.
    /// You can overwrite this config by setting the enable.auto.commit and enable.auto.offset.store property to `true`.
    ///
    /// # Group ID
    /// There are 2 types of group id's in DSH: private and shared. Private will have a unique group id per running instance.
    /// Shared will have the same group id for all running instances. With this you can horizontally scale your service.
    /// The group type can be manipulated by environment variable KAFKA_CONSUMER_GROUP_TYPE.
    /// If not set, it will default to private.
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::rdkafka::config::RDKafkaLogLevel;
    /// use dsh_sdk::rdkafka::consumer::stream_consumer::StreamConsumer;
    /// use dsh_sdk::dsh::Properties;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let dsh_properties = Properties::get();
    ///     let mut consumer_config = dsh_properties.consumer_rdkafka_config()?;
    ///     let consumer: StreamConsumer =  consumer_config.create()?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Default configs
    /// See full list of configs properties in case you want to add/overwrite the config:
    /// <https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md>
    ///
    /// | **config**                | **Default value**                      | **Remark**                                                                                                                            |
    /// |---------------------------|----------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
    /// | `bootstrap.servers`       | Brokers based on <br>datastreams.json  | Brokers from datastreams.json                                                                                                         |
    /// | `group.id`                | Group ID from <br>datastreams.json     | Set env variable KAFKA_CONSUMER_GROUP_TYPE to<br>"private" or "shared" to switch between group types.<br>DEFAULT: private, if not set |
    /// | `client.id`               | task_id of service                     | Based on task_id of running service                                                                                                   |
    /// | `enable.auto.commit`      | false                                  | Autocommmit                                                                                                                           |
    /// | `enable.auto.offset.store`| false                                  | Store autocommit of last message provided                                                                                             |
    /// | `auto.offset.reset`       | earliest                               | Start consuming from the beginning.                                                                                                   |
    /// | `security.protocol`       | ssl (DSH)<br>plaintext (local)         | Security protocol                                                                                                                     |
    /// | `ssl.key.pem`             | private key                            | Generated when bootstrap is initiated                                                                                                 |
    /// | `ssl.certificate.pem`     | dsh kafka certificate                  | Signed certificate to connect to kafka cluster <br>(signed when bootstrap is initiated)                                               |
    /// | `ssl.ca.pem`              | CA certifacte                          | Root certificate, provided by DSH.                                                                                                    |
    /// | `log_level`               | Info                                   | Log level of rdkafka                                                                                                                  |
    #[cfg(any(feature = "rdkafka-ssl", feature = "rdkafka-ssl-vendored"))]
    pub fn consumer_rdkafka_config(&self) -> Result<rdkafka::config::ClientConfig, DshError> {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.datastream().get_brokers_string())
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
        if let Ok(certificates) = &self.certificates() {
            config
                .set("security.protocol", "ssl")
                .set("ssl.key.pem", certificates.private_key_pem()?)
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
    ///     let dsh_properties = Properties::get();
    ///     let mut producer_config = dsh_properties.producer_rdkafka_config().expect("Producer config creation failed");
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
    pub fn producer_rdkafka_config(&self) -> Result<rdkafka::config::ClientConfig, DshError> {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.datastream().get_brokers_string())
            .set("client.id", self.client_id())
            .set_log_level(rdkafka::config::RDKafkaLogLevel::Info);

        // Set SSL if certificates are present
        if let Ok(certificates) = self.certificates() {
            config
                .set("security.protocol", "ssl")
                .set("ssl.key.pem", certificates.private_key_pem()?)
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

    /// Get reqwest async client config to connect to DSH Schema Registry.
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
    ///     let dsh_properties = Properties::get();
    ///     let client = dsh_properties.reqwest_client_config()?.build()?;
    ///
    /// #    Ok(())
    /// # }
    /// ```
    pub fn reqwest_client_config(&self) -> Result<reqwest::ClientBuilder, DshError> {
        let mut client_builder = reqwest::Client::builder();
        if let Ok(certificates) = &self.certificates() {
            client_builder = certificates.reqwest_client_config()?;
        }
        Ok(client_builder)
    }

    /// Get the certificates. If running local it returns None
    pub fn certificates(&self) -> Result<&certificates::Cert, DshError> {
        if let Some(cert) = &self.certificates {
            Ok(cert)
        } else {
            Err(DshError::NoCertificates)
        }
    }

    /// Get the client id based on the task id.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get the tenant name of running container.
    pub fn tenant_name(&self) -> &str {
        &self.tenant_name
    }

    /// Get the task id of running container.
    pub fn task_id(&self) -> &str {
        &self.task_id
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

impl Default for Properties {
    fn default() -> Self {
        let datastream = datastream::Datastream::load_local_datastreams().unwrap_or_default();
        Self {
            client_id: "local".to_string(),
            task_id: "local_task_id".to_string(),
            tenant_name: "local_tenant".to_string(),
            datastream,
            certificates: None,
        }
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

    #[test]
    fn test_get_or_init() {
        let properties = Properties::get();
        assert_eq!(properties.client_id(), "local");
        assert_eq!(properties.task_id(), "local_task_id");
        assert_eq!(properties.tenant_name(), "local_tenant");
    }

    #[test]
    fn test_consumer_rdkafka_config() {
        let properties = Properties::default();
        let config = properties.consumer_rdkafka_config();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(
            config.get("bootstrap.servers").unwrap(),
            properties.datastream().get_brokers_string()
        );
        assert_eq!(
            config.get("group.id").unwrap(),
            properties
                .datastream()
                .get_group_id(datastream::GroupType::from_env())
                .unwrap()
        );
        assert_eq!(config.get("client.id").unwrap(), properties.client_id());
        assert_eq!(config.get("enable.auto.commit").unwrap(), "false");
        assert_eq!(config.get("enable.auto.offset.store").unwrap(), "false");
        assert_eq!(config.get("auto.offset.reset").unwrap(), "earliest");
    }

    #[test]
    fn test_producer_rdkafka_config() {
        let properties = Properties::default();
        let config = properties.producer_rdkafka_config();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(
            config.get("bootstrap.servers").unwrap(),
            properties.datastream().get_brokers_string()
        );
        assert_eq!(config.get("client.id").unwrap(), properties.client_id());
    }

    #[test]
    fn test_reqwest_client_config() {
        let properties = Properties::default();
        let config = properties.reqwest_client_config();
        assert!(config.is_ok());
    }

    #[test]
    fn test_client_id() {
        let properties = Properties::default();
        assert_eq!(properties.client_id(), "local");
    }

    #[test]
    fn test_tenant_name() {
        let properties = Properties::default();
        assert_eq!(properties.tenant_name(), "local_tenant");
    }

    #[test]
    fn test_task_id() {
        let properties = Properties::default();
        assert_eq!(properties.task_id(), "local_task_id");
    }

    #[test]
    fn test_schema_registry_host() {
        let properties = Properties::default();
        assert_eq!(
            properties.schema_registry_host(),
            "http://localhost:8081/apis/ccompat/v7"
        );
    }
}
