# ManagedTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the tenant.  Must be identical to the tenant name used in the path. | 
**manager** | **String** | Name of the tenant that is acting as manager for this tenant.   Must be identical to the `manager` parameter in the path.  | 
**services** | Option<[**Vec<models::ManagedTenantServices>**](ManagedTenant_services.md)> | List of services that are enabled for this tenant.  At this point, `monitoring` is a requirement (it's  `enabled` value must be `true`).  The default values for `tracing` and `vpn` are both `false`.  The `vpn` service is only available on some platforms.  Requesting it on a platform that doesn't support it will  cause the request to be rejected.  | [optional][default to [{"name":"monitoring","enabled":true},{"name":"vpn","enabled":false},{"name":"tracing","enabled":false}]]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


