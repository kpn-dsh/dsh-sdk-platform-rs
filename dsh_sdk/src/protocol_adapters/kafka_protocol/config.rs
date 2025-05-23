//! Kafka configuration
//!
//! This module contains the configuration for the Kafka protocol adapter.
use std::env;
use std::sync::Arc;

use crate::datastream::Datastream;
use crate::utils::get_env_var;
use crate::*;

/// Kafka config
///
/// ## Environment variables
/// See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for more information
/// configuring the consmer via environment variables.
#[derive(Debug, Clone)]
pub struct KafkaConfig {
    // Datastreams
    datastream: Arc<Datastream>,
    // Consumer specific config
    enable_auto_commit: bool,
    auto_offset_reset: String,
    session_timeout: Option<i32>,
    queued_buffering_max_messages_kbytes: Option<i32>,
    // Producer specific config
    batch_num_messages: Option<i32>,
    queue_buffering_max_messages: Option<i32>,
    queue_buffering_max_kbytes: Option<i32>,
    queue_buffering_max_ms: Option<i32>,
}

impl KafkaConfig {
    pub fn new(datastream: Option<Arc<Datastream>>) -> Self {
        let datastream = datastream
            .unwrap_or_else(|| Arc::new(Datastream::load_local_datastreams().unwrap_or_default()));
        let enable_auto_commit = get_env_var(VAR_KAFKA_ENABLE_AUTO_COMMIT)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);
        let auto_offset_reset =
            get_env_var(VAR_KAFKA_AUTO_OFFSET_RESET).unwrap_or("earliest".to_string());
        let session_timeout = get_env_var(VAR_KAFKA_CONSUMER_SESSION_TIMEOUT_MS)
            .ok()
            .and_then(|v| v.parse().ok());
        let queued_buffering_max_messages_kbytes =
            get_env_var(VAR_KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES)
                .ok()
                .and_then(|v| v.parse().ok());
        let batch_num_messages = get_env_var(VAR_KAFKA_PRODUCER_BATCH_NUM_MESSAGES)
            .ok()
            .and_then(|v| v.parse().ok());
        let queue_buffering_max_messages =
            get_env_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES)
                .ok()
                .and_then(|v| v.parse().ok());
        let queue_buffering_max_kbytes = get_env_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES)
            .ok()
            .and_then(|v| v.parse().ok());
        let queue_buffering_max_ms = get_env_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS)
            .ok()
            .and_then(|v| v.parse().ok());
        Self {
            datastream,
            enable_auto_commit,
            auto_offset_reset,
            session_timeout,
            queued_buffering_max_messages_kbytes,
            batch_num_messages,
            queue_buffering_max_messages,
            queue_buffering_max_kbytes,
            queue_buffering_max_ms,
        }
    }

    /// Get the kafka config provided by DSH (datastreams.json)
    ///
    /// This datastream is fetched at initialization of the config, and can not be updated during runtime.
    pub fn datastream(&self) -> &Datastream {
        self.datastream.as_ref()
    }

    /// Get the Kafka brokers.
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_BOOTSTRAP_SERVERS`
    /// - Usage: Overwrite hostnames of brokers
    /// - Default: Brokers based on datastreams
    /// - Required: `false`
    pub fn kafka_brokers(&self) -> String {
        self.datastream().get_brokers_string()
    }

    /// Get the kafka_group_id
    ///
    /// ## Environment variables
    /// You can set the following environment variables to overwrite the default value.
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
    pub fn group_id(&self) -> String {
        // TODO: Stabilize this function to fetch it once and not every time
        let tenant_name = Dsh::get().tenant_name();
        if let Ok(group_id) = env::var(VAR_KAFKA_GROUP_ID) {
            if !group_id.starts_with(tenant_name) {
                format!("{}_{}", tenant_name, group_id)
            } else {
                group_id
            }
        } else {
            self.datastream()
                .get_group_id(crate::datastream::GroupType::from_env())
                .unwrap_or(&format!("{}_CONSUMER", tenant_name))
                .to_string()
        }
    }

    /// Get the confifured kafka auto commit setinngs.
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_ENABLE_AUTO_COMMIT`
    /// - Usage: Enable/Disable auto commit
    /// - Default: `false`
    /// - Required: `false`
    /// - Options: `true`, `false`
    pub fn enable_auto_commit(&self) -> bool {
        self.enable_auto_commit
    }

    /// Get the kafka auto offset reset settings.
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_AUTO_OFFSET_RESET`
    /// - Usage: Set the offset reset settings to start consuming from set option.
    /// - Default: earliest
    /// - Required: `false`
    /// - Options: smallest, earliest, beginning, largest, latest, end
    pub fn auto_offset_reset(&self) -> &str {
        &self.auto_offset_reset
    }

    /// Session timeout in milliseconds for consuming messages
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_CONSUMER_SESSION_TIMEOUT_MS`
    /// - Usage: Set the session timeout in milliseconds
    /// - Default: LibRdKafka default
    /// - Required: `false`
    /// - Options: Any integer
    pub fn session_timeout(&self) -> Option<i32> {
        self.session_timeout
    }

    /// Queued buffering max messages kbytes while consiuming
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES`
    /// - Usage: Set the queued buffering max messages kbytes
    /// - Default: LibRdKafka default
    /// - Required: `false`
    /// - Options: Any integer
    pub fn queued_buffering_max_messages_kbytes(&self) -> Option<i32> {
        self.queued_buffering_max_messages_kbytes
    }

    /// Batch number of messages to be produced
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_PRODUCER_BATCH_NUM_MESSAGES`
    /// - Usage: Set the batch number of messages to be produced
    /// - Default: LibRdKafka default
    /// - Required: `false`
    /// - Options: Any integer
    pub fn batch_num_messages(&self) -> Option<i32> {
        self.batch_num_messages
    }

    /// Maximum number of messages allowed on the producer queue
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES`
    /// - Usage: Set the maximum number of messages allowed on the producer queue
    /// - Default: LibRdKafka default
    /// - Required: `false`
    /// - Options: Any integer
    pub fn queue_buffering_max_messages(&self) -> Option<i32> {
        self.queue_buffering_max_messages
    }

    /// Maximum total message size in KBYTES sum allowed on the producer queue
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES`
    /// - Usage: Set the maximum total message size in KBYTES sum allowed on the producer queue
    /// - Default: LibRdKafka default
    /// - Required: `false`
    /// - Options: Any integer
    pub fn queue_buffering_max_kbytes(&self) -> Option<i32> {
        self.queue_buffering_max_kbytes
    }

    /// Delay in milliseconds to wait for messages in the producer queue to accumulate before sending in batch
    ///
    /// ## Environment variable
    /// You can set the following environment variable to overwrite the default value.
    ///
    /// ### `KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS`
    /// - Usage: Set the delay in milliseconds to wait for messages in the producer queue to accumulate before sending in batch
    /// - Default: LibRdKafka default
    /// - Required: `false`
    /// - Options: Any integer
    pub fn queue_buffering_max_ms(&self) -> Option<i32> {
        self.queue_buffering_max_ms
    }
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial(env_dependency)]
    fn test_kafka_config() {
        let consumer_config = KafkaConfig::new(None);
        assert_eq!(consumer_config.enable_auto_commit(), false);
        assert_eq!(consumer_config.auto_offset_reset(), "earliest");
        assert_eq!(consumer_config.session_timeout(), None);
        assert_eq!(consumer_config.queued_buffering_max_messages_kbytes(), None);
        assert_eq!(consumer_config.batch_num_messages(), None);
        assert_eq!(consumer_config.queue_buffering_max_messages(), None);
        assert_eq!(consumer_config.queue_buffering_max_kbytes(), None);
        assert_eq!(consumer_config.queue_buffering_max_ms(), None);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_kafka_config_default() {
        let consumer_config = KafkaConfig::default();
        assert_eq!(consumer_config.enable_auto_commit(), false);
        assert_eq!(consumer_config.auto_offset_reset(), "earliest");
        assert_eq!(consumer_config.session_timeout(), None);
        assert_eq!(consumer_config.queued_buffering_max_messages_kbytes(), None);
        assert_eq!(consumer_config.batch_num_messages(), None);
        assert_eq!(consumer_config.queue_buffering_max_messages(), None);
        assert_eq!(consumer_config.queue_buffering_max_kbytes(), None);
        assert_eq!(consumer_config.queue_buffering_max_ms(), None);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_consumer_kafka_config_env() {
        env::set_var(VAR_KAFKA_ENABLE_AUTO_COMMIT, "true");
        env::set_var(VAR_KAFKA_AUTO_OFFSET_RESET, "latest");
        env::set_var(VAR_KAFKA_CONSUMER_SESSION_TIMEOUT_MS, "1000");
        env::set_var(
            VAR_KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES,
            "1000",
        );
        let consumer_config = KafkaConfig::default();
        assert_eq!(consumer_config.enable_auto_commit(), true);
        assert_eq!(consumer_config.auto_offset_reset(), "latest");
        assert_eq!(consumer_config.session_timeout(), Some(1000));
        assert_eq!(
            consumer_config.queued_buffering_max_messages_kbytes(),
            Some(1000)
        );
        env::remove_var(VAR_KAFKA_ENABLE_AUTO_COMMIT);
        env::remove_var(VAR_KAFKA_AUTO_OFFSET_RESET);
        env::remove_var(VAR_KAFKA_CONSUMER_SESSION_TIMEOUT_MS);
        env::remove_var(VAR_KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_producer_kafka_config_env() {
        env::set_var(VAR_KAFKA_PRODUCER_BATCH_NUM_MESSAGES, "1000");
        env::set_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES, "1000");
        env::set_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES, "1000");
        env::set_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS, "1000");
        let producer_config = KafkaConfig::default();
        assert_eq!(producer_config.batch_num_messages(), Some(1000));
        assert_eq!(producer_config.queue_buffering_max_messages(), Some(1000));
        assert_eq!(producer_config.queue_buffering_max_kbytes(), Some(1000));
        assert_eq!(producer_config.queue_buffering_max_ms(), Some(1000));
        env::remove_var(VAR_KAFKA_PRODUCER_BATCH_NUM_MESSAGES);
        env::remove_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES);
        env::remove_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES);
        env::remove_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_kafka_group_id() {
        let config = KafkaConfig::default();
        let dsh = Dsh::default();
        assert_eq!(
            config.group_id(),
            config
                .datastream()
                .get_group_id(crate::datastream::GroupType::Shared(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "private");
        assert_eq!(
            config.group_id(),
            config
                .datastream()
                .get_group_id(crate::datastream::GroupType::Private(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "shared");
        assert_eq!(
            config.group_id(),
            config
                .datastream()
                .get_group_id(crate::datastream::GroupType::Shared(0))
                .unwrap()
        );
        env::set_var(VAR_KAFKA_GROUP_ID, "test_group");
        assert_eq!(
            config.group_id(),
            format!("{}_test_group", dsh.tenant_name())
        );
        env::set_var(
            VAR_KAFKA_GROUP_ID,
            format!("{}_test_group", dsh.tenant_name()),
        );
        assert_eq!(
            config.group_id(),
            format!("{}_test_group", dsh.tenant_name())
        );
        env::remove_var(VAR_KAFKA_CONSUMER_GROUP_TYPE);
        assert_eq!(
            config.group_id(),
            format!("{}_test_group", dsh.tenant_name())
        );
        env::remove_var(VAR_KAFKA_GROUP_ID);
    }
}
