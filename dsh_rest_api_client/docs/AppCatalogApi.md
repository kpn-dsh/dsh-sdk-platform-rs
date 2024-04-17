# \AppCatalogApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_appcatalogapp_actual_get**](AppCatalogApi.md#allocation_tenant_appcatalogapp_actual_get) | **GET** /allocation/{tenant}/appcatalogapp/actual | returns a list containing all App Catalog App allocations and their respective configurations of a given tenant, as they are actually deployed
[**allocation_tenant_appcatalogapp_appcatalogappid_actual_get**](AppCatalogApi.md#allocation_tenant_appcatalogapp_appcatalogappid_actual_get) | **GET** /allocation/{tenant}/appcatalogapp/{appcatalogappid}/actual | returns the configuration of an App Catalog App allocation as it is actually deployed. To only view the configuration parameters of this allocation, see the `appcatalogappconfiguration` section. 
[**allocation_tenant_appcatalogapp_appcatalogappid_configuration_get**](AppCatalogApi.md#allocation_tenant_appcatalogapp_appcatalogappid_configuration_get) | **GET** /allocation/{tenant}/appcatalogapp/{appcatalogappid}/configuration | returns the configuration of an App Catalog App allocation by a specified tenant name and App Catalog App Id. To only view the configuration parameters of this allocation, see the `appcatalogappconfiguration` section. 
[**allocation_tenant_appcatalogapp_configuration_get**](AppCatalogApi.md#allocation_tenant_appcatalogapp_configuration_get) | **GET** /allocation/{tenant}/appcatalogapp/configuration | returns a list containing all App Catalog App allocations and their respective configurations of a given tenant



## allocation_tenant_appcatalogapp_actual_get

> std::collections::HashMap<String, models::AppCatalogApp> allocation_tenant_appcatalogapp_actual_get(tenant)
returns a list containing all App Catalog App allocations and their respective configurations of a given tenant, as they are actually deployed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**std::collections::HashMap<String, models::AppCatalogApp>**](AppCatalogApp.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_appcatalogapp_appcatalogappid_actual_get

> models::AppCatalogApp allocation_tenant_appcatalogapp_appcatalogappid_actual_get(tenant, appcatalogappid)
returns the configuration of an App Catalog App allocation as it is actually deployed. To only view the configuration parameters of this allocation, see the `appcatalogappconfiguration` section. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appcatalogappid** | **String** | appcatalogapp name | [required] |

### Return type

[**models::AppCatalogApp**](AppCatalogApp.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_appcatalogapp_appcatalogappid_configuration_get

> models::AppCatalogApp allocation_tenant_appcatalogapp_appcatalogappid_configuration_get(tenant, appcatalogappid)
returns the configuration of an App Catalog App allocation by a specified tenant name and App Catalog App Id. To only view the configuration parameters of this allocation, see the `appcatalogappconfiguration` section. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appcatalogappid** | **String** | appcatalogapp name | [required] |

### Return type

[**models::AppCatalogApp**](AppCatalogApp.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_appcatalogapp_configuration_get

> std::collections::HashMap<String, models::AppCatalogApp> allocation_tenant_appcatalogapp_configuration_get(tenant)
returns a list containing all App Catalog App allocations and their respective configurations of a given tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**std::collections::HashMap<String, models::AppCatalogApp>**](AppCatalogApp.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

