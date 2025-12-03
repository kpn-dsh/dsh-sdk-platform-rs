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
use std::sync::{Arc, OnceLock};

use crate::certificates::{Cert, CertificatesError};
use crate::datastream::Datastream;
use crate::error::DshError;
use crate::utils;
use crate::*;

#[cfg(feature = "kafka")]
use crate::protocol_adapters::kafka_protocol::config::KafkaConfig;

/// Lazily initializes all related components to connect to DSH and Kafka.
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
        let tenant_name = utils::tenant_name().unwrap_or_else(|_| "local_tenant".to_string());
        let task_id =
            utils::get_env_var(VAR_TASK_ID).unwrap_or_else(|_| "local_task_id".to_string());
        let config_host =
            utils::get_env_var(VAR_DSH_KAFKA_CONFIG_ENDPOINT).map(utils::ensure_https_prefix);

        let certificates = if let Ok(cert) = Cert::from_pki_config_dir::<std::path::PathBuf>(None) {
            Some(cert)
        } else if let Ok(config_host) = &config_host {
            Cert::from_bootstrap(config_host, &tenant_name, &task_id)
                .inspect_err(|e| warn!("Could not bootstrap to DSH, due to: {}", e))
                .ok()
        } else {
            None
        };

        let config_host = config_host.unwrap_or_else(|_| DEFAULT_CONFIG_HOST.to_string());
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
    /// let dsh_kafka_certificate = dsh.certificates()?.dsh_signed_certificate_pem();
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
    /// Returns the [`KafkaConfig`] from this `Dsh` instance.
    pub fn kafka_config(&self) -> &KafkaConfig {
        &self.kafka_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
