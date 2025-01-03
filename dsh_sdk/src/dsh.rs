//! # Dsh
//!
//! This module contains the High-level struct for all related
//!
//! From `Dsh` there are level functions to get the correct config to connect to Kafka and schema store.
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
//! use dsh_sdk::Dsh;
//! use rdkafka::consumer::{Consumer, StreamConsumer};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let dsh_properties = Dsh::get();
//! let consumer_config = dsh_properties.consumer_rdkafka_config();
//! let consumer: StreamConsumer = consumer_config.create()?;
//!
//! # Ok(())
//! # }
//! ```
use log::{error, warn};
use std::env;
use std::sync::{Arc, OnceLock};

use crate::certificates::Cert;
use crate::datastream::Datastream;
use crate::error::DshError;
use crate::utils;
use crate::*;

#[cfg(feature = "kafka")]
use crate::protocol_adapters::kafka_protocol::config::KafkaConfig;

// TODO: Remove at v0.6.0
pub use crate::dsh_old::*;

/// DSH properties struct. Create new to initialize all related components to connect to the DSH kafka clusters
///  - Contains info from datastreams.json
///  - Metadata of running container/task
///  - Certificates for Kafka and DSH Schema Registry
///
/// ## Environment variables
/// See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for
/// more information configuring the consmer or producer via environment variables.

#[derive(Debug, Clone)]
pub struct Dsh {
    config_host: String,
    task_id: String,
    tenant_name: String,
    datastream: Arc<Datastream>,
    certificates: Option<Cert>,
    #[cfg(feature = "kafka")]
    kafka_config: KafkaConfig,
}

impl Dsh {
    /// New `Dsh` struct
    pub(crate) fn new(
        config_host: String,
        task_id: String,
        tenant_name: String,
        datastream: Datastream,
        certificates: Option<Cert>,
    ) -> Self {
        let datastream = Arc::new(datastream);
        Self {
            config_host,
            task_id,
            tenant_name,
            datastream: datastream.clone(),
            certificates,
            #[cfg(feature = "kafka")]
            kafka_config: KafkaConfig::new(Some(datastream)),
        }
    }
    /// Get the DSH Dsh on a lazy way. If not already initialized, it will initialize the properties
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
    /// use dsh_sdk::Dsh;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh = Dsh::get();
    /// let datastreams = dsh.datastream();
    /// # Ok(())
    /// # }
    /// ```
    pub fn get() -> &'static Self {
        static PROPERTIES: OnceLock<Dsh> = OnceLock::new();
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
        let config_host = utils::get_env_var(VAR_KAFKA_CONFIG_HOST)
            .map(|host| format!("https://{}", host))
            .unwrap_or_else(|_| {
                warn!(
                    "{} is not set, using default value {}",
                    VAR_KAFKA_CONFIG_HOST, DEFAULT_CONFIG_HOST
                );
                DEFAULT_CONFIG_HOST.to_string()
            });
        let certificates = if let Ok(cert) = Cert::from_pki_config_dir::<std::path::PathBuf>(None) {
            Some(cert)
        } else {
            Cert::from_bootstrap(&config_host, &tenant_name, &task_id)
                .inspect_err(|e| {
                    warn!("Could not bootstrap to DSH, due to: {}", e);
                })
                .ok()
        };
        let fetched_datastreams = certificates.as_ref().and_then(|cert| {
            cert.reqwest_blocking_client_config()
                .build()
                .ok()
                .and_then(|client| {
                    Datastream::fetch_blocking(&client, &config_host, &tenant_name, &task_id).ok()
                })
        });
        let datastream = if let Some(datastream) = fetched_datastreams {
            datastream
        } else {
            warn!("Could not fetch datastreams.json, using local or default datastreams");
            Datastream::load_local_datastreams().unwrap_or_default()
        };
        Self::new(config_host, task_id, tenant_name, datastream, certificates)
    }

    /// Get reqwest async client config to connect to DSH Schema Registry.
    /// If certificates are present, it will use SSL to connect to Schema Registry.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Dsh;
    /// # use reqwest::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh_properties = Dsh::get();
    /// let client = dsh_properties.reqwest_client_config().build()?;
    /// #    Ok(())
    /// # }
    /// ```
    #[deprecated(
        since = "0.5.0",
        note = "Reqwest client is not used in DSH SDK, use `dsh_sdk::schema_store::SchemaStoreClient` instead"
    )]
    pub fn reqwest_client_config(&self) -> reqwest::ClientBuilder {
        if let Ok(certificates) = &self.certificates() {
            certificates.reqwest_client_config()
        } else {
            reqwest::Client::builder()
        }
    }

    /// Get reqwest blocking client config to connect to DSH Schema Registry.
    /// If certificates are present, it will use SSL to connect to Schema Registry.
    ///
    /// Use [schema_registry_converter](https://crates.io/crates/schema_registry_converter) to connect to Schema Registry.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Dsh;
    /// # use reqwest::blocking::Client;
    /// # use dsh_sdk::error::DshError;
    /// # fn main() -> Result<(), DshError> {
    /// let dsh_properties = Dsh::get();
    /// let client = dsh_properties.reqwest_blocking_client_config().build()?;
    /// #    Ok(())
    /// # }
    /// ```
    #[deprecated(
        since = "0.5.0",
        note = "Reqwest client is not used in DSH SDK, use `dsh_sdk::schema_store::SchemaStoreClient` instead"
    )]
    pub fn reqwest_blocking_client_config(&self) -> reqwest::blocking::ClientBuilder {
        if let Ok(certificates) = &self.certificates() {
            certificates.reqwest_blocking_client_config()
        } else {
            reqwest::blocking::Client::builder()
        }
    }

    /// Get the certificates and private key. Returns an error when running on local machine.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::Dsh;
    /// # use dsh_sdk::error::DshError;
    /// # fn main() -> Result<(), DshError> {
    /// let dsh_properties = Dsh::get();
    /// let dsh_kafka_certificate = dsh_properties.certificates()?.dsh_kafka_certificate_pem();
    /// #    Ok(())
    /// # }
    /// ```
    pub fn certificates(&self) -> Result<&Cert, DshError> {
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
    pub fn datastream(&self) -> &Datastream {
        self.datastream.as_ref()
    }

    /// High level method to fetch the kafka properties provided by DSH (datastreams.json)
    /// This will fetch the datastream from DSH. This can be used to update the datastream during runtime.
    ///
    /// This method keeps the reqwest client in memory to prevent creating a new client for every request.
    ///
    /// # Panics
    /// This method panics when it can't initialize a reqwest client.
    ///
    /// Use [Datastream::fetch] as a lowlevel method where you can provide your own client.
    pub async fn fetch_datastream(&self) -> Result<Datastream, DshError> {
        static ASYNC_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

        let client = ASYNC_CLIENT.get_or_init(|| {
            self.reqwest_client_config()
                .build()
                .expect("Could not build reqwest client for fetching datastream")
        });
        Datastream::fetch(client, &self.config_host, &self.tenant_name, &self.task_id).await
    }

    /// High level method to fetch the kafka properties provided by DSH (datastreams.json) in a blocking way.
    /// This will fetch the datastream from DSH. This can be used to update the datastream during runtime.
    ///
    /// This method keeps the reqwest client in memory to prevent creating a new client for every request.
    ///
    /// # Panics
    /// This method panics when it can't initialize a reqwest client.
    ///
    /// Use [Datastream::fetch_blocking] as a lowlevel method where you can provide your own client.
    pub fn fetch_datastream_blocking(&self) -> Result<Datastream, DshError> {
        static BLOCKING_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

        let client = BLOCKING_CLIENT.get_or_init(|| {
            self.reqwest_blocking_client_config()
                .build()
                .expect("Could not build reqwest client for fetching datastream")
        });
        Datastream::fetch_blocking(client, &self.config_host, &self.tenant_name, &self.task_id)
    }

    /// Get schema host of DSH.
    pub fn schema_registry_host(&self) -> &str {
        self.datastream().schema_store()
    }

    #[cfg(feature = "kafka")]
    #[deprecated(
        since = "0.5.0",
        note = "Moved to `Dsh::kafka_config().kafka_brokers()` and is part of the `kafka` feature"
    )]
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

    #[cfg(feature = "kafka")]
    #[deprecated(
        since = "0.5.0",
        note = "Moved to `Dsh::kafka_config().group_id()` and is part of the `kafka` feature"
    )]
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
                .get_group_id(crate::datastream::GroupType::from_env())
                .unwrap_or(&format!("{}_CONSUMER", self.tenant_name()))
                .to_string()
        }
    }

    #[cfg(feature = "kafka")]
    #[deprecated(
        since = "0.5.0",
        note = "Moved to `Dsh::kafka_config().enable_auto_commit()` and is part of the `kafka` feature"
    )]
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
        self.kafka_config.enable_auto_commit()
    }

    #[cfg(feature = "kafka")]
    #[deprecated(
        since = "0.5.0",
        note = "Moved to `Dsh::kafka_config().auto_offset_reset()` and is part of the `kafka` feature"
    )]
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
        self.kafka_config.auto_offset_reset().to_string()
    }

    #[cfg(feature = "kafka")]
    /// Get the kafka config from initiated Dsh struct.
    pub fn kafka_config(&self) -> &KafkaConfig {
        &self.kafka_config
    }

    #[deprecated(
        since = "0.5.0",
        note = "Use `Dsh::DshKafkaConfig` trait instead, see https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)"
    )]
    #[cfg(feature = "rdkafka-config")]
    pub fn consumer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        use crate::protocol_adapters::kafka_protocol::DshKafkaConfig;
        let mut config = rdkafka::config::ClientConfig::new();
        config.dsh_consumer_config();
        config
    }

    #[deprecated(
        since = "0.5.0",
        note = "Use `Dsh::DshKafkaConfig` trait instead, see https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)"
    )]
    #[cfg(feature = "rdkafka-config")]
    pub fn producer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        use crate::protocol_adapters::kafka_protocol::DshKafkaConfig;
        let mut config = rdkafka::config::ClientConfig::new();
        config.dsh_producer_config();
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{VAR_KAFKA_BOOTSTRAP_SERVERS, VAR_KAFKA_CONSUMER_GROUP_TYPE};
    use serial_test::serial;
    use std::io::Read;

    impl Default for Dsh {
        fn default() -> Self {
            let datastream = Arc::new(Datastream::load_local_datastreams().unwrap_or_default());
            Self {
                task_id: "local_task_id".to_string(),
                tenant_name: "local_tenant".to_string(),
                config_host: "http://localhost/".to_string(),
                datastream,
                certificates: None,
                #[cfg(feature = "kafka")]
                kafka_config: KafkaConfig::default(),
            }
        }
    }

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

    // Define a reusable Dsh instance
    fn datastream() -> Datastream {
        serde_json::from_str(datastreams_json().as_str()).unwrap()
    }

    #[test]
    #[serial(env_dependency)]
    fn test_get_or_init() {
        let properties = Dsh::get();
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
    fn test_reqwest_client_config() {
        let properties = Dsh::default();
        let _ = properties.reqwest_client_config();
        assert!(true)
    }

    #[test]
    #[serial(env_dependency)]
    fn test_client_id() {
        let properties = Dsh::default();
        assert_eq!(properties.client_id(), "local_task_id");
    }

    #[test]
    #[serial(env_dependency)]
    fn test_tenant_name() {
        let properties = Dsh::default();
        assert_eq!(properties.tenant_name(), "local_tenant");
    }

    #[test]
    #[serial(env_dependency)]
    fn test_task_id() {
        let properties = Dsh::default();
        assert_eq!(properties.task_id(), "local_task_id");
    }

    #[test]
    #[serial(env_dependency)]
    fn test_schema_registry_host() {
        let properties = Dsh::default();
        assert_eq!(
            properties.schema_registry_host(),
            "http://localhost:8081/apis/ccompat/v7"
        );
    }

    #[test]
    #[serial(env_dependency)]
    fn test_kafka_brokers() {
        let properties = Dsh::default();
        assert_eq!(
            properties.kafka_brokers(),
            properties.datastream().get_brokers_string()
        );
        env::set_var(VAR_KAFKA_BOOTSTRAP_SERVERS, "test:9092");
        let properties = Dsh::default();
        assert_eq!(properties.kafka_brokers(), "test:9092");
        env::remove_var(VAR_KAFKA_BOOTSTRAP_SERVERS);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_kafka_group_id() {
        let properties = Dsh::default();
        assert_eq!(
            properties.kafka_group_id(),
            properties
                .datastream()
                .get_group_id(crate::datastream::GroupType::Shared(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "private");
        assert_eq!(
            properties.kafka_group_id(),
            properties
                .datastream()
                .get_group_id(crate::datastream::GroupType::Private(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "shared");
        assert_eq!(
            properties.kafka_group_id(),
            properties
                .datastream()
                .get_group_id(crate::datastream::GroupType::Shared(0))
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
    #[serial(env_dependency)]
    fn test_kafka_auto_commit() {
        let properties = Dsh::default();
        assert!(!properties.kafka_auto_commit());
    }

    #[test]
    #[serial(env_dependency)]
    fn test_kafka_auto_offset_reset() {
        let properties = Dsh::default();
        assert_eq!(properties.kafka_auto_offset_reset(), "earliest");
    }

    #[tokio::test]
    async fn test_fetch_datastream() {
        let mut server = mockito::Server::new_async().await;
        let tenant = "test-tenant";
        let task_id = "test-task-id";
        let host = server.url();
        let prop = Dsh::new(
            host,
            task_id.to_string(),
            tenant.to_string(),
            Datastream::default(),
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
        let prop = Dsh::new(
            host,
            task_id.to_string(),
            tenant.to_string(),
            Datastream::default(),
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

    #[test]
    #[serial(env_dependency)]
    fn test_consumer_rdkafka_config() {
        let dsh = Dsh::default();
        let config = dsh.consumer_rdkafka_config();
        assert_eq!(
            config.get("bootstrap.servers").unwrap(),
            dsh.datastream().get_brokers_string()
        );
        assert_eq!(
            config.get("group.id").unwrap(),
            dsh.datastream()
                .get_group_id(crate::datastream::GroupType::from_env())
                .unwrap()
        );
        assert_eq!(config.get("client.id").unwrap(), dsh.client_id());
        assert_eq!(config.get("enable.auto.commit").unwrap(), "false");
        assert_eq!(config.get("auto.offset.reset").unwrap(), "earliest");
    }

    #[test]
    #[serial(env_dependency)]
    fn test_producer_rdkafka_config() {
        let dsh = Dsh::default();
        let config = dsh.producer_rdkafka_config();
        assert_eq!(
            config.get("bootstrap.servers").unwrap(),
            dsh.datastream().get_brokers_string()
        );
        assert_eq!(config.get("client.id").unwrap(), dsh.client_id());
    }
}
