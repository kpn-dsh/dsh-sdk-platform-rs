use std::{
    fmt::{Display, Formatter},
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dashmap::DashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{error::DshError, Platform};

/// `MqttTokenFetcher` is responsible for fetching and managing MQTT tokens for DSH.
///
/// It ensures that the tokens are valid, and if not, it refreshes them automatically. The struct
/// is thread-safe and can be shared across multiple threads.   

pub struct MqttTokenFetcher {
    tenant_name: String,
    rest_api_key: String,
    rest_token: Mutex<RestToken>,
    mqtt_token: DashMap<String, MqttToken>, // Mapping from Client ID to MqttToken
    platform: Platform,
    //token_lifetime: Option<i32>, // TODO: Implement option of passing token lifetime to request token for specific duration
    // port: Port or connection_type: Connection // TODO: Platform provides two connection options, current implemetation only provides connecting over SSL, enable WebSocket too
}

/// Constructs a new `MqttTokenFetcher`.
///
/// # Arguments
///
/// * `tenant_name` - The tenant name in DSH.
/// * `rest_api_key` - The REST API key used for authentication.
/// * `platform` - The DSH platform environment
///
/// # Returns
///
/// Returns a `Result` containing a `MqttTokenFetcher` instance or a `DshError`.
impl MqttTokenFetcher {
    pub fn new(
        tenant_name: String,
        rest_api_key: String,
        platform: Platform,
    ) -> Result<MqttTokenFetcher, DshError> {
        let rest_token = RestToken::default();
        Ok(Self {
            tenant_name: tenant_name.clone(),
            rest_api_key: rest_api_key.clone(),
            rest_token: Mutex::new(rest_token),
            mqtt_token: DashMap::new(),
            platform,
        })
    }
    /// Retrieves an MQTT token for the specified client ID.
    ///
    /// If the token is expired or does not exist, it fetches a new token.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The identifier for the MQTT client.
    /// * `claims` - Optional claims for the MQTT token.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `MqttToken` or a `DshError`.
    pub async fn get_token(
        &self,
        client_id: &str,
        claims: Option<Vec<Claims>>,
    ) -> Result<MqttToken, DshError> {
        let mut mqtt_token = self
            .mqtt_token
            .entry(client_id.to_string())
            .or_insert(self.fetch_new_mqtt_token(client_id, claims.clone()).await?);

        if !mqtt_token.is_valid() {
            *mqtt_token = self
                .fetch_new_mqtt_token(client_id, claims.clone())
                .await
                .unwrap()
        };
        Ok(mqtt_token.clone())
    }
    /// Fetches a new MQTT token from the platform.
    ///
    /// This method handles token validation and fetching the token
    async fn fetch_new_mqtt_token(
        &self,
        client_id: &str,
        claims: Option<Vec<Claims>>,
    ) -> Result<MqttToken, DshError> {
        let mut rest_token = self
            .rest_token
            .lock()
            .expect("Error during reading saved Rest Token");

        if !rest_token.is_valid() {
            *rest_token =
                RestToken::get(&self.tenant_name, &self.rest_api_key, &self.platform).await?
        }

        let authorization_header = format!("Bearer {}", rest_token.raw_token);

        let mqtt_token_request = MqttTokenRequest::new(client_id, &self.tenant_name, claims)?;
        let payload = serde_json::to_value(&mqtt_token_request)?;

        let response = mqtt_token_request
            .send(&self.platform, &authorization_header, &payload)
            .await?;

        MqttToken::new(response)
    }
}

/// Represent Claims information for MQTT request
/// * `action` - can be subscribe or publish
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    resource: Resource,
    action: String,
}

impl Claims {
    pub fn new(resource: Resource, action: String) -> Claims {
        Claims { resource, action }
    }
}

/// Enumeration representing possible actions in MQTT claims.
pub enum Actions {
    Publish,
    Subscribe,
}

impl Display for Actions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Actions::Publish => write!(f, "Publish"),
            Actions::Subscribe => write!(f, "Subscribe"),
        }
    }
}

/// Represents a resource in the MQTT claim.
///
/// The resource defines what the client can access in terms of stream, prefix, topic, and type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    stream: String,
    prefix: String,
    topic: String,
    #[serde(rename = "type")]
    type_: Option<String>,
}

impl Resource {
    /// Creates a new `Resource` instance. Please check DSH MQTT Documentation for further explanation of the fields.
    ///
    /// # Arguments
    ///
    /// * `stream` - The data stream name.
    /// * `prefix` - The prefix of the topic.
    /// * `topic` - The topic name.
    /// * `type_` - The optional type of the resource.
    ///
    ///
    /// # Returns
    ///
    /// Returns a new `Resource` instance.
    pub fn new(stream: String, prefix: String, topic: String, type_: Option<String>) -> Resource {
        Resource {
            stream,
            prefix,
            topic,
            type_,
        }
    }
}

#[derive(Serialize)]
struct MqttTokenRequest {
    id: String,
    tenant: String,
    claims: Option<Vec<Claims>>,
}

impl MqttTokenRequest {
    fn new(
        client_id: &str,
        tenant: &str,
        claims: Option<Vec<Claims>>,
    ) -> Result<MqttTokenRequest, DshError> {
        let mut hasher = Sha256::new();
        hasher.update(client_id);
        let result = hasher.finalize();
        let id = format!("{:x}", result);

        Ok(Self {
            id,
            tenant: tenant.to_string(),
            claims: claims,
        })
    }

    async fn send(
        &self,
        platform: &Platform,
        authorization_header: &str,
        payload: &serde_json::Value,
    ) -> Result<String, DshError> {
        const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

        let reqwest_client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .http1_only()
            .build()
            .expect("Failed to build reqwest client");

        let response = reqwest_client
            .post(platform.endpoint_mqtt_token())
            .header("Authorization", authorization_header)
            .json(payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(DshError::DshCallError {
                url: platform.endpoint_mqtt_token().to_string(),
                status_code: response.status(),
                error_body: response.text().await?,
            })
        }
    }
}

/// Represents attributes associated with a mqtt token.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct MqttTokenAttributes {
    gen: i32,
    endpoint: String,
    iss: String,
    claims: Vec<Claims>,
    exp: i32,
    client_id: String,
    iat: i32,
    tenant_id: String,
}

/// Represents a token used for MQTT connections.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MqttToken {
    exp: i32,
    raw_token: String,
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

        let decoded_token = decode_base64(header_payload)?;

        let token_attributes: MqttTokenAttributes = serde_json::from_slice(&decoded_token)?;
        let token = MqttToken {
            exp: token_attributes.exp,
            raw_token,
        };

        Ok(token)
    }

    fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i32;
        self.exp >= current_unixtime - 5
    }
}

/// Represents attributes associated with a Rest token.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct RestTokenAttributes {
    gen: i64,
    pub endpoint: String,
    iss: String,
    pub claims: RestClaims,
    pub exp: i64,
    tenant_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RestClaims {
    #[serde(rename = "datastreams/v0/mqtt/token")]
    datastreams_token: DatastreamsData,
}

#[derive(Serialize, Deserialize, Debug)]
struct DatastreamsData {}

/// Represents a rest token with its raw value and attributes.
#[derive(Serialize, Deserialize, Debug)]
struct RestToken {
    raw_token: String,
    exp: i64,
}

impl RestToken {
    /// Retrieves a new REST token from the platform.
    ///
    /// # Arguments
    ///
    /// * `tenant` - The tenant name associated with the DSH platform.
    /// * `api_key` - The REST API key used for authentication.
    /// * `env` - The platform environment (e.g., production, staging).
    ///
    /// # Returns
    ///
    /// A Result containing the created `RestToken` or a `DshError`.
    async fn get(tenant: &str, api_key: &str, env: &Platform) -> Result<RestToken, DshError> {
        let raw_token = Self::fetch_token(tenant, api_key, env).await.unwrap();

        let header_payload = extract_header_and_payload(&raw_token)?;

        let decoded_token = decode_base64(header_payload)?;

        let token_attributes: RestTokenAttributes = serde_json::from_slice(&decoded_token)?;
        let token = RestToken {
            raw_token,
            exp: token_attributes.exp,
        };

        Ok(token)
    }

    fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i64;
        self.exp >= current_unixtime - 5
    }

    async fn fetch_token(tenant: &str, api_key: &str, env: &Platform) -> Result<String, DshError> {
        let json_body = json!({"tenant": tenant});

        const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

        let reqwest_client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .http1_only()
            .build()
            .expect("Failed to build reqwest client");

        let rest_client = reqwest_client;
        let response = rest_client
            .post(env.endpoint_rest_token())
            .header("apikey", api_key)
            .json(&json_body)
            .send()
            .await?;

        let status = response.status();
        let body_text = response.text().await?;
        match status {
            reqwest::StatusCode::OK => Ok(body_text),
            _ => Err(DshError::DshCallError {
                url: env.endpoint_rest_api().to_string(),
                status_code: status,
                error_body: body_text,
            }),
        }
    }
}

impl Default for RestToken {
    fn default() -> Self {
        Self {
            raw_token: "".to_string(),
            exp: 0,
        }
    }
}

/// Extracts the header and payload part of a JWT token.
///
/// # Arguments
///
/// * `raw_token` - The raw JWT token string.
///
/// # Returns
///
/// A Result containing the header and payload part of the JWT token or a `DshError`.
fn extract_header_and_payload(raw_token: &str) -> Result<&str, DshError> {
    let parts: Vec<&str> = raw_token.split('.').collect();
    parts
        .get(1)
        .copied()
        .ok_or_else(|| DshError::ParseDnError("Header and payload are missing".to_string()))
}

/// Decodes a Base64-encoded string.
///
/// # Arguments
///
/// * `payload` - The Base64-encoded string.
///
/// # Returns
///
/// A Result containing the decoded byte vector or a `DshError`.
fn decode_base64(payload: &str) -> Result<Vec<u8>, DshError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mqtt_token_fetcher_new() {
        let tenant_name = "test_tenant".to_string();
        let rest_api_key = "test_api_key".to_string();
        let platform = Platform::NpLz;

        let fetcher = MqttTokenFetcher::new(tenant_name, rest_api_key, platform).unwrap();

        assert!(fetcher.mqtt_token.is_empty());
    }

    #[test]
    fn test_claims_new() {
        let resource = Resource::new(
            "stream".to_string(),
            "prefix".to_string(),
            "topic".to_string(),
            None,
        );
        let action = "publish".to_string();

        let claims = Claims::new(resource.clone(), action.clone());

        assert_eq!(claims.resource.stream, "stream");
        assert_eq!(claims.action, "publish");
    }

    #[test]
    fn test_resource_new() {
        let resource = Resource::new(
            "stream".to_string(),
            "prefix".to_string(),
            "topic".to_string(),
            None,
        );

        assert_eq!(resource.stream, "stream");
        assert_eq!(resource.prefix, "prefix");
        assert_eq!(resource.topic, "topic");
    }

    #[test]
    fn test_mqtt_token_is_valid() {
        let raw_token = "valid.token.payload".to_string();
        let token = MqttToken {
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32
                + 3600,
            raw_token,
        };

        assert!(token.is_valid());
    }

    #[test]
    fn test_rest_token_is_valid() {
        let token = RestToken {
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                + 3600,
            raw_token: "valid.token.payload".to_string(),
        };

        assert!(token.is_valid());
    }
}
