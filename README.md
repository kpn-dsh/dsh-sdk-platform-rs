# DSH-SDK-PLATFORM-RS
This repository contains the Rust SDK for the Data Sharing Hub (DSH) platform. 

## DSH_SDK
The [dsh_sdk](dsh_sdk) is a Rust library that provides a simple interface to interact with the DSH platform. The SDK is used to create and manage data streams, and to send data to the DSH platform.
See [dsh_sdk/README.md](dsh_sdk/README.md) for more information.

## Example DSH Service
The [example_dsh_service](example_dsh_service) is a simple example of a service that uses the DSH SDK. It demonstrates how to create an app, consume data from Kafka, and how to build and deploy the service to DSH.

## Docker
The [docker](docker) directory contains a docker-compose file that can be used for local development. The docker-compose file starts a Kafka cluster, a Zookeeper instance and a schema registry.
---

### Changelog
See [CHANGELOG.md](CHANGELOG.md) for all changes per version.

### Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information on how to contribute to this project.

### License
See [LICENSE](LICENSE) for more information on the license for this project.

### Security
See [SECURITY.md](SECURITY.md) for more information on the security policy for this project.

---
_Copyright (c) Koninklijke KPN N.V._ 
