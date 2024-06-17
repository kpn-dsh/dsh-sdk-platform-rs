/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(rename = "instances")]
    pub instances: i32,
    #[serde(rename = "cpus")]
    pub cpus: f64,
    #[serde(rename = "mem")]
    pub mem: i32,
    #[serde(rename = "volumeSize")]
    pub volume_size: i32,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "snapshotInterval", skip_serializing_if = "Option::is_none")]
    pub snapshot_interval: Option<i32>,
}

impl Database {
    pub fn new(instances: i32, cpus: f64, mem: i32, volume_size: i32) -> Database {
        Database {
            instances,
            cpus,
            mem,
            volume_size,
            extensions: None,
            version: None,
            snapshot_interval: None,
        }
    }
}
