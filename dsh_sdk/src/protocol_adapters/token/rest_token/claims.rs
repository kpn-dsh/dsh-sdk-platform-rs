use serde::{Deserialize, Serialize};

/// Represents the claims for the [`RestToken`](super::RestToken)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Claims {
    // TODO: inverstigate if this is complete
    #[serde(rename = "datastreams/v0/mqtt/token")]
    mqtt_token_claim: DatastreamsMqttTokenClaim,
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            mqtt_token_claim: DatastreamsMqttTokenClaim::default(),
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

impl From<DatastreamsMqttTokenClaim> for Claims {
    fn from(claim: DatastreamsMqttTokenClaim) -> Self {
        Self {
            mqtt_token_claim: claim,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_from_mqtt_claims() {
        let claim = DatastreamsMqttTokenClaim::new();
        let claims: Claims = claim.clone().into();
        assert_eq!(claims.mqtt_token_claim(), &claim);
        let claim = claim
            .set_id("test-id")
            .set_tenant("test-tenant")
            .set_relexp(100)
            .set_exp(200);
        let claims: Claims = claim.clone().into();
        assert_eq!(claims.mqtt_token_claim(), &claim);
    }

    #[test]
    fn test_claims() {
        let claim = DatastreamsMqttTokenClaim::new();
        let claims = Claims::default();
        assert_eq!(claims.mqtt_token_claim(), &claim);
    }
}
