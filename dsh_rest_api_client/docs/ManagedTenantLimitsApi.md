# \ManagedTenantLimitsApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**manage_manager_tenant_tenant_limit_get**](ManagedTenantLimitsApi.md#manage_manager_tenant_tenant_limit_get) | **GET** /manage/{manager}/tenant/{tenant}/limit | get all limits of a managed tenant
[**manage_manager_tenant_tenant_limit_kind_get**](ManagedTenantLimitsApi.md#manage_manager_tenant_tenant_limit_kind_get) | **GET** /manage/{manager}/tenant/{tenant}/limit/{kind} | get a specific managed tenant limit set by the managing tenant
[**manage_manager_tenant_tenant_limit_kind_put**](ManagedTenantLimitsApi.md#manage_manager_tenant_tenant_limit_kind_put) | **PUT** /manage/{manager}/tenant/{tenant}/limit/{kind} | create and/or update the configured limits for a managed tenant
[**manage_manager_tenant_tenant_limit_patch**](ManagedTenantLimitsApi.md#manage_manager_tenant_tenant_limit_patch) | **PATCH** /manage/{manager}/tenant/{tenant}/limit | update multiple limits of a managed tenant



## manage_manager_tenant_tenant_limit_get

> Vec<models::LimitValue> manage_manager_tenant_tenant_limit_get(manager, tenant)
get all limits of a managed tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |

### Return type

[**Vec<models::LimitValue>**](LimitValue.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_limit_kind_get

> models::LimitValue manage_manager_tenant_tenant_limit_kind_get(manager, tenant, kind)
get a specific managed tenant limit set by the managing tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |
**kind** | **String** | Limit request type | [required] |

### Return type

[**models::LimitValue**](LimitValue.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_limit_kind_put

> manage_manager_tenant_tenant_limit_kind_put(manager, tenant, kind, limit_value)
create and/or update the configured limits for a managed tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |
**kind** | **String** | Limit request type | [required] |
**limit_value** | [**LimitValue**](LimitValue.md) | the JSON object containing the limit configuration of the managed tenant | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_limit_patch

> manage_manager_tenant_tenant_limit_patch(manager, tenant, limit_value)
update multiple limits of a managed tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |
**limit_value** | [**Vec<models::LimitValue>**](LimitValue.md) | a JSON list with multiple limits of the managed tenant | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

