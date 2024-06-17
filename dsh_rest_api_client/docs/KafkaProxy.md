# KafkaProxy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the new Kafka Proxy | [optional]
**zone** | **String** | Available networks on this platform | 
**cpus** | **f64** | CPU quota for each Kafka Proxy (minimum 0.3 = 30% of 1 CPU) | 
**mem** | **i32** | Memory (MB) for each Kafka Proxy (minimum 1024 = 1 GB) | 
**instances** | **i32** | Number of instances | 
**secret_name_ca_chain** | **String** | Secret name containing the Ca Cert | 
**certificate** | **String** | Secret name with the server certificate | 
**schema_store** | Option<**bool**> | Set to True no enable Schema Store | [optional]
**schema_store_cpus** | Option<**f64**> | CPU quota for Schema Store (minimum 0.3 = 30% of 1 CPU) | [optional]
**schema_store_mem** | Option<**i32**> | Memory (MB) for Schema Store (minimum 256MB) | [optional]
**validations** | Option<[**Vec<models::Validations>**](Validations.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


