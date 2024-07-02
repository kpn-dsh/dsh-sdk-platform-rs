use serde::{Deserialize, Serialize};

use crate::error::DshError;

use super::{
    token_request_attr::RetrieveTokenRequest,
    utils::{decode_payload, extract_header_and_payload},
};

/// Represents attributes associated with a mqtt token.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct MqttTokenAttributes {
    gen: i32,
    pub endpoint: String,
    iss: String,
    pub claims: Vec<Claims>,
    pub exp: i32,
    pub client_id: String,
    iat: i32,
    pub tenant_id: String,
}

/// Includes required request parameters for a MQTT token.
pub struct MqttTokenRequest {
    pub tenant: String,
    pub rest_token: String,
    pub claims: Option<Vec<Claims>>,
    pub client_id: String,
}
/// Represent Claims information for MQTT request
/// * `action` - can be subscribe or publish
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub resource: Resource,
    pub action: String,
}
/// Represent Resource information to be placed in claims
/// * `stream` - stream name in your tenant , e.g., weather
/// * `prefix` - equal to /tt
/// * `topic` - topic pattern, can be checked in MQTT documentation
/// * `type` - type of resource , e.g., topic
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub stream: String,
    pub prefix: String,
    pub topic: String,
    pub type_: Option<String>,
}
/// Represents a mqtt token with its raw value and attributes.
#[derive(Serialize, Deserialize, Debug)]
pub struct MqttToken {
    pub raw_token: String,
    pub token_attributes: MqttTokenAttributes,
}
impl MqttToken {
    /// Creates a new instance of `MqttToken` from a raw token string.
    ///
    /// # Arguments
    ///
    /// * `raw_token` - The raw token string.
    ///
    /// # Returns
    ///
    /// A Result containing the created MqttToken or an error.
    pub fn new(raw_token: String) -> Result<MqttToken, DshError> {
        let header_payload = extract_header_and_payload(&raw_token)?;

        let decoded_token = decode_payload(header_payload)?;

        let token_attributes = MqttToken::parse_token_attributes(&decoded_token)?;
        let token = MqttToken {
            raw_token,
            token_attributes,
        };

        Ok(token)
    }

    fn parse_token_attributes(decoded_token: &[u8]) -> Result<MqttTokenAttributes, DshError> {
        serde_json::from_slice(decoded_token).map_err(DshError::SerdeJson)
    }
}
impl MqttTokenRequest {
    /// Creates a new instance of `MqttTokenRequest`.
    ///
    /// # Arguments
    ///
    /// * `rest_token` - The REST token to be used for obtaining the MQTT token.
    /// * `value` - The struct `RetrieveTokenRequest` to get required parameters.
    pub fn new(rest_token: String, value: RetrieveTokenRequest) -> Self {
        MqttTokenRequest {
            tenant: value.tenant,
            rest_token,
            claims: value.claims,
            client_id: value.client_id,
        }
    }
}
