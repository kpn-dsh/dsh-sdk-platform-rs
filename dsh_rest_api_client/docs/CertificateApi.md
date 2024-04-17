# \CertificateApi

All URIs are relative to */resources/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocation_tenant_certificate_get**](CertificateApi.md#allocation_tenant_certificate_get) | **GET** /allocation/{tenant}/certificate | returns a list of all certificate names that are allocated to a tenant
[**allocation_tenant_certificate_id_actual_get**](CertificateApi.md#allocation_tenant_certificate_id_actual_get) | **GET** /allocation/{tenant}/certificate/{id}/actual | returns the actual configuration of a certificate allocation. This may not represent the wanted configuration.
[**allocation_tenant_certificate_id_configuration_delete**](CertificateApi.md#allocation_tenant_certificate_id_configuration_delete) | **DELETE** /allocation/{tenant}/certificate/{id}/configuration | deletes a certificate by id
[**allocation_tenant_certificate_id_configuration_get**](CertificateApi.md#allocation_tenant_certificate_id_configuration_get) | **GET** /allocation/{tenant}/certificate/{id}/configuration | returns the configuration of a certificate allocation
[**allocation_tenant_certificate_id_configuration_put**](CertificateApi.md#allocation_tenant_certificate_id_configuration_put) | **PUT** /allocation/{tenant}/certificate/{id}/configuration | create a new certificate. It is impossible to update an existing certificate. This requires a delete of the existing certificate and creation of a new one with the wanted configuration.
[**allocation_tenant_certificate_id_get**](CertificateApi.md#allocation_tenant_certificate_id_get) | **GET** /allocation/{tenant}/certificate/{id} | returns the status of a specific certificate allocation by id
[**allocation_tenant_certificate_id_status_get**](CertificateApi.md#allocation_tenant_certificate_id_status_get) | **GET** /allocation/{tenant}/certificate/{id}/status | retuns a brief status description of a certificate allocation



## allocation_tenant_certificate_get

> Vec<String> allocation_tenant_certificate_get(tenant)
returns a list of all certificate names that are allocated to a tenant

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


## allocation_tenant_certificate_id_actual_get

> models::Certificate allocation_tenant_certificate_id_actual_get(tenant, id)
returns the actual configuration of a certificate allocation. This may not represent the wanted configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | certificate name | [required] |

### Return type

[**models::Certificate**](Certificate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_certificate_id_configuration_delete

> allocation_tenant_certificate_id_configuration_delete(tenant, id)
deletes a certificate by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | certificate name | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_certificate_id_configuration_get

> models::Certificate allocation_tenant_certificate_id_configuration_get(tenant, id)
returns the configuration of a certificate allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | certificate name | [required] |

### Return type

[**models::Certificate**](Certificate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_certificate_id_configuration_put

> allocation_tenant_certificate_id_configuration_put(tenant, id, certificate)
create a new certificate. It is impossible to update an existing certificate. This requires a delete of the existing certificate and creation of a new one with the wanted configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | certificate name | [required] |
**certificate** | [**Certificate**](Certificate.md) | the JSON object containing the configuration of a certificate. certChainSecret and keySecret must be known to the platform. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_certificate_id_get

> models::CertificateStatus allocation_tenant_certificate_id_get(tenant, id)
returns the status of a specific certificate allocation by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | certificate name | [required] |

### Return type

[**models::CertificateStatus**](CertificateStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allocation_tenant_certificate_id_status_get

> models::AllocationStatus allocation_tenant_certificate_id_status_get(tenant, id)
retuns a brief status description of a certificate allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | tenant name | [required] |
**id** | **String** | certificate name | [required] |

### Return type

[**models::AllocationStatus**](AllocationStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

