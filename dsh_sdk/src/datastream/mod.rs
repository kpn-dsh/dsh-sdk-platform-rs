//! Datastream properties
//!
//! The datastreams.json can be parsed into a Datastream struct using serde_json.
//! This struct contains all the information from the datastreams properties file.
//!
//! # Example
//! ```no_run
//! use dsh_sdk::Dsh;
//!
//! # #[tokio::main]
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let dsh = Dsh::get();
//! let datastream = dsh.datastream(); // immutable datastream which is fetched at initialization of SDK
//! // Or
//! let datastream = dsh.fetch_datastream().await?; // fetch a fresh datastream from dsh server
//!
//! let brokers = datastream.get_brokers();
//! let schema_store_url = datastream.schema_store();
//! # Ok(())
//! }
//! ```
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{
    utils, VAR_KAFKA_BOOTSTRAP_SERVERS, VAR_KAFKA_CONSUMER_GROUP_TYPE, VAR_LOCAL_DATASTREAMS_JSON,
    VAR_SCHEMA_REGISTRY_HOST,
};
#[doc(inline)]
pub use error::DatastreamError;

mod error;

const FILE_NAME: &str = "local_datastreams.json";

/// Datastream properties file
///
/// Read from datastreams.json
///
/// # Example
/// ```
/// use dsh_sdk::Dsh;
///
/// let properties = Dsh::get();
/// let datastream = properties.datastream();
///
/// let brokers = datastream.get_brokers();
/// let streams = datastream.streams();
/// let schema_store_url = datastream.schema_store();
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
    pub fn get_group_id(&self, group_type: GroupType) -> Result<&str, DatastreamError> {
        let group_id = match group_type {
            GroupType::Private(i) => self.private_consumer_groups.get(i),
            GroupType::Shared(i) => self.shared_consumer_groups.get(i),
        };
        match group_id {
            Some(id) => Ok(id),
            None => Err(DatastreamError::IndexGroupIdError(group_type)),
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
    ) -> Result<(), DatastreamError> {
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
                return Err(DatastreamError::NotFoundTopicError(topic.to_string()));
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
    /// # use dsh_sdk::datastream::Datastream;
    /// # let datastream = Datastream::default();
    /// let path = std::path::PathBuf::from("/path/to/directory");
    /// datastream.to_file(&path).unwrap();
    /// ```
    pub fn to_file(&self, path: &std::path::Path) -> Result<(), DatastreamError> {
        let json_string = serde_json::to_string_pretty(self)?;
        std::fs::write(path.join("datastreams.json"), json_string)?;
        info!("File created ({})", path.display());
        Ok(())
    }

    /// Fetch datastreams from the dsh server (async)
    ///
    /// Make sure you use a Reqwest client from `Cert` to connect to the dsh server.
    /// As this client is already configured with the correct certificates.
    pub async fn fetch(
        client: &reqwest::Client,
        host: &str,
        tenant: &str,
        task_id: &str,
    ) -> Result<Self, DatastreamError> {
        let url = Self::datastreams_endpoint(host, tenant, task_id);
        let response = client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(DatastreamError::DshCallError {
                url,
                status_code: response.status(),
                error_body: response.text().await.unwrap_or_default(),
            });
        }
        Ok(response.json().await?)
    }

    /// Fetch datastreams from the dsh server (blocking)
    ///
    /// Make sure you use a Reqwest client from `Cert` to connect to the dsh server.
    /// As this client is already configured with the correct certificates.
    pub fn fetch_blocking(
        client: &reqwest::blocking::Client,
        host: &str,
        tenant: &str,
        task_id: &str,
    ) -> Result<Self, DatastreamError> {
        let url = Self::datastreams_endpoint(host, tenant, task_id);
        let response = client.get(&url).send()?;
        if !response.status().is_success() {
            return Err(DatastreamError::DshCallError {
                url,
                status_code: response.status(),
                error_body: response.text().unwrap_or_default(),
            });
        }
        Ok(response.json()?)
    }

    pub(crate) fn datastreams_endpoint(host: &str, tenant: &str, task_id: &str) -> String {
        format!("{}/kafka/config/{}/{}", host, tenant, task_id)
    }

    /// If local_datastreams.json is found, it will load the datastreams from this file.
    /// If it does not parse or the file is not found based on on Environment Variable, it will panic.
    /// If the Environment Variable is not set, it will look in the current directory. If it is not found,
    /// it will return a Error on the Result. Based on this it will use default Datastreams.
    pub(crate) fn load_local_datastreams() -> Result<Self, DatastreamError> {
        let path_buf = if let Ok(path) = utils::get_env_var(VAR_LOCAL_DATASTREAMS_JSON) {
            let path = std::path::PathBuf::from(path);
            if !path.exists() {
                panic!("{} not found", path.display());
            } else {
                path
            }
        } else {
            std::env::current_dir().unwrap().join(FILE_NAME)
        };
        debug!("Reading local datastreams from {}", path_buf.display());
        let mut file = File::open(&path_buf).map_err(|e| {
            debug!(
                "Failed opening local_datastreams.json ({}): {}",
                path_buf.display(),
                e
            );
            DatastreamError::IoError(e)
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut datastream: Datastream = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse {}, {:?}", path_buf.display(), e));
        if let Ok(brokers) = utils::get_env_var(VAR_KAFKA_BOOTSTRAP_SERVERS) {
            datastream.brokers = brokers.split(',').map(|s| s.to_string()).collect();
        }
        if let Ok(schema_store) = utils::get_env_var(VAR_SCHEMA_REGISTRY_HOST) {
            datastream.schema_store = schema_store;
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
    /// Get the Stream's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the Stream's cluster
    pub fn cluster(&self) -> &str {
        &self.cluster
    }

    /// Get the read pattern as stated in datastreams.
    ///
    /// Use `read_pattern` method to validate if read access is allowed.
    pub fn read(&self) -> &str {
        &self.read
    }

    /// Get the write pattern
    ///
    /// Use `write_pattern` method to validate if write access is allowed.
    pub fn write(&self) -> &str {
        &self.write
    }

    /// Get the Stream's number of partitions
    pub fn partitions(&self) -> i32 {
        self.partitions
    }

    /// Get the Stream's replication factor
    pub fn replication(&self) -> i32 {
        self.replication
    }

    /// Get the Stream's partitioner
    pub fn partitioner(&self) -> &str {
        &self.partitioner
    }

    /// Get the Stream's partitioning depth
    pub fn partitioning_depth(&self) -> i32 {
        self.partitioning_depth
    }

    /// Get the Stream's can retain value
    pub fn can_retain(&self) -> bool {
        self.can_retain
    }

    /// Check read access on topic based on datastream
    pub fn read_access(&self) -> bool {
        !self.read.is_empty()
    }

    /// Check write access on topic based on datastream
    pub fn write_access(&self) -> bool {
        !self.write.is_empty()
    }

    /// Get the Stream's Read whitelist pattern
    ///
    /// ## Error
    /// If the topic does not have read access it returns a `TopicPermissionsError`
    pub fn read_pattern(&self) -> Result<&str, DatastreamError> {
        if self.read_access() {
            Ok(&self.read)
        } else {
            Err(DatastreamError::TopicPermissionsError(
                self.name.clone(),
                ReadWriteAccess::Read,
            ))
        }
    }

    /// Get the Stream's Write pattern
    ///
    /// ## Error
    /// If the topic does not have write access it returns a `TopicPermissionsError`
    pub fn write_pattern(&self) -> Result<&str, DatastreamError> {
        if self.write_access() {
            Ok(&self.write)
        } else {
            Err(DatastreamError::TopicPermissionsError(
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

/// Enum to indicate the group type (private or shared)
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
                debug!("KAFKA_CONSUMER_GROUP_TYPE is not set, defaulting to shared group type.");
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
    use serial_test::serial;

    // Define a reusable Properties instance
    fn datastream() -> Datastream {
        serde_json::from_str(datastreams_json().as_str()).unwrap()
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

    #[test]
    fn test_name() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.name(), "scratch.test");
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.name(), "stream.test");
    }

    #[test]
    fn test_read() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.read(), "scratch.test.test-tenant");
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.read(), "stream\\.test\\.[^.]*");
    }

    #[test]
    fn test_write() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.write(), "scratch.test.test-tenant");
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.write(), "");
    }

    #[test]
    fn test_cluster() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.cluster(), "/tt");
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.cluster(), "/tt");
    }

    #[test]
    fn test_partitions() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.partitions(), 3);
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.partitions(), 1);
    }

    #[test]
    fn test_replication() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.replication(), 1);
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.replication(), 1);
    }

    #[test]
    fn test_partitioner() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.partitioner(), "default-partitioner");
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.partitioner(), "default-partitioner");
    }

    #[test]
    fn test_partitioning_depth() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.partitioning_depth(), 0);
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.partitioning_depth(), 0);
    }

    #[test]
    fn test_can_retain() {
        let datastream = datastream();
        let stream = datastream.streams().get("scratch.test").unwrap();
        assert_eq!(stream.can_retain(), false);
        let stream = datastream.streams().get("stream.test").unwrap();
        assert_eq!(stream.can_retain(), true);
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
    #[serial(env_dependency)]
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
            DatastreamError::TopicPermissionsError(_, ReadWriteAccess::Write)
        ));
    }

    #[test]
    fn test_to_file() {
        let test_path = std::path::PathBuf::from("test_files");
        let result = datastream().to_file(&test_path);
        assert!(result.is_ok())
    }

    #[test]
    #[serial(env_dependency)]
    fn test_load_local_valid_datastreams() {
        // load from root directory
        let datastream = Datastream::load_local_datastreams().is_ok();
        assert!(datastream);
        // load from custom directory
        let current_dir = env::current_dir().unwrap();
        let file_location = format!(
            "{}/test_resources/valid_datastreams.json",
            current_dir.display()
        );
        println!("file_location: {}", file_location);
        env::set_var(VAR_LOCAL_DATASTREAMS_JSON, file_location);
        let datastream = Datastream::load_local_datastreams().is_ok();
        assert!(datastream);
        env::remove_var(VAR_LOCAL_DATASTREAMS_JSON);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_load_local_nonexisting_datastreams() {
        let current_dir = env::current_dir().unwrap();
        let file_location = format!(
            "{}/test_resoources/nonexisting_datastreams.json",
            current_dir.display()
        );
        env::set_var(VAR_LOCAL_DATASTREAMS_JSON, file_location);
        // let it panic in a thread
        let join_handle = std::thread::spawn(move || {
            let _ = Datastream::load_local_datastreams();
        });
        let result = join_handle.join();
        assert!(result.is_err());
        env::remove_var(VAR_LOCAL_DATASTREAMS_JSON);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_load_local_invalid_datastreams() {
        let current_dir = env::current_dir().unwrap();
        let file_location = format!(
            "{}/test_resources/invalid_datastreams.json",
            current_dir.display()
        );
        env::set_var(VAR_LOCAL_DATASTREAMS_JSON, file_location);
        // let it panic in a thread
        let join_handle = std::thread::spawn(move || {
            let _ = Datastream::load_local_datastreams();
        });
        let result = join_handle.join();
        assert!(result.is_err());
        env::remove_var(VAR_LOCAL_DATASTREAMS_JSON);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_load_local_invalid_json() {
        let current_dir = env::current_dir().unwrap();
        let file_location = format!(
            "{}/test_resources/invalid_datastreams_missing_field.json",
            current_dir.display()
        );
        env::set_var(VAR_LOCAL_DATASTREAMS_JSON, file_location);
        // let it panic in a thread
        let join_handle = std::thread::spawn(move || {
            let _ = Datastream::load_local_datastreams();
        });
        let result = join_handle.join();
        assert!(result.is_err());
        env::remove_var(VAR_LOCAL_DATASTREAMS_JSON);
    }

    #[test]
    fn test_datastream_endpoint() {
        let host = "http://localhost:8080";
        let tenant = "test-tenant";
        let task_id = "test-task-id";
        let endpoint = Datastream::datastreams_endpoint(host, tenant, task_id);
        assert_eq!(
            endpoint,
            "http://localhost:8080/kafka/config/test-tenant/test-task-id"
        );
    }

    #[tokio::test]
    async fn test_fetch() {
        let mut dsh = mockito::Server::new_async().await;
        let tenant = "test-tenant";
        let task_id = "test-task-id";
        let host = dsh.url();
        dsh.mock("GET", "/kafka/config/test-tenant/test-task-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(datastreams_json())
            .create();
        let client = reqwest::Client::new();
        let fetched_datastream = Datastream::fetch(&client, &host, tenant, task_id)
            .await
            .unwrap();
        assert_eq!(fetched_datastream, datastream());
    }

    #[test]
    fn test_fetch_blocking() {
        let mut dsh = mockito::Server::new();
        let tenant = "test-tenant";
        let task_id = "test-task-id";
        let host = dsh.url();
        dsh.mock("GET", "/kafka/config/test-tenant/test-task-id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(datastreams_json())
            .create();
        let client = reqwest::blocking::Client::new();
        let fetched_datastream =
            Datastream::fetch_blocking(&client, &host, tenant, task_id).unwrap();
        assert_eq!(fetched_datastream, datastream());
    }
}
