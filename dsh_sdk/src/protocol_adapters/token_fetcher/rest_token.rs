//! Protocol Rest token
//!
//!

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::{JwtToken, ProtocolTokenError};

/// Token to request a [`DataAccessToken`](super::data_access_token::DataAccessToken).
///
/// The token is used to fetch a [`DataAccessToken`](super::data_access_token::DataAccessToken).
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RestToken {
    gen: i64,
    endpoint: String,
    iss: String,
    claims: Claims,
    exp: i64,
    tenant_id: String,
    #[serde(skip)]
    raw_token: String,
}

/// Represents the claims for the Rest token
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Claims {
    // TODO: inverstigate if this is complete
    #[serde(rename = "datastreams/v0/mqtt/token")]
    mqtt_token_claim: DatastreamsMqttTokenClaim,
}

/// Represents the claims for the "datastreams/v0/mqtt/token" endpoint
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DatastreamsMqttTokenClaim {
    /// External Client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// Tenant name
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant: Option<String>,
    /// Maximum token lifetime in seconds for to be requested [`DataAccessToken`](super::data_access_token::DataAccessToken) in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    relexp: Option<i32>,
    /// Requested expiration time in seconds (in seconds since UNIX epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i32>,
    /// Requested expiration time in seconds (in seconds since UNIX epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    claims: Option<Vec<()>>, // TODO: investigate which claims are possible
}

/// Request for geting a Rest token which can be used to get a [`DataAccessToken`](super::data_access_token::DataAccessToken)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RequestRestToken {
    /// Tenant name
    tenant: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requested expiration time in seconds (in seconds since UNIX epoch)
    exp: Option<i64>,
    /// Requested claims and permissions that the [`DataAccessToken`](super::data_access_token::DataAccessToken) should have
    #[serde(skip_serializing_if = "Option::is_none")]
    claims: Option<Claims>,
}

impl RestToken {
    /// Creates a new [`RestToken`] instance based on a JWT Token.
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
            iss: "".to_string(),
            claims: Claims::default(),
            exp: 0,
            tenant_id: "".to_string(),
            raw_token: "".to_string(),
        }
    }

    pub fn gen(&self) -> i64 {
        self.gen
    }

    /// Returns the endpoint which the MQTT client should connect to
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the iss
    pub fn iss(&self) -> &str {
        &self.iss
    }

    /// Returns the claims
    ///
    /// The claims are (optional) API endpoints and restrictions that the token can access.
    /// If no claims are present, the token will have full access to all endpoints.
    pub fn claims(&self) -> &Claims {
        &self.claims
    }

    /// Returns the expiration time (in seconds since UNIX epoch)
    pub fn exp(&self) -> i64 {
        self.exp
    }

    /// Returns the tenant id
    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    /// Returns the raw token
    pub fn raw_token(&self) -> &str {
        &self.raw_token
    }
    // Checks if the token is valid
    pub fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i64;
        self.exp >= current_unixtime + 5 && !self.raw_token.is_empty()
    }
}

impl std::fmt::Debug for RestToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RestToken")
            .field("gen", &self.gen)
            .field("endpoint", &self.endpoint)
            .field("iss", &self.iss)
            .field("claims", &self.claims)
            .field("exp", &self.exp)
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

impl DatastreamsMqttTokenClaim {
    /// Creates a new `DatastreamsMqttTokenClaim` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the external client ID for which the [`RestToken`]] is requested
    pub fn set_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Returns the external client ID for which the [`RestToken`] is requested
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Sets the tenant name
    pub fn set_tenant(mut self, tenant: impl Into<String>) -> Self {
        self.tenant = Some(tenant.into());
        self
    }

    /// Returns the tenant name
    pub fn tenant(&self) -> Option<&str> {
        self.tenant.as_deref()
    }

    /// Sets the requested expiration time in seconds for [`DataAccessToken`](super::data_access_token::DataAccessToken) (in seconds since from now)
    pub fn set_relexp(mut self, relexp: i32) -> Self {
        self.relexp = Some(relexp);
        self
    }

    /// Returns the requested expiration time in seconds (in seconds since from now)
    pub fn relexp(&self) -> Option<i32> {
        self.relexp
    }

    /// Sets the requested expiration time in seconds (in seconds since UNIX epoch)
    pub fn set_exp(mut self, exp: i32) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Returns the requested expiration time in seconds (in seconds since UNIX epoch)
    pub fn exp(&self) -> Option<i32> {
        self.exp
    }
}

impl Default for DatastreamsMqttTokenClaim {
    fn default() -> Self {
        Self {
            id: None,
            tenant: None,
            relexp: None,
            exp: None,
            claims: None,
        }
    }
}

impl RequestRestToken {
    /// Creates a new [`RequestRestToken`] instance with full access request.
    pub fn new(tenant: impl Into<String>) -> Self {
        Self {
            tenant: tenant.into(),
            exp: None,
            claims: None,
        }
    }

    /// Send the request to the DSH platform to get a [`RestToken`].
    ///
    /// # Arguments
    /// - `client` - The reqwest client to use for the request.
    /// - `api_key` - The API key to authenticate to the DSH platform.
    /// - `auth_url` - The URL of the DSH platform to send the request to (See [Platform::endpoint_protocol_rest_token](crate::Platform::endpoint_protocol_rest_token)).
    ///
    /// # Returns

    pub async fn send(
        &self,
        client: &reqwest::Client,
        api_key: &str,
        auth_url: &str,
    ) -> Result<RestToken, ProtocolTokenError> {
        log::debug!("Sending request to '{}': {:?}", auth_url, self);
        let response = client
            .post(auth_url)
            .header("apikey", api_key)
            .json(self)
            .send()
            .await?;

        let status = response.status();
        let body_text = response.text().await?;
        match status {
            reqwest::StatusCode::OK => Ok(RestToken::parse(body_text)?),
            _ => Err(ProtocolTokenError::DshCall {
                url: auth_url.to_string(),
                status_code: status,
                error_body: body_text,
            }),
        }
    }

    /// Returns the tenant
    pub fn tenant(&self) -> &str {
        &self.tenant
    }

    /// Sets the expiration time (in seconds since UNIX epoch)
    pub fn set_exp(mut self, exp: i64) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Returns the expiration time (in seconds since UNIX epoch)
    pub fn exp(&self) -> Option<i64> {
        self.exp
    }

    /// Sets the claims
    pub fn set_claims(mut self, claims: impl Into<Claims>) -> Self {
        self.claims = Some(claims.into());
        self
    }
    /// Returns the claims
    pub fn claims(&self) -> Option<&Claims> {
        self.claims.as_ref()
    }
}

impl PartialEq for RequestRestToken {
    fn eq(&self, other: &Self) -> bool {
        // Ignore the requested expiration time, not relevant for equality
        self.tenant == other.tenant && self.claims == other.claims
    }
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            mqtt_token_claim: DatastreamsMqttTokenClaim::default(),
        }
    }
}

impl From<DatastreamsMqttTokenClaim> for Claims {
    fn from(claim: DatastreamsMqttTokenClaim) -> Self {
        Self {
            mqtt_token_claim: claim,
        }
    }
}

impl Claims {
    pub fn set_mqtt_token_claim(mut self, claim: DatastreamsMqttTokenClaim) -> Self {
        self.mqtt_token_claim = claim;
        self
    }

    /// Returns the MQTT token claim
    pub fn mqtt_token_claim(&self) -> &DatastreamsMqttTokenClaim {
        &self.mqtt_token_claim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let raw_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJzdHJpbmciLCJnZW4iOjEsImV4cCI6MTczOTU0Nzg3OCwidGVuYW50LWlkIjoiZm9vIiwiZW5kcG9pbnQiOiJ0ZXN0X2VuZHBvaW50IiwiY2xhaW1zIjp7ImRhdGFzdHJlYW1zL3YwL21xdHQvdG9rZW4iOnsiaWQiOiJqdXN0LXRoaXMtZGV2aWNlIiwiZXhwIjoxNzM5NTQ3ODc4LCJ0ZW5hbnQiOiJmb28iLCJjbGFpbXMiOltdfX19.signature";
        let token = RestToken::parse(raw_token.to_string()).unwrap();
        assert_eq!(token.gen(), 1);
        assert_eq!(token.endpoint(), "test_endpoint");
        assert_eq!(token.iss(), "string");
        assert_eq!(
            token.claims().mqtt_token_claim.id,
            Some("just-this-device".to_string())
        );
        assert_eq!(token.claims().mqtt_token_claim.exp, Some(1739547878));
        assert_eq!(
            token.claims().mqtt_token_claim.tenant,
            Some("foo".to_string())
        );
        assert_eq!(token.exp(), 1739547878);
        assert_eq!(token.tenant_id(), "foo");
        assert_eq!(token.raw_token(), raw_token);
    }

    #[test]
    fn test_datastreams_mqtt_token_claim() {
        let claim = DatastreamsMqttTokenClaim::new();
        assert_eq!(claim.id(), None);
        assert_eq!(claim.tenant(), None);
        assert_eq!(claim.relexp(), None);
        assert_eq!(claim.exp(), None);
        let claim = claim
            .set_id("test-id")
            .set_tenant("test-tenant")
            .set_relexp(100)
            .set_exp(200);
        assert_eq!(claim.id(), Some("test-id"));
        assert_eq!(claim.tenant(), Some("test-tenant"));
        assert_eq!(claim.relexp(), Some(100));
        assert_eq!(claim.exp(), Some(200));
    }

    #[test]
    fn test_rest_token_request() {
        let request = RequestRestToken::new("test-tenant");
        assert_eq!(request.tenant(), "test-tenant");
        assert_eq!(request.exp(), None);
        assert_eq!(request.claims(), None);
        let claims: Claims = DatastreamsMqttTokenClaim::new().set_exp(1).into();
        let request = request.set_exp(100).set_claims(claims.clone());
        let request = request;
        assert_eq!(request.exp(), Some(100));
        assert_eq!(request.claims(), Some(&claims))
    }
}
