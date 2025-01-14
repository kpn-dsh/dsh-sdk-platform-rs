//! Provides an enum of DSH platforms and related metadata.
//!
//! This module defines the [`Platform`] enum, representing different DSH deployments,
//! each with its own realm, REST API endpoints, and token endpoints. The platform choice
//! influences how you authenticate and where you send REST/Protocol requests.
//!
//! # Platforms
//! The platforms defined are:
//! - `Prod` (kpn-dsh.com)
//! - `ProdAz` (az.kpn-dsh.com)
//! - `ProdLz` (dsh-prod.dsh.prod.aws.kpn.com)
//! - `NpLz` (dsh-dev.dsh.np.aws.kpn.com)
//! - `Poc` (poc.kpn-dsh.com)
//!
//! ## Usage
//! Use a [`Platform`] variant to generate appropriate URLs and client IDs for your environment.
//! For example, you might select `Platform::NpLz` when deploying a service to the development
//! landing zone.

/// Represents an available DSH platform and its related metadata.
///
/// The platform defined are:
/// - `Prod` (kpn-dsh.com)
/// - `ProdAz` (az.kpn-dsh.com)
/// - `ProdLz` (dsh-prod.dsh.prod.aws.kpn.com)
/// - `NpLz` (dsh-dev.dsh.np.aws.kpn.com)
/// - `Poc` (poc.kpn-dsh.com)
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Platform {
    /// Production platform (`kpn-dsh.com`).
    Prod,
    /// Production platform on Azure (`az.kpn-dsh.com`).
    ProdAz,
    /// Production Landing Zone on AWS (`dsh-prod.dsh.prod.aws.kpn.com`).
    ProdLz,
    /// Non-Production (Dev) Landing Zone on AWS (`dsh-dev.dsh.np.aws.kpn.com`).
    NpLz,
    /// Proof of Concept platform (`poc.kpn-dsh.com`).
    Poc,
}

impl Platform {
    /// Returns a properly formatted client ID for the DSH REST API, given a tenant name.
    ///
    /// The format is:  
    /// \[
    ///    `"robot:{realm}:{tenant_name}"`
    /// \]
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let client_id = platform.rest_client_id("my-tenant");
    /// assert_eq!(client_id, "robot:dev-lz-dsh:my-tenant");
    /// ```
    pub fn rest_client_id(&self, tenant: impl AsRef<str>) -> String {
        format!("robot:{}:{}", self.realm(), tenant.as_ref())
    }

    /// Returns the base URL for the DSH REST API, depending on the platform.
    ///
    /// This endpoint is typically used for general resource operations in DSH.  
    ///
    /// # Example
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

    /// Returns the URL endpoint for retrieving DSH REST API OAuth tokens.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let token_url = platform.endpoint_rest_access_token();
    /// assert_eq!(token_url, "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token");
    /// ```
    pub fn endpoint_rest_access_token(&self) -> &str {
        match self {
            Self::Prod => "https://auth.prod.cp.kpn-dsh.com/auth/realms/tt-dsh/protocol/openid-connect/token",
            Self::NpLz => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token",
            Self::ProdLz => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/prod-lz-dsh/protocol/openid-connect/token",
            Self::ProdAz => "https://auth.prod.cp.kpn-dsh.com/auth/realms/prod-azure-dsh/protocol/openid-connect/token",
            Self::Poc => "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token",
        }
    }

    /// (Deprecated) Returns the DSH REST authentication token endpoint.
    ///
    /// *Prefer using [`endpoint_management_api_token`](Self::endpoint_management_api_token) instead.*
    #[deprecated(since = "0.5.0", note = "Use `endpoint_management_api_token` instead")]
    pub fn endpoint_rest_token(&self) -> &str {
        self.endpoint_management_api_token()
    }

    /// Returns the endpoint for fetching a DSH Management API authentication token.
    ///
    /// This endpoint is typically used to authenticate requests to certain management or admin-level
    /// DSH services.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let mgmt_token_url = platform.endpoint_management_api_token();
    /// assert_eq!(mgmt_token_url, "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token");
    /// ```
    pub fn endpoint_management_api_token(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/auth/v0/token",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/auth/v0/token",
            Self::ProdAz => "https://api.az.kpn-dsh.com/auth/v0/token",
            Self::Poc => "https://api.poc.kpn-dsh.com/auth/v0/token",
        }
    }

    /// (Deprecated) Returns the DSH MQTT token endpoint.
    ///
    /// *Prefer using [`endpoint_protocol_token`](Self::endpoint_protocol_token) instead.*
    #[deprecated(since = "0.5.0", note = "Use `endpoint_protocol_token` instead")]
    pub fn endpoint_mqtt_token(&self) -> &str {
        self.endpoint_protocol_token()
    }

    /// Returns the endpoint for fetching DSH protocol tokens (e.g., for MQTT).
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::Prod;
    /// let protocol_token_url = platform.endpoint_protocol_token();
    /// assert_eq!(protocol_token_url, "https://api.kpn-dsh.com/datastreams/v0/mqtt/token");
    /// ```
    pub fn endpoint_protocol_token(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/datastreams/v0/mqtt/token",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/datastreams/v0/mqtt/token",
            Self::ProdAz => "https://api.az.kpn-dsh.com/datastreams/v0/mqtt/token",
            Self::Poc => "https://api.poc.kpn-dsh.com/datastreams/v0/mqtt/token",
        }
    }

    /// Returns the Keycloak realm string associated with this platform.
    ///
    /// This is used to construct OpenID Connect tokens (e.g., for Kafka or REST API authentication).
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let realm = Platform::Prod.realm();
    /// assert_eq!(realm, "tt-dsh");
    /// ```
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
