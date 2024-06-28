//! Utility functions for the SDK

use super::{VAR_APP_ID, VAR_DSH_TENANT_NAME};
use crate::error::DshError;
use log::{debug, warn};
use std::env;

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
        T: AsRef<str> + std::fmt::Display,
    {
        format!("robot:{}:{}", self.realm(), tenant)
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
    /// assert_eq!(endpoint, "https://auth.lz.lz-cp.dsh.np.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token");
    /// ```
    pub fn endpoint_rest_access_token(&self) -> &str {
        match self {
            Self::Prod =>   "https://auth.prod.cp.kpn-dsh.com/auth/realms/tt-dsh/protocol/openid-connect/token",
            Self::NpLz =>   "https://auth.lz.lz-cp.dsh.np.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token",
            Self::ProdLz => "https://auth.lz.lz-cp.dsh.np.aws.kpn.com/auth/realms/prod-lz-dsh/protocol/openid-connect/token",
            Self::ProdAz => "https://auth.prod.cp.kpn-dsh.com/auth/realms/prod-azure-dsh/protocol/openid-connect/token",
            Self::Poc =>    "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token", 
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

/// Get the configured topics from the environment variable TOPICS
/// Topics can be delimited by a comma
pub fn get_configured_topics() -> Result<Vec<String>, DshError> {
    let kafka_topic_string = env::var("TOPICS")?;
    Ok(kafka_topic_string
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect())
}

/// Get the tenant name from the environment variables
///
/// Derive the tenant name from the MARATHON_APP_ID or DSH_TENANT_NAME environment variables.
/// Returns `NoTenantName` error if neither of the environment variables are set.
pub(crate) fn tenant_name() -> Result<String, DshError> {
    if let Ok(app_id) = get_env_var(VAR_APP_ID) {
        let tenant_name = app_id.split('/').nth(1);
        match tenant_name {
            Some(tenant_name) => Ok(tenant_name.to_string()),
            None => {
                warn!(
                    "{} did not parse succesfully, using \"{}\" as tenant name",
                    VAR_APP_ID, app_id
                );
                Ok(app_id)
            }
        }
    } else if let Ok(tenant_name) = get_env_var(VAR_DSH_TENANT_NAME) {
        Ok(tenant_name)
    } else {
        Err(DshError::NoTenantName)
    }
}

/// Get an environment variable or return an error if not set,
///
/// Returns the value of the environment variable if it is set, otherwise returns
/// `VarError` error.
pub(crate) fn get_env_var(var_name: &str) -> Result<String, DshError> {
    debug!("Reading {} from environment variable", var_name);
    match env::var(var_name) {
        Ok(value) => Ok(value),
        Err(e) => {
            warn!("{} is not set", var_name);
            Err(e.into())
        }
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

    #[test]
    #[serial(env_dependency)]
    fn test_dsh_config_tenant_name() {
        let result = tenant_name();
        assert!(matches!(result, Err(DshError::NoTenantName)));
        env::set_var(VAR_APP_ID, "/parsed-tenant-name/app-name");
        let result = tenant_name().unwrap();
        assert_eq!(result, "parsed-tenant-name".to_string());
        env::set_var(VAR_APP_ID, "incorrect_app_id");
        let result = tenant_name().unwrap();
        assert_eq!(result, "incorrect_app_id".to_string(),);
        env::remove_var(VAR_APP_ID);
        env::set_var(VAR_DSH_TENANT_NAME, "tenant_name");
        let result = tenant_name().unwrap();
        assert_eq!(result, "tenant_name".to_string());
        env::remove_var(VAR_DSH_TENANT_NAME);
    }

    #[test]
    #[serial(env_dependency)]
    fn test_get_configured_topics() {
        std::env::set_var("TOPICS", "topic1, topic2, topic3");

        let topics = get_configured_topics().unwrap();
        assert_eq!(topics.len(), 3);
        assert_eq!(topics[0], "topic1");
        assert_eq!(topics[1], "topic2");
        assert_eq!(topics[2], "topic3");

        std::env::remove_var("TOPICS");

        let topics = get_configured_topics();
        assert!(topics.is_err());
    }

    #[test]
    fn test_get_env_var() {
        env::set_var("TEST_ENV_VAR", "test_value");
        let result = get_env_var("TEST_ENV_VAR").unwrap();
        assert_eq!(result, "test_value");
    }
}
