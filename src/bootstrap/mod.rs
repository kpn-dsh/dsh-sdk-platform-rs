use serde::{Deserialize, Serialize};

pub use dsh::Cert;

pub mod common;
pub mod dsh;
#[cfg(feature = "local")]
pub mod local;

/// Bootstrap struct. Create new to initialize all related components to connect to the DSH kafka clusters
///  - Contains a struct similar to datastreams.json
///  - Metadata of running container/task
///  - Certificates for Kafka and DSH Schema Registry
///
/// # Example
/// ```no_run
/// use dsh_sdk::bootstrap::Bootstrap;
/// use dsh_sdk::bootstrap::GroupType;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let bootstrap = Bootstrap::new().await.expect("Bootstrap failed");
///     // Get kafka brokers
///     let brokers = bootstrap.kafka_properties().get_brokers();
///     // Get private group id from kafka properties
///     // (selects private or shared based on env var KAFKA_CONSUMER_GROUP_TYPE, dedaults to private)
///     let group_id = bootstrap.kafka_properties().get_group_id(GroupType::Private(0));
///     // Get certificates (if present)
///     let certificates = bootstrap.certificates();
///     Ok(())
/// }
/// ```

#[derive(Debug, Clone)]
pub struct Bootstrap {
    kafka_properties: KafkaProperties,
    client_id: String,
    certificates: Option<dsh::Cert>,
}

/// This struct is equivalent to the datastreams.json
/// It is possible to deserialize the json into this struct using serde_json.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KafkaProperties {
    brokers: Vec<String>,
    streams: std::collections::HashMap<String, Datastream>,
    private_consumer_groups: Vec<String>,
    shared_consumer_groups: Vec<String>,
    non_enveloped_streams: Vec<String>,
    schema_store: String,
}

/// Struct containing all topic information which also is provided in datastreams.json
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Datastream {
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

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupType::Private(i) => write!(f, "private; index: {}", i),
            GroupType::Shared(i) => write!(f, "shared; index: {}", i),
        }
    }
}
