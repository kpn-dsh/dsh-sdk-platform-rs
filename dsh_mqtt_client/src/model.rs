use crate::error::DshError;
use serde::{Deserialize, Serialize};

/// Represents information to be sent by client to retrieve a token.
#[derive(Serialize, Deserialize, Debug, Clone)]
//add getter setter functions
pub struct RetrieveTokenRequest {
    pub tenant: String,
    pub api_key: String,
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
#[derive(Serialize, Deserialize, Debug)]
struct DatastreamsData {}

/// Includes required request parameters for a MQTT token.
pub struct MqttTokenRequest {
    pub tenant: String,
    pub rest_token: String,
    pub claims: Option<Vec<Claims>>,
    pub client_id: String,
}
/// Includes required request parameters for a REST token.
#[derive(Debug, Clone)]
pub struct RestTokenRequest {
    pub tenant: String,
    pub api_key: String,
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
        println!("RES: {:?}", res);
        res
    }
}
fn extract_header_and_payload(raw_token: &str) -> Result<&str, DshError> {
    let parts: Vec<&str> = raw_token.split('.').collect();
    parts
        .get(1)
        .copied()
        .ok_or_else(|| DshError::TokenError("Header and payload are missing".to_string()))
}

fn decode_payload(payload: &str) -> Result<Vec<u8>, DshError> {
    use base64::{alphabet, engine, read};
    use std::io::Read;

    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::NO_PAD);
    let mut decoder = read::DecoderReader::new(payload.as_bytes(), &engine);

    let mut decoded_token = Vec::new();
    decoder
        .read_to_end(&mut decoded_token)
        .map_err(DshError::IoError)?;

    Ok(decoded_token)
}
impl From<RetrieveTokenRequest> for RestTokenRequest {
    fn from(value: RetrieveTokenRequest) -> Self {
        RestTokenRequest {
            tenant: value.tenant,
            api_key: value.api_key,
        }
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
