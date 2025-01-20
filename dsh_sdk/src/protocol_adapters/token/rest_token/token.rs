use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::claims::Claims;
use crate::protocol_adapters::token::{JwtToken, ProtocolTokenError};

/// Token to request a [`DataAccessToken`](crate::protocol_adapters::token::data_access_token::DataAccessToken).
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

    /// Returns the client_id if it is set in claims
    pub fn client_id(&self) -> Option<&str> {
        self.claims.mqtt_token_claim().id()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rest_token() {
        let raw_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJzdHJpbmciLCJnZW4iOjEsImV4cCI6MTczOTU0Nzg3OCwidGVuYW50LWlkIjoiZm9vIiwiZW5kcG9pbnQiOiJ0ZXN0X2VuZHBvaW50IiwiY2xhaW1zIjp7ImRhdGFzdHJlYW1zL3YwL21xdHQvdG9rZW4iOnsiaWQiOiJqdXN0LXRoaXMtZGV2aWNlIiwiZXhwIjoxNzM5NTQ3ODc4LCJ0ZW5hbnQiOiJmb28iLCJjbGFpbXMiOltdfX19.signature";
        let token = RestToken::parse(raw_token.to_string()).unwrap();
        assert_eq!(token.gen(), 1);
        assert_eq!(token.endpoint(), "test_endpoint");
        assert_eq!(token.iss(), "string");
        assert_eq!(
            token.claims().mqtt_token_claim().id(),
            Some("just-this-device")
        );
        assert_eq!(token.claims().mqtt_token_claim().exp(), Some(1739547878));
        assert_eq!(token.claims().mqtt_token_claim().tenant(), Some("foo"));
        assert_eq!(token.exp(), 1739547878);
        assert_eq!(token.tenant_id(), "foo");
        assert_eq!(token.raw_token(), raw_token);
    }

    #[test]
    fn test_init_rest_token() {
        let token = RestToken::init();
        assert_eq!(token.gen(), 0);
        assert_eq!(token.endpoint(), "");
        assert_eq!(token.iss(), "");
        assert_eq!(token.claims().mqtt_token_claim().id(), None);
        assert_eq!(token.claims().mqtt_token_claim().exp(), None);
        assert_eq!(token.claims().mqtt_token_claim().tenant(), None);
        assert_eq!(token.exp(), 0);
        assert_eq!(token.tenant_id(), "");
        assert_eq!(token.raw_token(), "");
    }

    #[test]
    fn test_is_valid() {
        let mut token = RestToken::init();
        assert!(!token.is_valid());
        token.exp = 0;
        assert!(!token.is_valid());
        token.raw_token = "test".to_string();
        assert!(!token.is_valid());
        token.exp = 2147483647;
        assert!(token.is_valid());
    }

    #[test]
    fn test_debug_rest_token() {
        let raw_token =  "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJlbmRwb2ludCI6Imh0dHA6Ly8xMjcuMC4wLjE6Nzk5OSIsImNsYWltcyI6eyJkYXRhc3RyZWFtcy92MC9tcXR0L3Rva2VuIjp7fX19.j5ekqMiWyBhJyRQE_aARFS9mQJiN7S2rpKTsn3rZ5lQ";
        let token = RestToken::parse(raw_token).unwrap();
        assert_eq!(
            format!("{:?}", token),
            "RestToken { gen: 1, endpoint: \"http://127.0.0.1:7999\", iss: \"String\", claims: Claims { mqtt_token_claim: DatastreamsMqttTokenClaim { id: None, tenant: None, relexp: None, exp: None, claims: None } }, exp: 1, tenant_id: \"test_tenant\", raw_token: \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJlbmRwb2ludCI6Imh0dHA6Ly8xMjcuMC4wLjE6Nzk5OSIsImNsYWltcyI6eyJkYXRhc3RyZWFtcy92MC9tcXR0L3Rva2VuIjp7fX19\" }"
        );
        let token = RestToken::init();
        assert_eq!(
            format!("{:?}", token),
            "RestToken { gen: 0, endpoint: \"\", iss: \"\", claims: Claims { mqtt_token_claim: DatastreamsMqttTokenClaim { id: None, tenant: None, relexp: None, exp: None, claims: None } }, exp: 0, tenant_id: \"\", raw_token: \"\" }"
        );
    }
}
