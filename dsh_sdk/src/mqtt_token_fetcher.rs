use std::{
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dashmap::DashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{error::DshError, Platform};

pub struct MqttTokenFetcher {
    tenant_name: String,
    rest_api_key: String,
    claims: Option<Vec<Claims>>,
    rest_token: Mutex<RestToken>,
    mqtt_token: DashMap<String, MqttToken>, //Client_id, MqttToken
    //token_lifetime: Option<i32>,
    platform: Platform,
}

impl MqttTokenFetcher {
    pub fn new(
        tenant_name: String,
        rest_api_key: String,
        claims: Option<Vec<Claims>>,
        platform: Platform,
        //token_lifetime: Option<i32>,
    ) -> Result<MqttTokenFetcher, DshError> {
        let rest_token = RestToken::default();
        Ok(Self {
            tenant_name: tenant_name.clone(),
            rest_api_key: rest_api_key.clone(),
            claims,
            rest_token: Mutex::new(rest_token),
            mqtt_token: DashMap::new(),
            //token_lifetime,
            platform,
        })
    }

    pub async fn get_token(
        &self,
        client_id: &str,
        claims: Option<&Claims>,
    ) -> Result<MqttToken, DshError> {
        let mut mqtt_token = self
            .mqtt_token
            .entry(client_id.to_string())
            .or_insert(self.fetch_new_mqtt_token(client_id, claims).await?);

        if !mqtt_token.is_valid() {
            *mqtt_token = self.fetch_new_mqtt_token(client_id, claims).await.unwrap()
        };
        Ok(mqtt_token.clone())
    }

    async fn fetch_new_mqtt_token(
        &self,
        client_id: &str,
        claims: Option<&Claims>,
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

#[derive(derive_more::Display)]
pub enum Actions {
    Publish,
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    stream: String,
    prefix: String,
    topic: String,
    type_: Option<String>,
}

impl Resource {
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
    claims: Option<Claims>,
}

impl MqttTokenRequest {
    fn new(
        client_id: &str,
        tenant: &str,
        claims: Option<&Claims>,
    ) -> Result<MqttTokenRequest, DshError> {
        let mut hasher = Sha256::new();
        hasher.update(client_id);
        let result = hasher.finalize();
        let id = format!("{:x}", result);

        Ok(Self {
            id,
            tenant: tenant.to_string(),
            claims: claims.cloned(),
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

/// Represents a mqtt token with its raw value and attributes.
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
    raw_token: String, //impl setter/getter
    exp: i64,
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

fn extract_header_and_payload(raw_token: &str) -> Result<&str, DshError> {
    let parts: Vec<&str> = raw_token.split('.').collect();
    parts
        .get(1)
        .copied()
        .ok_or_else(|| DshError::ParseDnError("Header and payload are missing".to_string()))
}

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
