//! Utility functions for the SDK

use super::{VAR_APP_ID, VAR_DSH_TENANT_NAME};
use crate::error::DshError;
use log::{debug, warn};
use std::env;

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
                    VAR_APP_ID,
                    app_id
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
    #[serial(env_depencency)]
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
    #[serial(env_depencency)]
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
