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
pub struct Volume {
    #[serde(rename = "sizeGiB")]
    pub size_gi_b: i32,
}

impl Volume {
    pub fn new(size_gi_b: i32) -> Volume {
        Volume { size_gi_b }
    }
}
