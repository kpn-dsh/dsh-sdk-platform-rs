use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::rest_token::RestToken;
use super::{JwtToken, ProtocolTokenError};

/// Represents attributes associated with a mqtt token.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DataAccessToken {
    gen: i32,
    endpoint: String,
    ports: Ports,
    iss: String,
    claims: Vec<TopicPermission>,
    exp: i64,
    client_id: String,
    iat: i32,
    tenant_id: String,
    #[serde(skip)]
    raw_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ports {
    mqtts: Vec<u16>,
    mqttwss: Vec<u16>,
}

/// Represent Claims information for MQTT request
/// * `action` - can be subscribe or publish
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct TopicPermission {
    /// Publish or Subscribe
    action: Action,
    /// The resource to define what the client can access in terms of stream, prefix, topic, and type.
    resource: Resource,
}

/// Enumeration representing possible actions in MQTT claims.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub enum Action {
    #[serde(alias = "publish")]
    Publish,
    #[serde(alias = "subscribe")]
    Subscribe,
}

/// Represents a resource in the MQTT claim.
///
/// The resource defines what the client can access in terms of stream, prefix, topic, and type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
struct Resource {
    /// The type of the resource (always "topic").
    #[serde(rename = "type")]
    resource_type: String,
    /// data stream name, e.g. weather or ivi
    stream: String,
    /// topic prefix, e.g. /tt
    prefix: String,
    /// topic pattern, e.g. +/+/+/something/#
    topic: String,
}

/// Request for geting a [DataAccessToken] which can be used to authenticate to the DSH Mqtt or Http brokers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestDataAccessToken {
    /// Tenant name
    tenant: String,
    /// Unique client ID that must be used when connecting to the broker
    id: String,
    /// Requested expiration time (in seconds since UNIX epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>,
    /// Optional list of topic permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    claims: Option<Vec<TopicPermission>>,
    /// DSH Client Claims optional field for commiumicating between external clients and DSH
    #[serde(skip_serializing_if = "Option::is_none")]
    dshclc: Option<serde_json::Value>,
}

impl DataAccessToken {
    pub fn parse(raw_token: impl Into<String>) -> Result<Self, ProtocolTokenError> {
        let raw_token = raw_token.into();
        let jwt_token = JwtToken::parse(&raw_token)?;

        let mut token: Self = serde_json::from_slice(&jwt_token.b64_decode_payload()?)?;
        token.raw_token = raw_token;

        Ok(token)
    }

    pub(crate) fn init() -> Self {
        Self {
            gen: 0,
            endpoint: "".to_string(),
            ports: Ports {
                mqtts: vec![],
                mqttwss: vec![],
            },
            iss: "".to_string(),
            claims: Vec::new(),
            exp: 0,
            client_id: "".to_string(),
            iat: 0,
            tenant_id: "".to_string(),
            raw_token: "".to_string(),
        }
    }

    pub fn gen(&self) -> i32 {
        self.gen
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn endpoint_wss(&self) -> String {
        format!("wss://{}/mqtt", self.endpoint)
    }

    pub fn port_mqtt(&self) -> u16 {
        *self.ports.mqtts.get(0).unwrap_or(&8883)
    }

    pub fn port_wss(&self) -> u16 {
        *self.ports.mqttwss.get(0).unwrap_or(&443)
    }

    pub fn ports(&self) -> &Ports {
        &self.ports
    }

    pub fn iss(&self) -> &str {
        &self.iss
    }

    pub fn claims(&self) -> &Vec<TopicPermission> {
        &self.claims
    }

    pub fn exp(&self) -> i64 {
        self.exp
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn iat(&self) -> i32 {
        self.iat
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn raw_token(&self) -> &str {
        &self.raw_token
    }

    pub fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i64;
        self.exp >= current_unixtime + 5 && !self.raw_token.is_empty()
    }
}

impl std::fmt::Debug for DataAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DataAccessToken")
            .field("gen", &self.gen)
            .field("endpoint", &self.endpoint)
            .field("iss", &self.iss)
            .field("claims", &self.claims)
            .field("exp", &self.exp)
            .field("client_id", &self.client_id)
            .field("iat", &self.iat)
            .field("tenant_id", &self.tenant_id)
            .field(
                "raw_token",
                &self
                    .raw_token
                    .split('.')
                    .take(2)
                    .collect::<Vec<&str>>()
                    .join("."),
            )
            .finish()
    }
}

impl TopicPermission {
    /// Creates a new [`TopicPermission`] instance.
    ///
    /// # Arguments
    ///
    /// * `resource` - the resource to define what the client can access in terms of stream, prefix, topic, and type.
    /// * `action` - the action to define what the client can do with the resource.
    ///
    /// # Returns
    ///
    /// Returns a new [`TopicPermission`] instance.
    pub fn new(
        action: Action,
        stream: impl Into<String>,
        prefix: impl Into<String>,
        topic: impl Into<String>,
    ) -> Self {
        let resource = Resource::new(stream, prefix, topic);
        Self { resource, action }
    }

    /// Returns the full qualified topic name of resource.
    pub fn full_qualified_topic_name(&self) -> String {
        format!(
            "{}/{}/{}",
            self.resource.prefix, self.resource.stream, self.resource.topic
        )
    }

    /// topic prefix, e.g. /tt
    pub fn prefix(&self) -> &str {
        &self.resource.prefix
    }

    /// data stream name, e.g. `weather` or `ivi`
    pub fn stream(&self) -> &str {
        &self.resource.stream
    }

    /// topic pattern, e.g. `+/+/+/something/#`
    pub fn topic_pattern(&self) -> &str {
        &self.resource.topic
    }

    /// Returns the action to define what the client can do with the resource.
    pub fn action(&self) -> &Action {
        &self.action
    }
}

impl Resource {
    /// Creates a new [`Resource`] instance.
    ///
    /// # Arguments
    ///
    /// * `stream` - data stream name, e.g. `weather` or `ivi`
    /// * `prefix` - topic prefix, e.g.`/tt`
    /// * `topic` - topic pattern, e.g. `+/+/+/something/#`
    ///
    /// # Returns
    ///
    /// Returns a new [`Resource`] instance.
    pub fn new(
        stream: impl Into<String>,
        prefix: impl Into<String>,
        topic: impl Into<String>,
    ) -> Self {
        Self {
            stream: stream.into(),
            prefix: prefix.into(),
            topic: topic.into(),
            resource_type: "topic".to_string(), // always topic
        }
    }
}

impl RequestDataAccessToken {
    ///
    /// client_id: Has a maximum of 64 characters
    ///     Can only contain:
    ///     haracters (a-z, A-z, 0-9)
    ///     @, -, _, . and :
    pub fn new(tenant: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self {
            tenant: tenant.into(),
            id: client_id.into(),
            exp: None,
            claims: None,
            dshclc: None,
        }
    }

    pub fn tenant(&self) -> &str {
        &self.tenant
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the requested expiration time for the token.
    pub fn set_exp(mut self, exp: i64) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Returns the requested expiration time for the token.
    pub fn exp(&self) -> Option<i64> {
        self.exp
    }

    /// Set a list of [`TopicPermission`] for the token.
    pub fn set_claims(mut self, claims: Vec<TopicPermission>) -> Self {
        self.claims = Some(claims);
        self
    }

    /// Extend the list of [`TopicPermission`] for the token.
    pub fn extend_claims(mut self, claims: impl Iterator<Item = TopicPermission>) -> Self {
        self.claims.get_or_insert_with(Vec::new).extend(claims);
        self
    }

    /// Returns the list of [`TopicPermission`] for the token.
    pub fn claims(&self) -> Option<&Vec<TopicPermission>> {
        self.claims.as_ref()
    }

    /// Set the DSH Client Claims.
    ///
    /// This field is optional and can be used to communicate between external clients and the API client authentication service.
    pub fn set_dshclc(mut self, dshclc: impl Into<serde_json::Value>) -> Self {
        self.dshclc = Some(dshclc.into());
        self
    }

    /// Returns the DSH Client Claims.
    pub fn dshclc(&self) -> Option<&serde_json::Value> {
        self.dshclc.as_ref()
    }

    /// Send the request to the DSH platform to get a [`DataAccessToken`].
    ///
    /// # Arguments
    /// - `client` - The reqwest client to use for the request.
    /// - `rest_token` - The rest token to use for the request.   
    pub async fn send(
        &self,
        client: &reqwest::Client,
        rest_token: RestToken,
    ) -> Result<DataAccessToken, ProtocolTokenError> {
        super::validate_client_id(&self.id)?;

        let auth_url = format!(
            "https://{}/datastreams/v0/mqtt/token",
            rest_token.endpoint(),
        );
        log::debug!("Sending request to '{}': {:?}", auth_url, self);
        let response = client
            .post(&auth_url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", rest_token.raw_token()),
            )
            .json(self)
            .send()
            .await?;
        let status = response.status();
        let body_text = response.text().await?;
        match status {
            reqwest::StatusCode::OK => Ok(DataAccessToken::parse(body_text)?),
            _ => Err(ProtocolTokenError::DshCall {
                url: auth_url,
                status_code: status,
                error_body: body_text,
            }),
        }
    }
}

impl PartialEq for RequestDataAccessToken {
    fn eq(&self, other: &Self) -> bool {
        // Ignore the exp field
        self.tenant == other.tenant
            && self.id == other.id
            && self.claims == other.claims
            && self.dshclc == other.dshclc
    }
}

impl Hash for RequestDataAccessToken {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Ignore the exp field
        self.tenant.hash(state);
        self.id.hash(state);
        self.claims.hash(state);
        self.dshclc.hash(state);
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Publish => write!(f, "publish"),
            Self::Subscribe => write!(f, "subscribe"),
        }
    }
}

#[cfg(test)]
mod test {}
