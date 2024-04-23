/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.6.6
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// ApplicationSecret : a secret to be injected as an environment variable in the application
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSecret {
    /// the secret's name
    #[serde(rename = "name")]
    pub name: String,
    /// a list of environment variable names. The secret's value may be injected multiple times as different environment variables, so multiple environment variable names for the same secret can be provided
    #[serde(rename = "injections")]
    pub injections: Vec<std::collections::HashMap<String, String>>,
}

impl ApplicationSecret {
    /// a secret to be injected as an environment variable in the application
    pub fn new(
        name: String,
        injections: Vec<std::collections::HashMap<String, String>>,
    ) -> ApplicationSecret {
        ApplicationSecret { name, injections }
    }
}
