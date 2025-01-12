//! High-level API for interacting with DSH when your container is running on DSH.
//!
//! From [`Dsh`] you can retrieve the correct configuration to connect to Kafka and the schema store.
//!
//! For more low-level functions, see the [`datastream`] and [`certificates`] modules.
//!
//! ## Environment Variables
//! Refer to [`ENV_VARIABLES.md`](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md)
//! for more information on configuring the consumer or producer via environment variables.
//!
//! # Example
//! ```no_run
//! use dsh_sdk::Dsh;
//! use rdkafka::consumer::{Consumer, StreamConsumer};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let dsh = Dsh::get();
//! let certificates = dsh.certificates()?;
//! let datastreams = dsh.datastream();
//! let kafka_config = dsh.kafka_config();
//! let tenant_name = dsh.tenant_name();
//! let task_id = dsh.task_id();
//! # Ok(())
//! # }
//! ```
use log::warn;
use std::env;
use std::sync::{Arc, OnceLock};

use crate::certificates::{ensure_https_prefix, Cert, CertificatesError};
use crate::datastream::Datastream;
use crate::error::DshError;
use crate::utils;
use crate::*;

#[cfg(feature = "kafka")]
use crate::protocol_adapters::kafka_protocol::config::KafkaConfig;

// TODO: Remove at v0.6.0
pub use crate::dsh_old::*;

/// Lazily initializes all related components to connect to DSH:
/// - Information from `datastreams.json`
/// - Metadata of the running container/task
/// - Certificates for Kafka and DSH Schema Registry
///
/// ## Environment Variables
/// Refer to [`ENV_VARIABLES.md`](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md)
/// for details on configuring the consumer or producer via environment variables.
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
    /// Constructs a new `Dsh` struct. This is internal and should typically be accessed via [`Dsh::get()`].
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

    /// Returns a reference to the global `Dsh` instance, initializing it if necessary.
    ///
    /// This struct contains configuration and certificates needed to connect to Kafka and DSH:
    /// - A struct mirroring `datastreams.json`
    /// - Metadata for the running container/task
    /// - Certificates for Kafka and DSH
    ///
    /// # Panics
    /// Panics if attempting to load an incorrect `local_datastream.json` on a local machine.
    /// If no file is available or the `LOCAL_DATASTREAMS_JSON` env variable is unset, it returns a default
    /// `datastream` struct and does **not** panic.
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

    /// Initializes the properties and bootstraps to DSH.
    fn init() -> Self {
        let tenant_name = utils::tenant_name().unwrap_or_else(|| "local_tenant".to_string());
        let task_id =
            utils::get_env_var(VAR_TASK_ID).unwrap_or_else(|| "local_task_id".to_string());
        let config_host = utils::get_env_var(VAR_KAFKA_CONFIG_HOST).map(ensure_https_prefix);

        let certificates = if let Ok(cert) = Cert::from_pki_config_dir::<std::path::PathBuf>(None) {
            Some(cert)
        } else if let Ok(config_host) = &config_host {
            Cert::from_bootstrap(config_host, &tenant_name, &task_id)
                .inspect_err(|e| warn!("Could not bootstrap to DSH, due to: {}", e))
                .ok()
        } else {
            None
        };

        let config_host = config_host.unwrap_or_else(|| DEFAULT_CONFIG_HOST.to_string());
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
            warn!("Could not fetch datastreams.json; using local or default datastreams");
            Datastream::load_local_datastreams().unwrap_or_default()
        };

        Self::new(config_host, task_id, tenant_name, datastream, certificates)
    }

    /// Returns a `reqwest::ClientBuilder` configured to connect to the DSH Schema Registry.
    /// If certificates are present, SSL is used. Otherwise, it falls back to a non-SSL connection.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Dsh;
    /// # use reqwest::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh = Dsh::get();
    /// let client = dsh.reqwest_client_config().build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn reqwest_client_config(&self) -> reqwest::ClientBuilder {
        if let Ok(certificates) = &self.certificates() {
            certificates.reqwest_client_config()
        } else {
            reqwest::Client::builder()
        }
    }

    /// Returns a `reqwest::blocking::ClientBuilder` configured to connect to the DSH Schema Registry.
    /// If certificates are present, SSL is used. Otherwise, it falls back to a non-SSL connection.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Dsh;
    /// # use reqwest::blocking::Client;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh = Dsh::get();
    /// let client = dsh.reqwest_blocking_client_config().build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn reqwest_blocking_client_config(&self) -> reqwest::blocking::ClientBuilder {
        if let Ok(certificates) = &self.certificates() {
            certificates.reqwest_blocking_client_config()
        } else {
            reqwest::blocking::Client::builder()
        }
    }

    /// Retrieves the certificates and private key. Returns an error when running on a local machine.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::Dsh;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh = Dsh::get();
    /// let dsh_kafka_certificate = dsh.certificates()?.dsh_kafka_certificate_pem();
    /// # Ok(())
    /// # }
    /// ```
    pub fn certificates(&self) -> Result<&Cert, DshError> {
        match &self.certificates {
            Some(cert) => Ok(cert),
            None => Err(CertificatesError::NoCertificates.into()),
        }
    }

    /// Returns the client ID derived from the task ID.
    pub fn client_id(&self) -> &str {
        &self.task_id
    }

    /// Returns the tenant name of the running container.
    pub fn tenant_name(&self) -> &str {
        &self.tenant_name
    }

    /// Returns the task ID of the running container.
    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    /// Returns the current datastream object (fetched at initialization). This cannot be updated at runtime.
    pub fn datastream(&self) -> &Datastream {
        self.datastream.as_ref()
    }

    /// Fetches the latest datastream (Kafka properties) from DSH asynchronously.  
    /// This can be used to update the datastream during runtime.
    ///
    /// # Panics
    /// Panics if it fails to build a reqwest client.
    ///
    /// For a lower-level method allowing a custom client, see [`Datastream::fetch`].
    pub async fn fetch_datastream(&self) -> Result<Datastream, DshError> {
        static ASYNC_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

        let client = ASYNC_CLIENT.get_or_init(|| {
            self.reqwest_client_config()
                .build()
                .expect("Could not build reqwest client for fetching datastream")
        });

        Ok(Datastream::fetch(client, &self.config_host, &self.tenant_name, &self.task_id).await?)
    }

    /// Fetches the latest datastream from DSH in a blocking manner.  
    /// This can be used to update the datastream during runtime.
    ///
    /// # Panics
    /// Panics if it fails to build a reqwest blocking client.
    ///
    /// For a lower-level method allowing a custom client, see [`Datastream::fetch_blocking`].
    pub fn fetch_datastream_blocking(&self) -> Result<Datastream, DshError> {
        static BLOCKING_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

        let client = BLOCKING_CLIENT.get_or_init(|| {
            self.reqwest_blocking_client_config()
                .build()
                .expect("Could not build reqwest client for fetching datastream")
        });

        Ok(Datastream::fetch_blocking(
            client,
            &self.config_host,
            &self.tenant_name,
            &self.task_id,
        )?)
    }

    /// Returns the schema registry host as defined by the datastream.
    pub fn schema_registry_host(&self) -> &str {
        self.datastream().schema_store()
    }

    #[cfg(feature = "kafka")]
    #[deprecated(
        since = "0.5.0",
        note = "Moved to `Dsh::kafka_config().kafka_brokers()`. Part of the `kafka` feature."
    )]
    /// Returns the Kafka brokers.
    ///
    /// ## Environment Variables
    /// - `KAFKA_BOOTSTRAP_SERVERS`: Overwrites broker hostnames (optional).
    ///   Defaults to brokers from the datastream.
    pub fn kafka_brokers(&self) -> String {
        self.datastream().get_brokers_string()
    }

    #[cfg(feature = "kafka")]
    #[deprecated(
        since = "0.5.0",
        note = "Moved to `Dsh::kafka_config().group_id()`. Part of the `kafka` feature."
    )]
    /// Returns the Kafka group ID.
    ///
    /// ## Environment Variables
    /// - `KAFKA_CONSUMER_GROUP_TYPE`: Chooses a group ID type (private or shared).  
    /// - `KAFKA_GROUP_ID`: Custom group ID. Overrules `KAFKA_CONSUMER_GROUP_TYPE`.
    ///
    /// If the group ID doesn't start with the tenant name, it's automatically prefixed.
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
        note = "Moved to `Dsh::kafka_config().enable_auto_commit()`. Part of the `kafka` feature."
    )]
    /// Returns the configured Kafka auto-commit setting.
    ///
    /// ## Environment Variables
    /// - `KAFKA_ENABLE_AUTO_COMMIT`: Enables/disables auto commit (default: `false`).
    pub fn kafka_auto_commit(&self) -> bool {
        self.kafka_config.enable_auto_commit()
    }

    #[cfg(feature = "kafka")]
    #[deprecated(
        since = "0.5.0",
        note = "Moved to `Dsh::kafka_config().auto_offset_reset()`. Part of the `kafka` feature."
    )]
    /// Returns the Kafka auto-offset-reset setting.
    ///
    /// ## Environment Variables
    /// - `KAFKA_AUTO_OFFSET_RESET`: Set the offset reset policy (default: `earliest`).
    pub fn kafka_auto_offset_reset(&self) -> String {
        self.kafka_config.auto_offset_reset().to_string()
    }

    #[cfg(feature = "kafka")]
    /// Returns the [`KafkaConfig`] from this `Dsh` instance.
    pub fn kafka_config(&self) -> &KafkaConfig {
        &self.kafka_config
    }

    #[deprecated(
        since = "0.5.0",
        note = "Use the `DshKafkaConfig` trait instead. See wiki for migration details."
    )]
    #[cfg(feature = "rdkafka-config")]
    /// Returns an `rdkafka::config::ClientConfig` for a consumer, configured via Dsh.
    pub fn consumer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        use crate::protocol_adapters::kafka_protocol::DshKafkaConfig;
        let mut config = rdkafka::config::ClientConfig::new();
        config.set_dsh_consumer_config();
        config
    }

    #[deprecated(
        since = "0.5.0",
        note = "Use the `DshKafkaConfig` trait instead. See wiki for migration details."
    )]
    #[cfg(feature = "rdkafka-config")]
    /// Returns an `rdkafka::config::ClientConfig` for a producer, configured via Dsh.
    pub fn producer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        use crate::protocol_adapters::kafka_protocol::DshKafkaConfig;
        let mut config = rdkafka::config::ClientConfig::new();
        config.set_dsh_producer_config();
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

    // Helper to load test datastreams from a file.
    fn datastreams_json() -> String {
        std::fs::File::open("test_resources/valid_datastreams.json")
            .map(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents
            })
            .unwrap()
    }

    // Helper to create a test Datastream.
    fn datastream() -> Datastream {
        serde_json::from_str(&datastreams_json()).unwrap()
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
        assert!(true);
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
