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
pub struct LimitValueTopicCountAllOf {
    /// The number of topics available for the managed tenant
    #[serde(rename = "value")]
    pub value: i32,
}

impl LimitValueTopicCountAllOf {
    pub fn new(value: i32) -> LimitValueTopicCountAllOf {
        LimitValueTopicCountAllOf { value }
    }
}
