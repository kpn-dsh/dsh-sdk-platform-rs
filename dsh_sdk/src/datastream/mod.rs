//! Datastream properties for DSH.
//!
//! This module provides the [`Datastream`] struct, which represents the contents of
//! a `datastreams.json` file. This file contains the Kafka broker URLs, streams, consumer groups,
//! and additional metadata needed for interacting with DSH.  
//!
//! # Usage Overview
//! - **Local Loading**: By default, you can load `datastreams.json` from the local filesystem
//! if running on an environment outside of DSH.
//! - **Server Fetching**: You can also fetch an up-to-date `datastreams.json` from a DSH server (only works when running on DSH)
//!   using [`Datastream::fetch`] (async) or [`Datastream::fetch_blocking`] (blocking).  
//!
//! The [`Dsh`](crate::dsh::Dsh) struct uses these methods internally to provide either an
//! immutable, initialized `Datastream` or a freshly fetched copy.  
//!
//! # Example
//! ```no_run
//! use dsh_sdk::Dsh;
//!
//! # #[tokio::main]
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let dsh = Dsh::get();
//!
//! // An immutable Datastream, fetched at SDK initialization
//! let datastream = dsh.datastream();
//!
//! // Or fetch a new Datastream from the DSH server at runtime
//! let datastream = dsh.fetch_datastream().await?;
//!
//! let brokers = datastream.get_brokers();
//! let schema_store_url = datastream.schema_store();
//! # Ok(())
//! # }
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

/// Default filename for local datastream definitions.
const FILE_NAME: &str = "local_datastreams.json";

/// The main struct representing the datastream properties file (`datastreams.json`).
///
/// This file generally includes:
/// - A list of Kafka brokers  
/// - Configurable private/shared consumer groups  
/// - Mapping of topic names to [`Stream`] configurations  
/// - A Schema Store URL  
///
/// # Example
/// ```
/// use dsh_sdk::Dsh;
///
/// let dsh = Dsh::get();
/// let datastream = dsh.datastream(); // Typically loaded at init
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
    /// Returns a list of Kafka brokers (as `&str`) from this datastream configuration.
    pub fn get_brokers(&self) -> Vec<&str> {
        self.brokers.iter().map(|s| s.as_str()).collect()
    }

    /// Returns the Kafka brokers as a comma-separated string.
    pub fn get_brokers_string(&self) -> String {
        self.brokers.join(", ")
    }

    /// Returns the consumer group ID based on the specified [`GroupType`].
    ///
    /// # Errors
    /// Returns [`DatastreamError::IndexGroupIdError`] if the index is out of bounds or
    /// if no such group ID exists.
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

    /// Returns a reference to the map of all configured streams.
    ///
    /// Each entry typically corresponds to a topic or topic group in Kafka.
    pub fn streams(&self) -> &HashMap<String, Stream> {
        &self.streams
    }

    /// Looks up a specific stream by its topic name (truncating to the first two segments of the topic).
    ///
    /// If the topic is not found in the `streams` map, returns `None`.
    pub fn get_stream(&self, topic: &str) -> Option<&Stream> {
        let topic_name = topic.split('.').take(2).collect::<Vec<&str>>().join(".");
        self.streams().get(&topic_name)
    }

    /// Verifies that a list of topic names exist in either the `read` or `write` patterns
    /// (depending on the specified [`ReadWriteAccess`]).
    ///
    /// # Errors
    /// Returns [`DatastreamError::NotFoundTopicError`] if any provided topic is missing
    /// the required read or write patterns.
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

    /// Returns the schema store (registry) URL from this datastream configuration.
    ///
    /// # Connecting to the Schema Registry
    /// Use a [`reqwest::Client`] built from [`crate::certificates::Cert`] to connect securely.
    /// Tools like [`schema_registry_converter`](https://crates.io/crates/schema_registry_converter)
    /// can help fetch and decode messages.
    pub fn schema_store(&self) -> &str {
        &self.schema_store
    }

    /// Writes the current `Datastream` to a file named `datastreams.json` in the specified directory.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::datastream::Datastream;
    /// # let datastream = Datastream::default();
    /// let path = std::path::PathBuf::from("/path/to/directory");
    /// datastream.to_file(&path).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`DatastreamError::IoError`] if the file cannot be written.
    pub fn to_file(&self, path: &std::path::Path) -> Result<(), DatastreamError> {
        let json_string = serde_json::to_string_pretty(self)?;
        std::fs::write(path.join("datastreams.json"), json_string)?;
        info!("File created ({})", path.display());
        Ok(())
    }

    /// Asynchronously fetches a `Datastream` from the DSH server using a provided [`reqwest::Client`].
    ///
    /// The client should typically be built from [`crate::certificates::Cert::reqwest_client_config`]
    /// to include the required SSL certificates.
    ///
    /// # Errors
    /// Returns:
    /// - [`DatastreamError::DshCallError`] if the server responds with a non-success status code.
    /// - Any networking or deserialization errors wrapped by [`DatastreamError`].
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

    /// Fetches a `Datastream` from the DSH server in a **blocking** manner using a [`reqwest::blocking::Client`].
    ///
    /// The client should typically be built from [`crate::certificates::Cert::reqwest_blocking_client_config`]
    /// to include the required SSL certificates.
    ///
    /// # Errors
    /// Returns:
    /// - [`DatastreamError::DshCallError`] if the server responds with a non-success status code.
    /// - Any networking or deserialization errors wrapped by [`DatastreamError`].
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

    /// Constructs the URL endpoint for fetching datastreams from the DSH server.
    pub(crate) fn datastreams_endpoint(host: &str, tenant: &str, task_id: &str) -> String {
        format!("{}/kafka/config/{}/{}", host, tenant, task_id)
    }

    /// Attempts to load a local `datastreams.json` from either the current directory or
    /// from the path specified by the [`VAR_LOCAL_DATASTREAMS_JSON`] environment variable.
    ///
    /// If the file cannot be opened or parsed, the method will panic.  
    /// If the file isn’t found and no environment variable is set, returns an error wrapped in [`DatastreamError`].
    ///
    /// # Panics
    /// Panics if it finds a file but fails to parse valid JSON.
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
                "Failed to open local_datastreams.json ({}): {}",
                path_buf.display(),
                e
            );
            DatastreamError::IoError(e)
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut datastream: Datastream = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {:?}", path_buf.display(), e));

        // Allow env vars to override broker or schema store values
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
    /// Returns a `Datastream` with:
    /// - Default or environment-derived brokers
    /// - Placeholder consumer groups
    /// - A default schema store URL or the environment variable override
    ///
    /// Typically useful for local development if no `datastreams.json` is present.
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
            .unwrap_or_else(|_| "http://localhost:8081/apis/ccompat/v7".to_string());

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

/// Represents a single stream's information as provided by `datastreams.json`.
///
/// Includes topic names, partitioning information, read/write access patterns, and more.
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
    /// Returns this stream’s `name` field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns this stream’s `cluster` field (e.g., “/tt”).
    pub fn cluster(&self) -> &str {
        &self.cluster
    }

    /// Returns the read pattern (regex or exact topic name).
    ///
    /// Use [`Self::read_access`] or [`Self::read_pattern`] to confirm read permissions.
    pub fn read(&self) -> &str {
        &self.read
    }

    /// Returns the write pattern (regex or exact topic name).
    ///
    /// Use [`Self::write_access`] or [`Self::write_pattern`] to confirm write permissions.
    pub fn write(&self) -> &str {
        &self.write
    }

    /// Returns the number of partitions for this stream.
    pub fn partitions(&self) -> i32 {
        self.partitions
    }

    /// Returns the replication factor for this stream.
    pub fn replication(&self) -> i32 {
        self.replication
    }

    /// Returns the partitioner (e.g., “default-partitioner”).
    pub fn partitioner(&self) -> &str {
        &self.partitioner
    }

    /// Returns the partitioning depth (a more advanced Kafka concept).
    pub fn partitioning_depth(&self) -> i32 {
        self.partitioning_depth
    }

    /// Indicates whether data retention is possible for this stream.
    pub fn can_retain(&self) -> bool {
        self.can_retain
    }

    /// Checks if the stream has a `read` pattern configured.
    pub fn read_access(&self) -> bool {
        !self.read.is_empty()
    }

    /// Checks if the stream has a `write` pattern configured.
    pub fn write_access(&self) -> bool {
        !self.write.is_empty()
    }

    /// Returns the read pattern, or errors if the stream has no read access.
    ///
    /// # Errors
    /// Returns [`DatastreamError::TopicPermissionsError`] if the stream has no read pattern set.
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

    /// Returns the write pattern, or errors if the stream has no write access.
    ///
    /// # Errors
    /// Returns [`DatastreamError::TopicPermissionsError`] if the stream has no write pattern set.
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

/// Indicates whether the caller needs read or write access.
#[derive(Debug, Clone, PartialEq)]
pub enum ReadWriteAccess {
    Read,
    Write,
}

/// Specifies whether a consumer group is private or shared, along with an index
/// for selecting from the corresponding array in `Datastream`.
#[derive(Debug, PartialEq)]
pub enum GroupType {
    Private(usize),
    Shared(usize),
}

impl GroupType {
    /// Determines the group type from the `KAFKA_CONSUMER_GROUP_TYPE` environment variable,
    /// defaulting to [`GroupType::Shared(0)`] if unset or invalid.
    pub fn from_env() -> Self {
        let group_type = env::var(VAR_KAFKA_CONSUMER_GROUP_TYPE);
        match group_type {
            Ok(s) if s.eq_ignore_ascii_case("private") => GroupType::Private(0),
            Ok(s) if s.eq_ignore_ascii_case("shared") => GroupType::Shared(0),
            Ok(_) => {
                error!("KAFKA_CONSUMER_GROUP_TYPE is not set to \"shared\" or \"private\". Defaulting to shared group type.");
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
            GroupType::Private(i) => write!(f, "private; index: {i}"),
            GroupType::Shared(i) => write!(f, "shared; index: {i}"),
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
