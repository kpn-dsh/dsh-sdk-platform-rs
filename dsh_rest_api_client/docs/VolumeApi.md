# \VolumeApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_volume_get**](VolumeApi.md#allocation_tenant_volume_get) | **GET** /allocation/{tenant}/volume | returns a list containing the ids of all volumes of a given tenant
[**allocation_tenant_volume_id_actual_get**](VolumeApi.md#allocation_tenant_volume_id_actual_get) | **GET** /allocation/{tenant}/volume/{id}/actual | returns the actual state for a volume allocation
[**allocation_tenant_volume_id_configuration_delete**](VolumeApi.md#allocation_tenant_volume_id_configuration_delete) | **DELETE** /allocation/{tenant}/volume/{id}/configuration | deletes a volume
[**allocation_tenant_volume_id_configuration_get**](VolumeApi.md#allocation_tenant_volume_id_configuration_get) | **GET** /allocation/{tenant}/volume/{id}/configuration | returns the configuration for a volume allocation
[**allocation_tenant_volume_id_configuration_put**](VolumeApi.md#allocation_tenant_volume_id_configuration_put) | **PUT** /allocation/{tenant}/volume/{id}/configuration | create a new volume configuration. It is impossible to update an existing volume. This requires a delete of the existing volume and creation of a new one with the wanted configuration.
[**allocation_tenant_volume_id_get**](VolumeApi.md#allocation_tenant_volume_id_get) | **GET** /allocation/{tenant}/volume/{id} | returns the overall status of a volume allocation
[**allocation_tenant_volume_id_status_get**](VolumeApi.md#allocation_tenant_volume_id_status_get) | **GET** /allocation/{tenant}/volume/{id}/status | returns a brief status description of a volume allocation



## allocation_tenant_volume_get

> Vec<String> allocation_tenant_volume_get(tenant)
returns a list containing the ids of all volumes of a given tenant

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


## allocation_tenant_volume_id_actual_get

> models::Volume allocation_tenant_volume_id_actual_get(tenant, id)
returns the actual state for a volume allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | volume name | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_volume_id_configuration_delete

> allocation_tenant_volume_id_configuration_delete(tenant, id)
deletes a volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | volume name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_volume_id_configuration_get

> models::Volume allocation_tenant_volume_id_configuration_get(tenant, id)
returns the configuration for a volume allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | volume name | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_volume_id_configuration_put

> allocation_tenant_volume_id_configuration_put(tenant, id, volume)
create a new volume configuration. It is impossible to update an existing volume. This requires a delete of the existing volume and creation of a new one with the wanted configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | volume name | [required] |
**volume** | [**Volume**](Volume.md) | the JSON object containing the desired configuration of a volume allocation | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_volume_id_get

> models::VolumeStatus allocation_tenant_volume_id_get(tenant, id)
returns the overall status of a volume allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | volume name | [required] |

### Return type

[**models::VolumeStatus**](VolumeStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_volume_id_status_get

> models::AllocationStatus allocation_tenant_volume_id_status_get(tenant, id)
returns a brief status description of a volume allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | volume name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

