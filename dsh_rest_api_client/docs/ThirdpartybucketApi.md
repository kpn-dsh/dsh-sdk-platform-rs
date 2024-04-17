# \ThirdpartybucketApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_bucket_fromthirdparty_get**](ThirdpartybucketApi.md#allocation_tenant_bucket_fromthirdparty_get) | **GET** /allocation/{tenant}/bucket/fromthirdparty | lists only bucket names of a tenant that originated from a third party bucket concession
[**allocation_tenant_thirdpartybucketconcession_get**](ThirdpartybucketApi.md#allocation_tenant_thirdpartybucketconcession_get) | **GET** /allocation/{tenant}/thirdpartybucketconcession | list summaries of third party bucket concessions, registered using credentials shared to you by a third party
[**allocation_tenant_thirdpartybucketconcession_id_actual_get**](ThirdpartybucketApi.md#allocation_tenant_thirdpartybucketconcession_id_actual_get) | **GET** /allocation/{tenant}/thirdpartybucketconcession/{id}/actual | gets actual configuration of a third party bucket concession (received bucket access) allocation
[**allocation_tenant_thirdpartybucketconcession_id_configuration_delete**](ThirdpartybucketApi.md#allocation_tenant_thirdpartybucketconcession_id_configuration_delete) | **DELETE** /allocation/{tenant}/thirdpartybucketconcession/{id}/configuration | unregisters a third party bucket concession. This will also remove the virtual bucket.
[**allocation_tenant_thirdpartybucketconcession_id_configuration_get**](ThirdpartybucketApi.md#allocation_tenant_thirdpartybucketconcession_id_configuration_get) | **GET** /allocation/{tenant}/thirdpartybucketconcession/{id}/configuration | gets configuration of a third party bucket concession (received bucket access) allocation
[**allocation_tenant_thirdpartybucketconcession_id_get**](ThirdpartybucketApi.md#allocation_tenant_thirdpartybucketconcession_id_get) | **GET** /allocation/{tenant}/thirdpartybucketconcession/{id} | shows overall status of a third party bucket concession
[**allocation_tenant_thirdpartybucketconcession_id_status_get**](ThirdpartybucketApi.md#allocation_tenant_thirdpartybucketconcession_id_status_get) | **GET** /allocation/{tenant}/thirdpartybucketconcession/{id}/status | gets status description of third party bucket concession (received bucket access) allocation
[**allocation_tenant_thirdpartybucketconcession_post**](ThirdpartybucketApi.md#allocation_tenant_thirdpartybucketconcession_post) | **POST** /allocation/{tenant}/thirdpartybucketconcession | register a new bucket concession for which credentials were shared to you by a third party



## allocation_tenant_bucket_fromthirdparty_get

> Vec<String> allocation_tenant_bucket_fromthirdparty_get(tenant)
lists only bucket names of a tenant that originated from a third party bucket concession

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


## allocation_tenant_thirdpartybucketconcession_get

> Vec<String> allocation_tenant_thirdpartybucketconcession_get(tenant)
list summaries of third party bucket concessions, registered using credentials shared to you by a third party

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


## allocation_tenant_thirdpartybucketconcession_id_actual_get

> models::ThirdPartyBucketConcession allocation_tenant_thirdpartybucketconcession_id_actual_get(tenant, id)
gets actual configuration of a third party bucket concession (received bucket access) allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | your name of choice for the third party bucket | [required] |

### Return type

[**models::ThirdPartyBucketConcession**](ThirdPartyBucketConcession.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_thirdpartybucketconcession_id_configuration_delete

> allocation_tenant_thirdpartybucketconcession_id_configuration_delete(tenant, id)
unregisters a third party bucket concession. This will also remove the virtual bucket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | your name of choice for the third party bucket | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_thirdpartybucketconcession_id_configuration_get

> models::ThirdPartyBucketConcession allocation_tenant_thirdpartybucketconcession_id_configuration_get(tenant, id)
gets configuration of a third party bucket concession (received bucket access) allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | your name of choice for the third party bucket | [required] |

### Return type

[**models::ThirdPartyBucketConcession**](ThirdPartyBucketConcession.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_thirdpartybucketconcession_id_get

> models::ThirdPartyBucketConcessionStatus allocation_tenant_thirdpartybucketconcession_id_get(tenant, id)
shows overall status of a third party bucket concession

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | your name of choice for the third party bucket | [required] |

### Return type

[**models::ThirdPartyBucketConcessionStatus**](ThirdPartyBucketConcessionStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_thirdpartybucketconcession_id_status_get

> models::AllocationStatus allocation_tenant_thirdpartybucketconcession_id_status_get(tenant, id)
gets status description of third party bucket concession (received bucket access) allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | your name of choice for the third party bucket | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_thirdpartybucketconcession_post

> allocation_tenant_thirdpartybucketconcession_post(tenant, third_party_bucket_concession_registration)
register a new bucket concession for which credentials were shared to you by a third party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**third_party_bucket_concession_registration** | [**ThirdPartyBucketConcessionRegistration**](ThirdPartyBucketConcessionRegistration.md) | the secret value | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

