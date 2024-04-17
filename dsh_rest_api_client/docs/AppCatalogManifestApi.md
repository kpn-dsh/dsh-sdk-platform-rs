# \AppCatalogManifestApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**appcatalog_tenant_manifest_get**](AppCatalogManifestApi.md#appcatalog_tenant_manifest_get) | **GET** /appcatalog/{tenant}/manifest | returns a list of AppCatalog manifests for a given tenant



## appcatalog_tenant_manifest_get

> Vec<models::AppCatalogManifest> appcatalog_tenant_manifest_get(tenant)
returns a list of AppCatalog manifests for a given tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**Vec<models::AppCatalogManifest>**](AppCatalogManifest.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

