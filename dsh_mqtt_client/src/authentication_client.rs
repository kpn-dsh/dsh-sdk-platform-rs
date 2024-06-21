use crate::config::{ArcDshConfig, DshConfig};
use crate::error::DshError;
use crate::model::mqtt_model::{MqttToken, MqttTokenRequest};
use crate::model::rest_model::{RestToken, RestTokenRequest};
use dashmap::DashMap;
use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {   //TODO: Since there will be one tenant change here from dashmap to Rwlock of struct
    //tenant_id, (duration, token)
    static ref CACHED_REST_TOKENS: Arc<DashMap<String, (i64, String)>> = Arc::new(DashMap::new());
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
        let rest_cache = Arc::clone(&CACHED_REST_TOKENS);
        if let Some((expiry, token)) = rest_cache.get(&rest_request.tenant).map(|e| e.clone()) {
            if is_token_valid(expiry) {
                return Ok(token);
            } else {
                rest_cache.remove(&rest_request.tenant);
            }
        }
        self.get_new_token_add_cache(rest_request, &rest_cache)
            .await
    }

    async fn get_new_token_add_cache(
        &self,
        rest_request: &RestTokenRequest,
        rest_cache: &Arc<DashMap<String, (i64, String)>>,
    ) -> Result<String, DshError> {
        let created_raw_rest_token = self.create_rest_token(rest_request).await?;

        if let Ok(ref new_token) = RestToken::new(created_raw_rest_token.clone()) {
            rest_cache.insert(
                rest_request.tenant.clone(),
                (new_token.token_attributes.exp, new_token.raw_token.clone()),
            );
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
