# \ManagedTenantApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**manage_manager_tenant_get**](ManagedTenantApi.md#manage_manager_tenant_get) | **GET** /manage/{manager}/tenant | returns a list of tenants managed by the `manager` tenant
[**manage_manager_tenant_tenant_actual_get**](ManagedTenantApi.md#manage_manager_tenant_tenant_actual_get) | **GET** /manage/{manager}/tenant/{tenant}/actual | returns the actual state for a managed tenant allocation
[**manage_manager_tenant_tenant_configuration_delete**](ManagedTenantApi.md#manage_manager_tenant_tenant_configuration_delete) | **DELETE** /manage/{manager}/tenant/{tenant}/configuration | deletes a managed tenant for the managing tenant
[**manage_manager_tenant_tenant_configuration_get**](ManagedTenantApi.md#manage_manager_tenant_tenant_configuration_get) | **GET** /manage/{manager}/tenant/{tenant}/configuration | returns the configuration of tenant as managed by the manager
[**manage_manager_tenant_tenant_configuration_put**](ManagedTenantApi.md#manage_manager_tenant_tenant_configuration_put) | **PUT** /manage/{manager}/tenant/{tenant}/configuration | creates and/or updates a managed tenant for managing tenant or update its configuration
[**manage_manager_tenant_tenant_status_get**](ManagedTenantApi.md#manage_manager_tenant_tenant_status_get) | **GET** /manage/{manager}/tenant/{tenant}/status | returns a brief status description of a managed tenant allocation



## manage_manager_tenant_get

> Vec<String> manage_manager_tenant_get(manager)
returns a list of tenants managed by the `manager` tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_actual_get

> models::ManagedTenant manage_manager_tenant_tenant_actual_get(manager, tenant)
returns the actual state for a managed tenant allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::ManagedTenant**](ManagedTenant.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_configuration_delete

> manage_manager_tenant_tenant_configuration_delete(manager, tenant)
deletes a managed tenant for the managing tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_configuration_get

> models::ManagedTenant manage_manager_tenant_tenant_configuration_get(manager, tenant)
returns the configuration of tenant as managed by the manager

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::ManagedTenant**](ManagedTenant.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_configuration_put

> manage_manager_tenant_tenant_configuration_put(manager, tenant, managed_tenant)
creates and/or updates a managed tenant for managing tenant or update its configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |
**managed_tenant** | [**ManagedTenant**](ManagedTenant.md) | the JSON object containing the configuration of the managed tenant | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_manager_tenant_tenant_status_get

> models::AllocationStatus manage_manager_tenant_tenant_status_get(manager, tenant)
returns a brief status description of a managed tenant allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager** | **String** | Name of the tenant that is acting as manager for this request | [required] |
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

