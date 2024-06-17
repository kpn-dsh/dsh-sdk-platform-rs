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

/// Zone : available networks on this platform
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    #[serde(rename = "network")]
    pub network: Network,
}

impl Zone {
    /// available networks on this platform
    pub fn new(network: Network) -> Zone {
        Zone { network }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "public")]
    Public,
}

impl Default for Network {
    fn default() -> Network {
        Self::Internal
    }
}
