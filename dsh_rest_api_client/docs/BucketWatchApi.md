# \BucketWatchApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_bucket_id_bucketwatch_actual_get**](BucketWatchApi.md#allocation_tenant_bucket_id_bucketwatch_actual_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketwatch/actual | gets actual configuration of a bucketwatch allocation
[**allocation_tenant_bucket_id_bucketwatch_configuration_delete**](BucketWatchApi.md#allocation_tenant_bucket_id_bucketwatch_configuration_delete) | **DELETE** /allocation/{tenant}/bucket/{id}/bucketwatch/configuration | deletes a bucketwatch
[**allocation_tenant_bucket_id_bucketwatch_configuration_get**](BucketWatchApi.md#allocation_tenant_bucket_id_bucketwatch_configuration_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketwatch/configuration | gets configuration of a bucketwatch allocation
[**allocation_tenant_bucket_id_bucketwatch_configuration_put**](BucketWatchApi.md#allocation_tenant_bucket_id_bucketwatch_configuration_put) | **PUT** /allocation/{tenant}/bucket/{id}/bucketwatch/configuration | creates bucketwatch configuration.
[**allocation_tenant_bucket_id_bucketwatch_get**](BucketWatchApi.md#allocation_tenant_bucket_id_bucketwatch_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketwatch | shows overall status of a bucketwatch allocation
[**allocation_tenant_bucket_id_bucketwatch_status_get**](BucketWatchApi.md#allocation_tenant_bucket_id_bucketwatch_status_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketwatch/status | gets status description of a bucketwatch allocation
[**allocation_tenant_bucketwatch_get**](BucketWatchApi.md#allocation_tenant_bucketwatch_get) | **GET** /allocation/{tenant}/bucketwatch | lists all bucketwatches of a tenant



## allocation_tenant_bucket_id_bucketwatch_actual_get

> models::BucketWatch allocation_tenant_bucket_id_bucketwatch_actual_get(tenant, id)
gets actual configuration of a bucketwatch allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

[**models::BucketWatch**](BucketWatch.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketwatch_configuration_delete

> allocation_tenant_bucket_id_bucketwatch_configuration_delete(tenant, id)
deletes a bucketwatch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketwatch_configuration_get

> models::BucketWatch allocation_tenant_bucket_id_bucketwatch_configuration_get(tenant, id)
gets configuration of a bucketwatch allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

[**models::BucketWatch**](BucketWatch.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketwatch_configuration_put

> allocation_tenant_bucket_id_bucketwatch_configuration_put(tenant, id)
creates bucketwatch configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketwatch_get

> models::BucketWatchStatus allocation_tenant_bucket_id_bucketwatch_get(tenant, id)
shows overall status of a bucketwatch allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

[**models::BucketWatchStatus**](BucketWatchStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketwatch_status_get

> models::AllocationStatus allocation_tenant_bucket_id_bucketwatch_status_get(tenant, id)
gets status description of a bucketwatch allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucketwatch_get

> Vec<String> allocation_tenant_bucketwatch_get(tenant)
lists all bucketwatches of a tenant

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

