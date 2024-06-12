# How to run
The SDK is compatible with running in a container on a DSH tenant, on DSH System Space, on a machine with Kafka
Proxy/VPN or on a local machine to a local Kafka

[DSH](#dsh)
[System Space](#system-space)
[Kafka Proxy/VPN](#kafka-proxyvpn)
[Local](#local)

## DSH
The following environment variables are required to run on DSH, and are set by DSH automatically:
- `MESOS_TASK_ID` - The task id of the running container
- `MARATHON_APP_ID` - Includes the tenant name of the running container
- `DSH_CA_CERTIFICATE` - The CA certificate of DSH
- `DSH_SECRET_TOKEN` - The secret token to authenticate to DSH

### System Space
When running on DSH System Space, the following environment variables are required:
- `DSH_SECRET_TOKEN_PATH` - The path to the secret token file.

## Kafka Proxy/VPN
When running on a machine with Kafka Proxy/VPN, the following environment variables are required:
- `PKI_CONFIG_DIR` - The path to the directory containing the certificates and private key
- `DSH_TENANT_NAME` - The tenant name of which you want to connect to
- `KAFKA_BOOTSTRAP_SERVERS` - The hostnames of the Kafka brokers

### Note!
Currently only PEM formatted certificates and keys are supported. Make sure to convert your certificates and key to PEM format if they are not already.

## Local
You can start the [docker-compose](../docker/docker-compose.yml) file to start a local Kafka broker and Schema Registry.

When no environment variables are set, it will default to a local configuration.
- Kafka will be set to `localhost:9092` and uses plaintext instead of SSL
- Schema Registry will be set to `localhost:8081/apis/ccompat/v7`

You can overwrite this by providing a [local_datastreams.json](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdklocal_datastreams.json) file to the root of the project or by setting the following environment variables.
- `KAFKA_BOOTSTRAP_SERVERS` - The hostnames of the Kafka brokers
- `SCHEMA_REGISTRY_HOST` - The host of the Schema Registry