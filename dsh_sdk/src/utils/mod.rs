//! Utilities for DSH
//!
//! This module contains helpful functions and utilities for interacting with DSH.
use std::env;

use log::{debug, info, warn};

use super::{VAR_APP_ID, VAR_DSH_TENANT_NAME};

#[doc(inline)]
pub use error::UtilsError;

#[cfg(feature = "dlq")]
pub mod dlq;
#[cfg(feature = "graceful-shutdown")]
pub mod graceful_shutdown;
// #[cfg(feature = "hyper-client")] // TODO: to be implemented
// pub(crate) mod http_client;
#[cfg(feature = "metrics")]
pub mod metrics;

mod platform;

mod error;

#[doc(inline)]
pub use platform::Platform;

/// Get the configured topics from the environment variable TOPICS
/// Topics can be delimited by a comma
///
/// ## Example
/// ```
/// # use dsh_sdk::utils::get_configured_topics;
/// std::env::set_var("TOPICS", "topic1, topic2, topic3");
///
/// let topics = get_configured_topics().unwrap();
///
/// assert_eq!(topics[0], "topic1");
/// assert_eq!(topics[1], "topic2");
/// assert_eq!(topics[2], "topic3");
/// # std::env::remove_var("TOPICS");
/// ```
pub fn get_configured_topics() -> Result<Vec<String>, UtilsError> {
    let kafka_topic_string = get_env_var("TOPICS")?;
    Ok(kafka_topic_string
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect())
}

/// Get the tenant name from the environment variables
///
/// Derive the tenant name from the `MARATHON_APP_ID` or `DSH_TENANT_NAME` environment variables.
/// Returns `NoTenantName` error if neither of the environment variables are set.
///
/// ## Example
/// ```
/// # use dsh_sdk::utils::tenant_name;
/// # use dsh_sdk::utils::UtilsError;
/// std::env::set_var("MARATHON_APP_ID", "/dsh-tenant-name/app-name"); // Injected by DSH by default
///
/// let tenant = tenant_name().unwrap();
/// assert_eq!(&tenant, "dsh-tenant-name");
/// # std::env::remove_var("MARATHON_APP_ID");
///
/// std::env::set_var("DSH_TENANT_NAME", "your-tenant-name"); // Set by user, useful when running outside of DSH together with Kafka Proxy or VPN
/// let tenant = tenant_name().unwrap();
/// assert_eq!(&tenant, "your-tenant-name");
/// # std::env::remove_var("DSH_TENANT_NAME");
///
/// // If neither of the environment variables are set, it will return an error
/// let result = tenant_name();
/// assert!(matches!(result, Err(UtilsError::NoTenantName)));
/// ```

pub fn tenant_name() -> Result<String, UtilsError> {
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
        log::warn!("{} and {} are not set, this may cause unexpected behaviour when connecting to DSH Kafka cluster as the group ID is based on this!. Please set one of these environment variables.", VAR_DSH_TENANT_NAME, VAR_APP_ID);
        Err(UtilsError::NoTenantName)
    }
}

/// Get an environment variable or return an error if not set,
///
/// Returns the value of the environment variable if it is set, otherwise returns
/// `VarError` error.
pub(crate) fn get_env_var(var_name: &'static str) -> Result<String, UtilsError> {
    debug!("Reading {} from environment variable", var_name);
    match env::var(var_name) {
        Ok(value) => Ok(value),
        Err(e) => {
            info!("{} is not set", var_name);
            Err(UtilsError::EnvVarError(var_name, e))
        }
    }
}

/// Helper function to ensure that the host starts with `https://` or `http://`.
pub(crate) fn ensure_https_prefix(host: impl AsRef<str>) -> String {
    if host.as_ref().starts_with("http://") || host.as_ref().starts_with("https://") {
        host.as_ref().to_string()
    } else {
        format!("https://{}", host.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial(env_dependency)]
    fn test_dsh_config_tenant_name() {
        let result = tenant_name();
        assert!(matches!(result, Err(UtilsError::NoTenantName)));
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

    #[test]
    fn test_ensure_https_prefix() {
        let host = "http://example.com";
        let result = ensure_https_prefix(host);
        assert_eq!(result, "http://example.com");

        let host = "https://example.com";
        let result = ensure_https_prefix(host);
        assert_eq!(result, "https://example.com");

        let host = "example.com";
        let result = ensure_https_prefix(host);
        assert_eq!(result, "https://example.com");
    }
}
