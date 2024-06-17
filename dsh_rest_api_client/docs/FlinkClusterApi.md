# \FlinkClusterApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_flinkcluster_actual_get**](FlinkClusterApi.md#allocation_tenant_flinkcluster_actual_get) | **GET** /allocation/{tenant}/flinkcluster/actual | returns the actual configuration of a Flink Cluster.
[**allocation_tenant_flinkcluster_configuration_delete**](FlinkClusterApi.md#allocation_tenant_flinkcluster_configuration_delete) | **DELETE** /allocation/{tenant}/flinkcluster/configuration | deletes a Flink Cluster. Since only one cluster can be created per tenant, only the tenants' name needs to be specified.
[**allocation_tenant_flinkcluster_configuration_get**](FlinkClusterApi.md#allocation_tenant_flinkcluster_configuration_get) | **GET** /allocation/{tenant}/flinkcluster/configuration | returns the configuration of a Flink Cluster
[**allocation_tenant_flinkcluster_configuration_put**](FlinkClusterApi.md#allocation_tenant_flinkcluster_configuration_put) | **PUT** /allocation/{tenant}/flinkcluster/configuration | create a new Flink Cluster. It is impossible to update an existing Flink Cluster. This requires a delete of the existing Flink Cluster and creation of a new one with the wanted configuration.
[**allocation_tenant_flinkcluster_get**](FlinkClusterApi.md#allocation_tenant_flinkcluster_get) | **GET** /allocation/{tenant}/flinkcluster | returns the overall status of a Flink Cluster
[**allocation_tenant_flinkcluster_status_get**](FlinkClusterApi.md#allocation_tenant_flinkcluster_status_get) | **GET** /allocation/{tenant}/flinkcluster/status | returns a brief status description of a Flink Cluster



## allocation_tenant_flinkcluster_actual_get

> models::FlinkCluster allocation_tenant_flinkcluster_actual_get(tenant)
returns the actual configuration of a Flink Cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::FlinkCluster**](FlinkCluster.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_flinkcluster_configuration_delete

> allocation_tenant_flinkcluster_configuration_delete(tenant)
deletes a Flink Cluster. Since only one cluster can be created per tenant, only the tenants' name needs to be specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_flinkcluster_configuration_get

> models::FlinkCluster allocation_tenant_flinkcluster_configuration_get(tenant)
returns the configuration of a Flink Cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::FlinkCluster**](FlinkCluster.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_flinkcluster_configuration_put

> allocation_tenant_flinkcluster_configuration_put(tenant, flink_cluster)
create a new Flink Cluster. It is impossible to update an existing Flink Cluster. This requires a delete of the existing Flink Cluster and creation of a new one with the wanted configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**flink_cluster** | [**FlinkCluster**](FlinkCluster.md) | a JSON object containing the desired configuration of the Flink Cluster. Zone must be known to the platform. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_flinkcluster_get

> models::FlinkClusterStatus allocation_tenant_flinkcluster_get(tenant)
returns the overall status of a Flink Cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::FlinkClusterStatus**](FlinkClusterStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_flinkcluster_status_get

> models::AllocationStatus allocation_tenant_flinkcluster_status_get(tenant)
returns a brief status description of a Flink Cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

