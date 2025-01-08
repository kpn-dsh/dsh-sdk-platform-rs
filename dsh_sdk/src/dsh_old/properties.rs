//! # Properties
//!
//! This module contains logic to connect to Kafka on DSH and retreive all properties of your tenant.
//!
//! From `Properties` there are level functions to get the correct config to connect to Kafka and schema store.
//! For more low level functions, see
//!     - [datastream](datastream/index.html) module.
//!     - [certificates](certificates/index.html) module.
//!
//! ## Environment variables
//! See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for
//! more information configuring the consmer or producer via environment variables.
//!
//! # Example
//! ```
//! use dsh_sdk::Properties;
//! use rdkafka::consumer::{Consumer, StreamConsumer};
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
use log::{debug, error, warn};
use std::env;
use std::sync::OnceLock;

use super::bootstrap::bootstrap;
use super::error::DshError;
use super::{certificates, config, datastream, pki_config_dir};
use crate::utils;
use crate::*;
static PROPERTIES: OnceLock<Properties> = OnceLock::new();
static CONSUMER_CONFIG: OnceLock<config::ConsumerConfig> = OnceLock::new();
static PRODUCER_CONFIG: OnceLock<config::ProducerConfig> = OnceLock::new();

/// DSH properties struct. Create new to initialize all related components to connect to the DSH kafka clusters
///  - Contains info from datastreams.json
///  - Metadata of running container/task
///  - Certificates for Kafka and DSH Schema Registry
///
/// ## Environment variables
/// See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for
/// more information configuring the consmer or producer via environment variables.
///
/// # Example
/// ```
/// use dsh_sdk::Properties;
/// use rdkafka::consumer::{Consumer, StreamConsumer};
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
    config_host: String,
    task_id: String,
    tenant_name: String,
    datastream: datastream::Datastream,
    certificates: Option<certificates::Cert>,
}

impl Properties {
    /// New `Properties` struct
    pub(crate) fn new(
        config_host: String,
        task_id: String,
        tenant_name: String,
        datastream: datastream::Datastream,
        certificates: Option<certificates::Cert>,
    ) -> Self {
        Self {
            config_host,
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
    /// # Panics
    /// This method can panic when running on local machine and tries to load incorrect [local_datastream.json](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/local_datastreams.json).
    /// When no file is available in root or path on env variable `LOCAL_DATASTREAMS_JSON` is not set, it will
    /// return a default datastream struct and NOT panic.
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::Properties;
    /// use rdkafka::consumer::{Consumer, StreamConsumer};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh_properties = Properties::get();
    /// let consumer: StreamConsumer = dsh_properties.consumer_rdkafka_config().create()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get() -> &'static Self {
        PROPERTIES.get_or_init(|| tokio::task::block_in_place(Self::init))
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
        let config_host =
            utils::get_env_var(VAR_KAFKA_CONFIG_HOST).map(|host| format!("https://{}", host));
        let certificates = if let Ok(cert) = pki_config_dir::get_pki_cert() {
            Some(cert)
        } else if let Ok(config_host) = &config_host {
            bootstrap(config_host, &tenant_name, &task_id)
                .inspect_err(|e| {
                    warn!("Could not bootstrap to DSH, due to: {}", e);
                })
                .ok()
        } else {
            warn!("Could not bootstrap to DSH, as it does not seem to be running on DSH due to missing enivironment variables");
            None
        };
        let config_host = config_host.unwrap_or(DEFAULT_CONFIG_HOST.to_string()); // Default is for running on local machine with VPN
        let fetched_datastreams = certificates.as_ref().and_then(|cert| {
            cert.reqwest_blocking_client_config()
                .ok()
                .and_then(|cb| cb.build().ok())
                .and_then(|client| {
                    datastream::Datastream::fetch_blocking(
                        &client,
                        &config_host,
                        &tenant_name,
                        &task_id,
                    )
                    .ok()
                })
        });
        let datastream = if let Some(datastream) = fetched_datastreams {
            datastream
        } else {
            warn!("Could not fetch datastreams.json, using local or default datastreams");
            datastream::Datastream::load_local_datastreams().unwrap_or_default()
        };
        Self::new(config_host, task_id, tenant_name, datastream, certificates)
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
    /// use rdkafka::config::RDKafkaLogLevel;
    /// use rdkafka::consumer::stream_consumer::StreamConsumer;
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
    /// | **config**                | **Default value**                | **Remark**                                                             |
    /// |---------------------------|----------------------------------|------------------------------------------------------------------------|
    /// | `bootstrap.servers`       | Brokers based on datastreams     | Overwritable by env variable KAFKA_BOOTSTRAP_SERVERS`                  |
    /// | `group.id`                | Shared Group ID from datastreams | Overwritable by setting `KAFKA_GROUP_ID` or `KAFKA_CONSUMER_GROUP_TYPE`|
    /// | `client.id`               | Task_id of service               |                                                                        |
    /// | `enable.auto.commit`      | `false`                          | Overwritable by setting `KAFKA_ENABLE_AUTO_COMMIT`                     |
    /// | `auto.offset.reset`       | `earliest`                       | Overwritable by setting `KAFKA_AUTO_OFFSET_RESET`                      |
    /// | `security.protocol`       | ssl (DSH) / plaintext (local)    | Security protocol                                                      |
    /// | `ssl.key.pem`             | private key                      | Generated when bootstrap is initiated                                  |
    /// | `ssl.certificate.pem`     | dsh kafka certificate            | Signed certificate to connect to kafka cluster                         |
    /// | `ssl.ca.pem`              | CA certifacte                    | CA certificate, provided by DSH.                                       |
    ///
    /// ## Environment variables
    /// See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for more information
    /// configuring the consmer via environment variables.
    #[cfg(feature = "rdkafka-config")]
    pub fn consumer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        let consumer_config = CONSUMER_CONFIG.get_or_init(config::ConsumerConfig::new);
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.kafka_brokers())
            .set("group.id", self.kafka_group_id())
            .set("client.id", self.client_id())
            .set("enable.auto.commit", self.kafka_auto_commit().to_string())
            .set("auto.offset.reset", self.kafka_auto_offset_reset());
        if let Some(session_timeout) = consumer_config.session_timeout() {
            config.set("session.timeout.ms", session_timeout.to_string());
        }
        if let Some(queued_buffering_max_messages_kbytes) =
            consumer_config.queued_buffering_max_messages_kbytes()
        {
            config.set(
                "queued.max.messages.kbytes",
                queued_buffering_max_messages_kbytes.to_string(),
            );
        }
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
    /// use rdkafka::config::RDKafkaLogLevel;
    /// use rdkafka::producer::FutureProducer;
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
    /// See full list of configs properties in case you want to manually add/overwrite the config:
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
    /// ## Environment variables
    /// See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for more information
    /// configuring the producer via environment variables.
    #[cfg(feature = "rdkafka-config")]
    pub fn producer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        let producer_config = PRODUCER_CONFIG.get_or_init(config::ProducerConfig::new);
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.kafka_brokers())
            .set("client.id", self.client_id());
        if let Some(batch_num_messages) = producer_config.batch_num_messages() {
            config.set("batch.num.messages", batch_num_messages.to_string());
        }
        if let Some(queue_buffering_max_messages) = producer_config.queue_buffering_max_messages() {
            config.set(
                "queue.buffering.max.messages",
                queue_buffering_max_messages.to_string(),
            );
        }
        if let Some(queue_buffering_max_kbytes) = producer_config.queue_buffering_max_kbytes() {
            config.set(
                "queue.buffering.max.kbytes",
                queue_buffering_max_kbytes.to_string(),
            );
        }
        if let Some(queue_buffering_max_ms) = producer_config.queue_buffering_max_ms() {
            config.set("queue.buffering.max.ms", queue_buffering_max_ms.to_string());
        }
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

    /// Get reqwest blocking client config to connect to DSH Schema Registry.
    /// If certificates are present, it will use SSL to connect to Schema Registry.
    ///
    /// Use [schema_registry_converter](https://crates.io/crates/schema_registry_converter) to connect to Schema Registry.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Properties;
    /// # use reqwest::blocking::Client;
    /// # use dsh_sdk::error::DshError;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh_properties = Properties::get();
    /// let client = dsh_properties.reqwest_blocking_client_config()?.build()?;
    /// #    Ok(())
    /// # }
    pub fn reqwest_blocking_client_config(
        &self,
    ) -> Result<reqwest::blocking::ClientBuilder, DshError> {
        let mut client_builder: reqwest::blocking::ClientBuilder =
            reqwest::blocking::Client::builder();
        if let Ok(certificates) = &self.certificates() {
            client_builder = certificates.reqwest_blocking_client_config()?;
        }
        Ok(client_builder)
    }

    /// Get the certificates and private key. Returns an error when running on local machine.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::Properties;
    /// # use dsh_sdk::error::DshError;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>>{
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
    ///
    /// This datastream is fetched at initialization of the properties, and can not be updated during runtime.
    pub fn datastream(&self) -> &datastream::Datastream {
        &self.datastream
    }

    /// High level method to fetch the kafka properties provided by DSH (datastreams.json)
    /// This will fetch the datastream from DSH. This can be used to update the datastream during runtime.
    ///
    /// This method keeps the reqwest client in memory to prevent creating a new client for every request.
    ///
    /// # Panics
    /// This method panics when it can't initialize a reqwest client.
    ///
    /// Use [datastream::Datastream::fetch] as a lowlevel method where you can provide your own client.
    pub async fn fetch_datastream(&self) -> Result<datastream::Datastream, DshError> {
        static ASYNC_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

        let client = ASYNC_CLIENT.get_or_init(|| {
            self.reqwest_client_config()
                .expect("Failed loading certificates into reqwest client config")
                .build()
                .expect("Could not build reqwest client for fetching datastream")
        });
        datastream::Datastream::fetch(client, &self.config_host, &self.tenant_name, &self.task_id)
            .await
    }

    /// High level method to fetch the kafka properties provided by DSH (datastreams.json) in a blocking way.
    /// This will fetch the datastream from DSH. This can be used to update the datastream during runtime.
    ///
    /// This method keeps the reqwest client in memory to prevent creating a new client for every request.
    ///
    /// # Panics
    /// This method panics when it can't initialize a reqwest client.
    ///
    /// Use [datastream::Datastream::fetch_blocking] as a lowlevel method where you can provide your own client.
    pub fn fetch_datastream_blocking(&self) -> Result<datastream::Datastream, DshError> {
        static BLOCKING_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

        let client = BLOCKING_CLIENT.get_or_init(|| {
            self.reqwest_blocking_client_config()
                .expect("Failed loading certificates into reqwest client config")
                .build()
                .expect("Could not build reqwest client for fetching datastream")
        });
        datastream::Datastream::fetch_blocking(
            client,
            &self.config_host,
            &self.tenant_name,
            &self.task_id,
        )
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
        let consumer_config = CONSUMER_CONFIG.get_or_init(config::ConsumerConfig::new);
        consumer_config.enable_auto_commit()
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
        let consumer_config = CONSUMER_CONFIG.get_or_init(config::ConsumerConfig::new);
        consumer_config.auto_offset_reset()
    }
}

impl Default for Properties {
    fn default() -> Self {
        let datastream = datastream::Datastream::load_local_datastreams().unwrap_or_default();
        Self {
            task_id: "local_task_id".to_string(),
            tenant_name: "local_tenant".to_string(),
            config_host: "http://localhost/".to_string(),
            datastream,
            certificates: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{VAR_KAFKA_BOOTSTRAP_SERVERS, VAR_KAFKA_CONSUMER_GROUP_TYPE};
    use serial_test::serial;
    use std::io::Read;

    // maybe replace with local_datastreams.json?
    fn datastreams_json() -> String {
        std::fs::File::open("test_resources/valid_datastreams.json")
            .map(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents
            })
            .unwrap()
    }

    // Define a reusable Properties instance
    fn datastream() -> datastream::Datastream {
        serde_json::from_str(datastreams_json().as_str()).unwrap()
    }

    #[test]
    #[serial(env_dependency)]
    fn test_get_or_init() {
        let properties = Properties::get();
        assert_eq!(properties.client_id(), "local_task_id");
        assert_eq!(properties.task_id, "local_task_id");
        assert_eq!(properties.tenant_name, "local_tenant");
        assert_eq!(
            properties.config_host,
            "https://pikachu.dsh.marathon.mesos:4443"
        );
        assert!(properties.certificates.is_none());
    }

    #[test]
    #[serial(env_dependency)]
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
    #[serial(env_dependency)]
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
    #[serial(env_dependency)]
    fn test_reqwest_client_config() {
        let properties = Properties::default();
        let config = properties.reqwest_client_config();
        assert!(config.is_ok());
    }

    #[test]
    #[serial(env_dependency)]
    fn test_client_id() {
        let properties = Properties::default();
        assert_eq!(properties.client_id(), "local_task_id");
    }

    #[test]
    #[serial(env_dependency)]
    fn test_tenant_name() {
        let properties = Properties::default();
        assert_eq!(properties.tenant_name(), "local_tenant");
    }

    #[test]
    #[serial(env_dependency)]
    fn test_task_id() {
        let properties = Properties::default();
        assert_eq!(properties.task_id(), "local_task_id");
    }

    #[test]
    #[serial(env_dependency)]
    fn test_schema_registry_host() {
        let properties = Properties::default();
        assert_eq!(
            properties.schema_registry_host(),
            "http://localhost:8081/apis/ccompat/v7"
        );
    }

    #[test]
    #[serial(env_dependency)]
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
    #[serial(env_dependency)]
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
    fn test_kafka_auto_commit() {
        let properties = Properties::default();
        assert!(!properties.kafka_auto_commit());
    }

    #[test]
    fn test_kafka_auto_offset_reset() {
        let properties = Properties::default();
        assert_eq!(properties.kafka_auto_offset_reset(), "earliest");
    }

    #[tokio::test]
    async fn test_fetch_datastream() {
        let mut server = mockito::Server::new_async().await;
        let tenant = "test-tenant";
        let task_id = "test-task-id";
        let host = server.url();
        let prop = Properties::new(
            host,
            task_id.to_string(),
            tenant.to_string(),
            datastream::Datastream::default(),
            None,
        );
        server
            .mock("GET", "/kafka/config/test-tenant/test-task-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(datastreams_json())
            .create();
        let fetched_datastream = prop.fetch_datastream().await.unwrap();
        assert_eq!(fetched_datastream, datastream());
    }

    #[test]
    fn test_fetch_blocking_datastream() {
        let mut dsh = mockito::Server::new();
        let tenant = "test-tenant";
        let task_id = "test-task-id";
        let host = dsh.url();
        let prop = Properties::new(
            host,
            task_id.to_string(),
            tenant.to_string(),
            datastream::Datastream::default(),
            None,
        );
        dsh.mock("GET", "/kafka/config/test-tenant/test-task-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(datastreams_json())
            .create();
        let fetched_datastream = prop.fetch_datastream_blocking().unwrap();
        assert_eq!(fetched_datastream, datastream());
    }
}
