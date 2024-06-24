use crate::config::{ArcDshConfig, DshConfig};
use crate::error::DshError;
use crate::model::mqtt_model::{MqttToken, MqttTokenRequest};
use crate::model::rest_model::{RestToken, RestTokenRequest};
use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref CACHED_REST_TOKENS: Arc<RwLock<Option<TokenCache>>> = Arc::new(RwLock::new(None));
}

#[derive(Clone)]
struct TokenCache {
    token: String,
    expiration_date: i64,
}

impl TokenCache {
    fn read_cache() -> Option<TokenCache> {
        let cache = CACHED_REST_TOKENS.read().unwrap();
        cache.clone()
    }
    fn write_cache(token: String, expiration_date: i64) {
        let mut cache = CACHED_REST_TOKENS.write().unwrap();
        *cache = Some(TokenCache {
            token,
            expiration_date,
        })
    }
    fn clear_cache() {
        let mut cache = CACHED_REST_TOKENS.write().unwrap();
        *cache = None
    }
}

/// To create MQTT Token, Rest Token need to be fetched first. This fetching will be handled in DshRestAuthenticationClient implementation.
/// # Arguments
///
/// * `config` - The configuration for DSH authentication.
/// * `reqwest_client` - Common rest client to make api call.
pub struct DshRestAuthenticationClient {
    pub(crate) config: ArcDshConfig,
    pub(crate) reqwest_client: Client,
}

impl DshRestAuthenticationClient {
    pub fn new(config: Arc<DshConfig>, reqwest_client: Client) -> Self {
        DshRestAuthenticationClient {
            config,
            reqwest_client,
        }
    }

    /// Retrieves a REST token based on the provided `RestTokenRequest`.
    ///
    /// # Arguments
    ///
    /// * `rest_request` - The REST token request containing tenant and API key information.
    ///
    /// # Returns
    ///
    /// A Result containing the retrieved REST token or an error.
    pub async fn retrieve_rest_token(
        &self,
        rest_request: &RestTokenRequest,
    ) -> Result<String, DshError> {
        if let Some(token_cache) = TokenCache::read_cache() {
            if is_token_valid(token_cache.expiration_date) {
                return Ok(token_cache.token);
            } else {
                TokenCache::clear_cache()
            }
        }
        self.get_new_token_add_cache(rest_request).await
    }

    async fn get_new_token_add_cache(
        &self,
        rest_request: &RestTokenRequest,
    ) -> Result<String, DshError> {
        let created_raw_rest_token = self.create_rest_token(rest_request).await?;

        if let Ok(ref new_token) = RestToken::new(created_raw_rest_token.clone()) {
            TokenCache::write_cache(new_token.raw_token.clone(), new_token.token_attributes.exp)
        }

        Ok(created_raw_rest_token)
    }

    async fn create_rest_token(&self, rest_request: &RestTokenRequest) -> Result<String, DshError> {
        let tenant = &rest_request.tenant;
        let api_key = &rest_request.api_key;
        let json_body = json!({tenant: &tenant.to_string()});

        let rest_client = &self.reqwest_client;
        let response = rest_client
            .post(self.config.get_rest_token_endpoint())
            .header("apikey", &api_key.to_string())
            .json(&json_body)
            .send()
            .await?;

        let status = response.status();
        let body_text = response.text().await?;
        match status {
            reqwest::StatusCode::OK => Ok(body_text),
            _ => Err(DshError::RequestError(format!(
                "Error requesting token server response code: {:?} body: {:?}",
                status, body_text
            ))),
        }
    }
}

fn is_token_valid(exp: i64) -> bool {
    let current_unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs() as i64;
    exp >= current_unixtime
}

/// This struct and its implementation handles fetching MQTT token.
/// # Arguments
///
/// * `config` - The configuration for DSH authentication.
/// /// * `reqwest_client` - Common rest client to make api call.
pub struct DshMqttAuthenticationClient {
    pub config: ArcDshConfig,
    pub reqwest_client: Client,
}

impl DshMqttAuthenticationClient {
    /// Retrieves an MQTT token based on the provided `MqttTokenRequest`.
    ///
    /// # Arguments
    ///
    /// * `mqtt_request` - The MQTT token request containing tenant, REST token, claims, and client ID information.
    ///
    /// # Returns
    ///
    /// A Result containing the retrieved MQTT token or an error.
    pub async fn retrieve_mqtt_token(
        &self,
        mqtt_request: &MqttTokenRequest,
    ) -> Result<MqttToken, DshError> {
        let authorization_header = format!("Bearer {}", mqtt_request.rest_token);

        let payload = self.construct_mqtt_request_payload(mqtt_request)?;

        let response = self
            .send_mqtt_token_request(&authorization_header, &payload)
            .await;

        self.process_mqtt_token_response(response)
    }
    fn construct_mqtt_request_payload(
        &self,
        mqtt_request: &MqttTokenRequest,
    ) -> Result<serde_json::Value, DshError> {
        let tenant = &mqtt_request.tenant;
        let claims = &mqtt_request.claims;
        let client_id = &mqtt_request.client_id;

        use serde_json::Value;
        Ok(json!({
            "id": self.hash_client_id(client_id),
            "tenant": &tenant.to_string(),
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
        let rest_client = &self.reqwest_client;
        let response = rest_client
            .post(self.config.get_mqtt_token_endpoint())
            .header("Authorization", authorization_header)
            .json(payload)
            .send()
            .await?;

        let status = response.status();
        let body_text = response.text().await?;

        match status {
            reqwest::StatusCode::OK => Ok(body_text),
            _ => Err(DshError::RequestError(format!(
                "Error requesting token server response code: {:?} body: {:?}",
                status, body_text
            ))),
        }
    }
    fn process_mqtt_token_response(
        &self,
        raw_token: Result<String, DshError>,
    ) -> Result<MqttToken, DshError> {
        raw_token.and_then(|raw_token| {
            MqttToken::new(raw_token).map_err(|e| DshError::TokenError(e.to_string()))
        })
    }
    fn hash_client_id(&self, id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
