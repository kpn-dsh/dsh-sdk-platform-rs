# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Description
This library is a Rust implementation of the DSH SDK Platform. It is intended to be used as a base for services that will be used to interact with DSH. It is not intended to be used directly. Features include:
- Bootstrap to DSH
- Fetch Kafka Properties (datastream)
- Common functions 
  - Preconfigured RDKafka client config
- Graceful shutdown
- Dead Letter Queue (experimental)

## Feautures

The following features are available in this library and can be enabled/disabled in your Cargo.toml file.:

| **feature** | **default** | **Description** |
|---|---|---|
| `local` | &check; | Use bootstrap in a local environment* |
| `graceful_shutdown` | &check; | Create a signal handler for implementing a graceful shutdown |
| `dlq` | &cross; | Dead Letter Queue implementation (experimental) |
| `rdkafka-ssl` | &check; | Dynamically link to librdkafka to a locally installed OpenSSL |
| `rdkafka-ssl-vendored` | &cross; | Build OpenSSL during compile and statically link librdkafka<br>(No initial install required in environment, slower compile time) |

See api documentation for more information on how to use these features including.

\* Requires a local_datastream.json in your project root.

### Note
Rdkafka and thereby this library is dependent on CMAKE. Make sure it is installed in your environment and/or Dockerfile where you are compiling.

For example, in your Dockerfile:
```dockerfile
RUN apt-get update && apt-get install -y \
    cmake 
```

## Examples
See folder `examples` for more information on how to use this library.
Most examples require a local_datastream.json in your project root. 
Make sure to copy it from the root of this repository.

See folder `service_example` for a more complete example of how to use this library, including how to build the Rust project and post it to Harbor. See more information [here](service_example/README.md) .

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information on how to contribute to this project.

## License
See [LICENSE](LICENSE) for more information on the license for this project.

## Security
See [SECURITY.md](SECURITY.md) for more information on the security policy for this project.
