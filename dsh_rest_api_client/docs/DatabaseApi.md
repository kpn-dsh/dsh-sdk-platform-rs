# \DatabaseApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_database_get**](DatabaseApi.md#allocation_tenant_database_get) | **GET** /allocation/{tenant}/database | (beta release) lists ids of all databases of a tenant
[**allocation_tenant_database_id_actual_get**](DatabaseApi.md#allocation_tenant_database_id_actual_get) | **GET** /allocation/{tenant}/database/{id}/actual | (beta release) gets actual state for a database allocation
[**allocation_tenant_database_id_configuration_delete**](DatabaseApi.md#allocation_tenant_database_id_configuration_delete) | **DELETE** /allocation/{tenant}/database/{id}/configuration | (beta release) deletes a database
[**allocation_tenant_database_id_configuration_get**](DatabaseApi.md#allocation_tenant_database_id_configuration_get) | **GET** /allocation/{tenant}/database/{id}/configuration | (beta release) gets configuration for a database allocation
[**allocation_tenant_database_id_configuration_put**](DatabaseApi.md#allocation_tenant_database_id_configuration_put) | **PUT** /allocation/{tenant}/database/{id}/configuration | (beta release) creates a database configuration. It is impossible to update an existing database.
[**allocation_tenant_database_id_get**](DatabaseApi.md#allocation_tenant_database_id_get) | **GET** /allocation/{tenant}/database/{id} | (beta release) gets overall status of a database allocation
[**allocation_tenant_database_id_status_get**](DatabaseApi.md#allocation_tenant_database_id_status_get) | **GET** /allocation/{tenant}/database/{id}/status | (beta release) gets status description of a database allocation



## allocation_tenant_database_get

> Vec<String> allocation_tenant_database_get(tenant)
(beta release) lists ids of all databases of a tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_database_id_actual_get

> models::Database allocation_tenant_database_id_actual_get(tenant, id)
(beta release) gets actual state for a database allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | database name | [required] |

### Return type

[**models::Database**](Database.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_database_id_configuration_delete

> allocation_tenant_database_id_configuration_delete(tenant, id)
(beta release) deletes a database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | database name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_database_id_configuration_get

> models::Database allocation_tenant_database_id_configuration_get(tenant, id)
(beta release) gets configuration for a database allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | database name | [required] |

### Return type

[**models::Database**](Database.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_database_id_configuration_put

> allocation_tenant_database_id_configuration_put(tenant, id, database)
(beta release) creates a database configuration. It is impossible to update an existing database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | database name | [required] |
**database** | [**Database**](Database.md) | the JSON representation of the resource | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_database_id_get

> models::DatabaseStatus allocation_tenant_database_id_get(tenant, id)
(beta release) gets overall status of a database allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | database name | [required] |

### Return type

[**models::DatabaseStatus**](DatabaseStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_database_id_status_get

> models::AllocationStatus allocation_tenant_database_id_status_get(tenant, id)
(beta release) gets status description of a database allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | database name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

