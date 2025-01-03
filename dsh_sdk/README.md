# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![dependency status](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs/status.svg)](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# NOTE
As this is a release candidate it may contain bugs and/or incomplete features and incorrect documentation and future updates may contain breaking changes.

Please report any issues you encounter.

## Migration guide 0.4.X -> 0.5.X
See [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for more information on how to migrate from 0.4.X to 0.5.X.

## Description
This library can be used to interact with the DSH Platform. It is intended to be used as a base for services that will be used to interact with DSH. Features include:
- Connect to DSH 
- Fetch Kafka Properties and certificates
- Rest API Token Fetcher (to be used with [dsh_rest_api_client](https://crates.io/crates/dsh_rest_api_client))
- MQTT Token Fetcher
- Common functions 
  - Preconfigured RDKafka client config
  - Preconfigured Reqwest client config (for schema store)
- Graceful shutdown
- Prometheus Metrics (web server and re-export of metrics crate)
- Dead Letter Queue (experimental)

### Note
Rdkafka and thereby this library is dependent on CMAKE. Make sure it is installed in your environment and/or Dockerfile where you are compiling.
See [dockerfile](../example_dsh_service/Dockerfile) for an example.

## Usage
To use this SDK with the default features in your project, add the following to your Cargo.toml file:
  
```toml
[dependencies]
dsh_sdk = "0.5"
```

However, if you would like to use only specific features, you can specify them in your Cargo.toml file. For example, if you would like to use only the bootstrap feature, add the following to your Cargo.toml file:
  
```toml
[dependencies]
dsh_sdk = { version = "0.5", default-features = false, features = ["rdkafka-config"] }
rdkafka = { version =  "0.37", features = ["cmake-build", "ssl-vendored"] }
```

See [feature flags](#feature-flags) for more information on the available features.

To use this SDK in your project
```rust
use dsh_sdk::DshKafkaConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // get a rdkafka consumer config for example
    let consumer: StreamConsumer = ClientConfig::new().dsh_consumer_config().create()?;
}
```

## Connect to DSH
The SDK is compatible with running in a container on a DSH tenant, on DSH System Space, on a machine with Kafka Proxy/VPN or on a local machine to a local Kafka. 
See [CONNECT_PROXY_VPN_LOCAL](CONNECT_PROXY_VPN_LOCAL.md) for more info.

## Feature flags
See the [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for more information on the changes in feature flags since the v0.5.X update.

The following features are available in this library and can be enabled/disabled in your Cargo.toml file:

| **feature** | **default** | **Description** |
|---|---|---|
| `bootstrap` | &check; | Generate signed certificate and fetch datastreams info |
| `kafka` |  &check; | Enable `DshKafkaConfig` trait and Config struct to connect to DSH |
| `rdkafka-config` | &check; | Enable `DshKafkaConfig` implementation for RDKafka |
| `protocol-token-fetcher` | &cross; | Fetch tokens to use DSH Protocol adapters (MQTT and HTTP) |
| `management-api-token-fetcher` | &cross; | Fetch tokens to use DSH Management API |
| `metrics` | &cross; | Enable prometheus metrics including http server |
| `graceful-shutdown` | &cross; | Tokio based gracefull shutdown handler |
| `dlq` | &cross; | Dead Letter Queue implementation |

The following features are renamed or replaced:
| **feature** | **replacement** |
|---|---|---|
| `rest-token-fetcher` | Replaced by `management-api-token-fetcher` |
| `mqtt-token-fetcher` | Replaced by `protocol-token-fetcher` |

See api documentation for more information on how to use these features including.

## Environment variables
The default RDKafka config can be overwritten by setting environment variables. See [ENV_VARIABLES.md](ENV_VARIABLES.md) for more information.


## Api doc
See the [api documentation](https://docs.rs/dsh_sdk/latest/dsh_sdk/) for more information on how to use this library.

## Examples
See folder [dsh_sdk/examples](/examples/) for simple examples on how to use the SDK.

### Full service example
See folder [example_dsh_service](../example_dsh_service/) for a full service, including how to build the Rust project and post it to Harbor. See [readme](../example_dsh_service/README.md) for more information.

## Changelog
See [CHANGELOG.md](CHANGELOG.md) for all changes per version.

## Contributing
See [CONTRIBUTING.md](../CONTRIBUTING.md) for more information on how to contribute to this project.

## License
See [LICENSE](../LICENSE) for more information on the license for this project.

## Security
See [SECURITY.md](../SECURITY.md) for more information on the security policy for this project.

---
_Copyright (c) Koninklijke KPN N.V._ 
