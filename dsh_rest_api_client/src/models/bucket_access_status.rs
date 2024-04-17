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
pub struct BucketAccessStatus {
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<models::BucketAccessConfiguration>>,
    #[serde(rename = "actual", skip_serializing_if = "Option::is_none")]
    pub actual: Option<Box<models::BucketAccess>>,
    #[serde(rename = "status")]
    pub status: Box<models::AllocationStatus>,
}

impl BucketAccessStatus {
    pub fn new(status: models::AllocationStatus) -> BucketAccessStatus {
        BucketAccessStatus {
            configuration: None,
            actual: None,
            status: Box::new(status),
        }
    }
}

