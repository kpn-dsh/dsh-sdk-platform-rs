//! Module to handle the datastreams.json file.
//! 
//! The datastreams.json can be parsed into a Datastream struct using serde_json.
//! This struct contains all the information from the datastreams.json file.
//! 
//! You can get the Datastream struct via the 'Properties' struct.
//! 
//! # Example
//! ```
//! use dsh_sdk::Properties;
//! 
//! let properties = Properties::get();
//! let datastream = properties.datastream();
//! 
//! let brokers = datastream.get_brokers();
//! let schema_store = datastream.schema_store();
//! ```
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use super::{
    utils, VAR_KAFKA_BOOTSTRAP_SERVERS, VAR_KAFKA_CONSUMER_GROUP_TYPE, VAR_SCHEMA_REGISTRY_HOST,
};
use crate::error::DshError;

const FILE_NAME: &str = "local_datastreams.json";

/// This struct is equivalent to the datastreams.json
/// 
/// # Example
/// ```
/// use dsh_sdk::Properties;
/// 
/// let properties = Properties::get();
/// let datastream = properties.datastream();
/// 
/// let brokers = datastream.get_brokers();
/// let streams = datastream.streams();
/// let schema_store = datastream.schema_store();
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Datastream {
    brokers: Vec<String>,
    streams: HashMap<String, Stream>,
    private_consumer_groups: Vec<String>,
    shared_consumer_groups: Vec<String>,
    non_enveloped_streams: Vec<String>,
    schema_store: String,
}

impl Datastream {
    /// Get the kafka brokers from the datastreams as a vector of strings
    pub fn get_brokers(&self) -> Vec<&str> {
        self.brokers.iter().map(|s| s.as_str()).collect()
    }

    /// Get the kafka brokers as comma seperated string from the datastreams
    pub fn get_brokers_string(&self) -> String {
        self.brokers.join(", ")
    }

    /// Get the group id from the datastreams based on GroupType
    ///
    /// # Error
    /// If the index is greater then amount of groups in the datastreams
    /// (index out of bounds)
    pub fn get_group_id(&self, group_type: GroupType) -> Result<&str, DshError> {
        let group_id = match group_type {
            GroupType::Private(i) => self.private_consumer_groups.get(i),
            GroupType::Shared(i) => self.shared_consumer_groups.get(i),
        };
        match group_id {
            Some(id) => Ok(id),
            None => Err(DshError::IndexGroupIdError(group_type)),
        }
    }

    /// Get all available datastreams (scratch topics, internal topics and stream topics)
    pub fn streams(&self) -> &HashMap<String, Stream> {
        &self.streams
    }

    /// Get a specific datastream based on the topic name
    /// If the topic is not found, it will return None
    pub fn get_stream(&self, topic: &str) -> Option<&Stream> {
        // if topic name contains 2 dots, get the first 2 parts of the topic name
        // this is needed because the topic name in datastreams.json is only the first 2 parts
        let topic_name = topic.split('.').take(2).collect::<Vec<&str>>().join(".");
        self.streams().get(&topic_name)
    }

    /// Check if a list of topics is present in the read topics of datastreams
    pub fn verify_list_of_topics<T: std::fmt::Display>(
        &self,
        topics: &Vec<T>,
        access: ReadWriteAccess,
    ) -> Result<(), DshError> {
        let read_topics = self
            .streams()
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
                return Err(DshError::NotFoundTopicError(topic.to_string()));
            }
        }
        Ok(())
    }

    /// Get schema store url from datastreams.
    /// 
    /// ## How to connect to schema registry
    /// Use the Reqwest client from `Cert` to connect to the schema registry.
    /// As this client is already configured with the correct certificates.
    /// 
    /// You can use [schema_registry_converter](https://crates.io/crates/schema_registry_converter) 
    /// to fetch the schema and decode your payload.
    pub fn schema_store(&self) -> &str {
        &self.schema_store
    }

    /// Write datastreams.json in a directory
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::dsh::datastream::Datastream;
    /// # let datastream = Datastream::default();
    /// let path = std::path::PathBuf::from("/path/to/directory");
    /// datastream.to_file(&path).unwrap();
    /// ```
    pub fn to_file(&self, path: &std::path::Path) -> Result<(), DshError> {
        let json_string = serde_json::to_string_pretty(self)?;
        std::fs::write(path.join("datastreams.json"), json_string)?;
        info!("File created ({})", path.display());
        Ok(())
    }

    pub(crate) fn load_local_datastreams() -> Result<Self, DshError> {
        let path_buf: std::path::PathBuf = std::env::current_dir().unwrap().join(FILE_NAME);
        debug!("Reading local datastreams from {}", path_buf.display());
        let file_result = File::open(&path_buf);
        let mut file = match file_result {
            Ok(file) => file,
            Err(e) => {
                debug!(
                    "Failed opening local_datastreams.json ({}): {}",
                    path_buf.display(),
                    e
                );
                return Err(DshError::IoError(e));
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut datastream: Datastream = serde_json::from_str(&contents)?;
        if let Ok(brokers) = utils::get_env_var(VAR_KAFKA_BOOTSTRAP_SERVERS) {
            datastream.brokers = brokers.split(',').map(|s| s.to_string()).collect();
        }
        Ok(datastream)
    }
}

impl Default for Datastream {
    fn default() -> Self {
        let group_id = format!(
            "{}_default_group",
            utils::tenant_name().unwrap_or("local".to_string())
        );
        let brokers = if let Ok(brokers) = utils::get_env_var(VAR_KAFKA_BOOTSTRAP_SERVERS) {
            brokers.split(',').map(|s| s.to_string()).collect()
        } else {
            vec!["localhost:9092".to_string()]
        };
        let schema_store = utils::get_env_var(VAR_SCHEMA_REGISTRY_HOST)
            .unwrap_or("http://localhost:8081/apis/ccompat/v7".to_string());
        Datastream {
            brokers,
            streams: HashMap::new(),
            private_consumer_groups: vec![group_id.clone()],
            shared_consumer_groups: vec![group_id],
            non_enveloped_streams: Vec::new(),
            schema_store,
        }
    }
}

/// Struct containing all topic information which also is provided in datastreams.json
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    name: String,
    cluster: String,
    read: String,
    write: String,
    partitions: i32,
    replication: i32,
    partitioner: String,
    partitioning_depth: i32,
    can_retain: bool,
}

impl Stream {
    /// Check read access on topic based on datastream
    pub fn read_access(&self) -> bool {
        !self.read.is_empty()
    }

    /// Check write access on topic based on datastream
    pub fn write_access(&self) -> bool {
        !self.write.is_empty()
    }

    /// Get the Stream's Read whitelist pattern
    pub fn read_pattern(&self) -> Result<&str, DshError> {
        if self.read_access() {
            Ok(&self.read)
        } else {
            Err(DshError::TopicPermissionsError(
                self.name.clone(),
                ReadWriteAccess::Read,
            ))
        }
    }

    /// Get the Stream's Write pattern
    pub fn write_pattern(&self) -> Result<&str, DshError> {
        if self.write_access() {
            Ok(&self.write)
        } else {
            Err(DshError::TopicPermissionsError(
                self.name.clone(),
                ReadWriteAccess::Write,
            ))
        }
    }
}

/// Enum to indicate if we want to check the read or write topics
#[derive(Debug, Clone, PartialEq)]
pub enum ReadWriteAccess {
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub enum GroupType {
    Private(usize),
    Shared(usize),
}

impl GroupType {
    /// Get the group type from the environment variable KAFKA_CONSUMER_GROUP_TYPE
    /// If KAFKA_CONSUMER_GROUP_TYPE is not (properly) set, it defaults to shared
    pub fn from_env() -> Self {
        let group_type = env::var(VAR_KAFKA_CONSUMER_GROUP_TYPE);
        match group_type {
            Ok(s) if s.to_lowercase() == *"private" => GroupType::Private(0),
            Ok(s) if s.to_lowercase() == *"shared" => GroupType::Shared(0),
            Ok(_) => {
                error!("KAFKA_CONSUMER_GROUP_TYPE is not set with \"shared\" or \"private\", defaulting to shared group type.");
                GroupType::Shared(0)
            }
            Err(_) => {
                warn!("KAFKA_CONSUMER_GROUP_TYPE is not set, defaulting to shared group type.");
                GroupType::Shared(0)
            }
        }
    }
}

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupType::Private(i) => write!(f, "private; index: {}", i),
            GroupType::Shared(i) => write!(f, "shared; index: {}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a reusable Properties instance
    fn datastream() -> Datastream {
        let data_stream: Datastream = serde_json::from_str(datastreams_json().as_str()).unwrap();
        data_stream
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
    fn test_datastream_get_brokers() {
        assert_eq!(
            datastream().get_brokers(),
            vec![
                "broker-0.tt.kafka.mesos:9091",
                "broker-1.tt.kafka.mesos:9091",
                "broker-2.tt.kafka.mesos:9091"
            ]
        );
    }

    #[test]
    fn test_datastream_get_brokers_string() {
        assert_eq!(
            datastream().get_brokers_string(),
            "broker-0.tt.kafka.mesos:9091, broker-1.tt.kafka.mesos:9091, broker-2.tt.kafka.mesos:9091"
        );
    }

    #[test]
    fn test_datastream_verify_list_of_topics() {
        let topics = vec![
            "scratch.test.test-tenant".to_string(),
            "stream.test.test-tenant".to_string(),
        ];
        datastream()
            .verify_list_of_topics(&topics, ReadWriteAccess::Read)
            .unwrap()
    }

    #[test]
    fn test_datastream_get_schema_store() {
        assert_eq!(
            datastream().schema_store(),
            "http://schema-registry.tt.kafka.mesos:8081"
        );
    }

    #[test]
    fn test_datastream_get_group_type_from_env() {
        // Set the KAFKA_CONSUMER_GROUP_TYPE environment variable to "private"
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "private");
        assert_eq!(GroupType::from_env(), GroupType::Private(0),);
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "shared");
        assert_eq!(GroupType::from_env(), GroupType::Shared(0),);
        env::set_var(VAR_KAFKA_CONSUMER_GROUP_TYPE, "invalid-type");
        assert_eq!(GroupType::from_env(), GroupType::Shared(0),);
        env::remove_var(VAR_KAFKA_CONSUMER_GROUP_TYPE);
        assert_eq!(GroupType::from_env(), GroupType::Shared(0),);
    }

    #[test]
    fn test_datastream_get_group_id() {
        assert_eq!(
            datastream().get_group_id(GroupType::Private(0)).unwrap(),
            "test-app.7e93a513-6556-11eb-841e-f6ab8576620c_1",
            "KAFKA_CONSUMER_GROUP_TYPE is set to private, but did not return test-app.7e93a513-6556-11eb-841e-f6ab8576620c_1"
        );
        assert_eq!(
            datastream().get_group_id(GroupType::Shared(0)).unwrap(),
            "test-app_1",
            "KAFKA_CONSUMER_GROUP_TYPE is set to shared, but did not return test-app_1"
        );
        assert_eq!(
            datastream().get_group_id(GroupType::Shared(3)).unwrap(),
            "test-app_4",
            "KAFKA_CONSUMER_GROUP_TYPE is set to shared, but did not return test-app_1"
        );
        assert!(datastream().get_group_id(GroupType::Private(1000)).is_err(),);
    }

    #[test]
    fn test_datastream_check_access_read_topic() {
        assert_eq!(
            datastream()
                .get_stream("scratch.test.test-tenant")
                .unwrap()
                .read_access(),
            true
        );
        assert_eq!(
            datastream()
                .get_stream("stream.test.test-tenant")
                .unwrap()
                .read_access(),
            true
        );
    }

    #[test]
    fn test_datastream_check_access_write_topic() {
        assert_eq!(
            datastream()
                .get_stream("scratch.test.test-tenant")
                .unwrap()
                .write_access(),
            true
        );
        assert_eq!(
            datastream()
                .get_stream("stream.test.test-tenant")
                .unwrap()
                .write_access(),
            false
        );
    }

    #[test]
    fn test_datastream_check_read_topic() {
        assert_eq!(
            datastream()
                .get_stream("scratch.test.test-tenant")
                .unwrap()
                .read_pattern()
                .unwrap(),
            "scratch.test.test-tenant"
        );
        assert_eq!(
            datastream()
                .get_stream("stream.test.test-tenant")
                .unwrap()
                .read_pattern()
                .unwrap(),
            "stream\\.test\\.[^.]*"
        );
    }

    #[test]
    fn test_datastream_check_write_topic() {
        assert_eq!(
            datastream()
                .get_stream("scratch.test.test-tenant")
                .unwrap()
                .write_pattern()
                .unwrap(),
            "scratch.test.test-tenant"
        );
        let e = datastream()
            .get_stream("stream.test.test-tenant")
            .unwrap()
            .write_pattern()
            .unwrap_err();

        assert!(matches!(
            e,
            DshError::TopicPermissionsError(_, ReadWriteAccess::Write)
        ));
    }

    #[test]
    fn test_to_file() {
        let test_path = std::path::PathBuf::from("test_files");
        let result = datastream().to_file(&test_path);
        assert!(result.is_ok())
    }
}
