/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.6.6
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlinkClusterStatus {
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<models::FlinkCluster>>,
    #[serde(rename = "actual", skip_serializing_if = "Option::is_none")]
    pub actual: Option<Box<models::FlinkCluster>>,
    #[serde(rename = "status")]
    pub status: Box<models::AllocationStatus>,
}

impl FlinkClusterStatus {
    pub fn new(status: models::AllocationStatus) -> FlinkClusterStatus {
        FlinkClusterStatus {
            configuration: None,
            actual: None,
            status: Box::new(status),
        }
    }
}

