# \BucketaccessApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_bucket_id_bucketaccess_get**](BucketaccessApi.md#allocation_tenant_bucket_id_bucketaccess_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketaccess | shows bucketaccesses about a specific bucket
[**allocation_tenant_bucket_id_bucketaccess_name_actual_get**](BucketaccessApi.md#allocation_tenant_bucket_id_bucketaccess_name_actual_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketaccess/{name}/actual | gets actual configuration of a bucketaccess allocation
[**allocation_tenant_bucket_id_bucketaccess_name_configuration_delete**](BucketaccessApi.md#allocation_tenant_bucket_id_bucketaccess_name_configuration_delete) | **DELETE** /allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration | deletes a bucketaccess
[**allocation_tenant_bucket_id_bucketaccess_name_configuration_get**](BucketaccessApi.md#allocation_tenant_bucket_id_bucketaccess_name_configuration_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration | gets configuration of a bucketaccess allocation
[**allocation_tenant_bucket_id_bucketaccess_name_configuration_put**](BucketaccessApi.md#allocation_tenant_bucket_id_bucketaccess_name_configuration_put) | **PUT** /allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration | creates bucketaccess configuration.
[**allocation_tenant_bucket_id_bucketaccess_name_get**](BucketaccessApi.md#allocation_tenant_bucket_id_bucketaccess_name_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketaccess/{name} | shows overall status of a third party bucket concession
[**allocation_tenant_bucket_id_bucketaccess_name_status_get**](BucketaccessApi.md#allocation_tenant_bucket_id_bucketaccess_name_status_get) | **GET** /allocation/{tenant}/bucket/{id}/bucketaccess/{name}/status | gets status description of a bucketaccess allocation
[**allocation_tenant_bucketaccess_get**](BucketaccessApi.md#allocation_tenant_bucketaccess_get) | **GET** /allocation/{tenant}/bucketaccess | lists all bucketaccesses of a tenant



## allocation_tenant_bucket_id_bucketaccess_get

> Vec<String> allocation_tenant_bucket_id_bucketaccess_get(tenant, id)
shows bucketaccesses about a specific bucket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketaccess_name_actual_get

> models::BucketAccess allocation_tenant_bucket_id_bucketaccess_name_actual_get(tenant, id, name)
gets actual configuration of a bucketaccess allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |
**name** | **String** | bucket access name | [required] |

### Return type

[**models::BucketAccess**](BucketAccess.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketaccess_name_configuration_delete

> allocation_tenant_bucket_id_bucketaccess_name_configuration_delete(tenant, id, name)
deletes a bucketaccess

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |
**name** | **String** | bucket access name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketaccess_name_configuration_get

> models::BucketAccessConfiguration allocation_tenant_bucket_id_bucketaccess_name_configuration_get(tenant, id, name)
gets configuration of a bucketaccess allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |
**name** | **String** | bucket access name | [required] |

### Return type

[**models::BucketAccessConfiguration**](BucketAccessConfiguration.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketaccess_name_configuration_put

> allocation_tenant_bucket_id_bucketaccess_name_configuration_put(tenant, id, name, bucket_access_configuration)
creates bucketaccess configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |
**name** | **String** | bucket access name | [required] |
**bucket_access_configuration** | [**BucketAccessConfiguration**](BucketAccessConfiguration.md) | the wanted config of the (new) bucketaccess allocation | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketaccess_name_get

> models::BucketAccessStatus allocation_tenant_bucket_id_bucketaccess_name_get(tenant, id, name)
shows overall status of a third party bucket concession

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |
**name** | **String** | bucket access name | [required] |

### Return type

[**models::BucketAccessStatus**](BucketAccessStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucket_id_bucketaccess_name_status_get

> models::AllocationStatus allocation_tenant_bucket_id_bucketaccess_name_status_get(tenant, id, name)
gets status description of a bucketaccess allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | bucket name | [required] |
**name** | **String** | bucket access name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_bucketaccess_get

> Vec<String> allocation_tenant_bucketaccess_get(tenant)
lists all bucketaccesses of a tenant

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

