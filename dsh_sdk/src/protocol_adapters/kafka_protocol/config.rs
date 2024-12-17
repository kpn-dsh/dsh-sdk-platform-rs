//! Kafka configuration
//!
//! This module contains the configuration for the Kafka protocol adapter.

use crate::utils::get_env_var;
use crate::*;
use std::sync::OnceLock;

static KAFKA_CONFIG: OnceLock<KafkaConfig> = OnceLock::new();

/// Kafka config
///
/// ## Environment variables
/// See [ENV_VARIABLES.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/ENV_VARIABLES.md) for more information
/// configuring the consmer via environment variables.
#[derive(Debug, Clone)]
pub struct KafkaConfig {
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
    pub fn new() -> Self {
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
    // TODO: Check, does this make sense?
    pub fn get() -> &'static KafkaConfig {
        KAFKA_CONFIG.get_or_init(KafkaConfig::new)
    }
    pub fn enable_auto_commit(&self) -> bool {
        self.enable_auto_commit
    }
    pub fn auto_offset_reset(&self) -> String {
        self.auto_offset_reset.clone()
    }
    pub fn session_timeout(&self) -> Option<i32> {
        self.session_timeout
    }
    pub fn queued_buffering_max_messages_kbytes(&self) -> Option<i32> {
        self.queued_buffering_max_messages_kbytes
    }
    pub fn batch_num_messages(&self) -> Option<i32> {
        self.batch_num_messages
    }
    pub fn queue_buffering_max_messages(&self) -> Option<i32> {
        self.queue_buffering_max_messages
    }
    pub fn queue_buffering_max_kbytes(&self) -> Option<i32> {
        self.queue_buffering_max_kbytes
    }
    pub fn queue_buffering_max_ms(&self) -> Option<i32> {
        self.queue_buffering_max_ms
    }
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            enable_auto_commit: false,
            auto_offset_reset: "earliest".to_string(),
            session_timeout: None,
            queued_buffering_max_messages_kbytes: None,
            batch_num_messages: None,
            queue_buffering_max_messages: None,
            queue_buffering_max_kbytes: None,
            queue_buffering_max_ms: None,
        }
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
        let consumer_config = KafkaConfig::new();
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
        let consumer_config = KafkaConfig::new();
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
        let producer_config = KafkaConfig::new();
        assert_eq!(producer_config.batch_num_messages(), Some(1000));
        assert_eq!(producer_config.queue_buffering_max_messages(), Some(1000));
        assert_eq!(producer_config.queue_buffering_max_kbytes(), Some(1000));
        assert_eq!(producer_config.queue_buffering_max_ms(), Some(1000));
        env::remove_var(VAR_KAFKA_PRODUCER_BATCH_NUM_MESSAGES);
        env::remove_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES);
        env::remove_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES);
        env::remove_var(VAR_KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS);
    }
}
