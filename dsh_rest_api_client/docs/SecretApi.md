# \SecretApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_secret_get**](SecretApi.md#allocation_tenant_secret_get) | **GET** /allocation/{tenant}/secret | returns a list of all secret names of a tenant
[**allocation_tenant_secret_id_actual_get**](SecretApi.md#allocation_tenant_secret_id_actual_get) | **GET** /allocation/{tenant}/secret/{id}/actual | returns the actual state of a secret. The response body will always be empty because we cannot share the secret value, but the response code will tell you more about its state.
[**allocation_tenant_secret_id_configuration_delete**](SecretApi.md#allocation_tenant_secret_id_configuration_delete) | **DELETE** /allocation/{tenant}/secret/{id}/configuration | deletes a secret
[**allocation_tenant_secret_id_configuration_get**](SecretApi.md#allocation_tenant_secret_id_configuration_get) | **GET** /allocation/{tenant}/secret/{id}/configuration | returns the configuration of a secret allocation
[**allocation_tenant_secret_id_get**](SecretApi.md#allocation_tenant_secret_id_get) | **GET** /allocation/{tenant}/secret/{id} | returns the value of a secret
[**allocation_tenant_secret_id_put**](SecretApi.md#allocation_tenant_secret_id_put) | **PUT** /allocation/{tenant}/secret/{id} | update the value of a secret
[**allocation_tenant_secret_id_status_get**](SecretApi.md#allocation_tenant_secret_id_status_get) | **GET** /allocation/{tenant}/secret/{id}/status | returns a brief status description of a secret allocation
[**allocation_tenant_secret_post**](SecretApi.md#allocation_tenant_secret_post) | **POST** /allocation/{tenant}/secret | create a new secret



## allocation_tenant_secret_get

> Vec<String> allocation_tenant_secret_get(tenant)
returns a list of all secret names of a tenant

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


## allocation_tenant_secret_id_actual_get

> serde_json::Value allocation_tenant_secret_id_actual_get(tenant, id)
returns the actual state of a secret. The response body will always be empty because we cannot share the secret value, but the response code will tell you more about its state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | secret name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_secret_id_configuration_delete

> allocation_tenant_secret_id_configuration_delete(tenant, id)
deletes a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | secret name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_secret_id_configuration_get

> serde_json::Value allocation_tenant_secret_id_configuration_get(tenant, id)
returns the configuration of a secret allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | secret name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_secret_id_get

> String allocation_tenant_secret_id_get(tenant, id)
returns the value of a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | secret name | [required] |

### Return type

**String**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_secret_id_put

> allocation_tenant_secret_id_put(tenant, id, body)
update the value of a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | secret name | [required] |
**body** | **String** | the secret value as a string | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_secret_id_status_get

> models::AllocationStatus allocation_tenant_secret_id_status_get(tenant, id)
returns a brief status description of a secret allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | secret name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_secret_post

> allocation_tenant_secret_post(tenant, secret)
create a new secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**secret** | [**Secret**](Secret.md) | a JSON object containing the name and the secret value | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

