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
pub struct ThirdPartyBucketConcession {
    /// your name for this bucket owned by a third party
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "readable")]
    pub readable: bool,
    #[serde(rename = "writable")]
    pub writable: bool,
    #[serde(rename = "credentialidentifierref")]
    pub credentialidentifierref: String,
    #[serde(rename = "credentialsecretref")]
    pub credentialsecretref: String,
    #[serde(rename = "shareidentifier")]
    pub shareidentifier: String,
}

impl ThirdPartyBucketConcession {
    pub fn new(
        name: String,
        readable: bool,
        writable: bool,
        credentialidentifierref: String,
        credentialsecretref: String,
        shareidentifier: String,
    ) -> ThirdPartyBucketConcession {
        ThirdPartyBucketConcession {
            name,
            readable,
            writable,
            credentialidentifierref,
            credentialsecretref,
            shareidentifier,
        }
    }
}
