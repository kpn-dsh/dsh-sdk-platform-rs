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
pub struct LimitValuePartitionCount {
    #[serde(rename = "name")]
    pub name: Name,
    /// The number of partitions available for the managed tenant
    #[serde(rename = "value")]
    pub value: i32,
}

impl LimitValuePartitionCount {
    pub fn new(name: Name, value: i32) -> LimitValuePartitionCount {
        LimitValuePartitionCount { name, value }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "mem")]
    Mem,
    #[serde(rename = "certificateCount")]
    CertificateCount,
    #[serde(rename = "secretCount")]
    SecretCount,
    #[serde(rename = "topicCount")]
    TopicCount,
    #[serde(rename = "partitionCount")]
    PartitionCount,
    #[serde(rename = "consumerRate")]
    ConsumerRate,
    #[serde(rename = "producerRate")]
    ProducerRate,
    #[serde(rename = "requestRate")]
    RequestRate,
}

impl Default for Name {
    fn default() -> Name {
        Self::Cpu
    }
}
