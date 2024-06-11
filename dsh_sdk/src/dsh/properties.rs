//! # Properties
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
use log::{debug, error, info, warn};
use std::env;
use std::sync::OnceLock;

use super::bootstrap::bootstrap;
use super::{
    certificates, datastream, pki_config_dir, utils, VAR_APP_ID, VAR_DSH_TENANT_NAME,
    VAR_KAFKA_AUTO_OFFSET_RESET, VAR_KAFKA_ENABLE_AUTO_COMMIT, VAR_KAFKA_GROUP_ID, VAR_TASK_ID,
};
use crate::error::DshError;

static PROPERTIES: OnceLock<Properties> = OnceLock::new();

/// DSH properties struct. Create new to initialize all related components to connect to the DSH kafka clusters
///  - Contains info from datastreams.json
///  - Metadata of running container/task
///  - Certificates for Kafka and DSH Schema Registry
///
/// # Example
/// ```
/// use dsh_sdk::Properties;
/// use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let dsh_properties = Properties::get();
///     
///     let consumer_config = dsh_properties.consumer_rdkafka_config();
///     let consumer: StreamConsumer = consumer_config.create()?;
///
///     Ok(())
/// }
/// ```

#[derive(Debug, Clone)]
pub struct Properties {
    task_id: String,
    tenant_name: String,
    datastream: datastream::Datastream,
    certificates: Option<certificates::Cert>,
}

impl Properties {
    /// New `Properties` struct
    pub(crate) fn new(
        task_id: String,
        tenant_name: String,
        datastream: datastream::Datastream,
        certificates: Option<certificates::Cert>,
    ) -> Self {
        Self {
            task_id,
            tenant_name,
            datastream,
            certificates,
        }
    }
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
    /// use dsh_sdk::Properties;
    /// use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh_properties = Properties::get();
    /// let consumer: StreamConsumer = dsh_properties.consumer_rdkafka_config().create()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # How to run
    /// The SDK is compatible with running in a container on a DSH tenant, on DSH System Space, on a machine with Kafka
    /// Proxy/VPN or on a local machine to a local Kafka(for development purposes).
    ///
    /// ## DSH
    /// The following environment variables are required to run on DSH, and are set by DSH automatically:
    /// - `MESOS_TASK_ID` - The task id of the running container
    /// - `MARATHON_APP_ID` - Includes the tenant name of the running container
    /// - `DSH_CA_CERTIFICATE` - The CA certificate of DSH
    /// - `DSH_SECRET_TOKEN` - The secret token to authenticate to DSH
    ///
    /// ### System Space
    /// - `DSH_SECRET_TOKEN_PATH` - The path to the secret token file.
    ///
    /// ## Kafka Proxy/VPN
    /// When running on a machine with Kafka Proxy/VPN, the following environment variables are required:
    /// - 'PKI_CONFIG_DIR' - The path to the directory containing the certificates and private key
    /// - `DSH_TENANT_NAME` - The tenant name of which you want to connect to
    /// - 'KAFKA_BOOTSTRAP_SERVERS' - The hostnames of the Kafka brokers
    ///
    /// ### Note!
    /// Currently only PEM formatted certificates and keys are supported.
    ///
    /// ## Local
    /// When no environment variables are set, it will default to a local configuration.
    /// - Kafka will be set to `localhost:9092` and uses plaintext instead of SSL
    /// - Schema Registry will be set to `localhost:8081/apis/ccompat/v7`
    /// You can overwrite this by providing a [local_datastreams.json](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/local_datastreams.json)
    /// file in the root of the project or by setting the following environment variables.
    /// - `KAFKA_BOOTSTRAP_SERVERS` - The hostnames of the Kafka brokers
    /// - `SCHEMA_REGISTRY_HOST` - The host of the Schema Registryq

    pub fn get() -> &'static Self {
        PROPERTIES.get_or_init(Self::init)
    }

    /// Initialize the properties and bootstrap to DSH
    fn init() -> Self {
        let tenant_name = match utils::tenant_name() {
            Ok(tenant_name) => tenant_name,
            Err(_) => {
                error!("{} and {} are not set, this may cause unexpected behaviour when connecting to DSH Kafka cluster!. Please set one of these environment variables.", VAR_APP_ID, VAR_DSH_TENANT_NAME);
                "local_tenant".to_string()
            }
        };
        let task_id = utils::get_env_var(VAR_TASK_ID).unwrap_or("local_task_id".to_string());
        let (certificates, datastream) = if let Ok(cert) = pki_config_dir::get_pki_cert() {
            info!("Successfully loaded certificates from PKI config directory");
            (
                Some(cert),
                datastream::Datastream::load_local_datastreams().unwrap_or_default(),
            )
        } else {
            match bootstrap(&tenant_name, &task_id) {
                Ok((cert, datastream)) => {
                    info!("Successfully connected to DSH");
                    (Some(cert), datastream)
                }
                Err(e) => {
                    warn!("DSH_SDK was not able to connect to DSH, due to: {}", e);
                    warn!("Using local configuration instead");
                    (
                        None,
                        datastream::Datastream::load_local_datastreams().unwrap_or_default(),
                    )
                }
            }
        };
        Self::new(task_id, tenant_name, datastream, certificates)
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
    /// If not set, it will default to shared.
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::Properties;
    /// use dsh_sdk::rdkafka::config::RDKafkaLogLevel;
    /// use dsh_sdk::rdkafka::consumer::stream_consumer::StreamConsumer;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let dsh_properties = Properties::get();
    ///     let mut consumer_config = dsh_properties.consumer_rdkafka_config();
    ///     let consumer: StreamConsumer =  consumer_config.create()?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Default configs
    /// See full list of configs properties in case you want to add/overwrite the config:
    /// <https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md>
    ///
    /// Some configurations are overwitable by environment variables.
    ///
    /// | **config**                | **Default value**                | **Remark**                                            |
    /// |---------------------------|----------------------------------|-------------------------------------------------------|
    /// | `bootstrap.servers`       | Brokers based on datastreams     | Overwritable by env variable KAFKA_BOOTSTRAP_SERVERS` |
    /// | `group.id`                | Shared Group ID from datastreams | Overwritable by setting `KAFKA_GROUP_ID` or `KAFKA_CONSUMER_GROUP_TYPE`|
    /// | `client.id`               | Task_id of service               | |
    /// | `enable.auto.commit`      | `false`                          | Overwritable by setting `KAFKA_ENABLE_AUTO_COMMIT` |
    /// | `auto.offset.reset`       | `earliest`                       | Overwritable by setting `KAFKA_AUTO_OFFSET_RESET`    |
    /// | `security.protocol`       | ssl (DSH) / plaintext (local)    | Security protocol                               |
    /// | `ssl.key.pem`             | private key                      | Generated when bootstrap is initiated           |
    /// | `ssl.certificate.pem`     | dsh kafka certificate            | Signed certificate to connect to kafka cluster  |
    /// | `ssl.ca.pem`              | CA certifacte                    | CA certificate, provided by DSH.                |
    ///
    /// # Environment variables
    /// To manipulate the configuration during runtume, you can set the following environment variables.
    ///
    /// ### `KAFKA_BOOTSTRAP_SERVERS`
    /// - Usage: Overwrite hostnames of brokers (useful for local testing)
    /// - Default: Brokers based on datastreams
    /// - Required: `false`
    ///
    /// ### `KAFKA_CONSUMER_GROUP_TYPE`
    /// - Usage: Picks group_id based on type from datastreams
    /// - Default: Shared
    /// - Options: private, shared
    /// - Required: `false`
    ///
    /// ### `KAFKA_GROUP_ID`
    /// - Usage: Custom group id
    /// - Default: NA
    /// - Required: `false`
    /// - Remark: Overrules `KAFKA_CONSUMER_GROUP_TYPE`. Mandatory to start with tenant name. (will prefix tenant name automatically if not set)
    ///
    /// ### `KAFKA_ENABLE_AUTO_COMMIT`
    /// - Usage: Enable/Disable auto commit
    /// - Default: `false`
    /// - Required: `false`
    /// - Options: `true`, `false`
    ///
    /// ### `KAFKA_AUTO_OFFSET_RESET`
    /// - Usage: Set the offset reset settings to start consuming from set option.
    /// - Default: earliest
    /// - Required: `false`
    /// - Options: smallest, earliest, beginning, largest, latest, end
    #[cfg(any(feature = "rdkafka-ssl", feature = "rdkafka-ssl-vendored"))]
    pub fn consumer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.kafka_brokers())
            .set("group.id", self.kafka_group_id())
            .set("client.id", self.client_id())
            .set("enable.auto.commit", self.kafka_auto_commit().to_string())
            .set("auto.offset.reset", self.kafka_auto_offset_reset());
        debug!("Consumer config: {:#?}", config);
        // Set SSL if certificates are present
        if let Ok(certificates) = &self.certificates() {
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

    /// Get default RDKafka Producer config to connect to Kafka on DSH.
    /// If certificates are present, it will use SSL to connect to Kafka.
    /// If not, it will use plaintext so it can connect to local as well.
    ///
    /// Note: The default config is set to auto commit to false. You need to manually commit offsets.
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::rdkafka::config::RDKafkaLogLevel;
    /// use dsh_sdk::rdkafka::producer::FutureProducer;
    /// use dsh_sdk::Properties;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///     let dsh_properties = Properties::get();
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
    /// | **config**          | **Default value**              | **Remark**                                                                              |
    /// |---------------------|--------------------------------|-----------------------------------------------------------------------------------------|
    /// | bootstrap.servers   | Brokers based on datastreams   | Overwritable by env variable `KAFKA_BOOTSTRAP_SERVERS`                                  |
    /// | client.id           | task_id of service             | Based on task_id of running service                                                     |
    /// | security.protocol   | ssl (DSH)) / plaintext (local) | Security protocol                                                                       |
    /// | ssl.key.pem         | private key                    | Generated when bootstrap is initiated                                                   |
    /// | ssl.certificate.pem | dsh kafka certificate          | Signed certificate to connect to kafka cluster <br>(signed when bootstrap is initiated) |
    /// | ssl.ca.pem          | CA certifacte                  | CA certificate, provided by DSH.                                                        |
    /// | log_level           | Info                           | Log level of rdkafka                                                                    |
    ///
    /// # Environment variables
    /// To manipulate the configuration during runtume, you can set the following environment variables.
    ///
    /// ### `KAFKA_BOOTSTRAP_SERVERS`
    /// - Usage: Overwrite hostnames of brokers (useful for local testing)
    /// - Default: Brokers based on datastreams
    /// - Required: `false`
    #[cfg(any(feature = "rdkafka-ssl", feature = "rdkafka-ssl-vendored"))]
    pub fn producer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.kafka_brokers())
            .set("client.id", self.client_id());
        debug!("Producer config: {:#?}", config);
        // Set SSL if certificates are present
        if let Ok(certificates) = self.certificates() {
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

    /// Get reqwest async client config to connect to DSH Schema Registry.
    /// If certificates are present, it will use SSL to connect to Schema Registry.
    ///
    /// Use [schema_registry_converter](https://crates.io/crates/schema_registry_converter) to connect to Schema Registry.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Properties;
    /// # use reqwest::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh_properties = Properties::get();
    /// let client = dsh_properties.reqwest_client_config()?.build()?;
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

    /// Get the certificates and private key. Returns an error when running on local machine.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::Properties;
    /// # use dsh_sdk::error::DshError;
    /// # fn main() -> Result<(), DshError> {
    /// let dsh_properties = Properties::get();
    /// let dsh_kafka_certificate = dsh_properties.certificates()?.dsh_kafka_certificate_pem();
    /// #    Ok(())
    /// # }
    pub fn certificates(&self) -> Result<&certificates::Cert, DshError> {
        if let Some(cert) = &self.certificates {
            Ok(cert)
        } else {
            Err(DshError::NoCertificates)
        }
    }

    /// Get the client id based on the task id.
    pub fn client_id(&self) -> &str {
        &self.task_id
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
    pub fn schema_registry_host(&self) -> &str {
        self.datastream().schema_store()
    }

    /// Get the Kafka brokers.
    ///
    /// ## Environment variables
    /// To manipulate the hastnames of the brokers, you can set the following environment variables.
    ///
    /// ### `KAFKA_BOOTSTRAP_SERVERS`
    /// - Usage: Overwrite hostnames of brokers
    /// - Default: Brokers based on datastreams
    /// - Required: `false`
    pub fn kafka_brokers(&self) -> String {
        self.datastream().get_brokers_string()
    }

    /// Get the kafka_group_id based.
    ///
    /// ## Environment variables
    /// To manipulate the group id, you can set the following environment variables.
    ///  
    /// ### `KAFKA_CONSUMER_GROUP_TYPE`
    /// - Usage: Picks group_id based on type from datastreams
    /// - Default: Shared
    /// - Options: private, shared
    /// - Required: `false`
    ///
    /// ### `KAFKA_GROUP_ID`
    /// - Usage: Custom group id
    /// - Default: NA
    /// - Required: `false`
    /// - Remark: Overrules `KAFKA_CONSUMER_GROUP_TYPE`. Mandatory to start with tenant name. (will prefix tenant name automatically if not set)
    pub fn kafka_group_id(&self) -> String {
        if let Ok(group_id) = env::var(VAR_KAFKA_GROUP_ID) {
            if !group_id.starts_with(self.tenant_name()) {
                format!("{}_{}", self.tenant_name(), group_id)
            } else {
                group_id
            }
        } else {
            self.datastream()
                .get_group_id(datastream::GroupType::from_env())
                .unwrap_or(&format!("{}_CONSUMER", self.tenant_name()))
                .to_string()
        }
    }

    /// Get the confifured kafka auto commit setinngs.
    ///
    /// ## Environment variables
    /// To manipulate the auto commit settings, you can set the following environment variables.
    ///
    /// ### `KAFKA_ENABLE_AUTO_COMMIT`
    /// - Usage: Enable/Disable auto commit
    /// - Default: `false`
    /// - Required: `false`
    /// - Options: `true`, `false`
    pub fn kafka_auto_commit(&self) -> bool {
        env::var(VAR_KAFKA_ENABLE_AUTO_COMMIT)
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false)
    }

    /// Get the kafka auto offset reset settings.
    ///
    /// ## Environment variables
    /// To manipulate the auto offset reset settings, you can set the following environment variables.
    ///
    /// ### `KAFKA_AUTO_OFFSET_RESET`
    /// - Usage: Set the offset reset settings to start consuming from set option.
    /// - Default: earliest
    /// - Required: `false`
    /// - Options: smallest, earliest, beginning, largest, latest, end
    pub fn kafka_auto_offset_reset(&self) -> String {
        env::var(VAR_KAFKA_AUTO_OFFSET_RESET).unwrap_or_else(|_| "earliest".to_string())
    }
}

impl Default for Properties {
    fn default() -> Self {
        let datastream = datastream::Datastream::load_local_datastreams().unwrap_or_default();
        Self {
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
    use crate::dsh::{VAR_KAFKA_BOOTSTRAP_SERVERS, VAR_KAFKA_CONSUMER_GROUP_TYPE};
    use serial_test::serial;

    #[test]
    fn test_get_or_init() {
        let properties = Properties::get();
        assert_eq!(properties.client_id(), "local_task_id");
        assert_eq!(properties.task_id(), "local_task_id");
        assert_eq!(properties.tenant_name(), "local_tenant");
    }

    #[test]
    #[serial(env_depencency)]
    fn test_consumer_rdkafka_config() {
        let properties = Properties::default();
        let config = properties.consumer_rdkafka_config();
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
        assert_eq!(config.get("auto.offset.reset").unwrap(), "earliest");
    }

    #[test]
    #[serial(env_depencency)]
    fn test_producer_rdkafka_config() {
        let properties = Properties::default();
        let config = properties.producer_rdkafka_config();
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
        assert_eq!(properties.client_id(), "local_task_id");
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

    #[test]
    #[serial(env_depencency)]
    fn test_kafka_brokers() {
        let properties = Properties::default();
        assert_eq!(
            properties.kafka_brokers(),
            properties.datastream().get_brokers_string()
        );
        env::set_var(VAR_KAFKA_BOOTSTRAP_SERVERS, "test:9092");
        let properties = Properties::default();
        assert_eq!(properties.kafka_brokers(), "test:9092");
        env::remove_var(VAR_KAFKA_BOOTSTRAP_SERVERS);
    }

    #[test]
    #[serial(env_depencency)]
    fn test_kafka_group_id() {
        let properties = Properties::default();
        assert_eq!(
            properties.kafka_group_id(),
            properties
                .datastream()
                .get_group_id(datastream::GroupType::Shared(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "private");
        assert_eq!(
            properties.kafka_group_id(),
            properties
                .datastream()
                .get_group_id(datastream::GroupType::Private(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "shared");
        assert_eq!(
            properties.kafka_group_id(),
            properties
                .datastream()
                .get_group_id(datastream::GroupType::Shared(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_GROUP_ID, "test_group");
        assert_eq!(
            properties.kafka_group_id(),
            format!("{}_test_group", properties.tenant_name())
        );
        env::set_var(
            VAR_KAFKA_GROUP_ID,
            format!("{}_test_group", properties.tenant_name()),
        );
        assert_eq!(
            properties.kafka_group_id(),
            format!("{}_test_group", properties.tenant_name())
        );
        env::remove_var(VAR_KAFKA_CONSUMER_GROUP_TYPE);
        assert_eq!(
            properties.kafka_group_id(),
            format!("{}_test_group", properties.tenant_name())
        );
        env::remove_var(VAR_KAFKA_GROUP_ID);
    }

    #[test]
    #[serial(env_depencency)]
    fn test_kafka_auto_commit() {
        let properties = Properties::default();
        assert_eq!(properties.kafka_auto_commit(), false);
        env::set_var(VAR_KAFKA_ENABLE_AUTO_COMMIT, "false");
        assert_eq!(properties.kafka_auto_commit(), false);
        env::set_var(VAR_KAFKA_ENABLE_AUTO_COMMIT, "true");
        assert_eq!(properties.kafka_auto_commit(), true);
        env::set_var(VAR_KAFKA_ENABLE_AUTO_COMMIT, "X");
        assert_eq!(properties.kafka_auto_commit(), false);
        env::remove_var(VAR_KAFKA_ENABLE_AUTO_COMMIT);
    }

    #[test]
    #[serial(env_depencency)]
    fn test_kafka_auto_offset_reset() {
        let properties = Properties::default();
        assert_eq!(properties.kafka_auto_offset_reset(), "earliest");
        env::set_var(VAR_KAFKA_AUTO_OFFSET_RESET, "smallest");
        assert_eq!(properties.kafka_auto_offset_reset(), "smallest");
        env::set_var(VAR_KAFKA_AUTO_OFFSET_RESET, "earliest");
        assert_eq!(properties.kafka_auto_offset_reset(), "earliest");
        env::set_var(VAR_KAFKA_AUTO_OFFSET_RESET, "beginning");
        assert_eq!(properties.kafka_auto_offset_reset(), "beginning");
        env::set_var(VAR_KAFKA_AUTO_OFFSET_RESET, "largest");
        assert_eq!(properties.kafka_auto_offset_reset(), "largest");
        env::set_var(VAR_KAFKA_AUTO_OFFSET_RESET, "latest");
        assert_eq!(properties.kafka_auto_offset_reset(), "latest");
        env::set_var(VAR_KAFKA_AUTO_OFFSET_RESET, "end");
        assert_eq!(properties.kafka_auto_offset_reset(), "end");
        env::remove_var(VAR_KAFKA_AUTO_OFFSET_RESET);
    }
}
