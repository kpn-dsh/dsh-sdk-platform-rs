/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedTenant {
    /// Name of the tenant.  Must be identical to the tenant name used in the path.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of the tenant that is acting as manager for this tenant.   Must be identical to the `manager` parameter in the path.
    #[serde(rename = "manager")]
    pub manager: String,
    /// List of services that are enabled for this tenant.  At this point, `monitoring` is a requirement (it's  `enabled` value must be `true`).  The default values for `tracing` and `vpn` are both `false`.  The `vpn` service is only available on some platforms.  Requesting it on a platform that doesn't support it will  cause the request to be rejected.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<models::ManagedTenantServices>>,
}

impl ManagedTenant {
    pub fn new(name: String, manager: String) -> ManagedTenant {
        ManagedTenant {
            name,
            manager,
            services: None,
        }
    }
}
