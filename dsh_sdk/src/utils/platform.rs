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
//! - `Custom` (for user-defined platforms)
//!
//! ## Usage
//! Use a [`Platform`] variant to generate appropriate URLs and client IDs for your environment.
//! For example, you might select `Platform::NpLz` when deploying a service to the development
//! landing zone.
//!
//! You can also create a [`Platform::Custom`] by providing the necessary endpoints and realm.
//!
//! Use the [`from_env`](Platform::from_env) method to automatically determine the platform based on the `DSH_ENVIRONMENT`
//! environment variable, which can be configured in a DSH Service Confifguration like this:
//! ```json
//! {
//! ...
//!  "env": {
//!    "DSH_ENVIRONMENT": "{ variables('DSH_ENVIRONMENT')}"
//!  },
//! ...
//!}
//!```
use crate::utils::{UtilsError, get_env_var};

const VAR_DSH_ENVIRONMENT: &str = "DSH_ENVIRONMENT";
const VAR_DSH_REALM: &str = "DSH_REALM";
const VAR_DSH_ENDPOINT_MANAGEMENT_API: &str = "DSH_ENDPOINT_MANAGEMENT_API";
const VAR_DSH_ENDPOINT_MANAGEMENT_API_TOKEN: &str = "DSH_ENDPOINT_MANAGEMENT_API_TOKEN";
const VAR_DSH_ENDPOINT_PROTOCOL_ACCESS_TOKEN: &str = "DSH_ENDPOINT_PROTOCOL_ACCESS_TOKEN";
const VAR_DSH_ENDPOINT_PROTOCOL_REST_TOKEN: &str = "DSH_ENDPOINT_PROTOCOL_REST_TOKEN";

/// Represents an available DSH platform and its related metadata.
///
/// The platform defined are:
/// - `Prod` (kpn-dsh.com)
/// - `ProdAz` (az.kpn-dsh.com)
/// - `ProdLz` (dsh-prod.dsh.prod.aws.kpn.com)
/// - `NpLz` (dsh-dev.dsh.np.aws.kpn.com)
/// - `Poc` (poc.kpn-dsh.com)
/// - `Custom` (for user-defined platforms)
///
/// Each platform has it's own realm, endpoint for the DSH Rest API and endpoint for the DSH Rest API access token.
///
/// ## Usage
/// Use a [`Platform`] variant to generate appropriate URLs and client IDs for your environment.
/// For example, you might select `Platform::NpLz` when deploying a service to the development
/// landing zone.
///
/// You can also create a [`Platform::Custom`] by providing the necessary endpoints and realm.
///
/// Use the [`from_env`](Platform::from_env) method to automatically determine the platform based on the `DSH_PLATFORM`
/// environment variable, which can be configured in a DSH Service Confifguration like this:
/// ```json
/// {
/// ...
///    "env": {
///      "DSH_ENVIRONMENT": "{ variables('DSH_ENVIRONMENT')}"
///    },
/// ...
///}
///```
#[derive(Clone, Debug, PartialEq)]
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
    /// Custom platform, not predefined.
    Custom {
        /// Realm name for the platform (e.g."poc-dsh").
        realm: String,
        /// Endpoint for the DSH Management API (e.g. "https://api.poc.kpn-dsh.com/resources/v0").
        endpoint_management_api: String,
        /// Endpoint for fetching a DSH Management API authentication token.
        /// (e.g. "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token").
        endpoint_management_api_token: String,
        /// Endpoint for fetching DSH protocol [Access Tokens](crate::protocol_adapters::token::data_access_token::DataAccessToken)
        /// (e.g. "https://api.poc.kpn-dsh.com/datastreams/v0/mqtt/token").
        endpoint_protocol_access_token: String,
        /// Endpoint for retrieving Protocol [Rest Tokens](crate::protocol_adapters::token::rest_token::RestToken)
        /// which is needed to request [Access Tokens](crate::protocol_adapters::token::data_access_token::DataAccessToken)
        /// (e.g. "https://api.poc.kpn-dsh.com/auth/v0/token").
        endpoint_protocol_rest_token: String,
    },
}

impl Platform {
    /// Returns a properly formatted client ID for the DSH Management API, given a tenant name.
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
    /// let client_id = platform.management_api_client_id("my-tenant");
    /// assert_eq!(client_id, "robot:dev-lz-dsh:my-tenant");
    /// ```
    pub fn management_api_client_id(&self, tenant: impl AsRef<str>) -> String {
        format!("robot:{}:{}", self.realm(), tenant.as_ref())
    }

    /// Returns the endpoint for the DSH Management API
    ///
    /// It will return the endpoint for the DSH Rest API based on the platform
    ///
    /// ## Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let endpoint = platform.endpoint_management_api();
    /// assert_eq!(endpoint, "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0");
    /// ```
    pub fn endpoint_management_api(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/resources/v0",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/resources/v0",
            Self::ProdAz => "https://api.az.kpn-dsh.com/resources/v0",
            Self::Poc => "https://api.poc.kpn-dsh.com/resources/v0",
            Self::Custom {
                endpoint_management_api,
                ..
            } => endpoint_management_api,
        }
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
    /// assert_eq!(mgmt_token_url, "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token");
    /// ```
    pub fn endpoint_management_api_token(&self) -> &str {
        match self {
            Self::Prod => {
                "https://auth.prod.cp.kpn-dsh.com/auth/realms/tt-dsh/protocol/openid-connect/token"
            }
            Self::NpLz => {
                "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token"
            }
            Self::ProdLz => {
                "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/prod-lz-dsh/protocol/openid-connect/token"
            }
            Self::ProdAz => {
                "https://auth.prod.cp.kpn-dsh.com/auth/realms/prod-azure-dsh/protocol/openid-connect/token"
            }
            Self::Poc => {
                "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token"
            }
            Self::Custom {
                endpoint_management_api_token,
                ..
            } => endpoint_management_api_token,
        }
    }

    /// Returns the endpoint for fetching DSH protocol [Data Access Tokens](crate::protocol_adapters::token::data_access_token::DataAccessToken) (e.g., for MQTT).
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::Prod;
    /// let protocol_token_url = platform.endpoint_protocol_access_token();
    /// assert_eq!(protocol_token_url, "https://api.kpn-dsh.com/datastreams/v0/mqtt/token");
    /// ```
    pub fn endpoint_protocol_access_token(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/datastreams/v0/mqtt/token",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/datastreams/v0/mqtt/token",
            Self::ProdAz => "https://api.az.kpn-dsh.com/datastreams/v0/mqtt/token",
            Self::Poc => "https://api.poc.kpn-dsh.com/datastreams/v0/mqtt/token",
            Self::Custom {
                endpoint_protocol_access_token,
                ..
            } => endpoint_protocol_access_token,
        }
    }

    /// Returns the URL endpoint for retrieving DSH REST API OAuth tokens to fetch [Data Access Tokens](crate::protocol_adapters::token::data_access_token::DataAccessToken).
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::Platform;
    /// let platform = Platform::NpLz;
    /// let token_url = platform.endpoint_protocol_rest_token();
    /// assert_eq!(token_url, "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token");
    /// ```
    pub fn endpoint_protocol_rest_token(&self) -> &str {
        match self {
            Self::Prod => "https://api.kpn-dsh.com/auth/v0/token",
            Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token",
            Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/auth/v0/token",
            Self::ProdAz => "https://api.az.kpn-dsh.com/auth/v0/token",
            Self::Poc => "https://api.poc.kpn-dsh.com/auth/v0/token",
            Self::Custom {
                endpoint_protocol_rest_token,
                ..
            } => endpoint_protocol_rest_token,
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
    /// assert_eq!(realm, "prod-dsh");
    /// ```
    pub fn realm(&self) -> &str {
        match self {
            Self::Prod => "prod-dsh",
            Self::NpLz => "dev-lz-dsh",
            Self::ProdLz => "prod-lz-dsh",
            Self::ProdAz => "prod-azure-dsh",
            Self::Poc => "poc-dsh",
            Self::Custom { realm, .. } => realm,
        }
    }

    /// Creates a [`Platform`] instance based on the `DSH_ENVIRONMENT` environment variable.
    ///
    /// In you DSH Service Configuration, you can set the `DSH_ENVIRONMENT` variable like this
    /// ```json
    /// {
    /// ...
    ///   "env": {
    ///     "DSH_ENVIRONMENT": "{ variables('DSH_ENVIRONMENT')}"
    ///   },
    /// ...
    ///}
    ///```
    ///
    /// # Custom Platform
    /// If you want to use a custom platform, you can set the `DSH_ENVIRONMENT` to `custom` which
    /// whill try to instantiate a [`Platform::Custom`]. Set the following environment variables to set the endpoints and realm:
    ///
    /// | Variable Name | Description | Required |
    /// | ------------- | ----------- | :------: |
    /// | `DSH_ENVIRONMENT` | Set to `custom` | `Yes` |
    /// | `DSH_REALM` | The realm name for the to be used platform | `Yes` |
    /// | `DSH_ENDPOINT_MANAGEMENT_API` | The endpoint for the DSH Management API | `No` |
    /// | `DSH_ENDPOINT_MANAGEMENT_API_TOKEN` | The endpoint for fetching a DSH Management API authentication token | `No` |
    /// | `DSH_ENDPOINT_PROTOCOL_ACCESS_TOKEN` | The endpoint for fetching DSH protocol [Access Tokens](crate::protocol_adapters::token::data_access_token::DataAccessToken) | `No` |
    /// | `DSH_ENDPOINT_PROTOCOL_REST_TOKEN` | The endpoint for retrieving Protocol [Rest Tokens](crate::protocol_adapters::token::rest_token::RestToken) which is needed to request [Access Tokens](crate::protocol_adapters::token::data_access_token::DataAccessToken) | `No` |
    ///
    /// The endpoint variables are optional, if not set, the related token fetchers will not work.
    pub fn from_env() -> Result<Self, UtilsError> {
        let platform_env = get_env_var(VAR_DSH_ENVIRONMENT)?;
        if platform_env.to_lowercase() == "custom" {
            Self::custom_from_env()
        } else {
            Self::try_from(platform_env.as_str())
                .map_err(|_| UtilsError::InvalidPlatform(platform_env))
        }
    }

    fn custom_from_env() -> Result<Self, UtilsError> {
        let realm = get_env_var(VAR_DSH_REALM)?;
        let endpoint_management_api =
            get_env_var(VAR_DSH_ENDPOINT_MANAGEMENT_API).unwrap_or_default();
        let endpoint_management_api_token =
            get_env_var(VAR_DSH_ENDPOINT_MANAGEMENT_API_TOKEN).unwrap_or_default();
        let endpoint_protocol_access_token =
            get_env_var(VAR_DSH_ENDPOINT_PROTOCOL_ACCESS_TOKEN).unwrap_or_default();
        let endpoint_protocol_rest_token =
            get_env_var(VAR_DSH_ENDPOINT_PROTOCOL_REST_TOKEN).unwrap_or_default();
        Ok(Self::Custom {
            realm,
            endpoint_management_api,
            endpoint_management_api_token,
            endpoint_protocol_access_token,
            endpoint_protocol_rest_token,
        })
    }
}

impl TryFrom<&str> for Platform {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "prod-dsh" | "tt-dsh" | "prod" => Ok(Self::Prod),
            "prod-azure-dsh" | "prodaz" | "prod-az" => Ok(Self::ProdAz),
            "prod-lz-dsh" | "prodlz" | "prod-lz" => Ok(Self::ProdLz),
            "dev-lz-dsh" | "nplz" | "np-lz" => Ok(Self::NpLz),
            "poc-dsh" | "poc" => Ok(Self::Poc),
            _ => Err("Invalid platform"),
        }
    }
}

impl TryFrom<String> for Platform {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_platform_realm() {
        assert_eq!(Platform::NpLz.realm(), "dev-lz-dsh");
        assert_eq!(Platform::ProdLz.realm(), "prod-lz-dsh");
        assert_eq!(Platform::Poc.realm(), "poc-dsh");
    }

    #[test]
    fn test_platform_client_id() {
        assert_eq!(
            Platform::NpLz.management_api_client_id("my-tenant"),
            "robot:dev-lz-dsh:my-tenant"
        );
        assert_eq!(
            Platform::ProdLz.management_api_client_id("my-tenant".to_string()),
            "robot:prod-lz-dsh:my-tenant"
        );
        assert_eq!(
            Platform::Poc.management_api_client_id("my-tenant"),
            "robot:poc-dsh:my-tenant"
        );
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(Platform::try_from("prod").unwrap(), Platform::Prod);
        assert_eq!(Platform::try_from("PROD").unwrap(), Platform::Prod);
        assert_eq!(Platform::try_from("prod-az").unwrap(), Platform::ProdAz);
        assert_eq!(Platform::try_from("PROD-AZ").unwrap(), Platform::ProdAz);
        assert_eq!(Platform::try_from("prodaz").unwrap(), Platform::ProdAz);
        assert_eq!(Platform::try_from("PRODAZ").unwrap(), Platform::ProdAz);
        assert_eq!(Platform::try_from("prod-lz").unwrap(), Platform::ProdLz);
        assert_eq!(Platform::try_from("PROD-LZ").unwrap(), Platform::ProdLz);
        assert_eq!(Platform::try_from("prodlz").unwrap(), Platform::ProdLz);
        assert_eq!(Platform::try_from("PRODLZ").unwrap(), Platform::ProdLz);
        assert_eq!(Platform::try_from("np-lz").unwrap(), Platform::NpLz);
        assert_eq!(Platform::try_from("NP-LZ").unwrap(), Platform::NpLz);
        assert_eq!(Platform::try_from("nplz").unwrap(), Platform::NpLz);
        assert_eq!(Platform::try_from("NPLZ").unwrap(), Platform::NpLz);
        assert_eq!(Platform::try_from("poc").unwrap(), Platform::Poc);
        assert_eq!(Platform::try_from("POC").unwrap(), Platform::Poc);
        assert!(Platform::try_from("invalid").is_err());
    }

    #[test]
    fn test_try_from_string() {
        assert_eq!(
            Platform::try_from("prod".to_string()).unwrap(),
            Platform::Prod
        );
        assert_eq!(
            Platform::try_from("PROD".to_string()).unwrap(),
            Platform::Prod
        );
        assert_eq!(
            Platform::try_from("prod-az".to_string()).unwrap(),
            Platform::ProdAz
        );
        assert_eq!(
            Platform::try_from("PROD-AZ".to_string()).unwrap(),
            Platform::ProdAz
        );
        assert_eq!(
            Platform::try_from("prodaz".to_string()).unwrap(),
            Platform::ProdAz
        );
        assert_eq!(
            Platform::try_from("PRODAZ".to_string()).unwrap(),
            Platform::ProdAz
        );
        assert_eq!(
            Platform::try_from("prod-lz".to_string()).unwrap(),
            Platform::ProdLz
        );
        assert_eq!(
            Platform::try_from("PROD-LZ".to_string()).unwrap(),
            Platform::ProdLz
        );
        assert_eq!(
            Platform::try_from("prodlz".to_string()).unwrap(),
            Platform::ProdLz
        );
        assert_eq!(
            Platform::try_from("PRODLZ".to_string()).unwrap(),
            Platform::ProdLz
        );
        assert_eq!(
            Platform::try_from("np-lz".to_string()).unwrap(),
            Platform::NpLz
        );
        assert_eq!(
            Platform::try_from("NP-LZ".to_string()).unwrap(),
            Platform::NpLz
        );
        assert_eq!(
            Platform::try_from("nplz".to_string()).unwrap(),
            Platform::NpLz
        );
        assert_eq!(
            Platform::try_from("NPLZ".to_string()).unwrap(),
            Platform::NpLz
        );
        assert_eq!(
            Platform::try_from("poc".to_string()).unwrap(),
            Platform::Poc
        );
        assert_eq!(
            Platform::try_from("POC".to_string()).unwrap(),
            Platform::Poc
        );
        assert!(Platform::try_from("invalid".to_string()).is_err());
    }

    #[test]
    #[serial(env_dependency)]
    fn test_platform_from_env() {
        unsafe {
            std::env::set_var(VAR_DSH_ENVIRONMENT, "prod");
            let platform = Platform::from_env().unwrap();
            assert_eq!(platform, Platform::Prod);
            std::env::set_var(VAR_DSH_REALM, "this-should-not-be-used");
            let platform = Platform::from_env().unwrap();
            assert_eq!(platform.realm(), "prod-dsh");
            std::env::remove_var(VAR_DSH_ENVIRONMENT);
            std::env::remove_var(VAR_DSH_REALM);
        }
    }

    #[test]
    #[serial(env_dependency)]
    fn test_platform_from_env_custom() {
        unsafe {
            std::env::set_var(VAR_DSH_ENVIRONMENT, "custom");
            assert!(Platform::from_env().is_err());
            std::env::set_var(VAR_DSH_REALM, "custom-realm");
            assert!(Platform::from_env().is_ok());
            std::env::set_var(
                VAR_DSH_ENDPOINT_MANAGEMENT_API,
                "https://custom.api.endpoint",
            );
            std::env::set_var(
                VAR_DSH_ENDPOINT_MANAGEMENT_API_TOKEN,
                "https://custom.token.endpoint",
            );
            std::env::set_var(
                VAR_DSH_ENDPOINT_PROTOCOL_ACCESS_TOKEN,
                "https://custom.access.token.endpoint",
            );
            std::env::set_var(
                VAR_DSH_ENDPOINT_PROTOCOL_REST_TOKEN,
                "https://custom.rest.token.endpoint",
            );

            let platform = Platform::from_env().unwrap();
            assert_eq!(platform.realm(), "custom-realm");
            assert_eq!(
                platform.endpoint_management_api(),
                "https://custom.api.endpoint"
            );
            assert_eq!(
                platform.endpoint_management_api_token(),
                "https://custom.token.endpoint"
            );
            assert_eq!(
                platform.endpoint_protocol_access_token(),
                "https://custom.access.token.endpoint"
            );
            assert_eq!(
                platform.endpoint_protocol_rest_token(),
                "https://custom.rest.token.endpoint"
            );

            // Clean up environment variables
            std::env::remove_var(VAR_DSH_ENVIRONMENT);
            std::env::remove_var(VAR_DSH_REALM);
            std::env::remove_var(VAR_DSH_ENDPOINT_MANAGEMENT_API);
            std::env::remove_var(VAR_DSH_ENDPOINT_MANAGEMENT_API_TOKEN);
            std::env::remove_var(VAR_DSH_ENDPOINT_PROTOCOL_ACCESS_TOKEN);
            std::env::remove_var(VAR_DSH_ENDPOINT_PROTOCOL_REST_TOKEN);
        }
    }
}
