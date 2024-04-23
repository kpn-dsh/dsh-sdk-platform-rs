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

/// Certificate : information on a certificate which is wanted on the platform but may not yet be provisioned
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(rename = "keySecret")]
    pub key_secret: String,
    #[serde(rename = "certChainSecret")]
    pub cert_chain_secret: String,
    #[serde(rename = "passphraseSecret", skip_serializing_if = "Option::is_none")]
    pub passphrase_secret: Option<String>,
}

impl Certificate {
    /// information on a certificate which is wanted on the platform but may not yet be provisioned
    pub fn new(key_secret: String, cert_chain_secret: String) -> Certificate {
        Certificate {
            key_secret,
            cert_chain_secret,
            passphrase_secret: None,
        }
    }
}
