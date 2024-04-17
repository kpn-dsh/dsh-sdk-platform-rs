# \AppCatalogAppConfigurationApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_delete**](AppCatalogAppConfigurationApi.md#appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_delete) | **DELETE** /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration | deletes an App Catalog App
[**appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_get**](AppCatalogAppConfigurationApi.md#appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_get) | **GET** /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration | Returns the wanted configuration of an App Catalog App by its tenant name and AppCatalogApp Id. If an App Catalog App is stuck while deploying and not on actual, it will show up here.
[**appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_put**](AppCatalogAppConfigurationApi.md#appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_put) | **PUT** /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration | creates a new App Catalog App, or update its configuration
[**appcatalog_tenant_appcatalogapp_appcatalogappid_status_get**](AppCatalogAppConfigurationApi.md#appcatalog_tenant_appcatalogapp_appcatalogappid_status_get) | **GET** /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/status | gets status description of an App Catalog App



## appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_delete

> appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_delete(tenant, appcatalogappid)
deletes an App Catalog App

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appcatalogappid** | **String** | appcatalogapp name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_get

> models::AppCatalogAppConfiguration appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_get(tenant, appcatalogappid)
Returns the wanted configuration of an App Catalog App by its tenant name and AppCatalogApp Id. If an App Catalog App is stuck while deploying and not on actual, it will show up here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appcatalogappid** | **String** | appcatalogapp name | [required] |

### Return type

[**models::AppCatalogAppConfiguration**](AppCatalogAppConfiguration.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_put

> appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_put(tenant, appcatalogappid, app_catalog_app_configuration)
creates a new App Catalog App, or update its configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appcatalogappid** | **String** | appcatalogapp name | [required] |
**app_catalog_app_configuration** | [**AppCatalogAppConfiguration**](AppCatalogAppConfiguration.md) | JSON object containing required parameters for AppCatalogApp manifest. This is comparable to the configuration object on a regular Application service. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appcatalog_tenant_appcatalogapp_appcatalogappid_status_get

> models::AllocationStatus appcatalog_tenant_appcatalogapp_appcatalogappid_status_get(tenant, appcatalogappid)
gets status description of an App Catalog App

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appcatalogappid** | **String** | appcatalogapp name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

