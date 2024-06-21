use serde::{Deserialize, Serialize};

use crate::error::DshError;

use super::{
    token_request_attr::RetrieveTokenRequest,
    utils::{decode_payload, extract_header_and_payload},
};

/// Represents a rest token with its raw value and attributes.
#[derive(Serialize, Deserialize, Debug)]
pub struct RestToken {
    pub raw_token: String,
    pub token_attributes: RestTokenAttributes,
}

/// Represents attributes associated with a Rest token.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct RestTokenAttributes {
    gen: i64,
    pub endpoint: String,
    iss: String,
    pub claims: RestClaims,
    pub exp: i64,
    tenant_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RestClaims {
    #[serde(rename = "datastreams/v0/mqtt/token")]
    datastreams_token: DatastreamsData,
}
/// Includes required request parameters for a REST token.
#[derive(Debug, Clone)]
pub struct RestTokenRequest {
    pub tenant: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DatastreamsData {}
impl RestToken {
    /// Creates a new instance of `RestToken` from a raw token string.
    ///
    /// # Arguments
    ///
    /// * `raw_token` - The raw token string.
    ///
    /// # Returns
    ///
    /// A Result containing the created RestToken or an error.
    pub fn new(raw_token: String) -> Result<RestToken, DshError> {
        let header_payload = extract_header_and_payload(&raw_token)?;

        let decoded_token = decode_payload(header_payload)?;

        let token_attributes = RestToken::parse_token_attributes(&decoded_token)?;
        let token = RestToken {
            raw_token,
            token_attributes,
        };

        Ok(token)
    }

    fn parse_token_attributes(decoded_token: &[u8]) -> Result<RestTokenAttributes, DshError> {
        let res = serde_json::from_slice(decoded_token).map_err(DshError::SerdeJson);
        res
    }
}

impl From<RetrieveTokenRequest> for RestTokenRequest {
    fn from(value: RetrieveTokenRequest) -> Self {
        RestTokenRequest {
            tenant: value.tenant,
            api_key: value.api_key,
        }
    }
}
