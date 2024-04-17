# PortMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth** | Option<**String**> | TODO | [optional]
**mode** | Option<**String**> | Routing mode. The allowed values are:   * `http` (default if this property is omitted). HTTP routing and TLS termination are done by the platform. In this case, the `tls` and (optionally) `paths` settings should be configured as well.   * `tcp/<endpoint>`. The platform only does plain TCP routing, with TLS pass-through. When set, the `tls` and `paths` settings are ignored. The application is responsible for TLS termination and certificate management. There are various possible values for `<endpoint>` that may appear when listing allocation configurations, but the only value that is allowed to be set in regular application allocations is `tcp/https`.     * `tcp/https`. Any traffic arriving on `<vhost>:443` will be forwarded (TLS included) to the service.     * `tcp/kafka-proxy` is used by Kafka Proxies. This endpoint is auto-configured by the platform when allocating a Kafka Proxy application and should *not* be used when allocating regular applications.     * `tcp/vpn-tcp` is used by a VPN application. This endpoint is auto-configured by the platform when allocating a VPN application and should *not* be used when allocating regular applications.  | [optional]
**paths** | Option<[**Vec<models::PathSpec>**](PathSpec.md)> | The paths which are allowed on the associated vhost | [optional]
**tls** | Option<**String**> | The default is 'auto', indicating that the port will only accept secured connections. Put this to 'none' if you do not want the service to have a secure endpoint. | [optional]
**vhost** | Option<**String**> | The host name that needs to be assigned to this port (for multiple names, separate them with commas) | [optional]
**whitelist** | Option<**String**> | Put ip addresses or ip ranges that can call this service here (for multiple addresses, separate them with spaces) | [optional]
**service_group** | Option<**String**> | To load balance traffic between different services, use this optional field to put those services in the same service group. Choose any name consisting of all lowercase letters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


