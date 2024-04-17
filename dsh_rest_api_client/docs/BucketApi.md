# \BucketApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_bucket_get**](BucketApi.md#allocation_tenant_bucket_get) | **GET** /allocation/{tenant}/bucket | lists all bucket names of a tenant
[**allocation_tenant_bucket_id_actual_get**](BucketApi.md#allocation_tenant_bucket_id_actual_get) | **GET** /allocation/{tenant}/bucket/{id}/actual | gets actual configuration of a bucket allocation
[**allocation_tenant_bucket_id_configuration_delete**](BucketApi.md#allocation_tenant_bucket_id_configuration_delete) | **DELETE** /allocation/{tenant}/bucket/{id}/configuration | deletes a bucket
[**allocation_tenant_bucket_id_configuration_get**](BucketApi.md#allocation_tenant_bucket_id_configuration_get) | **GET** /allocation/{tenant}/bucket/{id}/configuration | gets configuration of a bucket allocation
[**allocation_tenant_bucket_id_configuration_put**](BucketApi.md#allocation_tenant_bucket_id_configuration_put) | **PUT** /allocation/{tenant}/bucket/{id}/configuration | creates bucket configuration.It is impossible to update an existing bucket. This requires a delete of the existing bucket and creation of a new one with the wanted configuration.
[**allocation_tenant_bucket_id_get**](BucketApi.md#allocation_tenant_bucket_id_get) | **GET** /allocation/{tenant}/bucket/{id} | shows overall status of a bucket allocation
[**allocation_tenant_bucket_id_status_get**](BucketApi.md#allocation_tenant_bucket_id_status_get) | **GET** /allocation/{tenant}/bucket/{id}/status | gets status description of a bucket allocation



## allocation_tenant_bucket_get

> Vec<String> allocation_tenant_bucket_get(tenant)
lists all bucket names of a tenant

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


## allocation_tenant_bucket_id_actual_get

> models::Bucket allocation_tenant_bucket_id_actual_get(tenant, id)
gets actual configuration of a bucket allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_configuration_delete

> allocation_tenant_bucket_id_configuration_delete(tenant, id)
deletes a bucket

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


## allocation_tenant_bucket_id_configuration_get

> models::Bucket allocation_tenant_bucket_id_configuration_get(tenant, id)
gets configuration of a bucket allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_configuration_put

> allocation_tenant_bucket_id_configuration_put(tenant, id, bucket)
creates bucket configuration.It is impossible to update an existing bucket. This requires a delete of the existing bucket and creation of a new one with the wanted configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |
**bucket** | [**Bucket**](Bucket.md) | the JSON representation of the resource | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_get

> models::BucketStatus allocation_tenant_bucket_id_get(tenant, id)
shows overall status of a bucket allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

[**models::BucketStatus**](BucketStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_status_get

> models::AllocationStatus allocation_tenant_bucket_id_status_get(tenant, id)
gets status description of a bucket allocation

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

