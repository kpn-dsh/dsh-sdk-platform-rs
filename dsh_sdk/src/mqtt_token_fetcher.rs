use std::{
 sync::Mutex, time::{Duration, SystemTime, UNIX_EPOCH}
};

use dashmap::DashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    error::DshError,
    utils::{decode_payload, extract_header_and_payload},
    Platform,
};

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
    pub async fn new(
        tenant_name: String,
        rest_api_key: String,
        claims: Option<Vec<Claims>>,
        platform: Platform,
        //token_lifetime: Option<i32>,
    ) -> Result<MqttTokenFetcher, DshError> {
        let rest_token = RestToken::new(&tenant_name, &rest_api_key, &platform).await?;
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
                RestToken::new(&self.tenant_name, &self.rest_api_key, &self.platform).await?
        }

        let authorization_header = format!("Bearer {}", rest_token.raw_token);

        let payload = self.construct_mqtt_request_payload(client_id, claims)?;

        let response = self
            .send_mqtt_token_request(&authorization_header, &payload)
            .await;

        let mqtt_token = response
            .and_then(|raw_token| MqttToken::new(raw_token))
            .unwrap();

        //mqtt_token_map.insert(client_id, mqtt_token.clone());
        Ok(mqtt_token)
    }

    fn construct_mqtt_request_payload(
        &self,
        client_id: &str,
        claims: Option<&Claims>,
    ) -> Result<serde_json::Value, DshError> {
        let tenant = &self.tenant_name;
        let client_id = self.hash_client_id(client_id);

        use serde_json::Value;
        Ok(json!({
            "id": client_id,
            "tenant": tenant.to_string(),
            "claims": match &claims {
                Some(claim) => serde_json::to_value(&claim)?,
                None => Value::Null,
            }
        }))
    }

    async fn send_mqtt_token_request(
        &self,
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
            .post(self.platform.endpoint_mqtt_token())
            .header("Authorization", authorization_header)
            .json(payload)
            .send()
            .await;

        match response {
            Ok(response_success) => {
                let status = response_success.status();
                let body_text = response_success.text().await?;

                match status {
                    reqwest::StatusCode::OK => Ok(body_text),
                    _ => Err(DshError::DshCallError {
                        url: self.platform.endpoint_mqtt_token().to_string(),
                        status_code: status,
                        error_body: "Response NOT OK".to_string(),
                    }),
                }
            }
            Err(error) => Err(DshError::ReqwestError(error)),
        }
    }

    fn hash_client_id(&self, id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id);
        let result = hasher.finalize();
        format!("{:x}", result)
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
    Subscribe
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

        let decoded_token = decode_payload(header_payload)?;

        let token_attributes = MqttToken::parse_token_attributes(&decoded_token)?;
        let token = MqttToken {
            exp: token_attributes.exp,
            raw_token,
        };

        Ok(token)
    }

    fn parse_token_attributes(decoded_token: &[u8]) -> Result<MqttTokenAttributes, DshError> {
        serde_json::from_slice(decoded_token).map_err(DshError::JsonError)
    }

    // add 5 sec margin  //check SystemTime vs Instance Time
    fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i32;
        self.exp >= current_unixtime
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
    async fn new(tenant: &String, api_key: &String, env: &Platform) -> Result<RestToken, DshError> {
        let raw_token = Self::create_rest_token(tenant, api_key, env).await.unwrap();

        let header_payload = extract_header_and_payload(&raw_token)?;

        let decoded_token = decode_payload(header_payload)?;

        let token_attributes = RestToken::parse_token_attributes(&decoded_token)?;
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
        self.exp >= current_unixtime
    }

    fn parse_token_attributes(decoded_token: &[u8]) -> Result<RestTokenAttributes, DshError> {
        let res = serde_json::from_slice(decoded_token).map_err(DshError::JsonError);
        res
    }

    async fn create_rest_token(
        tenant: &String,
        api_key: &String,
        env: &Platform,
    ) -> Result<String, DshError> {
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
            .header("apikey", &api_key.to_string())
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
