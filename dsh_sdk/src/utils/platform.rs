/// Available DSH platforms plus it's related metadata
///
/// The platform enum contains
/// - `Prod` (kpn-dsh.com)
/// - `ProdAz` (az.kpn-dsh.com)
/// - `ProdLz` (dsh-prod.dsh.prod.aws.kpn.com)
/// - `NpLz` (dsh-dev.dsh.np.aws.kpn.com)
/// - `Poc` (poc.kpn-dsh.com)
///
/// Each platform has it's own realm, endpoint for the DSH Rest API and endpoint for the DSH Rest API access token.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Platform {
    /// Production platform (kpn-dsh.com)
    Prod,
    /// Production platform on Azure (az.kpn-dsh.com)
    ProdAz,
    /// Production Landing Zone on AWS (dsh-prod.dsh.prod.aws.kpn.com)
    ProdLz,
    /// Non-Production (Dev) Landing Zone on AWS (dsh-dev.dsh.np.aws.kpn.com)
    NpLz,
    /// Proof of Concept platform (poc.kpn-dsh.com)
    Poc,
}

impl Platform {
    /// Get a properly formatted client_id for the Rest API based on the given name of a tenant
    ///
    /// It will return a string formatted as "robot:{realm}:{tenant_name}"
    ///
    /// ## Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let client_id = platform.rest_client_id("my-tenant");
    /// assert_eq!(client_id, "robot:dev-lz-dsh:my-tenant");
    /// ```
    pub fn rest_client_id<T>(&self, tenant: T) -> String
    where
        T: AsRef<str>,
    {
        format!("robot:{}:{}", self.realm(), tenant.as_ref())
    }

    /// Get the endpoint for the DSH Rest API
    ///
    /// It will return the endpoint for the DSH Rest API based on the platform
    ///
    /// ## Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let endpoint = platform.endpoint_rest_api();
    /// assert_eq!(endpoint, "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0");
    /// ```
    pub fn endpoint_rest_api(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/resources/v0",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/resources/v0",
            Self::ProdAz => "https://api.az.kpn-dsh.com/resources/v0",
            Self::Poc => "https://api.poc.kpn-dsh.com/resources/v0",
        }
    }
    /// Get the endpoint for the DSH Rest API access token
    ///
    /// It will return the endpoint for the DSH Rest API access token based on the platform
    ///
    /// ## Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let endpoint = platform.endpoint_rest_access_token();
    /// assert_eq!(endpoint, "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token");
    /// ```
    pub fn endpoint_rest_access_token(&self) -> &str {
        match self {
            Self::Prod =>   "https://auth.prod.cp.kpn-dsh.com/auth/realms/tt-dsh/protocol/openid-connect/token",
            Self::NpLz =>   "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token",
            Self::ProdLz => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/prod-lz-dsh/protocol/openid-connect/token",
            Self::ProdAz => "https://auth.prod.cp.kpn-dsh.com/auth/realms/prod-azure-dsh/protocol/openid-connect/token",
            Self::Poc =>    "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token", 
        }
    }

    #[deprecated(since = "0.5.0", note = "Use `endpoint_management_api_token` instead")]
    /// Get the endpoint for fetching DSH Rest Authentication Token
    ///
    /// With this token you can authenticate for the mqtt token endpoint
    ///
    /// It will return the endpoint for DSH Rest authentication token based on the platform
    pub fn endpoint_rest_token(&self) -> &str {
        self.endpoint_management_api_token()
    }

    /// Get the endpoint for fetching DSH Rest Authentication Token
    ///
    /// With this token you can authenticate for the mqtt token endpoint
    ///
    /// It will return the endpoint for DSH Rest authentication token based on the platform
    pub fn endpoint_management_api_token(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/auth/v0/token",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/auth/v0/token",
            Self::ProdAz => "https://api.az.kpn-dsh.com/auth/v0/token",
            Self::Poc => "https://api.poc.kpn-dsh.com/auth/v0/token",
        }
    }

    #[deprecated(since = "0.5.0", note = "Use `endpoint_protocol_token` instead")]
    /// Get the endpoint for fetching DSH mqtt token
    ///
    /// It will return the endpoint for DSH MQTT Token based on the platform
    pub fn endpoint_mqtt_token(&self) -> &str {
        self.endpoint_protocol_token()
    }

    /// Get the endpoint for fetching DSH Protocol token
    ///
    /// It will return the endpoint for DSH Protocol adapter Token based on the platform
    pub fn endpoint_protocol_token(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/datastreams/v0/mqtt/token",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/datastreams/v0/mqtt/token",
            Self::ProdAz => "https://api.az.kpn-dsh.com/datastreams/v0/mqtt/token",
            Self::Poc => "https://api.poc.kpn-dsh.com/datastreams/v0/mqtt/token",
        }
    }

    pub fn realm(&self) -> &str {
        match self {
            Self::Prod => "tt-dsh",
            Self::NpLz => "dev-lz-dsh",
            Self::ProdLz => "prod-lz-dsh",
            Self::ProdAz => "prod-azure-dsh",
            Self::Poc => "poc-dsh",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_platform_realm() {
        assert_eq!(Platform::NpLz.realm(), "dev-lz-dsh");
        assert_eq!(Platform::ProdLz.realm(), "prod-lz-dsh");
        assert_eq!(Platform::Poc.realm(), "poc-dsh");
    }

    #[test]
    fn test_platform_client_id() {
        assert_eq!(
            Platform::NpLz.rest_client_id("my-tenant"),
            "robot:dev-lz-dsh:my-tenant"
        );
        assert_eq!(
            Platform::ProdLz.rest_client_id("my-tenant".to_string()),
            "robot:prod-lz-dsh:my-tenant"
        );
        assert_eq!(
            Platform::Poc.rest_client_id("my-tenant"),
            "robot:poc-dsh:my-tenant"
        );
    }
}
