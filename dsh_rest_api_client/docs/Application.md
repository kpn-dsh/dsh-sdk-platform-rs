# Application

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpus** | **f64** | How many CPUs this application needs (0.5 = 50% of 1 cpu) | 
**mem** | **i32** | Amount of memory your application needs in MB | 
**env** | Option<**std::collections::HashMap<String, String>**> | Environment variables | [optional]
**exposed_ports** | Option<[**std::collections::HashMap<String, models::PortMapping>**](PortMapping.md)> | Exposes ports of your application outside the platform | [optional]
**health_check** | Option<[**models::HealthCheck**](HealthCheck.md)> |  | [optional]
**image** | **String** | The container image to launch | 
**instances** | Option<**i32**> | Number of instances that need to be spun up for this app | [optional][default to 1]
**needs_token** | Option<**bool**> | If true, the platform will provision a secret token in the `DSH_SECRET_TOKEN` environment variable. This token can be exchanged for a client certificate that can be used for authentication to, amongst others, the Kafka brokers.  | [optional][default to true]
**single_instance** | Option<**bool**> | If true, the platform will ensure that there is always at most one instance of this application running at the same time. This impacts restart and upgrade behavior: A single-instance application will be terminated before a replacement is started, whereas an application that is not single-instance will remain running until its replacement has started and reports healthy. **Note** Applications that define volumes are always implicitly treated as single-instance, even if this flag is not set. | [optional][default to false]
**user** | **String** | The userid:groupid combination used to start the application container. | 
**metrics** | Option<[**models::Metrics**](Metrics.md)> |  | [optional]
**spread_group** | Option<**String**> | The spread group - if any - to be used to ensure instances of one or more applications are not scheduled onto the same node. | [optional]
**secrets** | Option<[**Vec<models::ApplicationSecret>**](ApplicationSecret.md)> |  | [optional]
**topics** | Option<**Vec<String>**> | names of scratch topics to which the application needs access. | [optional]
**readable_streams** | Option<**Vec<String>**> | names of streams to which the application needs read access. | [optional]
**writable_streams** | Option<**Vec<String>**> | names of streams to which the application needs write access. | [optional]
**volumes** | Option<[**std::collections::HashMap<String, models::ApplicationVolumes>**](Application_volumes.md)> | The volumes to be mounted in the container. The dictionary key is the mount point. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


