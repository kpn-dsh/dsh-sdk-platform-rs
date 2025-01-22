//! DSH Configuration for Kafka
//!
//! This module provides all necessary configurations for consuming and producing messages
//! to/from the DSH (Data Services Hub) Kafka Cluster. The [`DshKafkaConfig`] trait is at
//! the core of this module, guiding you to set the essential Kafka config parameters
//! automatically (e.g., brokers, security certificates, group ID).
//!
//! # Example
//! ```
//! use dsh_sdk::DshKafkaConfig;
//! use rdkafka::ClientConfig;
//! use rdkafka::consumer::StreamConsumer;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Build an rdkafka consumer with DSH settings.
//! let consumer: StreamConsumer = ClientConfig::new()
//!     .set_dsh_consumer_config()
//!     .create()?;
//!
//! // Use your consumer...
//! # Ok(())
//! # }
//! ```

pub mod config;

#[cfg(feature = "rdkafka")]
mod rdkafka;

/// Trait defining core DSH configurations for Kafka consumers and producers.
///
/// Implementing `DshKafkaConfig` ensures that the correct settings (including SSL)
/// are applied for connecting to a DSH-managed Kafka cluster. The trait provides:
/// - [`set_dsh_consumer_config`](DshKafkaConfig::set_dsh_consumer_config)  
/// - [`set_dsh_producer_config`](DshKafkaConfig::set_dsh_producer_config)  
/// - [`set_dsh_group_id`](DshKafkaConfig::set_dsh_group_id)  
/// - [`set_dsh_certificates`](DshKafkaConfig::set_dsh_certificates)
///
/// # Environment Variables
/// Via environment variables you can override or supplement certain default settings:
///
/// See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for the full list.
///
/// By configuring these variables, you can control broker endpoints, group IDs, and
/// various Kafka client behaviors without modifying code.
pub trait DshKafkaConfig {
    /// Applies all required consumer settings to connect with the DSH Kafka Cluster.
    ///
    /// Below is a table of configurations applied by this function:
    ///
    /// | **Config Key**             | **Default Value**                | **Overridable?**                                               | **Description**                                                                 |
    /// |----------------------------|----------------------------------|----------------------------------------------------------------|---------------------------------------------------------------------------------|
    /// | `bootstrap.servers`       | Brokers from `datastreams.json`  | Env var `KAFKA_BOOTSTRAP_SERVERS`                               | List of Kafka brokers to connect to.                                            |
    /// | `group.id`                | Shared group from `datastreams`  | Env vars `KAFKA_GROUP_ID` / `KAFKA_CONSUMER_GROUP_TYPE`         | Consumer group ID (DSH requires tenant prefix).                                 |
    /// | `client.id`               | `task_id` of the service         | _No direct override_                                            | Used for consumer identification in logs/metrics.                               |
    /// | `enable.auto.commit`      | `false`                          | Env var `KAFKA_ENABLE_AUTO_COMMIT`                              | Controls whether offsets are committed automatically.                           |
    /// | `auto.offset.reset`       | `earliest`                       | Env var `KAFKA_AUTO_OFFSET_RESET`                               | Defines behavior when no valid offset is available (e.g., `earliest`, `latest`).|
    /// | `security.protocol`       | `ssl` to DSH, `plaintext` locally| _Internal_                                                      | Chooses SSL if DSH certificates are present, otherwise plaintext.               |
    /// | `ssl.key.pem`             | Private key from certificates    | _Auto-configured_                                               | Loaded from SDK during bootstrap.                                               |
    /// | `ssl.certificate.pem`     | DSH Kafka certificate            | _Auto-configured_                                               | Signed certificate to connect to the Kafka cluster.                             |
    /// | `ssl.ca.pem`              | CA certificate from DSH          | _Auto-configured_                                               | Authority certificate for SSL.                                                  |
    fn set_dsh_consumer_config(&mut self) -> &mut Self;

    /// Applies all required producer settings to publish messages to the DSH Kafka Cluster.
    ///
    /// ## Producer Configurations
    /// | **Config Key**             | **Default Value**                | **Overridable?**                                  | **Description**                                                                   |
    /// |----------------------------|----------------------------------|---------------------------------------------------|-------------------------------------------------------------------------------------|
    /// | `bootstrap.servers`       | Brokers from `datastreams.json`  | Env var `KAFKA_BOOTSTRAP_SERVERS`                 | List of Kafka brokers to connect to.                                               |
    /// | `client.id`               | `task_id` of the service         | _No direct override_                              | Used for producer identification in logs/metrics.                                  |
    /// | `security.protocol`       | `ssl` in DSH, `plaintext` locally| _Internal_                                        | Chooses SSL if DSH certificates are present, otherwise plaintext.                  |
    /// | `ssl.key.pem`             | Private key from certificates    | _Auto-configured_                                 | Loaded from SDK during bootstrap.                                                  |
    /// | `ssl.certificate.pem`     | DSH Kafka certificate            | _Auto-configured_                                 | Signed certificate (when bootstrapped) to connect to the Kafka cluster.            |
    /// | `ssl.ca.pem`              | CA certificate from DSH          | _Auto-configured_                                 | Authority certificate for SSL.                                                     |
    fn set_dsh_producer_config(&mut self) -> &mut Self;

    /// Applies a DSH-compatible group ID.
    ///
    /// DSH requires the consumer group ID to be prefixed with the tenant name.
    /// If an environment variable (e.g., `KAFKA_GROUP_ID` or `KAFKA_CONSUMER_GROUP_TYPE`)
    /// is set, that value can override what is found in `datastreams.json`.
    fn set_dsh_group_id(&mut self, group_id: &str) -> &mut Self;

    /// Sets the required DSH certificates for secure SSL connections.
    ///
    /// If the required certificates are found (via the DSH bootstrap or
    /// environment variables), this function configures SSL. Otherwise,
    /// it falls back to plaintext (for local development).
    ///
    /// # Note
    /// This method typically sets:
    /// - `security.protocol` to `ssl`
    /// - `ssl.key.pem`, `ssl.certificate.pem`, and `ssl.ca.pem`
    ///  
    /// If certificates are missing, `security.protocol` remains `plaintext`.
    fn set_dsh_certificates(&mut self) -> &mut Self;
}
