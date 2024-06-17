/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`allocation_tenant_datacatalog_asset_kind_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantDatacatalogAssetKindGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_datacatalog_asset_kind_name_configuration_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantDatacatalogAssetKindNameConfigurationDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_datacatalog_asset_kind_name_configuration_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantDatacatalogAssetKindNameConfigurationGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_datacatalog_asset_kind_name_configuration_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantDatacatalogAssetKindNameConfigurationPutError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_datacatalog_asset_kind_name_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantDatacatalogAssetKindNameGetError {
    UnknownValue(serde_json::Value),
}

pub async fn allocation_tenant_datacatalog_asset_kind_get(
    configuration: &configuration::Configuration,
    tenant: &str,
    kind: &str,
) -> Result<Vec<String>, Error<AllocationTenantDatacatalogAssetKindGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/allocation/{tenant}/datacatalog/asset/{kind}",
        local_var_configuration.base_path,
        tenant = crate::apis::urlencode(tenant),
        kind = crate::apis::urlencode(kind)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AllocationTenantDatacatalogAssetKindGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_datacatalog_asset_kind_name_configuration_delete(
    configuration: &configuration::Configuration,
    tenant: &str,
    kind: &str,
    name: &str,
) -> Result<(), Error<AllocationTenantDatacatalogAssetKindNameConfigurationDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration",
        local_var_configuration.base_path,
        tenant = crate::apis::urlencode(tenant),
        kind = crate::apis::urlencode(kind),
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<
            AllocationTenantDatacatalogAssetKindNameConfigurationDeleteError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_datacatalog_asset_kind_name_configuration_get(
    configuration: &configuration::Configuration,
    tenant: &str,
    kind: &str,
    name: &str,
) -> Result<
    models::DataCatalogAsset,
    Error<AllocationTenantDatacatalogAssetKindNameConfigurationGetError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration",
        local_var_configuration.base_path,
        tenant = crate::apis::urlencode(tenant),
        kind = crate::apis::urlencode(kind),
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<
            AllocationTenantDatacatalogAssetKindNameConfigurationGetError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_datacatalog_asset_kind_name_configuration_put(
    configuration: &configuration::Configuration,
    tenant: &str,
    kind: &str,
    name: &str,
    data_catalog_asset: models::DataCatalogAsset,
) -> Result<(), Error<AllocationTenantDatacatalogAssetKindNameConfigurationPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration",
        local_var_configuration.base_path,
        tenant = crate::apis::urlencode(tenant),
        kind = crate::apis::urlencode(kind),
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&data_catalog_asset);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<
            AllocationTenantDatacatalogAssetKindNameConfigurationPutError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_datacatalog_asset_kind_name_get(
    configuration: &configuration::Configuration,
    tenant: &str,
    kind: &str,
    name: &str,
) -> Result<models::DataCatalogAssetStatus, Error<AllocationTenantDatacatalogAssetKindNameGetError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/allocation/{tenant}/datacatalog/asset/{kind}/{name}",
        local_var_configuration.base_path,
        tenant = crate::apis::urlencode(tenant),
        kind = crate::apis::urlencode(kind),
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AllocationTenantDatacatalogAssetKindNameGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
