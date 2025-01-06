//! # MQTT Token Fetcher
//!
//! `MqttTokenFetcher` is responsible for fetching and managing MQTT tokens for DSH.
use std::collections::{hash_map::Entry, HashMap};
use std::fmt::{Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;

use crate::{error::DshError, Platform};

/// `ProtocolTokenFetcher` is responsible for fetching and managing tokens for the DSH Mqtt and Http protocol adapters.
///
/// It ensures that the tokens are valid, and if not, it refreshes them automatically. The struct
/// is thread-safe and can be shared across multiple threads.   

pub struct ProtocolTokenFetcher {
    tenant_name: String,
    rest_api_key: String,
    rest_token: RwLock<RestToken>,
    rest_auth_url: String,
    mqtt_token: RwLock<HashMap<String, MqttToken>>, // Mapping from Client ID to MqttToken
    mqtt_auth_url: String,
    client: reqwest::Client,
    //token_lifetime: Option<i32>, // TODO: Implement option of passing token lifetime to request token for specific duration
    // port: Port or connection_type: Connection // TODO: Platform provides two connection options, current implemetation only provides connecting over SSL, enable WebSocket too
}

/// Constructs a new `ProtocolTokenFetcher`.
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
impl ProtocolTokenFetcher {
    /// Constructs a new `ProtocolTokenFetcher`.
    ///
    /// # Arguments
    ///
    /// * `tenant_name` - The tenant name of DSH.
    /// * `api_key` - The realted API key of tenant used for authentication to fetech Token for MQTT.
    /// * `platform` - The target DSH platform environment.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::ProtocolTokenFetcher;
    /// use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let tenant_name = "test_tenant".to_string();
    /// let api_key = "aAbB123".to_string();
    /// let platform = Platform::NpLz;
    ///
    /// let fetcher = ProtocolTokenFetcher::new(tenant_name, api_key, platform);
    /// let token = fetcher.get_token("test_client", None).await.unwrap();
    /// # }
    /// ```
    pub fn new(tenant_name: String, api_key: String, platform: Platform) -> Self {
        const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

        let reqwest_client = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .http1_only()
            .build()
            .expect("Failed to build reqwest client");
        Self::new_with_client(tenant_name, api_key, platform, reqwest_client)
    }

    /// Constructs a new `ProtocolTokenFetcher` with a custom reqwest client.
    /// On this Reqwest client, you can set custom timeouts, headers, Rustls etc.
    ///
    /// # Arguments
    ///
    /// * `tenant_name` - The tenant name of DSH.
    /// * `api_key` - The realted API key of tenant used for authentication to fetech Token for MQTT.
    /// * `platform` - The target DSH platform environment.
    /// * `client` - User configured reqwest client to be used for fetching tokens
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dsh_sdk::mqtt_token_fetcher::ProtocolTokenFetcher;
    /// use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let tenant_name = "test_tenant".to_string();
    /// let api_key = "aAbB123".to_string();
    /// let platform = Platform::NpLz;
    /// let client = reqwest::Client::new();
    /// let fetcher = ProtocolTokenFetcher::new_with_client(tenant_name, api_key, platform, client);
    /// let token = fetcher.get_token("test_client", None).await.unwrap();
    /// # }
    /// ```
    pub fn new_with_client(
        tenant_name: String,
        api_key: String,
        platform: Platform,
        client: reqwest::Client,
    ) -> Self {
        let rest_token = RestToken::default();
        Self {
            tenant_name,
            rest_api_key: api_key,
            rest_token: RwLock::new(rest_token),
            rest_auth_url: platform.endpoint_rest_token().to_string(),
            mqtt_token: RwLock::new(HashMap::new()),
            mqtt_auth_url: platform.endpoint_mqtt_token().to_string(),
            client,
        }
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
        match self.mqtt_token.write().await.entry(client_id.to_string()) {
            Entry::Occupied(mut entry) => {
                let mqtt_token = entry.get_mut();
                if !mqtt_token.is_valid() {
                    *mqtt_token = self.fetch_new_mqtt_token(client_id, claims).await?;
                };
                Ok(mqtt_token.clone())
            }
            Entry::Vacant(entry) => {
                let mqtt_token = self.fetch_new_mqtt_token(client_id, claims).await?;
                entry.insert(mqtt_token.clone());
                Ok(mqtt_token)
            }
        }
    }

    /// Fetches a new MQTT token from the platform.
    ///
    /// This method handles token validation and fetching the token
    async fn fetch_new_mqtt_token(
        &self,
        client_id: &str,
        claims: Option<Vec<Claims>>,
    ) -> Result<MqttToken, DshError> {
        let mut rest_token = self.rest_token.write().await;

        if !rest_token.is_valid() {
            *rest_token = RestToken::get(
                &self.client,
                &self.tenant_name,
                &self.rest_api_key,
                &self.rest_auth_url,
            )
            .await?
        }

        let authorization_header = format!("Bearer {}", rest_token.raw_token);

        let mqtt_token_request = MqttTokenRequest::new(client_id, &self.tenant_name, claims)?;
        let payload = serde_json::to_value(&mqtt_token_request)?;

        let response = mqtt_token_request
            .send(
                &self.client,
                &self.mqtt_auth_url,
                &authorization_header,
                &payload,
            )
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
    pub fn new(resource: Resource, action: Actions) -> Claims {
        Claims {
            resource,
            action: action.to_string(),
        }
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
            Actions::Publish => write!(f, "publish"),
            Actions::Subscribe => write!(f, "subscribe"),
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
            claims,
        })
    }

    async fn send(
        &self,
        reqwest_client: &reqwest::Client,
        mqtt_auth_url: &str,
        authorization_header: &str,
        payload: &serde_json::Value,
    ) -> Result<String, DshError> {
        let response = reqwest_client
            .post(mqtt_auth_url)
            .header("Authorization", authorization_header)
            .json(payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(DshError::DshCallError {
                url: mqtt_auth_url.to_string(),
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
    claims: Option<Vec<Claims>>,
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

    /// Checks if the MQTT token is still valid.
    fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i32;
        self.exp >= current_unixtime + 5
    }
}

/// Represents attributes associated with a Rest token.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct RestTokenAttributes {
    gen: i64,
    endpoint: String,
    iss: String,
    claims: RestClaims,
    exp: i32,
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
    exp: i32,
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
    async fn get(
        client: &reqwest::Client,
        tenant: &str,
        api_key: &str,
        auth_url: &str,
    ) -> Result<RestToken, DshError> {
        let raw_token = Self::fetch_token(client, tenant, api_key, auth_url).await?;

        let header_payload = extract_header_and_payload(&raw_token)?;

        let decoded_token = decode_base64(header_payload)?;

        let token_attributes: RestTokenAttributes = serde_json::from_slice(&decoded_token)?;
        let token = RestToken {
            raw_token,
            exp: token_attributes.exp,
        };

        Ok(token)
    }

    // Checks if the REST token is still valid.
    fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i32;
        self.exp >= current_unixtime + 5
    }

    async fn fetch_token(
        client: &reqwest::Client,
        tenant: &str,
        api_key: &str,
        auth_url: &str,
    ) -> Result<String, DshError> {
        let json_body = json!({"tenant": tenant});

        let response = client
            .post(auth_url)
            .header("apikey", api_key)
            .json(&json_body)
            .send()
            .await?;

        let status = response.status();
        let body_text = response.text().await?;
        match status {
            reqwest::StatusCode::OK => Ok(body_text),
            _ => Err(DshError::DshCallError {
                url: auth_url.to_string(),
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
    use mockito::Matcher;

    async fn create_valid_fetcher() -> ProtocolTokenFetcher {
        let exp_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32
            + 3600;
        println!("exp_time: {}", exp_time);
        let rest_token: RestToken = RestToken {
            exp: exp_time as i32,
            raw_token: "valid.token.payload".to_string(),
        };
        let mqtt_token = MqttToken {
            exp: exp_time,
            raw_token: "valid.token.payload".to_string(),
        };
        let mqtt_token_map = RwLock::new(HashMap::new());
        mqtt_token_map
            .write()
            .await
            .insert("test_client".to_string(), mqtt_token.clone());
        ProtocolTokenFetcher {
            tenant_name: "test_tenant".to_string(),
            rest_api_key: "test_api_key".to_string(),
            rest_token: RwLock::new(rest_token),
            rest_auth_url: "test_auth_url".to_string(),
            mqtt_token: mqtt_token_map,
            client: reqwest::Client::new(),
            mqtt_auth_url: "test_auth_url".to_string(),
        }
    }

    #[tokio::test]
    async fn test_mqtt_token_fetcher_new() {
        let tenant_name = "test_tenant".to_string();
        let rest_api_key = "test_api_key".to_string();
        let platform = Platform::NpLz;

        let fetcher = ProtocolTokenFetcher::new(tenant_name, rest_api_key, platform);

        assert!(fetcher.mqtt_token.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_mqtt_token_fetcher_new_with_client() {
        let tenant_name = "test_tenant".to_string();
        let rest_api_key = "test_api_key".to_string();
        let platform = Platform::NpLz;

        let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();
        let fetcher =
            ProtocolTokenFetcher::new_with_client(tenant_name, rest_api_key, platform, client);

        assert!(fetcher.mqtt_token.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_fetch_new_mqtt_token() {
        let mut mockito_server = mockito::Server::new_async().await;
        let _m = mockito_server.mock("POST", "/rest_auth_url")
            .with_status(200)
            .with_body(r#"{"raw_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJnZW4iOjEsImVuZHBvaW50IjoidGVzdF9lbmRwb2ludCIsImlzcyI6IlN0cmluZyIsImNsYWltcyI6W3sicmVzb3VyY2UiOiJ0ZXN0IiwiYWN0aW9uIjoicHVzaCJ9XSwiZXhwIjoxLCJjbGllbnQtaWQiOiJ0ZXN0X2NsaWVudCIsImlhdCI6MCwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQifQ.WCf03qyxV1NwxXpzTYF7SyJYwB3uAkQZ7u-TVrDRJgE"}"#)
            .create_async()
            .await;
        let _m2 = mockito_server.mock("POST", "/mqtt_auth_url")
            .with_status(200)
            .with_body(r#"{"mqtt_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJnZW4iOjEsImVuZHBvaW50IjoidGVzdF9lbmRwb2ludCIsImlzcyI6IlN0cmluZyIsImV4cCI6MSwiY2xpZW50LWlkIjoidGVzdF9jbGllbnQiLCJpYXQiOjAsInRlbmFudC1pZCI6InRlc3RfdGVuYW50In0.VwlKomR4OnLtLX-NwI-Fpol8b6t-kmptRS_vPnwNd3A"}"#)
            .create();

        let client = reqwest::Client::new();
        let rest_token = RestToken {
            raw_token: "initial_token".to_string(),
            exp: 0,
        };

        let fetcher = ProtocolTokenFetcher {
            client,
            tenant_name: "test_tenant".to_string(),
            rest_api_key: "test_api_key".to_string(),
            mqtt_token: RwLock::new(HashMap::new()),
            rest_auth_url: mockito_server.url() + "/rest_auth_url",
            mqtt_auth_url: mockito_server.url() + "/mqtt_auth_url",
            rest_token: RwLock::new(rest_token),
        };

        let result = fetcher.fetch_new_mqtt_token("test_client_id", None).await;
        println!("{:?}", result);
        assert!(result.is_ok());
        let mqtt_token = result.unwrap();
        assert_eq!(mqtt_token.exp, 1);
    }

    #[tokio::test]
    async fn test_mqtt_token_fetcher_get_token() {
        let fetcher = create_valid_fetcher().await;
        let token = fetcher.get_token("test_client", None).await.unwrap();
        assert_eq!(token.raw_token, "valid.token.payload");
    }

    #[test]
    fn test_actions_display() {
        let action = Actions::Publish;
        assert_eq!(action.to_string(), "publish");
        let action = Actions::Subscribe;
        assert_eq!(action.to_string(), "subscribe");
    }

    #[test]
    fn test_token_request_new() {
        let request = MqttTokenRequest::new("test_client", "test_tenant", None).unwrap();
        assert_eq!(request.id.len(), 64);
        assert_eq!(request.tenant, "test_tenant");
    }

    #[tokio::test]
    async fn test_send_success() {
        let mut mockito_server = mockito::Server::new_async().await;
        let _m = mockito_server
            .mock("POST", "/mqtt_auth_url")
            .match_header("Authorization", "Bearer test_token")
            .match_body(Matcher::Json(json!({"key": "value"})))
            .with_status(200)
            .with_body("success_response")
            .create();

        let client = reqwest::Client::new();
        let payload = json!({"key": "value"});
        let request = MqttTokenRequest::new("test_client", "test_tenant", None).unwrap();
        let result = request
            .send(
                &client,
                &format!("{}/mqtt_auth_url", mockito_server.url()),
                "Bearer test_token",
                &payload,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success_response");
    }

    #[tokio::test]
    async fn test_send_failure() {
        let mut mockito_server = mockito::Server::new_async().await;
        let _m = mockito_server
            .mock("POST", "/mqtt_auth_url")
            .match_header("Authorization", "Bearer test_token")
            .match_body(Matcher::Json(json!({"key": "value"})))
            .with_status(400)
            .with_body("error_response")
            .create();

        let client = reqwest::Client::new();
        let payload = json!({"key": "value"});
        let request = MqttTokenRequest::new("test_client", "test_tenant", None).unwrap();
        let result = request
            .send(
                &client,
                &format!("{}/mqtt_auth_url", mockito_server.url()),
                "Bearer test_token",
                &payload,
            )
            .await;

        assert!(result.is_err());
        if let Err(DshError::DshCallError {
            url,
            status_code,
            error_body,
        }) = result
        {
            assert_eq!(url, format!("{}/mqtt_auth_url", mockito_server.url()));
            assert_eq!(status_code, reqwest::StatusCode::BAD_REQUEST);
            assert_eq!(error_body, "error_response");
        } else {
            panic!("Expected DshCallError");
        }
    }

    #[test]
    fn test_claims_new() {
        let resource = Resource::new(
            "stream".to_string(),
            "prefix".to_string(),
            "topic".to_string(),
            None,
        );
        let action = Actions::Publish;

        let claims = Claims::new(resource.clone(), action);

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
    fn test_mqtt_token_is_invalid() {
        let raw_token = "valid.token.payload".to_string();
        let token = MqttToken {
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32,
            raw_token,
        };

        assert!(!token.is_valid());
    }

    #[test]
    fn test_rest_token_is_valid() {
        let token = RestToken {
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32
                + 3600,
            raw_token: "valid.token.payload".to_string(),
        };

        assert!(token.is_valid());
    }

    #[test]
    fn test_rest_token_is_invalid() {
        let token = RestToken {
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32,
            raw_token: "valid.token.payload".to_string(),
        };

        assert!(!token.is_valid());
    }

    #[test]
    fn test_rest_token_default_is_invalid() {
        let token = RestToken::default();

        assert!(!token.is_valid());
    }

    #[test]
    fn test_extract_header_and_payload() {
        let raw = "header.payload.signature";
        let result = extract_header_and_payload(raw).unwrap();
        assert_eq!(result, "payload");

        let raw = "header.payload";
        let result = extract_header_and_payload(raw).unwrap();
        assert_eq!(result, "payload");

        let raw = "header";
        let result = extract_header_and_payload(raw);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fetch_token_success() {
        let mut mockito_server = mockito::Server::new_async().await;
        let _m = mockito_server
            .mock("POST", "/auth_url")
            .match_header("apikey", "test_api_key")
            .match_body(Matcher::Json(json!({"tenant": "test_tenant"})))
            .with_status(200)
            .with_body("test_token")
            .create();

        let client = reqwest::Client::new();
        let result = RestToken::fetch_token(
            &client,
            "test_tenant",
            "test_api_key",
            &format!("{}/auth_url", mockito_server.url()),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_token");
    }

    #[tokio::test]
    async fn test_fetch_token_failure() {
        let mut mockito_server = mockito::Server::new_async().await;
        let _m = mockito_server
            .mock("POST", "/auth_url")
            .match_header("apikey", "test_api_key")
            .match_body(Matcher::Json(json!({"tenant": "test_tenant"})))
            .with_status(400)
            .with_body("error_response")
            .create();

        let client = reqwest::Client::new();
        let result = RestToken::fetch_token(
            &client,
            "test_tenant",
            "test_api_key",
            &format!("{}/auth_url", mockito_server.url()),
        )
        .await;

        assert!(result.is_err());
        if let Err(DshError::DshCallError {
            url,
            status_code,
            error_body,
        }) = result
        {
            assert_eq!(url, format!("{}/auth_url", mockito_server.url()));
            assert_eq!(status_code, reqwest::StatusCode::BAD_REQUEST);
            assert_eq!(error_body, "error_response");
        } else {
            panic!("Expected DshCallError");
        }
    }
}
