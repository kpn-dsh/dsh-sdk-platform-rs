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
pub struct PathSpec {
    /// The path prefix (starting with `/`, ending without `/`) that will be matched for routing to this service.
    #[serde(rename = "prefix")]
    pub prefix: String,
}

impl PathSpec {
    pub fn new(prefix: String) -> PathSpec {
        PathSpec { prefix }
    }
}
