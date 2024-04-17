# \RobotApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**robot_tenant_generate_secret_get**](RobotApi.md#robot_tenant_generate_secret_get) | **GET** /robot/{tenant}/generate-secret | generate new client secret for a tenant



## robot_tenant_generate_secret_get

> models::ClientSecret robot_tenant_generate_secret_get(tenant)
generate new client secret for a tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |

### Return type

[**models::ClientSecret**](ClientSecret.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

