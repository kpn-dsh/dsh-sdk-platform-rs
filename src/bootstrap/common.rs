//! Common/public functions for bootstrap
//!
//! This module contains public functions to bootstrap to DSH and get infomaion from the datastreams

use log::{info, warn};
use std::{collections::HashMap, env};

use super::{dsh::Cert, Bootstrap, Datastream, GroupType, KafkaProperties, ReadWriteAccess};
use crate::error::DshError;

impl Bootstrap {
    /// Create a new Bootstrap struct that contains all information and certificates.
    /// needed to connect to Kafka and DSH.
    ///
    ///  - Contains a struct similar to datastreams.json
    ///  - Metadata of running container/task
    ///  - Certificates for Kafka and DSH
    ///
    /// If running locally, it will try to load the local_datastreams.json file
    /// and base the Bootstrap struct on this file
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
    pub async fn new() -> Result<Bootstrap, DshError> {
        #[cfg(not(feature = "local"))]
        Bootstrap::new_dsh().await;
        #[cfg(feature = "local")]
        match Bootstrap::new_dsh().await {
            Ok(b) => Ok(b),
            Err(e) => {
                eprintln!(
                    "Could not connect to DSH, start loading local settings\nError: {}",
                    e
                );
                Bootstrap::new_local()
            }
        }
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
    /// use rdkafka::config::RDKafkaLogLevel;
    /// use rdkafka::consumer::stream_consumer::StreamConsumer;
    /// use dsh_sdk::bootstrap::Bootstrap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let bootstrap = Bootstrap::new().await.expect("Bootstrap failed");
    ///     let mut consumer_config = bootstrap.consumer_rdkafka_config();
    ///
    ///     // Optional: set/overwrite extra config per your requirements
    ///     consumer_config.set("auto.offset.reset", "latest");
    ///
    ///     // Build the consumer
    ///     let consumer: StreamConsumer =  consumer_config.create().expect("Consumer creation failed");
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
    /// | `security.protocol`       | ssl (DSH))<br>plaintext (local)        | Security protocol                                                                                                                                                    |
    /// | `ssl.key.pem`             | private key                            | Generated when bootstrap is initiated                                                                                                                                |
    /// | `ssl.certificate.pem`     | dsh kafka certificate                  | Signed certificate to connect to kafka cluster <br>(signed when bootstrap is initiated)                                                                              |
    /// | `ssl.ca.pem`              | CA certifacte                          | Root certificate, provided by DSH.                                                                                                                                   |
    /// | `log_level`               | Info                                   | Log level of rdkafka                                                                                                                                                 |
    ///
    pub fn consumer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.kafka_properties().get_brokers())
            .set(
                "group.id",
                self.kafka_properties()
                    .get_group_id(GroupType::get_from_env())
                    .expect("Group type not found"),
            )
            .set("client.id", self.get_client_id())
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
        config
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
    /// use rdkafka::config::RDKafkaLogLevel;
    /// use rdkafka::producer::FutureProducer;
    /// use dsh_sdk::bootstrap::Bootstrap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let bootstrap = Bootstrap::new().await.expect("Bootstrap failed");
    ///     let mut producer_config = bootstrap.producer_rdkafka_config();
    ///
    ///     // Optional: set/overwrite extra config per your requirements
    ///     producer_config.set("request.required.acks", "1");
    ///
    ///     // Build the consumer
    ///     let producer: FutureProducer =  producer_config.create().expect("Producer creation failed");
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

    pub fn producer_rdkafka_config(&self) -> rdkafka::config::ClientConfig {
        let mut config = rdkafka::config::ClientConfig::new();
        config
            .set("bootstrap.servers", self.kafka_properties().get_brokers())
            .set("client.id", self.get_client_id())
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

    /// Get the certificates. If running local it returns None
    pub fn certificates(&self) -> Option<Cert> {
        self.certificates.clone()
    }

    /// Get the client id based on the task id.
    pub fn get_client_id(&self) -> &str {
        self.client_id.as_str()
    }

    /// Get the kafka properties provided by DSH (datastreams.json)
    pub fn kafka_properties(&self) -> &KafkaProperties {
        &self.kafka_properties
    }
}

impl KafkaProperties {
    /// Get the brokers as comma seperated string from the datastreams
    pub fn get_brokers(&self) -> String {
        self.brokers.clone().join(", ")
    }

    /// Get the group id from the datastreams based on GroupType
    ///
    /// # Error
    /// If the group type is not found in the datastreams
    /// (index out of bounds)
    pub fn get_group_id(&self, group_type: GroupType) -> Result<String, DshError> {
        let group_id = match group_type {
            GroupType::Private(i) => self.private_consumer_groups.get(i),

            GroupType::Shared(i) => self.shared_consumer_groups.get(i),
        };
        info!("Kafka group id: {:?}", group_id);
        match group_id {
            Some(id) => Ok(id.to_string()),
            None => Err(DshError::IndexGroupIdError(group_type)),
        }
    }

    /// Get schema host from datastreams info.
    ///
    /// Overwritable with environment variable SCHEMA_REGISTRY_HOST, if set
    pub fn get_schema_registry_host(&self) -> String {
        env::var("SCHEMA_REGISTRY_HOST").unwrap_or(self.schema_store.clone())
    }

    /// Get all available datastreams
    pub fn get_datastreams(&self) -> &HashMap<String, Datastream> {
        &self.streams
    }

    /// Get a specific datastream based on the topic name
    /// If the topic is not found, it will return None
    pub fn get_datastream(&self, topic: &str) -> Option<&Datastream> {
        // if topic name contains 2 dots, get the first 2 parts of the topic name
        // this is needed because the topic name in datastreams.json is only the first 2 parts
        let topic_name = topic.split('.').take(2).collect::<Vec<&str>>().join(".");
        self.get_datastreams().get(&topic_name)
    }

    /// Check if a list of topics is present in the read topics of datastreams
    pub fn verify_list_of_topics<T: std::fmt::Display>(
        &self,
        topics: Vec<T>,
        access: ReadWriteAccess,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let read_topics = self
            .get_datastreams()
            .values()
            .map(|datastream| match access {
                ReadWriteAccess::Read => datastream
                    .read
                    .split('.')
                    .take(2)
                    .collect::<Vec<&str>>()
                    .join(".")
                    .replace('\\', ""),
                ReadWriteAccess::Write => datastream
                    .write
                    .split('.')
                    .take(2)
                    .collect::<Vec<&str>>()
                    .join(".")
                    .replace('\\', ""),
            })
            .collect::<Vec<String>>();
        for topic in topics {
            let topic_name = topic
                .to_string()
                .split('.')
                .take(2)
                .collect::<Vec<&str>>()
                .join(".");
            if !read_topics.contains(&topic_name) {
                return Err(format!(
                    "Topic {} not found in datastreams.json provided by DSH",
                    topic
                )
                .into());
            }
        }
        Ok(())
    }
}

impl Datastream {
    /// Check read access on topic bases on datastream
    pub fn check_read_access(&self) -> bool {
        !self.read.is_empty()
    }

    /// Check write access on topic bases on datastream
    pub fn check_write_access(&self) -> bool {
        !self.write.is_empty()
    }
}

impl GroupType {
    /// Get the group type from the environment variable KAFKA_CONSUMER_GROUP_TYPE
    /// If KAFKA_CONSUMER_GROUP_TYPE is not (properly) set, it defaults to private
    pub fn get_from_env() -> Self {
        let group_type = env::var("KAFKA_CONSUMER_GROUP_TYPE");
        match group_type {
            Ok(s) if s == *"private" => GroupType::Private(0),
            Ok(s) if s == *"shared" => GroupType::Shared(0),
            _ => {
                warn!("KAFKA_CONSUMER_GROUP_TYPE is not set correctly, defaulting to private");
                GroupType::Private(0)
            }
        }
    }
}

/// Get the configured topics from the environment variable TOPICS
/// The set topics can be delimited by a comma
pub fn get_configured_topics() -> Vec<String> {
    let kafka_topic_string = match env::var("TOPICS") {
        Ok(s) => s,
        Err(_) => {
            warn!("TOPICS environment variable not set");
            return vec![];
        }
    };
    kafka_topic_string
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a reusable KafkaProperties instance
    fn kafka_props() -> KafkaProperties {
        let kafka_properties: KafkaProperties =
            serde_json::from_str(datastreams_json().as_str()).unwrap();
        kafka_properties
    }

    // maybe replace with local_datastreams.json?
    fn datastreams_json() -> String {
        serde_json::json!({
          "brokers": [
            "broker-0.tt.kafka.mesos:9091",
            "broker-1.tt.kafka.mesos:9091",
            "broker-2.tt.kafka.mesos:9091"
          ],
          "streams": {
            "scratch.test": {
              "name": "scratch.test",
              "cluster": "/tt",
              "read": "scratch.test.test-tenant",
              "write": "scratch.test.test-tenant",
              "partitions": 3,
              "replication": 1,
              "partitioner": "default-partitioner",
              "partitioningDepth": 0,
              "canRetain": false
            },
            "stream.test": {
              "name": "stream.test",
              "cluster": "/tt",
              "read": "stream\\.test\\.[^.]*",
              "write": "",
              "partitions": 1,
              "replication": 1,
              "partitioner": "default-partitioner",
              "partitioningDepth": 0,
              "canRetain": false
            }
          },
          "private_consumer_groups": [
            "test-app.7e93a513-6556-11eb-841e-f6ab8576620c_1",
            "test-app.7e93a513-6556-11eb-841e-f6ab8576620c_2",
            "test-app.7e93a513-6556-11eb-841e-f6ab8576620c_3",
            "test-app.7e93a513-6556-11eb-841e-f6ab8576620c_4"
          ],
          "shared_consumer_groups": [
            "test-app_1",
            "test-app_2",
            "test-app_3",
            "test-app_4"
          ],
          "non_enveloped_streams": [],
          "schema_store": "http://schema-registry.tt.kafka.mesos:8081"
        })
        .to_string()
    }

    #[test]
    fn test_kafka_prop_get_brokers() {
        assert_eq!(
            kafka_props().get_brokers(),
            String::from("broker-0.tt.kafka.mesos:9091, broker-1.tt.kafka.mesos:9091, broker-2.tt.kafka.mesos:9091")
        );
    }

    #[test]
    fn test_kafka_prop_verify_list_of_topics() {
        let topics = vec![
            "scratch.test.test-tenant".to_string(),
            "stream.test.test-tenant".to_string(),
        ];
        kafka_props()
            .verify_list_of_topics(topics, ReadWriteAccess::Read)
            .unwrap()
    }

    #[test]
    fn test_kafka_prop_get_schema_registry_host() {
        assert_eq!(
            kafka_props().get_schema_registry_host(),
            String::from("http://schema-registry.tt.kafka.mesos:8081")
        );
    }

    #[test]
    fn test_kafka_prop_get_group_type_from_env() {
        // Set the KAFKA_CONSUMER_GROUP_TYPE environment variable to "private"
        env::set_var("KAFKA_CONSUMER_GROUP_TYPE", "private");
        assert_eq!(GroupType::get_from_env(), GroupType::Private(0),);
        env::set_var("KAFKA_CONSUMER_GROUP_TYPE", "shared");
        assert_eq!(GroupType::get_from_env(), GroupType::Shared(0),);
        env::set_var("KAFKA_CONSUMER_GROUP_TYPE", "invalid-type");
        assert_eq!(GroupType::get_from_env(), GroupType::Private(0),);
        env::remove_var("KAFKA_CONSUMER_GROUP_TYPE");
        assert_eq!(GroupType::get_from_env(), GroupType::Private(0),);
    }

    #[test]
    fn test_kafka_prop_get_group_id() {
        assert_eq!(
            kafka_props().get_group_id(GroupType::Private(0)).unwrap(),
            "test-app.7e93a513-6556-11eb-841e-f6ab8576620c_1",
            "KAFKA_CONSUMER_GROUP_TYPE is set to private, but did not return test-app.7e93a513-6556-11eb-841e-f6ab8576620c_1"
        );
        assert_eq!(
            kafka_props().get_group_id(GroupType::Shared(0)).unwrap(),
            "test-app_1",
            "KAFKA_CONSUMER_GROUP_TYPE is set to shared, but did not return test-app_1"
        );
        assert_eq!(
            kafka_props().get_group_id(GroupType::Shared(3)).unwrap(),
            "test-app_4",
            "KAFKA_CONSUMER_GROUP_TYPE is set to shared, but did not return test-app_1"
        );
        assert!(kafka_props()
            .get_group_id(GroupType::Private(1000))
            .is_err(),);
    }

    #[test]
    fn test_check_access_read_topic() {
        assert_eq!(
            kafka_props()
                .get_datastream("scratch.test.test-tenant")
                .unwrap()
                .check_read_access(),
            true
        );
        assert_eq!(
            kafka_props()
                .get_datastream("stream.test.test-tenant")
                .unwrap()
                .check_read_access(),
            true
        );
    }

    #[test]
    fn test_check_access_write_topic() {
        assert_eq!(
            kafka_props()
                .get_datastream("scratch.test.test-tenant")
                .unwrap()
                .check_write_access(),
            true
        );
        assert_eq!(
            kafka_props()
                .get_datastream("stream.test.test-tenant")
                .unwrap()
                .check_write_access(),
            false
        );
    }

    #[test]
    fn test_get_configured_topics() {
        std::env::set_var("TOPICS", "topic1, topic2, topic3");

        let topics = get_configured_topics();

        assert_eq!(topics.len(), 3);
        assert_eq!(topics[0], "topic1");
        assert_eq!(topics[1], "topic2");
        assert_eq!(topics[2], "topic3");

        std::env::remove_var("TOPICS");

        let topics = get_configured_topics();
        assert_eq!(topics.len(), 0);
    }
}
