//! Local bootstrap for development
//!
//! This module contains logic to load the local_datastreams.json file and parse it into a Bootstrap struct
//! This struct can be used to create a connection to your local Kafka cluster

use std::fs::File;
use std::io::Read;

use super::datastream::Datastream;
use super::KafkaProperties;
use crate::error::DshError;

impl KafkaProperties {
    /// Create a new KafkaProperties struct for local development
    /// This function reads the local_datastreams.json file and parses it into a Bootstrap struct
    ///
    /// local_datastreams.json should be placed in the root of the project
    ///
    /// # Error
    /// - Error if local_datastreams.json is not present in the root of the project
    /// - Error if local_datastreams.json is not following the correct format
    pub(crate) fn new_local() -> Result<Self, DshError> {
        let datastream = Datastream::load_local_datastreams()?;
        let client_id = String::from("local");
        let tenant_name = String::from("local");
        let certificates = None;
        Ok(KafkaProperties {
            client_id,
            tenant_name,
            datastream,
            certificates,
        })
    }
}

impl Datastream {
    fn load_local_datastreams() -> Result<Self, DshError> {
        let mut file = File::open("local_datastreams.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let kafka_properties: Datastream = serde_json::from_str(&contents)?;
        Ok(kafka_properties)
    }
}
