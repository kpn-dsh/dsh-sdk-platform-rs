//! Access Token to authenticate to the DSH Mqtt or Http brokers
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::claims::TopicPermission;
use crate::protocol_adapters::token::{JwtToken, ProtocolTokenError};

/// Access Token to authenticate to the DSH Mqtt or Http brokers
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DataAccessToken {
    #[serde(rename = "gen")]
    generated: i32,
    pub(crate) endpoint: String,
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

impl DataAccessToken {
    /// Creates a new [`DataAccessToken`] instance based on a raw JWT Token.
    pub fn parse(raw_token: impl Into<String>) -> Result<Self, ProtocolTokenError> {
        let raw_token = raw_token.into();
        let jwt_token = JwtToken::parse(&raw_token)?;

        let mut token: Self = serde_json::from_slice(&jwt_token.b64_decode_payload()?)?;
        token.raw_token = raw_token;

        Ok(token)
    }

    pub(crate) fn init() -> Self {
        Self {
            generated: 0,
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

    /// Returns the generation of the token.
    ///
    /// An alias for `gen` to match the original token format.
    pub fn generated(&self) -> i32 {
        self.generated
    }

    /// Returns the endpoint which the MQTT client should connect to.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the endpoint which the MQTT websocket client should connect to.
    pub fn endpoint_wss(&self) -> String {
        format!("wss://{}/mqtt", self.endpoint)
    }

    /// Returns the port number which the MQTT client should connect to for `mqtt` protocol.
    pub fn port_mqtt(&self) -> u16 {
        *self.ports.mqtts.get(0).unwrap_or(&8883)
    }

    /// Returns the port number which the MQTT client should connect to for `websocket` protocol.
    pub fn port_wss(&self) -> u16 {
        *self.ports.mqttwss.get(0).unwrap_or(&443)
    }

    /// Returns the [`Ports`] which the MQTT client can connect to.
    pub fn ports(&self) -> &Ports {
        &self.ports
    }

    /// Returns the iss.
    pub fn iss(&self) -> &str {
        &self.iss
    }

    /// Returns the [`TopicPermission`] of the token
    pub fn claims(&self) -> &Vec<TopicPermission> {
        &self.claims
    }

    /// Returns the expiration time (in seconds since UNIX epoch).
    pub fn exp(&self) -> i64 {
        self.exp
    }

    /// Returns the client_id
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns the issued at time (in seconds since UNIX epoch).
    pub fn iat(&self) -> i32 {
        self.iat
    }

    /// Returns the tenant name.
    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    /// Returns the raw JWT token.
    pub fn raw_token(&self) -> &str {
        &self.raw_token
    }

    /// Checks if the token is valid.
    pub fn is_valid(&self) -> bool {
        let current_unixtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as i64;
        self.exp >= current_unixtime + 5 && !self.raw_token.is_empty()
    }
}

impl Ports {
    pub fn mqtts(&self) -> &Vec<u16> {
        &self.mqtts
    }

    pub fn mqttwss(&self) -> &Vec<u16> {
        &self.mqttwss
    }
}

impl std::fmt::Debug for DataAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DataAccessToken")
            .field("gen", &self.generated)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data_access_token() {
        let raw_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywiaWF0IjoyMTQ3NDgzNjQ3LCJlbmRwb2ludCI6InRlc3RfZW5kcG9pbnQiLCJwb3J0cyI6eyJtcXR0cyI6Wzg4ODNdLCJtcXR0d3NzIjpbNDQzLDg0NDNdfSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJjbGllbnQtaWQiOiJ0ZXN0X2NsaWVudCIsImNsYWltcyI6W3siYWN0aW9uIjoic3Vic2NyaWJlIiwicmVzb3VyY2UiOnsidHlwZSI6InRvcGljIiwicHJlZml4IjoiL3R0Iiwic3RyZWFtIjoidGVzdCIsInRvcGljIjoiL3Rlc3QvIyJ9fV19.LwYIMIX39J502TDqpEqH5T2Rlj-HczeT3WLfs5Do3B0";
        let token = DataAccessToken::parse(raw_token).unwrap();
        assert_eq!(token.generated(), 1);
        assert_eq!(token.endpoint(), "test_endpoint");
        assert_eq!(token.port_mqtt(), 8883);
        assert_eq!(token.port_wss(), 443);
        assert_eq!(token.iss(), "String");
        assert_eq!(token.exp(), 2147483647);
        assert_eq!(token.iat(), 2147483647);
        assert_eq!(token.client_id(), "test_client");
        assert_eq!(token.tenant_id(), "test_tenant");
        assert_eq!(token.raw_token(), raw_token);
        assert!(token.is_valid());
    }

    #[test]
    fn test_init_data_access_token() {
        let token = DataAccessToken::init();
        assert_eq!(token.generated(), 0);
        assert_eq!(token.endpoint(), "");
        assert_eq!(token.port_mqtt(), 8883);
        assert_eq!(token.port_wss(), 443);
        assert_eq!(token.iss(), "");
        assert_eq!(token.exp(), 0);
        assert_eq!(token.iat(), 0);
        assert_eq!(token.client_id(), "");
        assert_eq!(token.tenant_id(), "");
        assert_eq!(token.raw_token(), "");
        assert!(!token.is_valid());
    }

    #[test]
    fn test_is_valid_data_access_token() {
        let mut token = DataAccessToken::init();
        assert!(!token.is_valid());
        token.exp = 1;
        assert!(!token.is_valid());
        token.raw_token = "test".to_string();
        assert!(!token.is_valid());
        token.exp = 2147483647;
        assert!(token.is_valid());
    }

    #[test]
    fn test_debug_data_access_token() {
        let raw_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywiaWF0IjoyMTQ3NDgzNjQ3LCJlbmRwb2ludCI6InRlc3RfZW5kcG9pbnQiLCJwb3J0cyI6eyJtcXR0cyI6Wzg4ODNdLCJtcXR0d3NzIjpbNDQzLDg0NDNdfSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJjbGllbnQtaWQiOiJ0ZXN0X2NsaWVudCIsImNsYWltcyI6W3siYWN0aW9uIjoic3Vic2NyaWJlIiwicmVzb3VyY2UiOnsidHlwZSI6InRvcGljIiwicHJlZml4IjoiL3R0Iiwic3RyZWFtIjoidGVzdCIsInRvcGljIjoiL3Rlc3QvIyJ9fV19.LwYIMIX39J502TDqpEqH5T2Rlj-HczeT3WLfs5Do3B0";
        let token = DataAccessToken::parse(raw_token).unwrap();
        let debug = format!("{:?}", token);
        assert_eq!(
            debug,
            "DataAccessToken { gen: 1, endpoint: \"test_endpoint\", iss: \"String\", claims: [TopicPermission { action: Subscribe, resource: Resource { resource_type: \"topic\", stream: \"test\", prefix: \"/tt\", topic: \"/test/#\" } }], exp: 2147483647, client_id: \"test_client\", iat: 2147483647, tenant_id: \"test_tenant\", raw_token: \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywiaWF0IjoyMTQ3NDgzNjQ3LCJlbmRwb2ludCI6InRlc3RfZW5kcG9pbnQiLCJwb3J0cyI6eyJtcXR0cyI6Wzg4ODNdLCJtcXR0d3NzIjpbNDQzLDg0NDNdfSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJjbGllbnQtaWQiOiJ0ZXN0X2NsaWVudCIsImNsYWltcyI6W3siYWN0aW9uIjoic3Vic2NyaWJlIiwicmVzb3VyY2UiOnsidHlwZSI6InRvcGljIiwicHJlZml4IjoiL3R0Iiwic3RyZWFtIjoidGVzdCIsInRvcGljIjoiL3Rlc3QvIyJ9fV19\" }"
        );
        let init_token = format!("{:?}", DataAccessToken::init());
        let debug = format!("{:?}", init_token);
        assert_eq!(
            debug,
            "\"DataAccessToken { gen: 0, endpoint: \\\"\\\", iss: \\\"\\\", claims: [], exp: 0, client_id: \\\"\\\", iat: 0, tenant_id: \\\"\\\", raw_token: \\\"\\\" }\""
        );
    }
}
