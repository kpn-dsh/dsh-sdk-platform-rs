# \TopicApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_topic_get**](TopicApi.md#allocation_tenant_topic_get) | **GET** /allocation/{tenant}/topic | returns a list of topics of a tenant
[**allocation_tenant_topic_id_actual_get**](TopicApi.md#allocation_tenant_topic_id_actual_get) | **GET** /allocation/{tenant}/topic/{id}/actual | returns actual configuration of a topic allocation
[**allocation_tenant_topic_id_configuration_delete**](TopicApi.md#allocation_tenant_topic_id_configuration_delete) | **DELETE** /allocation/{tenant}/topic/{id}/configuration | deletes a topic
[**allocation_tenant_topic_id_configuration_get**](TopicApi.md#allocation_tenant_topic_id_configuration_get) | **GET** /allocation/{tenant}/topic/{id}/configuration | returns the configuration of a topic allocation
[**allocation_tenant_topic_id_configuration_put**](TopicApi.md#allocation_tenant_topic_id_configuration_put) | **PUT** /allocation/{tenant}/topic/{id}/configuration | create a new topic. It is impossible to update an existing topic. This requires a delete of the existing topic and creation of a new one with the wanted configuration.
[**allocation_tenant_topic_id_get**](TopicApi.md#allocation_tenant_topic_id_get) | **GET** /allocation/{tenant}/topic/{id} | returns the overall status of a topic allocation
[**allocation_tenant_topic_id_status_get**](TopicApi.md#allocation_tenant_topic_id_status_get) | **GET** /allocation/{tenant}/topic/{id}/status | returns a brief status description of a topic allocation



## allocation_tenant_topic_get

> Vec<String> allocation_tenant_topic_get(tenant)
returns a list of topics of a tenant

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


## allocation_tenant_topic_id_actual_get

> models::Topic allocation_tenant_topic_id_actual_get(tenant, id)
returns actual configuration of a topic allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | topic name | [required] |

### Return type

[**models::Topic**](Topic.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_topic_id_configuration_delete

> allocation_tenant_topic_id_configuration_delete(tenant, id)
deletes a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | topic name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_topic_id_configuration_get

> models::Topic allocation_tenant_topic_id_configuration_get(tenant, id)
returns the configuration of a topic allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | topic name | [required] |

### Return type

[**models::Topic**](Topic.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_topic_id_configuration_put

> allocation_tenant_topic_id_configuration_put(tenant, id, topic)
create a new topic. It is impossible to update an existing topic. This requires a delete of the existing topic and creation of a new one with the wanted configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | topic name | [required] |
**topic** | [**Topic**](Topic.md) | the JSON object containing the configuration of the desired topic | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_topic_id_get

> models::TopicStatus allocation_tenant_topic_id_get(tenant, id)
returns the overall status of a topic allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | topic name | [required] |

### Return type

[**models::TopicStatus**](TopicStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_topic_id_status_get

> models::AllocationStatus allocation_tenant_topic_id_status_get(tenant, id)
returns a brief status description of a topic allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | topic name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

