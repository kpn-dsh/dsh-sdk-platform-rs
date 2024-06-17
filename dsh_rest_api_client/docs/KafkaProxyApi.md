# \KafkaProxyApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_kafkaproxy_get**](KafkaProxyApi.md#allocation_tenant_kafkaproxy_get) | **GET** /allocation/{tenant}/kafkaproxy | returns a list of all kafka proxies of a tenant
[**allocation_tenant_kafkaproxy_id_configuration_delete**](KafkaProxyApi.md#allocation_tenant_kafkaproxy_id_configuration_delete) | **DELETE** /allocation/{tenant}/kafkaproxy/{id}/configuration | deletes a kafka proxy
[**allocation_tenant_kafkaproxy_id_configuration_get**](KafkaProxyApi.md#allocation_tenant_kafkaproxy_id_configuration_get) | **GET** /allocation/{tenant}/kafkaproxy/{id}/configuration | Returns the configuration of a certain kafka Proxy, specified by the tenant name and kafka Proxy name.
[**allocation_tenant_kafkaproxy_id_configuration_put**](KafkaProxyApi.md#allocation_tenant_kafkaproxy_id_configuration_put) | **PUT** /allocation/{tenant}/kafkaproxy/{id}/configuration | update the value of the kafka proxy



## allocation_tenant_kafkaproxy_get

> Vec<String> allocation_tenant_kafkaproxy_get(tenant)
returns a list of all kafka proxies of a tenant

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


## allocation_tenant_kafkaproxy_id_configuration_delete

> allocation_tenant_kafkaproxy_id_configuration_delete(tenant, id)
deletes a kafka proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | Kafka proxy id | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_kafkaproxy_id_configuration_get

> models::KafkaProxy allocation_tenant_kafkaproxy_id_configuration_get(tenant, id)
Returns the configuration of a certain kafka Proxy, specified by the tenant name and kafka Proxy name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | Kafka proxy id | [required] |

### Return type

[**models::KafkaProxy**](KafkaProxy.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_kafkaproxy_id_configuration_put

> allocation_tenant_kafkaproxy_id_configuration_put(tenant, id, kafka_proxy)
update the value of the kafka proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | Kafka proxy id | [required] |
**kafka_proxy** | [**KafkaProxy**](KafkaProxy.md) | the kafka proxy configuration options | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

