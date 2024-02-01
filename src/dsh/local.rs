//! Create a properties struct for local development
//!
//! This module contains logic to load the local_datastreams.json file and parse it into a datastream struct inside the properties struct.
//! This struct can be used to create a connection to your local Kafka cluster

use log::error;
use std::fs::File;
use std::io::Read;

use super::datastream::Datastream;
use super::Properties;
use crate::error::DshError;

const FILE_NAME: &str = "local_datastreams.json";

impl Properties {
    /// Create a new Properties struct for local development
    /// This function reads the local_datastreams.json file and parses it into a datastream struct
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
        Ok(Self {
            client_id,
            tenant_name,
            datastream,
            certificates,
        })
    }
}

impl Datastream {
    fn load_local_datastreams() -> Result<Self, DshError> {
        let path_buf: std::path::PathBuf = std::env::current_dir().unwrap().join(FILE_NAME);
        let file_result = File::open(&path_buf);
        let mut file = match file_result {
            Ok(file) => file,
            Err(e) => {
                error!("Error opening {}: {}", path_buf.display(), e);
                return Err(DshError::IoError(FILE_NAME, e));
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let datastream: Datastream = serde_json::from_str(&contents)?;
        Ok(datastream)
    }
}
