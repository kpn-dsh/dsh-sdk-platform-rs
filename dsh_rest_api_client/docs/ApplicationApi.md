# \ApplicationApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_application_actual_get**](ApplicationApi.md#allocation_tenant_application_actual_get) | **GET** /allocation/{tenant}/application/actual | returns a list containing the configuration of every deployed application of a given tenant
[**allocation_tenant_application_appid_actual_get**](ApplicationApi.md#allocation_tenant_application_appid_actual_get) | **GET** /allocation/{tenant}/application/{appid}/actual | returns the configuration of a deployed application allocation for a given app id and tenant
[**allocation_tenant_application_appid_configuration_delete**](ApplicationApi.md#allocation_tenant_application_appid_configuration_delete) | **DELETE** /allocation/{tenant}/application/{appid}/configuration | deletes an application by a specified application id
[**allocation_tenant_application_appid_configuration_get**](ApplicationApi.md#allocation_tenant_application_appid_configuration_get) | **GET** /allocation/{tenant}/application/{appid}/configuration | Returns the configuration of a certain application, specified by the tenant name and application name.
[**allocation_tenant_application_appid_configuration_put**](ApplicationApi.md#allocation_tenant_application_appid_configuration_put) | **PUT** /allocation/{tenant}/application/{appid}/configuration | creates an application allocation, or update it's configuration
[**allocation_tenant_application_appid_status_get**](ApplicationApi.md#allocation_tenant_application_appid_status_get) | **GET** /allocation/{tenant}/application/{appid}/status | returns a status description of an application allocation
[**allocation_tenant_application_configuration_get**](ApplicationApi.md#allocation_tenant_application_configuration_get) | **GET** /allocation/{tenant}/application/configuration | Returns the configuration of every application created by a given tenant.
[**allocation_tenant_task_appid_get**](ApplicationApi.md#allocation_tenant_task_appid_get) | **GET** /allocation/{tenant}/task/{appid} | return a list containing the ids of an application's derived tasks
[**allocation_tenant_task_appid_id_actual_get**](ApplicationApi.md#allocation_tenant_task_appid_id_actual_get) | **GET** /allocation/{tenant}/task/{appid}/{id}/actual | returns the actual state of a specific task
[**allocation_tenant_task_appid_id_get**](ApplicationApi.md#allocation_tenant_task_appid_id_get) | **GET** /allocation/{tenant}/task/{appid}/{id} | returns overall status of a task
[**allocation_tenant_task_appid_id_status_get**](ApplicationApi.md#allocation_tenant_task_appid_id_status_get) | **GET** /allocation/{tenant}/task/{appid}/{id}/status | returns a brief status description of a task
[**allocation_tenant_task_get**](ApplicationApi.md#allocation_tenant_task_get) | **GET** /allocation/{tenant}/task | return a list containing the ids of all applications with derived tasks



## allocation_tenant_application_actual_get

> std::collections::HashMap<String, models::Application> allocation_tenant_application_actual_get(tenant)
returns a list containing the configuration of every deployed application of a given tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**std::collections::HashMap<String, models::Application>**](Application.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_application_appid_actual_get

> models::Application allocation_tenant_application_appid_actual_get(tenant, appid)
returns the configuration of a deployed application allocation for a given app id and tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_application_appid_configuration_delete

> allocation_tenant_application_appid_configuration_delete(tenant, appid)
deletes an application by a specified application id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_application_appid_configuration_get

> models::Application allocation_tenant_application_appid_configuration_get(tenant, appid)
Returns the configuration of a certain application, specified by the tenant name and application name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_application_appid_configuration_put

> allocation_tenant_application_appid_configuration_put(tenant, appid, application)
creates an application allocation, or update it's configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |
**application** | [**Application**](Application.md) | a JSON containing the configuration of the application you want to deploy | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_application_appid_status_get

> models::AllocationStatus allocation_tenant_application_appid_status_get(tenant, appid)
returns a status description of an application allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_application_configuration_get

> std::collections::HashMap<String, models::Application> allocation_tenant_application_configuration_get(tenant)
Returns the configuration of every application created by a given tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**std::collections::HashMap<String, models::Application>**](Application.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_task_appid_get

> Vec<String> allocation_tenant_task_appid_get(tenant, appid)
return a list containing the ids of an application's derived tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_task_appid_id_actual_get

> models::Task allocation_tenant_task_appid_id_actual_get(tenant, appid, id)
returns the actual state of a specific task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |
**id** | **String** | task name | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_task_appid_id_get

> models::TaskStatus allocation_tenant_task_appid_id_get(tenant, appid, id)
returns overall status of a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |
**id** | **String** | task name | [required] |

### Return type

[**models::TaskStatus**](TaskStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_task_appid_id_status_get

> models::AllocationStatus allocation_tenant_task_appid_id_status_get(tenant, appid, id)
returns a brief status description of a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**appid** | **String** | application name | [required] |
**id** | **String** | task name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_task_get

> Vec<String> allocation_tenant_task_get(tenant)
return a list containing the ids of all applications with derived tasks

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

