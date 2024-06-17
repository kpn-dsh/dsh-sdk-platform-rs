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
pub struct ActualCertificateAllOf {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "notBefore")]
    pub not_before: String,
    #[serde(rename = "notAfter")]
    pub not_after: String,
    #[serde(rename = "distinguishedName")]
    pub distinguished_name: String,
    #[serde(rename = "dnsNames")]
    pub dns_names: Vec<String>,
}

impl ActualCertificateAllOf {
    pub fn new(
        serial_number: String,
        not_before: String,
        not_after: String,
        distinguished_name: String,
        dns_names: Vec<String>,
    ) -> ActualCertificateAllOf {
        ActualCertificateAllOf {
            serial_number,
            not_before,
            not_after,
            distinguished_name,
            dns_names,
        }
    }
}
