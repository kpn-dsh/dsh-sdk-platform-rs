# \DataCatalogApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_datacatalog_asset_kind_get**](DataCatalogApi.md#allocation_tenant_datacatalog_asset_kind_get) | **GET** /allocation/{tenant}/datacatalog/asset/{kind} | lists all data catalog assets of a tenant for the given kind
[**allocation_tenant_datacatalog_asset_kind_name_configuration_delete**](DataCatalogApi.md#allocation_tenant_datacatalog_asset_kind_name_configuration_delete) | **DELETE** /allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration | deletes a dataCatalogAsset
[**allocation_tenant_datacatalog_asset_kind_name_configuration_get**](DataCatalogApi.md#allocation_tenant_datacatalog_asset_kind_name_configuration_get) | **GET** /allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration | gets configuration of a data catalog asset allocation
[**allocation_tenant_datacatalog_asset_kind_name_configuration_put**](DataCatalogApi.md#allocation_tenant_datacatalog_asset_kind_name_configuration_put) | **PUT** /allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration | creates dataCatalogAsset configuration.
[**allocation_tenant_datacatalog_asset_kind_name_get**](DataCatalogApi.md#allocation_tenant_datacatalog_asset_kind_name_get) | **GET** /allocation/{tenant}/datacatalog/asset/{kind}/{name} | shows overall status of a datacatalog asset allocation



## allocation_tenant_datacatalog_asset_kind_get

> Vec<String> allocation_tenant_datacatalog_asset_kind_get(tenant, kind)
lists all data catalog assets of a tenant for the given kind

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**kind** | **String** | data catalog asset kind | [required] |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_datacatalog_asset_kind_name_configuration_delete

> allocation_tenant_datacatalog_asset_kind_name_configuration_delete(tenant, kind, name)
deletes a dataCatalogAsset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**kind** | **String** | data catalog asset kind | [required] |
**name** | **String** | data catalog asset name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_datacatalog_asset_kind_name_configuration_get

> models::DataCatalogAsset allocation_tenant_datacatalog_asset_kind_name_configuration_get(tenant, kind, name)
gets configuration of a data catalog asset allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**kind** | **String** | data catalog asset kind | [required] |
**name** | **String** | data catalog asset name | [required] |

### Return type

[**models::DataCatalogAsset**](DataCatalogAsset.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_datacatalog_asset_kind_name_configuration_put

> allocation_tenant_datacatalog_asset_kind_name_configuration_put(tenant, kind, name, data_catalog_asset)
creates dataCatalogAsset configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**kind** | **String** | data catalog asset kind | [required] |
**name** | **String** | data catalog asset name | [required] |
**data_catalog_asset** | [**DataCatalogAsset**](DataCatalogAsset.md) | the JSON representation of the resource | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_datacatalog_asset_kind_name_get

> models::DataCatalogAssetStatus allocation_tenant_datacatalog_asset_kind_name_get(tenant, kind, name)
shows overall status of a datacatalog asset allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**kind** | **String** | data catalog asset kind | [required] |
**name** | **String** | data catalog asset name | [required] |

### Return type

[**models::DataCatalogAssetStatus**](DataCatalogAssetStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

